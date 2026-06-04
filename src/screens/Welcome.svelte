<script>
  import { createEventDispatcher, onMount, onDestroy } from 'svelte';
  import { open as openDialog } from '@tauri-apps/plugin-dialog';
  import { open as openExternal } from '@tauri-apps/plugin-shell';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { t } from '../lib/i18n.js';
  import { defaultOutputPath, shortPath } from '../lib/format.js';

  const SNAP_EXPORT_URL = 'https://accounts.snapchat.com/v2/download-my-data';

  const dispatch = createEventDispatcher();

  /** @type {string[]} */
  let inputPaths = [];
  let outputPath = '';
  let overlayPhotos = true;
  let overlayVideos = true;
  let isDragOver = false;
  let unlisten   = null;

  onMount(async () => {
    unlisten = await getCurrentWindow().onDragDropEvent((e) => {
      if (e.payload.type === 'over') {
        isDragOver = true;
      } else if (e.payload.type === 'drop') {
        isDragOver = false;
        if (e.payload.paths?.length) addInputs(e.payload.paths);
      } else {
        isDragOver = false;
      }
    });
  });

  onDestroy(() => { if (unlisten) unlisten(); });

  function addInputs(paths) {
    const seen = new Set(inputPaths);
    for (const p of paths) {
      if (!seen.has(p)) {
        seen.add(p);
        inputPaths = [...inputPaths, p];
      }
    }
    if (!outputPath && inputPaths.length > 0) {
      outputPath = defaultOutputPath(inputPaths[0]);
    }
  }

  function clearInputs() {
    inputPaths = [];
  }

  function fileName(path) {
    if (!path) return '';
    const sep = path.includes('/') ? '/' : '\\';
    const parts = path.split(sep).filter(Boolean);
    return parts[parts.length - 1] || path;
  }

  async function pickZips() {
    const r = await openDialog({
      multiple: true,
      directory: false,
      filters: [
        { name: 'Snapchat export', extensions: ['zip'] },
        { name: 'All files', extensions: ['*'] },
      ],
    });
    if (r) addInputs(Array.isArray(r) ? r : [r]);
  }

  async function pickFolders() {
    const r = await openDialog({ multiple: true, directory: true });
    if (r) addInputs(Array.isArray(r) ? r : [r]);
  }

  async function pickOutput() {
    const r = await openDialog({ multiple: false, directory: true });
    if (r) outputPath = r;
  }

  function start() {
    dispatch('start', {
      input_paths:    inputPaths,
      output_path:    outputPath,
      overlay_photos: overlayPhotos,
      overlay_videos: overlayVideos,
    });
  }

  async function openSnapExport() {
    try {
      await openExternal(SNAP_EXPORT_URL);
    } catch (e) {
      console.error('Failed to open Snapchat export URL:', e);
    }
  }

  $: hasInputs = inputPaths.length > 0;
</script>

