// Swift FFI bridge for FoundationModels (macOS 26+ / iOS 26+).
//
// Provides C-compatible functions via @_cdecl that are called from Rust.
// All complex data is exchanged as JSON strings to avoid fragile C struct matching.
// Returned strings are allocated via strdup() and must be freed by calling
// swift_llm_free_string().

#if canImport(FoundationModels)
import Foundation
import FoundationModels

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

/// Return a JSON error C string.
private func errorCString(_ message: String) -> UnsafeMutablePointer<CChar> {
    let escaped = message.replacingOccurrences(of: "\"", with: "\\\"")
    return strdup("{\"error\":\"\(escaped)\"}")
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

/// Create a LanguageModelSession with optional system prompt.
private func makeSession(
    systemPrompt: String?,
    temperature: Double?,
    maxTokens: Int?,
    seed: UInt64?
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

private struct GenerateOptionsDTO: Decodable {
    let prompt: String
    let systemPrompt: String?
    let temperature: Double?
    let maxTokens: Int?
    let topP: Double?
    let topK: Int?
    let seed: UInt64?

    enum CodingKeys: String, CodingKey {
        case prompt, systemPrompt, temperature, maxTokens, topP, topK, seed
    }
}

private struct SessionOptionsDTO: Decodable {
    let systemPrompt: String?
    let temperature: Double?
    let maxTokens: Int?

    enum CodingKeys: String, CodingKey {
        case systemPrompt, temperature, maxTokens
    }
}

private struct GenerateResultDTO: Encodable {
    let content: String
    let model: String
    let finishReason: String
    let usage: UsageDTO?

    enum CodingKeys: String, CodingKey {
        case content, model, finishReason, usage
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

private struct ModelInfoDTO: Encodable {
    let id: String
    let name: String
    let provider: String
    let contextWindow: Int
    let onDevice: Bool
    let capabilities: ModelCapabilitiesDTO

    enum CodingKeys: String, CodingKey {
        case id, name, provider, contextWindow, onDevice, capabilities
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

    enum CodingKeys: String, CodingKey {
        case streaming, systemPrompts, temperatureControl, maxTokensControl
        case seedSupport, topPSupport, topKSupport, summarize, rewrite
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
            rewrite: true
        )
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

    do {
        let content: String = try runBlocking {
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
            let response = try await session.respond(to: opts.prompt, options: genOpts)
            return response.content
        }
        let result = GenerateResultDTO(
            content: content,
            model: "apple-foundation-model",
            finishReason: "stop",
            usage: nil
        )
        return jsonCString(result)
    } catch {
        return errorCString("Generation failed: \(error.localizedDescription)")
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
            let stream = session.streamResponse(to: opts.prompt, options: genOpts)
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
            usage: nil
        )
        return jsonCString(result)
    } catch {
        return errorCString("Session send failed: \(error.localizedDescription)")
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
