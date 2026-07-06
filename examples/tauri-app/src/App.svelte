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
    <button class="nav-btn" class:active={activeTab === 'capabilities'} aria-pressed={activeTab === 'capabilities'} onclick={() => activeTab = 'capabilities'}>
      <Icon class="icon" icon={clipboardListIcon} /> Capabilities
    </button>
    <button class="nav-btn" class:active={activeTab === 'speech'} aria-pressed={activeTab === 'speech'} onclick={() => activeTab = 'speech'}>
      <Icon class="icon" icon={micIcon} /> Speech
    </button>
    <button class="nav-btn" class:active={activeTab === 'vision'} aria-pressed={activeTab === 'vision'} onclick={() => activeTab = 'vision'}>
      <Icon class="icon" icon={eyeIcon} /> Vision
    </button>
    <button class="nav-btn" class:active={activeTab === 'text'} aria-pressed={activeTab === 'text'} onclick={() => activeTab = 'text'}>
      <Icon class="icon" icon={fileTextIcon} /> Text
    </button>
    <button class="nav-btn" class:active={activeTab === 'llm'} aria-pressed={activeTab === 'llm'} onclick={() => activeTab = 'llm'}>
      <Icon class="icon" icon={brainIcon} /> LLM
    </button>
    <button class="nav-btn" class:active={activeTab === 'logs'} aria-pressed={activeTab === 'logs'} onclick={() => activeTab = 'logs'}>
      <Icon class="icon" icon={scrollTextIcon} /> Logs {#if logs.length > 0}<span class="badge">{logs.length}</span>{/if}
    </button>
    <button class="nav-btn" class:active={activeTab === 'tests'} aria-pressed={activeTab === 'tests'} onclick={() => activeTab = 'tests'}>
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
                <span class="value status-value" class:available={llmAvailability.available}>
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

  :global(:root) {
    --surface-page: oklch(17% 0.025 205);
    --surface-shell: oklch(22% 0.03 205);
    --surface-panel: oklch(24% 0.028 205);
    --surface-card: oklch(20% 0.026 205);
    --surface-control: oklch(19% 0.024 205);
    --surface-control-hover: oklch(25% 0.032 205);
    --border-soft: oklch(38% 0.035 205);
    --border-strong: oklch(48% 0.06 190);
    --text-main: oklch(92% 0.012 205);
    --text-muted: oklch(66% 0.018 205);
    --text-dim: oklch(54% 0.018 205);
    --accent: oklch(72% 0.12 185);
    --accent-strong: oklch(64% 0.14 185);
    --accent-soft: oklch(30% 0.065 185);
    --accent-warm: oklch(76% 0.14 70);
    --success: oklch(74% 0.16 155);
    --danger: oklch(68% 0.2 25);
    --warning: oklch(78% 0.15 75);
    --info: oklch(72% 0.11 220);
    --space-2xs: 0.25rem;
    --space-xs: 0.5rem;
    --space-sm: 0.75rem;
    --space-md: 1rem;
    --space-lg: 1.5rem;
    --space-xl: 2rem;
    --space-2xl: 3rem;
    --radius-sm: 0.375rem;
    --radius-md: 0.5rem;
    --radius-lg: 0.75rem;
  }

  :global(body) {
    min-height: 100vh;
    margin: 0;
    background:
      radial-gradient(circle at 12% 0%, oklch(29% 0.07 185 / 0.5), transparent 34rem),
      linear-gradient(145deg, var(--surface-page), oklch(14% 0.03 210));
    color: var(--text-main);
    font-family: ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
  }

  .app {
    width: min(76rem, 100%);
    margin: 0 auto;
    padding: clamp(var(--space-md), 2vw, var(--space-xl));
  }

  .processing-banner {
    position: fixed;
    z-index: 1000;
    inset-block-start: 0;
    inset-inline: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: var(--space-sm);
    padding: var(--space-sm) var(--space-lg);
    background: var(--accent-soft);
    color: var(--text-main);
    font-weight: 650;
    box-shadow: 0 0.75rem 2rem oklch(8% 0.02 205 / 0.35);
    animation: slideDown 0.3s ease-out;
  }

  .processing-banner.error { background: oklch(28% 0.08 25); }
  .processing-banner.success { background: oklch(30% 0.08 155); }

  @keyframes slideDown {
    from { transform: translateY(-100%); }
    to { transform: translateY(0); }
  }

  .processing-spinner {
    width: 1rem;
    height: 1rem;
    border: 2px solid oklch(96% 0.01 205 / 0.28);
    border-top-color: var(--accent);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  .processing-message { font-size: 0.9rem; }

  .toast-container {
    position: fixed;
    z-index: 1001;
    inset-block-start: var(--space-md);
    inset-inline-end: var(--space-md);
    display: flex;
    flex-direction: column;
    gap: var(--space-xs);
    max-width: 25rem;
  }

  .toast {
    display: flex;
    align-items: flex-start;
    gap: var(--space-sm);
    padding: var(--space-md);
    background: var(--surface-shell);
    border: 1px solid var(--border-soft);
    border-radius: var(--radius-lg);
    box-shadow: 0 1rem 2rem oklch(8% 0.02 205 / 0.45);
    animation: toastSlideIn 0.3s ease-out;
  }

  .toast.error { border-color: color-mix(in oklch, var(--danger), transparent 45%); }
  .toast.success { border-color: color-mix(in oklch, var(--success), transparent 45%); }

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
    line-height: 1.45;
    word-break: break-word;
  }

  .toast-dismiss {
    flex-shrink: 0;
    padding: var(--space-2xs);
    background: transparent;
    border: 0;
    color: var(--text-dim);
    line-height: 1;
  }

  .toast-dismiss:hover { color: var(--text-main); }

  .icon {
    width: 1em;
    height: 1em;
    flex-shrink: 0;
    vertical-align: -0.15em;
  }

  .header {
    display: grid;
    justify-items: start;
    gap: var(--space-xs);
    padding-block: var(--space-xl) var(--space-lg);
  }

  .header h1 {
    display: inline-flex;
    align-items: center;
    gap: var(--space-sm);
    margin: 0;
    font-size: clamp(2rem, 4vw, 3.25rem);
    line-height: 1;
    letter-spacing: 0;
  }

  .title-icon {
    width: 2.25rem;
    height: 2.25rem;
    color: var(--accent);
  }

  .subtitle {
    margin: 0;
    color: var(--text-muted);
    font-size: 1rem;
  }

  .nav {
    position: sticky;
    z-index: 20;
    top: var(--space-md);
    display: flex;
    gap: var(--space-2xs);
    overflow-x: auto;
    margin-block-end: var(--space-xl);
    padding: var(--space-2xs);
    background: color-mix(in oklch, var(--surface-shell), transparent 8%);
    border: 1px solid var(--border-soft);
    border-radius: var(--radius-lg);
    box-shadow: 0 0.75rem 2rem oklch(8% 0.02 205 / 0.3);
  }

  .nav-btn {
    position: relative;
    display: inline-flex;
    flex: 1 0 max-content;
    align-items: center;
    justify-content: center;
    gap: var(--space-xs);
    min-height: 2.625rem;
    padding: var(--space-sm) var(--space-md);
    background: transparent;
    border: 1px solid transparent;
    border-radius: var(--radius-md);
    color: var(--text-muted);
    font-size: 0.9rem;
    font-weight: 650;
    white-space: nowrap;
    cursor: pointer;
    transition: background 0.18s ease, border-color 0.18s ease, color 0.18s ease, transform 0.18s ease;
  }

  .nav-btn:hover {
    background: var(--surface-control-hover);
    color: var(--text-main);
  }

  .nav-btn.active {
    background: color-mix(in oklch, var(--accent-soft), var(--surface-shell) 24%);
    border-color: color-mix(in oklch, var(--accent), transparent 35%);
    color: var(--text-main);
    box-shadow: inset 0 -0.125rem 0 var(--accent);
  }

  .badge {
    min-width: 1.35rem;
    padding: 0.1rem 0.4rem;
    background: var(--accent-warm);
    border-radius: 999px;
    color: oklch(16% 0.03 70);
    font-size: 0.72rem;
    font-weight: 800;
  }

  .panel {
    margin-block-end: var(--space-lg);
    padding: clamp(var(--space-lg), 2.4vw, var(--space-xl));
    background: color-mix(in oklch, var(--surface-panel), transparent 7%);
    border: 1px solid var(--border-soft);
    border-radius: var(--radius-lg);
  }

  .panel h2,
  .panel h3 {
    display: flex;
    align-items: center;
    gap: var(--space-xs);
    color: var(--text-main);
  }

  .panel h2 {
    margin: 0 0 var(--space-xs);
    font-size: 1.25rem;
    line-height: 1.2;
  }

  .panel h3 {
    margin: 0 0 var(--space-xs);
    font-size: 1rem;
  }

  .description {
    max-width: 65ch;
    margin: 0 0 var(--space-lg);
    color: var(--text-muted);
    font-size: 0.9rem;
    line-height: 1.55;
  }

  .coming-soon { opacity: 0.7; }

  .placeholder {
    padding: var(--space-xl);
    background: var(--surface-card);
    border: 1px solid var(--border-soft);
    border-radius: var(--radius-md);
    color: var(--text-muted);
    text-align: center;
  }

  .info-grid,
  .capabilities-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(13rem, 1fr));
    gap: var(--space-md);
  }

  .info-item,
  .capability-card,
  .test-item {
    background: var(--surface-card);
    border: 1px solid color-mix(in oklch, var(--border-soft), transparent 28%);
    border-radius: var(--radius-md);
  }

  .info-item {
    padding: var(--space-lg);
  }

  .info-item .label {
    display: block;
    margin-block-end: var(--space-sm);
    color: var(--text-dim);
    font-size: 0.82rem;
  }

  .info-item .value {
    font-weight: 650;
  }

  .status-value {
    display: inline-flex;
    align-items: center;
    gap: var(--space-xs);
    color: var(--danger);
  }

  .status-value.available {
    color: var(--success);
  }

  .capability-card {
    padding: var(--space-md);
    opacity: 0.62;
    transition: border-color 0.18s ease, opacity 0.18s ease, transform 0.18s ease;
  }

  .capability-card.available {
    border-color: color-mix(in oklch, var(--success), transparent 48%);
    opacity: 1;
  }

  .cap-header {
    display: flex;
    justify-content: space-between;
    margin-block-end: var(--space-sm);
  }

  .cap-icon {
    width: 1.5rem;
    height: 1.5rem;
    color: var(--accent);
  }

  .cap-status {
    width: 1.2rem;
    height: 1.2rem;
    color: var(--danger);
  }

  .capability-card.available .cap-status {
    color: var(--success);
  }

  .cap-label {
    margin-block-end: var(--space-sm);
    font-weight: 650;
  }

  .cap-details {
    display: flex;
    flex-direction: column;
    gap: var(--space-2xs);
    color: var(--text-muted);
    font-size: 0.75rem;
  }

  .cap-details span,
  .face-bbox,
  .face-orientation span,
  .face-attrs span {
    display: inline-flex;
    align-items: center;
    gap: var(--space-xs);
  }

  .cap-details .on-device {
    color: var(--success);
  }

  .control-group {
    margin-block-end: var(--space-md);
  }

  .control-group label,
  .flex-1 label {
    display: block;
    margin-block-end: var(--space-xs);
    color: var(--text-muted);
    font-size: 0.85rem;
  }

  .control-row {
    display: flex;
    align-items: end;
    flex-wrap: wrap;
    gap: var(--space-md);
  }

  .control-row.compact {
    align-items: center;
    margin-block-end: var(--space-md);
  }

  .flex-1 {
    flex: 1;
    min-width: 12.5rem;
  }

  select,
  textarea,
  input[type="text"],
  .chat-input-row input {
    width: 100%;
    padding: var(--space-sm) var(--space-md);
    background: var(--surface-control);
    border: 1px solid var(--border-soft);
    border-radius: var(--radius-md);
    color: var(--text-main);
    font-family: inherit;
    font-size: 0.9rem;
  }

  select:focus,
  textarea:focus,
  input:focus {
    outline: 2px solid color-mix(in oklch, var(--accent), transparent 45%);
    outline-offset: 2px;
    border-color: var(--accent);
  }

  textarea {
    min-height: 5rem;
    resize: vertical;
  }

  input[type="range"] {
    width: 100%;
    accent-color: var(--accent);
  }

  .checkbox-label {
    display: flex;
    align-items: center;
    gap: var(--space-xs);
    color: var(--text-muted);
    font-size: 0.85rem;
    cursor: pointer;
  }

  .checkbox-label input {
    width: auto;
    accent-color: var(--accent);
  }

  .button-group {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-sm);
  }

  button {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: var(--space-xs);
    min-height: 2.5rem;
    padding: var(--space-sm) var(--space-md);
    background: var(--surface-control);
    border: 1px solid var(--border-soft);
    border-radius: var(--radius-md);
    color: var(--text-main);
    font-size: 0.9rem;
    font-weight: 650;
    cursor: pointer;
    transition: background 0.18s ease, border-color 0.18s ease, transform 0.18s ease;
  }

  button:hover:not(:disabled) {
    background: var(--surface-control-hover);
    border-color: var(--border-strong);
  }

  button:active:not(:disabled) {
    transform: translateY(1px);
  }

  button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  button.primary {
    background: var(--accent-strong);
    border-color: var(--accent);
    color: oklch(15% 0.03 185);
  }

  button.primary:hover:not(:disabled) {
    background: var(--accent);
  }

  button.danger {
    background: color-mix(in oklch, var(--danger), var(--surface-control) 35%);
    border-color: color-mix(in oklch, var(--danger), transparent 25%);
    color: var(--text-main);
  }

  button.danger:hover:not(:disabled) {
    background: var(--danger);
  }

  .file-input-wrapper {
    margin-block-end: var(--space-md);
  }

  .file-input-wrapper input[type="file"] {
    width: 100%;
    padding: var(--space-md);
    background: var(--surface-control);
    border: 1px dashed var(--border-strong);
    border-radius: var(--radius-md);
    color: var(--text-muted);
    cursor: pointer;
  }

  .file-input-wrapper input[type="file"]:hover {
    border-color: var(--accent);
  }

  .result-box {
    margin-block-start: var(--space-md);
    padding: var(--space-md);
    background: var(--surface-card);
    border: 1px solid var(--border-soft);
    border-radius: var(--radius-md);
  }

  .result-box h4 {
    margin: 0 0 var(--space-sm);
    color: var(--text-muted);
    font-size: 0.9rem;
  }

  .recognized-text {
    margin: 0 0 var(--space-sm);
    font-size: 1.1rem;
    font-style: italic;
  }

  .result-meta {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-md);
    margin-block-start: var(--space-xs);
    color: var(--text-dim);
    font-size: 0.8rem;
  }

  .text-result {
    max-height: 12.5rem;
    margin: 0;
    padding: var(--space-md);
    overflow-y: auto;
    background: var(--surface-control);
    border-radius: var(--radius-sm);
    font-family: inherit;
    font-size: 0.9rem;
    line-height: 1.55;
    white-space: pre-wrap;
    word-break: break-word;
  }

  .no-results,
  .loading,
  .empty-logs {
    padding: var(--space-md);
    color: var(--text-dim);
    font-style: italic;
    text-align: center;
  }

  .vision-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(19rem, 1fr));
    gap: var(--space-lg);
  }

  .vision-grid .panel {
    margin-block-end: 0;
  }

  .image-preview {
    overflow: hidden;
    margin-block-end: var(--space-md);
    background: var(--surface-control);
    border-radius: var(--radius-md);
  }

  .image-preview img {
    display: block;
    width: 100%;
    height: auto;
    max-height: 12.5rem;
    object-fit: contain;
  }

  .barcode-item,
  .face-item {
    padding: var(--space-sm);
    background: var(--surface-control);
    border: 1px solid color-mix(in oklch, var(--border-soft), transparent 35%);
    border-radius: var(--radius-sm);
  }

  .barcode-item {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    margin-block-end: var(--space-xs);
  }

  .barcode-format {
    padding: 0.25rem 0.5rem;
    background: color-mix(in oklch, var(--accent-warm), transparent 8%);
    border-radius: var(--radius-sm);
    color: oklch(18% 0.035 70);
    font-size: 0.75rem;
    font-weight: 800;
    text-transform: uppercase;
  }

  .barcode-value {
    font-family: "SFMono-Regular", Consolas, monospace;
    font-size: 0.85rem;
    word-break: break-all;
  }

  .face-item {
    display: grid;
    gap: var(--space-xs);
    margin-block-end: var(--space-xs);
  }

  .face-attrs,
  .face-orientation,
  .face-landmarks {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-sm);
    color: var(--text-muted);
    font-size: 0.8rem;
  }

  .landmarks-title {
    display: inline-flex;
    align-items: center;
    gap: var(--space-xs);
  }

  .landmarks-grid {
    display: grid;
    gap: var(--space-2xs);
    width: 100%;
  }

  .classification-list {
    display: flex;
    flex-direction: column;
    gap: var(--space-xs);
  }

  .classification-item {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
  }

  .cls-label {
    min-width: 6.25rem;
    font-size: 0.85rem;
  }

  .cls-bar-wrapper {
    flex: 1;
    height: 0.5rem;
    overflow: hidden;
    background: var(--surface-control);
    border-radius: 999px;
  }

  .cls-bar {
    height: 100%;
    background: var(--accent-warm);
    border-radius: 999px;
  }

  .cls-confidence {
    min-width: 3.125rem;
    color: var(--text-dim);
    font-size: 0.8rem;
    text-align: right;
  }

  .sample-texts {
    display: flex;
    align-items: center;
    flex-wrap: wrap;
    gap: var(--space-xs);
    margin-block-end: var(--space-md);
  }

  .sample-label,
  .hint,
  .lang-confidence {
    color: var(--text-dim);
    font-size: 0.85rem;
  }

  .sample-btn {
    min-height: 2rem;
    padding: var(--space-xs) var(--space-sm);
    font-size: 0.8rem;
  }

  .lang-result {
    display: flex;
    align-items: center;
    gap: var(--space-md);
  }

  .lang-code {
    padding: var(--space-xs) var(--space-md);
    background: color-mix(in oklch, var(--accent), transparent 78%);
    border: 1px solid color-mix(in oklch, var(--accent), transparent 45%);
    border-radius: var(--radius-md);
    color: var(--accent);
    font-size: 1.5rem;
    font-weight: 750;
  }

  .logs-panel {
    display: flex;
    flex-direction: column;
    max-height: 37.5rem;
  }

  .logs-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--space-md);
    margin-block-end: var(--space-md);
  }

  .logs-header h2 {
    margin: 0;
  }

  .logs-container,
  .chat-container {
    overflow-y: auto;
    background: var(--surface-control);
    border: 1px solid color-mix(in oklch, var(--border-soft), transparent 35%);
    border-radius: var(--radius-md);
  }

  .logs-container {
    flex: 1;
    padding: var(--space-md);
    font-family: "SFMono-Regular", Consolas, monospace;
    font-size: 0.8rem;
  }

  .log-entry {
    display: flex;
    gap: var(--space-sm);
    padding-block: var(--space-xs);
    border-bottom: 1px solid color-mix(in oklch, var(--border-soft), transparent 55%);
  }

  .log-entry:last-child {
    border-bottom: 0;
  }

  .log-time {
    flex-shrink: 0;
    color: var(--text-dim);
  }

  .log-message {
    word-break: break-word;
  }

  .log-entry.success .log-message { color: var(--success); }
  .log-entry.error .log-message { color: var(--danger); }
  .log-entry.info .log-message { color: var(--info); }

  .test-results {
    display: flex;
    flex-direction: column;
    gap: var(--space-sm);
    margin-block-start: var(--space-lg);
  }

  .test-item {
    padding: var(--space-md);
  }

  .test-item.pass {
    border-color: color-mix(in oklch, var(--success), transparent 45%);
    background: color-mix(in oklch, var(--success), var(--surface-card) 88%);
  }

  .test-item.fail {
    border-color: color-mix(in oklch, var(--danger), transparent 42%);
    background: color-mix(in oklch, var(--danger), var(--surface-card) 88%);
  }

  .test-item.error {
    border-color: color-mix(in oklch, var(--warning), transparent 42%);
    background: color-mix(in oklch, var(--warning), var(--surface-card) 88%);
  }

  .test-item.skipped {
    border-color: var(--border-soft);
  }

  .test-header {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    margin-block-end: var(--space-2xs);
  }

  .test-icon,
  .chat-role {
    width: 1.2rem;
    height: 1.2rem;
    flex-shrink: 0;
  }

  .test-name {
    flex: 1;
    font-weight: 700;
  }

  .test-status-badge {
    padding: 0.15rem 0.4rem;
    border-radius: var(--radius-sm);
    font-size: 0.7rem;
    font-weight: 800;
  }

  .test-status-badge.pass { background: color-mix(in oklch, var(--success), transparent 78%); color: var(--success); }
  .test-status-badge.fail { background: color-mix(in oklch, var(--danger), transparent 78%); color: var(--danger); }
  .test-status-badge.error { background: color-mix(in oklch, var(--warning), transparent 78%); color: var(--warning); }
  .test-status-badge.skipped { background: color-mix(in oklch, var(--border-soft), transparent 65%); color: var(--text-muted); }

  .test-desc,
  .test-details {
    margin-inline-start: 2rem;
  }

  .test-desc {
    margin-block-end: var(--space-xs);
    color: var(--text-muted);
    font-size: 0.85rem;
  }

  .test-details {
    color: var(--text-main);
    font-family: "SFMono-Regular", Consolas, monospace;
    font-size: 0.8rem;
  }

  .cursor-blink {
    animation: blink 0.8s step-end infinite;
  }

  @keyframes blink {
    50% { opacity: 0; }
  }

  .chat-container {
    display: flex;
    flex-direction: column;
    gap: var(--space-xs);
    max-height: 25rem;
    margin-block-end: var(--space-md);
    padding: var(--space-sm);
  }

  .chat-msg {
    display: flex;
    gap: var(--space-xs);
    padding: var(--space-sm);
    border-radius: var(--radius-sm);
  }

  .chat-msg.user { background: color-mix(in oklch, var(--info), transparent 84%); }
  .chat-msg.assistant { background: color-mix(in oklch, var(--success), transparent 88%); }
  .chat-msg.error { background: color-mix(in oklch, var(--danger), transparent 84%); }

  .chat-content {
    font-size: 0.9rem;
    line-height: 1.5;
    white-space: pre-wrap;
    word-wrap: break-word;
  }

  .chat-content.thinking {
    color: var(--text-dim);
    font-style: italic;
  }

  .chat-input-row {
    display: flex;
    gap: var(--space-xs);
  }

  @media (max-width: 42rem) {
    .app {
      padding: var(--space-md);
    }

    .header {
      padding-block-start: var(--space-lg);
    }

    .header h1 {
      font-size: 2rem;
    }

    .nav {
      top: var(--space-xs);
      margin-inline: calc(var(--space-md) * -1);
      border-inline: 0;
      border-radius: 0;
    }

    .nav-btn {
      flex: 0 0 auto;
    }

    .chat-input-row,
    .logs-header {
      align-items: stretch;
      flex-direction: column;
    }
  }
</style>
