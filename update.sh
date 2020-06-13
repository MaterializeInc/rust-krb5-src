#!/usr/bin/env bash

set -euo pipefail

cd "$(dirname "$0")"

if [[ $# -ne 1 ]]; then
    echo "fatal: usage: $0 VERSION" >&2
    exit 1
fi

version=$1

set -x
for ext in tar.gz tar.gz.asc; do
    major=$(grep -oE '^[0-9]+\.[0-9]+' <<< "$version")
    curl -fsSL "https://web.mit.edu/kerberos/dist/krb5/$major/krb5-$version.$ext" > "krb5.$ext"
done

gpg --verify krb5.tar.gz.asc krb5.tar.gz

rm -rf krb5
mkdir -p krb5
tar --strip-components=1 -C krb5 -xf krb5.tar.gz
rm krb5.tar.gz krb5.tar.gz.asc

patch -p1 <<'EOF'
diff --git a/krb5/src/aclocal.m4 b/krb5/src/aclocal.m4
index 2394f7e33..6fe87705a 100644
--- a/krb5/src/aclocal.m4
+++ b/krb5/src/aclocal.m4
@@ -280,8 +280,6 @@ if test $krb5_cv_func_sigprocmask_use = yes; then
 fi
 ])dnl
 dnl
-AC_DEFUN(AC_PROG_ARCHIVE, [AC_CHECK_PROG(ARCHIVE, ar, ar cqv, false)])dnl
-AC_DEFUN(AC_PROG_ARCHIVE_ADD, [AC_CHECK_PROG(ARADD, ar, ar cruv, false)])dnl
 dnl
 dnl check for <dirent.h> -- CHECK_DIRENT
 dnl (may need to be more complex later)
@@ -1035,10 +1033,8 @@ AC_DEFUN(KRB5_BUILD_LIBRARY,
 [AC_REQUIRE([KRB5_LIB_AUX])dnl
 AC_REQUIRE([AC_PROG_LN_S])dnl
 AC_REQUIRE([AC_PROG_RANLIB])dnl
-AC_REQUIRE([AC_PROG_ARCHIVE])dnl
-AC_REQUIRE([AC_PROG_ARCHIVE_ADD])dnl
 AC_REQUIRE([AC_PROG_INSTALL])dnl
-AC_CHECK_PROG(AR, ar, ar, false)
+AC_CHECK_TOOL(AR, ar, false)
 if test "$AR" = "false"; then
   AC_MSG_ERROR([ar not found in PATH])
 fi
EOF
(cd krb5/src && autoreconf -vif)
