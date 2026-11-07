<div align="center">

# Tin-music

**A modern, fast, and lightweight native desktop client for Navidrome (Subsonic).**  
*Built with Rust + Tauri 2 + Svelte 5. Zero Electron, zero web bloat.*

<p align="center">
  <a href="https://github.com/T1n777/Tin-music/releases/latest"><img alt="GitHub Release" src="https://img.shields.io/github/v/release/T1n777/Tin-music?display_name=release&style=for-the-badge&color=e50914"></a>
  <img alt="License" src="https://img.shields.io/github/license/T1n777/Tin-music?style=for-the-badge&color=1881cc">
  <img alt="Arch Linux" src="https://img.shields.io/badge/Arch_Linux-1793D1?style=for-the-badge&logo=arch-linux&logoColor=white">
  <img alt="Linux" src="https://img.shields.io/badge/Linux-FCC624?style=for-the-badge&logo=linux&logoColor=black">
  <img alt="Windows" src="https://img.shields.io/badge/Windows-0078D6?style=for-the-badge&logoColor=white">
  <img alt="macOS" src="https://img.shields.io/badge/macOS-000000?style=for-the-badge&logo=apple&logoColor=white">
  <img alt="Tauri 2" src="https://img.shields.io/badge/Tauri_2-24C8D8?style=for-the-badge&logo=tauri&logoColor=white">
  <img alt="Rust" src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white">
</p>

**Tin-music** connects directly to your self-hosted [Navidrome](https://www.navidrome.org/) (or any Subsonic-compatible) music server to deliver a snappy, native desktop music experience. Powered by an optimized Rust backend and `libmpv`, it offers bit-perfect playback, offline downloads, real-time audio controls, and scrobbling without the overhead of heavy browser runtimes.

</div>

---

## Highlights

- ⚡ **Native Performance**: Built on Tauri 2 and Rust with a reactive Svelte 5 frontend — launches instantly with minimal CPU and memory footprint.
- 📡 **Full Subsonic & Navidrome API**: Browse your complete library — Artists, Albums, Songs, Playlists, and Starred tracks.
- 🎚️ **Transcoding Selector**: Switch streaming quality on the fly in Settings (Direct/FLAC lossless, 320 kbps, 256 kbps, or 128 kbps Data Saver).
- 💾 **Offline Downloads**:
  - Download individual tracks directly from the `⋯` menu.
  - Download full albums to `~/Music/Tin-music/<Artist> - <Album>/`.
  - Download entire playlists to `~/Music/Tin-music/Playlists/<Name>/`.
  - Bulk library download with automatic deduplication (skips already-downloaded tracks).
- 🔔 **Sleek Update System**:
  - Discreet update button in the titlebar next to your profile that lights up with an animated glow when an update is available.
  - One-time toast notification (no repeated nagging).
  - One-click update and relaunch.
- 🎧 **Audiophile Playback via `libmpv`**:
  - Gapless transitions and loudness normalization.
  - Real-time tempo and pitch controls with custom room reverb presets.
- 📜 **Synced Lyrics**: Embedded side panel with synchronized line-by-line and word-by-word scrolling (LRCLIB and more).
- 📊 **Scrobbling & Presence**:
  - **Last.fm**: Connect with one click from the title bar; plays scrobble automatically.
  - **ListenBrainz**: Native token authentication and real-time scrobbles.
  - **Discord Rich Presence**: Live album art, track details, and elapsed progress.
- 🖼️ **Mini Player & Theater Mode**: Keep music in a floating compact window or expand to an immersive fullscreen view.
- 📁 **Local Music Playback**: Play offline audio files directly from your local storage.
- 🎨 **Adaptive Theming**: Themes that adapt their accent palette dynamically to match the playing album art.

---

## Download & Installation

Visit the [GitHub Releases](https://github.com/T1n777/Tin-music/releases/latest) page to download pre-built binaries.

### Arch Linux / CachyOS / Manjaro / EndeavourOS

#### Option 1: Native Pacman Package (`.pkg.tar.zst`)
Download the `.pkg.tar.zst` package from Releases and install via `pacman`:

```bash
sudo pacman -U tin-music-1.4.0-1-x86_64.pkg.tar.zst
```

#### Option 2: From Source / Local Git Clone
```bash
git clone https://github.com/T1n777/Tin-music.git
cd Tin-music
./scripts/build-arch.sh
sudo pacman -U target/release/bundle/arch/tin-music-*.pkg.tar.zst
```

To update an existing installation from Git at any time:
```bash
./scripts/update.sh
```

#### Option 3: AUR (`tin-music-git`)
```bash
yay -S tin-music-git
```

---

### Debian / Ubuntu (`.deb`)

Download the `.deb` package and install via `apt`:

```bash
sudo apt install ./Tin-music_1.4.0-rc.4_amd64.deb
```

---

### Linux AppImage (Universal)

Download the `.AppImage` bundle, make it executable, and run:

```bash
chmod +x Tin-music_1.4.0-rc.4_amd64.AppImage
./Tin-music_1.4.0-rc.4_amd64.AppImage
```

---

## Connecting to Your Navidrome Server

1. Launch **Tin-music**.
2. Click **Connect** (or the server icon) in the titlebar.
3. Enter your Navidrome server URL (e.g. `https://music.example.com`), username, and password.
4. Click **Connect**. Your library will sync and populate immediately.

---

## Building from Source

### Dependencies

#### Arch Linux / CachyOS:
```bash
sudo pacman -S --needed base-devel git rust cargo npm mpv gtk3 webkit2gtk-4.1 openssl
```

#### Ubuntu / Debian:
```bash
sudo apt update
sudo apt install -y build-essential curl git libmpv-dev libgtk-3-dev libwebkit2gtk-4.1-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev
```

#### Fedora:
```bash
sudo dnf install mpv-libs-devel gtk3-devel webkit2gtk4.1-devel openssl-devel
```

### Build Instructions

```bash
# 1. Clone repository
git clone https://github.com/T1n777/Tin-music.git
cd Tin-music

# 2. Install UI dependencies
npm --prefix ui install

# 3. Build release binary and packages
cargo tauri build --ignore-version-mismatches --no-sign
```

---

## License

This project is licensed under the [GNU General Public License v3.0](LICENSE).
