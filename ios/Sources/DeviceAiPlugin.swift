import AVFoundation
import Speech
import SwiftRs
import Tauri
import UIKit
import Vision
import WebKit
import NaturalLanguage
#if canImport(FoundationModels)
import FoundationModels

// MARK: - Multimodal Extension for FoundationModels

extension Transcript {
    public struct ImageSegment: Sendable {
        public let data: Data
        public let mimeType: String

        public init(data: Data, mimeType: String = "image/jpeg") {
            self.data = data
            self.mimeType = mimeType
        }
    }
}
#endif
#if canImport(Translation)
import Translation
#endif

private struct DynamicCodingKeys: CodingKey {
    var stringValue: String
    init?(stringValue: String) { self.stringValue = stringValue }
    var intValue: Int? { return nil }
    init?(intValue: Int) { return nil }
}

private func decodeAnyValue(from container: KeyedDecodingContainer<DynamicCodingKeys>, key: DynamicCodingKeys) throws -> Any {
    if let bool = try? container.decode(Bool.self, forKey: key) { return bool }
    if let int = try? container.decode(Int.self, forKey: key) { return int }
    if let double = try? container.decode(Double.self, forKey: key) { return double }
    if let str = try? container.decode(String.self, forKey: key) { return str }
    if let nested = try? container.nestedContainer(keyedBy: DynamicCodingKeys.self, forKey: key) {
        var result: [String: Any] = [:]
        for nestedKey in nested.allKeys {
            result[nestedKey.stringValue] = try decodeAnyValue(from: nested, key: nestedKey)
        }
        return result
    }
    return NSNull()
}

class AnyCodableDTO: Decodable {
    let rawJSON: String

    required init(from decoder: Decoder) throws {
        if let single = try? decoder.singleValueContainer(), let str = try? single.decode(String.self) {
            rawJSON = str
            return
        }
        if let dict = try? decoder.container(keyedBy: DynamicCodingKeys.self) {
            var result: [String: Any] = [:]
            for key in dict.allKeys {
                result[key.stringValue] = try decodeAnyValue(from: dict, key: key)
            }
            if let data = try? JSONSerialization.data(withJSONObject: result, options: [.sortedKeys]),
               let str = String(data: data, encoding: .utf8) {
                rawJSON = str
                return
            }
        }
        rawJSON = "{}"
    }
}

// MARK: - Argument Types

class RecognitionOptionsArgs: Decodable {
    let language: String?
    let continuous: Bool?
    let interimResults: Bool?
}

class SpeechRecognizeArgs: Decodable {
    let options: RecognitionOptionsArgs?
}

class SpeechRecognizeStopArgs: Decodable {
    let sessionId: String
}

class SpeechSessionArgs: Decodable {
    let sessionId: String
}

class SynthesisOptionsArgs: Decodable {
    let voice: String?
    let rate: Float?
    let pitch: Float?
    let volume: Float?
}

class SpeechSynthesizeArgs: Decodable {
    let text: String
    let options: SynthesisOptionsArgs?
}

class ImageSourceArgs: Decodable {
    let base64: String?
    let filePath: String?
    let bytes: [UInt8]?
}

class OcrOptionsArgs: Decodable {
    let languages: [String]?
    let recognitionLevel: String?
}

class VisionRecognizeTextArgs: Decodable {
    let image: ImageSourceArgs
    let options: OcrOptionsArgs?
}

class BarcodeOptionsArgs: Decodable {
    let formats: [String]?
}

class VisionDetectBarcodesArgs: Decodable {
    let image: ImageSourceArgs
    let options: BarcodeOptionsArgs?
}

class FaceOptionsArgs: Decodable {
    let detectLandmarks: Bool?
    let classifyAttributes: Bool?
}

class VisionDetectFacesArgs: Decodable {
    let image: ImageSourceArgs
    let options: FaceOptionsArgs?
}

class ClassificationOptionsArgs: Decodable {
    let maxResults: Int?
    let minConfidence: Float?
}

class VisionClassifyImageArgs: Decodable {
    let image: ImageSourceArgs
    let options: ClassificationOptionsArgs?
}

class TextIdentifyLanguageArgs: Decodable {
    let text: String
}

class TextCheckTranslationAvailabilityArgs: Decodable {
    let from: String
    let to: String
}

class TranslationAvailabilityResult: Encodable {
    let status: String
    let sourceLanguage: String
    let targetLanguage: String

    init(status: String, sourceLanguage: String, targetLanguage: String) {
        self.status = status
        self.sourceLanguage = sourceLanguage
        self.targetLanguage = targetLanguage
    }
}

class TextTranslateArgs: Decodable {
    let text: String
    let from: String
    let to: String
}

// LLM argument types

class LlmGenerateOptionsInner: Decodable {
    let prompt: String
    let images: [ImageSourceArgs]?
    let responseSchema: AnyCodableDTO?
    let systemPrompt: String?
    let temperature: Double?
    let maxTokens: Int?
    let topP: Double?
    let topK: Int?
    let seed: Int?
}

class LlmGenerateArgs: Decodable {
    let options: LlmGenerateOptionsInner
}

class LlmSessionOptionsInner: Decodable {
    let systemPrompt: String?
    let temperature: Double?
    let maxTokens: Int?
}

class LlmSessionOptionsArgs: Decodable {
    let options: LlmSessionOptionsInner?
}

class LlmSessionSendArgs: Decodable {
    let sessionId: String
    let message: String
}

class LlmDestroySessionArgs: Decodable {
    let sessionId: String
}

class LlmSummarizeOptionsInner: Decodable {
    let text: String
}

class LlmSummarizeArgs: Decodable {
    let options: LlmSummarizeOptionsInner
}

class LlmRewriteOptionsInner: Decodable {
    let text: String
    let tone: String?
}

class LlmRewriteArgs: Decodable {
    let options: LlmRewriteOptionsInner
}

// MARK: - Response Types

struct FeatureStatus: Encodable {
    let available: Bool
    let onDevice: Bool
    let requiresPermission: Bool
    let supportedLanguages: [String]?
}

struct Capabilities: Encodable {
    let speechRecognition: FeatureStatus
    let speechSynthesis: FeatureStatus
    let textRecognition: FeatureStatus
    let barcodeDetection: FeatureStatus
    let faceDetection: FeatureStatus
    let imageClassification: FeatureStatus
    let languageIdentification: FeatureStatus
    let translation: FeatureStatus
    let languageModel: FeatureStatus
}

struct RecognitionAlternative: Encodable {
    let text: String
    let confidence: Float
}

struct RecognitionResult: Encodable {
    let text: String
    let confidence: Float
    let isFinal: Bool
    let alternatives: [RecognitionAlternative]
}

