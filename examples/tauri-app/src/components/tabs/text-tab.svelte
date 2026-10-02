<script>
   import Icon from '@iconify/svelte';

   import { sampleLanguageTexts } from '../../demo-config.js';

   let {
      langIdText = $bindable(),
      langIdResult,
      isIdentifyingLang,
      identifyLanguage,
      translateText = $bindable(''),
      translateFrom = $bindable('en'),
      translateTo = $bindable('es'),
      translateResult,
      isTranslating,
      handleTranslate,
   } = $props();

   const languages = [
      { code: 'en', label: 'English' },
      { code: 'es', label: 'Spanish' },
      { code: 'fr', label: 'French' },
      { code: 'de', label: 'German' },
      { code: 'it', label: 'Italian' },
      { code: 'ja', label: 'Japanese' },
      { code: 'ko', label: 'Korean' },
      { code: 'pt', label: 'Portuguese' },
      { code: 'zh', label: 'Chinese' },
   ];
</script>

<div class='tab-grid'>
   <section class='panel'>
      <div class='panel-header'>
         <Icon icon='lucide:languages' />
         <div>
            <h2>Language identification</h2>
            <p>Analyze a text passage and return the most likely language match.</p>
         </div>
      </div>

      <div class='field'>
         <label for='lang-id-input'>Text to analyze</label>
         <textarea id='lang-id-input' bind:value={langIdText} rows='5'></textarea>
      </div>

      <div class='sample-list'>
         {#each sampleLanguageTexts as sample}
            <button
               type='button'
               class='sample-btn'
               onclick={() => {
                  langIdText = sample.value;
               }}
            >
               {sample.label}
            </button>
         {/each}
      </div>

      <button
         type='button'
         class='primary'
         disabled={isIdentifyingLang || !langIdText.trim()}
         onclick={identifyLanguage}
      >
         <span class:spin={isIdentifyingLang}>
            <Icon icon={isIdentifyingLang ? 'lucide:loader-2' : 'lucide:search'} />
         </span>
         {isIdentifyingLang ? 'Analyzing...' : 'Identify language'}
      </button>

      {#if langIdResult}
         <div class='result-card'>
            <div class='result-pill'>
               <span class='code'>{langIdResult.language}</span>
               <span>{(langIdResult.confidence * 100).toFixed(1)}% confidence</span>
            </div>
         </div>
      {/if}
   </section>

   <section class='panel'>
      <div class='panel-header'>
         <Icon icon='lucide:repeat' />
         <div>
            <h2>Translation</h2>
            <p>Translate text between languages using on-device models.</p>
         </div>
      </div>

      <div class='field'>
         <label for='translate-input'>Text to translate</label>
         <textarea
            id='translate-input'
            bind:value={translateText}
            rows='4'
            placeholder='Enter text to translate...'
         ></textarea>
      </div>

      <div class='lang-selectors'>
         <div class='field select-field'>
            <label for='from-lang'>From</label>
            <select id='from-lang' bind:value={translateFrom}>
               {#each languages as lang}
                  <option value={lang.code}>{lang.label} ({lang.code})</option>
               {/each}
            </select>
         </div>

         <button
            type='button'
            class='icon-btn swap-btn'
            title='Swap languages'
            onclick={() => {
               const tmp = translateFrom;
               translateFrom = translateTo;
               translateTo = tmp;
            }}
         >
            <Icon icon='lucide:arrow-left-right' />
         </button>

         <div class='field select-field'>
            <label for='to-lang'>To</label>
            <select id='to-lang' bind:value={translateTo}>
               {#each languages as lang}
                  <option value={lang.code}>{lang.label} ({lang.code})</option>
               {/each}
            </select>
         </div>
      </div>

      <button
         type='button'
         class='primary'
         disabled={isTranslating || !translateText.trim()}
         onclick={handleTranslate}
      >
         <span class:spin={isTranslating}>
            <Icon icon={isTranslating ? 'lucide:loader-2' : 'lucide:repeat'} />
         </span>
         {isTranslating ? 'Translating...' : 'Translate'}
      </button>

      {#if translateResult}
         <div class='result-card'>
            <div class='result-pill'>
               <span class='code'>{translateResult.sourceLanguage} &rarr; {translateResult.targetLanguage}</span>
            </div>
            <p class='translated-text'>{translateResult.translatedText}</p>
         </div>
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
         flex-shrink: 0;
         font-size: 1.2rem;
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

   textarea {
      width: 100%;
      min-height: 10rem;
      border: 1px solid oklch(100% 0 0 / 8%);
      border-radius: var(--radius-md);
      background: oklch(9% 0.01 260 / 72%);
      color: var(--text);
      padding: 0.85rem 0.95rem;
      resize: vertical;
   }

   .lang-selectors {
      display: flex;
      align-items: flex-end;
      gap: 0.75rem;

      .select-field {
         flex: 1;

         select {
            width: 100%;
            border: 1px solid oklch(100% 0 0 / 8%);
            border-radius: var(--radius-md);
            background: oklch(9% 0.01 260 / 72%);
            color: var(--text);
            padding: 0.65rem 0.85rem;
         }
      }

      .swap-btn {
         display: inline-flex;
         align-items: center;
         justify-content: center;
         height: 2.6rem;
         width: 2.6rem;
         border: 1px solid oklch(100% 0 0 / 8%);
         border-radius: var(--radius-md);
         background: oklch(100% 0 0 / 5%);
         color: var(--text);
         cursor: pointer;
         transition: background 0.15s ease;

         &:hover {
            background: oklch(100% 0 0 / 10%);
         }
      }
   }

   .sample-list {
      display: flex;
      flex-wrap: wrap;
      gap: 0.65rem;
   }

   .sample-btn,
   .primary {
      border: 1px solid oklch(100% 0 0 / 8%);
      border-radius: 999px;
      cursor: pointer;
      padding: 0.7rem 0.95rem;
      transition: transform 0.2s ease, border-color 0.2s ease;
   }

   .sample-btn {
      background: oklch(100% 0 0 / 5%);
      color: var(--text);

      &:hover {
         border-color: oklch(100% 0 0 / 18%);
         transform: translateY(-0.05rem);
      }
   }

   .primary {
      display: inline-flex;
      width: fit-content;
      align-items: center;
      gap: 0.5rem;
      background: linear-gradient(135deg, var(--accent), oklch(72% 0.2 220));
      color: white;

      &:disabled {
         cursor: not-allowed;
         opacity: 0.6;
      }
   }

   .result-card {
      border: 1px solid oklch(100% 0 0 / 8%);
      border-radius: var(--radius-md);
      background: oklch(9% 0.01 260 / 72%);
      padding: 1rem;
   }

   .result-pill {
      display: inline-flex;
      align-items: center;
      gap: 0.75rem;
      border-radius: 999px;
      background: oklch(70% 0.15 150 / 14%);
      color: var(--success);
      padding: 0.55rem 0.85rem;
   }

   .code {
      font-weight: 700;
      letter-spacing: 0.08em;
      text-transform: uppercase;
   }

   .translated-text {
      margin-top: 0.75rem;
      margin-bottom: 0;
      font-size: 1rem;
      line-height: 1.5;
      color: var(--text);
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
