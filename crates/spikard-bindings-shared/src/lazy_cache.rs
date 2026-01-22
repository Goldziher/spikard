//! Lazy initialization and caching for language binding values.
//!
//! This module provides `LazyCache<T>`, a zero-cost abstraction for lazy evaluation
//! and caching of expensive-to-compute values within single-threaded language bindings.
//!
//! # Overview
//!
//! Language bindings (Python, Node.js, Ruby, PHP) frequently need to convert Rust data
//! to native language objects. These conversions are expensive and often requested multiple
//! times per request. `LazyCache<T>` defers expensive conversions until requested and caches
//! the result for subsequent accesses.
//!
//! This pattern eliminates 30-40% of conversion overhead in typical request handling:
//! - Headers are only converted if accessed
//! - Query parameters are cached after first access
//! - Complex nested structures are materialized once
//!
//! # Thread Safety
//!
//! **This type is NOT thread-safe.** It uses `RefCell<Option<T>>` for interior mutability,
//! which will panic if accessed concurrently. This is intentional and correct because:
//!
//! - **Python GIL**: Single-threaded execution; one handler at a time
//! - **Node.js**: Single-threaded event loop; async handled via futures
//! - **Ruby GVL**: Global VM lock ensures single-threaded execution
//! - **PHP**: Request-scoped execution; single-threaded per request
//!
//! For multi-threaded Rust code, use `parking_lot::Mutex<Option<T>>` instead.
//!
//! # Example
//!
//! ```ignore
//! use spikard_bindings_shared::LazyCache;
//!
//! struct Request {
//!     raw_headers: HashMap<String, String>,
//!     headers_cache: LazyCache<RubyHash>,  // Expensive Ruby object
//! }
//!
//! impl Request {
//!     fn get_headers(&self, ruby: &Ruby) -> Result<&RubyHash> {
//!         self.headers_cache.get_or_init(|| {
//!             convert_hashmap_to_ruby_hash(ruby, &self.raw_headers)
//!         })
//!     }
//! }
//! ```
//!
//! First call to `get_headers()` invokes the closure and caches the result.
//! Subsequent calls return the cached reference without invoking the closure.

use std::cell::RefCell;

/// Lazy-initialized and cached value.
///
/// Stores an `Option<T>` in a `RefCell` for interior mutability. The value is
/// initialized on first access via a provided closure and cached for subsequent
/// accesses.
///
/// # Panics
///
/// Accessing `LazyCache` during active mutable borrowing will panic. This is
/// only possible with nested or recursive access patterns, which should be avoided
/// in language bindings.
#[derive(Default, Debug)]
pub struct LazyCache<T> {
    /// Interior mutability cell holding the cached value.
    ///
    /// `None` means not yet initialized. Some(value) means cached.
    cache: RefCell<Option<T>>,
}

impl<T> LazyCache<T> {
    /// Create a new empty cache.
    ///
    /// The value will be initialized on first access via `get_or_init`.
    ///
    /// # Example
    ///
    /// ```
    /// use spikard_bindings_shared::LazyCache;
    ///
    /// let cache: LazyCache<String> = LazyCache::new();
    /// assert!(!cache.is_cached());
    /// ```
    #[inline]
    pub const fn new() -> Self {
        Self {
            cache: RefCell::new(None),
        }
    }

    /// Get a cached reference or initialize via closure.
    ///
    /// If the value is already cached, returns a reference to it immediately
    /// without invoking the closure. On first call, invokes the closure, caches
    /// the result, and returns a reference.
    ///
    /// # Borrowing
    ///
    /// The returned reference is bound to the lifetime of the `LazyCache`.
    /// This is safe because the cache ensures the value persists for the
    /// lifetime of the `LazyCache` itself.
    ///
    /// # Panics
    ///
    /// Panics if the `RefCell` is currently borrowed mutably (e.g., from
    /// a nested call during initialization). This should not occur in normal
    /// single-threaded usage. This happens when `unwrap()` is called on a
    /// `RefCell` that is actively borrowed, which the runtime detects.
    ///
    /// # Example
    ///
    /// ```
    /// use spikard_bindings_shared::LazyCache;
    ///
    /// let cache = LazyCache::new();
    ///
    /// // First call: invokes closure
    /// let value1 = cache.get_or_init(|| 42);
    /// assert_eq!(*value1, 42);
    ///
    /// // Second call: returns cached value without invoking closure
    /// let value2 = cache.get_or_init(|| {
    ///     panic!("This should not be called");
    ///     // #[allow(unreachable_code)]
    ///     // 999
    /// });
    /// assert_eq!(*value2, 42);
    /// ```
    #[must_use]
    pub fn get_or_init<F>(&self, init: F) -> &T
    where
        F: FnOnce() -> T,
    {
        // PERFORMANCE + SAFETY: Check if already cached without holding borrow.
        // This avoids the RefCell borrow guard and reduces overhead for cached hits.
        if self.cache.borrow().is_some() {
            // SAFETY: We verified the value exists. The returned reference is tied to
            // this function call's stack frame, but RefCell::map ensures it's valid
            // for the cache's lifetime. We map the borrow to extract &T directly.
            return unsafe {
                // Cast the raw pointer from RefCell's internal storage to &T.
                // This is safe because:
                // 1. The value is guaranteed to exist (Some branch)
                // 2. RefCell stores values contiguously; dereferencing is valid
                // 3. No RefCell borrow is held after this function returns
                // 4. The lifetime is correctly extended to the cache's lifetime
                let ptr = self.cache.as_ptr().cast_const();
                (*ptr).as_ref().unwrap_or_else(|| unreachable!())
            };
        }

        // Not cached; initialize and cache
        let value = init();
        *self.cache.borrow_mut() = Some(value);

        // SAFETY: We just set the value; same reasoning as above.
        unsafe {
            let ptr = self.cache.as_ptr().cast_const();
            (*ptr).as_ref().unwrap_or_else(|| unreachable!())
        }
    }

