use std::env;
use std::process::Command;

fn main() {
    let php_config = env::var("PHP_CONFIG").unwrap_or_else(|_| "php-config".to_string());

    // Set minimum macOS version to avoid linker mismatches
    // Object files built on newer macOS need compatible deployment target
    #[cfg(target_os = "macos")]
    {
        if env::var("MACOSX_DEPLOYMENT_TARGET").is_err() {
            println!("cargo:rustc-env=MACOSX_DEPLOYMENT_TARGET=13.0");
        }
    }

    // Add OpenSSL library path from environment (set by setup-openssl action in CI)
    if let Ok(openssl_lib_dir) = env::var("OPENSSL_LIB_DIR") {
        println!("cargo:rustc-link-search=native={}", openssl_lib_dir);
    }
    println!("cargo:rerun-if-env-changed=OPENSSL_LIB_DIR");

    // On macOS with Homebrew, auto-detect OpenSSL location
    #[cfg(target_os = "macos")]
    {
        if env::var("OPENSSL_LIB_DIR").is_err() {
            // Try to find Homebrew's OpenSSL installation
            if let Ok(output) = Command::new("brew").arg("--prefix").arg("openssl@3").output()
                && output.status.success()
            {
                let openssl_prefix = String::from_utf8_lossy(&output.stdout).trim().to_string();
                let openssl_lib = format!("{}/lib", openssl_prefix);
                println!("cargo:rustc-link-search=native={}", openssl_lib);
            }
        }
    }

    // Get PHP includes (for header validation, not linking)
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
    // On macOS, PHP extensions use -undefined dynamic_lookup (set in .cargo/config.toml)
    // to defer symbol resolution to runtime when the extension is loaded by PHP.
    // We do NOT explicitly link against the PHP library on macOS.
    // On other platforms, we use a whitelist approach to avoid linking optional/missing extensions.
    #[cfg(not(target_os = "macos"))]
    {
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
    }

    // On macOS, we rely on -undefined dynamic_lookup from .cargo/config.toml
    // PHP symbols will be resolved at runtime when the extension is loaded
    #[cfg(target_os = "macos")]
    {
        // Get ldflags for library search paths only (not -l flags)
        if let Ok(output) = Command::new(&php_config).arg("--ldflags").output()
            && output.status.success()
        {
            let ldflags = String::from_utf8_lossy(&output.stdout);
            for flag in ldflags.split_whitespace() {
                if let Some(path) = flag.strip_prefix("-L") {
                    println!("cargo:rustc-link-search=native={}", path);
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
