//! macOS-specific implementations using Apple frameworks.
//!
//! This module provides real implementations of device AI capabilities for macOS
//! using Apple's Vision, NaturalLanguage, and AppKit frameworks via objc2 bindings.

use crate::models::{
    Barcode, BarcodeFormat, BarcodeOptions, BoundingBox, Classification, ClassificationOptions,
    Face, FaceLandmarks, FaceOptions, ImageSource, LanguageAlternative, LanguageIdentification,
    OcrOptions, Point, RecognitionLevel, SynthesisOptions, TextBlock, TextLine,
    TextRecognitionResult, Voice, VoiceGender, VoiceQuality,
};
use crate::{Error, Result};

use objc2::rc::Retained;
use objc2::runtime::AnyObject;
use objc2::{class, msg_send, AnyThread};
use objc2_foundation::{NSData, NSDictionary, NSError, NSNumber, NSString};

// =============================================================================
// Helper: Decode image data
// =============================================================================

/// Decode base64 image data to raw bytes
fn decode_image_source(image: &ImageSource) -> Result<Vec<u8>> {
    match image {
        ImageSource::Base64(data) => {
            // Remove data URI prefix if present
            let base64_data = if data.contains(',') {
                data.split(',').next_back().unwrap_or(data)
            } else {
                data.as_str()
            };

            use base64::Engine;
            base64::engine::general_purpose::STANDARD
                .decode(base64_data)
                .map_err(|e| Error::ImageProcessingFailed {
                    message: format!("Failed to decode base64 image: {}", e),
                })
        }
        ImageSource::Bytes(bytes) => Ok(bytes.clone()),
        ImageSource::FilePath(path) => {
            std::fs::read(path).map_err(|e| Error::ImageProcessingFailed {
                message: format!("Failed to read image file: {}", e),
            })
        }
    }
}

// =============================================================================
// Helper: Extract confidence from SFTranscription segments
// =============================================================================

/// Extract average confidence from an SFTranscription's segments via objc2 msg_send.
/// Returns 0.0 if no segments are available.
unsafe fn extract_transcription_confidence(transcription: *mut AnyObject) -> f32 {
    if transcription.is_null() {
        return 0.0;
    }

    let segments: *mut AnyObject = msg_send![transcription, segments];
    if segments.is_null() {
        return 0.0;
    }

    let count: usize = msg_send![segments, count];
    if count == 0 {
        return 0.0;
    }

    let mut total_confidence: f32 = 0.0;
    for i in 0..count {
        let segment: *mut AnyObject = msg_send![segments, objectAtIndex: i];
        if !segment.is_null() {
            let conf: f32 = msg_send![segment, confidence];
            total_confidence += conf;
        }
    }

    total_confidence / count as f32
}

// =============================================================================
// Language Identification
// =============================================================================

