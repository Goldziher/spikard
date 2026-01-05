//! Debug logging utilities for spikard-http
//!
//! This module provides debug logging that can be enabled via:
//! - Building in debug mode (`cfg(debug_assertions)`)
//! - Setting `SPIKARD_DEBUG=1` environment variable

use std::sync::atomic::{AtomicBool, Ordering};

static DEBUG_ENABLED: AtomicBool = AtomicBool::new(false);

/// Initialize debug logging based on environment and build mode
pub fn init() {
    let enabled = cfg!(debug_assertions) || std::env::var("SPIKARD_DEBUG").is_ok() || std::env::var("DEBUG").is_ok();

    eprintln!(
        "[spikard-http::debug] init() called, cfg!(debug_assertions)={}, DEBUG={}, enabled={}",
        cfg!(debug_assertions),
        std::env::var("DEBUG").is_ok(),
        enabled
    );

    DEBUG_ENABLED.store(enabled, Ordering::Relaxed);

    if enabled {
        eprintln!("[spikard-http] Debug logging enabled");
    }
}

/// Check if debug logging is enabled
#[inline]
pub fn is_enabled() -> bool {
    DEBUG_ENABLED.load(Ordering::Relaxed)
}

/// Log a debug message if debugging is enabled
#[macro_export]
macro_rules! debug_log {
    ($($arg:tt)*) => {
        if $crate::debug::is_enabled() {
            eprintln!("[spikard-http] {}", format!($($arg)*));
        }
    };
}

/// Log a debug message with a specific module/component name
#[macro_export]
macro_rules! debug_log_module {
    ($module:expr, $($arg:tt)*) => {
        if $crate::debug::is_enabled() {
            eprintln!("[spikard-http::{}] {}", $module, format!($($arg)*));
        }
    };
}

/// Log a debug value with pretty-printing
#[macro_export]
macro_rules! debug_log_value {
    ($name:expr, $value:expr) => {
        if $crate::debug::is_enabled() {
            eprintln!("[spikard-http] {} = {:?}", $name, $value);
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;
    use std::sync::atomic::Ordering;

    static FLAG_LOCK: Mutex<()> = Mutex::new(());

    struct DebugFlagGuard {
        previous_flag: bool,
        previous_env: Option<String>,
    }

    impl Drop for DebugFlagGuard {
        fn drop(&mut self) {
            DEBUG_ENABLED.store(self.previous_flag, Ordering::Relaxed);
            if let Some(prev) = &self.previous_env {
                unsafe { std::env::set_var("SPIKARD_DEBUG", prev) };
            } else {
                unsafe { std::env::remove_var("SPIKARD_DEBUG") };
            }
        }
    }

    #[test]
    fn init_sets_debug_enabled_in_tests() {
        let _lock = FLAG_LOCK.lock().unwrap();
        let previous = DEBUG_ENABLED.load(Ordering::Relaxed);
        let previous_env = std::env::var("SPIKARD_DEBUG").ok();
        let _guard = DebugFlagGuard {
            previous_flag: previous,
            previous_env,
        };

        unsafe { std::env::set_var("SPIKARD_DEBUG", "1") };

        init();
        assert!(is_enabled(), "init should enable debug when SPIKARD_DEBUG is set");
    }

    #[test]
    fn macros_follow_debug_flag() {
        let _lock = FLAG_LOCK.lock().unwrap();
        let previous = DEBUG_ENABLED.load(Ordering::Relaxed);
        let previous_env = std::env::var("SPIKARD_DEBUG").ok();
        let _guard = DebugFlagGuard {
            previous_flag: previous,
            previous_env,
        };

        DEBUG_ENABLED.store(false, Ordering::Relaxed);
        debug_log!("disabled branch");
        debug_log_module!("core", "disabled");
        debug_log_value!("counter", 0_u8);
        assert!(!is_enabled());

        DEBUG_ENABLED.store(true, Ordering::Relaxed);
        debug_log!("enabled branch {}", 1);
        debug_log_module!("core", "enabled");
        debug_log_value!("counter", 2_i32);
        assert!(is_enabled());
    }
}
