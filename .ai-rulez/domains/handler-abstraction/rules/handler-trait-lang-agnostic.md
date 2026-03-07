---
name: handler-trait-lang-agnostic
priority: high
---
Handler trait must remain completely language-agnostic. No language-specific
type hints or dependency requirements. RequestData and HandlerResponse must be
serializable and FFI-friendly. Zero-cost abstractions via Arc<dyn Handler>.