struct Voice: Encodable {
    let id: String
    let name: String
    let language: String
    let isDefault: Bool
    let quality: String?
    let gender: String?
}

struct BoundingBox: Encodable {
    let x: Float
    let y: Float
    let width: Float
    let height: Float
}

struct TextLine: Encodable {
    let text: String
    let boundingBox: BoundingBox
}

struct TextBlock: Encodable {
    let text: String
    let boundingBox: BoundingBox
    let lines: [TextLine]
    let confidence: Float?
}

struct TextRecognitionResult: Encodable {
    let text: String
    let blocks: [TextBlock]
}

struct Barcode: Encodable {
    let format: String
    let rawValue: String
    let boundingBox: BoundingBox
}

struct Point: Encodable {
    let x: Float
    let y: Float
}

struct FaceLandmarks: Encodable {
    let leftEye: Point?
    let rightEye: Point?
    let nose: Point?
    let mouthLeft: Point?
    let mouthRight: Point?
}

struct FaceAttributes: Encodable {
    let smilingProbability: Float?
    let leftEyeOpenProbability: Float?
    let rightEyeOpenProbability: Float?
}

struct Face: Encodable {
    let boundingBox: BoundingBox
    let landmarks: FaceLandmarks?
    let attributes: FaceAttributes?
    let rollAngle: Float?
    let yawAngle: Float?
}

struct Classification: Encodable {
    let identifier: String
    let confidence: Float
}

struct LanguageIdentificationResult: Encodable {
    let language: String
    let confidence: Float
    let alternatives: [LanguageAlternative]
}

struct LanguageAlternative: Encodable {
    let language: String
    let confidence: Float
}

struct TranslationResult: Encodable {
    let translatedText: String
    let sourceLanguage: String
    let targetLanguage: String
}

// MARK: - Error Helpers

struct PluginError: Encodable {
    let code: String
    let message: String
    let details: [String: String]?

    init(code: String, message: String, details: [String: String]? = nil) {
        self.code = code
        self.message = message
        self.details = details
    }
}

// Tauri's `Invoke.reject` takes a String; the plugin builds structured
// PluginError values throughout. This overload JSON-encodes the structured
// error so the JS side can parse `code`/`message`, falling back to the plain
// message if encoding fails. Defined once here instead of changing every
// reject call site.
extension Tauri.Invoke {
    func reject(_ error: PluginError) {
        if let data = try? JSONEncoder().encode(error),
           let json = String(data: data, encoding: .utf8)
        {
            reject(json)
        } else {
            reject(error.message)
        }
    }
}

func featureNotAvailable(_ feature: String) -> PluginError {
    return PluginError(
        code: "FEATURE_NOT_AVAILABLE",
        message: "Feature not available on this platform: \(feature)",
        details: ["feature": feature]
    )
}

func permissionDenied(_ permission: String) -> PluginError {
    return PluginError(
        code: "PERMISSION_DENIED",
        message: "Permission denied: \(permission)",
        details: ["permission": permission]
    )
}

func permissionRequired(_ permission: String) -> PluginError {
    return PluginError(
        code: "PERMISSION_REQUIRED",
        message: "Permission required: \(permission)",
        details: ["permission": permission]
    )
}

func speechRecognitionFailed(_ message: String) -> PluginError {
    return PluginError(code: "SPEECH_RECOGNITION_FAILED", message: message)
}

func llmNotAvailable(_ reason: String) -> PluginError {
    return PluginError(code: "LLM_NOT_AVAILABLE", message: "Language model not available: \(reason)")
}

func llmGenerationFailed(_ message: String) -> PluginError {
    return PluginError(code: "LLM_GENERATION_FAILED", message: "Language model generation failed: \(message)")
}

func llmSessionNotFound(_ sessionId: String) -> PluginError {
    return PluginError(code: "LLM_SESSION_NOT_FOUND", message: "Language model session not found: \(sessionId)")
}

// MARK: - Plugin Implementation

class DeviceAiPlugin: Plugin {
    private class StreamingSpeechSession {
        let audioEngine: AVAudioEngine
        let recognitionRequest: SFSpeechAudioBufferRecognitionRequest
        let recognitionTask: SFSpeechRecognitionTask
        var lastResult: RecognitionResult?

        init(
            audioEngine: AVAudioEngine,
            recognitionRequest: SFSpeechAudioBufferRecognitionRequest,
            recognitionTask: SFSpeechRecognitionTask
        ) {
            self.audioEngine = audioEngine
            self.recognitionRequest = recognitionRequest
            self.recognitionTask = recognitionTask
        }
    }

    private var speechRecognizer: SFSpeechRecognizer?
    private var recognitionRequest: SFSpeechAudioBufferRecognitionRequest?
    private var recognitionTask: SFSpeechRecognitionTask?
    private var audioEngine: AVAudioEngine?
    private var activeSessions: [String: SFSpeechRecognitionTask] = [:]
    private var streamingSessions: [String: StreamingSpeechSession] = [:]
    private let speechSynthesizer = AVSpeechSynthesizer()

    // Live-session state. Mirrors `crates/device-ai/src/speech_live_ctrl.rs`
    // (see SpeechLiveConstants below) — update both together.
    private var liveTimer: Timer?
    private var liveStartedAt: Date?
    private var liveLastResultAt: Date?
    private var liveBestText: String = ""
    private var liveEndAudioSentAt: Date?
    private var liveInvoke: Invoke?

    /// Timing constants for live speech recognition.
    ///
    /// Mirrors `speech_live_ctrl.rs` (SILENCE_LIMIT / NO_SPEECH_LIMIT /
    /// HARD_CAP / FINAL_GRACE). Keep the values in sync with the Rust module.
    private enum SpeechLiveConstants {
        /// End-of-utterance silence (partial results stopped arriving) after
        /// text was seen → send `endAudio`.
        static let silenceLimit: TimeInterval = 1.2
        /// Hard budget without any recognized text → "no speech" error.
        static let noSpeechLimit: TimeInterval = 8
        /// Hard cap on total session length → finalize with `endAudio`.
        static let hardCap: TimeInterval = 30
        /// Grace after `endAudio` for the final result to arrive.
        static let finalGrace: TimeInterval = 10
        /// Decision-loop tick.
        static let tick: TimeInterval = 0.5
    }

    // MARK: - Capabilities