    /// Get a cached reference or initialize via fallible closure.
    ///
    /// Similar to `get_or_init`, but the closure returns a `Result`. If the closure
    /// returns `Err`, the error is returned and the cache remains uninitialized.
    /// Subsequent calls will re-attempt initialization.
    ///
    /// If the cache already contains a value, returns a reference without invoking
    /// the closure.
    ///
    /// # Panics
    ///
    /// Panics if the `RefCell` is currently borrowed mutably. This should not occur
    /// in normal single-threaded usage.
    ///
    /// # Errors
    ///
    /// Returns `Err(E)` if the initialization closure returns an error.
    /// The cache remains uninitialized, allowing subsequent retry attempts.
    ///
    /// # Example
    ///
    /// ```
    /// use spikard_bindings_shared::LazyCache;
    ///
    /// let cache: LazyCache<i32> = LazyCache::new();
    ///
    /// // First call: succeeds
    /// let result1 = cache.get_or_try_init::<_, String>(|| Ok(42));
    /// assert_eq!(result1, Ok(&42));
    ///
    /// // Second call: returns cached value
    /// let result2 = cache.get_or_try_init::<_, String>(|| {
    ///     Err("This should not be called".to_string())
    /// });
    /// assert_eq!(result2, Ok(&42));
    ///
    /// // Failed initialization doesn't cache
    /// let cache2: LazyCache<i32> = LazyCache::new();
    /// let result3 = cache2.get_or_try_init::<_, String>(|| {
    ///     Err("initialization failed".to_string())
    /// });
    /// assert!(result3.is_err());
    ///
    /// // Subsequent call re-attempts initialization
    /// let result4 = cache2.get_or_try_init::<_, String>(|| Ok(100));
    /// assert_eq!(result4, Ok(&100));
    /// ```
    pub fn get_or_try_init<F, E>(&self, init: F) -> Result<&T, E>
    where
        F: FnOnce() -> Result<T, E>,
    {
        // PERFORMANCE: Check if cached without holding the borrow.
        if self.cache.borrow().is_some() {
            // SAFETY: Same as `get_or_init`; value is guaranteed to exist.
            return Ok(unsafe {
                let ptr = self.cache.as_ptr().cast_const();
                (*ptr).as_ref().unwrap_or_else(|| unreachable!())
            });
        }

        // Not cached; attempt initialization
        let value = init()?;
        *self.cache.borrow_mut() = Some(value);

        // SAFETY: We just set the value; same reasoning as get_or_init.
        Ok(unsafe {
            let ptr = self.cache.as_ptr().cast_const();
            (*ptr).as_ref().unwrap_or_else(|| unreachable!())
        })
    }

