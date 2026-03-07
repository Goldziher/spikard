---
name: request-data-efficiency
priority: high
---
RequestData must:
- Use Arc<> for all HashMap fields (path_params, headers, cookies, raw_query_params)
- Prefer raw_body for language bindings (zero-copy)
- Lazy-parse body only when validation needed
- Include validated_params from ParameterValidator, not raw params
- Support optional DI dependencies when feature enabled
- Be serializable for FFI boundaries
