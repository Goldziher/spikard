---
name: di-optional-feature
priority: high
---

DependencyInjectingHandler must:

- Be optional (feature-gated with "di")
- Wrap any Handler implementation
- Resolve dependencies per-request
- Pass resolved dependencies in RequestData
- Have zero cost when feature disabled
- Support all language bindings
