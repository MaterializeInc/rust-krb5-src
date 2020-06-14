# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog], and this project adheres to [Semantic
Versioning].

## [0.2.3+1.18.2] - 2020-06-13

* Fix detection of `ar` tool when cross compiling.

* Upgrade to libkrb5 v1.18.2.

## [0.2.2+1.18.1] - 2020-05-23

* Configure libkrb5 with `--disable-aesni`, which disables use of the AES-NI
  instruction set, as libkrb5's AES-NI support is currently broken when
  compiling statically. Follow [#1] for details.

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

[0.1.1+1.18.1]: https://github.com/MaterializeInc/rust-krb5-src/compare/v0.1.0+1.18.1...v0.1.1+1.18.1
[0.2.0+1.18.1]: https://github.com/MaterializeInc/rust-krb5-src/compare/v0.1.1+1.18.1...v0.2.0+1.18.1
[0.2.1+1.18.1]: https://github.com/MaterializeInc/rust-krb5-src/compare/v0.2.0+1.18.1...v0.2.1+1.18.1
[0.2.2+1.18.1]: https://github.com/MaterializeInc/rust-krb5-src/compare/v0.2.1+1.18.1...v0.2.2+1.18.1
[0.2.3+1.18.1]: https://github.com/MaterializeInc/rust-krb5-src/compare/v0.2.2+1.18.1...v0.2.3+1.18.2

[Keep a Changelog]: https://keepachangelog.com/en/1.0.0/
[Semantic Versioning]: https://semver.org/spec/v2.0.0.html
[crates-io-page]: https://crates.io/crates/krb5-src
