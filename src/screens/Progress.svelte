<script>
  import { createEventDispatcher, onMount, onDestroy, tick } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';
  import { t } from '../lib/i18n.js';
  import { formatDuration } from '../lib/format.js';

  export let options;

  const dispatch   = createEventDispatcher();

  let phaseCount = options?.overlay_videos ? 5 : 4;
  let phase      = 0;
  let phaseLabel = '';
  let processed  = 0;
  let total      = 0;
  let currentFile = null;
  let logs        = [];
  let logEl;

  let eta       = null;
  let elapsedInterval = null;
  let unlisten  = null;

  // ETA tracking — countdown model. We compute a target ETA only when an item
  // finishes (so the value updates on real signal), then tick it down smoothly
  // by elapsed wall-clock time between completions. avgMs uses a wide window
  // to avoid a single slow item swinging the displayed value.
  const ETA_WINDOW = 30;
  let prevPhase = 0;
  let prevProcessed = 0;
  let lastItemTime = null;      // timestamp of last processed-counter increment
  let itemDurations = [];        // ms per completed item, current phase only
  let etaTargetSec = null;       // ETA seconds at lastItemTime

  function recomputeEtaTarget(p) {
    if (!p.total || itemDurations.length === 0) { etaTargetSec = null; return; }
    const remaining = p.total - p.processed;
    if (remaining <= 0) { etaTargetSec = 0; return; }
    const avgMs = itemDurations.reduce((a, b) => a + b, 0) / itemDurations.length;
    etaTargetSec = (remaining * avgMs) / 1000;
  }

  onMount(async () => {
    unlisten = await listen('progress', (event) => {
      const p = event.payload;

      if (p.total_phases) phaseCount = p.total_phases;

      let completionEvent = false;
      if (p.phase !== prevPhase) {
        // New phase — reset all ETA state
        prevPhase = p.phase;
        prevProcessed = p.processed;
        lastItemTime = Date.now();
        itemDurations = [];
        etaTargetSec = null;
      } else if (p.processed > prevProcessed) {
        // One (or more) items just completed
        const now = Date.now();
        const completed = p.processed - prevProcessed;
        const perItem = (now - lastItemTime) / completed;
        for (let i = 0; i < completed; i++) itemDurations.push(perItem);
        if (itemDurations.length > ETA_WINDOW) {
          itemDurations = itemDurations.slice(-ETA_WINDOW);
        }
        lastItemTime = now;
        prevProcessed = p.processed;
        completionEvent = true;
      }

      phase = p.phase;
      phaseLabel = p.phase_label;
      processed = p.processed;
      total = p.total;
      currentFile = p.current_file ?? null;

      if (completionEvent) recomputeEtaTarget(p);
      tickEta();
    });

    elapsedInterval = setInterval(tickEta, 1000);

    try {
      const result = await invoke('start_processing', { request: options });
      dispatch('done', result);
    } catch (err) {
      logs = [...logs, { msg: `${$t('fatal_error')}: ${err}`, isError: true }];
      scrollLog();
    } finally {
      if (elapsedInterval) clearInterval(elapsedInterval);
    }
  });

  onDestroy(() => {
    if (unlisten) unlisten();
    if (elapsedInterval) clearInterval(elapsedInterval);
  });

  function tickEta() {
    if (etaTargetSec === null) { eta = null; return; }
    const elapsedSec = (Date.now() - lastItemTime) / 1000;
    eta = Math.max(0, etaTargetSec - elapsedSec);
  }

  async function scrollLog() {
    await tick();
    if (logEl) logEl.scrollTop = logEl.scrollHeight;
  }

  $: if (currentFile) {
    logs = [...logs, { msg: currentFile, isError: false }];
    scrollLog();
  }

  $: percent = total > 0 ? Math.min(100, (processed / total) * 100) : 0;
  $: isIndeterminate = total === 0 && phase > 0;
  $: errCount = logs.filter(l => l.isError).length;
</script>

<div class="screen">
  <div class="card" style="max-width:520px">

    <!-- Header -->
    <div style="margin-bottom:20px">
      <h1 style="margin-bottom:5px">{$t('progress_title')}</h1>
      <p class="muted">{$t('progress_sub')}</p>
    </div>

    <!-- Phase pips -->
    <div class="phases">
      {#each Array(phaseCount) as _, i}
        <div
          class="phase-pip"
          class:done={i + 1 < phase}
          class:active={i + 1 === phase}
        ></div>
      {/each}
    </div>

    <!-- Phase label + counter -->
    <div style="display:flex;justify-content:space-between;align-items:baseline;margin-bottom:8px">
      <span style="font-weight:700;font-size:14px">{phaseLabel ? $t(phaseLabel) : '…'}</span>
      <span class="muted" style="font-size:12px">
        {$t('phase_label')} {phase || '…'} / {phaseCount}
      </span>
    </div>

    <!-- Progress bar -->
    <div class="progress-track" style="margin-bottom:10px">
      <div
        class="progress-fill"
        class:indeterminate={isIndeterminate}
        style="width:{isIndeterminate ? 40 : percent}%"
      ></div>
    </div>

    <!-- Files + ETA -->
    <div style="display:flex;justify-content:space-between;margin-bottom:18px">
      <span class="muted" style="font-size:13px">
        {#if total > 0}
          {processed} / {total} {$t('files_unit')}
        {:else if phase > 0}
          {$t('in_progress')}
        {/if}
      </span>
      {#if eta !== null}
        <span class="muted" style="font-size:13px;display:inline-flex;align-items:center;gap:4px">
          <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
            <circle cx="12" cy="12" r="10"/>
            <path d="M12 6v6l3.5 3.5"/>
          </svg>
          ~{formatDuration(eta)} {$t('eta_remaining')}
        </span>
      {/if}
    </div>

    <div class="divider" style="margin:0 0 14px"></div>

    <!-- Current file -->
    <div class="field" style="margin-bottom:12px">
      <div class="label" style="margin-bottom:5px">{$t('current_file_label')}</div>
      <div style="
        font-size:12px;
        font-family:'SF Mono','Cascadia Code',monospace;
        color:var(--muted);
        overflow:hidden;
        text-overflow:ellipsis;
        white-space:nowrap;
        min-height:18px
      ">{currentFile ?? '—'}</div>
    </div>

    <!-- Log -->
    <div class="field">
      <div class="label" style="margin-bottom:5px">
        {$t('log_label')}
        {#if errCount > 0}
          <span style="color:var(--error);font-size:10px;font-weight:800;margin-left:6px">
            {errCount} {errCount === 1 ? $t('warning_one') : $t('warning_many')}
          </span>
        {/if}
      </div>
      <div class="log-box" bind:this={logEl}>
        {#if logs.length === 0}
          <span>{$t('log_waiting')}</span>
        {:else}
          {#each logs as line}
            <div class:log-error={line.isError}>{line.msg}</div>
          {/each}
        {/if}
      </div>
    </div>

  </div>
</div>
