//! Token bucket rate limiter for WASI components.
//!
//! Uses wall-clock time available through WASI clocks.

use std::sync::Mutex;

/// Token bucket rate limiter.
pub struct RateLimiter {
    inner: Mutex<TokenBucket>,
}

struct TokenBucket {
    tokens: f64,
    max_tokens: f64,
    refill_rate: f64, // tokens per second
    last_refill: u64, // unix timestamp in milliseconds
}

impl RateLimiter {
    /// Create a new rate limiter.
    #[must_use]
    pub fn new(max_tokens: f64, refill_rate: f64) -> Self {
        Self {
            inner: Mutex::new(TokenBucket {
                tokens: max_tokens,
                max_tokens,
                refill_rate,
                last_refill: current_time_ms(),
            }),
        }
    }

    /// Try to consume a token. Returns `true` if allowed.
    pub fn try_acquire(&self) -> bool {
        let mut bucket = self.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let now = current_time_ms();
        #[allow(clippy::cast_precision_loss)]
        let elapsed_secs = (now - bucket.last_refill) as f64 / 1000.0;

        bucket.tokens = (bucket.tokens + elapsed_secs * bucket.refill_rate).min(bucket.max_tokens);
        bucket.last_refill = now;

        if bucket.tokens >= 1.0 {
            bucket.tokens -= 1.0;
            true
        } else {
            false
        }
    }
}

fn current_time_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| {
            #[allow(clippy::cast_possible_truncation)]
            let ms = d.as_millis() as u64;
            ms
        })
        .unwrap_or(0)
}
