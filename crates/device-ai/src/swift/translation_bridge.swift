// Swift FFI bridge for Apple Translation framework (macOS 15+ / iOS 18+).
//
// Provides C-compatible functions via @_cdecl that are called from Rust.
// Complex data (Translation result) is returned as a JSON string.
// Returned strings are allocated via strdup() and must be freed by calling
// swift_translation_free_string().

#if canImport(Translation)
import Foundation
import Translation

private struct TranslationResultDTO: Codable {
    let translatedText: String
    let sourceLanguage: String
    let targetLanguage: String
}

private struct TranslationErrorDTO: Encodable {
    let error: String
    let code: String
    let modelType: String?
    let sourceLanguage: String?
    let targetLanguage: String?
}

private func jsonCString<T: Encodable>(_ value: T) -> UnsafeMutablePointer<CChar>? {
    let encoder = JSONEncoder()
    guard let data = try? encoder.encode(value),
          let str = String(data: data, encoding: .utf8) else {
        return strdup("{\"error\":\"JSON encoding failed\"}")
    }
    return strdup(str)
}

private func errorCString(_ message: String) -> UnsafeMutablePointer<CChar> {
    let escaped = message.replacingOccurrences(of: "\"", with: "\\\"")
    return strdup("{\"error\":\"\(escaped)\"}")
}

@_cdecl("swift_translation_check_availability")
public func swift_translation_check_availability(
    sourceLang: UnsafePointer<CChar>?,
    targetLang: UnsafePointer<CChar>?
) -> Int32 {
    guard #available(macOS 15.0, iOS 18.0, *) else {
        return 2 // unsupported
    }
    guard let srcPtr = sourceLang, let tgtPtr = targetLang else {
        return 1 // framework is available
    }
    let srcStr = String(cString: srcPtr)
    let tgtStr = String(cString: tgtPtr)
    let source = Locale.Language(identifier: srcStr)
    let target = Locale.Language(identifier: tgtStr)

    var result: Int32 = 2
    let semaphore = DispatchSemaphore(value: 0)
    Task {
        let availability = LanguageAvailability()
        let status = await availability.status(from: source, to: target)
        switch status {
        case .installed:
            result = 0
        case .supported:
            result = 1
        case .unsupported:
            result = 2
        @unknown default:
            result = 2
        }
        semaphore.signal()
    }
    if semaphore.wait(timeout: .now() + 5) == .timedOut {
        return 2
    }
    return result
}

@_cdecl("swift_translation_translate")
public func swift_translation_translate(
    textPtr: UnsafePointer<CChar>,
    fromPtr: UnsafePointer<CChar>,
    toPtr: UnsafePointer<CChar>
) -> UnsafeMutablePointer<CChar> {
    guard #available(macOS 15.0, iOS 18.0, *) else {
        return errorCString("Translation framework requires macOS 15+ / iOS 18+")
    }
    let text = String(cString: textPtr)
    let from = String(cString: fromPtr)
    let to = String(cString: toPtr)

    if text.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty {
        let emptyResult = TranslationResultDTO(
            translatedText: "",
            sourceLanguage: from,
            targetLanguage: to
        )
        return jsonCString(emptyResult) ?? errorCString("JSON encoding failed")
    }

    let source = Locale.Language(identifier: from)
    let target = Locale.Language(identifier: to)
    var responseJSON: UnsafeMutablePointer<CChar>?
    let semaphore = DispatchSemaphore(value: 0)

    Task {
        let availability = LanguageAvailability()
        let status = await availability.status(from: source, to: target)

        if status == .unsupported {
            let err = TranslationErrorDTO(
                error: "Language pair from '\(from)' to '\(to)' is not supported for on-device translation",
                code: "LANGUAGE_NOT_SUPPORTED",
                modelType: "translation",
                sourceLanguage: from,
                targetLanguage: to
            )
            responseJSON = jsonCString(err)
            semaphore.signal()
            return
        }

        do {
            let session = TranslationSession(installedSource: source, target: target)
            let result = try await session.translate(text)
            let dto = TranslationResultDTO(
                translatedText: result.targetText,
                sourceLanguage: from,
                targetLanguage: to
            )
            responseJSON = jsonCString(dto)
        } catch TranslationError.notInstalled {
            let err = TranslationErrorDTO(
                error: "Translation model for '\(from)' to '\(to)' is not installed on this device",
                code: "MODEL_NOT_INSTALLED",
                modelType: "translation",
                sourceLanguage: from,
                targetLanguage: to
            )
            responseJSON = jsonCString(err)
        } catch TranslationError.unsupportedSourceLanguage {
            let err = TranslationErrorDTO(
                error: "Source language '\(from)' is not supported for on-device translation",
                code: "LANGUAGE_NOT_SUPPORTED",
                modelType: "translation",
                sourceLanguage: from,
                targetLanguage: to
            )
            responseJSON = jsonCString(err)
        } catch TranslationError.unsupportedTargetLanguage {
            let err = TranslationErrorDTO(
                error: "Target language '\(to)' is not supported for on-device translation",
                code: "LANGUAGE_NOT_SUPPORTED",
                modelType: "translation",
                sourceLanguage: from,
                targetLanguage: to
            )
            responseJSON = jsonCString(err)
        } catch TranslationError.unsupportedLanguagePairing {
            let err = TranslationErrorDTO(
                error: "Translation pair from '\(from)' to '\(to)' is not supported",
                code: "LANGUAGE_NOT_SUPPORTED",
                modelType: "translation",
                sourceLanguage: from,
                targetLanguage: to
            )
            responseJSON = jsonCString(err)
        } catch {
            let err = TranslationErrorDTO(
                error: "Translation failed: \(error.localizedDescription)",
                code: "TRANSLATION_FAILED",
                modelType: "translation",
                sourceLanguage: from,
                targetLanguage: to
            )
            responseJSON = jsonCString(err)
        }
        semaphore.signal()
    }

    _ = semaphore.wait(timeout: .now() + 30)
    return responseJSON ?? errorCString("Translation timed out or failed to produce a response")
}

@_cdecl("swift_translation_free_string")
public func swift_translation_free_string(_ ptr: UnsafeMutablePointer<CChar>?) {
    guard let ptr = ptr else { return }
    free(ptr)
}

#else

@_cdecl("swift_translation_check_availability")
public func swift_translation_check_availability(
    sourceLang: UnsafePointer<CChar>?,
    targetLang: UnsafePointer<CChar>?
) -> Int32 {
    return 2 // unsupported
}

@_cdecl("swift_translation_translate")
public func swift_translation_translate(
    textPtr: UnsafePointer<CChar>,
    fromPtr: UnsafePointer<CChar>,
    toPtr: UnsafePointer<CChar>
) -> UnsafeMutablePointer<CChar> {
    let escaped = "Translation framework not available on this platform"
    return strdup("{\"error\":\"\(escaped)\"}")
}

@_cdecl("swift_translation_free_string")
public func swift_translation_free_string(_ ptr: UnsafeMutablePointer<CChar>?) {
    guard let ptr = ptr else { return }
    free(ptr)
}

#endif
