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
pub enum HookResult {
    /// Continue with normal processing
    Continue,
    /// Short-circuit and return this response
    ShortCircuit(serde_json::Value),
}

/// Trait for implementing lifecycle hooks in language bindings
pub trait LifecycleHook: Send + Sync {
    /// Execute the lifecycle hook
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
    pub fn get_hooks(&self, hook_type: LifecycleHookType) -> Vec<Arc<dyn LifecycleHook>> {
        self.hooks.get(&hook_type).cloned().unwrap_or_default()
    }
}

impl Default for LifecycleConfig {
    fn default() -> Self {
        Self::new()
    }
}
