# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog], and this project adheres to [Semantic
Versioning].

<!-- #release:next-header -->

## [Unreleased] <!-- #release:date -->

## [0.3.2+1.19.2] - 2022-01-16

* Support building on Windows ([#16]).

* Make the `openssl-sys` dependency optional unless the `openssl-vendored`
  feature is enabled.

## [0.3.1+1.19.2] - 2021-12-22

* Incorporate a patch to remove an errant linker flag that was breaking
  cross-compilation from macOS to Linux. See [krb5/krb5#1233] for details.

## [0.3.0+1.19.2] - 2021-11-28

* Upgrade to libkrb5 v1.19.2.

* Drop the `--disable-aesni` configuration flag as the option no longer causes
  static compilation to fail. See [#4] for details.

## [0.2.4+1.18.2] - 2020-07-08

* Disable native language support (NLS) by default, since it complicates linking
  on some platforms.

## [0.2.3+1.18.2] - 2020-06-13

* Fix detection of `ar` tool when cross compiling.

* Upgrade to libkrb5 v1.18.2.

## [0.2.2+1.18.1] - 2020-05-23

* Configure libkrb5 with `--disable-aesni`, which disables use of the AES-NI
  instruction set, as libkrb5's AES-NI support is currently broken when
  compiling statically. Follow [#4] for details.

## [0.2.1+1.18.1] - 2020-05-03

* Introduce the `binaries` Cargo feature which, when enabled, will build the
  binaries that ship with libkrb5 (kinit, kdestroy, etc.) and install into the
  output directory.

## [0.2.0+1.18.1] - 2020-04-28

* Permit cross-compilation by requiring that krb5-src be used as a normal
  dependency, not a build dependency. The `krb5_src::INSTALL_DIR` symbol has
  been removed. The install directory is instead exposed by Cargo in the
  `DEP_KRB5_SRC_ROOT` environment variable in build scripts for crates that
  depend on `krb5_src`.

  **This is a backwards-incompatible change.**

## [0.1.1+1.18.1] - 2020-04-27

* Explicitly request position-independent code (`-fPIC`) when building libkrb5.
  This is required to link with Rust code and is not the default on all
  platforms.

## 0.1.0+1.18.1 - 2020-04-27

Initial release.

<!-- #release:next-url -->
[Unreleased]: https://github.com/MaterializeInc/rust-krb5-src/compare/v0.3.2+1.19.2...HEAD
[0.3.2+1.19.2]: https://github.com/MaterializeInc/rust-krb5-src/compare/v0.3.1+1.19.2...v0.3.2+1.19.2
[0.3.1+1.19.2]: https://github.com/MaterializeInc/rust-krb5-src/compare/v0.3.0+1.19.2...v0.3.1+1.19.2
[0.3.0+1.19.2]: https://github.com/MaterializeInc/rust-krb5-src/compare/v0.2.4+1.18.2...v0.3.0+1.19.2
[0.2.4+1.18.2]: https://github.com/MaterializeInc/rust-krb5-src/compare/v0.2.3+1.18.2...v0.2.4+1.18.2
[0.2.3+1.18.2]: https://github.com/MaterializeInc/rust-krb5-src/compare/v0.2.2+1.18.1...v0.2.3+1.18.2
[0.2.2+1.18.1]: https://github.com/MaterializeInc/rust-krb5-src/compare/v0.2.1+1.18.1...v0.2.2+1.18.1
[0.2.1+1.18.1]: https://github.com/MaterializeInc/rust-krb5-src/compare/v0.2.0+1.18.1...v0.2.1+1.18.1
[0.2.0+1.18.1]: https://github.com/MaterializeInc/rust-krb5-src/compare/v0.1.1+1.18.1...v0.2.0+1.18.1
[0.1.1+1.18.1]: https://github.com/MaterializeInc/rust-krb5-src/compare/v0.1.0+1.18.1...v0.1.1+1.18.1

[Keep a Changelog]: https://keepachangelog.com/en/1.0.0/
[Semantic Versioning]: https://semver.org/spec/v2.0.0.html
[krb5/krb5#1233]: https://github.com/krb5/krb5/pull/1233

[#4]: https://github.com/MaterializeInc/rust-krb5-src/issues/4
[#16]: https://github.com/MaterializeInc/rust-krb5-src/issues/16
