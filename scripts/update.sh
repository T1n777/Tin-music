#!/usr/bin/env bash
set -euo pipefail

# Pull latest commits, rebuild, and update Tin-music on Arch Linux / CachyOS
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "${PROJECT_ROOT}"

echo "==> Fetching latest changes from git..."
git pull origin main

echo "==> Rebuilding and packaging Arch package..."
./scripts/build-arch.sh

echo "==> Updating installed package..."
sudo pacman -U --noconfirm "${PROJECT_ROOT}/target/release/bundle/arch"/tin-music-*.pkg.tar.zst

echo "==> Update complete! Tin-music is now up to date."
