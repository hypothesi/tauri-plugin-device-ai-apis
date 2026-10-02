export const navItems = Object.freeze([
  { id: "capabilities", icon: "lucide:layout-dashboard", label: "Capabilities" },
  { id: "speech", icon: "lucide:mic", label: "Speech" },
  { id: "vision", icon: "lucide:eye", label: "Vision" },
  { id: "text", icon: "lucide:file-text", label: "Text" },
  { id: "llm", icon: "lucide:brain-circuit", label: "LLM" },
  { id: "logs", icon: "lucide:scroll-text", label: "Activity Logs" },
  { id: "tests", icon: "lucide:flask-conical", label: "Tests" },
]);

export const capabilityItems = Object.freeze([
  { key: "speechRecognition", icon: "lucide:mic", label: "Speech recognition" },
  { key: "speechSynthesis", icon: "lucide:volume-2", label: "Speech synthesis" },
  { key: "textRecognition", icon: "lucide:file-search", label: "Text recognition" },
  { key: "barcodeDetection", icon: "lucide:scan-barcode", label: "Barcode detection" },
  { key: "faceDetection", icon: "lucide:user-square-2", label: "Face detection" },
  { key: "imageClassification", icon: "lucide:image", label: "Image classification" },
  { key: "languageIdentification", icon: "lucide:languages", label: "Language ID" },
  { key: "translation", icon: "lucide:repeat", label: "Translation" },
]);

export const recognitionLanguages = Object.freeze([
  { value: "en-US", label: "English (US)" },
  { value: "en-GB", label: "English (UK)" },
  { value: "es-ES", label: "Spanish" },
  { value: "fr-FR", label: "French" },
  { value: "de-DE", label: "German" },
  { value: "it-IT", label: "Italian" },
  { value: "ja-JP", label: "Japanese" },
  { value: "ko-KR", label: "Korean" },
  { value: "zh-CN", label: "Chinese (Simplified)" },
]);

export const sampleLanguageTexts = Object.freeze([
  { label: "English", value: "Hello, how are you today?" },
  { label: "French", value: "Bonjour, comment allez-vous?" },
  { label: "Spanish", value: "Hola, como estas?" },
  { label: "German", value: "Guten Tag, wie geht es Ihnen?" },
  { label: "Japanese", value: "Konnichiwa, ogenki desu ka?" },
  { label: "Chinese", value: "Ni hao, jin tian hao ma?" },
]);

export const rewriteTones = Object.freeze([
  { value: "casual", label: "Casual" },
  { value: "formal", label: "Formal" },
  { value: "professional", label: "Professional" },
]);

export const llmCapabilityLabels = Object.freeze([
  { key: "streaming", label: "Streaming" },
  { key: "systemPrompts", label: "System prompts" },
  { key: "temperatureControl", label: "Temperature control" },
  { key: "maxTokensControl", label: "Token limits" },
  { key: "seedSupport", label: "Seed support" },
  { key: "topPSupport", label: "Top-p" },
  { key: "topKSupport", label: "Top-k" },
  { key: "summarize", label: "Summarize" },
  { key: "rewrite", label: "Rewrite" },
]);