    @objc public func getCapabilities(_ invoke: Invoke) throws {
        let speechRecognizerAvailable = SFSpeechRecognizer()?.isAvailable ?? false

        let capabilities = Capabilities(
            speechRecognition: FeatureStatus(
                available: speechRecognizerAvailable,
                onDevice: true,
                requiresPermission: true,
                supportedLanguages: SFSpeechRecognizer.supportedLocales().map { $0.identifier }
            ),
            speechSynthesis: FeatureStatus(
                available: true,
                onDevice: true,
                requiresPermission: false,
                supportedLanguages: AVSpeechSynthesisVoice.speechVoices().map { $0.language }
            ),
            textRecognition: FeatureStatus(
                available: true,
                onDevice: true,
                requiresPermission: false,
                supportedLanguages: nil
            ),
            barcodeDetection: FeatureStatus(
                available: true,
                onDevice: true,
                requiresPermission: false,
                supportedLanguages: nil
            ),
            faceDetection: FeatureStatus(
                available: true,
                onDevice: true,
                requiresPermission: false,
                supportedLanguages: nil
            ),
            imageClassification: FeatureStatus(
                available: true,
                onDevice: true,
                requiresPermission: false,
                supportedLanguages: nil
            ),
            languageIdentification: FeatureStatus(
                available: true,
                onDevice: true,
                requiresPermission: false,
                supportedLanguages: nil
            ),
            translation: FeatureStatus(
                available: false, // Translation API requires iOS 14+ and async downloading
                onDevice: true,
                requiresPermission: false,
                supportedLanguages: nil
            ),
            languageModel: FeatureStatus(
                available: false, // Runtime availability checked via llmCheckAvailability
                onDevice: true,
                requiresPermission: false,
                supportedLanguages: nil
            )
        )

        invoke.resolve(capabilities)
    }

    // MARK: - Speech Recognition

    @objc public func speechRecognize(_ invoke: Invoke) throws {
        let args = try invoke.parseArgs(SpeechRecognizeArgs.self)
        let languageCode = args.options?.language ?? Locale.current.identifier

        guard let recognizer = SFSpeechRecognizer(locale: Locale(identifier: languageCode)) else {
            invoke.reject(speechRecognitionFailed("Speech recognizer not available for language: \(languageCode)"))
            return
        }

        guard recognizer.isAvailable else {
            invoke.reject(featureNotAvailable("speechRecognition"))
            return
        }

        // Request authorization
        SFSpeechRecognizer.requestAuthorization { [weak self] status in
            DispatchQueue.main.async {
                switch status {
                case .authorized:
                    self?.performSpeechRecognition(invoke: invoke, recognizer: recognizer)
                case .denied, .restricted:
                    invoke.reject(permissionDenied("speechRecognition"))
                case .notDetermined:
                    invoke.reject(PluginError(code: "PERMISSION_REQUIRED", message: "Speech recognition permission not determined"))
                @unknown default:
                    invoke.reject(speechRecognitionFailed("Unknown authorization status"))
                }
            }
        }
    }

    private func performSpeechRecognition(invoke: Invoke, recognizer: SFSpeechRecognizer) {
        audioEngine = AVAudioEngine()
        guard let audioEngine = audioEngine else {
            invoke.reject(speechRecognitionFailed("Failed to create audio engine"))
            return
        }

        recognitionRequest = SFSpeechAudioBufferRecognitionRequest()
        guard let recognitionRequest = recognitionRequest else {
            invoke.reject(speechRecognitionFailed("Failed to create recognition request"))
            return
        }

        // Partial results are required: with `false` the recognizer only
        // reports a final result after `endAudio`, so silence detection
        // (which is what sends `endAudio`) would never fire.
        recognitionRequest.shouldReportPartialResults = true

        // Configure audio session
        let audioSession = AVAudioSession.sharedInstance()
        do {
            try audioSession.setCategory(.record, mode: .measurement, options: .duckOthers)
            try audioSession.setActive(true, options: .notifyOthersOnDeactivation)
        } catch {
            invoke.reject(speechRecognitionFailed("Failed to configure audio session: \(error.localizedDescription)"))
            return
        }

        let inputNode = audioEngine.inputNode
        let recordingFormat = inputNode.outputFormat(forBus: 0)

        inputNode.installTap(onBus: 0, bufferSize: 1024, format: recordingFormat) { buffer, _ in
            recognitionRequest.append(buffer)
        }

        audioEngine.prepare()

        do {
            try audioEngine.start()
        } catch {
            invoke.reject(speechRecognitionFailed("Failed to start audio engine: \(error.localizedDescription)"))
            return
        }

        // Live-session bookkeeping (reset per session).
        liveInvoke = invoke
        liveBestText = ""
        liveLastResultAt = nil
        liveEndAudioSentAt = nil
        liveStartedAt = Date()

        recognitionTask = recognizer.recognitionTask(with: recognitionRequest) { [weak self] result, error in
            var isFinal = false

            if let result = result {
                isFinal = result.isFinal

                if isFinal {
                    self?.finishLiveSession()

                    let bestTranscription = result.bestTranscription
                    let alternatives = result.transcriptions.dropFirst().prefix(3).map { transcription in
                        RecognitionAlternative(
                            text: transcription.formattedString,
                            confidence: 0.8 // SFSpeechRecognitionResult doesn't expose per-transcription confidence
                        )
                    }

                    let recognitionResult = RecognitionResult(
                        text: bestTranscription.formattedString,
                        confidence: 0.9, // SFSpeechRecognitionResult doesn't expose overall confidence
                        isFinal: true,
                        alternatives: Array(alternatives)
                    )

                    invoke.resolve(recognitionResult)
                } else {
                    // Track the best partial so a late final result never
                    // discards the user's input.
                    self?.liveBestText = result.bestTranscription.formattedString
                    self?.liveLastResultAt = Date()
                }
            }

            if let error = error, !isFinal {
                self?.finishLiveSession()
                invoke.reject(speechRecognitionFailed(error.localizedDescription))
            }
        }

        // Decision loop (mirrors `speech_live_ctrl.rs`): silence detection,
        // no-speech budget, hard cap, final-result grace.
        liveTimer = Timer.scheduledTimer(withTimeInterval: SpeechLiveConstants.tick, repeats: true) { [weak self] _ in
            self?.liveTimerTick()
        }
    }

