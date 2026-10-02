// Swift FFI bridge for FoundationModels (macOS 26+ / iOS 26+).
//
// Provides C-compatible functions via @_cdecl that are called from Rust.
// All complex data is exchanged as JSON strings to avoid fragile C struct matching.
// Returned strings are allocated via strdup() and must be freed by calling
// swift_llm_free_string().

#if canImport(FoundationModels)
import Foundation
import FoundationModels
import Vision
import AppKit

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

private struct AnyCodableDTO: Decodable {
    let rawJSON: String

    init(from decoder: Decoder) throws {
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

private struct ImageSourceDTO: Decodable {
    let base64: String?
    let bytes: [UInt8]?
    let filePath: String?
}

private func getImageData(from dto: ImageSourceDTO) -> Data? {
    if let base64 = dto.base64 {
        let cleaned: String
        if base64.starts(with: "data:") {
            if let commaIndex = base64.firstIndex(of: ",") {
                cleaned = String(base64[base64.index(after: commaIndex)...])
            } else {
                cleaned = base64
            }
        } else {
            cleaned = base64
        }
        return Data(base64Encoded: cleaned)
    } else if let bytes = dto.bytes {
        return Data(bytes)
    } else if let filePath = dto.filePath {
        return try? Data(contentsOf: URL(fileURLWithPath: filePath))
    }
    return nil
}

private func executeOCR(on data: Data) -> String {
    guard let image = NSImage(data: data),
          let cgImage = image.cgImage(forProposedRect: nil, context: nil, hints: nil) else {
        return ""
    }
    let requestHandler = VNImageRequestHandler(cgImage: cgImage, options: [:])
    let request = VNRecognizeTextRequest()
    request.recognitionLevel = .accurate
    do {
        try requestHandler.perform([request])
        let observations = request.results as? [VNRecognizedTextObservation] ?? []
        let lines = observations.compactMap { $0.topCandidates(1).first?.string }
        return lines.joined(separator: "\n")
    } catch {
        return ""
    }
}

private func executeBarcode(on data: Data) -> String {
    guard let image = NSImage(data: data),
          let cgImage = image.cgImage(forProposedRect: nil, context: nil, hints: nil) else {
        return ""
    }
    let requestHandler = VNImageRequestHandler(cgImage: cgImage, options: [:])
    let request = VNDetectBarcodesRequest()
    do {
        try requestHandler.perform([request])
        let observations = request.results as? [VNBarcodeObservation] ?? []
        let payloads = observations.compactMap { $0.payloadStringValue }
        return payloads.joined(separator: ", ")
    } catch {
        return ""
    }
}

private func makeImageSegment(from dto: ImageSourceDTO) -> Transcript.ImageSegment? {
    if let base64 = dto.base64 {
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
    } else if let bytes = dto.bytes {
        return Transcript.ImageSegment(data: Data(bytes), mimeType: "image/jpeg")
    } else if let filePath = dto.filePath {
        guard let data = try? Data(contentsOf: URL(fileURLWithPath: filePath)) else { return nil }
        let ext = URL(fileURLWithPath: filePath).pathExtension.lowercased()
        let mime = ext == "png" ? "image/png" : (ext == "webp" ? "image/webp" : "image/jpeg")
        return Transcript.ImageSegment(data: data, mimeType: mime)
    }
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

// MARK: - Session Storage

/// Thread-safe storage for multi-turn sessions.
private final class SessionStore: @unchecked Sendable {
    static let shared = SessionStore()
    private var sessions: [String: LanguageModelSession] = [:]
    private let lock = NSLock()

    func store(_ session: LanguageModelSession, id: String) {
        lock.lock()
        defer { lock.unlock() }
        sessions[id] = session
    }

    func get(_ id: String) -> LanguageModelSession? {
        lock.lock()
        defer { lock.unlock() }
        return sessions[id]
    }

    func remove(_ id: String) -> Bool {
        lock.lock()
        defer { lock.unlock() }
        return sessions.removeValue(forKey: id) != nil
    }
}

// MARK: - Helpers

/// Return a JSON-encoded C string. Caller must free via swift_llm_free_string.
private func jsonCString<T: Encodable>(_ value: T) -> UnsafeMutablePointer<CChar>? {
    let encoder = JSONEncoder()
    guard let data = try? encoder.encode(value),
          let str = String(data: data, encoding: .utf8) else {
        return strdup("{\"error\":\"JSON encoding failed\"}")
    }
    return strdup(str)
}

/// Return a JSON error C string with optional programmatic code.
private func errorCString(_ message: String, code: String? = nil) -> UnsafeMutablePointer<CChar> {
    let escaped = message.replacingOccurrences(of: "\"", with: "\\\"")
    if let code = code {
        return strdup("{\"error\":\"\(escaped)\",\"code\":\"\(code)\"}")
    }
    return strdup("{\"error\":\"\(escaped)\"}")
}

private func mapError(_ error: Error) -> (message: String, code: String?) {
    let desc = error.localizedDescription
    let lower = desc.lowercased()
    if lower.contains("context") && (lower.contains("exceed") || lower.contains("limit") || lower.contains("length")) {
        return (desc, "LLM_CONTEXT_EXCEEDED")
    }
    if lower.contains("guardrail") || lower.contains("safety") || lower.contains("filter") {
        return (desc, "LLM_CONTENT_FILTERED")
    }
    return (desc, nil)
}

/// Decode JSON from a C string pointer.
private func decodeJSON<T: Decodable>(_ ptr: UnsafePointer<CChar>) -> T? {
    let str = String(cString: ptr)
    guard let data = str.data(using: .utf8) else { return nil }
    return try? JSONDecoder().decode(T.self, from: data)
}

/// Build GenerationOptions from parameters.
private func makeGenerationOptions(
    temperature: Double?,
    maxTokens: Int?,
    seed: UInt64?
) -> GenerationOptions {
    let sampling: GenerationOptions.SamplingMode? = seed.map {
        .random(top: 50, seed: $0)
    }
    return GenerationOptions(
        sampling: sampling,
        temperature: temperature,
        maximumResponseTokens: maxTokens
    )
}

/// Create a LanguageModelSession with optional system prompt and target.
private func makeSession(
    systemPrompt: String?,
    temperature: Double?,
    maxTokens: Int?,
    seed: UInt64?,
    modelTarget: String? = nil
) -> LanguageModelSession {
    let model = SystemLanguageModel(guardrails: .default)
    guard let systemPrompt, !systemPrompt.isEmpty else {
        return LanguageModelSession(model: model)
    }
    let segment = Transcript.TextSegment(content: systemPrompt)
    let instructions = Transcript.Instructions(
        segments: [.text(segment)],
        toolDefinitions: []
    )
    return LanguageModelSession(
        model: model,
        transcript: Transcript(entries: [.instructions(instructions)])
    )
}

/// Run an async block synchronously by blocking the calling thread.
private func runBlocking<T>(_ body: @escaping @Sendable () async throws -> T) throws -> T {
    let semaphore = DispatchSemaphore(value: 0)
    nonisolated(unsafe) var result: Swift.Result<T, Error>?
    Task {
        do {
            let value = try await body()
            result = .success(value)
        } catch {
            result = .failure(error)
        }
        semaphore.signal()
    }
    semaphore.wait()
    return try result!.get()
}

// MARK: - JSON DTOs (matching Rust camelCase serde)

private struct ToolDTO: Decodable {
    let type: String
    let name: String?
    let description: String?
    let parameters: AnyCodableDTO?

    enum CodingKeys: String, CodingKey {
        case type, name, description, parameters
    }
}

private struct ToolCallDTO: Encodable {
    let id: String
    let name: String
    let arguments: [String: String]

    enum CodingKeys: String, CodingKey {
        case id, name, arguments
    }
}

private struct GenerateOptionsDTO: Decodable {
    let prompt: String
    let images: [ImageSourceDTO]?
    let responseSchema: AnyCodableDTO?
    let modelTarget: String?
    let tools: [ToolDTO]?
    let toolChoice: String?
    let maxToolCalls: Int?
    let systemPrompt: String?
    let temperature: Double?
    let maxTokens: Int?
    let topP: Double?
    let topK: Int?
    let seed: UInt64?

    enum CodingKeys: String, CodingKey {
        case prompt, images, responseSchema, modelTarget, tools, toolChoice, maxToolCalls, systemPrompt, temperature, maxTokens, topP, topK, seed
    }
}

private struct SessionOptionsDTO: Decodable {
    let modelTarget: String?
    let tools: [ToolDTO]?
    let toolChoice: String?
    let maxToolCalls: Int?
    let systemPrompt: String?
    let temperature: Double?
    let maxTokens: Int?

    enum CodingKeys: String, CodingKey {
        case modelTarget, tools, toolChoice, maxToolCalls, systemPrompt, temperature, maxTokens
    }
}

private struct GenerateResultDTO: Encodable {
    let content: String
    let model: String
    let finishReason: String
    let toolCalls: [ToolCallDTO]
    let usage: UsageDTO?

    enum CodingKeys: String, CodingKey {
        case content, model, finishReason, toolCalls, usage
    }
}

private struct UsageDTO: Encodable {
    let promptTokens: Int?
    let completionTokens: Int?
    let totalTokens: Int?

    enum CodingKeys: String, CodingKey {
        case promptTokens, completionTokens, totalTokens
    }
}

private struct AvailabilityDTO: Encodable {
    let available: Bool
    let reason: String?
}

private struct ModelTargetInfoDTO: Encodable {
    let id: String
    let name: String
    let contextWindow: Int
    let onDevice: Bool

    enum CodingKeys: String, CodingKey {
        case id, name, contextWindow, onDevice
    }
}

private struct ModelInfoDTO: Encodable {
    let id: String
    let name: String
    let provider: String
    let contextWindow: Int
    let onDevice: Bool
    let capabilities: ModelCapabilitiesDTO
    let availableTargets: [ModelTargetInfoDTO]

    enum CodingKeys: String, CodingKey {
        case id, name, provider, contextWindow, onDevice, capabilities, availableTargets
    }
}

private struct ModelCapabilitiesDTO: Encodable {
    let streaming: Bool
    let systemPrompts: Bool
    let temperatureControl: Bool
    let maxTokensControl: Bool
    let seedSupport: Bool
    let topPSupport: Bool
    let topKSupport: Bool
    let summarize: Bool
    let rewrite: Bool
    let multimodal: Bool
    let structuredOutput: Bool
    let toolCalling: Bool

    enum CodingKeys: String, CodingKey {
        case streaming, systemPrompts, temperatureControl, maxTokensControl
        case seedSupport, topPSupport, topKSupport, summarize, rewrite
        case multimodal, structuredOutput, toolCalling
    }
}

private struct StreamToolCallDTO: Encodable {
    let type_ = "toolCall"
    let toolCall: ToolCallDTO

    enum CodingKeys: String, CodingKey {
        case type_ = "type"
        case toolCall
    }
}

private struct StreamDeltaDTO: Encodable {
    let type_ = "delta"
    let content: String

    enum CodingKeys: String, CodingKey {
        case type_ = "type"
        case content
    }
}

private struct StreamDoneDTO: Encodable {
    let type_ = "done"
    let content: String
    let finishReason: String
    let usage: UsageDTO?

    enum CodingKeys: String, CodingKey {
        case type_ = "type"
        case content, finishReason, usage
    }
}

private struct StreamErrorDTO: Encodable {
    let type_ = "error"
    let message: String

    enum CodingKeys: String, CodingKey {
        case type_ = "type"
        case message
    }
}

// MARK: - Callback Type

public typealias StreamCallback = @convention(c) (
    UnsafeMutableRawPointer,   // context (Rust-owned, passed back opaquely)
    UnsafePointer<CChar>       // JSON event string
) -> Void

// MARK: - FFI Functions

@_cdecl("swift_llm_free_string")
public func freeString(_ ptr: UnsafeMutablePointer<CChar>?) {
    free(ptr)
}

@_cdecl("swift_llm_check_availability")
public func checkAvailability() -> UnsafeMutablePointer<CChar>? {
    let model = SystemLanguageModel(guardrails: .default)
    let availability = model.availability

    let dto: AvailabilityDTO
    switch availability {
    case .available:
        dto = AvailabilityDTO(available: true, reason: nil)
    case .unavailable(let reason):
        dto = AvailabilityDTO(available: false, reason: String(describing: reason))
    @unknown default:
        dto = AvailabilityDTO(available: false, reason: "Unknown unavailability reason")
    }
    return jsonCString(dto)
}

@_cdecl("swift_llm_get_model_info")
public func getModelInfo() -> UnsafeMutablePointer<CChar>? {
    let targets = [
        ModelTargetInfoDTO(
            id: "system",
            name: "Apple Foundation Models (On-Device)",
            contextWindow: 4096,
            onDevice: true
        ),
        ModelTargetInfoDTO(
            id: "private-cloud-compute",
            name: "Apple Foundation Models (Private Cloud Compute)",
            contextWindow: 32768,
            onDevice: false
        )
    ]
    let dto = ModelInfoDTO(
        id: "apple-foundation-model",
        name: "Apple Foundation Model",
        provider: "apple-foundationmodels",
        contextWindow: 4096,
        onDevice: true,
        capabilities: ModelCapabilitiesDTO(
            streaming: true,
            systemPrompts: true,
            temperatureControl: true,
            maxTokensControl: true,
            seedSupport: true,
            topPSupport: false,
            topKSupport: false,
            summarize: true,
            rewrite: true,
            multimodal: true,
            structuredOutput: true,
            toolCalling: true
        ),
        availableTargets: targets
    )
    return jsonCString(dto)
}

@_cdecl("swift_llm_generate")
public func generate(_ optionsJSON: UnsafePointer<CChar>) -> UnsafeMutablePointer<CChar>? {
    let jsonStr = String(cString: optionsJSON)
    guard let data = jsonStr.data(using: .utf8),
          let opts = try? JSONDecoder().decode(GenerateOptionsDTO.self, from: data) else {
        return errorCString("Failed to parse generate options")
    }

    var toolCalls: [ToolCallDTO] = []
    var augmentedPrompt = opts.prompt
    if let tools = opts.tools, opts.toolChoice != "none" {
        for tool in tools {
            let toolName = (tool.name ?? tool.type).lowercased()
            if toolName == "ocr" || toolName == "perception.ocr" || tool.type == "ocr" {
                if let images = opts.images, !images.isEmpty {
                    for (idx, img) in images.enumerated() {
                        if let data = getImageData(from: img) {
                            let text = executeOCR(on: data)
                            let callId = "call_\(UUID().uuidString.prefix(8))"
                            toolCalls.append(ToolCallDTO(
                                id: callId,
                                name: "ocr",
                                arguments: ["imageIndex": String(idx)]
                            ))
                            augmentedPrompt += "\n\n[Perception Tool: OCR Result for image \(idx)]\n\(text)"
                        }
                    }
                }
            } else if toolName == "barcode" || toolName == "perception.barcode" || tool.type == "barcode" {
                if let images = opts.images, !images.isEmpty {
                    for (idx, img) in images.enumerated() {
                        if let data = getImageData(from: img) {
                            let codes = executeBarcode(on: data)
                            let callId = "call_\(UUID().uuidString.prefix(8))"
                            toolCalls.append(ToolCallDTO(
                                id: callId,
                                name: "barcode",
                                arguments: ["imageIndex": String(idx)]
                            ))
                            augmentedPrompt += "\n\n[Perception Tool: Barcode Result for image \(idx)]\n\(codes)"
                        }
                    }
                }
            }
        }
    }

    do {
        let content: String = try runBlocking {
            let session = makeSession(
                systemPrompt: opts.systemPrompt,
                temperature: opts.temperature,
                maxTokens: opts.maxTokens,
                seed: opts.seed,
                modelTarget: opts.modelTarget
            )
            let genOpts = makeGenerationOptions(
                temperature: opts.temperature,
                maxTokens: opts.maxTokens,
                seed: opts.seed
            )
            let promptText = preparePromptWithSchema(
                prompt: augmentedPrompt,
                schemaString: opts.responseSchema?.rawJSON
            )
            let _ = opts.images?.compactMap { makeImageSegment(from: $0) }
            let response = try await session.respond(to: promptText, options: genOpts)
            var resultText = response.content
            if opts.responseSchema != nil {
                resultText = sanitizeJSONOutput(resultText)
            }
            return resultText
        }
        let result = GenerateResultDTO(
            content: content,
            model: "apple-foundation-model",
            finishReason: "stop",
            toolCalls: toolCalls,
            usage: nil
        )
        return jsonCString(result)
    } catch {
        let (msg, code) = mapError(error)
        return errorCString("Generation failed: \(msg)", code: code)
    }
}

@_cdecl("swift_llm_generate_stream")
public func generateStream(
    _ optionsJSON: UnsafePointer<CChar>,
    _ context: UnsafeMutableRawPointer,
    _ callback: StreamCallback
) -> Int32 {
    let jsonStr = String(cString: optionsJSON)
    guard let data = jsonStr.data(using: .utf8),
          let opts = try? JSONDecoder().decode(GenerateOptionsDTO.self, from: data) else {
        let errorJSON = "{\"type\":\"error\",\"message\":\"Failed to parse options\"}"
        errorJSON.withCString { callback(context, $0) }
        return -1
    }

    do {
        try runBlocking {
            let session = makeSession(
                systemPrompt: opts.systemPrompt,
                temperature: opts.temperature,
                maxTokens: opts.maxTokens,
                seed: opts.seed
            )
            let genOpts = makeGenerationOptions(
                temperature: opts.temperature,
                maxTokens: opts.maxTokens,
                seed: opts.seed
            )
            var toolCalls: [ToolCallDTO] = []
            var augmentedPrompt = opts.prompt
            if let tools = opts.tools, opts.toolChoice != "none" {
                for tool in tools {
                    let toolName = (tool.name ?? tool.type).lowercased()
                    if toolName == "ocr" || toolName == "perception.ocr" || tool.type == "ocr" {
                        if let images = opts.images, !images.isEmpty {
                            for (idx, img) in images.enumerated() {
                                if let data = getImageData(from: img) {
                                    let text = executeOCR(on: data)
                                    let callId = "call_\(UUID().uuidString.prefix(8))"
                                    let tc = ToolCallDTO(id: callId, name: "ocr", arguments: ["imageIndex": String(idx)])
                                    toolCalls.append(tc)
                                    let stc = StreamToolCallDTO(toolCall: tc)
                                    if let stcJSON = jsonCString(stc) {
                                        callback(context, stcJSON)
                                        free(stcJSON)
                                    }
                                    augmentedPrompt += "\n\n[Perception Tool: OCR Result for image \(idx)]\n\(text)"
                                }
                            }
                        }
                    } else if toolName == "barcode" || toolName == "perception.barcode" || tool.type == "barcode" {
                        if let images = opts.images, !images.isEmpty {
                            for (idx, img) in images.enumerated() {
                                if let data = getImageData(from: img) {
                                    let codes = executeBarcode(on: data)
                                    let callId = "call_\(UUID().uuidString.prefix(8))"
                                    let tc = ToolCallDTO(id: callId, name: "barcode", arguments: ["imageIndex": String(idx)])
                                    toolCalls.append(tc)
                                    let stc = StreamToolCallDTO(toolCall: tc)
                                    if let stcJSON = jsonCString(stc) {
                                        callback(context, stcJSON)
                                        free(stcJSON)
                                    }
                                    augmentedPrompt += "\n\n[Perception Tool: Barcode Result for image \(idx)]\n\(codes)"
                                }
                            }
                        }
                    }
                }
            }
            let promptText = preparePromptWithSchema(
                prompt: augmentedPrompt,
                schemaString: opts.responseSchema?.rawJSON
            )
            let _ = opts.images?.compactMap { makeImageSegment(from: $0) }
            let stream = session.streamResponse(to: promptText, options: genOpts)
            var previousContent = ""

            for try await snapshot in stream {
                let currentContent = snapshot.content
                if currentContent.count > previousContent.count {
                    let idx = currentContent.index(
                        currentContent.startIndex,
                        offsetBy: previousContent.count
                    )
                    let delta = String(currentContent[idx...])
                    let deltaDTO = StreamDeltaDTO(content: delta)
                    if let deltaJSON = jsonCString(deltaDTO) {
                        callback(context, deltaJSON)
                        free(deltaJSON)
                    }
                }
                previousContent = currentContent
            }

            // Send done event
            var finalContent = previousContent
            if opts.responseSchema != nil {
                finalContent = sanitizeJSONOutput(finalContent)
            }
            let doneDTO = StreamDoneDTO(
                content: finalContent,
                finishReason: "stop",
                usage: nil
            )
            if let doneJSON = jsonCString(doneDTO) {
                callback(context, doneJSON)
                free(doneJSON)
            }
        }
        return 0
    } catch {
        let errDTO = StreamErrorDTO(message: "Stream failed: \(error.localizedDescription)")
        if let errJSON = jsonCString(errDTO) {
            callback(context, errJSON)
            free(errJSON)
        }
        return -1
    }
}

@_cdecl("swift_llm_create_session")
public func createSession(_ optionsJSON: UnsafePointer<CChar>) -> UnsafeMutablePointer<CChar>? {
    let jsonStr = String(cString: optionsJSON)
    guard let data = jsonStr.data(using: .utf8),
          let opts = try? JSONDecoder().decode(SessionOptionsDTO.self, from: data) else {
        return errorCString("Failed to parse session options")
    }

    let sessionID = UUID().uuidString
    let session = makeSession(
        systemPrompt: opts.systemPrompt,
        temperature: opts.temperature,
        maxTokens: opts.maxTokens,
        seed: nil
    )
    SessionStore.shared.store(session, id: sessionID)

    // Return the session ID as a JSON string
    return strdup("\"\(sessionID)\"")
}

@_cdecl("swift_llm_session_send")
public func sessionSend(
    _ sessionIDPtr: UnsafePointer<CChar>,
    _ messagePtr: UnsafePointer<CChar>
) -> UnsafeMutablePointer<CChar>? {
    let sessionID = String(cString: sessionIDPtr)
    let message = String(cString: messagePtr)

    guard let session = SessionStore.shared.get(sessionID) else {
        return errorCString("Session not found: \(sessionID)")
    }

    do {
        let content: String = try runBlocking {
            let response = try await session.respond(to: message)
            return response.content
        }
        let result = GenerateResultDTO(
            content: content,
            model: "apple-foundation-model",
            finishReason: "stop",
            toolCalls: [],
            usage: nil
        )
        return jsonCString(result)
    } catch {
        let (msg, code) = mapError(error)
        return errorCString("Session send failed: \(msg)", code: code)
    }
}

@_cdecl("swift_llm_session_send_stream")
public func sessionSendStream(
    _ sessionIDPtr: UnsafePointer<CChar>,
    _ messagePtr: UnsafePointer<CChar>,
    _ context: UnsafeMutableRawPointer,
    _ callback: StreamCallback
) -> Int32 {
    let sessionID = String(cString: sessionIDPtr)
    let message = String(cString: messagePtr)

    guard let session = SessionStore.shared.get(sessionID) else {
        let errDTO = StreamErrorDTO(message: "Session not found: \(sessionID)")
        if let errJSON = jsonCString(errDTO) {
            callback(context, errJSON)
            free(errJSON)
        }
        return -1
    }

    do {
        try runBlocking {
            let stream = session.streamResponse(to: message)
            var previousContent = ""

            for try await snapshot in stream {
                let currentContent = snapshot.content
                if currentContent.count > previousContent.count {
                    let idx = currentContent.index(
                        currentContent.startIndex,
                        offsetBy: previousContent.count
                    )
                    let delta = String(currentContent[idx...])
                    let deltaDTO = StreamDeltaDTO(content: delta)
                    if let deltaJSON = jsonCString(deltaDTO) {
                        callback(context, deltaJSON)
                        free(deltaJSON)
                    }
                }
                previousContent = currentContent
            }

            let doneDTO = StreamDoneDTO(
                content: previousContent,
                finishReason: "stop",
                usage: nil
            )
            if let doneJSON = jsonCString(doneDTO) {
                callback(context, doneJSON)
                free(doneJSON)
            }
        }
        return 0
    } catch {
        let errDTO = StreamErrorDTO(message: "Stream failed: \(error.localizedDescription)")
        if let errJSON = jsonCString(errDTO) {
            callback(context, errJSON)
            free(errJSON)
        }
        return -1
    }
}

@_cdecl("swift_llm_destroy_session")
public func destroySession(_ sessionIDPtr: UnsafePointer<CChar>) -> Int32 {
    let sessionID = String(cString: sessionIDPtr)
    return SessionStore.shared.remove(sessionID) ? 0 : -1
}

#else
// FoundationModels not available — provide no-op stubs so the file can still be compiled
// on older SDKs (though build.rs should skip compilation entirely in that case).
import Foundation

@_cdecl("swift_llm_free_string")
public func freeString(_ ptr: UnsafeMutablePointer<CChar>?) { free(ptr) }

@_cdecl("swift_llm_check_availability")
public func checkAvailability() -> UnsafeMutablePointer<CChar>? {
    return strdup("{\"available\":false,\"reason\":\"FoundationModels not available\"}")
}

@_cdecl("swift_llm_get_model_info")
public func getModelInfo() -> UnsafeMutablePointer<CChar>? {
    return strdup("{\"error\":\"FoundationModels not available\"}")
}

@_cdecl("swift_llm_generate")
public func generate(_ opts: UnsafePointer<CChar>) -> UnsafeMutablePointer<CChar>? {
    return strdup("{\"error\":\"FoundationModels not available\"}")
}

@_cdecl("swift_llm_generate_stream")
public func generateStream(
    _ opts: UnsafePointer<CChar>,
    _ ctx: UnsafeMutableRawPointer,
    _ cb: @convention(c) (UnsafeMutableRawPointer, UnsafePointer<CChar>) -> Void
) -> Int32 { return -1 }

@_cdecl("swift_llm_create_session")
public func createSession(_ opts: UnsafePointer<CChar>) -> UnsafeMutablePointer<CChar>? {
    return strdup("{\"error\":\"FoundationModels not available\"}")
}

@_cdecl("swift_llm_session_send")
public func sessionSend(
    _ sid: UnsafePointer<CChar>,
    _ msg: UnsafePointer<CChar>
) -> UnsafeMutablePointer<CChar>? {
    return strdup("{\"error\":\"FoundationModels not available\"}")
}

@_cdecl("swift_llm_session_send_stream")
public func sessionSendStream(
    _ sid: UnsafePointer<CChar>,
    _ msg: UnsafePointer<CChar>,
    _ ctx: UnsafeMutableRawPointer,
    _ cb: @convention(c) (UnsafeMutableRawPointer, UnsafePointer<CChar>) -> Void
) -> Int32 { return -1 }

@_cdecl("swift_llm_destroy_session")
public func destroySession(_ sid: UnsafePointer<CChar>) -> Int32 { return -1 }

#endif
