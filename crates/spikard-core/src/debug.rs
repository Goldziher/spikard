//! Debug logging utilities for spikard-http
//!
//! This module provides debug logging that can be enabled via:
//! - Building in debug mode (cfg(debug_assertions))
//! - Setting SPIKARD_DEBUG=1 environment variable

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
    use std::sync::atomic::Ordering;

    struct DebugFlagGuard(bool);

    impl Drop for DebugFlagGuard {
        fn drop(&mut self) {
            DEBUG_ENABLED.store(self.0, Ordering::Relaxed);
        }
    }

    #[test]
    fn init_sets_debug_enabled_in_tests() {
        let previous = DEBUG_ENABLED.load(Ordering::Relaxed);
        let _guard = DebugFlagGuard(previous);

        DEBUG_ENABLED.store(false, Ordering::Relaxed);

        init();
        assert!(is_enabled(), "init should enable debug in debug/test builds");
    }

    #[test]
    fn macros_follow_debug_flag() {
        let previous = DEBUG_ENABLED.load(Ordering::Relaxed);
        let _guard = DebugFlagGuard(previous);

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
