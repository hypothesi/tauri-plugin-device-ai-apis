# Changelog

## [0.3.0] - 2026-10-04

### Added

- FoundationModels native multimodal prompts (image inputs alongside text) and structured JSON schema outputs (`schema`, `schemaName`, `schemaDescription`) on macOS and iOS.
- Hybrid scaling model targets (`system` on-device and `private-cloud-compute`) with automatic context overflow detection and typed recovery errors.
- Native perception tool calling for FoundationModels on macOS and iOS, routing tool calls and executing perception tools natively.
- On-device translation via Apple `TranslationSession` on macOS and iOS with pre-flight availability checks (`text_check_translation_availability`).
- Real-time streaming speech recognition via `SpeechAnalyzer` / `AVAudioEngine` with automated silence detection and live transcript event streaming.
- Programmatic, typed error codes and details (`DeviceAiErrorCode`) across all modules without forcing raw platform UI strings on developers.
- End-to-end automated QA testing runner (`scripts/qa-runner.mjs`) and macOS TCC privacy Info.plist configuration for desktop tests.

### Changed

- Hardened release automation for signed and verified git tags with crates.io and npm Trusted Publishing idempotency.
- Aligned Tauri permissions and ACL manifests for all new translation and perception capabilities.

## [0.2.0] - 2026-08-27

### Changed

- Android OCR: ML Kit script recognizers (Japanese, Chinese, Korean, Devanagari)
  are now `compileOnly`; host apps opt in per script. Latin remains always
  bundled. Previously Japanese was hardcoded as a required dependency.
- Replaced 8 `unwrap()` calls in macOS speech recognition callbacks with
  poison-recovery pattern to prevent host-application crashes

### Added

- `OcrScript` enum (Rust, TypeScript) for Android script model selection
- `OcrOptions.script` field + `with_script()` builder method
- README section documenting Android OCR opt-in with Gradle examples
- Unit tests for `OcrScript` serialization and `OcrOptions` script field

### Fixed

- Android and iOS mobile bridge compilation
- Android OCR client creation when optional script models are supplied by the host app
- iOS speech synthesis now flushes the previous utterance before speaking
- macOS live speech recognition silence handling, authorization, and error reporting
- Windows API usage for the current `windows` crate