<div class="screen">
  <div class="card" style="max-width:520px">

    <!-- Header -->
    <div style="margin-bottom:20px">
      <div style="margin-bottom:10px">
        <span class="badge">
          <svg width="11" height="11" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
            <rect x="3" y="11" width="18" height="11" rx="2"/>
            <path d="M7 11V7a5 5 0 0 1 10 0v4"/>
          </svg>
          {$t('local_badge')}
        </span>
      </div>
      <h1 style="margin-bottom:5px">GetSnapBack</h1>
      <p class="muted">{$t('app_subtitle')}</p>
    </div>

    <!-- Snapchat export link: opens the official "Download my data" page so
         users who don't have their export yet can grab it without leaving the
         app context. -->
    <button
      class="btn btn-outline snap-export-btn"
      on:click={openSnapExport}
      style="width:100%;margin-bottom:14px;font-size:13px;gap:7px"
    >
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor"
           stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round">
        <path d="M7 17L17 7"/>
        <path d="M9 7h8v8"/>
      </svg>
      {$t('snap_export_btn')}
    </button>

    <!-- Drop zone -->
    <!-- svelte-ignore a11y-click-events-have-key-events a11y-no-static-element-interactions -->
    <div
      class="drop-zone"
      class:drag-over={isDragOver}
      class:has-file={hasInputs && !isDragOver}
      on:click={hasInputs ? undefined : pickZips}
    >
      {#if hasInputs}
        <!-- Check circle -->
        <span class="drop-icon">
          <svg width="48" height="48" viewBox="0 0 48 48" fill="none">
            <circle cx="24" cy="24" r="22" fill="#10B981" opacity=".12"/>
            <circle cx="24" cy="24" r="17" fill="#10B981"/>
            <path d="M15 24l7 7 11-11" stroke="#fff" stroke-width="2.8" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
        </span>
        <div style="font-weight:700;font-size:14px">
          {inputPaths.length === 1
            ? $t('drop_selected_title')
            : `${inputPaths.length} ${$t('drop_selected_many')}`}
        </div>
        <ul class="input-list">
          {#each inputPaths.slice(0, 3) as p}
            <li>{fileName(p)}</li>
          {/each}
          {#if inputPaths.length > 3}
            <li class="muted">+ {inputPaths.length - 3} {$t('drop_selected_more')}</li>
          {/if}
        </ul>
        <div style="display:flex;gap:6px;justify-content:center;margin-top:10px;flex-wrap:wrap">
          <button class="btn btn-ghost" on:click|stopPropagation={pickZips}>
            {$t('btn_add_zip')}
          </button>
          <button class="btn btn-ghost" on:click|stopPropagation={pickFolders}>
            {$t('btn_add_folder')}
          </button>
          <button class="btn btn-ghost" on:click|stopPropagation={clearInputs} style="color:var(--warning)">
            {$t('btn_clear')}
          </button>
        </div>
      {:else if isDragOver}
        <span class="drop-icon">
          <svg width="48" height="48" viewBox="0 0 48 48" fill="none">
            <path d="M6 18a3 3 0 0 1 3-3h8l3.5 4H39a3 3 0 0 1 3 3v14a3 3 0 0 1-3 3H9a3 3 0 0 1-3-3V18z" fill="var(--primary)" opacity=".9"/>
            <path d="M6 22h36" stroke="var(--primary-h)" stroke-width="1.5"/>
            <circle cx="24" cy="31" r="6" fill="white" opacity=".25"/>
            <path d="M24 28v6M21 31l3 3 3-3" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
        </span>
        <div style="font-weight:700;font-size:14px">{$t('drop_over_title')}</div>
      {:else}
        <span class="drop-icon">
          <svg width="48" height="48" viewBox="0 0 48 48" fill="none">
            <rect x="2" y="2" width="44" height="44" rx="14" fill="var(--primary-10)"/>
            <path d="M24 33V19" stroke="var(--primary)" stroke-width="2.5" stroke-linecap="round"/>
            <path d="M17 26l7-7 7 7" stroke="var(--primary)" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"/>
            <path d="M14 36h20" stroke="var(--primary)" stroke-width="2.5" stroke-linecap="round"/>
          </svg>
        </span>
        <div style="font-weight:700;font-size:14px">{$t('drop_idle_title')}</div>
        <p class="muted" style="margin-top:5px;font-size:12px">{$t('drop_idle_sub')}</p>
        <div style="display:flex;gap:8px;justify-content:center;margin-top:14px;flex-wrap:wrap">
          <button class="btn btn-outline" on:click|stopPropagation={pickZips}>
            {$t('btn_pick_zip')}
          </button>
          <button class="btn btn-outline" on:click|stopPropagation={pickFolders}>
            {$t('btn_pick_folder')}
          </button>
        </div>
      {/if}
    </div>

    <div class="divider"></div>

    <!-- Options -->
    <div style="margin-bottom:18px">
      <div class="label" style="margin-bottom:8px">{$t('options_heading')}</div>

      <div class="check-row">
        <input type="checkbox" id="ov-photo" bind:checked={overlayPhotos} />
        <div>
          <label for="ov-photo">{$t('opt_photo_label')}</label>
          <div class="check-desc">{$t('opt_photo_desc')}</div>
        </div>
      </div>

      <div class="check-row">
        <input type="checkbox" id="ov-video" bind:checked={overlayVideos} />
        <div>
          <label for="ov-video">
            {$t('opt_video_label')}
            <span class="warn-tag">
              <svg width="9" height="9" viewBox="0 0 24 24" fill="currentColor">
                <path d="M12 2L1 21h22L12 2zm1 15h-2v-2h2v2zm0-4h-2V9h2v4z"/>
              </svg>
              {$t('opt_slow')}
            </span>
          </label>
          <div class="check-desc">{$t('opt_video_desc')}</div>
        </div>
      </div>
    </div>

    <!-- Output path -->
    <div class="field" style="margin-bottom:22px">
      <div class="label">{$t('output_label')}</div>
      <div class="path-row" class:empty={!outputPath}>
        <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"
             style="flex-shrink:0;color:var(--muted)">
          <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/>
        </svg>
        <span>{outputPath ? shortPath(outputPath) : $t('output_placeholder')}</span>
        <button class="btn btn-ghost" style="flex-shrink:0;padding:3px 8px" on:click={pickOutput}>
          {$t('btn_change_output')}
        </button>
      </div>
    </div>

    <!-- Start -->
    <button class="btn btn-primary" disabled={!hasInputs || !outputPath} on:click={start}>
      <svg width="15" height="15" viewBox="0 0 24 24" fill="currentColor">
        <polygon points="5 3 19 12 5 21 5 3"/>
      </svg>
      {$t('btn_start')}
    </button>

  </div>
</div>
