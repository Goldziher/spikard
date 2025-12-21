use std::env;
use std::process::Command;

fn main() {
    let php_config = env::var("PHP_CONFIG").unwrap_or_else(|_| "php-config".to_string());

    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-arg=-undefined");
        println!("cargo:rustc-link-arg=dynamic_lookup");

        if env::var("MACOSX_DEPLOYMENT_TARGET").is_err() {
            println!("cargo:rustc-env=MACOSX_DEPLOYMENT_TARGET=14.0");
        }
    }

    if let Ok(openssl_lib_dir) = env::var("OPENSSL_LIB_DIR") {
        println!("cargo:rustc-link-search=native={}", openssl_lib_dir);
    }
    println!("cargo:rerun-if-env-changed=OPENSSL_LIB_DIR");

    #[cfg(target_os = "macos")]
    {
        if env::var("OPENSSL_LIB_DIR").is_err()
            && let Ok(output) = Command::new("brew").arg("--prefix").arg("openssl@3").output()
            && output.status.success()
        {
            let openssl_prefix = String::from_utf8_lossy(&output.stdout).trim().to_string();
            let openssl_lib = format!("{}/lib", openssl_prefix);
            println!("cargo:rustc-link-search=native={}", openssl_lib);
        }
    }

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
                } else if let Some(name) = lib.strip_prefix("-l")
                    && essential_libs.contains(&name)
                {
                    println!("cargo:rustc-link-lib=dylib={}", name);
                }
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
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

    if env::var("SPIKARD_PHP_BUILD_NOTICE").is_ok()
        && let Ok(output) = Command::new(&php_config).arg("--version").output()
        && output.status.success()
    {
        let version = String::from_utf8_lossy(&output.stdout);
        println!("cargo:warning=Building for PHP {}", version.trim());
    }

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=PHP_CONFIG");
    println!("cargo:rerun-if-env-changed=SPIKARD_PHP_BUILD_NOTICE");
}
