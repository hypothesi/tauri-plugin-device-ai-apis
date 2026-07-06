<script>
  import Icon from '@iconify/svelte'
  import alertTriangleIcon from '@iconify-icons/lucide/alert-triangle'
  import badgeInfoIcon from '@iconify-icons/lucide/badge-info'
  import barChartIcon from '@iconify-icons/lucide/bar-chart-3'
  import botIcon from '@iconify-icons/lucide/bot'
  import brainIcon from '@iconify-icons/lucide/brain'
  import checkCircleIcon from '@iconify-icons/lucide/check-circle'
  import clipboardListIcon from '@iconify-icons/lucide/clipboard-list'
  import cloudIcon from '@iconify-icons/lucide/cloud'
  import eyeIcon from '@iconify-icons/lucide/eye'
  import fileTextIcon from '@iconify-icons/lucide/file-text'
  import flaskConicalIcon from '@iconify-icons/lucide/flask-conical'
  import globeIcon from '@iconify-icons/lucide/globe-2'
  import hourglassIcon from '@iconify-icons/lucide/hourglass'
  import languagesIcon from '@iconify-icons/lucide/languages'
  import lockIcon from '@iconify-icons/lucide/lock'
  import mapPinIcon from '@iconify-icons/lucide/map-pin'
  import messageCircleIcon from '@iconify-icons/lucide/message-circle'
  import micIcon from '@iconify-icons/lucide/mic'
  import mic2Icon from '@iconify-icons/lucide/mic-2'
  import moveHorizontalIcon from '@iconify-icons/lucide/move-horizontal'
  import pencilIcon from '@iconify-icons/lucide/pencil'
  import playIcon from '@iconify-icons/lucide/play'
  import plusCircleIcon from '@iconify-icons/lucide/plus-circle'
  import refreshCwIcon from '@iconify-icons/lucide/refresh-cw'
  import rotateCwIcon from '@iconify-icons/lucide/rotate-cw'
  import scanFaceIcon from '@iconify-icons/lucide/scan-face'
  import scrollTextIcon from '@iconify-icons/lucide/scroll-text'
  import searchIcon from '@iconify-icons/lucide/search'
  import shieldCheckIcon from '@iconify-icons/lucide/shield-check'
  import smileIcon from '@iconify-icons/lucide/smile'
  import squareIcon from '@iconify-icons/lucide/square'
  import tagIcon from '@iconify-icons/lucide/tag'
  import userIcon from '@iconify-icons/lucide/user'
  import volume2Icon from '@iconify-icons/lucide/volume-2'
  import wavesIcon from '@iconify-icons/lucide/waves'
  import wrenchIcon from '@iconify-icons/lucide/wrench'
  import xIcon from '@iconify-icons/lucide/x'
  import xCircleIcon from '@iconify-icons/lucide/x-circle'

  import {
    getCapabilities,
    speech,
    vision,
    text,
    llm,
    isTauri,
    isWeb,
    hasWebSpeechRecognition,
    hasWebSpeechSynthesis,
    hasBarcodeDetection
  } from '@hypothesi/tauri-plugin-device-ai-apis'

  // ============================================================================
  // State
  // ============================================================================

  const capabilityItems = [
    { key: 'speechRecognition', icon: micIcon, label: 'Speech Recognition' },
    { key: 'speechSynthesis', icon: volume2Icon, label: 'Speech Synthesis' },
    { key: 'textRecognition', icon: fileTextIcon, label: 'Text Recognition (OCR)' },
    { key: 'barcodeDetection', icon: barChartIcon, label: 'Barcode Detection' },
    { key: 'faceDetection', icon: smileIcon, label: 'Face Detection' },
    { key: 'imageClassification', icon: tagIcon, label: 'Image Classification' },
    { key: 'languageIdentification', icon: globeIcon, label: 'Language Identification' },
    { key: 'translation', icon: languagesIcon, label: 'Translation' }
  ]

  const getToastIcon = (type) => {
    if (type === 'error') return xCircleIcon
    if (type === 'success') return checkCircleIcon
    return badgeInfoIcon
  }

  const getChatRoleIcon = (role) => {
    if (role === 'user') return userIcon
    if (role === 'error') return alertTriangleIcon
    return botIcon
  }

  const getTestStatusIcon = (status) => {
    if (status === 'pass') return checkCircleIcon
    if (status === 'fail') return xCircleIcon
    if (status === 'error') return alertTriangleIcon
    return badgeInfoIcon
  }

  // Navigation
  let activeTab = $state('capabilities')

  // Capabilities
  let capabilities = $state(null)
  let platformInfo = $state({ isTauri: false, isWeb: false })

  // Speech Recognition
  let isRecognizing = $state(false)
  let recognitionResult = $state(null)
  let recognitionLanguage = $state('en-US')
  let streamingSessionId = $state(null)

  // Speech Synthesis
  let ttsText = $state('Hello! This is a demonstration of text-to-speech synthesis.')
  let voices = $state([])
  let selectedVoice = $state('')
  let ttsRate = $state(1.0)
  let ttsPitch = $state(1.0)
  let isSpeaking = $state(false)

  // Vision - OCR
  let ocrResult = $state(null)
  let ocrImage = $state(null)
  let isProcessingOcr = $state(false)

  // Vision - Barcode
  let barcodeResults = $state([])
  let barcodeImage = $state(null)
  let isProcessingBarcode = $state(false)

  // Vision - Face Detection
  let faceResults = $state([])
  let faceImage = $state(null)
  let isProcessingFaces = $state(false)
  let detectLandmarks = $state(true)
  let classifyAttributes = $state(true)

  // Vision - Image Classification
  let classificationResults = $state([])
  let classificationImage = $state(null)
  let isProcessingClassification = $state(false)
  let maxClassifications = $state(5)
  let minConfidence = $state(0.1)

  // Text - Language ID
  let langIdText = $state('Bonjour, comment allez-vous? Je suis très heureux de vous rencontrer.')
  let langIdResult = $state(null)
  let isIdentifyingLang = $state(false)

  // LLM
  let llmAvailability = $state(null)
  let llmModelInfo = $state(null)
  let llmPrompt = $state('Explain quantum computing in one paragraph.')
  let llmSystemPrompt = $state('')
  let llmTemperature = $state(0.7)
  let llmMaxTokens = $state(512)
  let llmResult = $state(null)
  let llmStreamContent = $state('')
  let isGenerating = $state(false)
  let isStreaming = $state(false)
  // LLM Sessions
  let llmSessionId = $state(null)
  let llmChatHistory = $state([])
  let llmChatInput = $state('')
  let isSendingChat = $state(false)
  // Text Intelligence
  let summarizeText = $state('Artificial intelligence has transformed the way we interact with technology. From virtual assistants to autonomous vehicles, AI systems are becoming increasingly integrated into our daily lives. Machine learning, a subset of AI, enables computers to learn from data without being explicitly programmed. Deep learning, which uses neural networks with many layers, has achieved remarkable results in image recognition, natural language processing, and game playing.')
  let summarizeResult = $state(null)
  let isSummarizing = $state(false)
  let rewriteText = $state('hey wanna grab lunch tmrw? lmk if ur free')
  let rewriteTone = $state('formal')
  let rewriteResult = $state(null)
  let isRewriting = $state(false)

  // Logs
  let logs = $state([])

  // Processing status messages
  let processingStatus = $state(null)

  // Toast notifications
  let toasts = $state([])

  // ============================================================================
  // Tests
  // ============================================================================
  let testResults = $state([])
  let isRunningTests = $state(false)
  let testProgress = $state({ current: 0, total: 0 })

  // ============================================================================
  // Helpers
  // ============================================================================

  /**
   * Extract error message from various error formats.
   * Handles Tauri plugin errors ({code, message}), standard JS errors, and plain strings.
   */
  function getErrorMessage(error) {
    if (!error) return 'Unknown error';
    // Tauri plugin error format: { code: string, message: string }
    if (typeof error === 'object' && error.message) return error.message;
    // Plain string error
    if (typeof error === 'string') return error;
    // Try to stringify
    return String(error);
  }

  function setProcessingStatus(message, type = 'info') {
    processingStatus = { message, type, startTime: Date.now() };
  }

  function clearProcessingStatus() {
    processingStatus = null;
  }

  function showToast(message, type = 'info', duration = 5000) {
    const id = Date.now();
    toasts = [...toasts, { id, message, type }];
    if (duration > 0) {
      setTimeout(() => dismissToast(id), duration);
    }
    return id;
  }

  function dismissToast(id) {
    toasts = toasts.filter(t => t.id !== id);
  }

  function showError(message) {
    showToast(message, 'error', 8000);
  }

  // ============================================================================
  // Logging
  // ============================================================================

  function log(message, type = 'info') {
    const timestamp = new Date().toLocaleTimeString()
    logs = [{ timestamp, message, type, id: Date.now() }, ...logs].slice(0, 100)
  }

  function clearLogs() {
    logs = []
  }

  // ============================================================================
  // Capabilities
  // ============================================================================

  async function loadCapabilities() {
    try {
      platformInfo = {
        isTauri: isTauri(),
        isWeb: isWeb(),
        hasWebSpeech: hasWebSpeechRecognition(),
        hasWebSynthesis: hasWebSpeechSynthesis(),
        hasWebBarcode: hasBarcodeDetection()
      }
      capabilities = await getCapabilities()
      log('Capabilities loaded successfully', 'success')
    } catch (error) {
      const msg = `Failed to load capabilities: ${getErrorMessage(error)}`
      log(msg, 'error')
      showError(msg)
    }
  }

  // ============================================================================
  // Speech Recognition
  // ============================================================================

  async function startRecognition() {
    if (isRecognizing) return
    isRecognizing = true
    recognitionResult = null
    setProcessingStatus('Listening for speech...', 'info')
    log(`Starting speech recognition (${recognitionLanguage})...`, 'info')

    try {
      const result = await speech.recognize({ language: recognitionLanguage })
      recognitionResult = result
      log(`Recognized: "${result.text}" (confidence: ${(result.confidence * 100).toFixed(1)}%)`, 'success')
    } catch (error) {
      const msg = `Speech recognition failed: ${getErrorMessage(error)}`
      log(msg, 'error')
      showError(msg)
    } finally {
      isRecognizing = false
      clearProcessingStatus()
    }
  }

  async function startStreamingRecognition() {
    if (streamingSessionId) return
    setProcessingStatus('Starting streaming recognition...', 'info')
    log('Starting streaming recognition...', 'info')

    try {
      streamingSessionId = await speech.startRecognition({
        language: recognitionLanguage,
        continuous: true,
        interimResults: true
      })
      setProcessingStatus('Listening (streaming)...', 'info')
      log(`Streaming session started: ${streamingSessionId}`, 'success')
    } catch (error) {
      const msg = `Failed to start streaming: ${getErrorMessage(error)}`
      log(msg, 'error')
      showError(msg)
      clearProcessingStatus()
    }
  }

  async function stopStreamingRecognition() {
    if (!streamingSessionId) return
    setProcessingStatus('Stopping streaming...', 'info')
    log('Stopping streaming recognition...', 'info')

    try {
      const result = await speech.stopRecognition(streamingSessionId)
      recognitionResult = result
      log(`Final result: "${result.text}"`, 'success')
    } catch (error) {
      const msg = `Failed to stop streaming: ${getErrorMessage(error)}`
      log(msg, 'error')
      showError(msg)
    } finally {
      streamingSessionId = null
      clearProcessingStatus()
    }
  }

  // ============================================================================
  // Speech Synthesis
  // ============================================================================

  async function loadVoices() {
    try {
      const result = await speech.getVoices()
      voices = Array.isArray(result) ? result : (result.voices || [])
      log(`Loaded ${voices.length} voices`, 'success')
    } catch (error) {
      const msg = `Failed to load voices: ${getErrorMessage(error)}`
      log(msg, 'error')
      showError(msg)
    }
  }

  async function synthesizeSpeech() {
    if (!ttsText.trim() || isSpeaking) return
    isSpeaking = true
    setProcessingStatus('Speaking...', 'info')
    log(`Synthesizing speech...`, 'info')

    try {
      await speech.synthesize(ttsText, {
        voice: selectedVoice || undefined,
        rate: ttsRate,
        pitch: ttsPitch
      })
      log('Speech synthesis completed', 'success')
    } catch (error) {
      const msg = `Speech synthesis failed: ${getErrorMessage(error)}`
      log(msg, 'error')
      showError(msg)
    } finally {
      isSpeaking = false
      clearProcessingStatus()
    }
  }

  // ============================================================================
  // Vision - Common
  // ============================================================================

  function fileToBase64(file) {
    return new Promise((resolve, reject) => {
      const reader = new FileReader()
      reader.onload = (e) => resolve(e.target.result.split(',')[1])
      reader.onerror = reject
      reader.readAsDataURL(file)
    })
  }

  function fileToDataUrl(file) {
    return new Promise((resolve, reject) => {
      const reader = new FileReader()
      reader.onload = (e) => resolve(e.target.result)
      reader.onerror = reject
      reader.readAsDataURL(file)
    })
  }

  // ============================================================================
  // Vision - OCR
  // ============================================================================

  async function handleOcrImage(event) {
    const file = event.target.files[0]
    if (!file) return

    isProcessingOcr = true
    ocrResult = null
    setProcessingStatus(`Processing image for OCR...`, 'info')
    log('Processing image for text recognition...', 'info')

    try {
      ocrImage = await fileToDataUrl(file)
      const base64 = await fileToBase64(file)
      const result = await vision.recognizeText({ base64 })
      ocrResult = result
      log(`Found ${result.blocks?.length || 0} text blocks`, 'success')
    } catch (error) {
      const msg = `OCR failed: ${getErrorMessage(error)}`
      log(msg, 'error')
      showError(msg)
    } finally {
      isProcessingOcr = false
      clearProcessingStatus()
    }
  }

  // ============================================================================
  // Vision - Barcode
  // ============================================================================

  async function handleBarcodeImage(event) {
    const file = event.target.files[0]
    if (!file) return

    isProcessingBarcode = true
    barcodeResults = []
    setProcessingStatus('Scanning for barcodes...', 'info')
    log('Scanning for barcodes...', 'info')

    try {
      barcodeImage = await fileToDataUrl(file)
      const base64 = await fileToBase64(file)
      const results = await vision.detectBarcodes({ base64 })
      barcodeResults = Array.isArray(results) ? results : (results.barcodes || [])
      log(`Found ${barcodeResults.length} barcode(s)`, 'success')
    } catch (error) {
      const msg = `Barcode detection failed: ${getErrorMessage(error)}`
      log(msg, 'error')
      showError(msg)
    } finally {
      isProcessingBarcode = false
      clearProcessingStatus()
    }
  }

  // ============================================================================
  // Vision - Face Detection
  // ============================================================================

  async function handleFaceImage(event) {
    const file = event.target.files[0]
    if (!file) return

    isProcessingFaces = true
    faceResults = []
    setProcessingStatus('Detecting faces...', 'info')
    log('Detecting faces...', 'info')

    try {
      faceImage = await fileToDataUrl(file)
      const base64 = await fileToBase64(file)
      const results = await vision.detectFaces({ base64 }, {
        detectLandmarks,
        classifyAttributes
      })
      faceResults = Array.isArray(results) ? results : (results.faces || [])
      log(`Detected ${faceResults.length} face(s)`, 'success')
    } catch (error) {
      const msg = `Face detection failed: ${getErrorMessage(error)}`
      log(msg, 'error')
      showError(msg)
    } finally {
      isProcessingFaces = false
      clearProcessingStatus()
    }
  }

  // ============================================================================
  // Vision - Image Classification
  // ============================================================================

  async function handleClassificationImage(event) {
    const file = event.target.files[0]
    if (!file) return

    isProcessingClassification = true
    classificationResults = []
    setProcessingStatus('Classifying image...', 'info')
    log('Classifying image...', 'info')

    try {
      classificationImage = await fileToDataUrl(file)
      const base64 = await fileToBase64(file)
      const results = await vision.classifyImage({ base64 }, {
        maxResults: maxClassifications,
        minConfidence
      })
      classificationResults = Array.isArray(results) ? results : (results.classifications || [])
      log(`Found ${classificationResults.length} classification(s)`, 'success')
    } catch (error) {
      const msg = `Image classification failed: ${getErrorMessage(error)}`
      log(msg, 'error')
      showError(msg)
    } finally {
      isProcessingClassification = false
      clearProcessingStatus()
    }
  }

  // ============================================================================
  // Text Processing
  // ============================================================================

  async function identifyLanguage() {
    if (!langIdText.trim()) return
    isIdentifyingLang = true
    langIdResult = null
    setProcessingStatus('Identifying language...', 'info')
    log('Identifying language...', 'info')

    try {
      const result = await text.identifyLanguage(langIdText)
      langIdResult = result
      log(`Detected: ${result.language} (${(result.confidence * 100).toFixed(1)}% confidence)`, 'success')
    } catch (error) {
      const msg = `Language identification failed: ${getErrorMessage(error)}`
      log(msg, 'error')
      showError(msg)
    } finally {
      isIdentifyingLang = false
      clearProcessingStatus()
    }
  }

  // ============================================================================
  // LLM
  // ============================================================================

  async function checkLlmAvailability() {
    try {
      setProcessingStatus('Checking LLM availability...', 'info')
      llmAvailability = await llm.checkAvailability()
      log(`LLM available: ${llmAvailability.available}${llmAvailability.reason ? ` (${llmAvailability.reason})` : ''}`, llmAvailability.available ? 'success' : 'info')
      if (llmAvailability.available) {
        llmModelInfo = await llm.getModelInfo()
        log(`Model: ${llmModelInfo.name} (${llmModelInfo.provider})`, 'success')
      }
    } catch (error) {
      showError(getErrorMessage(error))
      log(`LLM check failed: ${getErrorMessage(error)}`, 'error')
    } finally {
      clearProcessingStatus()
    }
  }

  async function generateText() {
    if (isGenerating || !llmPrompt.trim()) return
    isGenerating = true
    llmResult = null
    try {
      setProcessingStatus('Generating text...', 'info')
      const opts = {
        prompt: llmPrompt,
        ...(llmSystemPrompt.trim() ? { systemPrompt: llmSystemPrompt } : {}),
        temperature: llmTemperature,
        maxTokens: llmMaxTokens,
      }
      llmResult = await llm.generate(opts)
      log(`Generated ${llmResult.content.length} chars (${llmResult.finishReason})`, 'success')
    } catch (error) {
      showError(getErrorMessage(error))
      log(`Generation failed: ${getErrorMessage(error)}`, 'error')
    } finally {
      isGenerating = false
      clearProcessingStatus()
    }
  }

  async function streamText() {
    if (isStreaming || !llmPrompt.trim()) return
    isStreaming = true
    llmStreamContent = ''
    llmResult = null
    try {
      setProcessingStatus('Streaming response...', 'info')
      const opts = {
        prompt: llmPrompt,
        ...(llmSystemPrompt.trim() ? { systemPrompt: llmSystemPrompt } : {}),
        temperature: llmTemperature,
        maxTokens: llmMaxTokens,
      }
      await llm.generateStream(opts, (event) => {
        if (event.type === 'delta') {
          llmStreamContent += event.content
        } else if (event.type === 'done') {
          llmStreamContent = event.content
          llmResult = { content: event.content, model: '', finishReason: event.finishReason, usage: event.usage }
          log(`Streamed ${event.content.length} chars (${event.finishReason})`, 'success')
        } else if (event.type === 'error') {
          showError(event.message)
          log(`Stream error: ${event.message}`, 'error')
        }
      })
    } catch (error) {
      showError(getErrorMessage(error))
      log(`Stream failed: ${getErrorMessage(error)}`, 'error')
    } finally {
      isStreaming = false
      clearProcessingStatus()
    }
  }

  async function createLlmSession() {
    try {
      setProcessingStatus('Creating session...', 'info')
      const opts = {
        ...(llmSystemPrompt.trim() ? { systemPrompt: llmSystemPrompt } : {}),
        temperature: llmTemperature,
        maxTokens: llmMaxTokens,
      }
      llmSessionId = await llm.createSession(opts)
      llmChatHistory = []
      log(`Session created: ${llmSessionId.substring(0, 8)}...`, 'success')
    } catch (error) {
      showError(getErrorMessage(error))
      log(`Session creation failed: ${getErrorMessage(error)}`, 'error')
    } finally {
      clearProcessingStatus()
    }
  }

  async function sendChatMessage() {
    if (isSendingChat || !llmChatInput.trim() || !llmSessionId) return
    isSendingChat = true
    const message = llmChatInput
    llmChatInput = ''
    llmChatHistory = [...llmChatHistory, { role: 'user', content: message }]
    try {
      setProcessingStatus('Thinking...', 'info')
      const result = await llm.sessionSend(llmSessionId, message)
      llmChatHistory = [...llmChatHistory, { role: 'assistant', content: result.content }]
      log(`Session response: ${result.content.length} chars`, 'success')
    } catch (error) {
      showError(getErrorMessage(error))
      llmChatHistory = [...llmChatHistory, { role: 'error', content: getErrorMessage(error) }]
    } finally {
      isSendingChat = false
      clearProcessingStatus()
    }
  }

  async function endLlmSession() {
    if (!llmSessionId) return
    try {
      await llm.destroySession(llmSessionId)
      log(`Session destroyed: ${llmSessionId.substring(0, 8)}...`, 'info')
    } catch (error) {
      log(`Session destroy failed: ${getErrorMessage(error)}`, 'error')
    }
    llmSessionId = null
    llmChatHistory = []
  }

  async function summarizeTextHandler() {
    if (isSummarizing || !summarizeText.trim()) return
    isSummarizing = true
    summarizeResult = null
    try {
      setProcessingStatus('Summarizing...', 'info')
      summarizeResult = await llm.summarize({ text: summarizeText })
      log(`Summarized to ${summarizeResult.summary.length} chars`, 'success')
    } catch (error) {
      showError(getErrorMessage(error))
      log(`Summarize failed: ${getErrorMessage(error)}`, 'error')
    } finally {
      isSummarizing = false
      clearProcessingStatus()
    }
  }

  async function rewriteTextHandler() {
    if (isRewriting || !rewriteText.trim()) return
    isRewriting = true
    rewriteResult = null
    try {
      setProcessingStatus('Rewriting...', 'info')
      rewriteResult = await llm.rewrite({ text: rewriteText, tone: rewriteTone })
      log(`Rewritten in ${rewriteTone} tone`, 'success')
    } catch (error) {
      showError(getErrorMessage(error))
      log(`Rewrite failed: ${getErrorMessage(error)}`, 'error')
    } finally {
      isRewriting = false
      clearProcessingStatus()
    }
  }

  // ============================================================================
  // Tests
  // ============================================================================

  async function runTests() {
    if (isRunningTests) return
    isRunningTests = true
    testResults = []
    setProcessingStatus('Running end-to-end tests...', 'info')
    log('Starting end-to-end tests...', 'info')

    try {
      // 1. Fetch manifest
      const response = await fetch('/samples/manifest.json')
      if (!response.ok) throw new Error('Failed to load manifest.json')
      const manifest = await response.json()
      const samples = manifest.samples

      testProgress = { current: 0, total: samples.length }

      for (const sample of samples) {
        testProgress.current++
        const result = {
          name: sample.type,
          description: sample.description,
          status: 'pending',
          details: ''
        }

        try {
          // Load file
          const fileResponse = await fetch(`/samples/${sample.file}`)
          if (!fileResponse.ok) throw new Error(`Failed to load ${sample.file}`)
          const blob = await fileResponse.blob()
          const base64 = await new Promise((resolve) => {
            const reader = new FileReader()
            reader.onloadend = () => resolve(reader.result.split(',')[1])
            reader.readAsDataURL(blob)
          })

          // Run test based on type
          if (sample.type === 'face-detection') {
            const faces = await vision.detectFaces({ base64 })
            const count = Array.isArray(faces) ? faces.length : (faces.faces || []).length
            if (count >= sample.expected.minFaces) {
              result.status = 'pass'
              result.details = `Detected ${count} faces (Expected >= ${sample.expected.minFaces})`
            } else {
              result.status = 'fail'
              result.details = `Detected ${count} faces (Expected >= ${sample.expected.minFaces})`
            }
          } else if (sample.type === 'ocr') {
            const ocr = await vision.recognizeText({ base64 })
            const textContent = ocr.text || ''
            const missing = sample.expected.contains.filter(word => !textContent.toLowerCase().includes(word.toLowerCase()))
            if (missing.length === 0) {
              result.status = 'pass'
              result.details = `Found all expected words: ${sample.expected.contains.join(', ')}`
            } else {
              result.status = 'fail'
              result.details = `Missing words: ${missing.join(', ')}`
            }
          } else if (sample.type === 'barcode') {
            const barcodes = await vision.detectBarcodes({ base64 })
            const list = Array.isArray(barcodes) ? barcodes : (barcodes.barcodes || [])
            const match = list.find(b => b.format === sample.expected.format && b.rawValue === sample.expected.rawValue)
            if (match) {
              result.status = 'pass'
              result.details = `Found ${sample.expected.format} with value: ${sample.expected.rawValue}`
            } else {
              result.status = 'fail'
              result.details = `Expected ${sample.expected.format} (${sample.expected.rawValue}), found ${list.length} barcodes`
            }
          } else if (sample.type === 'image-classification') {
            const classifications = await vision.classifyImage({ base64 })
            const list = Array.isArray(classifications) ? classifications : (classifications.classifications || [])
            const match = list.find(c => c.identifier.toLowerCase().includes(sample.expected.className.toLowerCase()))
            if (match) {
              result.status = 'pass'
              result.details = `Classified as ${match.identifier} (${(match.confidence * 100).toFixed(1)}%)`
            } else {
              result.status = 'fail'
              result.details = `Expected ${sample.expected.className}, found: ${list.map(c => c.identifier).join(', ')}`
            }
          } else if (sample.type === 'speech-recognition') {
            // Speech recognition from audio file
            const speechResult = await speech.recognize({
              language: 'en-US',
              audioSource: { base64 }
            })
            const expectedText = sample.expected.text.toLowerCase()
            const actualText = speechResult.text.toLowerCase()

            // Check if the recognized text contains key words from expected text
            const expectedWords = expectedText.split(' ').filter(w => w.length > 3)
            const matchedWords = expectedWords.filter(word => actualText.includes(word))
            const matchRatio = matchedWords.length / expectedWords.length

            if (matchRatio > 0.5) {
              result.status = 'pass'
              result.details = `Recognized: "${speechResult.text}" (${(matchRatio * 100).toFixed(0)}% word match)`
            } else {
              result.status = 'fail'
              result.details = `Expected: "${sample.expected.text}", Got: "${speechResult.text}"`
            }
          } else {
            result.status = 'skipped'
            result.details = `Unknown test type: ${sample.type}`
          }

        } catch (err) {
          result.status = 'error'
          result.details = err.message
        }

        testResults = [...testResults, result]
      }

      log('End-to-end tests completed', 'success')
      showToast('Tests completed!', 'success')

    } catch (error) {
      const msg = `Test runner failed: ${error.message}`
      log(msg, 'error')
      showError(msg)
    } finally {
      isRunningTests = false
      clearProcessingStatus()
    }
  }

  // ============================================================================
  // Lifecycle
  // ============================================================================

  $effect(() => {
    loadCapabilities()
    loadVoices()
  })
