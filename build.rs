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
#[cfg(unix)]
use std::ffi::OsString;
use std::fs;
use std::io::{stderr, Write};
#[cfg(unix)]
use std::path::Path;
use std::path::PathBuf;
use std::str;

use duct::cmd;

struct Metadata {
    host: String,
    target: String,
    build_dir: PathBuf,
    install_dir: PathBuf,
}

impl Metadata {
    fn is_cross_compiling(&self) -> bool {
        self.host != self.target
    }

    fn get_target_env(&self, name: &str) -> Option<String> {
        if self.is_cross_compiling() {
            env::var(format!("{}_{}", name, self.target.replace("-", "_"))).ok()
        } else {
            env::var(name).ok()
        }
    }
}

trait DuctExpressionExt: Sized {
    fn set_target_env_vars(self, metadata: &Metadata) -> Self;
}

impl DuctExpressionExt for duct::Expression {
    fn set_target_env_vars(mut self, metadata: &Metadata) -> Self {
        if let Some(target_cc) = metadata.get_target_env("CC") {
            self = self.env("CC", target_cc)
        }
        if let Some(target_ar) = metadata.get_target_env("AR") {
            self = self.env("AR", target_ar)
        }
        self
    }
}

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let metadata = Metadata {
        host: env::var("HOST").unwrap(),
        target: env::var("TARGET").unwrap(),
        build_dir: out_dir.join("build"),
        install_dir: out_dir.join("install"),
    };

    fs::create_dir_all(&metadata.build_dir).expect("failed to create build dir");
    fs::create_dir_all(&metadata.install_dir).expect("failed to create install dir");
    build(&metadata);

    println!("cargo:root={}", metadata.install_dir.display());
}

#[cfg(unix)]
fn build(metadata: &Metadata) {
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
            format!("--prefix={}", metadata.install_dir.display()),
            "--enable-static".into(),
            "--disable-shared".into(),
            #[cfg(feature = "nls")]
            "--enable-nls".into(),
            #[cfg(not(feature = "nls"))]
            "--disable-nls".into(),
            format!("CPPFLAGS={}", cppflags),
            format!("CFLAGS={}", cflags),
        ];

        // If we're cross-compiling, let configure know.
        if metadata.is_cross_compiling() {
            configure_args.push(format!("--build={}", metadata.host));
            // `--host` is the target platform in autotools parlance, not the
            // host platform.
            configure_args.push(format!("--host={}", metadata.target));
        }

        let configure_path = Path::new("krb5").join("src").join("configure");

        let configure = cmd(configure_path, &configure_args)
            .dir(&metadata.build_dir)
            .set_target_env_vars(metadata)
            .env_remove("CONFIG_SITE");
        let configure_output = configure.stderr_capture().unchecked().run().unwrap();
        let _ = stderr().write_all(&configure_output.stderr);
        if !configure_output.status.success() && metadata.is_cross_compiling() {
            if let Ok(s) = str::from_utf8(&configure_output.stderr) {
                let last_error = s.trim_end();
                if last_error.ends_with(
                    "Cannot test for constructor/destructor support when cross compiling",
                ) {
                    eprintln!("set krb5_cv_attr_constructor_destructor={{yes|no}} in the environment to skip this test");
                }
                if last_error.ends_with("Cannot test regcomp when cross compiling") {
                    eprintln!(
                        "set ac_cv_func_regcomp={{yes|no}} in the environment to skip this test"
                    );
                }
                if last_error.ends_with(
                    "Cannot test for printf positional argument support when cross compiling",
                ) {
                    eprintln!("set ac_cv_printf_positional={{yes|no}} in the environment to skip this test");
                }
            }
            panic!("configure failed");
        }
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
            .dir(&metadata.build_dir)
            .set_target_env_vars(metadata)
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
                    .dir(metadata.build_dir.join(dir))
                    .set_target_env_vars(metadata)
                    .env("MAKEFLAGS", &make_flags)
                    .run()
                    .unwrap_or_else(|_| panic!("make failed in {}", dir));
            }
        }
    }
}

#[cfg(windows)]
fn build(metadata: &Metadata) {
    if metadata.host != metadata.target {
        panic!("cross-compilation on a Windows host is not supported");
    }

    // The Windows build system doesn't seem to support out-of-tree builds, so
    // copy the source tree into the build directory since we're not allowed to
    // build in the checkout directly.
    let output = cmd!("robocopy", "krb5\\src", &metadata.build_dir, "/s", "/e")
        .unchecked()
        .run()
        .unwrap_or_else(|e| panic!("copying source tree failed: {}", e));
    // https://docs.microsoft.com/en-us/troubleshoot/windows-server/backup-and-storage/return-codes-used-robocopy-utility
    if !matches!(output.status.code(), Some(0..=7)) {
        panic!("copying source tree failed: {:?}", output);
    }

    let nmake = |args: &[&str]| {
        cmd("nmake", args)
            .dir(&metadata.build_dir)
            .env("KRB_INSTALL_DIR", &metadata.install_dir)
    };

    // Prepare Windows Makefile.
    nmake(&["-f", "Makefile.in", "prep-windows"])
        .run()
        .unwrap_or_else(|e| panic!("nmake prep failed: {}", e));

    // Build.
    nmake(&[])
        .run()
        .unwrap_or_else(|e| panic!("nmake build failed: {}", e));

    // Install.
    nmake(&["install"])
        .run()
        .unwrap_or_else(|e| panic!("nmake install failed: {}", e));
}
