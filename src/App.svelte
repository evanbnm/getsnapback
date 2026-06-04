<script>
  import { onMount } from 'svelte';
  import { open as openExternal } from '@tauri-apps/plugin-shell';
  import './app.css';
  import { locale, t } from './lib/i18n.js';
  import { checkForUpdate, dismissUpdate } from './lib/version.js';
  import Welcome  from './screens/Welcome.svelte';
  import Progress from './screens/Progress.svelte';
  import Summary  from './screens/Summary.svelte';

  let screen = 'welcome';
  let options = null;
  let summary = null;

  /** @type {{tag: string, url: string} | null} */
  let update = null;

  onMount(async () => {
    update = await checkForUpdate();
  });

  function handleStart(e)      { options = e.detail; screen = 'progress'; }
  function handleDone(e)       { summary = e.detail;  screen = 'summary';  }
  function handleCancelled()   { options = null; screen = 'welcome'; }
  function handleRestart() {
    options = null;
    summary = null;
    screen = 'welcome';
    // Summary screen can be scrolled (year histogram + import steps). Reset
    // viewport so the user lands at the top of the welcome screen, not wherever
    // they left off on Summary.
    window.scrollTo(0, 0);
  }
  function toggleLang()        { $locale = $locale === 'en' ? 'fr' : 'en'; }

  async function openUpdate() {
    if (!update) return;
    try { await openExternal(update.url); } catch (e) { console.error(e); }
  }

  function dismiss() {
    if (update) {
      dismissUpdate(update.tag);
      update = null;
    }
  }
</script>

<!-- Update banner: only shows when a newer release exists on GitHub and the
     user hasn't already dismissed this specific version. -->
{#if update}
  <div class="update-banner">
    <button class="update-pill" on:click={openUpdate}>
      <span class="update-dot"></span>
      <span class="update-text">
        <strong>{$t('update_available')}</strong>
        <span class="update-tag">{update.tag}</span>
      </span>
      <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor"
           stroke-width="2.4" stroke-linecap="round" stroke-linejoin="round">
        <path d="M7 17L17 7"/>
        <path d="M9 7h8v8"/>
      </svg>
    </button>
    <button class="update-dismiss" on:click={dismiss} aria-label="dismiss">
      <svg width="11" height="11" viewBox="0 0 24 24" fill="none" stroke="currentColor"
           stroke-width="2.6" stroke-linecap="round">
        <path d="M6 6l12 12M18 6l-12 12"/>
      </svg>
    </button>
  </div>
{/if}

<!-- Language toggle: floats top-right above any card -->
<div style="position:fixed;top:14px;right:16px;z-index:100">
  <button class="lang-btn" on:click={toggleLang}>
    {$locale === 'en' ? 'FR' : 'EN'}
  </button>
</div>

{#if screen === 'welcome'}
  <Welcome on:start={handleStart} />
{:else if screen === 'progress'}
  <Progress {options} on:done={handleDone} on:cancelled={handleCancelled} />
{:else if screen === 'summary'}
  <Summary {summary} on:restart={handleRestart} />
{/if}
