#!/bin/sh

mkdir -p testing && touch testing/PKGBUILD

cat >> testing/PKGBUILD <<END

_realname=alacritty
pkgbase=mingw-w64-alacritty
pkgname=mingw-w64-ucrt-x86_64-alacritty
pkgver=0.12.3
pkgrel=3
pkgdesc="A cross-platform, OpenGL terminal emulator (mingw-w64)"
arch=('any')
mingw_arch=('ucrt64')
url="https://alacritty.org"
license=('spdx:Apache-2.0 OR MIT')
makedepends=("mingw-w64-ucrt-x86_64-rust"
             "mingw-w64-ucrt-x86_64-cmake"
             "mingw-w64-ucrt-x86_64-ncurses"
             "mingw-w64-ucrt-x86_64-desktop-file-utils"
             'git')
depends=("mingw-w64-ucrt-x86_64-freetype" "mingw-w64-ucrt-x86_64-fontconfig")
checkdepends=("mingw-w64-ucrt-x86_64-ttf-dejavu")
source=("https://github.com/alacritty/alacritty/archive/v\${pkgver}/alacritty-\${pkgver}.tar.gz")
validpgpkeys=('4DAA67A9EA8B91FCC15B699C85CDAE3C164BA7B4'
              'A56EF308A9F1256C25ACA3807EA8F8B94622A6A9')
sha256sums=('7825639d971e561b2ea3cc41e30b57cde8e185a400fee001843bb634df6b28ab')
noextract=("alacritty-\${pkgver}.tar.gz")

prepare() {
  cd "\${srcdir}"
  tar -xzf "alacritty-\${pkgver}.tar.gz"
  cd "\${srcdir}/alacritty-\${pkgver}"

  cargo fetch --locked --target x86_64-pc-windows-gnu
}

build() {
  cd "\${srcdir}/alacritty-\${pkgver}"

  WINAPI_NO_BUNDLED_LIBRARIES=1 \
    cargo build --release --locked
}

check() {
  cd "\${srcdir}/alacritty-\${pkgver}"

  WINAPI_NO_BUNDLED_LIBRARIES=1 \
    cargo test --release --locked
}

package() {
  cd "\${srcdir}/alacritty-\${pkgver}"

  install -Dm755 "target/release/alacritty.exe" "\${pkgdir}/ucrt64/bin/alacritty.exe"
}
END

./target/debug/updpkg --directory testing --version '0.13.0' --make-mingw --flags='-c' ||
./target/release/updpkg --directory testing --version '0.13.0' --make-mingw --flags='-c'

rm -rf testing
