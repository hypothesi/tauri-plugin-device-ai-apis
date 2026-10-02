<script>
   import {
      getCapabilities,
      speech,
      vision,
      text,
      llm,
      isModelNotInstalled,
      isLanguageNotSupported,
      isTauri,
      isWeb,
      hasWebSpeechRecognition,
      hasWebSpeechSynthesis,
      hasBarcodeDetection,
   } from '@hypothesi/tauri-plugin-device-ai-apis';

   import { navItems } from './demo-config.js';
   import { runDemoTestSuite } from './demo-test-runner.js';
   import { blobToBase64, fileToDataUrl, getErrorMessage } from './demo-utils.js';
   import SidebarNav from './components/sidebar-nav.svelte';
   import TopBar from './components/top-bar.svelte';
   import ToastStack from './components/toast-stack.svelte';
   import CapabilitiesTab from './components/tabs/capabilities-tab.svelte';
   import SpeechTab from './components/tabs/speech-tab.svelte';
   import VisionTab from './components/tabs/vision-tab.svelte';
   import TextTab from './components/tabs/text-tab.svelte';
   import LlmTab from './components/tabs/llm-tab.svelte';
   import LogsTab from './components/tabs/logs-tab.svelte';
   import TestsTab from './components/tabs/tests-tab.svelte';

   let activeTab = $state('capabilities'),
      capabilities = $state(null),
      platformInfo = $state({ isTauri: false, isWeb: false });

   let isRecognizing = $state(false),
      recognitionResult = $state(null),
      recognitionLanguage = $state('en-US'),
      streamingSessionId = $state(null);

   let ttsText = $state('Hello! This is a demonstration of text-to-speech synthesis.'),
      voices = $state([]),
      selectedVoice = $state(''),
      ttsRate = $state(1.0),
      ttsPitch = $state(1.0),
      isSpeaking = $state(false);

   let ocrResult = $state(null),
      ocrImage = $state(null),
      isProcessingOcr = $state(false);

   let barcodeResults = $state([]),
      barcodeImage = $state(null),
      isProcessingBarcode = $state(false);

   let faceResults = $state([]),
      faceImage = $state(null),
      isProcessingFaces = $state(false),
      detectLandmarks = $state(true),
      classifyAttributes = $state(true);

   let classificationResults = $state([]),
      classificationImage = $state(null),
      isProcessingClassification = $state(false),
      maxClassifications = $state(5),
      minConfidence = $state(0.1);

   let langIdText = $state(
         'Bonjour, comment allez-vous? Je suis tres heureux de vous rencontrer.',
      ),
      langIdResult = $state(null),
      isIdentifyingLang = $state(false),
      translateText = $state('Hello, how are you? I am very pleased to meet you.'),
      translateFrom = $state('en'),
      translateTo = $state('es'),
      translateResult = $state(null),
      isTranslating = $state(false);

   let llmAvailability = $state(null),
      llmModelInfo = $state(null),
      llmPrompt = $state('Explain quantum computing in one paragraph.'),
      llmSystemPrompt = $state(''),
      llmTemperature = $state(0.7),
      llmMaxTokens = $state(512),
      llmResult = $state(null),
      llmStreamContent = $state(''),
      isGenerating = $state(false),
      isStreaming = $state(false),
      llmSessionId = $state(null),
      llmChatHistory = $state([]),
      llmChatInput = $state(''),
      isSendingChat = $state(false);

   let summarizeText = $state(
         'Artificial intelligence has transformed the way we interact with technology. ' +
         'From virtual assistants to autonomous vehicles, AI systems are becoming increasingly ' +
         'integrated into our daily lives. Machine learning, a subset of AI, enables computers ' +
         'to learn from data without being explicitly programmed. Deep learning, which uses ' +
         'neural networks with many layers, has achieved remarkable results in image recognition, ' +
         'natural language processing, and game playing.',
      ),
      summarizeResult = $state(null),
      isSummarizing = $state(false),
      rewriteText = $state('hey wanna grab lunch tmrw? lmk if ur free'),
      rewriteTone = $state('formal'),
      rewriteResult = $state(null),
      isRewriting = $state(false);

   let logs = $state([]),
      processingStatus = $state(null),
      toasts = $state([]),
      testResults = $state([]),
      isRunningTests = $state(false),
      testProgress = $state({ current: 0, total: 0 });

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
      toasts = toasts.filter((toast) => {
         return toast.id !== id;
      });
   }

   function showError(message) {
      showToast(message, 'error', 8000);
   }

   function log(message, type = 'info') {
      const timestamp = new Date().toLocaleTimeString();

      logs = [{ timestamp, message, type, id: Date.now() }, ...logs].slice(0, 100);
   }

   function clearLogs() {
      logs = [];
   }

   function listSidebarItems() {
      return navItems.map((item) => {
         return item.id === 'logs' ? { ...item, badge: logs.length } : item;
      });
   }

   async function loadCapabilities() {
      try {
         platformInfo = {
            isTauri: isTauri(),
            isWeb: isWeb(),
            hasWebSpeech: hasWebSpeechRecognition(),
            hasWebSynthesis: hasWebSpeechSynthesis(),
            hasWebBarcode: hasBarcodeDetection(),
         };
         capabilities = await getCapabilities();
         log('Capabilities loaded successfully', 'success');
      } catch (error) {
         const message = `Failed to load capabilities: ${getErrorMessage(error)}`;

         log(message, 'error');
         showError(message);
      }
   }

   async function startRecognition() {
      if (isRecognizing) {
         return;
      }

      isRecognizing = true;
      recognitionResult = null;
      setProcessingStatus('Listening for speech...');
      log(`Starting speech recognition (${recognitionLanguage})...`);

      try {
         const result = await speech.recognize({ language: recognitionLanguage });

         recognitionResult = result;
         log(
            `Recognized: "${result.text}" (confidence: ${(result.confidence * 100).toFixed(1)}%)`,
            'success',
         );
      } catch (error) {
         const message = `Speech recognition failed: ${getErrorMessage(error)}`;

         log(message, 'error');
         showError(message);
      } finally {
         isRecognizing = false;
         clearProcessingStatus();
      }
   }

   async function startStreamingRecognition() {
      if (streamingSessionId) {
         return;
      }

      setProcessingStatus('Starting streaming recognition...');
      log('Starting streaming recognition...');

      try {
         streamingSessionId = await speech.startRecognition({
            language: recognitionLanguage,
            continuous: true,
            interimResults: true,
         });
         setProcessingStatus('Listening (streaming)...');
         log(`Streaming session started: ${streamingSessionId}`, 'success');
      } catch (error) {
         const message = `Failed to start streaming: ${getErrorMessage(error)}`;

         log(message, 'error');
         showError(message);
         clearProcessingStatus();
      }
   }

   async function stopStreamingRecognition() {
      if (!streamingSessionId) {
         return;
      }

      setProcessingStatus('Stopping streaming...');
      log('Stopping streaming recognition...');

      try {
         const result = await speech.stopRecognition(streamingSessionId);

         recognitionResult = result;
         log(`Final result: "${result.text}"`, 'success');
      } catch (error) {
         const message = `Failed to stop streaming: ${getErrorMessage(error)}`;

         log(message, 'error');
         showError(message);
      } finally {
         streamingSessionId = null;
         clearProcessingStatus();
      }
   }

   async function loadVoices() {
      try {
         const result = await speech.getVoices();

         voices = Array.isArray(result) ? result : result.voices || [];
         log(`Loaded ${voices.length} voices`, 'success');
      } catch (error) {
         const message = `Failed to load voices: ${getErrorMessage(error)}`;

         log(message, 'error');
         showError(message);
      }
   }

   async function synthesizeSpeech() {
      if (!ttsText.trim() || isSpeaking) {
         return;
      }

      isSpeaking = true;
      setProcessingStatus('Speaking...');
      log('Synthesizing speech...');

      try {
         await speech.synthesize(ttsText, {
            voice: selectedVoice || undefined,
            rate: ttsRate,
            pitch: ttsPitch,
         });
         log('Speech synthesis completed', 'success');
      } catch (error) {
         const message = `Speech synthesis failed: ${getErrorMessage(error)}`;

         log(message, 'error');
         showError(message);
      } finally {
         isSpeaking = false;
         clearProcessingStatus();
      }
   }

   async function handleOcrImage(event) {
      const file = event.target.files[0];

      if (!file) {
         return;
      }

      isProcessingOcr = true;
      ocrResult = null;
      setProcessingStatus('Processing image for OCR...');
      log('Processing image for text recognition...');

      try {
         ocrImage = await fileToDataUrl(file);

         const base64 = await blobToBase64(file);
         const result = await vision.recognizeText({ base64 });

         ocrResult = result;
         log(`Found ${result.blocks?.length || 0} text blocks`, 'success');
      } catch (error) {
         const message = `OCR failed: ${getErrorMessage(error)}`;

         log(message, 'error');
         showError(message);
      } finally {
         isProcessingOcr = false;
         clearProcessingStatus();
      }
   }

   async function handleBarcodeImage(event) {
      const file = event.target.files[0];

      if (!file) {
         return;
      }

      isProcessingBarcode = true;
      barcodeResults = [];
      setProcessingStatus('Scanning for barcodes...');
      log('Scanning for barcodes...');

      try {
         barcodeImage = await fileToDataUrl(file);

         const base64 = await blobToBase64(file);
         const results = await vision.detectBarcodes({ base64 });

         barcodeResults = Array.isArray(results) ? results : results.barcodes || [];
         log(`Found ${barcodeResults.length} barcode(s)`, 'success');
      } catch (error) {
         const message = `Barcode detection failed: ${getErrorMessage(error)}`;

         log(message, 'error');
         showError(message);
      } finally {
         isProcessingBarcode = false;
         clearProcessingStatus();
      }
   }

   async function handleFaceImage(event) {
      const file = event.target.files[0];

      if (!file) {
         return;
      }

      isProcessingFaces = true;
      faceResults = [];
      setProcessingStatus('Detecting faces...');
      log('Detecting faces...');

      try {
         faceImage = await fileToDataUrl(file);

         const base64 = await blobToBase64(file);
         const results = await vision.detectFaces(
            { base64 },
            { detectLandmarks, classifyAttributes },
         );

         faceResults = Array.isArray(results) ? results : results.faces || [];
         log(`Detected ${faceResults.length} face(s)`, 'success');
      } catch (error) {
         const message = `Face detection failed: ${getErrorMessage(error)}`;

         log(message, 'error');
         showError(message);
      } finally {
         isProcessingFaces = false;
         clearProcessingStatus();
      }
   }

   async function handleClassificationImage(event) {
      const file = event.target.files[0];

      if (!file) {
         return;
      }

      isProcessingClassification = true;
      classificationResults = [];
      setProcessingStatus('Classifying image...');
      log('Classifying image...');

      try {
         classificationImage = await fileToDataUrl(file);

         const base64 = await blobToBase64(file);
         const results = await vision.classifyImage(
            { base64 },
            { maxResults: maxClassifications, minConfidence },
         );

         classificationResults = Array.isArray(results)
            ? results
            : results.classifications || [];
         log(`Found ${classificationResults.length} classification(s)`, 'success');
      } catch (error) {
         const message = `Image classification failed: ${getErrorMessage(error)}`;

         log(message, 'error');
         showError(message);
      } finally {
         isProcessingClassification = false;
         clearProcessingStatus();
      }
   }

   async function identifyLanguage() {
      if (!langIdText.trim()) {
         return;
      }

      isIdentifyingLang = true;
      langIdResult = null;
      setProcessingStatus('Identifying language...');
      log('Identifying language...');

      try {
         const result = await text.identifyLanguage(langIdText);

         langIdResult = result;
         log(
            `Detected: ${result.language} (${(result.confidence * 100).toFixed(1)}% confidence)`,
            'success',
         );
      } catch (error) {
         const message = `Language identification failed: ${getErrorMessage(error)}`;

         log(message, 'error');
         showError(message);
      } finally {
         isIdentifyingLang = false;
         clearProcessingStatus();
      }
   }

   async function handleTranslate() {
      if (!translateText.trim()) {
         return;
      }

      isTranslating = true;
      translateResult = null;
      setProcessingStatus(`Translating from ${translateFrom} to ${translateTo}...`);
      log(`Translating from ${translateFrom} to ${translateTo}...`);

      try {
         const result = await text.translate(translateText, translateFrom, translateTo);

         translateResult = result;
         log(`Translation: ${result.translatedText}`, 'success');
      } catch (error) {
         if (isModelNotInstalled(error)) {
            const message = `Translation model for ${translateFrom} → ${translateTo} is not installed on this device. Please download it in your system settings.`;
            log(message, 'warning');
            showError(message);
         } else if (isLanguageNotSupported(error)) {
            const message = `Translation pair ${translateFrom} → ${translateTo} is not supported on this device.`;
            log(message, 'warning');
            showError(message);
         } else {
            const message = `Translation failed: ${getErrorMessage(error)}`;
            log(message, 'error');
            showError(message);
         }
      } finally {
         isTranslating = false;
         clearProcessingStatus();
      }
   }

   async function checkLlmAvailability() {
      try {
         setProcessingStatus('Checking LLM availability...');
         llmAvailability = await llm.checkAvailability();
         log(
            `LLM available: ${llmAvailability.available}${llmAvailability.reason ? ` (${llmAvailability.reason})` : ''}`,
            llmAvailability.available ? 'success' : 'info',
         );

         if (llmAvailability.available) {
            llmModelInfo = await llm.getModelInfo();
            log(`Model: ${llmModelInfo.name} (${llmModelInfo.provider})`, 'success');
         } else {
            llmModelInfo = null;
         }
      } catch (error) {
         showError(getErrorMessage(error));
         log(`LLM check failed: ${getErrorMessage(error)}`, 'error');
      } finally {
         clearProcessingStatus();
      }
   }

   async function generateText() {
      if (isGenerating || !llmPrompt.trim()) {
         return;
      }

      isGenerating = true;
      llmResult = null;

      try {
         setProcessingStatus('Generating text...');

         const options = {
            prompt: llmPrompt,
            ...(llmSystemPrompt.trim() ? { systemPrompt: llmSystemPrompt } : {}),
            temperature: llmTemperature,
            maxTokens: llmMaxTokens,
         };

         llmResult = await llm.generate(options);
         log(`Generated ${llmResult.content.length} chars (${llmResult.finishReason})`, 'success');
      } catch (error) {
         showError(getErrorMessage(error));
         log(`Generation failed: ${getErrorMessage(error)}`, 'error');
      } finally {
         isGenerating = false;
         clearProcessingStatus();
      }
   }

   async function streamText() {
      if (isStreaming || !llmPrompt.trim()) {
         return;
      }

      isStreaming = true;
      llmStreamContent = '';
      llmResult = null;

      try {
         setProcessingStatus('Streaming response...');

         const options = {
            prompt: llmPrompt,
            ...(llmSystemPrompt.trim() ? { systemPrompt: llmSystemPrompt } : {}),
            temperature: llmTemperature,
            maxTokens: llmMaxTokens,
         };

         await llm.generateStream(options, (event) => {
            if (event.type === 'delta') {
               llmStreamContent += event.content;
               return;
            }

            if (event.type === 'done') {
               llmStreamContent = event.content;
               llmResult = {
                  content: event.content,
                  model: '',
                  finishReason: event.finishReason,
                  usage: event.usage,
               };
               log(
                  `Streamed ${event.content.length} chars (${event.finishReason})`,
                  'success',
               );
               return;
            }

            if (event.type === 'error') {
               showError(event.message);
               log(`Stream error: ${event.message}`, 'error');
            }
         });
      } catch (error) {
         showError(getErrorMessage(error));
         log(`Stream failed: ${getErrorMessage(error)}`, 'error');
      } finally {
         isStreaming = false;
         clearProcessingStatus();
      }
   }

   async function createLlmSession() {
      try {
         setProcessingStatus('Creating session...');

         const options = {
            ...(llmSystemPrompt.trim() ? { systemPrompt: llmSystemPrompt } : {}),
            temperature: llmTemperature,
            maxTokens: llmMaxTokens,
         };

         llmSessionId = await llm.createSession(options);
         llmChatHistory = [];
         log(`Session created: ${llmSessionId.substring(0, 8)}...`, 'success');
      } catch (error) {
         showError(getErrorMessage(error));
         log(`Session creation failed: ${getErrorMessage(error)}`, 'error');
      } finally {
         clearProcessingStatus();
      }
   }

   async function sendChatMessage() {
      if (isSendingChat || !llmChatInput.trim() || !llmSessionId) {
         return;
      }

      isSendingChat = true;

      const message = llmChatInput;
      llmChatInput = '';
      llmChatHistory = [...llmChatHistory, { role: 'user', content: message }];

      try {
         setProcessingStatus('Thinking...');

         const result = await llm.sessionSend(llmSessionId, message);

         llmChatHistory = [
            ...llmChatHistory,
            { role: 'assistant', content: result.content },
         ];
         log(`Session response: ${result.content.length} chars`, 'success');
      } catch (error) {
         showError(getErrorMessage(error));
         llmChatHistory = [
            ...llmChatHistory,
            { role: 'error', content: getErrorMessage(error) },
         ];
      } finally {
         isSendingChat = false;
         clearProcessingStatus();
      }
   }

   async function endLlmSession() {
      if (!llmSessionId) {
         return;
      }

      try {
         await llm.destroySession(llmSessionId);
         log(`Session destroyed: ${llmSessionId.substring(0, 8)}...`);
      } catch (error) {
         log(`Session destroy failed: ${getErrorMessage(error)}`, 'error');
      }

      llmSessionId = null;
      llmChatHistory = [];
   }

   async function summarizeTextHandler() {
      if (isSummarizing || !summarizeText.trim()) {
         return;
      }

      isSummarizing = true;
      summarizeResult = null;

      try {
         setProcessingStatus('Summarizing...');
         summarizeResult = await llm.summarize({ text: summarizeText });
         log(`Summarized to ${summarizeResult.summary.length} chars`, 'success');
      } catch (error) {
         showError(getErrorMessage(error));
         log(`Summarize failed: ${getErrorMessage(error)}`, 'error');
      } finally {
         isSummarizing = false;
         clearProcessingStatus();
      }
   }

   async function rewriteTextHandler() {
      if (isRewriting || !rewriteText.trim()) {
         return;
      }

      isRewriting = true;
      rewriteResult = null;

      try {
         setProcessingStatus('Rewriting...');
         rewriteResult = await llm.rewrite({ text: rewriteText, tone: rewriteTone });
         log(`Rewritten in ${rewriteTone} tone`, 'success');
      } catch (error) {
         showError(getErrorMessage(error));
         log(`Rewrite failed: ${getErrorMessage(error)}`, 'error');
      } finally {
         isRewriting = false;
         clearProcessingStatus();
      }
   }

   async function runTests() {
      if (isRunningTests) {
         return;
      }

      isRunningTests = true;
      testResults = [];
      testProgress = { current: 0, total: 0 };
      setProcessingStatus('Running end-to-end tests...');
      log('Starting end-to-end tests...');

      try {
         testResults = await runDemoTestSuite({
            fetchImpl: fetch,
            visionApi: vision,
            speechApi: speech,
            onProgress: (progress) => {
               testProgress = progress;
            },
         });
         log('End-to-end tests completed', 'success');
         showToast('Tests completed!', 'success');
      } catch (error) {
         const message = `Test runner failed: ${getErrorMessage(error)}`;

         log(message, 'error');
         showError(message);
      } finally {
         isRunningTests = false;
         clearProcessingStatus();
      }
   }

   $effect(() => {
      loadCapabilities();
      loadVoices();
   });
