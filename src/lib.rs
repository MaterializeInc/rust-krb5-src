// Copyright Materialize, Inc. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License in the LICENSE file at the
// root of this repository, or online at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Build system integration for [libkrb5], MIT's Kerberos implementation.
//!
//! This crate builds a vendored copy of libkrb5 using Cargo's support for
//! custom build scripts. It is not intended for direct consumption, but as a
//! dependency for other crates that need libkrb5 available, like [sasl2-sys].
//!
//! krb5-src is currently bundling libkrb5 [v1.19.2].
//!
//! To use this crate, declare a `dependency` or `dev-dependency` on `krb5-src`.
//! Then, in the build script for your crate, the environment variable
//! `DEP_KRB5_SRC_ROOT` will point to the directory in which the bundled copy of
//! libkrb5 has been installed. You can build and link another C library against
//! this copy of libkrb5, or generate Rust bindings and link Rust code against
//! this copy of libkrb5.
//!
//! Note that you are responsible for instructing Cargo to link in the
//! components of libkrb5 that you depend upon. Here is an example build script
//! fragment.
//!
//! ```no_run
//! # use std::env;
//! # use std::path::PathBuf;
//! println!(
//!     "cargo:rustc-link-search=native={}",
//!     PathBuf::from(env::var("DEP_KRB5_SRC_ROOT").unwrap()).join("lib").display(),
//! );
//! println!("cargo:rustc-link-lib=static=gssapi_krb5");
//! println!("cargo:rustc-link-lib=static=krb5");
//! println!("cargo:rustc-link-lib=static=k5crypto");
//! println!("cargo:rustc-link-lib=static=com_err");
//! println!("cargo:rustc-link-lib=static=krb5support");
//! ```
//!
//! # Cargo features
//!
//! krb5-src can be configured with the following Cargo features:
//!
//! * **`binaries`** builds the binaries that come with libkrb5 (kinit,
//!   kdestroy, et al.) and installs them into `DEP_KRB5_SRC_ROOT/bin`.
//!
//! * **`nls`** enables native language support (i.e., localization). This
//!   feature corresponds to the `--enable-nls` configure flag.
//!
//!   On some platforms, when this feature is enabled, the application must
//!   additionally link against libintl.
//!
//! * **`openssl-vendored`** enables the `vendored` feature of the `openssl-sys`
//!   crate.
//!
//! Note that none of these features have any effect when compiling on Windows.
//!
//! # Platform support
//!
//! krb5-src is tested on recent versions of Ubuntu, macOS, and Windows. Patches
//! that improve support for other platforms are welcome.
//!
//! [libkrb5]: https://web.mit.edu/kerberos/
//! [v1.19.2]: https://web.mit.edu/kerberos/krb5-1.19/
//! [sasl2-sys]: https://github.com/MaterializeInc/rust-sasl
