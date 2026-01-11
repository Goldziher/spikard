//! Lifecycle hook base implementations

use std::sync::Arc;

/// Lifecycle hook types supported across all bindings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LifecycleHookType {
    /// Called at the start of request processing
    OnRequest,
    /// Called before validation
    PreValidation,
    /// Called before handler execution
    PreHandler,
    /// Called after handler execution
    OnResponse,
    /// Called when an error occurs
    OnError,
}

/// Result type for lifecycle hooks
#[derive(Clone)]
pub enum HookResult {
    /// Continue with normal processing
    Continue,
    /// Short-circuit and return this response
    ShortCircuit(serde_json::Value),
}

/// Trait for implementing lifecycle hooks in language bindings
pub trait LifecycleHook: Send + Sync {
    /// Execute the lifecycle hook
    ///
    /// # Errors
    ///
    /// Returns an error if hook execution fails.
    fn execute(&self, context: serde_json::Value) -> Result<HookResult, String>;

    /// Get the hook type
    fn hook_type(&self) -> LifecycleHookType;
}

/// Base configuration for lifecycle hooks
pub struct LifecycleConfig {
    /// Registered hooks by type
    hooks: std::collections::HashMap<LifecycleHookType, Vec<Arc<dyn LifecycleHook>>>,
}

impl LifecycleConfig {
    /// Create a new lifecycle configuration
    #[must_use]
    pub fn new() -> Self {
        Self {
            hooks: std::collections::HashMap::new(),
        }
    }

    /// Register a lifecycle hook
    pub fn register(&mut self, hook: Arc<dyn LifecycleHook>) {
        self.hooks.entry(hook.hook_type()).or_default().push(hook);
    }

    /// Get hooks for a specific type
    #[must_use]
    pub fn get_hooks(&self, hook_type: LifecycleHookType) -> Vec<Arc<dyn LifecycleHook>> {
        self.hooks.get(&hook_type).cloned().unwrap_or_default()
    }
}