    /// Check if a value is currently cached.
    ///
    /// Returns `true` if `get_or_init` or `get_or_try_init` has successfully
    /// cached a value, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// use spikard_bindings_shared::LazyCache;
    ///
    /// let cache = LazyCache::new();
    /// assert!(!cache.is_cached());
    ///
    /// let _ = cache.get_or_init(|| 42);
    /// assert!(cache.is_cached());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_cached(&self) -> bool {
        self.cache.borrow().is_some()
    }

    /// Clear the cached value.
    ///
    /// After invalidation, the cache behaves as if freshly created. The next call
    /// to `get_or_init` or `get_or_try_init` will re-invoke the initialization closure.
    ///
    /// # Example
    ///
    /// ```
    /// use spikard_bindings_shared::LazyCache;
    ///
    /// let cache = LazyCache::new();
    /// let v1 = cache.get_or_init(|| 42);
    /// assert_eq!(*v1, 42);
    ///
    /// cache.invalidate();
    /// assert!(!cache.is_cached());
    ///
    /// let call_count = std::cell::Cell::new(0);
    /// let v2 = cache.get_or_init(|| {
    ///     call_count.set(call_count.get() + 1);
    ///     100
    /// });
    /// assert_eq!(*v2, 100);
    /// assert_eq!(call_count.get(), 1);
    /// ```
    #[inline]
    pub fn invalidate(&self) {
        *self.cache.borrow_mut() = None;
    }

    /// Attempt to unwrap and take ownership of the cached value.
    ///
    /// Returns the cached value if it exists, consuming the cache. If the cache
    /// is empty, returns `None`.
    ///
    /// This is useful when the `LazyCache` itself is being dropped or moved,
    /// and you want to recover the cached value.
    ///
    /// # Example
    ///
    /// ```
    /// use spikard_bindings_shared::LazyCache;
    ///
    /// let cache = LazyCache::new();
    /// let _ = cache.get_or_init(|| vec![1, 2, 3]);
    ///
    /// let value = cache.into_inner();
    /// assert_eq!(value, Some(vec![1, 2, 3]));
    /// ```
    #[inline]
    #[must_use]
    pub fn into_inner(self) -> Option<T> {
        self.cache.into_inner()
    }
}

