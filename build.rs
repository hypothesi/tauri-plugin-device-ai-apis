const COMMANDS: &[&str] = &[
    // Capabilities
    "get_capabilities",
    // Speech
    "speech_recognize",
    "speech_recognize_start",
    "speech_recognize_stop",
    "speech_synthesize",
    "speech_get_voices",
    // Vision
    "vision_recognize_text",
    "vision_detect_barcodes",
    "vision_detect_faces",
    "vision_classify_image",
    // Text
    "text_identify_language",
    "text_translate",
    // LLM
    "llm_check_availability",
    "llm_get_model_info",
    "llm_generate",
    "llm_generate_stream",
    "llm_create_session",
    "llm_session_send",
    "llm_session_send_stream",
    "llm_destroy_session",
    "llm_summarize",
    "llm_rewrite",
];

#[cfg(target_os = "macos")]
#[path = "crates/device-ai/build-support/swift-runtime-search-paths.rs"]
mod swift_runtime;

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .ios_path("ios")
        .build();

    #[cfg(target_os = "macos")]
    swift_runtime::link_swift_runtime_search_paths();
}