impl Default for LifecycleConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    struct TestHook {
        hook_type: LifecycleHookType,
        result: HookResult,
    }

    impl LifecycleHook for TestHook {
        fn execute(&self, _context: serde_json::Value) -> Result<HookResult, String> {
            Ok(self.result.clone())
        }

        fn hook_type(&self) -> LifecycleHookType {
            self.hook_type
        }
    }

    #[test]
    fn test_lifecycle_hook_type_equality() {
        assert_eq!(LifecycleHookType::OnRequest, LifecycleHookType::OnRequest);
        assert_ne!(LifecycleHookType::OnRequest, LifecycleHookType::OnResponse);
    }

    #[test]
    fn test_lifecycle_hook_type_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(LifecycleHookType::OnRequest);
        set.insert(LifecycleHookType::PreHandler);
        set.insert(LifecycleHookType::OnRequest);

        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_hook_result_continue() {
        let result = HookResult::Continue;
        match result {
            HookResult::Continue => {}
            HookResult::ShortCircuit(_) => panic!("Expected Continue"),
        }
    }

    #[test]
    fn test_hook_result_short_circuit() {
        let response = json!({ "status": "error" });
        let result = HookResult::ShortCircuit(response.clone());
        match result {
            HookResult::Continue => panic!("Expected ShortCircuit"),
            HookResult::ShortCircuit(r) => {
                assert_eq!(r, response);
            }
        }
    }

    #[test]
    fn test_lifecycle_config_new() {
        let config = LifecycleConfig::new();
        assert_eq!(config.get_hooks(LifecycleHookType::OnRequest).len(), 0);
    }

    #[test]
    fn test_lifecycle_config_default() {
        let config = LifecycleConfig::default();
        assert_eq!(config.get_hooks(LifecycleHookType::OnRequest).len(), 0);
    }

    #[test]
    fn test_register_single_hook() {
        let mut config = LifecycleConfig::new();
        let hook = Arc::new(TestHook {
            hook_type: LifecycleHookType::OnRequest,
            result: HookResult::Continue,
        });

        config.register(hook);

        let hooks = config.get_hooks(LifecycleHookType::OnRequest);
        assert_eq!(hooks.len(), 1);
    }

    #[test]
    fn test_register_multiple_hooks_same_type() {
        let mut config = LifecycleConfig::new();

        for i in 0..3 {
            let hook = Arc::new(TestHook {
                hook_type: LifecycleHookType::OnRequest,
                result: if i == 0 {
                    HookResult::Continue
                } else {
                    HookResult::ShortCircuit(json!({ "index": i }))
                },
            });
            config.register(hook);
        }

        let hooks = config.get_hooks(LifecycleHookType::OnRequest);
        assert_eq!(hooks.len(), 3);
    }

    #[test]
    fn test_register_hooks_different_types() {
        let mut config = LifecycleConfig::new();

        let hook_on_request = Arc::new(TestHook {
            hook_type: LifecycleHookType::OnRequest,
            result: HookResult::Continue,
        });

        let hook_on_error = Arc::new(TestHook {
            hook_type: LifecycleHookType::OnError,
            result: HookResult::Continue,
        });

        config.register(hook_on_request);
        config.register(hook_on_error);

        assert_eq!(config.get_hooks(LifecycleHookType::OnRequest).len(), 1);
        assert_eq!(config.get_hooks(LifecycleHookType::OnError).len(), 1);
        assert_eq!(config.get_hooks(LifecycleHookType::PreHandler).len(), 0);
    }

    #[test]
    fn test_get_hooks_empty() {
        let config = LifecycleConfig::new();
        let hooks = config.get_hooks(LifecycleHookType::PreValidation);
        assert!(hooks.is_empty());
    }

    #[test]
    fn test_get_hooks_multiple_calls() {
        let mut config = LifecycleConfig::new();

        let hook_a = Arc::new(TestHook {
            hook_type: LifecycleHookType::OnResponse,
            result: HookResult::Continue,
        });

        let hook_b = Arc::new(TestHook {
            hook_type: LifecycleHookType::OnResponse,
            result: HookResult::Continue,
        });

        config.register(hook_a);
        config.register(hook_b);

        let hooks_on_response_first = config.get_hooks(LifecycleHookType::OnResponse);
        let hooks_on_response_second = config.get_hooks(LifecycleHookType::OnResponse);

        assert_eq!(hooks_on_response_first.len(), 2);
        assert_eq!(hooks_on_response_second.len(), 2);
    }

    #[test]
    fn test_hook_execute() {
        let hook = TestHook {
            hook_type: LifecycleHookType::OnRequest,
            result: HookResult::Continue,
        };

        let context = json!({ "test": "data" });
        let result = hook.execute(context);

        assert!(result.is_ok());
        match result.unwrap() {
            HookResult::Continue => {}
            HookResult::ShortCircuit(_) => panic!("Expected Continue"),
        }
    }

    #[test]
    fn test_hook_type_retrieval() {
        let hook = TestHook {
            hook_type: LifecycleHookType::PreValidation,
            result: HookResult::Continue,
        };

        assert_eq!(hook.hook_type(), LifecycleHookType::PreValidation);
    }

    #[test]
    fn test_all_hook_types() {
        let hook_types = vec![
            LifecycleHookType::OnRequest,
            LifecycleHookType::PreValidation,
            LifecycleHookType::PreHandler,
            LifecycleHookType::OnResponse,
            LifecycleHookType::OnError,
        ];

        let mut config = LifecycleConfig::new();

        for hook_type in &hook_types {
            let hook = Arc::new(TestHook {
                hook_type: *hook_type,
                result: HookResult::Continue,
            });
            config.register(hook);
        }

        for hook_type in hook_types {
            let hooks = config.get_hooks(hook_type);
            assert_eq!(hooks.len(), 1);
        }
    }

    #[test]
    fn test_hook_result_clone() {
        let original = HookResult::ShortCircuit(json!({ "key": "value" }));
        let cloned = original;

        match cloned {
            HookResult::ShortCircuit(response) => {
                assert_eq!(response["key"], "value");
            }
            HookResult::Continue => panic!("Expected ShortCircuit"),
        }
    }
}
