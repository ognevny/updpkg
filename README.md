# updpkg

a small tool for updating your PKGBUILD recipes

## synopsis

```
A small tool for updating PKGBUILD recipes

Usage: updpkg.exe [OPTIONS] [DIRECTORY]

Arguments:
  [DIRECTORY]  path to directory with PKGBUILD [default: .]

Options:
  -v, --ver <VERSION>       new version of package (tarball)
  -m, --make <FLAGS>        invoke `makepkg` with optional flags (like you are invoking it manually)
  -M, --make-mingw <FLAGS>  the same as `make`, but for `makepkg-mingw`
      --git <SHA>           specify commit SHA
  -r, --rm <FILES>...       removes files from directory and recipe
  -h, --help                Print help
  -V, --version             Print version
```

### usage example

update checksums for PKGBUILD in current directory

```console
$ updpkg
```

update checksums, version to 1.1.1 and invoke `makepkg-mingw -sc` for directory `mingw-w64-dummy`

```console
$ updpkg mingw-w64-dummy --ver '1.1.1' --make-mingw='-sc'
```

remove `dummy.patch` from recipe and directory, update checksums and invoke `makepkg -sc` for
directory `dummy`

```console
$ updpkg dummy --rm 'dummy.patch' --make='-sc'
```

update checksums and commit SHA in recipe for current directory

```console
$ updpkg --git='55932aad9ec31456a0ed8c3488173e8b78113652'
```
