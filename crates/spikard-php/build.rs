use std::env;
use std::process::Command;

fn main() {
    let php_config = env::var("PHP_CONFIG").unwrap_or_else(|_| "php-config".to_string());

    // Add OpenSSL library path from environment (set by setup-openssl action in CI)
    if let Ok(openssl_lib_dir) = env::var("OPENSSL_LIB_DIR") {
        println!("cargo:rustc-link-search=native={}", openssl_lib_dir);
    }
    println!("cargo:rerun-if-env-changed=OPENSSL_LIB_DIR");

    // Get PHP includes
    if let Ok(output) = Command::new(&php_config).arg("--includes").output()
        && output.status.success()
    {
        let includes = String::from_utf8_lossy(&output.stdout);
        for include in includes.split_whitespace() {
            if let Some(path) = include.strip_prefix("-I") {
                println!("cargo:rustc-link-search=native={}", path);
            }
        }
    }

    // Get PHP libs
    // Use a whitelist approach for PHP libraries to avoid linking optional/missing extensions.
    // Different PHP builds include different optional extensions (libsodium, aspell, tidy, sybdb, etc)
    // that may not be available in all CI environments. We only link essential core libraries.
    // This ensures consistent builds across different PHP installations and platforms.
    let essential_libs = ["php", "m", "c", "pthread", "resolv", "xml2", "z", "ssl", "crypto"];

    if let Ok(output) = Command::new(&php_config).arg("--libs").output()
        && output.status.success()
    {
        let libs = String::from_utf8_lossy(&output.stdout);
        for lib in libs.split_whitespace() {
            if let Some(path) = lib.strip_prefix("-L") {
                println!("cargo:rustc-link-search=native={}", path);
            } else if let Some(name) = lib.strip_prefix("-l") {
                // Only link essential PHP core libraries (whitelist approach)
                if essential_libs.contains(&name) {
                    println!("cargo:rustc-link-lib=dylib={}", name);
                }
            }
        }
    }

    // Get PHP version for informational purposes
    if let Ok(output) = Command::new(&php_config).arg("--version").output()
        && output.status.success()
    {
        let version = String::from_utf8_lossy(&output.stdout);
        println!("cargo:warning=Building for PHP {}", version.trim());
    }

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=PHP_CONFIG");
}
