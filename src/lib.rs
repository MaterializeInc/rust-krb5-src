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
//! custom build scripts. It is not intended for direct consumption, but
//! as a dependency for other crates that need libkrb5 available, like
//! [sasl2-sys].
//!
//! krb5-src is currently bundling libkrb5 [v1.18.1].
//!
//! [libkrb5]: https://web.mit.edu/kerberos/
//! [v1.18.1]: https://web.mit.edu/kerberos/krb5-1.18/
//! [sasl2-sys]: https://github.com/MaterializeInc/rust-sasl

/// The directory in which the vendored copy of libkrb5 has been installed.
pub const INSTALL_DIR: &str = concat!(env!("OUT_DIR"), "/install");
