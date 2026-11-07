# Arch Linux Packaging for Tin-music

This directory contains the Arch Linux packaging files for Tin-music.

## Quick Installation (.pkg.tar.zst)

To install the pre-built pacman package:

```bash
sudo pacman -U target/release/bundle/arch/tin-music-1.4.0-1-x86_64.pkg.tar.zst
```

## Building the Arch Package

Run the helper script from the repository root:

```bash
./scripts/build-arch.sh
```

This packages the compiled release binary and desktop assets into `target/release/bundle/arch/tin-music-1.4.0-1-x86_64.pkg.tar.zst`.

## Building from Source (makepkg)

```bash
cd packaging/arch
makepkg -si
```
