<script>
  import { createEventDispatcher } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { save as saveDialog } from '@tauri-apps/plugin-dialog';
  import { writeFile } from '@tauri-apps/plugin-fs';
  import { t, locale } from '../lib/i18n.js';
  import { shortPath } from '../lib/format.js';

  export let summary;

  const dispatch = createEventDispatcher();

  let showErrors = false;
  let savingCard = false;

  $: total = (summary?.photos_dated   ?? 0)
           + (summary?.videos_dated   ?? 0);

  // For the histogram bar heights — tallest bar = 100% of the track.
  $: maxYearCount = (summary?.years ?? []).reduce(
    (m, y) => (y.count > m ? y.count : m), 1
  );
  $: yearRange = (() => {
    const ys = summary?.years ?? [];
    if (ys.length === 0) return null;
    const first = ys[0].year;
    const last  = ys[ys.length - 1].year;
    return first === last ? `${first}` : `${first} – ${last}`;
  })();

  async function openOutputFolder() {
    if (summary?.output_path) {
      await invoke('open_folder', { path: summary.output_path });
    }
  }

  // ── Share card ──────────────────────────────────────────────────────────
  // Vertical 1080×1920 ("story" format, Instagram/TikTok/Snapchat).
  // Pure Canvas, no extra deps.
  function drawShareCard() {
    const W = 1080;
    const H = 1920;
    const canvas = document.createElement('canvas');
    canvas.width = W;
    canvas.height = H;
    const ctx = canvas.getContext('2d');

    const FONT =
      '-apple-system, BlinkMacSystemFont, "Segoe UI", system-ui, sans-serif';
    const C = {
      bg: '#FFEF5C',
      ink: '#1A1A2E',
      muted: '#7C7C9A',
      teal: '#00C9B1',
      tealDeep: '#00A896',
      pink: '#FF6B9D',
      surface: '#FFFDF5',
      border: '#EDE9C5',
    };

    // 1. Background
    ctx.fillStyle = C.bg;
    ctx.fillRect(0, 0, W, H);

    // 2. Decorative teal blob top-right
    ctx.globalAlpha = 0.14;
    ctx.fillStyle = C.teal;
    ctx.beginPath();
    ctx.arc(W + 60, 180, 420, 0, Math.PI * 2);
    ctx.fill();
    // 3. Decorative pink blob bottom-left
    ctx.fillStyle = C.pink;
    ctx.beginPath();
    ctx.arc(-60, H - 240, 360, 0, Math.PI * 2);
    ctx.fill();
    ctx.globalAlpha = 1;

    // 4. Wordmark + pulsing dot accent (centered top)
    ctx.fillStyle = C.teal;
    ctx.beginPath();
    ctx.arc(W / 2 - 200, 165, 12, 0, Math.PI * 2);
    ctx.fill();
    ctx.fillStyle = C.ink;
    ctx.font = `800 44px ${FONT}`;
    ctx.textAlign = 'left';
    ctx.textBaseline = 'middle';
    ctx.fillText('GetSnapBack', W / 2 - 175, 165);

    // 5. Big number, auto-shrink to fit
    const totalStr = total.toLocaleString(
      $locale === 'fr' ? 'fr-FR' : 'en-US'
    );
    let numSize = 320;
    ctx.font = `900 ${numSize}px ${FONT}`;
    while (ctx.measureText(totalStr).width > W - 160 && numSize > 110) {
      numSize -= 12;
      ctx.font = `900 ${numSize}px ${FONT}`;
    }
    ctx.fillStyle = C.teal;
    ctx.textAlign = 'center';
    ctx.textBaseline = 'alphabetic';
    ctx.fillText(totalStr, W / 2, 560);

    // 6. Sparkle next to the number (decorative)
    drawStar(ctx, W / 2 + 290, 380, 18, C.pink);
    drawStar(ctx, W / 2 - 320, 340, 14, C.ink, 0.5);
    drawStar(ctx, W / 2 + 250, 580, 11, C.teal, 0.6);

    // 7. Caption (two lines max). Auto-wrap on space.
    ctx.fillStyle = C.ink;
    ctx.font = `800 64px ${FONT}`;
    ctx.textAlign = 'center';
    drawWrapped(ctx, $t('share_card_caption'), W / 2, 700, W - 160, 76);

    // 8. Year range pill
    if (yearRange) {
      const txt = `${$t('share_card_range_prefix')} ${yearRange}`;
      ctx.font = `700 32px ${FONT}`;
      const tw = ctx.measureText(txt).width;
      const pillW = tw + 56;
      const pillH = 62;
      const px = W / 2 - pillW / 2;
      const py = 820;
      ctx.fillStyle = C.ink;
      roundRect(ctx, px, py, pillW, pillH, pillH / 2);
      ctx.fill();
      ctx.fillStyle = C.bg;
      ctx.textBaseline = 'middle';
      ctx.fillText(txt, W / 2, py + pillH / 2 + 1);
    }

    // 9. Histogram (cream panel, gradient teal bars, year labels)
    const years = summary?.years ?? [];
    if (years.length > 0) {
      drawHistogram(ctx, years, 80, 970, W - 160, 660, C, FONT);
    }

    // 10. URL bottom
    ctx.fillStyle = C.ink;
    ctx.font = `700 32px ${FONT}`;
    ctx.textAlign = 'center';
    ctx.textBaseline = 'alphabetic';
    ctx.fillText('getsnapback.vercel.app', W / 2, H - 80);

    return canvas;
  }

  function drawHistogram(ctx, years, x, y, w, h, C, FONT) {
    // Panel
    roundRect(ctx, x, y, w, h, 28);
    ctx.fillStyle = C.surface;
    ctx.fill();
    ctx.strokeStyle = C.border;
    ctx.lineWidth = 3;
    ctx.stroke();

    // Geometry
    const padX = 56;
    const padTop = 56;
    const padBot = 78; // room for year labels
    const innerW = w - padX * 2;
    const innerH = h - padTop - padBot;

    const n = years.length;
    const gap = Math.min(20, Math.max(6, innerW / (n * 6)));
    const barW = (innerW - gap * (n - 1)) / n;
    const maxCount = years.reduce(
      (m, yr) => (yr.count > m ? yr.count : m),
      1
    );

    // Bars
    for (let i = 0; i < n; i++) {
      const yr = years[i];
      const ratio = yr.count / maxCount;
      const barH = Math.max(14, ratio * innerH);
      const bx = x + padX + i * (barW + gap);
      const by = y + padTop + innerH - barH;

      const grad = ctx.createLinearGradient(0, by, 0, by + barH);
      grad.addColorStop(0, C.teal);
      grad.addColorStop(1, C.tealDeep);
      ctx.fillStyle = grad;
      roundRect(ctx, bx, by, barW, barH, Math.min(10, barW / 3));
      ctx.fill();
    }

    // Year labels (every bar if there's room; otherwise every other)
    ctx.fillStyle = C.ink;
    ctx.textAlign = 'center';
    ctx.textBaseline = 'top';
    const labelFontSize = Math.max(18, Math.min(28, barW * 0.45));
    ctx.font = `700 ${labelFontSize}px ${FONT}`;
    const labelEvery = barW < 50 ? 2 : 1;
    for (let i = 0; i < n; i++) {
      if (i % labelEvery !== 0 && i !== n - 1) continue;
      const yr = years[i];
      const bx = x + padX + i * (barW + gap);
      ctx.globalAlpha = 0.85;
      ctx.fillText(String(yr.year), bx + barW / 2, y + h - padBot + 20);
      ctx.globalAlpha = 1;
    }
  }

  function drawWrapped(ctx, text, cx, y, maxW, lineH) {
    const words = text.split(' ');
    const lines = [];
    let line = '';
    for (const w of words) {
      const candidate = line ? `${line} ${w}` : w;
      if (ctx.measureText(candidate).width > maxW && line) {
        lines.push(line);
        line = w;
      } else {
        line = candidate;
      }
    }
    if (line) lines.push(line);
    ctx.textBaseline = 'alphabetic';
    for (let i = 0; i < lines.length; i++) {
      ctx.fillText(lines[i], cx, y + i * lineH);
    }
  }

  function roundRect(ctx, x, y, w, h, r) {
    const rr = Math.min(r, w / 2, h / 2);
    ctx.beginPath();
    ctx.moveTo(x + rr, y);
    ctx.lineTo(x + w - rr, y);
    ctx.quadraticCurveTo(x + w, y, x + w, y + rr);
    ctx.lineTo(x + w, y + h - rr);
    ctx.quadraticCurveTo(x + w, y + h, x + w - rr, y + h);
    ctx.lineTo(x + rr, y + h);
    ctx.quadraticCurveTo(x, y + h, x, y + h - rr);
    ctx.lineTo(x, y + rr);
    ctx.quadraticCurveTo(x, y, x + rr, y);
    ctx.closePath();
  }

  function drawStar(ctx, cx, cy, r, color, alpha = 1) {
    const spikes = 5;
    const inner = r * 0.42;
    ctx.save();
    ctx.globalAlpha = alpha;
    ctx.fillStyle = color;
    ctx.beginPath();
    for (let i = 0; i < spikes * 2; i++) {
      const ang = (Math.PI / spikes) * i - Math.PI / 2;
      const rad = i % 2 === 0 ? r : inner;
      const x = cx + Math.cos(ang) * rad;
      const y = cy + Math.sin(ang) * rad;
      if (i === 0) ctx.moveTo(x, y);
      else ctx.lineTo(x, y);
    }
    ctx.closePath();
    ctx.fill();
    ctx.restore();
  }

  async function saveShareCard() {
    if (savingCard) return;
    savingCard = true;
    try {
      const canvas = drawShareCard();
      const blob = await new Promise((resolve, reject) => {
        canvas.toBlob((b) => (b ? resolve(b) : reject(new Error('canvas.toBlob failed'))), 'image/png');
      });
      const bytes = new Uint8Array(await blob.arrayBuffer());
      const filePath = await saveDialog({
        defaultPath: `getsnapback-${total}-memories.png`,
        filters: [{ name: 'PNG', extensions: ['png'] }],
      });
      if (filePath) {
        await writeFile(filePath, bytes);
      }
    } catch (e) {
      console.error('Failed to save share card:', e);
    } finally {
      savingCard = false;
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

    <!-- Year histogram -->
    {#if summary?.years?.length > 0}
      <div class="histogram">
        <div class="hist-head">
          <span class="hist-label">{$t('hist_by_year')}</span>
          {#if yearRange}
            <span class="hist-range">{yearRange}</span>
          {/if}
        </div>
        <div class="hist-bars">
          {#each summary.years as y}
            <div class="hist-bar" title="{y.year} · {y.count}">
              <div class="hist-bar-track">
                <div
                  class="hist-bar-fill"
                  style="height:{Math.max(4, (y.count / maxYearCount) * 100)}%"
                ></div>
              </div>
              <div class="hist-bar-year">{y.year}</div>
            </div>
          {/each}
        </div>
      </div>
    {/if}

    <!-- Share card -->
    {#if total > 0}
      <button
        class="btn btn-outline share-btn"
        style="width:100%;margin-bottom:10px"
        on:click={saveShareCard}
        disabled={savingCard}
      >
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor"
             stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M4 16v3a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-3"/>
          <path d="M12 3v13"/>
          <path d="M7 8l5-5 5 5"/>
        </svg>
        {savingCard ? $t('share_saving') : $t('share_btn')}
      </button>
    {/if}

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
