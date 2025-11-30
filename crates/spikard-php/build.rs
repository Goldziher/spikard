use std::env;
use std::process::Command;

fn main() {
    let php_config = env::var("PHP_CONFIG").unwrap_or_else(|_| "php-config".to_string());

    // Get PHP includes
    if let Ok(output) = Command::new(&php_config).arg("--includes").output() {
        if output.status.success() {
            let includes = String::from_utf8_lossy(&output.stdout);
            for include in includes.split_whitespace() {
                if let Some(path) = include.strip_prefix("-I") {
                    println!("cargo:rustc-link-search=native={}", path);
                }
            }
        }
    }

    // Get PHP libs
    if let Ok(output) = Command::new(&php_config).arg("--libs").output() {
        if output.status.success() {
            let libs = String::from_utf8_lossy(&output.stdout);
            for lib in libs.split_whitespace() {
                if let Some(path) = lib.strip_prefix("-L") {
                    println!("cargo:rustc-link-search=native={}", path);
                } else if let Some(name) = lib.strip_prefix("-l") {
                    println!("cargo:rustc-link-lib=dylib={}", name);
                }
            }
        }
    }

    // Get PHP version for informational purposes
    if let Ok(output) = Command::new(&php_config).arg("--version").output() {
        if output.status.success() {
            let version = String::from_utf8_lossy(&output.stdout);
            println!("cargo:warning=Building for PHP {}", version.trim());
        }
    }

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=PHP_CONFIG");
}
