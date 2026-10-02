<script>
   import Icon from '@iconify/svelte';

   import { capabilityItems } from '../../demo-config.js';

   let { platformInfo, capabilities } = $props();

   function formatLanguages(languages = []) {
      if (languages.length <= 3) {
         return languages.join(', ');
      }

      return `${languages.slice(0, 3).join(', ')} +${languages.length - 3}`;
   }
</script>

<div class='tab'>
   <section class='section'>
      <div class='section-copy'>
         <h2>Platform support</h2>
         <p>Host environment and related browser APIs available to the demo.</p>
      </div>

      <div class='stats-grid'>
         <article class='stat-card'>
            <span class='eyebrow'>Environment</span>
            <strong>{platformInfo.isTauri ? 'Tauri v2 runtime' : 'Web browser'}</strong>
         </article>

         <article class='stat-card' class:supported={platformInfo.hasWebSpeech}>
            <span class='eyebrow'>Web Speech API</span>
            <strong>{platformInfo.hasWebSpeech ? 'Available' : 'Unavailable'}</strong>
         </article>

         <article class='stat-card' class:supported={platformInfo.hasWebSynthesis}>
            <span class='eyebrow'>Speech synthesis</span>
            <strong>{platformInfo.hasWebSynthesis ? 'Available' : 'Unavailable'}</strong>
         </article>

         <article class='stat-card' class:supported={platformInfo.hasWebBarcode}>
            <span class='eyebrow'>Barcode detector</span>
            <strong>{platformInfo.hasWebBarcode ? 'Available' : 'Unavailable'}</strong>
         </article>
      </div>
   </section>

   <section class='section'>
      <div class='section-copy'>
         <h2>Device capabilities</h2>
         <p>Feature availability reported by the plugin on the current hardware.</p>
      </div>

      {#if capabilities}
         <div class='capability-grid'>
            {#each capabilityItems as capability}
               {@const feature = capabilities[capability.key]}

               <article class='capability-card' class:available={feature?.available}>
                  <div class='capability-header'>
                     <div class='icon-wrap'>
                        <Icon icon={capability.icon} />
                     </div>
                     <div class='status-pill' class:available={feature?.available}>
                        {feature?.available ? 'Supported' : 'Unavailable'}
                     </div>
                  </div>

                  <h3>{capability.label}</h3>

                  <div class='meta'>
                     {#if feature?.available}
                        <span class='meta-pill'>
                           <Icon icon={feature.onDevice ? 'lucide:shield-check' : 'lucide:cloud'} />
                           {feature.onDevice ? 'On-device' : 'Cloud backed'}
                        </span>

                        {#if feature.requiresPermission}
                           <span class='meta-pill warning'>
                              <Icon icon='lucide:lock-keyhole' />
                              Permission required
                           </span>
                        {/if}

                        {#if feature.supportedLanguages?.length}
                           <p class='detail'>
                              Supports {formatLanguages(feature.supportedLanguages)}.
                           </p>
                        {/if}
                     {:else}
                        <p class='detail'>This feature is not available in the current environment.</p>
                     {/if}
                  </div>
               </article>
            {/each}
         </div>
      {:else}
         <div class='skeleton-grid'>
            {#each capabilityItems as capability}
               <div class='skeleton-card' aria-hidden='true'></div>
            {/each}
         </div>
      {/if}
   </section>
</div>

<style lang='scss'>
   .tab {
      display: flex;
      flex-direction: column;
      gap: 2rem;
   }

   .section {
      display: flex;
      flex-direction: column;
      gap: 1.25rem;
   }

   .section-copy {
      h2,
      p {
         margin: 0;
      }

      h2 {
         font-size: 1.25rem;
         letter-spacing: -0.03em;
      }

      p {
         color: var(--text-muted);
         margin-top: 0.35rem;
      }
   }

   .stats-grid,
   .capability-grid,
   .skeleton-grid {
      display: grid;
      gap: 1rem;
      grid-template-columns: repeat(auto-fit, minmax(13rem, 1fr));
   }

   .stat-card,
   .capability-card,
   .skeleton-card {
      border: 1px solid oklch(100% 0 0 / 8%);
      border-radius: var(--radius-lg);
      background: oklch(100% 0 0 / 5%);
      box-shadow: var(--shadow-md);
      padding: 1.2rem;
   }

   .stat-card {
      display: flex;
      flex-direction: column;
      gap: 0.45rem;

      &.supported strong {
         color: var(--success);
      }
   }

   .eyebrow {
      color: var(--text-muted);
      font-size: 0.78rem;
      text-transform: uppercase;
      letter-spacing: 0.08em;
   }

   .capability-card {
      display: flex;
      flex-direction: column;
      gap: 1rem;

      &.available {
         border-color: oklch(72% 0.2 220 / 28%);
         background: linear-gradient(135deg, var(--accent-low), oklch(72% 0.2 220 / 7%));
      }

      h3,
      p {
         margin: 0;
      }
   }

   .capability-header {
      display: flex;
      align-items: center;
      justify-content: space-between;
      gap: 0.75rem;
   }

   .icon-wrap {
      display: grid;
      width: 2.5rem;
      height: 2.5rem;
      place-items: center;
      border-radius: 1rem;
      background: oklch(100% 0 0 / 6%);
      color: var(--accent);
      font-size: 1.15rem;
   }

   .status-pill,
   .meta-pill {
      display: inline-flex;
      width: fit-content;
      align-items: center;
      gap: 0.35rem;
      border-radius: 999px;
      padding: 0.35rem 0.65rem;
      font-size: 0.8rem;
      font-weight: 600;
   }

   .status-pill {
      background: oklch(100% 0 0 / 6%);
      color: var(--text-muted);

      &.available {
         background: oklch(70% 0.15 150 / 16%);
         color: var(--success);
      }
   }

   .meta {
      display: flex;
      flex-wrap: wrap;
      gap: 0.55rem;
      align-items: flex-start;
   }

   .meta-pill {
      background: oklch(100% 0 0 / 6%);
      color: var(--text);

      &.warning {
         background: oklch(80% 0.16 90 / 14%);
         color: var(--warning);
      }
   }

   .detail {
      width: 100%;
      color: var(--text-muted);
      line-height: 1.5;
   }

   .skeleton-card {
      min-height: 12rem;
      background:
         linear-gradient(
            90deg,
            oklch(100% 0 0 / 4%) 0%,
            oklch(100% 0 0 / 8%) 50%,
            oklch(100% 0 0 / 4%) 100%
         );
      background-size: 200% 100%;
      animation: shimmer 1.4s linear infinite;
   }

   @keyframes shimmer {
      from {
         background-position: 200% 0;
      }

      to {
         background-position: -200% 0;
      }
   }
</style>
