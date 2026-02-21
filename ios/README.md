# iOS bridge

This directory contains the Swift side of the Tauri mobile plugin.

- `Sources/DeviceAiPlugin.swift` implements the iOS bridge used by the root
  `tauri-plugin-device-ai-apis` crate
- iOS still routes through this plugin-hosted bridge after the `crates/device-ai`
  extraction; the extracted Rust crate does not currently provide direct iOS-native
  implementations

Current iOS support includes speech, vision, language identification, and conditional
FoundationModels-backed LLM support. Translation and native streaming speech recognition
are not implemented.
