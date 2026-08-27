# Changelog

## Unreleased

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
