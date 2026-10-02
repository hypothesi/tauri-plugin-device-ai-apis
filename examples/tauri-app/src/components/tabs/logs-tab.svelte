<script>
   import Icon from '@iconify/svelte';

   let { logs, clearLogs } = $props();
</script>

<section class='panel'>
   <div class='panel-header'>
      <div class='copy'>
         <Icon icon='lucide:terminal-square' />
         <div>
            <h2>Activity logs</h2>
            <p>Recent plugin activity, status updates, and error messages.</p>
         </div>
      </div>

      <button type='button' onclick={clearLogs} disabled={logs.length === 0}>
         Clear
      </button>
   </div>

   <div class='log-list'>
      {#each logs as logEntry (logEntry.id)}
         <article class='log-entry' class:error={logEntry.type === 'error'} class:success={logEntry.type === 'success'}>
            <time>{logEntry.timestamp}</time>
            <p>{logEntry.message}</p>
         </article>
      {:else}
         <div class='empty-state'>No activity yet. Use a tool panel and logs will appear here.</div>
      {/each}
   </div>
</section>

<style lang='scss'>
   .panel {
      display: flex;
      flex-direction: column;
      gap: 1rem;
      min-height: 34rem;
      border: 1px solid oklch(100% 0 0 / 8%);
      border-radius: var(--radius-lg);
      background: oklch(100% 0 0 / 5%);
      box-shadow: var(--shadow-md);
      padding: 1.3rem;
   }

   .panel-header,
   .copy {
      display: flex;
      align-items: flex-start;
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
         margin-top: 0.1rem;
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

   button {
      border: 1px solid oklch(100% 0 0 / 8%);
      border-radius: var(--radius-md);
      background: oklch(100% 0 0 / 5%);
      color: var(--text);
      cursor: pointer;
      padding: 0.75rem 1rem;

      &:disabled {
         cursor: not-allowed;
         opacity: 0.6;
      }
   }

   .log-list {
      display: flex;
      flex: 1;
      flex-direction: column;
      gap: 0.75rem;
      overflow-y: auto;
   }

   .log-entry,
   .empty-state {
      border: 1px solid oklch(100% 0 0 / 8%);
      border-radius: var(--radius-md);
      background: oklch(9% 0.01 260 / 72%);
      padding: 0.95rem 1rem;
   }

   .log-entry {
      display: grid;
      gap: 0.35rem;
      grid-template-columns: 1fr;
      font-family: var(--font-mono);

      time {
         color: var(--text-muted);
         font-size: 0.78rem;
      }

      p {
         margin: 0;
         line-height: 1.55;
      }

      &.error p {
         color: oklch(84% 0.06 25);
      }

      &.success p {
         color: var(--success);
      }
   }

   .empty-state {
      display: grid;
      flex: 1;
      place-items: center;
      color: var(--text-muted);
      text-align: center;
   }
</style>