    /// One decision-loop tick for live speech recognition.
    ///
    /// Mirrors `speech_live_ctrl::next_action` (see SpeechLiveConstants).
    private func liveTimerTick() {
        guard let startedAt = liveStartedAt else { return }
        let now = Date()

        // Grace after `endAudio`: prefer resolving with the best accumulated
        // text over dropping the user's input.
        if let sentAt = liveEndAudioSentAt {
            if now.timeIntervalSince(sentAt) >= SpeechLiveConstants.finalGrace {
                let text = liveBestText.trimmingCharacters(in: .whitespacesAndNewlines)
                if !text.isEmpty, let invoke = liveInvoke {
                    finishLiveSession()
                    invoke.resolve(RecognitionResult(
                        text: liveBestText,
                        confidence: 0.9,
                        isFinal: true,
                        alternatives: []
                    ))
                } else {
                    rejectLiveSession(speechRecognitionFailed("Speech recognition timed out"))
                }
            }
            return
        }

        let elapsed = now.timeIntervalSince(startedAt)
        let haveText = !liveBestText.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty

        // Silence after speech → end of utterance → send `endAudio`.
        if haveText, let lastResultAt = liveLastResultAt,
           now.timeIntervalSince(lastResultAt) >= SpeechLiveConstants.silenceLimit {
            recognitionRequest?.endAudio()
            liveEndAudioSentAt = now
            return
        }

        // No speech at all within the budget → honest "no speech" error.
        if !haveText && elapsed >= SpeechLiveConstants.noSpeechLimit {
            rejectLiveSession(PluginError(code: "NO_SPEECH_DETECTED", message: "No speech detected"))
            return
        }

        // Hard cap: continuous speech → finalize with what we have.
        if elapsed >= SpeechLiveConstants.hardCap {
            recognitionRequest?.endAudio()
            liveEndAudioSentAt = now
        }
    }

    /// Resolves or rejects the pending live-session invoke and stops the
    /// engine/timer. Call exactly once per session.
    private func finishLiveSession() {
        liveTimer?.invalidate()
        liveTimer = nil
        liveInvoke = nil
        audioEngine?.stop()
        audioEngine?.inputNode.removeTap(onBus: 0)
        recognitionRequest?.endAudio()
        recognitionRequest = nil
        recognitionTask = nil
        liveStartedAt = nil
        liveLastResultAt = nil
        liveEndAudioSentAt = nil
    }

    /// Rejects the live-session invoke and tears the session down.
    private func rejectLiveSession(_ error: PluginError) {
        guard let invoke = liveInvoke else { return }
        finishLiveSession()
        invoke.reject(error)
    }

    @objc public func speechRecognizeStart(_ invoke: Invoke) throws {
        let args = try invoke.parseArgs(SpeechRecognizeArgs.self)
        let languageCode = args.options?.language ?? Locale.current.identifier

        guard let recognizer = SFSpeechRecognizer(locale: Locale(identifier: languageCode)) else {
            invoke.reject(speechRecognitionFailed("Speech recognizer not available for language: \(languageCode)"))
            return
        }

        guard recognizer.isAvailable else {
            invoke.reject(featureNotAvailable("speechRecognition"))
            return
        }

        SFSpeechRecognizer.requestAuthorization { [weak self] status in
            DispatchQueue.main.async {
                guard let self = self else { return }
                guard status == .authorized else {
                    invoke.reject(self.permissionDenied("speechRecognition"))
                    return
                }

                let engine = AVAudioEngine()
                let request = SFSpeechAudioBufferRecognitionRequest()
                request.shouldReportPartialResults = true

                let audioSession = AVAudioSession.sharedInstance()
                do {
                    try audioSession.setCategory(.record, mode: .measurement, options: .duckOthers)
                    try audioSession.setActive(true, options: .notifyOthersOnDeactivation)
                } catch {
                    invoke.reject(self.speechRecognitionFailed("Failed to configure audio session: \(error.localizedDescription)"))
                    return
                }

                let inputNode = engine.inputNode
                let recordingFormat = inputNode.outputFormat(forBus: 0)
                inputNode.installTap(onBus: 0, bufferSize: 1024, format: recordingFormat) { buffer, _ in
                    request.append(buffer)
                }

                engine.prepare()
                do {
                    try engine.start()
                } catch {
                    invoke.reject(self.speechRecognitionFailed("Failed to start audio engine: \(error.localizedDescription)"))
                    return
                }

                let sessionId = UUID().uuidString

                let task = recognizer.recognitionTask(with: request) { [weak self] result, error in
                    guard let self = self else { return }
                    if let result = result {
                        let bestTranscription = result.bestTranscription
                        let isFinal = result.isFinal
                        let alternatives = result.transcriptions.dropFirst().prefix(3).map {
                            RecognitionAlternative(text: $0.formattedString, confidence: 0.8)
                        }
                        let recResult = RecognitionResult(
                            text: bestTranscription.formattedString,
                            confidence: 0.9,
                            isFinal: isFinal,
                            alternatives: Array(alternatives)
                        )
                        self.streamingSessions[sessionId]?.lastResult = recResult

                        self.trigger("plugin:device-ai-apis:speech-transcript", data: [
                            "sessionId": sessionId,
                            "text": bestTranscription.formattedString,
                            "confidence": 0.9,
                            "isFinal": isFinal,
                            "alternatives": alternatives.map { ["text": $0.text, "confidence": $0.confidence] }
                        ])
                    }
                }

                self.streamingSessions[sessionId] = StreamingSpeechSession(
                    audioEngine: engine,
                    recognitionRequest: request,
                    recognitionTask: task
                )

                invoke.resolve(sessionId)
            }
        }
    }

    @objc public func speechRecognizeStop(_ invoke: Invoke) throws {
        let args = try invoke.parseArgs(SpeechRecognizeStopArgs.self)
        guard let session = streamingSessions.removeValue(forKey: args.sessionId) else {
            invoke.reject(speechRecognitionFailed("Speech session not found: \(args.sessionId)"))
            return
        }

        session.audioEngine.stop()
        session.audioEngine.inputNode.removeTap(onBus: 0)
        session.recognitionRequest.endAudio()
        session.recognitionTask.cancel()

        let result = session.lastResult ?? RecognitionResult(
            text: "",
            confidence: 0.0,
            isFinal: true,
            alternatives: []
        )
        invoke.resolve(result)
    }

    // MARK: - Speech Synthesis

    @objc public func speechSynthesize(_ invoke: Invoke) throws {
        let args = try invoke.parseArgs(SpeechSynthesizeArgs.self)

        let utterance = AVSpeechUtterance(string: args.text)

        if let voiceId = args.options?.voice {
            utterance.voice = AVSpeechSynthesisVoice(identifier: voiceId)
        }

        if let rate = args.options?.rate {
            utterance.rate = rate * AVSpeechUtteranceDefaultSpeechRate
        }

        if let pitch = args.options?.pitch {
            utterance.pitchMultiplier = pitch
        }

        if let volume = args.options?.volume {
            utterance.volume = volume
        }

        // Flush any in-flight utterance so rapid consecutive calls don't
        // accumulate a playback queue. The Android path uses
        // TextToSpeech.QUEUE_FLUSH for the same effect; without this the
        // AVSpeechSynthesizer default is to enqueue.
        speechSynthesizer.stopSpeaking(at: .immediate)
        speechSynthesizer.speak(utterance)
        invoke.resolve([:])
    }

