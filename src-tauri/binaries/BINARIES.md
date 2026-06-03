# Sidecar binaries

Place the platform-specific binaries here before building.
Tauri v2 convention: `<name>-<target-triple>[.exe]`

## exiftool (required)

For macOS, exiftool must be PAR-packed into a standalone binary (the script from
exiftool.org requires Perl, which may not be present on users' machines).

### macOS — build the standalone binary locally

```bash
# Install exiftool + PAR::Packer, then pack
brew install exiftool
curl -fsSL https://cpanmin.us | perl - App::cpanminus
~/perl5/bin/cpanm --notest PAR::Packer

EXIFTOOL_LIB="$(brew --prefix exiftool)/libexec/lib/perl5"
EXIFTOOL_BIN="$(brew --prefix exiftool)/libexec/bin/exiftool"
ARCH=$(rustc -vV | awk '/^host:/ { print $2 }')   # e.g. aarch64-apple-darwin

perl \
  -I ~/perl5/lib/perl5 \
  -I ~/perl5/lib/perl5/$(perl -MConfig -e 'print $Config{archname}') \
  ~/perl5/bin/pp \
  -I "$EXIFTOOL_LIB" \
  -o "binaries/exiftool-$ARCH" \
  "$EXIFTOOL_BIN"

chmod +x "binaries/exiftool-$ARCH"
```

### Windows — download pre-built standalone exe

```
curl -LO https://exiftool.org/exiftool-13.55_64.zip
unzip exiftool-13.55_64.zip
mv "exiftool(-k).exe" binaries/exiftool-x86_64-pc-windows-msvc.exe
```

### Linux — use system exiftool (Perl is available on all Linux distros)

```bash
sudo apt-get install -y libimage-exiftool-perl
cp "$(which exiftool)" binaries/exiftool-x86_64-unknown-linux-gnu
chmod +x binaries/exiftool-x86_64-unknown-linux-gnu
```

---

## ffmpeg (only needed for video overlay option)

Download a static LGPL build from https://ffmpeg.org/download.html
or https://github.com/eugeneware/ffmpeg-static :

| Platform | Rename to |
|---|---|
| macOS Apple Silicon | `ffmpeg-aarch64-apple-darwin` |
| macOS Intel | `ffmpeg-x86_64-apple-darwin` |
| Windows x64 | `ffmpeg-x86_64-pc-windows-msvc.exe` |
| Linux x64 | `ffmpeg-x86_64-unknown-linux-gnu` |
