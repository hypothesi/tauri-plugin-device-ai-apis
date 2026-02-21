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
#endif

// MARK: - Argument Types

class RecognitionOptionsArgs: Decodable {
    let language: String?
    let continuous: Bool?
    let interimResults: Bool?
}

class SpeechRecognizeArgs: Decodable {
    let options: RecognitionOptionsArgs?
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

class TextTranslateArgs: Decodable {
    let text: String
    let from: String
    let to: String
}

// LLM argument types

class LlmGenerateOptionsInner: Decodable {
    let prompt: String
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
}

func featureNotAvailable(_ feature: String) -> PluginError {
    return PluginError(code: "FEATURE_NOT_AVAILABLE", message: "Feature not available: \(feature)")
}

func permissionDenied(_ permission: String) -> PluginError {
    return PluginError(code: "PERMISSION_DENIED", message: "Permission denied: \(permission)")
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
    private var speechRecognizer: SFSpeechRecognizer?
    private var recognitionRequest: SFSpeechAudioBufferRecognitionRequest?
    private var recognitionTask: SFSpeechRecognitionTask?
    private var audioEngine: AVAudioEngine?
    private var activeSessions: [String: SFSpeechRecognitionTask] = [:]
    private let speechSynthesizer = AVSpeechSynthesizer()

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

        recognitionRequest.shouldReportPartialResults = false

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

        recognitionTask = recognizer.recognitionTask(with: recognitionRequest) { [weak self] result, error in
            var isFinal = false

            if let result = result {
                isFinal = result.isFinal

                if isFinal {
                    self?.stopRecording()

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
                }
            }

            if let error = error, !isFinal {
                self?.stopRecording()
                invoke.reject(speechRecognitionFailed(error.localizedDescription))
            }
        }

        // Auto-stop after 60 seconds (iOS limitation)
        DispatchQueue.main.asyncAfter(deadline: .now() + 60) { [weak self] in
            if self?.audioEngine?.isRunning == true {
                self?.recognitionRequest?.endAudio()
            }
        }
    }

    private func stopRecording() {
        audioEngine?.stop()
        audioEngine?.inputNode.removeTap(onBus: 0)
        recognitionRequest?.endAudio()
        recognitionRequest = nil
        recognitionTask = nil
    }

    @objc public func speechRecognizeStart(_ invoke: Invoke) throws {
        // For now, return not available - streaming requires more complex setup
        invoke.reject(featureNotAvailable("streamingSpeechRecognition"))
    }

    @objc public func speechRecognizeStop(_ invoke: Invoke) throws {
        invoke.reject(featureNotAvailable("streamingSpeechRecognition"))
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
            invoke.reject(PluginError(code: "INVALID_IMAGE", message: "Failed to load image"))
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
                request.recognitionLevel = .fast
            case "accurate":
                request.recognitionLevel = .accurate
            default:
                request.recognitionLevel = .accurate
            }
        } else {
            request.recognitionLevel = .accurate
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
            invoke.reject(PluginError(code: "INVALID_IMAGE", message: "Failed to load image"))
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
            invoke.reject(PluginError(code: "INVALID_IMAGE", message: "Failed to load image"))
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
            invoke.reject(PluginError(code: "INVALID_IMAGE", message: "Failed to load image"))
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
            guard let uiImage = UIImage(data: bytes) else {
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

    @objc public func textTranslate(_ invoke: Invoke) throws {
        invoke.reject(featureNotAvailable("translation - not yet implemented"))
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
                    let session: LanguageModelSession
                    if let systemPrompt = args.options.systemPrompt {
                        let instructions = Transcript.Instructions(segments: [.text(Transcript.TextSegment(content: systemPrompt))], toolDefinitions: [])
                        session = LanguageModelSession(model: model, instructions: instructions)
                    } else {
                        session = LanguageModelSession(model: model)
                    }

                    var genOpts = GenerationOptions()
                    if let temp = args.options.temperature {
                        genOpts.temperature = temp
                    }
                    if let maxTokens = args.options.maxTokens {
                        genOpts.maximumResponseTokens = maxTokens
                    }

                    let response = try await session.respond(to: args.options.prompt, options: genOpts)

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
            if let systemPrompt = args.options?.systemPrompt {
                let instructions = Transcript.Instructions(segments: [.text(Transcript.TextSegment(content: systemPrompt))], toolDefinitions: [])
                session = LanguageModelSession(model: model, instructions: instructions)
            } else {
                session = LanguageModelSession(model: model)
            }

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