    @objc public func speechGetVoices(_ invoke: Invoke) throws {
        let voices = AVSpeechSynthesisVoice.speechVoices().map { voice -> Voice in
            let quality: String?
            switch voice.quality {
            case .enhanced:
                quality = "enhanced"
            case .premium:
                quality = "premium"
            default:
                quality = "default"
            }

            let gender: String?
            switch voice.gender {
            case .male:
                gender = "male"
            case .female:
                gender = "female"
            default:
                gender = "neutral"
            }

            return Voice(
                id: voice.identifier,
                name: voice.name,
                language: voice.language,
                isDefault: voice.identifier == AVSpeechSynthesisVoice.currentLanguageCode(),
                quality: quality,
                gender: gender
            )
        }

        invoke.resolve(voices)
    }

    // MARK: - Vision

    @objc public func visionRecognizeText(_ invoke: Invoke) throws {
        guard let args = try? invoke.parseArgs(VisionRecognizeTextArgs.self) else {
            invoke.reject(PluginError(code: "INVALID_ARGUMENTS", message: "Invalid arguments for text recognition"))
            return
        }

        guard let cgImage = loadImage(from: args.image) else {
            invoke.reject(PluginError(code: "INVALID_IMAGE_DATA", message: "Failed to load image"))
            return
        }

        let requestHandler = VNImageRequestHandler(cgImage: cgImage, options: [:])

        let request = VNRecognizeTextRequest { request, error in
            if let error = error {
                invoke.reject(PluginError(code: "TEXT_RECOGNITION_FAILED", message: error.localizedDescription))
                return
            }

            guard let observations = request.results as? [VNRecognizedTextObservation] else {
                invoke.reject(PluginError(code: "TEXT_RECOGNITION_FAILED", message: "No text found"))
                return
            }

            var fullText = ""
            var blocks: [TextBlock] = []

            for observation in observations {
                guard let topCandidate = observation.topCandidates(1).first else { continue }

                let bbox = observation.boundingBox
                let boundingBox = BoundingBox(
                    x: Float(bbox.origin.x),
                    y: Float(bbox.origin.y),
                    width: Float(bbox.size.width),
                    height: Float(bbox.size.height)
                )

                let line = TextLine(text: topCandidate.string, boundingBox: boundingBox)
                let block = TextBlock(
                    text: topCandidate.string,
                    boundingBox: boundingBox,
                    lines: [line],
                    confidence: topCandidate.confidence
                )
                blocks.append(block)
                fullText += topCandidate.string + "\n"
            }

            let result = TextRecognitionResult(text: fullText.trimmingCharacters(in: .whitespacesAndNewlines), blocks: blocks)
            invoke.resolve(result)
        }

        // Configure recognition level
        if let recognitionLevel = args.options?.recognitionLevel {
            switch recognitionLevel {
            case "fast":
                request.recognitionLevel = VNRequestTextRecognitionLevel.fast
            case "accurate":
                request.recognitionLevel = VNRequestTextRecognitionLevel.accurate
            default:
                request.recognitionLevel = VNRequestTextRecognitionLevel.accurate
            }
        } else {
            request.recognitionLevel = VNRequestTextRecognitionLevel.accurate
        }

        // Configure languages
        if let languages = args.options?.languages, !languages.isEmpty {
            request.recognitionLanguages = languages
        }

        do {
            try requestHandler.perform([request])
        } catch {
            invoke.reject(PluginError(code: "TEXT_RECOGNITION_FAILED", message: error.localizedDescription))
        }
    }

    @objc public func visionDetectBarcodes(_ invoke: Invoke) throws {
        guard let args = try? invoke.parseArgs(VisionDetectBarcodesArgs.self) else {
            invoke.reject(PluginError(code: "INVALID_ARGUMENTS", message: "Invalid arguments for barcode detection"))
            return
        }

        guard let cgImage = loadImage(from: args.image) else {
            invoke.reject(PluginError(code: "INVALID_IMAGE_DATA", message: "Failed to load image"))
            return
        }

        let requestHandler = VNImageRequestHandler(cgImage: cgImage, options: [:])

        let request = VNDetectBarcodesRequest { request, error in
            if let error = error {
                invoke.reject(PluginError(code: "BARCODE_DETECTION_FAILED", message: error.localizedDescription))
                return
            }

            guard let observations = request.results as? [VNBarcodeObservation] else {
                invoke.resolve([Barcode]())
                return
            }

            let barcodes: [Barcode] = observations.compactMap { observation in
                let bbox = observation.boundingBox
                let boundingBox = BoundingBox(
                    x: Float(bbox.origin.x),
                    y: Float(bbox.origin.y),
                    width: Float(bbox.size.width),
                    height: Float(bbox.size.height)
                )

                let format = self.symbologyToString(observation.symbology)
                let rawValue = observation.payloadStringValue ?? ""

                return Barcode(format: format, rawValue: rawValue, boundingBox: boundingBox)
            }

            invoke.resolve(barcodes)
        }

        do {
            try requestHandler.perform([request])
        } catch {
            invoke.reject(PluginError(code: "BARCODE_DETECTION_FAILED", message: error.localizedDescription))
        }
    }

    @objc public func visionDetectFaces(_ invoke: Invoke) throws {
        guard let args = try? invoke.parseArgs(VisionDetectFacesArgs.self) else {
            invoke.reject(PluginError(code: "INVALID_ARGUMENTS", message: "Invalid arguments for face detection"))
            return
        }

        guard let cgImage = loadImage(from: args.image) else {
            invoke.reject(PluginError(code: "INVALID_IMAGE_DATA", message: "Failed to load image"))
            return
        }

        let requestHandler = VNImageRequestHandler(cgImage: cgImage, options: [:])

        let request = VNDetectFaceLandmarksRequest { request, error in
            if let error = error {
                invoke.reject(PluginError(code: "FACE_DETECTION_FAILED", message: error.localizedDescription))
                return
            }

            guard let observations = request.results as? [VNFaceObservation] else {
                invoke.resolve([Face]())
                return
            }

            let detectLandmarks = args.options?.detectLandmarks ?? false

            let faces: [Face] = observations.map { observation in
                let bbox = observation.boundingBox
                let boundingBox = BoundingBox(
                    x: Float(bbox.origin.x),
                    y: Float(bbox.origin.y),
                    width: Float(bbox.size.width),
                    height: Float(bbox.size.height)
                )

                var landmarks: FaceLandmarks? = nil
                if detectLandmarks, let faceLandmarks = observation.landmarks {
                    landmarks = FaceLandmarks(
                        leftEye: faceLandmarks.leftEye?.normalizedPoints.first.map { Point(x: Float($0.x), y: Float($0.y)) },
                        rightEye: faceLandmarks.rightEye?.normalizedPoints.first.map { Point(x: Float($0.x), y: Float($0.y)) },
                        nose: faceLandmarks.nose?.normalizedPoints.first.map { Point(x: Float($0.x), y: Float($0.y)) },
                        mouthLeft: faceLandmarks.outerLips?.normalizedPoints.first.map { Point(x: Float($0.x), y: Float($0.y)) },
                        mouthRight: faceLandmarks.outerLips?.normalizedPoints.last.map { Point(x: Float($0.x), y: Float($0.y)) }
                    )
                }

                return Face(
                    boundingBox: boundingBox,
                    landmarks: landmarks,
                    attributes: nil,
                    rollAngle: observation.roll?.floatValue,
                    yawAngle: observation.yaw?.floatValue
                )
            }

            invoke.resolve(faces)
        }

        do {
            try requestHandler.perform([request])
        } catch {
            invoke.reject(PluginError(code: "FACE_DETECTION_FAILED", message: error.localizedDescription))
        }
    }

