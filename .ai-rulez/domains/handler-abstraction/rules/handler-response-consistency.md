---
name: handler-response-consistency
priority: high
---
HandlerResponse must:
- Support Json, Text, Binary, and Empty body types
- Include HTTP status code (u16)
- Support optional response headers
- Never fail serialization (errors return 500)
- Preserve content-type negotiation from Accept header