/// Identify the language of the given text using NLLanguageRecognizer.
pub fn text_identify_language(text: &str) -> Result<LanguageIdentification> {
    use objc2_natural_language::NLLanguageRecognizer;

    // Create the language recognizer
    let recognizer = unsafe { NLLanguageRecognizer::new() };

    // Create NSString from the input text
    let ns_text = NSString::from_str(text);

    // Process the text
    unsafe {
        recognizer.processString(&ns_text);
    }

    // Get the dominant language
    let dominant_language = unsafe { recognizer.dominantLanguage() };

    let language = match dominant_language {
        Some(lang) => lang.to_string(),
        None => {
            return Err(Error::TextProcessingFailed {
                message: "Could not identify language".to_string(),
            });
        }
    };

    // Get hypothesis with confidence
    let hypotheses = unsafe { recognizer.languageHypothesesWithMaximum(5) };

    // Get all keys and build alternatives list
    let all_keys = hypotheses.allKeys();
    let mut primary_confidence = 1.0f32;
    let mut alternatives: Vec<LanguageAlternative> = Vec::new();

    for key in all_keys.iter() {
        let lang_code = key.to_string();
        if let Some(value) = hypotheses.objectForKey(&*key) {
            let confidence = value.doubleValue() as f32;
            if lang_code == language {
                primary_confidence = confidence;
            } else {
                alternatives.push(LanguageAlternative {
                    language: lang_code,
                    confidence,
                });
            }
        }
    }

    // Sort alternatives by confidence descending
    alternatives.sort_by(|a, b| {
        b.confidence
            .partial_cmp(&a.confidence)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    // Reset the recognizer for future use
    unsafe {
        recognizer.reset();
    }

    Ok(LanguageIdentification {
        language,
        confidence: primary_confidence,
        alternatives,
    })
}
// =============================================================================
// Speech Recognition
// =============================================================================

use block2::StackBlock;
use std::sync::{Arc, Condvar, Mutex};
use std::time::Duration;

/// Recognize speech from the microphone (one-shot).
pub fn speech_recognize(
    options: crate::models::RecognitionOptions,
) -> Result<crate::models::RecognitionResult> {
    // Check if we're using an audio file or microphone
    match &options.audio_source {
        Some(crate::models::AudioSource::Base64(data)) => {
            speech_recognize_from_file_data(decode_audio_base64(data)?, &options)
        }
        Some(crate::models::AudioSource::Bytes(bytes)) => {
            speech_recognize_from_file_data(bytes.clone(), &options)
        }
        Some(crate::models::AudioSource::FilePath(path)) => {
            let data = std::fs::read(path).map_err(|e| Error::SpeechRecognitionFailed {
                message: format!("Failed to read audio file: {}", e),
            })?;
            speech_recognize_from_file_data(data, &options)
        }
        Some(crate::models::AudioSource::Microphone) | None => {
            speech_recognize_from_microphone(options)
        }
    }
}

/// Decode base64 audio data
fn decode_audio_base64(data: &str) -> Result<Vec<u8>> {
    // Remove data URI prefix if present
    let base64_data = if data.contains(',') {
        data.split(',').next_back().unwrap_or(data)
    } else {
        data
    };

    use base64::Engine;
    base64::engine::general_purpose::STANDARD
        .decode(base64_data)
        .map_err(|e| Error::SpeechRecognitionFailed {
            message: format!("Failed to decode base64 audio: {}", e),
        })
}

unsafe fn create_available_speech_recognizer(
    options: &crate::models::RecognitionOptions,
) -> Result<*mut AnyObject> {
    let locale_str = options.language.as_deref().unwrap_or("en-US");
    let locale_ns = NSString::from_str(locale_str);

    let locale: *mut AnyObject = msg_send![class!(NSLocale), alloc];
    let locale: *mut AnyObject = msg_send![locale, initWithLocaleIdentifier: &*locale_ns];

    let recognizer: *mut AnyObject = msg_send![class!(SFSpeechRecognizer), alloc];
    let recognizer: *mut AnyObject = msg_send![recognizer, initWithLocale: locale];

    if recognizer.is_null() {
        return Err(Error::SpeechRecognitionFailed {
            message: "Failed to create speech recognizer".to_string(),
        });
    }

    let is_available: bool = msg_send![recognizer, isAvailable];
    if !is_available {
        return Err(Error::SpeechRecognitionFailed {
            message: "Speech recognizer not available".to_string(),
        });
    }

    let auth_status: isize = msg_send![class!(SFSpeechRecognizer), authorizationStatus];
    if auth_status == 1 || auth_status == 2 {
        return Err(Error::SpeechRecognitionFailed {
            message: "Speech recognition not authorized".to_string(),
        });
    }

    Ok(recognizer)
}

/// Recognize speech from an audio file
fn speech_recognize_from_file_data(
    audio_data: Vec<u8>,
    options: &crate::models::RecognitionOptions,
) -> Result<crate::models::RecognitionResult> {
    unsafe {
        let recognizer = create_available_speech_recognizer(options)?;

        // Create NSURL for the audio data (we need to write to a temp file)
        let temp_dir = std::env::temp_dir();
        let temp_path = temp_dir.join(format!("speech_recognition_{}.wav", uuid::Uuid::new_v4()));
        std::fs::write(&temp_path, &audio_data).map_err(|e| Error::SpeechRecognitionFailed {
            message: format!("Failed to write temp audio file: {}", e),
        })?;

        let path_str = temp_path
            .to_str()
            .ok_or_else(|| Error::SpeechRecognitionFailed {
                message: "Invalid temp file path".to_string(),
            })?;
        let path_ns = NSString::from_str(path_str);
        let url: *mut AnyObject = msg_send![class!(NSURL), fileURLWithPath: &*path_ns];

        // Create recognition request from URL
        let request: *mut AnyObject = msg_send![class!(SFSpeechURLRecognitionRequest), alloc];
        let request: *mut AnyObject = msg_send![request, initWithURL: url];

        if request.is_null() {
            let _ = std::fs::remove_file(&temp_path);
            return Err(Error::SpeechRecognitionFailed {
                message: "Failed to create recognition request".to_string(),
            });
        }

        let _: () = msg_send![request, setShouldReportPartialResults: false];

        // Shared result storage
        let result_data = Arc::new((
            Mutex::new(None::<Result<crate::models::RecognitionResult>>),
            Condvar::new(),
        ));
        let result_data_clone = result_data.clone();

        // Create completion block
        let block = StackBlock::new(move |result: *mut AnyObject, error: *mut AnyObject| {
            let (lock, cvar) = &*result_data_clone;

            if !error.is_null() {
                // Get error description
                let desc: *mut AnyObject = msg_send![error, localizedDescription];
                let ns_str: &NSString = &*(desc as *const NSString);
                let error_msg = ns_str.to_string();

                let mut data = lock.lock().unwrap();
                *data = Some(Err(crate::Error::SpeechRecognitionFailed {
                    message: format!("Recognition error: {}", error_msg),
                }));
                cvar.notify_one();
                return;
            }

            if !result.is_null() {
                let is_final: bool = msg_send![result, isFinal];

                // Only process final results
                if is_final {
                    let best_transcription: *mut AnyObject = msg_send![result, bestTranscription];
                    if !best_transcription.is_null() {
                        let formatted_string: *mut AnyObject =
                            msg_send![best_transcription, formattedString];
                        if !formatted_string.is_null() {
                            let ns_str: &NSString = &*(formatted_string as *const NSString);
                            let text = ns_str.to_string();
                            let confidence = extract_transcription_confidence(best_transcription);

                            let mut data = lock.lock().unwrap();
                            *data = Some(Ok(crate::models::RecognitionResult {
                                text,
                                confidence,
                                is_final: true,
                                alternatives: vec![],
                            }));
                            cvar.notify_one();
                        }
                    }
                }
            }
        });
        let block = block.copy();

        // Start recognition task
        let task: *mut AnyObject =
            msg_send![recognizer, recognitionTaskWithRequest: request, resultHandler: &*block];

        if task.is_null() {
            let _ = std::fs::remove_file(&temp_path);
            return Err(Error::SpeechRecognitionFailed {
                message: "Failed to start recognition task".to_string(),
            });
        }

        // Wait for result with timeout (30 seconds for file processing)
        let (lock, cvar) = &*result_data;
        let result = {
            let data = lock.lock().unwrap();
            let mut wait_result = cvar.wait_timeout(data, Duration::from_secs(30)).unwrap();

            if wait_result.1.timed_out() {
                let _ = std::fs::remove_file(&temp_path);
                return Err(Error::SpeechRecognitionFailed {
                    message: "Speech recognition timed out - no response from recognizer"
                        .to_string(),
                });
            }

            wait_result.0.take()
        };

        // Clean up temp file
        let _ = std::fs::remove_file(&temp_path);

        match result {
            Some(Ok(res)) => Ok(res),
            Some(Err(e)) => Err(e),
            None => Err(Error::SpeechRecognitionFailed {
                message: "No speech recognized in audio file".to_string(),
            }),
        }
    }
}

/// Recognize speech from microphone (live input)
fn speech_recognize_from_microphone(
    options: crate::models::RecognitionOptions,
) -> Result<crate::models::RecognitionResult> {
    unsafe {
        let recognizer = create_available_speech_recognizer(&options)?;

        // Create audio engine and request
        let audio_engine: *mut AnyObject = msg_send![class!(AVAudioEngine), new];
        let input_node: *mut AnyObject = msg_send![audio_engine, inputNode];

        let request: *mut AnyObject = msg_send![class!(SFSpeechAudioBufferRecognitionRequest), new];
        let _: () = msg_send![request, setShouldReportPartialResults: false];

        // Configure audio session
        let session: *mut AnyObject = msg_send![class!(AVAudioSession), sharedInstance];
        let category = NSString::from_str("AVAudioSessionCategoryRecord");
        let mode = NSString::from_str("AVAudioSessionModeMeasurement");
        let mut error: *mut NSError = std::ptr::null_mut();
        let _: bool = msg_send![session, setCategory: &*category, mode: &*mode, options: 0usize, error: &mut error];
        let _: bool = msg_send![session, setActive: true, withOptions: 0usize, error: &mut error];

        // Shared result storage
        let result_data = Arc::new((
            Mutex::new(None::<crate::models::RecognitionResult>),
            Condvar::new(),
        ));
        let result_data_clone = result_data.clone();

        // Create completion block
        let block = StackBlock::new(move |result: *mut AnyObject, error: *mut AnyObject| {
            let (lock, cvar) = &*result_data_clone;

            if !error.is_null() {
                let mut data = lock.lock().unwrap();
                *data = Some(crate::models::RecognitionResult {
                    text: String::new(),
                    confidence: 0.0,
                    is_final: true,
                    alternatives: vec![],
                });
                cvar.notify_one();
                return;
            }

            if !result.is_null() {
                let best_transcription: *mut AnyObject = msg_send![result, bestTranscription];
                let formatted_string: *mut AnyObject =
                    msg_send![best_transcription, formattedString];
                let ns_str: &NSString = &*(formatted_string as *const NSString);
                let text = ns_str.to_string();

                let is_final: bool = msg_send![result, isFinal];
                let confidence = extract_transcription_confidence(best_transcription);

                let mut data = lock.lock().unwrap();
                *data = Some(crate::models::RecognitionResult {
                    text,
                    confidence,
                    is_final,
                    alternatives: vec![],
                });
                cvar.notify_one();
            }
        });
        let block = block.copy();

        // Start recognition task
        let task: *mut AnyObject =
            msg_send![recognizer, recognitionTaskWithRequest: request, resultHandler: &*block];

        if task.is_null() {
            return Err(Error::SpeechRecognitionFailed {
                message: "Failed to start recognition task".to_string(),
            });
        }

        // Install tap on audio input
        let recording_format: *mut AnyObject = msg_send![input_node, outputFormatForBus: 0usize];
        let tap_block = StackBlock::new(move |buffer: *mut AnyObject, _when: *mut AnyObject| {
            let _: () = msg_send![request, appendAudioPCMBuffer: buffer];
        });
        let tap_block = tap_block.copy();

        let _: () = msg_send![input_node, installTapOnBus: 0usize, bufferSize: 1024u32, format: recording_format, block: &*tap_block];

        // Start audio engine
        let mut engine_error: *mut NSError = std::ptr::null_mut();
        let started: bool = msg_send![audio_engine, startAndReturnError: &mut engine_error];

        if !started {
            return Err(Error::SpeechRecognitionFailed {
                message: "Failed to start audio engine".to_string(),
            });
        }

        // Wait for result with timeout (5 seconds)
        let (lock, cvar) = &*result_data;
        let result = {
            let data = lock.lock().unwrap();
            let mut wait_result = cvar.wait_timeout(data, Duration::from_secs(5)).unwrap();

            // Stop audio engine
            let _: () = msg_send![audio_engine, stop];
            let _: () = msg_send![input_node, removeTapOnBus: 0usize];
            let _: () = msg_send![request, endAudio];

            if wait_result.1.timed_out() {
                return Err(Error::SpeechRecognitionFailed {
                    message: "Speech recognition timed out".to_string(),
                });
            }

            wait_result.0.take()
        };

        result.ok_or_else(|| Error::SpeechRecognitionFailed {
            message: "No speech recognized".to_string(),
        })
    }
}

// =============================================================================
// Speech Synthesis
// =============================================================================

/// Synthesize and speak text using NSSpeechSynthesizer.
pub fn speech_synthesize(text: &str, options: SynthesisOptions) -> Result<()> {
    unsafe {
        // Create NSSpeechSynthesizer
        let synthesizer: *mut AnyObject = msg_send![class!(NSSpeechSynthesizer), alloc];

        // Initialize with voice if specified
        let synthesizer: *mut AnyObject = if let Some(ref voice_id) = options.voice {
            let voice_str = NSString::from_str(voice_id);
            msg_send![synthesizer, initWithVoice: &*voice_str]
        } else {
            msg_send![synthesizer, init]
        };

        if synthesizer.is_null() {
            return Err(Error::SpeechSynthesisFailed {
                message: "Failed to create speech synthesizer".to_string(),
            });
        }

        // Set rate if specified (normal rate is about 180-200 words/min)
        if let Some(rate) = options.rate {
            // NSSpeechSynthesizer rate is words per minute, normalize around 175
            let words_per_minute: f32 = 175.0 * rate;
            let _: () = msg_send![synthesizer, setRate: words_per_minute];
        }

        // Set volume if specified
        if let Some(volume) = options.volume {
            let _: () = msg_send![synthesizer, setVolume: volume];
        }

        // Speak the text
        let ns_text = NSString::from_str(text);
        let success: bool = msg_send![synthesizer, startSpeakingString: &*ns_text];

        if !success {
            return Err(Error::SpeechSynthesisFailed {
                message: "Failed to start speaking".to_string(),
            });
        }

        // Wait for speech to complete
        loop {
            let is_speaking: bool = msg_send![synthesizer, isSpeaking];
            if !is_speaking {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(50));
        }

        Ok(())
    }
}

/// Get available voices for speech synthesis.
pub fn speech_get_voices() -> Result<Vec<Voice>> {
    unsafe {
        // Get available voice identifiers
        let voice_ids: *mut AnyObject = msg_send![class!(NSSpeechSynthesizer), availableVoices];

        if voice_ids.is_null() {
            return Err(Error::SpeechSynthesisFailed {
                message: "Failed to get available voices".to_string(),
            });
        }

        let count: usize = msg_send![voice_ids, count];
        let mut voices = Vec::with_capacity(count);

        for i in 0..count {
            let voice_id: *mut AnyObject = msg_send![voice_ids, objectAtIndex: i];

            // Get voice attributes
            let attrs: *mut AnyObject =
                msg_send![class!(NSSpeechSynthesizer), attributesForVoice: voice_id];

            if attrs.is_null() {
                continue;
            }

            // Get voice name
            let name_key = NSString::from_str("VoiceName");
            let name: *mut AnyObject = msg_send![attrs, objectForKey: &*name_key];
            let name_str = if !name.is_null() {
                let s: *const AnyObject = name;
                let ns_str: &NSString = &*(s as *const NSString);
                ns_str.to_string()
            } else {
                "Unknown".to_string()
            };

            // Get language
            let lang_key = NSString::from_str("VoiceLocaleIdentifier");
            let lang: *mut AnyObject = msg_send![attrs, objectForKey: &*lang_key];
            let lang_str = if !lang.is_null() {
                let s: *const AnyObject = lang;
                let ns_str: &NSString = &*(s as *const NSString);
                ns_str.to_string()
            } else {
                "en-US".to_string()
            };

            // Get voice identifier as string
            let id_ns_str: &NSString = &*(voice_id as *const NSString);
            let id_str = id_ns_str.to_string();

            // Determine gender from voice name (heuristic)
            let gender = if name_str.to_lowercase().contains("female")
                || name_str.contains("Samantha")
                || name_str.contains("Victoria")
                || name_str.contains("Karen")
                || name_str.contains("Fiona")
            {
                Some(VoiceGender::Female)
            } else if name_str.to_lowercase().contains("male")
                || name_str.contains("Alex")
                || name_str.contains("Daniel")
                || name_str.contains("Fred")
            {
                Some(VoiceGender::Male)
            } else {
                None
            };

            // Check quality (premium voices have "Premium" or "Enhanced" in identifier)
            let quality = if id_str.contains("premium") || id_str.contains("Premium") {
                Some(VoiceQuality::Premium)
            } else if id_str.contains("enhanced") || id_str.contains("Enhanced") {
                Some(VoiceQuality::Enhanced)
            } else {
                Some(VoiceQuality::Default)
            };

            voices.push(Voice {
                id: id_str,
                name: name_str,
                language: lang_str,
                is_default: i == 0, // First voice is typically default
                quality,
                gender,
            });
        }

        Ok(voices)
    }
}

// =============================================================================
// Vision - Text Recognition (OCR)
// =============================================================================

/// Recognize text in an image using VNRecognizeTextRequest.
pub fn vision_recognize_text(
    image: ImageSource,
    options: OcrOptions,
) -> Result<TextRecognitionResult> {
    let data = decode_image_source(&image)?;

    use objc2_vision::{
        VNImageRequestHandler, VNRecognizeTextRequest, VNRequestTextRecognitionLevel,
    };

    unsafe {
        // Create NSData
        let ns_data = NSData::with_bytes(&data);

        // Create VNImageRequestHandler with empty options dictionary
        let empty_dict = objc2_foundation::NSDictionary::new();
        let handler = VNImageRequestHandler::initWithData_options(
            VNImageRequestHandler::alloc(),
            &ns_data,
            &empty_dict,
        );

        // Create VNRecognizeTextRequest
        let request = VNRecognizeTextRequest::new();

        // Set recognition level
        let level = match options.recognition_level {
            Some(RecognitionLevel::Fast) => VNRequestTextRecognitionLevel::Fast,
            Some(RecognitionLevel::Accurate) | None => VNRequestTextRecognitionLevel::Accurate,
        };
        request.setRecognitionLevel(level);

        // Set languages if specified
        if !options.languages.is_empty() {
            let languages: Vec<Retained<NSString>> = options
                .languages
                .iter()
                .map(|l| NSString::from_str(l))
                .collect();
            let ns_array = objc2_foundation::NSArray::from_retained_slice(&languages);
            request.setRecognitionLanguages(&ns_array);
        }

        // Perform request — upcast VNRecognizeTextRequest -> VNImageBasedRequest -> VNRequest
        let request_as_vn: Retained<objc2_vision::VNRequest> =
            Retained::into_super(Retained::into_super(request.clone()));
        let requests = objc2_foundation::NSArray::from_retained_slice(&[request_as_vn]);

        handler
            .performRequests_error(&requests)
            .map_err(|e| Error::ImageProcessingFailed {
                message: format!("Text recognition failed: {}", e),
            })?;

        // Get results
        let results = request
            .results()
            .ok_or_else(|| Error::ImageProcessingFailed {
                message: "No results returned from request".to_string(),
            })?;

        let mut all_text = String::new();
        let mut blocks = Vec::new();

        for observation_obj in results {
            let observation = observation_obj
                .downcast::<objc2_vision::VNRecognizedTextObservation>()
                .ok()
                .ok_or_else(|| Error::ImageProcessingFailed {
                    message: "Failed to cast result to VNRecognizedTextObservation".to_string(),
                })?;

            let candidates = observation.topCandidates(1);
            if candidates.count() > 0 {
                let candidate = candidates.objectAtIndex(0);
                let text_ns: Retained<NSString> = candidate.string();
                let text_string = text_ns.to_string();
                let confidence = candidate.confidence();

                let bbox = observation.boundingBox();
                let width = bbox.size.width;
                let height = bbox.size.height;
                let x = bbox.origin.x;
                // Vision: bottom-left origin, normalized.
                // Tauri: top-left origin, normalized.
                // Flip Y.
                let y = 1.0 - bbox.origin.y - height;

                let bounding_box = BoundingBox {
                    x: x as f32,
                    y: y as f32,
                    width: width as f32,
                    height: height as f32,
                };

                // VNRecognizedText doesn't expose boundingBox directly easier than observation
                // observation is VNRecognizedTextObservation which has boundingBox
                // But we iterating over results which IS `Retained<VNRecognizedTextObservation>`

                if !all_text.is_empty() {
                    all_text.push('\n');
                }
                all_text.push_str(&text_string);

                blocks.push(TextBlock {
                    text: text_string.clone(),
                    bounding_box, // Placeholder as calculating from recognized text path is complex
                    lines: vec![TextLine {
                        text: text_string,
                        bounding_box,
                    }],
                    confidence: Some(confidence),
                });
            }
        }

        Ok(TextRecognitionResult {
            text: all_text,
            blocks,
        })
    }
}

// =============================================================================
// Vision - Barcode Detection
// =============================================================================

/// Detect barcodes in an image using VNDetectBarcodesRequest.
pub fn vision_detect_barcodes(
    image: ImageSource,
    _options: BarcodeOptions,
) -> Result<Vec<Barcode>> {
    let data = decode_image_source(&image)?;

    use objc2_vision::{VNDetectBarcodesRequest, VNImageRequestHandler};

    unsafe {
        let ns_data = NSData::with_bytes(&data);

        let empty_dict = objc2_foundation::NSDictionary::new();
        let handler = VNImageRequestHandler::initWithData_options(
            VNImageRequestHandler::alloc(),
            &ns_data,
            &empty_dict,
        );

        let request = VNDetectBarcodesRequest::new();

        // Upcast VNDetectBarcodesRequest -> VNImageBasedRequest -> VNRequest
        let request_as_vn: Retained<objc2_vision::VNRequest> =
            Retained::into_super(Retained::into_super(request.clone()));
        let requests = objc2_foundation::NSArray::from_retained_slice(&[request_as_vn]);

        handler
            .performRequests_error(&requests)
            .map_err(|e| Error::ImageProcessingFailed {
                message: format!("Barcode detection failed: {}", e),
            })?;

        let results = request
            .results()
            .ok_or_else(|| Error::ImageProcessingFailed {
                message: "No results returned".to_string(),
            })?;

        let mut barcodes = Vec::new();

        for observation_obj in results {
            let observation = observation_obj
                .downcast::<objc2_vision::VNBarcodeObservation>()
                .ok()
                .ok_or_else(|| Error::ImageProcessingFailed {
                    message: "Failed to cast result to VNBarcodeObservation".to_string(),
                })?;

            let payload = observation.payloadStringValue();
            let raw_value = payload.map(|s| s.to_string()).unwrap_or_default();

            // Symbology
            let symbology = observation.symbology();
            let symbology_str = symbology.to_string();

            let format = match symbology_str.as_str() {
                s if s.contains("QR") => BarcodeFormat::QrCode,
                s if s.contains("EAN13") || s.contains("EAN-13") => BarcodeFormat::Ean13,
                s if s.contains("EAN8") || s.contains("EAN-8") => BarcodeFormat::Ean8,
                s if s.contains("Code128") => BarcodeFormat::Code128,
                s if s.contains("Code39") => BarcodeFormat::Code39,
                s if s.contains("Code93") => BarcodeFormat::Code93,
                s if s.contains("Codabar") => BarcodeFormat::Codabar,
                s if s.contains("ITF") || s.contains("I2of5") => BarcodeFormat::Itf,
                s if s.contains("UPCE") => BarcodeFormat::UpcE,
                s if s.contains("PDF417") => BarcodeFormat::Pdf417,
                s if s.contains("Aztec") => BarcodeFormat::Aztec,
                s if s.contains("DataMatrix") => BarcodeFormat::DataMatrix,
                _ => BarcodeFormat::QrCode, // Default
            };

            let bbox = observation.boundingBox();
            // Vision: bottom-left origin, normalized.
            // Tauri: top-left origin, normalized.
            // Flip Y.
            let width = bbox.size.width;
            let height = bbox.size.height;
            let x = bbox.origin.x;
            // y = 1 - origin_y - height
            let y = 1.0 - bbox.origin.y - height;

            let bounding_box = BoundingBox {
                x: x as f32,
                y: y as f32,
                width: width as f32,
                height: height as f32,
            };

            barcodes.push(Barcode {
                format,
                raw_value,
                bounding_box,
                parsed_data: None,
            });
        }

        Ok(barcodes)
    }
}

// =============================================================================
// Vision - Face Detection
// =============================================================================

/// Detect faces using CoreImage (CIDetector) for attributes support.
fn detect_faces_ci(data: &[u8], options: FaceOptions) -> Result<Vec<Face>> {
    use objc2_core_image::{CIDetector, CIFaceFeature, CIImage};

    unsafe {
        let ns_data = NSData::with_bytes(data);

        // Wrap execution in objc2::exception::catch for maximum safety
        let result = objc2::exception::catch(|| {
            let ci_image =
                CIImage::imageWithData(&ns_data).ok_or_else(|| Error::ImageProcessingFailed {
                    message: "Failed to create CIImage".to_string(),
                })?;

            // Get image extent
            let extent = ci_image.extent();
            let image_width = extent.size.width as f32;
            let image_height = extent.size.height as f32;

            // Create CIDetector
            let type_face = NSString::from_str("CIDetectorTypeFace");
            let accuracy_key = NSString::from_str("CIDetectorAccuracy");
            let accuracy_high = NSString::from_str("CIDetectorAccuracyHigh");

            let keys = [&*accuracy_key];
            let accuracy_high_any: Retained<AnyObject> =
                Retained::into_super(Retained::into_super(accuracy_high));
            let objs = [&*accuracy_high_any];
            let opts = NSDictionary::<NSString, AnyObject>::from_slices(&keys, &objs);

            let detector = CIDetector::detectorOfType_context_options(
                &type_face,
                None, // No context
                Some(&*opts),
            )
            .ok_or_else(|| Error::ImageProcessingFailed {
                message: "Failed to create CIDetector".to_string(),
            })?;

            // Detect features
            let smile_key = NSString::from_str("CIDetectorSmile");
            let blink_key = NSString::from_str("CIDetectorEyeBlink");
            let orientation_key = NSString::from_str("CIDetectorImageOrientation");

            // Orientation 1 is standard (TopLeft)
            let orientation_val = NSNumber::numberWithInt(1);

            let mut feat_keys = vec![orientation_key];
            // NSNumber -> NSValue -> NSObject -> AnyObject
            let mut feat_objs: Vec<Retained<AnyObject>> = vec![Retained::into_super(
                Retained::into_super(Retained::into_super(orientation_val)),
            )];

            let true_val = NSNumber::numberWithBool(true);

            if options.classify_attributes {
                feat_keys.push(smile_key);
                feat_objs.push(Retained::into_super(Retained::into_super(
                    Retained::into_super(true_val.clone()),
                )));
                feat_keys.push(blink_key);
                feat_objs.push(Retained::into_super(Retained::into_super(
                    Retained::into_super(true_val),
                )));
            }

            let feat_keys_ref: Vec<&NSString> = feat_keys.iter().map(|k| &**k).collect();
            let feat_objs_ref: Vec<&AnyObject> = feat_objs.iter().map(|o| &**o).collect();

            let feat_opts =
                NSDictionary::<NSString, AnyObject>::from_slices(&feat_keys_ref, &feat_objs_ref);

            let features = detector.featuresInImage_options(&ci_image, Some(&feat_opts));
            let mut faces = Vec::new();

            for i in 0..features.count() {
                let feature_obj = features.objectAtIndex(i);

                // Downcast to CIFaceFeature
                let feature = feature_obj.downcast::<CIFaceFeature>().map_err(|_| {
                    Error::ImageProcessingFailed {
                        message: "Failed to cast feature to CIFaceFeature".to_string(),
                    }
                })?;

                // Bounds
                let bounds = feature.bounds();

                // Normalize bounds (CI coordinates are bottom-left)
                // Vision/Tauri expects top-left origin, normalized 0-1
                let x = bounds.origin.x as f32 / image_width;
                let w = bounds.size.width as f32 / image_width;
                let h = bounds.size.height as f32 / image_height;
                // Flip Y: (height - y - h) / height
                let y = (image_height - (bounds.origin.y as f32 + bounds.size.height as f32))
                    / image_height;

                let bounding_box = BoundingBox {
                    x,
                    y,
                    width: w,
                    height: h,
                };

                // Attributes
                let attributes = if options.classify_attributes {
                    let has_smile = feature.hasSmile();
                    let left_closed = feature.leftEyeClosed();
                    let right_closed = feature.rightEyeClosed();

                    Some(crate::models::FaceAttributes {
                        smiling_probability: Some(if has_smile { 1.0 } else { 0.0 }),
                        left_eye_open_probability: Some(if left_closed { 0.0 } else { 1.0 }),
                        right_eye_open_probability: Some(if right_closed { 0.0 } else { 1.0 }),
                    })
                } else {
                    None
                };

                // Angle
                let angle = if feature.hasFaceAngle() {
                    feature.faceAngle()
                } else {
                    0.0
                };

                // Landmarks
                let landmarks = if options.detect_landmarks {
                    let left_eye = if feature.hasLeftEyePosition() {
                        let p = feature.leftEyePosition();
                        Some(Point {
                            x: p.x as f32 / image_width,
                            y: (image_height - p.y as f32) / image_height,
                        })
                    } else {
                        None
                    };

                    let right_eye = if feature.hasRightEyePosition() {
                        let p = feature.rightEyePosition();
                        Some(Point {
                            x: p.x as f32 / image_width,
                            y: (image_height - p.y as f32) / image_height,
                        })
                    } else {
                        None
                    };

                    let mouth = if feature.hasMouthPosition() {
                        let p = feature.mouthPosition();
                        Some(Point {
                            x: p.x as f32 / image_width,
                            y: (image_height - p.y as f32) / image_height,
                        })
                    } else {
                        None
                    };

                    Some(FaceLandmarks {
                        left_eye,
                        right_eye,
                        nose: None,
                        mouth_left: mouth,
                        mouth_right: None,
                    })
                } else {
                    None
                };

                faces.push(Face {
                    bounding_box,
                    landmarks,
                    attributes,
                    roll_angle: Some(angle),
                    yaw_angle: None,
                });
            }

            Ok(faces)
        });

        match result {
            Ok(res) => res,
            Err(exception) => Err(Error::ImageProcessingFailed {
                message: format!(
                    "Objective-C exception during face detection: {:?}",
                    exception
                ),
            }),
        }
    }
}

/// Detect faces in an image using VNDetectFaceRectanglesRequest or CIDetector.
pub fn vision_detect_faces(image: ImageSource, options: FaceOptions) -> Result<Vec<Face>> {
    let data = decode_image_source(&image)?;

    // Use CIDetector if attributes are requested, as Vision framework doesn't support
    // smile/blink detection in the standard request.
    if options.classify_attributes {
        // We still use msg_send for CoreImage as objc2-core-image might not be full featured yet
        // or we need to switch context. But let's check imports.
        // Actually implementing performRequests_error handles the crash by catching the exception automatically in objc2.
        return detect_faces_ci_safe(&data, options);
    }

    use objc2_vision::{
        VNDetectFaceLandmarksRequest, VNDetectFaceRectanglesRequest, VNImageRequestHandler,
    };

    unsafe {
        let ns_data = NSData::with_bytes(&data);
        let empty_dict = objc2_foundation::NSDictionary::new();
        let handler = VNImageRequestHandler::initWithData_options(
            VNImageRequestHandler::alloc(),
            &ns_data,
            &empty_dict,
        );

        // We use either Landmarks request or Rectangles request.
        // Both inherit from VNImageBasedRequest -> VNRequest.
        // We need to store them as generic retained request to put in array.

        // However, they return different result types (VNFaceObservation).

        let request: Retained<objc2_vision::VNRequest>;

        let landmarks_req;
        let rects_req;

        if options.detect_landmarks {
            landmarks_req = VNDetectFaceLandmarksRequest::new();
            request = Retained::into_super(Retained::into_super(landmarks_req.clone()));
        } else {
            rects_req = VNDetectFaceRectanglesRequest::new();
            request = Retained::into_super(Retained::into_super(rects_req.clone()));
        }

        let requests =
            objc2_foundation::NSArray::from_retained_slice(std::slice::from_ref(&request));

        handler
            .performRequests_error(&requests)
            .map_err(|e| Error::ImageProcessingFailed {
                message: format!("Face detection failed: {}", e),
            })?;

        // Retrieve results from the specific request objects we created
        // We need to downcast or just access the original variables if we kept them.
        // Since we moved them or cloned them...
        // Actually, 'results' is a method on VNRequest.

        let results = request
            .results()
            .ok_or_else(|| Error::ImageProcessingFailed {
                message: "No results returned".to_string(),
            })?;

        let mut faces = Vec::new();

        for observation_obj in results {
            // Downcast to VNFaceObservation
            let observation = observation_obj
                .downcast::<objc2_vision::VNFaceObservation>()
                .ok()
                .ok_or_else(|| Error::ImageProcessingFailed {
                    message: "Failed to cast result to VNFaceObservation".to_string(),
                })?;

            let bbox = observation.boundingBox();
            let width = bbox.size.width;
            let height = bbox.size.height;
            let x = bbox.origin.x;
            let y = 1.0 - bbox.origin.y - height;

            let bounding_box = BoundingBox {
                x: x as f32,
                y: y as f32,
                width: width as f32,
                height: height as f32,
            };

            let roll_angle = observation
                .roll()
                .map(|n| n.doubleValue() as f32 * 180.0 / std::f32::consts::PI);
            let yaw_angle = observation
                .yaw()
                .map(|n| n.doubleValue() as f32 * 180.0 / std::f32::consts::PI);

            let landmarks = if options.detect_landmarks {
                observation
                    .landmarks()
                    .map(|lm| extract_face_landmarks(&lm))
            } else {
                None
            };

            faces.push(Face {
                bounding_box,
                landmarks,
                attributes: None,
                roll_angle,
                yaw_angle,
            });
        }

        Ok(faces)
    }
}

/// Extract face landmark points from VNFaceLandmarks2D into our FaceLandmarks model.
/// Computes the centroid of each region's normalized points for single-point fields.
fn extract_face_landmarks(lm: &objc2_vision::VNFaceLandmarks2D) -> FaceLandmarks {
    unsafe {
        let left_eye = region_centroid(lm.leftEye().as_deref());
        let right_eye = region_centroid(lm.rightEye().as_deref());
        let nose = region_centroid(lm.nose().as_deref());

        // For mouth corners, get the leftmost and rightmost points of outer lips
        let (mouth_left, mouth_right) = outer_lip_corners(lm.outerLips().as_deref());

        FaceLandmarks {
            left_eye,
            right_eye,
            nose,
            mouth_left,
            mouth_right,
        }
    }
}

/// Compute the centroid of a VNFaceLandmarkRegion2D's normalized points.
unsafe fn region_centroid(region: Option<&objc2_vision::VNFaceLandmarkRegion2D>) -> Option<Point> {
    let region = region?;
    let count = region.pointCount();
    if count == 0 {
        return None;
    }
    let points = region.normalizedPoints();
    let mut sum_x: f64 = 0.0;
    let mut sum_y: f64 = 0.0;
    for i in 0..count {
        let p = *points.add(i);
        sum_x += p.x;
        sum_y += p.y;
    }
    Some(Point {
        x: (sum_x / count as f64) as f32,
        y: (sum_y / count as f64) as f32,
    })
}

/// Extract leftmost and rightmost points from the outer lips region as mouth corners.
unsafe fn outer_lip_corners(
    region: Option<&objc2_vision::VNFaceLandmarkRegion2D>,
) -> (Option<Point>, Option<Point>) {
    let Some(region) = region else {
        return (None, None);
    };
    let count = region.pointCount();
    if count == 0 {
        return (None, None);
    }
    let points = region.normalizedPoints();

    let mut min_x = f64::MAX;
    let mut max_x = f64::MIN;
    let mut left_point = *points;
    let mut right_point = *points;

    for i in 0..count {
        let p = *points.add(i);
        if p.x < min_x {
            min_x = p.x;
            left_point = p;
        }
        if p.x > max_x {
            max_x = p.x;
            right_point = p;
        }
    }

    (
        Some(Point {
            x: left_point.x as f32,
            y: left_point.y as f32,
        }),
        Some(Point {
            x: right_point.x as f32,
            y: right_point.y as f32,
        }),
    )
}

/// Helper for CIDetector using objc2 catch_unwind capability implicitly via proper bindings?
/// No, objc2 msg_send! doesn't catch exceptions by default unless configured.
/// But using manual exception handling with `objc2::exception::catch`.
fn detect_faces_ci_safe(data: &[u8], options: FaceOptions) -> Result<Vec<Face>> {
    let result = std::panic::catch_unwind(|| detect_faces_ci(data, options));

    match result {
        Ok(r) => r,
        Err(_) => Err(Error::ImageProcessingFailed {
            message: "Crash during CoreImage face detection".to_string(),
        }),
    }
}

// =============================================================================
// Vision - Image Classification
// =============================================================================

/// Classify an image using VNClassifyImageRequest.
pub fn vision_classify_image(
    image: ImageSource,
    options: ClassificationOptions,
) -> Result<Vec<Classification>> {
    let data = decode_image_source(&image)?;

    use objc2_vision::{VNClassifyImageRequest, VNImageRequestHandler};

    unsafe {
        let ns_data = NSData::with_bytes(&data);

        let empty_dict = objc2_foundation::NSDictionary::new();
        let handler = VNImageRequestHandler::initWithData_options(
            VNImageRequestHandler::alloc(),
            &ns_data,
            &empty_dict,
        );

        let request = VNClassifyImageRequest::new();

        // Upcast VNClassifyImageRequest -> VNImageBasedRequest -> VNRequest
        let request_as_vn: Retained<objc2_vision::VNRequest> =
            Retained::into_super(Retained::into_super(request.clone()));
        let requests = objc2_foundation::NSArray::from_retained_slice(&[request_as_vn]);

        handler
            .performRequests_error(&requests)
            .map_err(|e| Error::ImageProcessingFailed {
                message: format!("Image classification failed: {}", e),
            })?;

        let results = request
            .results()
            .ok_or_else(|| Error::ImageProcessingFailed {
                message: "No results returned".to_string(),
            })?;

        let max_results = options.max_results.unwrap_or(10) as usize;
        let min_confidence = options.min_confidence.unwrap_or(0.0);

        let mut classifications = Vec::new();

        for observation_obj in results {
            let observation = observation_obj
                .downcast::<objc2_vision::VNClassificationObservation>()
                .ok()
                .ok_or_else(|| Error::ImageProcessingFailed {
                    message: "Failed to cast result to VNClassificationObservation".to_string(),
                })?;

            let identifier = observation.identifier().to_string();
            let confidence = observation.confidence();

            if confidence >= min_confidence {
                classifications.push(Classification {
                    identifier,
                    confidence,
                });
            }

            if classifications.len() >= max_results {
                break;
            }
        }

        // Sort by confidence descending
        classifications.sort_by(|a, b| {
            b.confidence
                .partial_cmp(&a.confidence)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        Ok(classifications)
    }
}

// =============================================================================
// LLM - Language Model (FoundationModels)
// =============================================================================
// Actual FoundationModels integration will be added via Swift FFI bridge (T-04/T-05).
// These stubs allow compilation on macOS while the bridge is being built.

use crate::models::{
    LlmAvailability, LlmGenerateOptions, LlmGenerateResult, LlmModelInfo, LlmRewriteOptions,
    LlmRewriteResult, LlmSessionId, LlmSessionOptions, LlmStreamEvent, LlmSummarizeOptions,
    LlmSummarizeResult,
};

pub fn llm_check_availability() -> Result<LlmAvailability> {
    #[cfg(has_foundation_models)]
    {
        llm_ffi::check_availability()
    }
    #[cfg(not(has_foundation_models))]
    {
        Ok(LlmAvailability {
            available: false,
            reason: Some("FoundationModels not available (requires macOS 26 SDK)".to_string()),
        })
    }
}

pub fn llm_get_model_info() -> Result<LlmModelInfo> {
    #[cfg(has_foundation_models)]
    {
        llm_ffi::get_model_info()
    }
    #[cfg(not(has_foundation_models))]
    {
        Err(Error::LlmNotAvailable {
            reason: "FoundationModels not available (requires macOS 26 SDK)".to_string(),
        })
    }
}

pub fn llm_generate(options: LlmGenerateOptions) -> Result<LlmGenerateResult> {
    #[cfg(has_foundation_models)]
    {
        llm_ffi::generate(options)
    }
    #[cfg(not(has_foundation_models))]
    {
        let _ = options;
        Err(Error::LlmNotAvailable {
            reason: "FoundationModels not available (requires macOS 26 SDK)".to_string(),
        })
    }
}

pub fn llm_generate_stream<F>(options: LlmGenerateOptions, on_event: F) -> Result<()>
where
    F: FnMut(LlmStreamEvent) -> Result<()> + 'static,
{
    #[cfg(has_foundation_models)]
    {
        llm_ffi::generate_stream(options, on_event)
    }
    #[cfg(not(has_foundation_models))]
    {
        let _ = (options, on_event);
        Err(Error::LlmNotAvailable {
            reason: "FoundationModels not available (requires macOS 26 SDK)".to_string(),
        })
    }
}

pub fn llm_create_session(options: LlmSessionOptions) -> Result<LlmSessionId> {
    #[cfg(has_foundation_models)]
    {
        llm_ffi::create_session(options)
    }
    #[cfg(not(has_foundation_models))]
    {
        let _ = options;
        Err(Error::LlmNotAvailable {
            reason: "FoundationModels not available (requires macOS 26 SDK)".to_string(),
        })
    }
}

pub fn llm_session_send(session_id: LlmSessionId, message: String) -> Result<LlmGenerateResult> {
    #[cfg(has_foundation_models)]
    {
        llm_ffi::session_send(session_id, message)
    }
    #[cfg(not(has_foundation_models))]
    {
        let _ = (session_id, message);
        Err(Error::LlmNotAvailable {
            reason: "FoundationModels not available (requires macOS 26 SDK)".to_string(),
        })
    }
}

pub fn llm_session_send_stream<F>(
    session_id: LlmSessionId,
    message: String,
    on_event: F,
) -> Result<()>
where
    F: FnMut(LlmStreamEvent) -> Result<()> + 'static,
{
    #[cfg(has_foundation_models)]
    {
        llm_ffi::session_send_stream(session_id, message, on_event)
    }
    #[cfg(not(has_foundation_models))]
    {
        let _ = (session_id, message, on_event);
        Err(Error::LlmNotAvailable {
            reason: "FoundationModels not available (requires macOS 26 SDK)".to_string(),
        })
    }
}

pub fn llm_destroy_session(session_id: LlmSessionId) -> Result<()> {
    #[cfg(has_foundation_models)]
    {
        llm_ffi::destroy_session(session_id)
    }
    #[cfg(not(has_foundation_models))]
    {
        let _ = session_id;
        Err(Error::LlmNotAvailable {
            reason: "FoundationModels not available (requires macOS 26 SDK)".to_string(),
        })
    }
}

pub fn llm_summarize(options: LlmSummarizeOptions) -> Result<LlmSummarizeResult> {
    #[cfg(has_foundation_models)]
    {
        llm_ffi::summarize(options)
    }
    #[cfg(not(has_foundation_models))]
    {
        let _ = options;
        Err(Error::LlmNotAvailable {
            reason: "FoundationModels not available (requires macOS 26 SDK)".to_string(),
        })
    }
}

pub fn llm_rewrite(options: LlmRewriteOptions) -> Result<LlmRewriteResult> {
    #[cfg(has_foundation_models)]
    {
        llm_ffi::rewrite(options)
    }
    #[cfg(not(has_foundation_models))]
    {
        let _ = options;
        Err(Error::LlmNotAvailable {
            reason: "FoundationModels not available (requires macOS 26 SDK)".to_string(),
        })
    }
}

/// FFI bridge module — compiled only when the FoundationModels SDK is present.
#[cfg(has_foundation_models)]
mod llm_ffi {
    use super::*;
    use std::ffi::{CStr, CString};
    use std::os::raw::{c_char, c_void};

    type StreamHandler = dyn FnMut(LlmStreamEvent) -> Result<()>;
    type StreamCallback = extern "C" fn(*mut c_void, *const c_char);

    struct StreamContext {
        on_event: Box<StreamHandler>,
        error: Option<Error>,
    }

    extern "C" {
        fn swift_llm_check_availability() -> *mut c_char;
        fn swift_llm_get_model_info() -> *mut c_char;
        fn swift_llm_generate(options_json: *const c_char) -> *mut c_char;
        fn swift_llm_generate_stream(
            options_json: *const c_char,
            context: *mut c_void,
            callback: StreamCallback,
        ) -> i32;
        fn swift_llm_create_session(options_json: *const c_char) -> *mut c_char;
        fn swift_llm_session_send(session_id: *const c_char, message: *const c_char)
            -> *mut c_char;
        fn swift_llm_session_send_stream(
            session_id: *const c_char,
            message: *const c_char,
            context: *mut c_void,
            callback: StreamCallback,
        ) -> i32;
        fn swift_llm_destroy_session(session_id: *const c_char) -> i32;
        fn swift_llm_free_string(ptr: *mut c_char);
    }

    /// Parse a JSON string returned from Swift FFI, freeing the C string.
    fn parse_swift_json<T: serde::de::DeserializeOwned>(ptr: *mut c_char) -> Result<T> {
        if ptr.is_null() {
            return Err(Error::LlmGenerationFailed {
                message: "Swift FFI returned null".to_string(),
            });
        }
        let json_str = unsafe {
            let s = CStr::from_ptr(ptr).to_string_lossy().into_owned();
            swift_llm_free_string(ptr);
            s
        };

        // Check for Swift-side error envelope: {"error":"..."}
        if let Ok(err_obj) = serde_json::from_str::<serde_json::Value>(&json_str) {
            if let Some(err_msg) = err_obj.get("error").and_then(|v| v.as_str()) {
                if err_msg.starts_with("Session not found") {
                    return Err(Error::LlmSessionNotFound {
                        session_id: err_msg
                            .strip_prefix("Session not found: ")
                            .unwrap_or(err_msg)
                            .to_string(),
                    });
                }
                return Err(Error::LlmGenerationFailed {
                    message: err_msg.to_string(),
                });
            }
        }

        serde_json::from_str(&json_str).map_err(|e| Error::LlmGenerationFailed {
            message: format!("Failed to parse Swift response: {e}"),
        })
    }

    fn create_stream_context<F>(event_handler: F) -> (Box<StreamContext>, *mut c_void)
    where
        F: FnMut(LlmStreamEvent) -> Result<()> + 'static,
    {
        let mut context = Box::new(StreamContext {
            on_event: Box::new(event_handler),
            error: None,
        });
        let ctx = (&mut *context) as *mut StreamContext as *mut c_void;

        (context, ctx)
    }

    extern "C" fn on_stream_event(ctx: *mut c_void, json: *const c_char) {
        if json.is_null() || ctx.is_null() {
            return;
        }

        let context = unsafe { &mut *(ctx as *mut StreamContext) };
        let json_str = unsafe { CStr::from_ptr(json).to_string_lossy() };

        if let Ok(event) = serde_json::from_str::<LlmStreamEvent>(&json_str) {
            if let Err(error) = (context.on_event)(event) {
                context.error = Some(error);
            }
        }
    }

    fn finish_stream(context: &mut StreamContext, result: i32, message: &str) -> Result<()> {
        if let Some(error) = context.error.take() {
            return Err(error);
        }

        if result == 0 {
            return Ok(());
        }

        Err(Error::LlmGenerationFailed {
            message: message.to_string(),
        })
    }

    pub fn check_availability() -> Result<LlmAvailability> {
        let ptr = unsafe { swift_llm_check_availability() };
        parse_swift_json(ptr)
    }

    pub fn get_model_info() -> Result<LlmModelInfo> {
        let ptr = unsafe { swift_llm_get_model_info() };
        parse_swift_json(ptr)
    }

    pub fn generate(options: LlmGenerateOptions) -> Result<LlmGenerateResult> {
        let json = serde_json::to_string(&options).map_err(|e| Error::LlmGenerationFailed {
            message: format!("Failed to serialize options: {e}"),
        })?;
        let c_json = CString::new(json).map_err(|e| Error::LlmGenerationFailed {
            message: format!("Invalid options string: {e}"),
        })?;
        let ptr = unsafe { swift_llm_generate(c_json.as_ptr()) };
        parse_swift_json(ptr)
    }

    pub fn generate_stream<F>(options: LlmGenerateOptions, event_handler: F) -> Result<()>
    where
        F: FnMut(LlmStreamEvent) -> Result<()> + 'static,
    {
        let json = serde_json::to_string(&options).map_err(|e| Error::LlmGenerationFailed {
            message: format!("Failed to serialize options: {e}"),
        })?;
        let c_json = CString::new(json).map_err(|e| Error::LlmGenerationFailed {
            message: format!("Invalid options string: {e}"),
        })?;

        let (mut context, ctx) = create_stream_context(event_handler);
        let result = unsafe { swift_llm_generate_stream(c_json.as_ptr(), ctx, on_stream_event) };

        finish_stream(&mut context, result, "Stream generation failed")
    }

    pub fn create_session(options: LlmSessionOptions) -> Result<LlmSessionId> {
        let json = serde_json::to_string(&options).map_err(|e| Error::LlmGenerationFailed {
            message: format!("Failed to serialize options: {e}"),
        })?;
        let c_json = CString::new(json).map_err(|e| Error::LlmGenerationFailed {
            message: format!("Invalid options string: {e}"),
        })?;
        let ptr = unsafe { swift_llm_create_session(c_json.as_ptr()) };
        parse_swift_json(ptr)
    }

    pub fn session_send(session_id: LlmSessionId, message: String) -> Result<LlmGenerateResult> {
        let c_session = CString::new(session_id).map_err(|e| Error::LlmGenerationFailed {
            message: format!("Invalid session ID: {e}"),
        })?;
        let c_message = CString::new(message).map_err(|e| Error::LlmGenerationFailed {
            message: format!("Invalid message: {e}"),
        })?;
        let ptr = unsafe { swift_llm_session_send(c_session.as_ptr(), c_message.as_ptr()) };
        parse_swift_json(ptr)
    }

    pub fn session_send_stream<F>(
        session_id: LlmSessionId,
        message: String,
        event_handler: F,
    ) -> Result<()>
    where
        F: FnMut(LlmStreamEvent) -> Result<()> + 'static,
    {
        let c_session = CString::new(session_id).map_err(|e| Error::LlmGenerationFailed {
            message: format!("Invalid session ID: {e}"),
        })?;
        let c_message = CString::new(message).map_err(|e| Error::LlmGenerationFailed {
            message: format!("Invalid message: {e}"),
        })?;

        let (mut context, ctx) = create_stream_context(event_handler);
        let result = unsafe {
            swift_llm_session_send_stream(
                c_session.as_ptr(),
                c_message.as_ptr(),
                ctx,
                on_stream_event,
            )
        };

        finish_stream(&mut context, result, "Stream session send failed")
    }

    pub fn destroy_session(session_id: LlmSessionId) -> Result<()> {
        let c_session =
            CString::new(session_id.clone()).map_err(|e| Error::LlmGenerationFailed {
                message: format!("Invalid session ID: {e}"),
            })?;
        let result = unsafe { swift_llm_destroy_session(c_session.as_ptr()) };
        if result == 0 {
            Ok(())
        } else {
            Err(Error::LlmSessionNotFound { session_id })
        }
    }

    pub fn summarize(options: LlmSummarizeOptions) -> Result<LlmSummarizeResult> {
        // On macOS, summarize is implemented via a prompted generation
        let prompt = format!(
            "Summarize the following text concisely:\n\n{}",
            options.text
        );
        let gen_options = LlmGenerateOptions {
            prompt,
            system_prompt: Some(
                "You are a text summarization assistant. Provide concise, accurate summaries."
                    .to_string(),
            ),
            temperature: Some(0.3),
            max_tokens: Some(512),
            top_p: None,
            top_k: None,
            seed: None,
        };
        let result = generate(gen_options)?;
        Ok(LlmSummarizeResult {
            summary: result.content,
            model: result.model,
        })
    }

    pub fn rewrite(options: LlmRewriteOptions) -> Result<LlmRewriteResult> {
        let tone_instruction = match options.tone {
            Some(crate::models::LlmRewriteTone::Casual) => "in a casual, friendly tone",
            Some(crate::models::LlmRewriteTone::Formal) => "in a formal, professional tone",
            Some(crate::models::LlmRewriteTone::Professional) => "in a polished, professional tone",
            None => "improving clarity and style",
        };
        let prompt = format!(
            "Rewrite the following text {tone_instruction}:\n\n{}",
            options.text
        );
        let gen_options = LlmGenerateOptions {
            prompt,
            system_prompt: Some(
                "You are a text rewriting assistant. Rewrite the text as instructed, preserving the original meaning."
                    .to_string(),
            ),
            temperature: Some(0.5),
            max_tokens: Some(1024),
            top_p: None,
            top_k: None,
            seed: None,
        };
        let result = generate(gen_options)?;
        Ok(LlmRewriteResult {
            rewritten_text: result.content,
            model: result.model,
        })
    }
}
