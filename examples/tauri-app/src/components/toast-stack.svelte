<script>
   import Icon from '@iconify/svelte';

   let { toasts, onDismiss } = $props();

   function getToastIcon(type) {
      return type === 'error' ? 'lucide:alert-triangle' : 'lucide:badge-check';
   }
</script>

<div class='toast-stack' aria-live='polite'>
   {#each toasts as toast (toast.id)}
      <div class='toast' class:error={toast.type === 'error'}>
         <Icon icon={getToastIcon(toast.type)} />
         <p>{toast.message}</p>
         <button
            type='button'
            aria-label='Dismiss notification'
            onclick={() => {
               onDismiss(toast.id);
            }}
         >
            <Icon icon='lucide:x' />
         </button>
      </div>
   {/each}
</div>

<style lang='scss'>
   .toast-stack {
      position: fixed;
      top: 1.5rem;
      right: 1.5rem;
      z-index: 20;
      display: flex;
      max-width: min(24rem, calc(100vw - 2rem));
      flex-direction: column;
      gap: 0.8rem;
   }

   .toast {
      display: grid;
      grid-template-columns: auto minmax(0, 1fr) auto;
      align-items: start;
      gap: 0.8rem;
      border: 1px solid oklch(100% 0 0 / 8%);
      border-radius: var(--radius-md);
      background: oklch(14% 0.02 260 / 95%);
      box-shadow: var(--shadow-lg);
      padding: 0.9rem 1rem;

      &.error {
         border-color: oklch(60% 0.18 25 / 55%);
         color: oklch(84% 0.06 25);
      }

      p {
         margin: 0;
         color: var(--text);
         font-size: 0.88rem;
         line-height: 1.45;
      }

      button {
         border: none;
         background: transparent;
         color: var(--text-muted);
         cursor: pointer;
         padding: 0;
      }
   }

   @media (max-width: 980px) {
      .toast-stack {
         top: auto;
         right: 1rem;
         bottom: 1rem;
         left: 1rem;
         max-width: none;
      }
   }
</style>
