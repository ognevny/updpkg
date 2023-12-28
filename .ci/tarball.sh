#!/bin/sh

mkdir -p testing && touch testing/PKGBUILD

cat >> testing/PKGBUILD <<END

_realname=alacritty
pkgbase=mingw-w64-${_realname}
pkgname="${MINGW_PACKAGE_PREFIX}-${_realname}"
pkgver=0.12.3
pkgrel=3
pkgdesc="A cross-platform, OpenGL terminal emulator (mingw-w64)"
arch=('any')
mingw_arch=('mingw64' 'ucrt64' 'clang64' 'clangarm64')
url="https://alacritty.org"
msys2_repository_url="https://github.com/alacritty/alacritty"
license=('spdx:Apache-2.0 OR MIT')
makedepends=("${MINGW_PACKAGE_PREFIX}-rust"
             "${MINGW_PACKAGE_PREFIX}-cmake"
             "${MINGW_PACKAGE_PREFIX}-ncurses"
             "${MINGW_PACKAGE_PREFIX}-desktop-file-utils"
             'git')
depends=("${MINGW_PACKAGE_PREFIX}-freetype"
         "${MINGW_PACKAGE_PREFIX}-fontconfig")
checkdepends=("${MINGW_PACKAGE_PREFIX}-ttf-dejavu")
optdepends=("${MINGW_PACKAGE_PREFIX}-ncurses: for alacritty terminfo database")
source=("https://github.com/alacritty/alacritty/archive/v${pkgver}/${_realname}-${pkgver}.tar.gz")
validpgpkeys=('4DAA67A9EA8B91FCC15B699C85CDAE3C164BA7B4'  # Christian Dürr <contact@christianduerr.com>
              'A56EF308A9F1256C25ACA3807EA8F8B94622A6A9') # Kirill Chibisov <contact@kchibisov.com>
sha256sums=('7825639d971e561b2ea3cc41e30b57cde8e185a400fee001843bb634df6b28ab')
noextract=("${_realname}-${pkgver}.tar.gz")

prepare() {
  cd "${srcdir}"
  tar -xzf "${_realname}-${pkgver}.tar.gz"
  cd "${srcdir}/${_realname}-${pkgver}"

  local _target="${CARCH}-pc-windows-gnu"
  # https://github.com/msys2/MINGW-packages/pull/17827#issuecomment-1646226014
  # if [[ $MINGW_PACKAGE_PREFIX == *-clang-aarch64 ]]; then
    # _target="${CARCH}-pc-windows-gnullvm"
  # fi
  cargo fetch --locked --target "${_target}"
}

build() {
  cd "${srcdir}/${_realname}-${pkgver}"

  WINAPI_NO_BUNDLED_LIBRARIES=1 \
    "${MINGW_PREFIX}/bin/cargo.exe" build \
    --release \
    --locked
}

check() {
  cd "${srcdir}/${_realname}-${pkgver}"

  WINAPI_NO_BUNDLED_LIBRARIES=1 \
  "${MINGW_PREFIX}/bin/cargo.exe" test \
    --release \
    --locked
}

package() {
  cd "${srcdir}/${_realname}-${pkgver}"

  install -Dm755 "target/release/${_realname}.exe" "${pkgdir}${MINGW_PREFIX}/bin/${_realname}.exe"
}
END

target/debug/updpkg --verbose --directory testing --version '0.13.0' --make-mingw --flags '-c' ||
target/release/updpkg --verbose --directory testing --version '0.13.0' --make-mingw --flags '-c'
