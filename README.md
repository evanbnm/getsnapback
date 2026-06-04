# GetSnapBack

**Restore the real dates on your Snapchat memories export.** Free, open
source, runs entirely on your computer.

When you download your Snapchat *Memories* archive, every photo and video
is stamped with the date of the **download**, not the date the moment
actually happened. Import that into Apple Photos or Google Photos and a
photo from 2017 ends up sorted next to your 2024 ones. GetSnapBack reads
the real date from each filename, writes it back into EXIF (photos) and
QuickTime (videos), composites the Snapchat overlays — captions, drawings,
stickers — back onto each source frame, removes duplicates, and gives you
a clean folder ready to import.

> 100% local. No upload, no account, no telemetry, no analytics. Your
> memories never leave your machine.

🌐 **Website**: <https://getsnapback.vercel.app>
📦 **Download**: <https://github.com/evanbnm/getsnapback/releases/latest>

---

## Does this fix your problem?

You've probably landed here because one of these matches what you're
experiencing:

- "My Snapchat memories are all dated 2024 / today instead of when I took them."
- "I exported my memories and Apple Photos / Google Photos sorts them at
  the wrong year."
- "Snapchat split my export into several ZIP files (over 2 GB) and I don't
  know how to handle them together."
- "The overlays — text I drew, captions, stickers — came out as separate
  PNG files. How do I get them baked back onto the photo?"
- "I want to keep my Snapchat memories long-term but the dates are wrong."

GetSnapBack solves all of the above, locally, in one drop-and-process step.

---

## What it does

- **Restores real dates** on every photo and video.
  - **Photos**: writes the correct timestamp into EXIF (`DateTimeOriginal`,
    `DateTimeDigitized`, `DateTime`).
  - **Videos**: writes into QuickTime atoms (`creation_time` and
    `com.apple.quicktime.creationdate`), readable by Apple Photos, Google
    Photos and Final Cut.
- **Composites overlays back onto the source.** Snapchat exports overlays
  (text, doodles, stickers) as separate transparent PNG files. GetSnapBack
  paints them back, pixel-perfect, onto each photo (and optionally each
  video).
- **Deduplicates** the output: identical-content files (copy-pasted then
  renamed) and Snapchat re-dated memories (we keep the oldest filename).
- **Merges multi-ZIP exports.** When Snapchat splits a large export into
  several ZIPs, drop them all at once — GetSnapBack reassembles them into
  one clean output.
- **Outputs a clean folder** ready to drag into Apple Photos or Google
  Photos.

---

## How it works

