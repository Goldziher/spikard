# Testing And Fixtures

Spikard is fixture-first. Generated code should be validated with real tests, not just shape checks.

AsyncAPI helpers:

```bash
spikard testing asyncapi fixtures SCHEMA --output testing_data
spikard testing asyncapi test-app SCHEMA --lang python --output app.py
spikard testing asyncapi all SCHEMA --output e2e
spikard validate-asyncapi SCHEMA
```

Working rules:
- Validate suspect AsyncAPI documents before generating broad bundles.
- Keep fixture output and generated test apps aligned with the same schema revision.
- Prefer temporary directories or isolated output roots in tests so generated artifacts can be asserted directly.
- When modifying generator behavior, compare generated file names and contents across CLI and MCP paths.

Good validation targets:
- expected file count
- expected file names
- non-empty generated files
- schema-derived handler names
- stable defaults for DTO style and language output
