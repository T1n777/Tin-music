# Arch Linux Packaging & Updates for Tin-music

This directory contains the Arch Linux packaging files for Tin-music.

## Quick Installation (.pkg.tar.zst)

To install the pre-built pacman package:

```bash
sudo pacman -U target/release/bundle/arch/tin-music-1.4.0-1-x86_64.pkg.tar.zst
```

## Updating When Installed from Git

If you cloned this repository and installed the package locally, run:

```bash
./scripts/update.sh
```

This single command fetches the latest commits from `main`, rebuilds the package, and updates your system installation cleanly via `pacman`.

## Making Updates Work via `yay -Syu` / `cachy-update` (AUR)

Packages installed locally with `pacman -U <file>` are registered by pacman as **local/foreign packages**. `pacman` and AUR helpers (`yay`, `paru`, `cachy-update`) do not poll arbitrary git remotes on their own unless the package is registered in an Arch repository or the **Arch User Repository (AUR)**.

### Publishing to AUR (Enables `yay -Syu` automatic updates)

Once published to the AUR as `tin-music-git`:

1. Users install it via `yay`:
   ```bash
   yay -S tin-music-git
   ```
2. Running standard system updates will automatically update Tin-music:
   ```bash
   yay -Syu --devel
   # or
   cachy-update
   ```
   `yay` detects new Git commits on your GitHub repository, pulls them, compiles the app, and installs the update.

To publish `tin-music-git` to the AUR:
```bash
# 1. Clone your empty AUR repo
git clone ssh://aur@aur.archlinux.org/tin-music-git.git /tmp/tin-music-git

# 2. Copy the PKGBUILD-git into it as PKGBUILD
cp packaging/arch/PKGBUILD-git /tmp/tin-music-git/PKGBUILD

# 3. Generate .SRCINFO
cd /tmp/tin-music-git
makepkg --printsrcinfo > .SRCINFO

# 4. Commit and push
git add PKGBUILD .SRCINFO
git commit -m "Initial commit for tin-music-git"
git push origin master
```