</script>

<div class='app'>
   <SidebarNav
      activeTab={activeTab}
      items={listSidebarItems()}
      onSelect={(tabID) => {
         activeTab = tabID;
      }}
      {platformInfo}
   />

   <main class='main'>
      <TopBar processingStatus={processingStatus} onRefresh={loadCapabilities} />

      <div class='scroll-content'>
         {#if activeTab === 'capabilities'}
            <CapabilitiesTab {platformInfo} {capabilities} />
         {:else if activeTab === 'speech'}
            <SpeechTab
               bind:recognitionLanguage
               bind:ttsText
               bind:selectedVoice
               bind:ttsRate
               bind:ttsPitch
               {voices}
               {isRecognizing}
               {recognitionResult}
               {streamingSessionId}
               {isSpeaking}
               startRecognition={startRecognition}
               startStreamingRecognition={startStreamingRecognition}
               stopStreamingRecognition={stopStreamingRecognition}
               synthesizeSpeech={synthesizeSpeech}
               loadVoices={loadVoices}
            />
         {:else if activeTab === 'vision'}
            <VisionTab
               bind:detectLandmarks
               bind:classifyAttributes
               bind:maxClassifications
               {ocrResult}
               {ocrImage}
               {isProcessingOcr}
               {barcodeResults}
               {barcodeImage}
               {isProcessingBarcode}
               {faceResults}
               {faceImage}
               {isProcessingFaces}
               {classificationResults}
               {classificationImage}
               {isProcessingClassification}
               handleOcrImage={handleOcrImage}
               handleBarcodeImage={handleBarcodeImage}
               handleFaceImage={handleFaceImage}
               handleClassificationImage={handleClassificationImage}
            />
         {:else if activeTab === 'text'}
            <TextTab
               bind:langIdText
               {langIdResult}
               {isIdentifyingLang}
               identifyLanguage={identifyLanguage}
               bind:translateText
               bind:translateFrom
               bind:translateTo
               {translateResult}
               {isTranslating}
               {handleTranslate}
            />
         {:else if activeTab === 'llm'}
            <LlmTab
               bind:llmPrompt
               bind:llmSystemPrompt
               bind:llmTemperature
               bind:llmMaxTokens
               bind:llmChatInput
               bind:summarizeText
               bind:rewriteText
               bind:rewriteTone
               {llmAvailability}
               {llmModelInfo}
               {llmResult}
               {llmStreamContent}
               {isGenerating}
               {isStreaming}
               {llmSessionId}
               {llmChatHistory}
               {isSendingChat}
               {summarizeResult}
               {isSummarizing}
               {rewriteResult}
               {isRewriting}
               checkLlmAvailability={checkLlmAvailability}
               generateText={generateText}
               streamText={streamText}
               createLlmSession={createLlmSession}
               sendChatMessage={sendChatMessage}
               endLlmSession={endLlmSession}
               summarizeTextHandler={summarizeTextHandler}
               rewriteTextHandler={rewriteTextHandler}
            />
         {:else if activeTab === 'logs'}
            <LogsTab {logs} clearLogs={clearLogs} />
         {:else if activeTab === 'tests'}
            <TestsTab
               {testResults}
               {isRunningTests}
               {testProgress}
               runTests={runTests}
            />
         {/if}
      </div>
   </main>

   <ToastStack toasts={toasts} onDismiss={dismissToast} />
</div>

<style lang='scss'>
   :global(:root) {
      --bg: oklch(14% 0.02 260);
      --bg-elevated: oklch(16% 0.02 260);
      --surface: oklch(18% 0.02 260);
      --surface-higher: oklch(22% 0.03 260);
      --border: oklch(28% 0.04 260);
      --text: oklch(92% 0.01 260);
      --text-muted: oklch(70% 0.02 260);
      --accent: oklch(65% 0.2 260);
      --accent-low: oklch(65% 0.2 260 / 15%);
      --error: oklch(60% 0.18 25);
      --success: oklch(70% 0.15 150);
      --warning: oklch(80% 0.16 90);
      --radius-lg: 18px;
      --radius-md: 12px;
      --radius-sm: 8px;
      --shadow-lg: 0 24px 80px oklch(0% 0 0 / 28%);
      --shadow-md: 0 12px 36px oklch(0% 0 0 / 16%);
      --font: Inter, system-ui, sans-serif;
      --font-mono: 'JetBrains Mono', 'Fira Code', monospace;
   }

   :global(*) {
      box-sizing: border-box;
   }

   :global(body) {
      min-height: 100vh;
      margin: 0;
      background:
         radial-gradient(circle at top, oklch(22% 0.08 260 / 50%), transparent 32%),
         linear-gradient(180deg, var(--bg-elevated), var(--bg));
      color: var(--text);
      font-family: var(--font);
   }

   :global(button),
   :global(input),
   :global(select),
   :global(textarea) {
      font: inherit;
   }

   .app {
      display: grid;
      grid-template-columns: minmax(15rem, 17.5rem) minmax(0, 1fr);
      min-height: 100vh;
   }

   .main {
      display: flex;
      min-width: 0;
      min-height: 100vh;
      flex-direction: column;
   }

   .scroll-content {
      flex: 1;
      overflow-y: auto;
      padding: 2rem;
   }

   @media (max-width: 980px) {
      .app {
         grid-template-columns: 1fr;
      }

      .main {
         min-height: 0;
      }

      .scroll-content {
         padding: 1rem;
      }
   }
</style>
