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
# https://github.com/krb5/krb5/pull/1233
patch -p1 <<'EOF'
--- a/krb5/src/aclocal.m4
+++ b/krb5/src/aclocal.m4
@@ -585,10 +585,6 @@ if test "$GCC" = yes ; then
       CFLAGS="$CFLAGS -fno-common"
       ;;
     esac
-    case "$LD $LDFLAGS" in
-    *-Wl,-search_paths_first*) ;;
-    *) LDFLAGS="${LDFLAGS} -Wl,-search_paths_first" ;;
-    esac
   fi
 else
   if test "`uname -s`" = AIX ; then
EOF
(cd krb5/src && autoreconf -vif)
rm -r krb5/doc krb5/src/autom4te.cache
