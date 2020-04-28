# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog], and this project adheres to [Semantic
Versioning].

## [0.1.1+1.18.1] - 2020-04-27

* Explicitly request position-independent code (`-fPIC`) when building libkrb5.
  This is required to link with Rust code and is not the default on all
  platforms.

## 0.1.0+1.18.1 - 2020-04-27

Initial release.

[0.1.1]: https://github.com/MaterializeInc/rust-sasl/compare/0.1.0+1.18.1...0.1.1+1.18.1

[Keep a Changelog]: https://keepachangelog.com/en/1.0.0/
[Semantic Versioning]: https://semver.org/spec/v2.0.0.html
[crates-io-page]: https://crates.io/crates/krb5-src