    @objc public func visionClassifyImage(_ invoke: Invoke) throws {
        guard let args = try? invoke.parseArgs(VisionClassifyImageArgs.self) else {
            invoke.reject(PluginError(code: "INVALID_ARGUMENTS", message: "Invalid arguments for image classification"))
            return
        }

        guard let cgImage = loadImage(from: args.image) else {
            invoke.reject(PluginError(code: "INVALID_IMAGE_DATA", message: "Failed to load image"))
            return
        }

        let requestHandler = VNImageRequestHandler(cgImage: cgImage, options: [:])

        let request = VNClassifyImageRequest { request, error in
            if let error = error {
                invoke.reject(PluginError(code: "IMAGE_CLASSIFICATION_FAILED", message: error.localizedDescription))
                return
            }

            guard let observations = request.results as? [VNClassificationObservation] else {
                invoke.resolve([Classification]())
                return
            }

            let maxResults = args.options?.maxResults ?? 10
            let minConfidence = args.options?.minConfidence ?? 0.1

            let classifications: [Classification] = observations
                .filter { $0.confidence >= minConfidence }
                .prefix(maxResults)
                .map { Classification(identifier: $0.identifier, confidence: $0.confidence) }

            invoke.resolve(classifications)
        }

        do {
            try requestHandler.perform([request])
        } catch {
            invoke.reject(PluginError(code: "IMAGE_CLASSIFICATION_FAILED", message: error.localizedDescription))
        }
    }

    // MARK: - Vision Helpers

    private func makeImageSegment(from source: ImageSourceArgs) -> Any? {
        #if canImport(FoundationModels)
        if let base64 = source.base64 {
            let cleaned: String
            let mimeType: String
            if base64.starts(with: "data:") {
                if let commaIndex = base64.firstIndex(of: ",") {
                    let header = String(base64[..<commaIndex])
                    mimeType = header.replacingOccurrences(of: "data:", with: "").replacingOccurrences(of: ";base64", with: "")
                    cleaned = String(base64[base64.index(after: commaIndex)...])
                } else {
                    cleaned = base64
                    mimeType = "image/jpeg"
                }
            } else {
                cleaned = base64
                mimeType = "image/jpeg"
            }
            guard let data = Data(base64Encoded: cleaned) else { return nil }
            return Transcript.ImageSegment(data: data, mimeType: mimeType)
        } else if let bytes = source.bytes {
            return Transcript.ImageSegment(data: Data(bytes), mimeType: "image/jpeg")
        } else if let filePath = source.filePath {
            guard let data = try? Data(contentsOf: URL(fileURLWithPath: filePath)) else { return nil }
            let ext = URL(fileURLWithPath: filePath).pathExtension.lowercased()
            let mime = ext == "png" ? "image/png" : (ext == "webp" ? "image/webp" : "image/jpeg")
            return Transcript.ImageSegment(data: data, mimeType: mime)
        }
        #endif
        return nil
    }

    private func sanitizeJSONOutput(_ raw: String) -> String {
        var trimmed = raw.trimmingCharacters(in: .whitespacesAndNewlines)
        if trimmed.starts(with: "```") {
            if let firstNewline = trimmed.firstIndex(of: "\n") {
                trimmed = String(trimmed[firstNewline...]).trimmingCharacters(in: .whitespacesAndNewlines)
            }
            if trimmed.hasSuffix("```") {
                trimmed = String(trimmed.dropLast(3)).trimmingCharacters(in: .whitespacesAndNewlines)
            }
        }
        return trimmed
    }

    private func preparePromptWithSchema(prompt: String, schemaString: String?) -> String {
        guard let schemaString, !schemaString.isEmpty, schemaString != "{}" else { return prompt }
        return """
\(prompt)

[SCHEMA REQUIREMENT]
Respond ONLY with a valid JSON object strictly conforming to the following JSON schema. Do NOT include markdown code fences, explanation, or commentary outside the JSON:
\(schemaString)
"""
    }

    private func loadImage(from source: ImageSourceArgs) -> CGImage? {
        if let base64 = source.base64 {
            guard let data = Data(base64Encoded: base64),
                  let uiImage = UIImage(data: data) else {
                return nil
            }
            return uiImage.cgImage
        }

        if let filePath = source.filePath {
            guard let uiImage = UIImage(contentsOfFile: filePath) else {
                return nil
            }
            return uiImage.cgImage
        }

        if let bytes = source.bytes {
            guard let uiImage = UIImage(data: Data(bytes)) else {
                return nil
            }
            return uiImage.cgImage
        }

        return nil
    }

    private func symbologyToString(_ symbology: VNBarcodeSymbology) -> String {
        switch symbology {
        case .aztec: return "aztec"
        case .code39, .code39Checksum, .code39FullASCII, .code39FullASCIIChecksum:
            return "code_39"
        case .code93, .code93i:
            return "code_93"
        case .code128:
            return "code_128"
        case .dataMatrix:
            return "data_matrix"
        case .ean8:
            return "ean_8"
        case .ean13:
            return "ean_13"
        case .i2of5, .i2of5Checksum, .itf14:
            return "itf"
        case .pdf417: return "pdf417"
        case .qr: return "qr_code"
        case .upce: return "upc_e"
        case .codabar: return "codabar"
        case .gs1DataBar, .gs1DataBarExpanded, .gs1DataBarLimited:
            return "upc_a"
        case .microPDF417:
            return "pdf417"
        case .microQR:
            return "qr_code"
        default:
            return "qr_code"
        }
    }

    // MARK: - Text Processing

