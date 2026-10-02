<script>
   import Icon from '@iconify/svelte';

   import { llmCapabilityLabels, rewriteTones } from '../../demo-config.js';

   let {
      llmPrompt = $bindable(),
      llmSystemPrompt = $bindable(),
      llmTemperature = $bindable(),
      llmMaxTokens = $bindable(),
      llmChatInput = $bindable(),
      summarizeText = $bindable(),
      rewriteText = $bindable(),
      rewriteTone = $bindable(),
      llmAvailability,
      llmModelInfo,
      llmResult,
      llmStreamContent,
      isGenerating,
      isStreaming,
      llmSessionId,
      llmChatHistory,
      isSendingChat,
      summarizeResult,
      isSummarizing,
      rewriteResult,
      isRewriting,
      checkLlmAvailability,
      generateText,
      streamText,
      createLlmSession,
      sendChatMessage,
      endLlmSession,
      summarizeTextHandler,
      rewriteTextHandler,
   } = $props();

   function listSupportedCapabilities(capabilities) {
      return llmCapabilityLabels.filter((item) => {
         return capabilities?.[item.key];
      });
   }
</script>

<div class='tab-grid'>
   <section class='panel'>
      <div class='panel-header'>
         <Icon icon='lucide:brain-circuit' />
         <div>
            <h2>Model availability</h2>
            <p>Check whether an on-device model is ready and inspect its supported features.</p>
         </div>
      </div>

      <div class='actions-row'>
         <button type='button' class='primary' onclick={checkLlmAvailability}>
            <Icon icon='lucide:search' />
            Check capabilities
         </button>
      </div>

      {#if llmAvailability}
         <div class='availability-grid'>
            <article class='availability-card'>
               <span class='label'>Status</span>
               <strong class:ok={llmAvailability.available}>
                  {llmAvailability.available ? 'Ready' : 'Not available'}
               </strong>
            </article>

            {#if llmAvailability.reason}
               <article class='availability-card wide'>
                  <span class='label'>Reason</span>
                  <p class='reason'>{llmAvailability.reason}</p>
               </article>
            {/if}

            {#if llmModelInfo}
               <article class='availability-card'>
                  <span class='label'>Model</span>
                  <strong>{llmModelInfo.name}</strong>
               </article>

               <article class='availability-card'>
                  <span class='label'>Provider</span>
                  <strong>{llmModelInfo.provider}</strong>
               </article>

               <article class='availability-card'>
                  <span class='label'>Context window</span>
                  <strong>{llmModelInfo.contextWindow} tokens</strong>
               </article>

               <article class='availability-card'>
                  <span class='label'>Execution</span>
                  <strong>{llmModelInfo.onDevice ? 'On-device' : 'Hosted'}</strong>
               </article>

               <article class='availability-card wide'>
                  <span class='label'>Supported capabilities</span>
                  <div class='chip-list'>
                     {#each listSupportedCapabilities(llmModelInfo.capabilities) as capability}
                        <span>{capability.label}</span>
                     {/each}
                  </div>
               </article>
            {/if}
         </div>
      {/if}
   </section>

   <section class='panel'>
      <div class='panel-header'>
         <Icon icon='lucide:message-square-plus' />
         <div>
            <h2>Text generation</h2>
            <p>Generate a complete response or stream tokens as they arrive.</p>
         </div>
      </div>

      <div class='field'>
         <label for='llm-prompt'>Prompt</label>
         <textarea id='llm-prompt' bind:value={llmPrompt} rows='4'></textarea>
      </div>

      <div class='field'>
         <label for='llm-system-prompt'>System prompt</label>
         <input
            id='llm-system-prompt'
            type='text'
            bind:value={llmSystemPrompt}
            placeholder='You are a helpful coding assistant.'
         />
      </div>

      <div class='control-grid'>
         <div class='field'>
            <label for='llm-temperature'>Temperature: {llmTemperature.toFixed(1)}</label>
            <input
               id='llm-temperature'
               type='range'
               bind:value={llmTemperature}
               min='0'
               max='2'
               step='0.1'
            />
         </div>

         <div class='field'>
            <label for='llm-max-tokens'>Max tokens: {llmMaxTokens}</label>
            <input
               id='llm-max-tokens'
               type='range'
               bind:value={llmMaxTokens}
               min='64'
               max='4096'
               step='64'
            />
         </div>
      </div>

      <div class='actions-row'>
         <button
            type='button'
            class='primary'
            disabled={isGenerating || isStreaming || !llmPrompt.trim()}
            onclick={generateText}
         >
            <span class:spin={isGenerating}>
               <Icon icon={isGenerating ? 'lucide:loader-2' : 'lucide:sparkles'} />
            </span>
            {isGenerating ? 'Generating...' : 'Generate'}
         </button>

         <button
            type='button'
            disabled={isGenerating || isStreaming || !llmPrompt.trim()}
            onclick={streamText}
         >
            <span class:spin={isStreaming}>
               <Icon icon={isStreaming ? 'lucide:loader-2' : 'lucide:waves'} />
            </span>
            {isStreaming ? 'Streaming...' : 'Stream'}
         </button>
      </div>

      {#if isStreaming && llmStreamContent}
         <div class='result-card'>
            <div class='result-header'>
               <h3>Streaming response</h3>
               <span>Live tokens</span>
            </div>
            <pre>{llmStreamContent}<span class='cursor'>▊</span></pre>
         </div>
      {/if}

      {#if llmResult && !isStreaming}
         <div class='result-card'>
            <div class='result-header'>
               <h3>Generated output</h3>
               <span>{llmResult.finishReason}</span>
            </div>
            <pre>{llmResult.content}</pre>

            <div class='result-meta'>
               {#if llmResult.usage?.totalTokens != null}
                  <span>Total tokens: {llmResult.usage.totalTokens}</span>
               {/if}
            </div>
         </div>
      {/if}
   </section>

   <section class='panel'>
      <div class='panel-header'>
         <Icon icon='lucide:messages-square' />
         <div>
            <h2>Chat session</h2>
            <p>Keep a persistent session open for multi-turn, contextual conversations.</p>
         </div>
      </div>

      {#if !llmSessionId}
         <div class='empty-state'>
            <Icon icon='lucide:message-circle-more' />
            <p>Start a session to reuse the system prompt and generation settings above.</p>
            <button type='button' class='primary' onclick={createLlmSession}>Start session</button>
         </div>
      {:else}
         <div class='chat-box'>
            <div class='messages'>
               {#each llmChatHistory as message}
                  <article class='message' class:user={message.role === 'user'} class:error={message.role === 'error'}>
                     <div class='avatar'>
                        <Icon
                           icon={
                              message.role === 'user'
                                 ? 'lucide:user'
                                 : message.role === 'error'
                                    ? 'lucide:triangle-alert'
                                    : 'lucide:bot'
                           }
                        />
                     </div>
                     <div class='bubble'>{message.content}</div>
                  </article>
               {/each}

               {#if isSendingChat}
                  <article class='message'>
                     <div class='avatar'>
               <Icon icon='lucide:loader-2' class='spin' />
                     </div>
                     <div class='bubble'>Thinking...</div>
                  </article>
               {/if}
            </div>

            <div class='chat-controls'>
               <input
                  type='text'
                  bind:value={llmChatInput}
                  placeholder='Type a message...'
                  onkeydown={(event) => {
                     if (event.key === 'Enter' && !event.shiftKey) {
                        sendChatMessage();
                     }
                  }}
               />

               <button
                  type='button'
                  class='primary'
                  disabled={isSendingChat || !llmChatInput.trim()}
                  onclick={sendChatMessage}
               >
                  <Icon icon='lucide:send' />
               </button>

               <button type='button' class='danger' onclick={endLlmSession}>
                  End
               </button>
            </div>
         </div>
      {/if}
   </section>

   <section class='panel'>
      <div class='panel-header'>
         <Icon icon='lucide:sparkles' />
         <div>
            <h2>Text intelligence</h2>
            <p>Use the device model for summarization and tone-aware rewriting.</p>
         </div>
      </div>

      <div class='sub-panel-grid'>
         <section class='sub-panel'>
            <h3>Summarize</h3>
            <div class='field'>
               <textarea bind:value={summarizeText} rows='6'></textarea>
            </div>
            <button
               type='button'
               class='primary'
               disabled={isSummarizing || !summarizeText.trim()}
               onclick={summarizeTextHandler}
            >
               <span class:spin={isSummarizing}>
                  <Icon icon={isSummarizing ? 'lucide:loader-2' : 'lucide:scroll-text'} />
               </span>
               {isSummarizing ? 'Summarizing...' : 'Summarize'}
            </button>

            {#if summarizeResult}
               <div class='result-card compact'>
                  <pre>{summarizeResult.summary}</pre>
               </div>
            {/if}
         </section>

         <section class='sub-panel'>
            <h3>Rewrite</h3>
            <div class='field'>
               <label for='rewrite-tone'>Tone</label>
               <select id='rewrite-tone' bind:value={rewriteTone}>
                  {#each rewriteTones as tone}
                     <option value={tone.value}>{tone.label}</option>
                  {/each}
               </select>
            </div>

            <div class='field'>
               <textarea bind:value={rewriteText} rows='4'></textarea>
            </div>

            <button
               type='button'
               class='primary'
               disabled={isRewriting || !rewriteText.trim()}
               onclick={rewriteTextHandler}
            >
               <span class:spin={isRewriting}>
                  <Icon icon={isRewriting ? 'lucide:loader-2' : 'lucide:pencil-line'} />
               </span>
               {isRewriting ? 'Rewriting...' : 'Rewrite'}
            </button>

            {#if rewriteResult}
               <div class='result-card compact'>
                  <pre>{rewriteResult.rewrittenText}</pre>
               </div>
            {/if}
         </section>
      </div>
   </section>
</div>

<style lang='scss'>
   .tab-grid {
      display: grid;
      gap: 1rem;
      grid-template-columns: repeat(auto-fit, minmax(20rem, 1fr));
   }

   .panel,
   .sub-panel,
   .availability-card,
   .result-card {
      border: 1px solid oklch(100% 0 0 / 8%);
      border-radius: var(--radius-lg);
      background: oklch(100% 0 0 / 5%);
      box-shadow: var(--shadow-md);
   }

   .panel {
      display: flex;
      flex-direction: column;
      gap: 1rem;
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

   .actions-row,
   .control-grid,
   .availability-grid,
   .sub-panel-grid {
      display: grid;
      gap: 0.85rem;
   }

   .actions-row {
      grid-template-columns: repeat(auto-fit, minmax(10rem, max-content));
   }

   .control-grid,
   .availability-grid,
   .sub-panel-grid {
      grid-template-columns: repeat(auto-fit, minmax(14rem, 1fr));
   }

   .availability-card,
   .result-card,
   .sub-panel {
      padding: 1rem;
   }

   .availability-card {
      display: flex;
      flex-direction: column;
      gap: 0.45rem;

      &.wide {
         grid-column: 1 / -1;
      }

      .label {
         color: var(--text-muted);
         font-size: 0.78rem;
         text-transform: uppercase;
         letter-spacing: 0.08em;
      }

      strong,
      p {
         margin: 0;
      }

      strong.ok {
         color: var(--success);
      }
   }

   .reason {
      color: var(--text);
      line-height: 1.55;
      white-space: pre-wrap;
      word-break: break-word;
   }

   .chip-list {
      display: flex;
      flex-wrap: wrap;
      gap: 0.55rem;

      span {
         border-radius: 999px;
         background: oklch(100% 0 0 / 6%);
         color: var(--text);
         padding: 0.35rem 0.65rem;
         font-size: 0.8rem;
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

   textarea,
   input,
   select {
      width: 100%;
      border: 1px solid oklch(100% 0 0 / 8%);
      border-radius: var(--radius-md);
      background: oklch(9% 0.01 260 / 72%);
      color: var(--text);
      padding: 0.85rem 0.95rem;
   }

   textarea {
      resize: vertical;
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
      pre {
         margin: 0;
         white-space: pre-wrap;
         word-break: break-word;
         font-family: var(--font-mono);
         line-height: 1.6;
      }

      &.compact {
         margin-top: 0.85rem;
      }
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

   .result-meta {
      color: var(--text-muted);
      font-size: 0.82rem;
      margin-top: 0.8rem;
   }

   .cursor {
      animation: blink 1s step-end infinite;
   }

   .chat-box {
      display: flex;
      flex-direction: column;
      gap: 0.85rem;
   }

   .messages {
      display: flex;
      max-height: 20rem;
      flex-direction: column;
      gap: 0.85rem;
      overflow-y: auto;
   }

   .message {
      display: flex;
      gap: 0.75rem;
      align-items: flex-start;

      &.user {
         flex-direction: row-reverse;

         .bubble {
            background: linear-gradient(135deg, var(--accent), oklch(72% 0.2 220));
            color: white;
         }
      }

      &.error .bubble {
         border-color: oklch(60% 0.18 25 / 40%);
         color: oklch(84% 0.06 25);
      }
   }

   .avatar {
      display: grid;
      width: 2.25rem;
      height: 2.25rem;
      place-items: center;
      border-radius: 999px;
      background: oklch(100% 0 0 / 6%);
      color: var(--text-muted);
      flex-shrink: 0;
   }

   .bubble {
      flex: 1;
      border: 1px solid oklch(100% 0 0 / 8%);
      border-radius: var(--radius-md);
      background: oklch(9% 0.01 260 / 72%);
      line-height: 1.55;
      padding: 0.85rem 0.95rem;
      white-space: pre-wrap;
   }

   .chat-controls {
      display: grid;
      gap: 0.75rem;
      grid-template-columns: minmax(0, 1fr) auto auto;
   }

   .empty-state {
      display: grid;
      place-items: center;
      gap: 0.85rem;
      min-height: 14rem;
      border: 1px dashed oklch(100% 0 0 / 10%);
      border-radius: var(--radius-md);
      color: var(--text-muted);
      text-align: center;
      padding: 1.5rem;

      p {
         margin: 0;
         max-width: 22rem;
      }
   }

   .sub-panel-grid {
      align-items: start;
   }

   .sub-panel {
      display: flex;
      flex-direction: column;
      gap: 0.9rem;

      h3 {
         margin: 0;
      }
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

   @keyframes blink {
      0%,
      49% {
         opacity: 1;
      }

      50%,
      100% {
         opacity: 0;
      }
   }

   @media (max-width: 700px) {
      .chat-controls {
         grid-template-columns: 1fr;
      }
   }
</style>
