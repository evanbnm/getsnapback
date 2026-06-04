# Sidecar binaries

Tauri v2 convention: `<name>-<target-triple>[.exe]`

The release CI fetches the right binary per platform automatically (see
`.github/workflows/build.yml`). The instructions below are only useful when
running `npm run tauri dev` locally on a fresh clone.

> **Important** — must be **statically linked**. A Homebrew or system binary
> that links to dynamic libs (libavcodec.dylib, libavformat.dylib, …) will
> work on your machine but break on every user's machine.

## ffmpeg (needed for video date rewrite and the video overlay option)

Sources are chosen for static linkage + matching the HW encoders our
`video_encoder.rs` probes for:

| Target                          | Source                          | HW encoders included     |
|---------------------------------|---------------------------------|--------------------------|
| `aarch64-apple-darwin`          | osxexperts.net (ffmpeg71arm)    | h264_videotoolbox        |
| `x86_64-apple-darwin`           | osxexperts.net (ffmpeg71intel)  | h264_videotoolbox        |
| `x86_64-pc-windows-msvc.exe`    | BtbN/FFmpeg-Builds (win64-gpl)  | h264_nvenc/qsv/amf       |
| `x86_64-unknown-linux-gnu`      | BtbN/FFmpeg-Builds (linux64-gpl)| h264_nvenc/qsv           |

### macOS arm64
```bash
curl -fsSL https://www.osxexperts.net/ffmpeg71arm.zip -o /tmp/ff.zip
unzip -o /tmp/ff.zip -d /tmp/ff && \
  cp /tmp/ff/ffmpeg src-tauri/binaries/ffmpeg-aarch64-apple-darwin && \
  chmod +x src-tauri/binaries/ffmpeg-aarch64-apple-darwin
```

### macOS Intel
```bash
curl -fsSL https://www.osxexperts.net/ffmpeg71intel.zip -o /tmp/ff.zip
unzip -o /tmp/ff.zip -d /tmp/ff && \
  cp /tmp/ff/ffmpeg src-tauri/binaries/ffmpeg-x86_64-apple-darwin && \
  chmod +x src-tauri/binaries/ffmpeg-x86_64-apple-darwin
```

### Windows x64
Download `ffmpeg-master-latest-win64-gpl.zip` from
https://github.com/BtbN/FFmpeg-Builds/releases/latest then
```
mv bin/ffmpeg.exe src-tauri/binaries/ffmpeg-x86_64-pc-windows-msvc.exe
```

### Linux x64
```bash
asset=$(curl -sL https://api.github.com/repos/BtbN/FFmpeg-Builds/releases/latest \
  | grep -oE 'https://[^"]+linux64-gpl\.tar\.xz' | grep -v shared | head -1)
curl -fsSL "$asset" -o /tmp/ff.tar.xz
mkdir -p /tmp/ff && tar -xJf /tmp/ff.tar.xz -C /tmp/ff --strip-components=1
cp /tmp/ff/bin/ffmpeg src-tauri/binaries/ffmpeg-x86_64-unknown-linux-gnu
chmod +x src-tauri/binaries/ffmpeg-x86_64-unknown-linux-gnu
```

### Verifying a binary is portable
On macOS:
```bash
otool -L src-tauri/binaries/ffmpeg-<triple>
```
Output should only reference `/System/Library/*` and `/usr/lib/*`. Any
`/opt/homebrew/*` or `/usr/local/*` dependency means the binary is not static
and will not work on other machines.
