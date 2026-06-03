<script>
  import { createEventDispatcher } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { t } from '../lib/i18n.js';
  import { shortPath } from '../lib/format.js';

  export let summary;

  const dispatch = createEventDispatcher();

  let showErrors = false;

  $: total = (summary?.photos_dated   ?? 0)
           + (summary?.videos_dated   ?? 0)
           + (summary?.overlays_photo ?? 0)
           + (summary?.overlays_video ?? 0);

  async function openOutputFolder() {
    if (summary?.output_path) {
      await invoke('open_folder', { path: summary.output_path });
    }
  }
</script>

<div class="screen">
  <div class="card" style="max-width:520px">

    <!-- Header -->
    <div style="text-align:center;margin-bottom:22px">
      <!-- Celebration icon: checkmark + sparkles -->
      <div style="margin-bottom:12px;display:flex;justify-content:center">
        <svg width="72" height="72" viewBox="0 0 72 72" fill="none">
          <!-- Glow ring -->
          <circle cx="36" cy="36" r="32" fill="var(--primary)" opacity=".1"/>
          <!-- Main circle -->
          <circle cx="36" cy="36" r="24" fill="var(--primary)"/>
          <!-- Checkmark -->
          <path d="M24 36l9 9 15-15" stroke="white" stroke-width="3.5" stroke-linecap="round" stroke-linejoin="round"/>
          <!-- Sparkle top-left -->
          <path d="M12 14 L13.2 10 L14.4 14 L18 15.2 L14.4 16.4 L13.2 20 L12 16.4 L8 15.2 Z" fill="#FFEF5C"/>
          <!-- Sparkle top-right -->
          <path d="M58 10 L59 7 L60 10 L63 11 L60 12 L59 15 L58 12 L55 11 Z" fill="var(--accent)"/>
          <!-- Small dots -->
          <circle cx="62" cy="28" r="2.5" fill="var(--primary)" opacity=".5"/>
          <circle cx="9"  cy="46" r="2"   fill="var(--accent)"  opacity=".6"/>
          <circle cx="64" cy="48" r="1.8" fill="#FFEF5C"/>
          <circle cx="14" cy="26" r="1.5" fill="var(--primary)" opacity=".4"/>
        </svg>
      </div>
      <h1>{$t('done_title')}</h1>
      <p class="muted" style="margin-top:5px">
        {total} {total === 1 ? $t('done_sub_one') : $t('done_sub_many')}
      </p>
    </div>

    <!-- Stats -->
    <div class="stats-grid" style="margin-bottom:20px">
      <div class="stat-card">
        <div class="stat-value">{summary?.photos_dated ?? 0}</div>
        <div class="stat-label">{$t('stat_photos')}</div>
      </div>
      <div class="stat-card">
        <div class="stat-value">{summary?.videos_dated ?? 0}</div>
        <div class="stat-label">{$t('stat_videos')}</div>
      </div>
      <div class="stat-card">
        <div class="stat-value">{summary?.overlays_photo ?? 0}</div>
        <div class="stat-label">{$t('stat_overlays')}</div>
      </div>
      <div class="stat-card">
        <div class="stat-value">{(summary?.dedup_content ?? 0) + (summary?.dedup_uuid ?? 0)}</div>
        <div class="stat-label">{$t('stat_dupes')}</div>
      </div>
    </div>

    <!-- Output path -->
    {#if summary?.output_path}
      <div class="path-row" style="margin-bottom:12px">
        <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"
             style="flex-shrink:0;color:var(--success)">
          <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/>
        </svg>
        <span style="color:var(--text)">{shortPath(summary.output_path)}</span>
      </div>
    {/if}

    <!-- Open folder -->
    <button class="btn btn-primary" style="margin-bottom:10px" on:click={openOutputFolder}>
      <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
        <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/>
      </svg>
      {$t('btn_open_folder')}
    </button>

    <div class="divider"></div>

    <!-- Import steps -->
    <div class="steps" style="margin-bottom:18px">
      <div class="label" style="margin-bottom:8px">{$t('import_heading')}</div>
      <ol>
        <li>{$t('import_s1')}</li>
        <li>{$t('import_s2')}</li>
        <li>{$t('import_s3')}</li>
        <li>{$t('import_s4')}</li>
      </ol>
    </div>

    <!-- Errors collapsible -->
    {#if summary?.errors?.length > 0}
      <div style="margin-bottom:14px">
        <button
          class="btn btn-ghost"
          style="padding:0;margin-bottom:8px;color:var(--warning)"
          on:click={() => showErrors = !showErrors}
        >
          <svg width="11" height="11" viewBox="0 0 24 24" fill="currentColor">
            <path d="M12 2L1 21h22L12 2zm1 15h-2v-2h2v2zm0-4h-2V9h2v4z"/>
          </svg>
          {summary.errors.length}
          {summary.errors.length === 1 ? $t('errors_one') : $t('errors_many')}
          <svg width="10" height="10" viewBox="0 0 10 10" fill="none" style="margin-left:2px">
            {#if showErrors}
              <path d="M1 7l4-4 4 4" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"/>
            {:else}
              <path d="M1 3l4 4 4-4" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"/>
            {/if}
          </svg>
        </button>
        {#if showErrors}
          <div class="log-box" style="height:100px;color:var(--warning)">
            {#each summary.errors as err}
              <div>{err}</div>
            {/each}
          </div>
        {/if}
      </div>
    {/if}

    <!-- Restart -->
    <button class="btn btn-outline" style="width:100%" on:click={() => dispatch('restart')}>
      {$t('btn_restart')}
    </button>

  </div>
</div>
