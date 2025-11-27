#!/bin/bash
# Build script for PHP bindings on ARM Mac
# This sets the required environment variables for ext-php-rs bindgen to work

export LIBCLANG_PATH=/opt/homebrew/opt/llvm/lib
export LLVM_CONFIG_PATH=/opt/homebrew/opt/llvm/bin/llvm-config
export BINDGEN_EXTRA_CLANG_ARGS="-isysroot /Library/Developer/CommandLineTools/SDKs/MacOSX.sdk"

cargo build -p spikard-php --features extension-module "$@"