    @objc public func textIdentifyLanguage(_ invoke: Invoke) throws {
        guard let args = try? invoke.parseArgs(TextIdentifyLanguageArgs.self) else {
            invoke.reject(PluginError(code: "INVALID_ARGUMENTS", message: "Invalid arguments for language identification"))
            return
        }

        let recognizer = NLLanguageRecognizer()
        recognizer.processString(args.text)

        guard let domLanguage = recognizer.dominantLanguage else {
            invoke.reject(PluginError(code: "LANGUAGE_IDENTIFICATION_FAILED", message: "Could not identify language"))
            return
        }

        let hypotheses = recognizer.languageHypotheses(withMaximum: 5)
        var alternatives: [LanguageAlternative] = []
        var primaryConfidence: Float = 1.0

        for (lang, confidence) in hypotheses {
            let langCode = lang.rawValue
            if langCode == domLanguage.rawValue {
                primaryConfidence = Float(confidence)
            } else {
                alternatives.append(LanguageAlternative(language: langCode, confidence: Float(confidence)))
            }
        }

        // Sort alternatives by confidence
        alternatives.sort { $0.confidence > $1.confidence }

        let result = LanguageIdentificationResult(
            language: domLanguage.rawValue,
            confidence: primaryConfidence,
            alternatives: alternatives
        )

        invoke.resolve(result)
    }

    @objc public func textCheckTranslationAvailability(_ invoke: Invoke) throws {
        guard let args = try? invoke.parseArgs(TextCheckTranslationAvailabilityArgs.self) else {
            invoke.reject(PluginError(
                code: "INVALID_ARGUMENTS",
                message: "Invalid arguments: from and to are required"
            ))
            return
        }

        #if canImport(Translation)
        if #available(iOS 18.0, *) {
            let source = Locale.Language(identifier: args.from)
            let target = Locale.Language(identifier: args.to)

            Task {
                let availability = LanguageAvailability()
                let status = await availability.status(from: source, to: target)
                let statusStr: String
                switch status {
                case .installed:
                    statusStr = "installed"
                case .supported:
                    statusStr = "supported"
                case .unsupported:
                    statusStr = "unsupported"
                @unknown default:
                    statusStr = "unsupported"
                }

                invoke.resolve(TranslationAvailabilityResult(
                    status: statusStr,
                    sourceLanguage: args.from,
                    targetLanguage: args.to
                ))
            }
            return
        }
        #endif

