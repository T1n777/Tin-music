#!/usr/bin/env bash
set -euo pipefail

# Build Arch Linux (.pkg.tar.zst) package for Tin-music
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BIN_PATH="${PROJECT_ROOT}/target/release/limusic-app"
OUTPUT_DIR="${PROJECT_ROOT}/target/release/bundle/arch"

if [ ! -f "${BIN_PATH}" ]; then
    echo "Release binary not found at ${BIN_PATH}. Building release first..."
    (cd "${PROJECT_ROOT}" && cargo tauri build --ignore-version-mismatches --no-sign --no-bundle)
fi

echo "Packaging Tin-music for Arch Linux..."
BUILD_DIR="$(mktemp -d /tmp/tin-music-arch-XXXXXX)"
trap 'rm -rf "${BUILD_DIR}"' EXIT

cp "${BIN_PATH}" "${BUILD_DIR}/limusic-app"
cp "${PROJECT_ROOT}/packaging/arch/tin-music.desktop" "${BUILD_DIR}/"
cp "${PROJECT_ROOT}/LICENSE" "${BUILD_DIR}/"
cp "${PROJECT_ROOT}/src-tauri/icons/32x32.png" "${BUILD_DIR}/icon-32.png"
cp "${PROJECT_ROOT}/src-tauri/icons/64x64.png" "${BUILD_DIR}/icon-64.png"
cp "${PROJECT_ROOT}/src-tauri/icons/128x128.png" "${BUILD_DIR}/icon-128.png"
cp "${PROJECT_ROOT}/src-tauri/icons/128x128@2x.png" "${BUILD_DIR}/icon-256.png"
cp "${PROJECT_ROOT}/src-tauri/icons/icon.png" "${BUILD_DIR}/icon-512.png"

cat <<'EOF' > "${BUILD_DIR}/PKGBUILD"
# Maintainer: Tin <https://github.com/T1n777/Tin-music>
pkgname=tin-music
pkgver=1.4.0
pkgrel=1
pkgdesc="A modern, lightweight desktop music player for Navidrome"
arch=('x86_64')
url="https://github.com/T1n777/Tin-music"
license=('GPL-3.0-only')
depends=('gtk3' 'webkit2gtk-4.1' 'mpv' 'glib2' 'openssl')
provides=('tin-music' 'limusic-app')
conflicts=('tin-music-bin' 'limusic-app')
options=('!strip')
source=(
    "limusic-app"
    "tin-music.desktop"
    "LICENSE"
    "icon-32.png"
    "icon-64.png"
    "icon-128.png"
    "icon-256.png"
    "icon-512.png"
)
sha256sums=(
    'SKIP'
    'SKIP'
    'SKIP'
    'SKIP'
    'SKIP'
    'SKIP'
    'SKIP'
    'SKIP'
)

package() {
    install -Dm755 "${srcdir}/limusic-app" "${pkgdir}/usr/bin/tin-music"
    ln -s "tin-music" "${pkgdir}/usr/bin/limusic-app"

    install -Dm644 "${srcdir}/tin-music.desktop" "${pkgdir}/usr/share/applications/tin-music.desktop"

    install -Dm644 "${srcdir}/icon-32.png" "${pkgdir}/usr/share/icons/hicolor/32x32/apps/tin-music.png"
    install -Dm644 "${srcdir}/icon-64.png" "${pkgdir}/usr/share/icons/hicolor/64x64/apps/tin-music.png"
    install -Dm644 "${srcdir}/icon-128.png" "${pkgdir}/usr/share/icons/hicolor/128x128/apps/tin-music.png"
    install -Dm644 "${srcdir}/icon-256.png" "${pkgdir}/usr/share/icons/hicolor/256x256@2/apps/tin-music.png"
    install -Dm644 "${srcdir}/icon-512.png" "${pkgdir}/usr/share/icons/hicolor/512x512/apps/tin-music.png"

    ln -s "tin-music.png" "${pkgdir}/usr/share/icons/hicolor/32x32/apps/limusic-app.png"
    ln -s "tin-music.png" "${pkgdir}/usr/share/icons/hicolor/64x64/apps/limusic-app.png"
    ln -s "tin-music.png" "${pkgdir}/usr/share/icons/hicolor/128x128/apps/limusic-app.png"
    ln -s "tin-music.png" "${pkgdir}/usr/share/icons/hicolor/256x256@2/apps/limusic-app.png"
    ln -s "tin-music.png" "${pkgdir}/usr/share/icons/hicolor/512x512/apps/limusic-app.png"

    install -Dm644 "${srcdir}/LICENSE" "${pkgdir}/usr/share/licenses/tin-music/LICENSE"
}
EOF

(
    cd "${BUILD_DIR}"
    makepkg -f --nodeps
)

mkdir -p "${OUTPUT_DIR}"
cp "${BUILD_DIR}"/tin-music-*.pkg.tar.zst "${OUTPUT_DIR}/"

echo "Arch package generated successfully:"
ls -lh "${OUTPUT_DIR}"/tin-music-*.pkg.tar.zst
echo ""
echo "To install on Arch Linux / CachyOS / Manjaro / EndeavourOS:"
echo "  sudo pacman -U ${OUTPUT_DIR}/tin-music-1.4.0-1-x86_64.pkg.tar.zst"
