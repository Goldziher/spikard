dnl Configuration for Rust-based PHP extension via ext-php-rs.
dnl Allows phpize to recognize this extension during source compilation (PIE fallback).

PHP_ARG_ENABLE([spikard_php],
  [whether to enable the spikard_php extension],
  [AS_HELP_STRING([--enable-spikard_php],
    [Enable spikard_php extension support])],
  [yes])

if test "$PHP_SPIKARD_PHP_ENABLED" = "yes"; then
  dnl Recognize the extension directory for phpize/make
  PHP_NEW_EXTENSION(spikard_php, [], $ext_shared)

  dnl Invoke cargo build to compile the Rust FFI library
  AC_CONFIG_COMMANDS([cargo-build], [
    if test -f "crates/spikard-php/Cargo.toml"; then
      cargo build --release --manifest-path crates/spikard-php/Cargo.toml || exit 1
      cargo_output_dir="crates/spikard-php/target/release"
      ext_soname="spikard_php"

      dnl Detect output filename based on platform
      if test -f "${cargo_output_dir}/libspikard-php_php.dylib"; then
        cargo_lib="${cargo_output_dir}/libspikard-php_php.dylib"
      elif test -f "${cargo_output_dir}/libspikard-php_php.so"; then
        cargo_lib="${cargo_output_dir}/libspikard-php_php.so"
      else
        AC_MSG_ERROR([cargo build succeeded but .so/.dylib not found])
      fi

      dnl Copy the compiled library to modules/ directory for phpize to install
      cp "${cargo_lib}" "modules/${ext_soname}.so" || exit 1
    else
      AC_MSG_ERROR([crates/spikard-php/Cargo.toml not found])
    fi
  ], [
    extension_name=spikard_php
  ])
fi