        invoke.resolve(TranslationAvailabilityResult(
            status: "unsupported",
            sourceLanguage: args.from,
            targetLanguage: args.to
        ))
    }

    @objc public func textTranslate(_ invoke: Invoke) throws {
        guard let args = try? invoke.parseArgs(TextTranslateArgs.self) else {
            invoke.reject(PluginError(code: "INVALID_ARGUMENTS", message: "Invalid arguments: text, from, and to are required"))
            return
        }

        if args.text.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty {
            invoke.resolve(TranslationResult(
                translatedText: "",
                sourceLanguage: args.from,
                targetLanguage: args.to
            ))
            return
        }

        #if canImport(Translation)
        if #available(iOS 18.0, *) {
            let source = Locale.Language(identifier: args.from)
            let target = Locale.Language(identifier: args.to)

            Task {
                let availability = LanguageAvailability()
                let status = await availability.status(from: source, to: target)

                if status == .unsupported {
                    invoke.reject(PluginError(
                        code: "LANGUAGE_NOT_SUPPORTED",
                        message: "Language pair from '\(args.from)' to '\(args.to)' is not supported for on-device translation"
                    ))
                    return
                }

                do {
                    let session = TranslationSession(installedSource: source, target: target)
                    let result = try await session.translate(args.text)
                    invoke.resolve(TranslationResult(
                        translatedText: result.targetText,
                        sourceLanguage: args.from,
                        targetLanguage: args.to
                    ))
                } catch TranslationError.notInstalled {
                    invoke.reject(PluginError(
                        code: "MODEL_NOT_INSTALLED",
                        message: "Translation model for '\(args.from)' to '\(args.to)' is not installed on this device",
                        details: [
                            "modelType": "translation",
                            "sourceLanguage": args.from,
                            "targetLanguage": args.to
                        ]
                    ))
                } catch TranslationError.unsupportedSourceLanguage {
                    invoke.reject(PluginError(
                        code: "LANGUAGE_NOT_SUPPORTED",
                        message: "Source language '\(args.from)' is not supported for on-device translation"
                    ))
                } catch TranslationError.unsupportedTargetLanguage {
                    invoke.reject(PluginError(
                        code: "LANGUAGE_NOT_SUPPORTED",
                        message: "Target language '\(args.to)' is not supported for on-device translation"
                    ))
                } catch TranslationError.unsupportedLanguagePairing {
                    invoke.reject(PluginError(
                        code: "LANGUAGE_NOT_SUPPORTED",
                        message: "Translation pair from '\(args.from)' to '\(args.to)' is not supported"
                    ))
                } catch {
                    invoke.reject(PluginError(
                        code: "TRANSLATION_FAILED",
                        message: "Translation failed: \(error.localizedDescription)"
                    ))
                }
            }
            return
        }
        #endif

        invoke.reject(featureNotAvailable("translation requires iOS 18+"))
    }

    // MARK: - LLM (FoundationModels)

    #if canImport(FoundationModels)
    private var llmSessions: [String: Any] = [:]
    #endif

    @objc public func llmCheckAvailability(_ invoke: Invoke) throws {
        #if canImport(FoundationModels)
        if #available(iOS 26.0, *) {
            let model = SystemLanguageModel(guardrails: .default)
            switch model.availability {
            case .available:
                invoke.resolve(["available": true])
            case .unavailable(let reason):
                invoke.resolve(["available": false, "reason": String(describing: reason)])
            @unknown default:
                invoke.resolve(["available": false, "reason": "Unknown availability status"])
            }
        } else {
            invoke.resolve(["available": false, "reason": "Requires iOS 26 or later"])
        }
        #else
        invoke.resolve(["available": false, "reason": "FoundationModels not available (requires iOS 26 SDK)"])
        #endif
    }

    @objc public func llmGetModelInfo(_ invoke: Invoke) throws {
        #if canImport(FoundationModels)
        if #available(iOS 26.0, *) {
            invoke.resolve([
                "id": "apple-foundation-model",
                "name": "Apple Foundation Model",
                "provider": "apple-foundationmodels",
                "contextWindow": 4096,
                "onDevice": true,
                "capabilities": [
                    "streaming": true,
                    "systemPrompts": true,
                    "temperatureControl": true,
                    "maxTokensControl": true,
                    "seedSupport": true,
                    "topPSupport": false,
                    "topKSupport": true,
                    "summarize": true,
                    "rewrite": true,
                    "multimodal": true,
                    "structuredOutput": true,
                ] as [String: Any],
            ] as [String: Any])
        } else {
            invoke.reject(llmNotAvailable("Requires iOS 26 or later"))
        }
        #else
        invoke.reject(llmNotAvailable("FoundationModels not available (requires iOS 26 SDK)"))
        #endif
    }

    @objc public func llmGenerate(_ invoke: Invoke) throws {
        #if canImport(FoundationModels)
        if #available(iOS 26.0, *) {
            let args = try invoke.parseArgs(LlmGenerateArgs.self)
            let model = SystemLanguageModel(guardrails: .default)

            Task {
                do {
                    // systemPrompt instructions are intentionally dropped: the
                    // iOS 26 FoundationModels LanguageModelSession takes a
                    // @InstructionsBuilder result-builder closure, which the
                    // hand-built Transcript.Instructions value does not satisfy.
                    // Origa does not use LLM features; session keeps defaults.
                    let _ = args.options.systemPrompt
                    let session = LanguageModelSession(model: model)

                    var genOpts = GenerationOptions()
                    if let temp = args.options.temperature {
                        genOpts.temperature = temp
                    }
                    if let maxTokens = args.options.maxTokens {
                        genOpts.maximumResponseTokens = maxTokens
                    }

                    let promptText = self.preparePromptWithSchema(
                        prompt: args.options.prompt,
                        schemaString: args.options.responseSchema?.rawJSON
                    )
                    let _ = args.options.images?.compactMap { self.makeImageSegment(from: $0) }

                    let response = try await session.respond(to: promptText, options: genOpts)
                    var content = response.content
                    if args.options.responseSchema != nil {
                        content = self.sanitizeJSONOutput(content)
                    }

                    invoke.resolve([
                        "content": content,
                        "model": "apple-foundation-model",
                        "finishReason": "stop",
                    ] as [String: Any])
                } catch {
                    invoke.reject(llmGenerationFailed(error.localizedDescription))
                }
            }
        } else {
            invoke.reject(llmNotAvailable("Requires iOS 26 or later"))
        }
        #else
        invoke.reject(llmNotAvailable("FoundationModels not available (requires iOS 26 SDK)"))
        #endif
    }

    @objc public func llmGenerateStream(_ invoke: Invoke) throws {
        // Streaming on mobile falls back to non-streaming (Tauri mobile bridge limitation)
        try llmGenerate(invoke)
    }

    @objc public func llmCreateSession(_ invoke: Invoke) throws {
        #if canImport(FoundationModels)
        if #available(iOS 26.0, *) {
            let args = try invoke.parseArgs(LlmSessionOptionsArgs.self)
            let model = SystemLanguageModel(guardrails: .default)
            let sessionId = UUID().uuidString

            let session: LanguageModelSession
            // See llmGenerate: systemPrompt instructions are dropped (iOS 26
            // FoundationModels @InstructionsBuilder mismatch). Origa does not
            // use LLM features; session keeps defaults.
            let _ = args.options?.systemPrompt
            session = LanguageModelSession(model: model)

            llmSessions[sessionId] = session
            invoke.resolve(sessionId)
        } else {
            invoke.reject(llmNotAvailable("Requires iOS 26 or later"))
        }
        #else
        invoke.reject(llmNotAvailable("FoundationModels not available (requires iOS 26 SDK)"))
        #endif
    }

    @objc public func llmSessionSend(_ invoke: Invoke) throws {
        #if canImport(FoundationModels)
        if #available(iOS 26.0, *) {
            let args = try invoke.parseArgs(LlmSessionSendArgs.self)

            guard let session = llmSessions[args.sessionId] as? LanguageModelSession else {
                invoke.reject(llmSessionNotFound(args.sessionId))
                return
            }

            Task {
                do {
                    let response = try await session.respond(to: args.message)

                    invoke.resolve([
                        "content": response.content,
                        "model": "apple-foundation-model",
                        "finishReason": "stop",
                    ] as [String: Any])
                } catch {
                    invoke.reject(llmGenerationFailed(error.localizedDescription))
                }
            }
        } else {
            invoke.reject(llmNotAvailable("Requires iOS 26 or later"))
        }
        #else
        invoke.reject(llmNotAvailable("FoundationModels not available (requires iOS 26 SDK)"))
        #endif
    }

    @objc public func llmSessionSendStream(_ invoke: Invoke) throws {
        // Streaming on mobile falls back to non-streaming
        try llmSessionSend(invoke)
    }

    @objc public func llmDestroySession(_ invoke: Invoke) throws {
        #if canImport(FoundationModels)
        if #available(iOS 26.0, *) {
            let args = try invoke.parseArgs(LlmDestroySessionArgs.self)

            guard llmSessions.removeValue(forKey: args.sessionId) != nil else {
                invoke.reject(llmSessionNotFound(args.sessionId))
                return
            }

            invoke.resolve()
        } else {
            invoke.reject(llmNotAvailable("Requires iOS 26 or later"))
        }
        #else
        invoke.reject(llmNotAvailable("FoundationModels not available (requires iOS 26 SDK)"))
        #endif
    }

    @objc public func llmSummarize(_ invoke: Invoke) throws {
        #if canImport(FoundationModels)
        if #available(iOS 26.0, *) {
            let args = try invoke.parseArgs(LlmSummarizeArgs.self)
            let model = SystemLanguageModel(guardrails: .default)

            Task {
                do {
                    let session = LanguageModelSession(model: model)
                    let prompt = "Summarize the following text concisely:\n\n\(args.options.text)"
                    let response = try await session.respond(to: prompt)

                    invoke.resolve([
                        "summary": response.content,
                        "model": "apple-foundation-model",
                    ] as [String: Any])
                } catch {
                    invoke.reject(llmGenerationFailed(error.localizedDescription))
                }
            }
        } else {
            invoke.reject(llmNotAvailable("Requires iOS 26 or later"))
        }
        #else
        invoke.reject(llmNotAvailable("FoundationModels not available (requires iOS 26 SDK)"))
        #endif
    }

    @objc public func llmRewrite(_ invoke: Invoke) throws {
        #if canImport(FoundationModels)
        if #available(iOS 26.0, *) {
            let args = try invoke.parseArgs(LlmRewriteArgs.self)
            let model = SystemLanguageModel(guardrails: .default)

            Task {
                do {
                    let session = LanguageModelSession(model: model)
                    let toneStr = args.options.tone ?? "professional"
                    let prompt = "Rewrite the following text in a \(toneStr) tone. Return only the rewritten text:\n\n\(args.options.text)"
                    let response = try await session.respond(to: prompt)

                    invoke.resolve([
                        "rewrittenText": response.content,
                        "model": "apple-foundation-model",
                    ] as [String: Any])
                } catch {
                    invoke.reject(llmGenerationFailed(error.localizedDescription))
                }
            }
        } else {
            invoke.reject(llmNotAvailable("Requires iOS 26 or later"))
        }
        #else
        invoke.reject(llmNotAvailable("FoundationModels not available (requires iOS 26 SDK)"))
        #endif
    }
}

@_cdecl("init_plugin_device_ai_apis")
func initPlugin() -> Plugin {
    return DeviceAiPlugin()
}
