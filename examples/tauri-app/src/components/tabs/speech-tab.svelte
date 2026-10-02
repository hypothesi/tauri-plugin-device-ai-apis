<script>
   import Icon from '@iconify/svelte';

   import { recognitionLanguages } from '../../demo-config.js';

   let {
      recognitionLanguage = $bindable(),
      ttsText = $bindable(),
      selectedVoice = $bindable(),
      ttsRate = $bindable(),
      ttsPitch = $bindable(),
      voices,
      isRecognizing,
      recognitionResult,
      streamingSessionId,
      isSpeaking,
      startRecognition,
      startStreamingRecognition,
      stopStreamingRecognition,
      synthesizeSpeech,
      loadVoices,
   } = $props();
</script>

<div class='tab-grid'>
   <section class='panel'>
      <div class='panel-header'>
         <Icon icon='lucide:mic' />
         <div>
            <h2>Speech recognition</h2>
            <p>Capture speech as a single result or keep a live streaming session open.</p>
         </div>
      </div>

      <div class='field'>
         <label for='recognition-language'>Recognition language</label>
         <select id='recognition-language' bind:value={recognitionLanguage}>
            {#each recognitionLanguages as language}
               <option value={language.value}>{language.label}</option>
            {/each}
         </select>
      </div>

      <div class='actions-row'>
         <button type='button' class='primary' disabled={isRecognizing} onclick={startRecognition}>
            <span class:spin={isRecognizing}>
               <Icon icon={isRecognizing ? 'lucide:loader-2' : 'lucide:play'} />
            </span>
            {isRecognizing ? 'Listening...' : 'One-shot recognition'}
         </button>

         {#if !streamingSessionId}
            <button type='button' onclick={startStreamingRecognition}>
               <Icon icon='lucide:radio' />
               Start streaming
            </button>
         {:else}
            <button type='button' class='danger' onclick={stopStreamingRecognition}>
               <Icon icon='lucide:square' />
               Stop streaming
            </button>
         {/if}
      </div>

      {#if recognitionResult}
         <div class='result-card'>
            <div class='result-header'>
               <h3>Transcript</h3>
               <span>{recognitionResult.isFinal ? 'Final result' : 'Interim result'}</span>
            </div>

            <p class='transcript'>"{recognitionResult.text}"</p>

            <div class='result-meta'>
               <span>Confidence: {(recognitionResult.confidence * 100).toFixed(1)}%</span>
               {#if recognitionResult.alternatives?.length > 1}
                  <span>{recognitionResult.alternatives.length} alternatives returned</span>
               {/if}
            </div>
         </div>
      {/if}
   </section>

   <section class='panel'>
      <div class='panel-header'>
         <Icon icon='lucide:volume-2' />
         <div>
            <h2>Text to speech</h2>
            <p>Preview the available voice set and synthesize speech directly on the device.</p>
         </div>
      </div>

      <div class='field'>
         <label for='tts-text'>Text to speak</label>
         <textarea id='tts-text' bind:value={ttsText} rows='4'></textarea>
      </div>

      <div class='field'>
         <label for='tts-voice'>Voice</label>
         <select id='tts-voice' bind:value={selectedVoice}>
            <option value=''>Default voice</option>

            {#each voices as voice}
               <option value={voice.id}>
                  {voice.name} ({voice.language}) {voice.isDefault ? '★' : ''}
               </option>
            {/each}
         </select>
      </div>

      <div class='slider-grid'>
         <div class='field'>
            <label for='tts-rate'>Rate: {ttsRate.toFixed(1)}x</label>
            <input id='tts-rate' type='range' bind:value={ttsRate} min='0.5' max='2' step='0.1' />
         </div>

         <div class='field'>
            <label for='tts-pitch'>Pitch: {ttsPitch.toFixed(1)}x</label>
            <input id='tts-pitch' type='range' bind:value={ttsPitch} min='0.5' max='2' step='0.1' />
         </div>
      </div>

      <div class='actions-row'>
         <button
            type='button'
            class='primary'
            disabled={isSpeaking || !ttsText.trim()}
            onclick={synthesizeSpeech}
         >
            <span class:spin={isSpeaking}>
               <Icon icon={isSpeaking ? 'lucide:loader-2' : 'lucide:play'} />
            </span>
            {isSpeaking ? 'Speaking...' : 'Speak'}
         </button>

         <button type='button' onclick={loadVoices}>
            <Icon icon='lucide:refresh-ccw' />
            Refresh voices
         </button>
      </div>

      {#if voices.length > 0}
         <p class='hint'>{voices.length} voice(s) available in the current environment.</p>
      {/if}
   </section>
</div>

<style lang='scss'>
   .tab-grid {
      display: grid;
      gap: 1rem;
      grid-template-columns: repeat(auto-fit, minmax(20rem, 1fr));
   }

   .panel {
      display: flex;
      flex-direction: column;
      gap: 1rem;
      border: 1px solid oklch(100% 0 0 / 8%);
      border-radius: var(--radius-lg);
      background: oklch(100% 0 0 / 5%);
      box-shadow: var(--shadow-md);
      padding: 1.3rem;
   }

   .panel-header {
      display: flex;
      gap: 0.9rem;
      align-items: flex-start;

      :global(svg) {
         color: var(--accent);
         font-size: 1.25rem;
         flex-shrink: 0;
         margin-top: 0.15rem;
      }

      h2,
      p {
         margin: 0;
      }

      p {
         color: var(--text-muted);
         margin-top: 0.3rem;
      }
   }

   .field {
      display: flex;
      flex-direction: column;
      gap: 0.5rem;

      label {
         color: var(--text-muted);
         font-size: 0.86rem;
      }
   }

   select,
   textarea,
   input[type='range'] {
      width: 100%;
   }

   select,
   textarea,
   input {
      border: 1px solid oklch(100% 0 0 / 8%);
      border-radius: var(--radius-md);
      background: oklch(9% 0.01 260 / 72%);
      color: var(--text);
      padding: 0.85rem 0.95rem;
   }

   textarea {
      min-height: 8rem;
      resize: vertical;
   }

   .slider-grid {
      display: grid;
      gap: 1rem;
      grid-template-columns: repeat(auto-fit, minmax(12rem, 1fr));
   }

   .actions-row {
      display: flex;
      flex-wrap: wrap;
      gap: 0.75rem;
   }

   button {
      display: inline-flex;
      align-items: center;
      justify-content: center;
      gap: 0.5rem;
      border: 1px solid oklch(100% 0 0 / 8%);
      border-radius: var(--radius-md);
      background: oklch(100% 0 0 / 5%);
      color: var(--text);
      cursor: pointer;
      padding: 0.8rem 1rem;
      transition: border-color 0.2s ease, transform 0.2s ease;

      &:hover:not(:disabled) {
         border-color: oklch(100% 0 0 / 18%);
         transform: translateY(-0.05rem);
      }

      &:disabled {
         cursor: not-allowed;
         opacity: 0.6;
      }

      &.primary {
         background: linear-gradient(135deg, var(--accent), oklch(72% 0.2 220));
         border-color: transparent;
         color: white;
      }

      &.danger {
         background: linear-gradient(135deg, var(--error), oklch(70% 0.18 10));
         border-color: transparent;
         color: white;
      }
   }

   .result-card {
      border: 1px solid oklch(100% 0 0 / 8%);
      border-radius: var(--radius-md);
      background: oklch(9% 0.01 260 / 72%);
      padding: 1rem;
   }

   .result-header,
   .result-meta {
      display: flex;
      flex-wrap: wrap;
      align-items: center;
      justify-content: space-between;
      gap: 0.75rem;
   }

   .result-header {
      margin-bottom: 0.8rem;

      h3,
      span {
         margin: 0;
      }

      span {
         color: var(--text-muted);
         font-size: 0.82rem;
      }
   }

   .transcript {
      margin: 0;
      font-size: 1.05rem;
      line-height: 1.6;
   }

   .result-meta {
      color: var(--text-muted);
      font-size: 0.84rem;
      margin-top: 0.8rem;
   }

   .hint {
      margin: 0;
      color: var(--text-muted);
      font-size: 0.82rem;
   }

   .spin {
      animation: spin 1s linear infinite;
   }

   @keyframes spin {
      from {
         transform: rotate(0deg);
      }

      to {
         transform: rotate(360deg);
      }
   }
</style>
