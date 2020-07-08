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

use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};

use duct::cmd;

fn main() {
    let host = env::var("HOST").unwrap();
    let target = env::var("TARGET").unwrap();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let build_dir = out_dir.join("build");
    let install_dir = out_dir.join("install");
    fs::create_dir_all(&build_dir).expect("failed to create build dir");

    // Configure.
    {
        let mut cppflags = env::var("CPPFLAGS").ok().unwrap_or_else(String::new);
        let mut cflags = env::var("CFLAGS").ok().unwrap_or_else(String::new);

        // Some platforms require that we explicitly request
        // position-independent code in our static libraries.
        cflags += " -fPIC";

        // If OpenSSL has been vendored, point libkrb5 at the vendored headers.
        if let Ok(openssl_root) = env::var("DEP_OPENSSL_ROOT") {
            cppflags += &format!(" -I{}", Path::new(&openssl_root).join("include").display());
        }

        let mut configure_args = vec![
            format!("--prefix={}", install_dir.display()),
            "--enable-static".into(),
            "--disable-shared".into(),
            // AES-NI support appears to be broken (perhaps related to static
            // libraries?). See #4.
            "--disable-aesni".into(),
            #[cfg(feature = "nls")]
            "--enable-nls".into(),
            #[cfg(not(feature = "nls"))]
            "--disable-nls".into(),
            format!("CPPFLAGS={}", cppflags),
            format!("CFLAGS={}", cflags),
        ];

        // If we're cross-compiling, let configure know.
        if host != target {
            configure_args.push(format!("--host={}", target));
        }

        let configure_path = Path::new("krb5").join("src").join("configure");
        cmd(configure_path, &configure_args)
            .dir(&build_dir)
            .run()
            .expect("configure failed");
    }

    // Make.
    {
        let mut make_flags = OsString::new();
        let mut make_args = vec![];
        if let Ok(s) = env::var("NUM_JOBS") {
            match env::var_os("CARGO_MAKEFLAGS") {
                // Only do this on non-windows and non-bsd
                // On Windows, we could be invoking make instead of
                // mingw32-make which doesn't work with our jobserver
                // bsdmake also does not work with our job server
                Some(ref s)
                    if !(cfg!(windows)
                        || cfg!(target_os = "openbsd")
                        || cfg!(target_os = "netbsd")
                        || cfg!(target_os = "freebsd")
                        || cfg!(target_os = "bitrig")
                        || cfg!(target_os = "dragonflybsd")) =>
                {
                    make_flags = s.clone()
                }

                // This looks like `make`, let's hope it understands `-jN`.
                _ => make_args.push(format!("-j{}", s)),
            }
        }

        // Hack our way through building just the libraries, and not any of the
        // utility programs. This avoids a dependency on Yacc.
        cmd!("make", "install-mkdirs")
            .dir(&build_dir)
            .run()
            .expect("install-mkdirs failed");
        for dir in &[
            "util/support",
            "util/et",
            "util/profile",
            "util/verto",
            "include",
            "lib",
            #[cfg(feature = "binaries")]
            "plugins/kdb/db2",
            #[cfg(feature = "binaries")]
            "clients",
        ] {
            for target in &["all", "install"] {
                cmd!("make", target)
                    .dir(&build_dir.join(dir))
                    .env("MAKEFLAGS", &make_flags)
                    .run()
                    .unwrap_or_else(|_| panic!("make failed in {}", dir));
            }
        }
    }

    println!("cargo:root={}", install_dir.display());
}