1. **Get your export from Snapchat.** Open the [Snapchat data download
   page](https://accounts.snapchat.com/v2/download-my-data), tick only
   *Export My Memories*, submit. Snapchat emails you a download link
   (usually within an hour, sometimes up to 24 h). If the archive is
   over 2 GB it's split into several ZIPs.
2. **Open GetSnapBack** and drop the ZIP(s) or the unzipped folder onto
   the window.
3. **Pick options** (composite photo overlays / video overlays). Photo
   overlays are fast and recommended; video overlays require re-encoding
   and use hardware acceleration when available (VideoToolbox on macOS,
   NVENC / QSV / AMF on Windows).
4. **Wait** — the progress bar walks you through the phases: analyze,
   date, overlay photo, overlay video, deduplicate.
5. **Import the output folder** into Apple Photos (*File → Import*) or
   Google Photos (drag onto photos.google.com). Dates appear correctly in
   the *Years* view.

---

## Privacy

- The app runs entirely on your computer.
- No HTTP request to any server. You can run it offline.
- No account, no telemetry, no analytics.
- Open source under [MIT](./LICENSE).

---

## Installation

Download the latest release for your system:
<https://github.com/evanbnm/getsnapback/releases/latest>

| System            | File                                |
|-------------------|-------------------------------------|
| macOS             | `GetSnapBack_*_aarch64.dmg`         |
| Windows           | `GetSnapBack_*_x64-setup.exe` or `.msi` |
| Linux             | `.AppImage`, `.deb`, `.rpm`         |

### First launch — security warnings

GetSnapBack isn't signed with a paid Apple Developer or Microsoft
certificate, so your system shows a warning the first time you open it.

**macOS** (one-time setup):
1. Drag GetSnapBack from the DMG into **Applications**.
2. Try to open the app — macOS shows a warning, close the dialog.
3. Open **System Settings → Privacy & Security**.
4. Scroll to the bottom, next to the GetSnapBack line click
   **Open Anyway** and confirm.
5. From that point on, the app opens normally on double-click.

**Windows**: on the "Windows protected your PC" screen,
**More info → Run anyway**.

**Linux (AppImage)**:
```bash
chmod +x GetSnapBack_*.AppImage
./GetSnapBack_*.AppImage
```

---

## FAQ

**Why are my Snapchat memories all dated 2024 instead of when I took
them?**
Snapchat stamps every exported file with the date of the export, not the
date the photo or video was taken. The real date is preserved only in the
filename prefix (`YYYY-MM-DD`). GetSnapBack reads that prefix and writes
it back into EXIF / QuickTime metadata.

**Will my photos and videos be uploaded anywhere?**
No. GetSnapBack runs entirely on your computer. There is no account, no
server, no telemetry.

**Snapchat split my export into several ZIPs — can the app handle that?**
Yes. Drop all the ZIPs onto the app together (or extract them into the
same folder) and they're merged into one output.

**Does it also fix the date on videos?**
Yes. Video dates go into the QuickTime atoms — both Apple Photos and
Google Photos read them.

**Is it really free?**
Yes — free and open source under MIT. If it helped you,
[buy me a coffee](https://www.buymeacoffee.com/evanbnm) — but there's no
paid version and no plan to add one.

---

## Development

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (stable)
- [Node.js](https://nodejs.org/) (LTS)
- Tauri system dependencies: see the
  [official prerequisites](https://v2.tauri.app/start/prerequisites/).

### Run locally

```bash
npm install
npm run tauri dev
```

### Build production binaries

```bash
npm run tauri build
```

Artifacts are written to `src-tauri/target/release/bundle/`. The CI matrix
in `.github/workflows/build.yml` produces the official binaries for macOS
arm64, Windows x64 and Linux x64.

---

## Repository structure

```
.
├── src/                 # Svelte UI
├── src-tauri/           # Rust core (processing pipeline, Tauri commands)
├── website/             # Next.js marketing site
├── reference/
│   └── snapchat_to_photos.sh   # reference implementation (bash spec)
└── README.md
```

The processing pipeline lives in `src-tauri/src/processor/`:
- `scan.rs` — recognise Snapchat filenames
- `date.rs` — EXIF / QuickTime date writers
- `overlay_img.rs` — photo overlay compositor
- `overlay_vid.rs` — video overlay compositor (calls bundled ffmpeg)
- `video_encoder.rs` — runtime HW encoder probe (VideoToolbox / NVENC / QSV / AMF / libx264)
- `dedup.rs` — content + UUID deduplication
- `mod.rs` — orchestrates the full pipeline

---

## Known limits

- **Video overlay** requires re-encoding — slow on exports with many
  videos. Can be turned off in options.
- **Photos without a recoverable hour** are stamped at noon — the day is
  still correct so the chronological order is preserved.
- The app is **ad-hoc signed**: see *First launch* above for the
  one-time unblock.

---

## En français

GetSnapBack restaure les **vraies dates** de vos souvenirs Snapchat avant
de les importer dans Photos (Apple) ou Google Photos. Quand vous
téléchargez votre export *Memories*, chaque fichier est daté du jour du
téléchargement et non du moment où la photo ou la vidéo a vraiment été
prise. GetSnapBack lit la vraie date dans le nom de fichier, la réinscrit
dans l'EXIF (photos) et les atomes QuickTime (vidéos), réincruste les
overlays Snapchat (textes, dessins, stickers) sur chaque média d'origine,
supprime les doublons, et vous rend un dossier propre prêt à importer.

100 % local — aucun envoi réseau, aucun compte, aucune télémétrie.

---

## License

MIT — see [LICENSE](./LICENSE).
