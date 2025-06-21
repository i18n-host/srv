#!/usr/bin/env bash

export RUSTFLAGS='--cfg reqwest_unstable -C target-feature=+aes'

case $(uname -s) in
Linux*)
  # wait for fix https://github.com/dtolnay/linkme/issues/107#issuecomment-2577583543
  export RUSTFLAGS="$RUSTFLAGS -C linker=clang -C link-arg=-fuse-ld=/usr/bin/mold"
  ;;
Darwin*) ;;
CYGWIN* | MINGW32* | MSYS* | MINGW*) ;;
*) ;;
esac
