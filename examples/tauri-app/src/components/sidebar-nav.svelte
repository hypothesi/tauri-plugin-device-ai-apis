<script>
   import Icon from '@iconify/svelte';

   let { items, activeTab, onSelect, platformInfo } = $props();
</script>

<aside class='sidebar'>
   <div class='brand'>
      <div class='brand-icon'>
         <Icon icon='lucide:brain-circuit' />
      </div>
      <div class='brand-copy'>
         <h1>Device AI</h1>
         <p>Plugin demo</p>
      </div>
   </div>

   <nav class='nav' aria-label='Demo sections'>
      {#each items as item}
         <button
            type='button'
            class='nav-btn'
            class:active={activeTab === item.id}
            aria-current={activeTab === item.id ? 'page' : undefined}
            onclick={() => {
               onSelect(item.id);
            }}
         >
            <span class='icon-wrap'>
               <Icon icon={item.icon} />
            </span>
            <span class='label'>{item.label}</span>

            {#if item.badge > 0}
               <span class='badge'>{item.badge}</span>
            {/if}
         </button>
      {/each}
   </nav>

   <div class='footer'>
      <div class='platform-pill' class:tauri={platformInfo.isTauri}>
         <Icon icon={platformInfo.isTauri ? 'lucide:terminal-square' : 'lucide:globe'} />
         <span>{platformInfo.isTauri ? 'Running in Tauri' : 'Running on the web'}</span>
      </div>
   </div>
</aside>

<style lang='scss'>
   .sidebar {
      display: flex;
      height: 100vh;
      flex-direction: column;
      gap: 1.5rem;
      border-inline-end: 1px solid var(--border);
      background: oklch(13% 0.02 260 / 72%);
      backdrop-filter: blur(22px);
      padding: 1.5rem 1rem 1rem;
      position: sticky;
      top: 0;
   }

   .brand {
      display: flex;
      align-items: center;
      gap: 0.9rem;
      padding-inline: 0.5rem;
   }

   .brand-icon {
      display: grid;
      width: 2.75rem;
      height: 2.75rem;
      place-items: center;
      border-radius: 1rem;
      background: linear-gradient(135deg, var(--accent), oklch(72% 0.2 220));
      box-shadow: var(--shadow-md);
      color: white;
      font-size: 1.35rem;
   }

   .brand-copy {
      h1,
      p {
         margin: 0;
      }

      h1 {
         font-size: 1rem;
         font-weight: 700;
         letter-spacing: -0.03em;
      }

      p {
         color: var(--text-muted);
         font-size: 0.82rem;
      }
   }

   .nav {
      display: flex;
      flex: 1;
      flex-direction: column;
      gap: 0.4rem;
   }

   .nav-btn {
      display: flex;
      width: 100%;
      align-items: center;
      gap: 0.85rem;
      border: 1px solid transparent;
      border-radius: var(--radius-md);
      background: transparent;
      color: var(--text-muted);
      cursor: pointer;
      padding: 0.8rem 0.9rem;
      text-align: start;
      transition: border-color 0.2s ease, background 0.2s ease, color 0.2s ease,
         transform 0.2s ease;

      &:hover {
         transform: translateX(0.1rem);
         border-color: oklch(100% 0 0 / 8%);
         background: oklch(100% 0 0 / 4%);
         color: var(--text);
      }

      &.active {
         border-color: oklch(100% 0 0 / 8%);
         background: linear-gradient(135deg, var(--accent-low), oklch(72% 0.2 220 / 8%));
         color: var(--text);
      }
   }

   .icon-wrap {
      display: grid;
      width: 2rem;
      height: 2rem;
      place-items: center;
      border-radius: 0.8rem;
      background: oklch(100% 0 0 / 4%);
      font-size: 1rem;
   }

   .label {
      flex: 1;
      font-weight: 500;
   }

   .badge {
      min-width: 1.6rem;
      border-radius: 999px;
      background: var(--accent);
      color: white;
      font-size: 0.75rem;
      font-weight: 700;
      padding: 0.15rem 0.45rem;
      text-align: center;
   }

   .footer {
      padding-inline: 0.4rem;
   }

   .platform-pill {
      display: inline-flex;
      width: 100%;
      align-items: center;
      gap: 0.65rem;
      border: 1px solid oklch(100% 0 0 / 8%);
      border-radius: var(--radius-md);
      background: oklch(100% 0 0 / 4%);
      color: var(--text-muted);
      font-size: 0.82rem;
      padding: 0.8rem 0.9rem;

      &.tauri {
         color: var(--success);
      }
   }

   @media (max-width: 980px) {
      .sidebar {
         height: auto;
         gap: 1rem;
         border-inline-end: none;
         border-bottom: 1px solid var(--border);
         padding: 1rem;
      }

      .nav {
         overflow-x: auto;
         flex-direction: row;
         padding-bottom: 0.25rem;
      }

      .nav-btn {
         min-width: max-content;
         white-space: nowrap;
      }
   }
</style>
