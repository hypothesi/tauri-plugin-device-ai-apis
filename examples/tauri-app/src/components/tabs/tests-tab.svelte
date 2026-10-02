<script>
   import Icon from '@iconify/svelte';

   let { testResults, isRunningTests, testProgress, runTests } = $props();

   function getStatusIcon(status) {
      if (status === 'pass') {
         return 'lucide:badge-check';
      }

      if (status === 'fail') {
         return 'lucide:circle-x';
      }

      if (status === 'error') {
         return 'lucide:triangle-alert';
      }

      return 'lucide:minus-circle';
   }
</script>

<section class='panel'>
   <div class='panel-header'>
      <div class='copy'>
         <Icon icon='lucide:flask-conical' />
         <div>
            <h2>End-to-end tests</h2>
            <p>Run the sample-based validation suite across the shipped AI features.</p>
         </div>
      </div>

      {#if isRunningTests}
         <span class='progress-pill'>
            {testProgress.current}/{testProgress.total}
         </span>
      {/if}
   </div>

   <button type='button' class='primary large' disabled={isRunningTests} onclick={runTests}>
      <span class:spin={isRunningTests}>
         <Icon icon={isRunningTests ? 'lucide:loader-2' : 'lucide:play-circle'} />
      </span>
      {isRunningTests
         ? `Running (${testProgress.current}/${testProgress.total})...`
         : 'Run tests'}
   </button>

   {#if testResults.length > 0}
      <div class='result-list'>
         {#each testResults as result}
            <article class='result-card' class:pass={result.status === 'pass'} class:fail={result.status === 'fail'} class:error={result.status === 'error'}>
               <div class='result-header'>
                  <div class='title-row'>
                     <Icon icon={getStatusIcon(result.status)} />
                     <strong>{result.name}</strong>
                  </div>
                  <span class='status-badge'>{result.status.toUpperCase()}</span>
               </div>

               <p class='description'>{result.description}</p>
               <p class='details'>{result.details}</p>
            </article>
         {/each}
      </div>
   {/if}
</section>

<style lang='scss'>
   .panel,
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

   .panel-header,
   .copy,
   .result-header,
   .title-row {
      display: flex;
      align-items: center;
      gap: 0.85rem;
      justify-content: space-between;
   }

   .panel-header {
      flex-wrap: wrap;
   }

   .copy {
      justify-content: flex-start;

      :global(svg) {
         color: var(--accent);
         font-size: 1.2rem;
         flex-shrink: 0;
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

   .progress-pill {
      border-radius: 999px;
      background: oklch(72% 0.2 220 / 14%);
      color: var(--accent);
      font-weight: 700;
      padding: 0.45rem 0.75rem;
   }

   button {
      display: inline-flex;
      align-items: center;
      justify-content: center;
      gap: 0.5rem;
      border: 1px solid transparent;
      border-radius: var(--radius-md);
      cursor: pointer;
      padding: 0.9rem 1rem;

      &:disabled {
         cursor: not-allowed;
         opacity: 0.6;
      }
   }

   .primary {
      background: linear-gradient(135deg, var(--accent), oklch(72% 0.2 220));
      color: white;

      &.large {
         width: 100%;
      }
   }

   .result-list {
      display: grid;
      gap: 0.85rem;
   }

   .result-card {
      padding: 1rem;

      &.pass {
         border-color: oklch(70% 0.15 150 / 25%);
      }

      &.fail {
         border-color: oklch(80% 0.16 90 / 25%);
      }

      &.error {
         border-color: oklch(60% 0.18 25 / 35%);
      }
   }

   .result-header {
      flex-wrap: wrap;
   }

   .title-row {
      justify-content: flex-start;
   }

   .status-badge {
      border-radius: 999px;
      background: oklch(100% 0 0 / 6%);
      color: var(--text-muted);
      font-size: 0.78rem;
      font-weight: 700;
      letter-spacing: 0.08em;
      padding: 0.35rem 0.65rem;
   }

   .description,
   .details {
      margin: 0.75rem 0 0;
      line-height: 1.55;
   }

   .description {
      color: var(--text-muted);
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