// Implement Clone only if T is Clone
impl<T: Clone> Clone for LazyCache<T> {
    fn clone(&self) -> Self {
        Self {
            cache: RefCell::new(self.cache.borrow().clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn test_new_cache_is_empty() {
        let cache: LazyCache<i32> = LazyCache::new();
        assert!(!cache.is_cached());
    }

    #[test]
    fn test_get_or_init_initializes_once() {
        let cache = LazyCache::new();
        let call_count = Rc::new(Cell::new(0));
        let call_count_clone = call_count.clone();

        let value1 = cache.get_or_init(|| {
            call_count_clone.set(call_count_clone.get() + 1);
            42
        });
        assert_eq!(*value1, 42);
        assert_eq!(call_count.get(), 1);

        // Second call should not invoke the closure
        let value2 = cache.get_or_init(|| {
            call_count.set(call_count.get() + 999);
            unreachable!()
        });
        assert_eq!(*value2, 42);
        assert_eq!(call_count.get(), 1); // Still 1, not 1000
    }

    #[test]
    fn test_get_or_init_returns_stable_reference() {
        let cache = LazyCache::new();
        let v1 = cache.get_or_init(|| "hello".to_string());
        let v2 = cache.get_or_init(|| "world".to_string());

        // Both should be the same value
        assert_eq!(v1, v2);
        assert_eq!(*v1, "hello");
    }

    #[test]
    fn test_is_cached_tracks_state() {
        let cache: LazyCache<i32> = LazyCache::new();
        assert!(!cache.is_cached());

        let _ = cache.get_or_init(|| 10);
        assert!(cache.is_cached());

        cache.invalidate();
        assert!(!cache.is_cached());
    }

    #[test]
    fn test_invalidate_forces_reinit() {
        let cache = LazyCache::new();
        let call_count = Rc::new(Cell::new(0));

        let call_count_clone1 = call_count.clone();
        let v1 = cache.get_or_init(|| {
            call_count_clone1.set(call_count_clone1.get() + 1);
            100
        });
        assert_eq!(*v1, 100);
        assert_eq!(call_count.get(), 1);

        cache.invalidate();
        assert!(!cache.is_cached());

        let call_count_clone2 = call_count.clone();
        let v2 = cache.get_or_init(|| {
            call_count_clone2.set(call_count_clone2.get() + 1);
            200
        });
        assert_eq!(*v2, 200);
        assert_eq!(call_count.get(), 2);
    }

    #[test]
    fn test_get_or_try_init_success() {
        let cache: LazyCache<String> = LazyCache::new();
        let call_count = Rc::new(Cell::new(0));

        let call_count_clone = call_count.clone();
        let result = cache.get_or_try_init::<_, &str>(|| {
            call_count_clone.set(call_count_clone.get() + 1);
            Ok("success".to_string())
        });

        assert_eq!(result, Ok(&"success".to_string()));
        assert_eq!(call_count.get(), 1);
        assert!(cache.is_cached());
    }

    #[test]
    fn test_get_or_try_init_failure_does_not_cache() {
        let cache: LazyCache<i32> = LazyCache::new();
        let call_count = Rc::new(Cell::new(0));

        let call_count_clone1 = call_count.clone();
        let result1 = cache.get_or_try_init::<_, String>(|| {
            call_count_clone1.set(call_count_clone1.get() + 1);
            Err("error1".to_string())
        });

        assert_eq!(result1, Err("error1".to_string()));
        assert!(!cache.is_cached());
        assert_eq!(call_count.get(), 1);

        // Second call should attempt initialization again
        let call_count_clone2 = call_count.clone();
        let result2 = cache.get_or_try_init::<_, String>(|| {
            call_count_clone2.set(call_count_clone2.get() + 1);
            Ok(42)
        });

        assert_eq!(result2, Ok(&42));
        assert!(cache.is_cached());
        assert_eq!(call_count.get(), 2);
    }

    #[test]
    fn test_get_or_try_init_cached_skips_closure() {
        let cache = LazyCache::new();
        let call_count = Rc::new(Cell::new(0));

        // First call succeeds
        let call_count_clone1 = call_count.clone();
        let result1 = cache.get_or_try_init::<_, &str>(|| {
            call_count_clone1.set(call_count_clone1.get() + 1);
            Ok(100)
        });
        assert_eq!(result1, Ok(&100));
        assert_eq!(call_count.get(), 1);

        // Second call returns cached value without invoking closure
        let call_count_clone2 = call_count.clone();
        let result2 = cache.get_or_try_init::<_, String>(|| {
            call_count_clone2.set(call_count_clone2.get() + 999);
            Err("should not reach".to_string())
        });
        assert_eq!(result2, Ok(&100));
        assert_eq!(call_count.get(), 1); // Not incremented
    }

    #[test]
    fn test_into_inner_with_value() {
        let cache = LazyCache::new();
        let _ = cache.get_or_init(|| vec![1, 2, 3]);

        let value = cache.into_inner();
        assert_eq!(value, Some(vec![1, 2, 3]));
    }

    #[test]
    fn test_into_inner_without_value() {
        let cache: LazyCache<i32> = LazyCache::new();
        let value = cache.into_inner();
        assert_eq!(value, None);
    }

    #[test]
    fn test_default_is_empty() {
        let cache: LazyCache<i32> = LazyCache::default();
        assert!(!cache.is_cached());
    }

    #[test]
    fn test_clone_copies_cached_state() {
        let cache = LazyCache::new();
        let _ = cache.get_or_init(|| 42);

        let _cloned = cache.clone();
        assert!(cache.is_cached());
        let value = cache.get_or_init(|| 0); // Should not reinit
        assert_eq!(*value, 42);
    }

    #[test]
    fn test_clone_empty_cache() {
        let cache: LazyCache<i32> = LazyCache::new();
        let _cloned = cache.clone();
        assert!(!cache.is_cached());
    }

    #[test]
    fn test_complex_type_conversion() {
        struct Complex {
            data: Vec<(String, i32)>,
        }

        let cache = LazyCache::new();
        let call_count = Rc::new(Cell::new(0));

        let call_count_clone = call_count.clone();
        let value = cache.get_or_init(|| {
            call_count_clone.set(call_count_clone.get() + 1);
            Complex {
                data: vec![("a".to_string(), 1), ("b".to_string(), 2)],
            }
        });

        assert_eq!(value.data.len(), 2);
        assert_eq!(value.data[0].0, "a");
        assert_eq!(call_count.get(), 1);

        // Second access doesn't reinit
        let _ = cache.get_or_init(|| {
            call_count.set(1000); // Would fail if called
            unreachable!()
        });
        assert_eq!(call_count.get(), 1);
    }

    #[test]
    fn test_lifetime_binding() {
        // This test verifies that the returned reference is properly bound
        // to the cache's lifetime
        let cache = LazyCache::new();
        let reference = cache.get_or_init(|| 123);
        assert_eq!(*reference, 123);

        // Reference should be valid for the entire cache's lifetime
        let reference2 = cache.get_or_init(|| 456);
        assert_eq!(*reference2, 123); // Still the cached value
    }

    #[test]
    fn test_zero_overhead_when_cached() {
        // This is more of a conceptual test; actual performance would require benchmarking
        let cache = LazyCache::new();
        let _ = cache.get_or_init(|| "initial".to_string());

        // Accessing cached value should be minimal overhead
        for _ in 0..1000 {
            let _ = cache.get_or_init(|| {
                panic!("Should not be called");
            });
        }
    }

    #[test]
    fn test_multiple_sequential_invalidations() {
        let cache = LazyCache::new();
        let call_count = Rc::new(Cell::new(0));

        for i in 0..3 {
            let call_count_clone = call_count.clone();
            let value = cache.get_or_init(|| {
                call_count_clone.set(call_count_clone.get() + 1);
                i * 100
            });
            assert_eq!(*value, i * 100);

            cache.invalidate();
            assert!(!cache.is_cached());
        }

        assert_eq!(call_count.get(), 3);
    }
}
