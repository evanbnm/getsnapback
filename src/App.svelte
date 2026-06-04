<script>
  import './app.css';
  import { locale } from './lib/i18n.js';
  import Welcome  from './screens/Welcome.svelte';
  import Progress from './screens/Progress.svelte';
  import Summary  from './screens/Summary.svelte';

  let screen = 'welcome';
  let options = null;
  let summary = null;

  function handleStart(e)   { options = e.detail; screen = 'progress'; }
  function handleDone(e)    { summary = e.detail;  screen = 'summary';  }
  function handleRestart()  { options = null; summary = null; screen = 'welcome'; }
  function toggleLang()     { $locale = $locale === 'en' ? 'fr' : 'en'; }
</script>

<!-- Language toggle: floats top-right above any card -->
<div style="position:fixed;top:14px;right:16px;z-index:100">
  <button class="lang-btn" on:click={toggleLang}>
    {$locale === 'en' ? 'FR' : 'EN'}
  </button>
</div>

{#if screen === 'welcome'}
  <Welcome on:start={handleStart} />
{:else if screen === 'progress'}
  <Progress {options} on:done={handleDone} />
{:else if screen === 'summary'}
  <Summary {summary} on:restart={handleRestart} />
{/if}
