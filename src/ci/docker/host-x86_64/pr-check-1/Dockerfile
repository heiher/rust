FROM ubuntu:22.04

ARG DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y --no-install-recommends \
  g++ \
  make \
  ninja-build \
  file \
  curl \
  ca-certificates \
  python3 \
  python3-pip \
  python3-pkg-resources \
  git \
  cmake \
  sudo \
  gdb \
  xz-utils \
  libssl-dev \
  pkg-config \
  mingw-w64 \
  && rm -rf /var/lib/apt/lists/*

ENV RUST_CONFIGURE_ARGS="--set rust.validate-mir-opts=3"

COPY scripts/nodejs.sh /scripts/
RUN sh /scripts/nodejs.sh /node
ENV PATH="/node/bin:${PATH}"

COPY scripts/sccache.sh /scripts/
RUN sh /scripts/sccache.sh

COPY host-x86_64/pr-check-1/reuse-requirements.txt /tmp/
RUN pip3 install --no-deps --no-cache-dir --require-hashes -r /tmp/reuse-requirements.txt

COPY host-x86_64/pr-check-1/check-default-config-profiles.sh /scripts/
COPY host-x86_64/pr-check-1/validate-toolstate.sh /scripts/

# Check library crates on all tier 1 targets.
# We disable optimized compiler built-ins because that requires a C toolchain for the target.
# We also skip the x86_64-unknown-linux-gnu target as it is well-tested by other jobs.
ENV SCRIPT \
  python3 ../x.py check bootstrap && \
  /scripts/check-default-config-profiles.sh && \
  python3 ../x.py build src/tools/build-manifest && \
  python3 ../x.py check compiletest --set build.compiletest-use-stage0-libtest=true && \
  python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu && \
  python3 ../x.py check --set build.optimized-compiler-builtins=false core alloc std --target=aarch64-unknown-linux-gnu,i686-pc-windows-msvc,i686-unknown-linux-gnu,x86_64-apple-darwin,x86_64-pc-windows-gnu,x86_64-pc-windows-msvc && \
  /scripts/validate-toolstate.sh && \
  reuse --include-submodules lint && \
  python3 ../x.py test collect-license-metadata
