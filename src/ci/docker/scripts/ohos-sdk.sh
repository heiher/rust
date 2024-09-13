#!/bin/sh
set -ex

mkdir -p /opt/ohos-sdk/native/

URL=https://hev.cc/sftp/ohos/clang-dev-linux-x86_64.tar.bz2
curl $URL | tar xj -C /opt/ohos-sdk/native/

URL=https://hev.cc/sftp/ohos/ohos-sysroot-dev.tar.bz2
curl $URL | tar xj -C /opt/ohos-sdk/native/

URL=https://static.rust-lang.org/dist/rust-nightly-x86_64-unknown-linux-gnu.tar.xz
curl $URL | tar xJ -C /tmp
/tmp/rust-nightly-x86_64-unknown-linux-gnu/install.sh --prefix=/usr
rm -rf /tmp/rust-nightly-x86_64-unknown-linux-gnu