</script>

<div class="app">
  <!-- Processing Status Banner -->
  {#if processingStatus}
    <div class="processing-banner {processingStatus.type}">
      <span class="processing-spinner"></span>
      <span class="processing-message">{processingStatus.message}</span>
    </div>
  {/if}

  <!-- Toast Notifications -->
  {#if toasts.length > 0}
    <div class="toast-container">
      {#each toasts as toast (toast.id)}
        <div class="toast {toast.type}">
          <Icon class="toast-icon" icon={getToastIcon(toast.type)} />
          <span class="toast-message">{toast.message}</span>
          <button class="toast-dismiss" aria-label="Dismiss notification" onclick={() => dismissToast(toast.id)}>
            <Icon class="icon" icon={xIcon} />
          </button>
        </div>
      {/each}
    </div>
  {/if}

  <!-- Header -->
  <header class="header">
    <h1><Icon class="icon title-icon" icon={botIcon} /> Device AI APIs</h1>
    <p class="subtitle">Tauri Plugin Demo</p>
  </header>

  <!-- Navigation -->
  <nav class="nav">
    <button class="nav-btn" class:active={activeTab === 'capabilities'} onclick={() => activeTab = 'capabilities'}>
      <Icon class="icon" icon={clipboardListIcon} /> Capabilities
    </button>
    <button class="nav-btn" class:active={activeTab === 'speech'} onclick={() => activeTab = 'speech'}>
      <Icon class="icon" icon={micIcon} /> Speech
    </button>
    <button class="nav-btn" class:active={activeTab === 'vision'} onclick={() => activeTab = 'vision'}>
      <Icon class="icon" icon={eyeIcon} /> Vision
    </button>
    <button class="nav-btn" class:active={activeTab === 'text'} onclick={() => activeTab = 'text'}>
      <Icon class="icon" icon={fileTextIcon} /> Text
    </button>
    <button class="nav-btn" class:active={activeTab === 'llm'} onclick={() => activeTab = 'llm'}>
      <Icon class="icon" icon={brainIcon} /> LLM
    </button>
    <button class="nav-btn" class:active={activeTab === 'logs'} onclick={() => activeTab = 'logs'}>
      <Icon class="icon" icon={scrollTextIcon} /> Logs {#if logs.length > 0}<span class="badge">{logs.length}</span>{/if}
    </button>
    <button class="nav-btn" class:active={activeTab === 'tests'} onclick={() => activeTab = 'tests'}>
      <Icon class="icon" icon={flaskConicalIcon} /> Tests
    </button>
  </nav>

  <!-- Main Content -->
  <main class="content">
    <!-- Capabilities Tab -->
    {#if activeTab === 'capabilities'}
      <section class="panel">
        <h2>Platform Information</h2>
        <div class="info-grid">
          <div class="info-item">
            <span class="label">Environment</span>
            <span class="value">{platformInfo.isTauri ? 'Tauri App' : 'Web Browser'}</span>
          </div>
          <div class="info-item">
            <span class="label">Web Speech API</span>
            <span class="value status-value" class:available={platformInfo.hasWebSpeech}>
              <Icon class="icon" icon={platformInfo.hasWebSpeech ? checkCircleIcon : xCircleIcon} />
              {platformInfo.hasWebSpeech ? 'Available' : 'Not available'}
            </span>
          </div>
          <div class="info-item">
            <span class="label">Web Synthesis API</span>
            <span class="value status-value" class:available={platformInfo.hasWebSynthesis}>
              <Icon class="icon" icon={platformInfo.hasWebSynthesis ? checkCircleIcon : xCircleIcon} />
              {platformInfo.hasWebSynthesis ? 'Available' : 'Not available'}
            </span>
          </div>
          <div class="info-item">
            <span class="label">Web Barcode API</span>
            <span class="value status-value" class:available={platformInfo.hasWebBarcode}>
              <Icon class="icon" icon={platformInfo.hasWebBarcode ? checkCircleIcon : xCircleIcon} />
              {platformInfo.hasWebBarcode ? 'Available' : 'Not available'}
            </span>
          </div>
        </div>
      </section>

      <section class="panel">
        <h2>Device Capabilities</h2>
        {#if capabilities}
          <div class="capabilities-grid">
            {#each capabilityItems as cap}
              <div class="capability-card" class:available={capabilities[cap.key]?.available}>
                <div class="cap-header">
                  <Icon class="cap-icon" icon={cap.icon} />
                  <Icon class="cap-status" icon={capabilities[cap.key]?.available ? checkCircleIcon : xCircleIcon} />
                </div>
                <div class="cap-label">{cap.label}</div>
                {#if capabilities[cap.key]?.available}
                  <div class="cap-details">
                    <span class:on-device={capabilities[cap.key]?.onDevice}>
                      <Icon class="icon" icon={capabilities[cap.key]?.onDevice ? lockIcon : cloudIcon} />
                      {capabilities[cap.key]?.onDevice ? 'On-device' : 'Cloud'}
                    </span>
                    {#if capabilities[cap.key]?.requiresPermission}
                      <span><Icon class="icon" icon={shieldCheckIcon} /> Requires permission</span>
                    {/if}
                  </div>
                {/if}
              </div>
            {/each}
          </div>
        {:else}
          <div class="loading">Loading capabilities...</div>
        {/if}
      </section>

    <!-- Speech Tab -->
    {:else if activeTab === 'speech'}
      <section class="panel">
        <h2><Icon class="icon" icon={micIcon} /> Speech Recognition</h2>
        <p class="description">Convert spoken words to text using on-device speech recognition.</p>

        <div class="control-group">
          <label for="recognition-lang">Language</label>
          <select id="recognition-lang" bind:value={recognitionLanguage}>
            <option value="en-US">English (US)</option>
            <option value="en-GB">English (UK)</option>
            <option value="es-ES">Spanish</option>
            <option value="fr-FR">French</option>
            <option value="de-DE">German</option>
            <option value="it-IT">Italian</option>
            <option value="ja-JP">Japanese</option>
            <option value="ko-KR">Korean</option>
            <option value="zh-CN">Chinese (Simplified)</option>
          </select>
        </div>

        <div class="button-group">
          <button class="primary" onclick={startRecognition} disabled={isRecognizing}>
            <Icon class="icon" icon={isRecognizing ? mic2Icon : micIcon} />
            {isRecognizing ? 'Listening...' : 'One-shot Recognition'}
          </button>

          {#if !streamingSessionId}
            <button onclick={startStreamingRecognition}><Icon class="icon" icon={playIcon} /> Start Streaming</button>
          {:else}
            <button class="danger" onclick={stopStreamingRecognition}><Icon class="icon" icon={squareIcon} /> Stop Streaming</button>
          {/if}
        </div>

        {#if recognitionResult}
          <div class="result-box">
            <h4>Result</h4>
            <p class="recognized-text">"{recognitionResult.text}"</p>
            <div class="result-meta">
              <span>Confidence: {(recognitionResult.confidence * 100).toFixed(1)}%</span>
              <span>Final: {recognitionResult.isFinal ? 'Yes' : 'No'}</span>
            </div>
          </div>
        {/if}
      </section>

      <section class="panel">
        <h2><Icon class="icon" icon={volume2Icon} /> Text-to-Speech</h2>
        <p class="description">Convert text to spoken audio using available voices.</p>

        <div class="control-group">
          <label for="tts-text">Text to speak</label>
          <textarea id="tts-text" bind:value={ttsText} placeholder="Enter text to speak..." rows="3"></textarea>
        </div>

        <div class="control-row">
          <div class="control-group flex-1">
            <label for="tts-voice">Voice</label>
            <select id="tts-voice" bind:value={selectedVoice}>
              <option value="">Default Voice</option>
              {#each voices as voice}
                <option value={voice.id}>{voice.name} ({voice.language}) {voice.isDefault ? '(default)' : ''}</option>
              {/each}
            </select>
          </div>

          <div class="control-group">
            <label for="tts-rate">Rate: {ttsRate.toFixed(1)}x</label>
            <input type="range" id="tts-rate" bind:value={ttsRate} min="0.5" max="2" step="0.1" />
          </div>

          <div class="control-group">
            <label for="tts-pitch">Pitch: {ttsPitch.toFixed(1)}x</label>
            <input type="range" id="tts-pitch" bind:value={ttsPitch} min="0.5" max="2" step="0.1" />
          </div>
        </div>

        <div class="button-group">
          <button class="primary" onclick={synthesizeSpeech} disabled={isSpeaking || !ttsText.trim()}>
            <Icon class="icon" icon={volume2Icon} />
            {isSpeaking ? 'Speaking...' : 'Speak'}
          </button>
          <button onclick={loadVoices}><Icon class="icon" icon={refreshCwIcon} /> Refresh Voices</button>
        </div>

        {#if voices.length > 0}
          <p class="hint">{voices.length} voice(s) available</p>
        {/if}
      </section>

    <!-- Vision Tab -->
    {:else if activeTab === 'vision'}
      <div class="vision-grid">
        <!-- OCR Panel -->
        <section class="panel">
          <h2><Icon class="icon" icon={fileTextIcon} /> Text Recognition (OCR)</h2>
          <p class="description">Extract text from images.</p>

          <div class="file-input-wrapper">
            <input type="file" accept="image/*" onchange={handleOcrImage} disabled={isProcessingOcr} />
          </div>

          {#if ocrImage}
            <div class="image-preview"><img src={ocrImage} alt="OCR input" /></div>
          {/if}

          {#if isProcessingOcr}
            <div class="loading">Processing image...</div>
          {:else if ocrResult}
            <div class="result-box">
              <h4>Extracted Text</h4>
              <pre class="text-result">{ocrResult.text || '(No text found)'}</pre>
            </div>
          {/if}
        </section>

        <!-- Barcode Panel -->
        <section class="panel">
          <h2><Icon class="icon" icon={barChartIcon} /> Barcode Detection</h2>
          <p class="description">Detect and decode QR codes and barcodes.</p>

          <div class="file-input-wrapper">
            <input type="file" accept="image/*" onchange={handleBarcodeImage} disabled={isProcessingBarcode} />
          </div>

          {#if barcodeImage}
            <div class="image-preview"><img src={barcodeImage} alt="Barcode input" /></div>
          {/if}

          {#if isProcessingBarcode}
            <div class="loading">Scanning for barcodes...</div>
          {:else if barcodeResults.length > 0}
            <div class="result-box">
              <h4>Detected Barcodes ({barcodeResults.length})</h4>
              {#each barcodeResults as barcode}
                <div class="barcode-item">
                  <span class="barcode-format">{barcode.format}</span>
                  <code class="barcode-value">{barcode.rawValue}</code>
                </div>
              {/each}
            </div>
          {:else if barcodeImage && !isProcessingBarcode}
            <p class="no-results">No barcodes detected</p>
          {/if}
        </section>

        <!-- Face Detection Panel -->
        <section class="panel">
          <h2><Icon class="icon" icon={scanFaceIcon} /> Face Detection</h2>
          <p class="description">Detect faces and analyze attributes.</p>

          <div class="control-row compact">
            <label class="checkbox-label">
              <input type="checkbox" bind:checked={detectLandmarks} /> Detect landmarks
            </label>
            <label class="checkbox-label">
              <input type="checkbox" bind:checked={classifyAttributes} /> Classify attributes
            </label>
          </div>

          <div class="file-input-wrapper">
            <input type="file" accept="image/*" onchange={handleFaceImage} disabled={isProcessingFaces} />
          </div>

          {#if faceImage}
            <div class="image-preview"><img src={faceImage} alt="Face detection input" /></div>
          {/if}

          {#if isProcessingFaces}
            <div class="loading">Detecting faces...</div>
          {:else if faceResults.length > 0}
            <div class="result-box">
              <h4>Detected Faces ({faceResults.length})</h4>
              {#each faceResults as face, i}
                <div class="face-item">
                  <strong>Face {i + 1}</strong>

                  <!-- Orientation angles -->
                  {#if face.rollAngle != null || face.yawAngle != null}
                    <div class="face-orientation">
                      {#if face.rollAngle != null}
                        <span title="Head tilt"><Icon class="icon" icon={rotateCwIcon} /> Roll: {face.rollAngle.toFixed(1)}°</span>
                      {/if}
                      {#if face.yawAngle != null}
                        <span title="Looking left/right"><Icon class="icon" icon={moveHorizontalIcon} /> Yaw: {face.yawAngle.toFixed(1)}°</span>
                      {/if}
                    </div>
                  {/if}

                  <!-- Bounding box info -->
                  {#if face.boundingBox}
                    <div class="face-bbox">
                      <Icon class="icon" icon={mapPinIcon} /> Position: ({(face.boundingBox.x * 100).toFixed(0)}%, {(face.boundingBox.y * 100).toFixed(0)}%)
                      Size: {(face.boundingBox.width * 100).toFixed(0)}% by {(face.boundingBox.height * 100).toFixed(0)}%
                    </div>
                  {/if}

                  <!-- Landmarks -->
                  {#if face.landmarks}
                    <div class="face-landmarks">
                      <span class="landmarks-title"><Icon class="icon" icon={eyeIcon} /> Landmarks:</span>
                      <div class="landmarks-grid">
                        {#if face.landmarks.leftEye}
                          <span>Left Eye: ({(face.landmarks.leftEye.x * 100).toFixed(0)}%, {(face.landmarks.leftEye.y * 100).toFixed(0)}%)</span>
                        {/if}
                        {#if face.landmarks.rightEye}
                          <span>Right Eye: ({(face.landmarks.rightEye.x * 100).toFixed(0)}%, {(face.landmarks.rightEye.y * 100).toFixed(0)}%)</span>
                        {/if}
                        {#if face.landmarks.nose}
                          <span>Nose: ({(face.landmarks.nose.x * 100).toFixed(0)}%, {(face.landmarks.nose.y * 100).toFixed(0)}%)</span>
                        {/if}
                        {#if face.landmarks.mouthLeft}
                          <span>Mouth L: ({(face.landmarks.mouthLeft.x * 100).toFixed(0)}%, {(face.landmarks.mouthLeft.y * 100).toFixed(0)}%)</span>
                        {/if}
                        {#if face.landmarks.mouthRight}
                          <span>Mouth R: ({(face.landmarks.mouthRight.x * 100).toFixed(0)}%, {(face.landmarks.mouthRight.y * 100).toFixed(0)}%)</span>
                        {/if}
                      </div>
                    </div>
                  {/if}

                  <!-- Attributes -->
                  {#if face.attributes}
                    <div class="face-attrs">
                      {#if face.attributes.smilingProbability != null}
                        <span><Icon class="icon" icon={smileIcon} /> Smiling: {(face.attributes.smilingProbability * 100).toFixed(0)}%</span>
                      {/if}
                      {#if face.attributes.leftEyeOpenProbability != null}
                        <span><Icon class="icon" icon={eyeIcon} /> Left Eye Open: {(face.attributes.leftEyeOpenProbability * 100).toFixed(0)}%</span>
                      {/if}
                      {#if face.attributes.rightEyeOpenProbability != null}
                        <span><Icon class="icon" icon={eyeIcon} /> Right Eye Open: {(face.attributes.rightEyeOpenProbability * 100).toFixed(0)}%</span>
                      {/if}
                    </div>
                  {/if}
                </div>
              {/each}
            </div>
          {:else if faceImage && !isProcessingFaces}
            <p class="no-results">No faces detected</p>
          {/if}
        </section>

        <!-- Image Classification Panel -->
        <section class="panel">
          <h2><Icon class="icon" icon={tagIcon} /> Image Classification</h2>
          <p class="description">Identify objects and scenes.</p>

          <div class="control-row compact">
            <div class="control-group">
              <label for="max-results">Max results: {maxClassifications}</label>
              <input type="range" id="max-results" bind:value={maxClassifications} min="1" max="20" step="1" />
            </div>
          </div>

          <div class="file-input-wrapper">
            <input type="file" accept="image/*" onchange={handleClassificationImage} disabled={isProcessingClassification} />
          </div>

          {#if classificationImage}
            <div class="image-preview"><img src={classificationImage} alt="Classification input" /></div>
          {/if}

          {#if isProcessingClassification}
            <div class="loading">Classifying image...</div>
          {:else if classificationResults.length > 0}
            <div class="result-box">
              <h4>Classifications</h4>
              <div class="classification-list">
                {#each classificationResults as cls}
                  <div class="classification-item">
                    <span class="cls-label">{cls.identifier}</span>
                    <div class="cls-bar-wrapper">
                      <div class="cls-bar" style="width: {cls.confidence * 100}%"></div>
                    </div>
                    <span class="cls-confidence">{(cls.confidence * 100).toFixed(1)}%</span>
                  </div>
                {/each}
              </div>
            </div>
          {:else if classificationImage && !isProcessingClassification}
            <p class="no-results">No classifications found</p>
          {/if}
        </section>
      </div>

    <!-- Text Tab -->
    {:else if activeTab === 'text'}
      <section class="panel">
        <h2><Icon class="icon" icon={globeIcon} /> Language Identification</h2>
        <p class="description">Detect the language of a text passage.</p>

        <div class="control-group">
          <label for="lang-text">Text to analyze</label>
          <textarea id="lang-text" bind:value={langIdText} placeholder="Enter text in any language..." rows="4"></textarea>
        </div>

        <div class="sample-texts">
          <span class="sample-label">Try samples:</span>
          <button class="sample-btn" onclick={() => langIdText = 'Hello, how are you today?'}>English</button>
          <button class="sample-btn" onclick={() => langIdText = 'Bonjour, comment allez-vous?'}>French</button>
          <button class="sample-btn" onclick={() => langIdText = 'Hola, ¿cómo estás?'}>Spanish</button>
          <button class="sample-btn" onclick={() => langIdText = 'Guten Tag, wie geht es Ihnen?'}>German</button>
          <button class="sample-btn" onclick={() => langIdText = 'こんにちは、お元気ですか？'}>Japanese</button>
          <button class="sample-btn" onclick={() => langIdText = '你好，你今天好吗？'}>Chinese</button>
        </div>

        <div class="button-group">
          <button class="primary" onclick={identifyLanguage} disabled={isIdentifyingLang || !langIdText.trim()}>
            <Icon class="icon" icon={isIdentifyingLang ? searchIcon : globeIcon} />
            {isIdentifyingLang ? 'Analyzing...' : 'Identify Language'}
          </button>
        </div>

        {#if langIdResult}
          <div class="result-box">
            <h4>Detected Language</h4>
            <div class="lang-result">
              <span class="lang-code">{langIdResult.language}</span>
              <span class="lang-confidence">{(langIdResult.confidence * 100).toFixed(1)}% confidence</span>
            </div>
          </div>
        {/if}
      </section>

      <section class="panel coming-soon">
        <h2><Icon class="icon" icon={languagesIcon} /> Translation</h2>
        <p class="description">Translate text between languages. (Coming soon)</p>
        <div class="placeholder">This feature will be available in a future update.</div>
      </section>

    <!-- LLM Tab -->
    {:else if activeTab === 'llm'}
      <!-- Availability & Model Info -->
      <section class="panel">
        <h2><Icon class="icon" icon={brainIcon} /> On-Device Language Model</h2>
        <p class="description">Generate text, have conversations, and use text intelligence powered by on-device AI.</p>

        <div class="button-group" style="margin-bottom: 1rem;">
          <button class="primary" onclick={checkLlmAvailability}>
            <Icon class="icon" icon={llmAvailability ? refreshCwIcon : searchIcon} />
            {llmAvailability ? 'Refresh' : 'Check Availability'}
          </button>
        </div>

        {#if llmAvailability}
          <div class="result-box">
            <h4>Status</h4>
            <div class="info-grid">
              <div class="info-item">
                <span class="label">Available</span>
                <span class="value" style="color: {llmAvailability.available ? '#22c55e' : '#ef4444'}">
                  <Icon class="icon" icon={llmAvailability.available ? checkCircleIcon : xCircleIcon} />
                  {llmAvailability.available ? 'Yes' : 'No'}
                </span>
              </div>
              {#if llmAvailability.reason}
                <div class="info-item">
                  <span class="label">Reason</span>
                  <span class="value">{llmAvailability.reason}</span>
                </div>
              {/if}
              {#if llmModelInfo}
                <div class="info-item">
                  <span class="label">Model</span>
                  <span class="value">{llmModelInfo.name}</span>
                </div>
                <div class="info-item">
                  <span class="label">Context Window</span>
                  <span class="value">{llmModelInfo.contextWindow} tokens</span>
                </div>
                <div class="info-item">
                  <span class="label">Provider</span>
                  <span class="value">{llmModelInfo.provider}</span>
                </div>
                <div class="info-item">
                  <span class="label">On-Device</span>
                  <span class="value">
                    <Icon class="icon" icon={llmModelInfo.onDevice ? checkCircleIcon : xCircleIcon} />
                  </span>
                </div>
              {/if}
            </div>
          </div>
        {/if}
      </section>

      <!-- Generation -->
      <section class="panel">
        <h2><Icon class="icon" icon={pencilIcon} /> Text Generation</h2>

        <div class="control-group">
          <label for="llm-prompt">Prompt</label>
          <textarea id="llm-prompt" bind:value={llmPrompt} placeholder="Enter your prompt..." rows="3"></textarea>
        </div>

        <div class="control-group">
          <label for="llm-system">System prompt (optional)</label>
          <input type="text" id="llm-system" bind:value={llmSystemPrompt} placeholder="e.g. You are a helpful coding assistant." />
        </div>

        <div class="control-row" style="margin-bottom: 1rem;">
          <div class="flex-1">
            <label for="llm-temp">Temperature: {llmTemperature.toFixed(1)}</label>
            <input type="range" id="llm-temp" min="0" max="2" step="0.1" bind:value={llmTemperature} />
          </div>
          <div class="flex-1">
            <label for="llm-tokens">Max tokens: {llmMaxTokens}</label>
            <input type="range" id="llm-tokens" min="64" max="4096" step="64" bind:value={llmMaxTokens} />
          </div>
        </div>

        <div class="button-group">
          <button class="primary" onclick={generateText} disabled={isGenerating || isStreaming || !llmPrompt.trim()}>
            <Icon class="icon" icon={isGenerating ? hourglassIcon : pencilIcon} />
            {isGenerating ? 'Generating...' : 'Generate'}
          </button>
          <button class="primary" onclick={streamText} disabled={isGenerating || isStreaming || !llmPrompt.trim()}>
            <Icon class="icon" icon={isStreaming ? hourglassIcon : wavesIcon} />
            {isStreaming ? 'Streaming...' : 'Stream'}
          </button>
        </div>

        {#if isStreaming && llmStreamContent}
          <div class="result-box">
            <h4>Streaming...</h4>
            <pre class="text-result">{llmStreamContent}<span class="cursor-blink">▊</span></pre>
          </div>
        {/if}

        {#if llmResult && !isStreaming}
          <div class="result-box">
            <h4>Result</h4>
            <pre class="text-result">{llmResult.content}</pre>
            <div class="result-meta">
              <span>Finish: {llmResult.finishReason}</span>
              {#if llmResult.usage}
                <span>Tokens: {llmResult.usage.totalTokens ?? '—'}</span>
              {/if}
            </div>
          </div>
        {/if}
      </section>

      <!-- Sessions -->
      <section class="panel">
        <h2><Icon class="icon" icon={messageCircleIcon} /> Multi-Turn Session</h2>
        <p class="description">Have a back-and-forth conversation with context maintained across messages.</p>

        {#if !llmSessionId}
          <div class="button-group">
            <button class="primary" onclick={createLlmSession}>
              <Icon class="icon" icon={plusCircleIcon} /> Start Session
            </button>
          </div>
          <p class="hint">System prompt and parameters from above will be used for the session.</p>
        {:else}
          <div class="chat-container">
            {#each llmChatHistory as msg}
              <div class="chat-msg {msg.role}">
                <Icon class="chat-role" icon={getChatRoleIcon(msg.role)} />
                <div class="chat-content">{msg.content}</div>
              </div>
            {/each}
            {#if isSendingChat}
              <div class="chat-msg assistant">
                <Icon class="chat-role" icon={botIcon} />
                <div class="chat-content thinking">Thinking...</div>
              </div>
            {/if}
          </div>

          <div class="chat-input-row">
            <input type="text" bind:value={llmChatInput} placeholder="Type a message..."
              onkeydown={(e) => { if (e.key === 'Enter' && !e.shiftKey) sendChatMessage() }} />
            <button class="primary" onclick={sendChatMessage} disabled={isSendingChat || !llmChatInput.trim()}>
              Send
            </button>
            <button class="danger" onclick={endLlmSession}>End</button>
          </div>
        {/if}
      </section>

      <!-- Text Intelligence -->
      <section class="panel">
        <h2><Icon class="icon" icon={wrenchIcon} /> Text Intelligence</h2>

        <!-- Summarize -->
        <div style="margin-bottom: 1.5rem;">
          <h3 style="margin: 0 0 0.5rem;"><Icon class="icon" icon={clipboardListIcon} /> Summarize</h3>
          <div class="control-group">
            <textarea bind:value={summarizeText} placeholder="Enter long text to summarize..." rows="4"></textarea>
          </div>
          <button class="primary" onclick={summarizeTextHandler} disabled={isSummarizing || !summarizeText.trim()}>
            <Icon class="icon" icon={isSummarizing ? hourglassIcon : clipboardListIcon} />
            {isSummarizing ? 'Summarizing...' : 'Summarize'}
          </button>
          {#if summarizeResult}
            <div class="result-box">
              <h4>Summary</h4>
              <pre class="text-result">{summarizeResult.summary}</pre>
            </div>
          {/if}
        </div>

        <!-- Rewrite -->
        <div>
          <h3 style="margin: 0 0 0.5rem;"><Icon class="icon" icon={pencilIcon} /> Rewrite</h3>
          <div class="control-group">
            <textarea bind:value={rewriteText} placeholder="Enter text to rewrite..." rows="3"></textarea>
          </div>
          <div class="control-row" style="margin-bottom: 1rem;">
            <div class="flex-1">
              <label for="rewrite-tone">Tone</label>
              <select id="rewrite-tone" bind:value={rewriteTone}>
                <option value="casual">Casual</option>
                <option value="formal">Formal</option>
                <option value="professional">Professional</option>
              </select>
            </div>
          </div>
          <button class="primary" onclick={rewriteTextHandler} disabled={isRewriting || !rewriteText.trim()}>
            <Icon class="icon" icon={isRewriting ? hourglassIcon : pencilIcon} />
            {isRewriting ? 'Rewriting...' : 'Rewrite'}
          </button>
          {#if rewriteResult}
            <div class="result-box">
              <h4>Rewritten ({rewriteTone})</h4>
              <pre class="text-result">{rewriteResult.rewrittenText}</pre>
            </div>
          {/if}
        </div>
      </section>

    <!-- Logs Tab -->
    {:else if activeTab === 'logs'}
      <section class="panel logs-panel">
        <div class="logs-header">
          <h2><Icon class="icon" icon={scrollTextIcon} /> Activity Logs</h2>
          <button onclick={clearLogs} disabled={logs.length === 0}>Clear</button>
        </div>

        <div class="logs-container">
          {#if logs.length === 0}
            <p class="empty-logs">No activity yet. Try using some features!</p>
          {:else}
            {#each logs as log (log.id)}
              <div class="log-entry {log.type}">
                <span class="log-time">{log.timestamp}</span>
                <span class="log-message">{log.message}</span>
              </div>
            {/each}
          {/if}
        </div>
      </section>

    <!-- Tests Tab -->
    {:else if activeTab === 'tests'}
      <section class="panel">
        <h2><Icon class="icon" icon={flaskConicalIcon} /> End-to-End Tests</h2>
        <p class="description">Run automated tests using the sample data.</p>

        <div class="button-group">
          <button class="primary" onclick={runTests} disabled={isRunningTests}>
            <Icon class="icon" icon={isRunningTests ? hourglassIcon : playIcon} />
            {isRunningTests ? `Running (${testProgress.current}/${testProgress.total})...` : 'Run Tests'}
          </button>
        </div>

        {#if testResults.length > 0}
          <div class="test-results">
            {#each testResults as result}
              <div class="test-item {result.status}">
                <div class="test-header">
                  <Icon class="test-icon" icon={getTestStatusIcon(result.status)} />
                  <span class="test-name">{result.name}</span>
                  <span class="test-status-badge {result.status}">{result.status.toUpperCase()}</span>
                </div>
                <div class="test-desc">{result.description}</div>
                <div class="test-details">{result.details}</div>
              </div>
            {/each}
          </div>
        {/if}
      </section>
    {/if}
  </main>
</div>

<style>
  :global(*) { box-sizing: border-box; }
  :global(body) {
    margin: 0;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
    background: linear-gradient(135deg, #1a1a2e 0%, #16213e 100%);
    color: #e4e4e7;
    min-height: 100vh;
  }

  .app { max-width: 1200px; margin: 0 auto; padding: 1rem; }

  /* Processing Status Banner */
  .processing-banner {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    z-index: 1000;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.75rem;
    padding: 0.75rem 1.5rem;
    background: linear-gradient(90deg, #4f46e5, #7c3aed);
    color: white;
    font-weight: 500;
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.3);
    animation: slideDown 0.3s ease-out;
  }
  .processing-banner.error {
    background: linear-gradient(90deg, #dc2626, #ef4444);
  }
  .processing-banner.success {
    background: linear-gradient(90deg, #059669, #10b981);
  }
  @keyframes slideDown {
    from { transform: translateY(-100%); }
    to { transform: translateY(0); }
  }
  .processing-spinner {
    width: 16px;
    height: 16px;
    border: 2px solid rgba(255, 255, 255, 0.3);
    border-top-color: white;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }
  @keyframes spin {
    to { transform: rotate(360deg); }
  }
  .processing-message {
    font-size: 0.9rem;
  }

  /* Toast Notifications */
  .toast-container {
    position: fixed;
    top: 1rem;
    right: 1rem;
    z-index: 1001;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    max-width: 400px;
  }
  .toast {
    display: flex;
    align-items: flex-start;
    gap: 0.75rem;
    padding: 1rem;
    background: #1e1e2e;
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 12px;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.4);
    animation: toastSlideIn 0.3s ease-out;
  }
  .toast.error {
    background: linear-gradient(135deg, #2d1f1f 0%, #1e1e2e 100%);
    border-color: rgba(239, 68, 68, 0.3);
  }
  .toast.success {
    background: linear-gradient(135deg, #1f2d1f 0%, #1e1e2e 100%);
    border-color: rgba(34, 197, 94, 0.3);
  }
  @keyframes toastSlideIn {
    from { transform: translateX(100%); opacity: 0; }
    to { transform: translateX(0); opacity: 1; }
  }
  .toast-icon {
    width: 1.25rem;
    height: 1.25rem;
    flex-shrink: 0;
  }
  .toast-message {
    flex: 1;
    font-size: 0.9rem;
    line-height: 1.4;
    word-break: break-word;
  }
  .toast-dismiss {
    background: none;
    border: none;
    color: #71717a;
    font-size: 1.25rem;
    cursor: pointer;
    padding: 0;
    line-height: 1;
    flex-shrink: 0;
  }
  .toast-dismiss:hover {
    color: #e4e4e7;
  }

  .icon {
    width: 1em;
    height: 1em;
    flex-shrink: 0;
    vertical-align: -0.15em;
  }

  .header { text-align: center; padding: 1.5rem 0; }
  .header h1 {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    margin: 0;
    font-size: 2rem;
  }
  .title-icon {
    width: 2rem;
    height: 2rem;
  }
  .subtitle { margin: 0.5rem 0 0; color: #71717a; }

  .nav {
    display: flex; gap: 0.5rem; padding: 0.5rem;
    background: rgba(255,255,255,0.05); border-radius: 12px;
    margin-bottom: 1.5rem; flex-wrap: wrap; justify-content: center;
  }
  .nav-btn {
    display: inline-flex;
    align-items: center;
    gap: 0.4rem;
    background: transparent; border: none; color: #a1a1aa;
    padding: 0.75rem 1.25rem; border-radius: 8px; cursor: pointer;
    font-size: 0.9rem; font-weight: 500; transition: all 0.2s;
  }
  .nav-btn:hover { background: rgba(255,255,255,0.1); color: #e4e4e7; }
  .nav-btn.active { background: #4f46e5; color: white; }
  .badge {
    background: #ef4444; color: white; font-size: 0.7rem;
    padding: 0.125rem 0.4rem; border-radius: 999px; margin-left: 0.25rem;
  }

  .panel {
    background: rgba(255,255,255,0.05); border: 1px solid rgba(255,255,255,0.1);
    border-radius: 16px; padding: 1.5rem; margin-bottom: 1.5rem;
  }
  .panel h2,
  .panel h3 {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }
  .panel h2 { margin: 0 0 0.5rem; font-size: 1.25rem; }
  .description { color: #71717a; margin: 0 0 1.5rem; font-size: 0.9rem; }
  .coming-soon { opacity: 0.6; }
  .placeholder { padding: 2rem; text-align: center; color: #71717a; background: rgba(0,0,0,0.2); border-radius: 8px; }

  .info-grid, .capabilities-grid {
    display: grid; grid-template-columns: repeat(auto-fill, minmax(200px, 1fr)); gap: 1rem;
  }
  .info-item { background: rgba(0,0,0,0.2); padding: 1rem; border-radius: 8px; }
  .info-item .label { display: block; font-size: 0.8rem; color: #71717a; margin-bottom: 0.25rem; }
  .info-item .value { font-weight: 500; }
  .status-value {
    display: inline-flex;
    align-items: center;
    gap: 0.35rem;
    color: #ef4444;
  }
  .status-value.available {
    color: #22c55e;
  }

  .capability-card {
    background: rgba(0,0,0,0.3); border: 1px solid rgba(255,255,255,0.05);
    border-radius: 12px; padding: 1rem; opacity: 0.5; transition: all 0.2s;
  }
  .capability-card.available { opacity: 1; background: rgba(34,197,94,0.1); border-color: rgba(34,197,94,0.3); }
  .cap-header { display: flex; justify-content: space-between; margin-bottom: 0.5rem; }
  .cap-icon { width: 1.5rem; height: 1.5rem; }
  .cap-status { width: 1.25rem; height: 1.25rem; color: #ef4444; }
  .capability-card.available .cap-status { color: #22c55e; }
  .cap-label { font-weight: 500; margin-bottom: 0.5rem; }
  .cap-details { display: flex; flex-direction: column; gap: 0.25rem; font-size: 0.75rem; color: #71717a; }
  .cap-details span {
    display: inline-flex;
    align-items: center;
    gap: 0.35rem;
  }
  .cap-details .on-device { color: #22c55e; }

  .control-group { margin-bottom: 1rem; }
  .control-group label { display: block; font-size: 0.85rem; color: #a1a1aa; margin-bottom: 0.5rem; }
  .control-row { display: flex; gap: 1rem; flex-wrap: wrap; }
  .control-row.compact { margin-bottom: 1rem; }
  .flex-1 { flex: 1; min-width: 200px; }

  select, textarea, input[type="text"] {
    width: 100%; padding: 0.75rem; background: rgba(0,0,0,0.3);
    border: 1px solid rgba(255,255,255,0.1); border-radius: 8px;
    color: #e4e4e7; font-size: 0.9rem; font-family: inherit;
  }
  select:focus, textarea:focus, input:focus { outline: none; border-color: #4f46e5; }
  textarea { resize: vertical; min-height: 80px; }
  input[type="range"] { width: 100%; accent-color: #4f46e5; }
  .checkbox-label { display: flex; align-items: center; gap: 0.5rem; font-size: 0.85rem; cursor: pointer; }
  .checkbox-label input { width: auto; }

  .button-group { display: flex; gap: 0.75rem; flex-wrap: wrap; }
  button {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 0.4rem;
    padding: 0.75rem 1.5rem; background: rgba(255,255,255,0.1);
    border: 1px solid rgba(255,255,255,0.1); border-radius: 8px;
    color: #e4e4e7; font-size: 0.9rem; font-weight: 500; cursor: pointer; transition: all 0.2s;
  }
  button:hover:not(:disabled) { background: rgba(255,255,255,0.15); }
  button:disabled { opacity: 0.5; cursor: not-allowed; }
  button.primary { background: #4f46e5; border-color: #4f46e5; color: white; }
  button.primary:hover:not(:disabled) { background: #4338ca; }
  button.danger { background: #ef4444; border-color: #ef4444; color: white; }

  .file-input-wrapper { margin-bottom: 1rem; }
  .file-input-wrapper input[type="file"] {
    width: 100%; padding: 1rem; background: rgba(0,0,0,0.2);
    border: 2px dashed rgba(255,255,255,0.2); border-radius: 8px; color: #a1a1aa; cursor: pointer;
  }
  .file-input-wrapper input[type="file"]:hover { border-color: #4f46e5; }

  .result-box {
    background: rgba(0,0,0,0.2); border: 1px solid rgba(255,255,255,0.1);
    border-left: 3px solid #4f46e5; border-radius: 8px; padding: 1rem; margin-top: 1rem;
  }
  .result-box h4 { margin: 0 0 0.75rem; font-size: 0.9rem; color: #a1a1aa; }
  .recognized-text { font-size: 1.1rem; font-style: italic; margin: 0 0 0.75rem; }
  .result-meta { display: flex; gap: 1rem; font-size: 0.8rem; color: #71717a; }
  .text-result {
    background: rgba(0,0,0,0.3); padding: 1rem; border-radius: 6px;
    white-space: pre-wrap; word-break: break-word; font-family: inherit;
    font-size: 0.9rem; margin: 0; max-height: 200px; overflow-y: auto;
  }
  .no-results { color: #71717a; font-style: italic; text-align: center; padding: 1rem; }

  .vision-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 1.5rem; }
  .vision-grid .panel { margin-bottom: 0; }
  .image-preview { margin-bottom: 1rem; border-radius: 8px; overflow: hidden; background: rgba(0,0,0,0.3); }
  .image-preview img { width: 100%; height: auto; max-height: 200px; object-fit: contain; display: block; }

  .barcode-item {
    display: flex; align-items: center; gap: 0.75rem; padding: 0.5rem;
    background: rgba(0,0,0,0.2); border-radius: 6px; margin-bottom: 0.5rem;
  }
  .barcode-format {
    background: #4f46e5; color: white; padding: 0.25rem 0.5rem;
    border-radius: 4px; font-size: 0.75rem; font-weight: 600; text-transform: uppercase;
  }
  .barcode-value { font-family: 'Monaco', monospace; font-size: 0.85rem; word-break: break-all; }

  .face-item { padding: 0.75rem; background: rgba(0,0,0,0.2); border-radius: 6px; margin-bottom: 0.5rem; }
  .face-attrs { display: flex; gap: 1rem; margin-top: 0.5rem; font-size: 0.8rem; color: #a1a1aa; }

  .classification-list { display: flex; flex-direction: column; gap: 0.5rem; }
  .classification-item { display: flex; align-items: center; gap: 0.75rem; }
  .cls-label { min-width: 100px; font-size: 0.85rem; }
  .cls-bar-wrapper { flex: 1; height: 8px; background: rgba(0,0,0,0.3); border-radius: 4px; overflow: hidden; }
  .cls-bar { height: 100%; background: linear-gradient(90deg, #4f46e5, #7c3aed); border-radius: 4px; }
  .cls-confidence { min-width: 50px; font-size: 0.8rem; color: #71717a; text-align: right; }

  .sample-texts { display: flex; flex-wrap: wrap; gap: 0.5rem; align-items: center; margin-bottom: 1rem; }
  .sample-label { font-size: 0.85rem; color: #71717a; }
  .sample-btn { padding: 0.4rem 0.75rem; font-size: 0.8rem; }
  .lang-result { display: flex; align-items: center; gap: 1rem; }
  .lang-code { font-size: 1.5rem; font-weight: 600; background: rgba(79,70,229,0.2); padding: 0.5rem 1rem; border-radius: 8px; }
  .lang-confidence { color: #71717a; }

  .logs-panel { display: flex; flex-direction: column; max-height: 600px; }
  .logs-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 1rem; }
  .logs-header h2 { margin: 0; }
  .logs-container {
    flex: 1; overflow-y: auto; background: rgba(0,0,0,0.3); border-radius: 8px;
    padding: 1rem; font-family: 'Monaco', monospace; font-size: 0.8rem;
  }
  .empty-logs { color: #71717a; text-align: center; font-style: italic; }
  .log-entry { padding: 0.4rem 0; border-bottom: 1px solid rgba(255,255,255,0.05); display: flex; gap: 0.75rem; }
  .log-entry:last-child { border-bottom: none; }
  .log-time { color: #52525b; flex-shrink: 0; }
  .log-message { word-break: break-word; }
  .log-entry.success .log-message { color: #4ade80; }
  .log-entry.error .log-message { color: #f87171; }
  .log-entry.info .log-message { color: #60a5fa; }

  .loading { text-align: center; padding: 2rem; color: #71717a; }
  .hint { font-size: 0.8rem; color: #71717a; margin-top: 0.75rem; }

  /* Test Results */
  .test-results { display: flex; flex-direction: column; gap: 0.75rem; margin-top: 1.5rem; }
  .test-item {
    background: rgba(0,0,0,0.2); border: 1px solid rgba(255,255,255,0.05);
    border-radius: 8px; padding: 1rem;
  }
  .test-item.pass { border-left: 3px solid #22c55e; background: rgba(34,197,94,0.05); }
  .test-item.fail { border-left: 3px solid #ef4444; background: rgba(239,68,68,0.05); }
  .test-item.error { border-left: 3px solid #f59e0b; background: rgba(245,158,11,0.05); }
  .test-item.skipped { border-left: 3px solid #71717a; }

  .test-header { display: flex; align-items: center; gap: 0.75rem; margin-bottom: 0.25rem; }
  .test-name { font-weight: 600; flex: 1; }
  .test-status-badge {
    font-size: 0.7rem; font-weight: 700; padding: 0.15rem 0.4rem; border-radius: 4px;
  }
  .test-status-badge.pass { background: rgba(34,197,94,0.2); color: #4ade80; }
  .test-status-badge.fail { background: rgba(239,68,68,0.2); color: #f87171; }
  .test-status-badge.error { background: rgba(245,158,11,0.2); color: #fbbf24; }
  .test-status-badge.skipped { background: rgba(113,113,122,0.2); color: #a1a1aa; }

  .test-desc { font-size: 0.85rem; color: #a1a1aa; margin-bottom: 0.5rem; margin-left: 2rem; }
  .test-details { font-family: 'Monaco', monospace; font-size: 0.8rem; color: #e4e4e7; margin-left: 2rem; }

  /* LLM */
  .control-row { display: flex; gap: 1rem; align-items: end; }
  .flex-1 { flex: 1; }
  .control-group label, .flex-1 label { display: block; margin-bottom: 0.25rem; font-size: 0.85rem; color: #a1a1aa; }
  .control-group textarea, .control-group input[type="text"] { width: 100%; background: rgba(0,0,0,0.3); border: 1px solid #3f3f46; border-radius: 6px; color: #e4e4e7; padding: 0.5rem; font-size: 0.9rem; font-family: inherit; resize: vertical; }
  input[type="range"] { width: 100%; accent-color: #3b82f6; }
  select { background: rgba(0,0,0,0.3); border: 1px solid #3f3f46; border-radius: 6px; color: #e4e4e7; padding: 0.5rem; font-size: 0.9rem; width: 100%; }
  .text-result { white-space: pre-wrap; word-wrap: break-word; font-family: inherit; font-size: 0.9rem; margin: 0; line-height: 1.5; }
  .result-meta { display: flex; gap: 1rem; margin-top: 0.5rem; font-size: 0.8rem; color: #71717a; }
  .cursor-blink { animation: blink 0.8s step-end infinite; }
  @keyframes blink { 50% { opacity: 0; } }
  .hint { font-size: 0.8rem; color: #71717a; margin-top: 0.5rem; }

  .chat-container { max-height: 400px; overflow-y: auto; padding: 0.5rem; background: rgba(0,0,0,0.2); border-radius: 8px; margin-bottom: 1rem; display: flex; flex-direction: column; gap: 0.5rem; }
  .chat-msg { display: flex; gap: 0.5rem; padding: 0.5rem; border-radius: 6px; }
  .chat-msg.user { background: rgba(59,130,246,0.15); }
  .chat-msg.assistant { background: rgba(34,197,94,0.1); }
  .chat-msg.error { background: rgba(239,68,68,0.15); }
  .chat-role { font-size: 1.2rem; flex-shrink: 0; }
  .chat-content { white-space: pre-wrap; word-wrap: break-word; line-height: 1.5; font-size: 0.9rem; }
  .chat-content.thinking { color: #71717a; font-style: italic; }
  .chat-input-row { display: flex; gap: 0.5rem; }
  .chat-input-row input { flex: 1; background: rgba(0,0,0,0.3); border: 1px solid #3f3f46; border-radius: 6px; color: #e4e4e7; padding: 0.5rem; font-size: 0.9rem; }
  button.danger { background: #dc2626; color: white; border: none; border-radius: 6px; padding: 0.5rem 1rem; cursor: pointer; font-weight: 600; }
  button.danger:hover { background: #b91c1c; }
</style>
