---
name: schema-validation-codegen
priority: high
---
Code generation must produce validators that:
- Check required vs optional parameters
- Validate parameter types (string, number, boolean, object, array)
- Support format validation (email, uuid, date-time, uri)
- Enforce pattern constraints (regex)
- Check min/max/length constraints
- Use SchemaValidator from spikard-core
