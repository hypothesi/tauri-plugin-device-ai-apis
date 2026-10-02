<script>
   import Icon from '@iconify/svelte';

   let { processingStatus, onRefresh } = $props();
</script>

<header class='top-bar'>
   <div class='status-strip'>
      {#if processingStatus}
         <div class='status-pill is-busy'>
            <span class='spinner'></span>
            <span>{processingStatus.message}</span>
         </div>
      {:else}
         <div class='status-pill is-ready'>
            <Icon icon='lucide:check-circle-2' />
            <span>System ready</span>
         </div>
      {/if}
   </div>

   <button
      type='button'
      class='refresh-btn'
      aria-label='Refresh capabilities'
      title='Refresh capabilities'
      onclick={onRefresh}
   >
      <Icon icon='lucide:refresh-cw' />
   </button>
</header>

<style lang='scss'>
   .top-bar {
      display: flex;
      align-items: center;
      justify-content: space-between;
      gap: 1rem;
      border-bottom: 1px solid var(--border);
      background: oklch(13% 0.02 260 / 78%);
      backdrop-filter: blur(22px);
      padding: 1rem 1.5rem;
      position: sticky;
      top: 0;
      z-index: 10;
   }

   .status-strip {
      min-width: 0;
   }

   .status-pill {
      display: inline-flex;
      max-width: 100%;
      align-items: center;
      gap: 0.65rem;
      border: 1px solid oklch(100% 0 0 / 8%);
      border-radius: 999px;
      background: oklch(100% 0 0 / 5%);
      color: var(--text);
      font-size: 0.9rem;
      font-weight: 500;
      padding: 0.65rem 0.9rem;

      &.is-ready {
         color: var(--success);
      }

      span:last-child {
         overflow: hidden;
         text-overflow: ellipsis;
         white-space: nowrap;
      }
   }

   .spinner {
      width: 0.9rem;
      height: 0.9rem;
      border: 2px solid oklch(100% 0 0 / 14%);
      border-top-color: var(--accent);
      border-radius: 999px;
      animation: spin 1s linear infinite;
      flex-shrink: 0;
   }

   .refresh-btn {
      display: inline-flex;
      align-items: center;
      justify-content: center;
      border: 1px solid oklch(100% 0 0 / 8%);
      border-radius: var(--radius-md);
      background: oklch(100% 0 0 / 5%);
      color: var(--text);
      cursor: pointer;
      padding: 0.75rem;
      transition: transform 0.2s ease, border-color 0.2s ease;

      &:hover {
         transform: translateY(-0.05rem);
         border-color: oklch(100% 0 0 / 18%);
      }
   }

   @keyframes spin {
      from {
         transform: rotate(0deg);
      }

      to {
         transform: rotate(360deg);
      }
   }

   @media (max-width: 980px) {
      .top-bar {
         padding: 1rem;
      }
   }
</style>
