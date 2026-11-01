<!--
AI-RULEZ :: GENERATED FILE — DO NOT EDIT
Content-Hash: blake3:05d8054d7ad5e1098a8159f90ca66a28a0db1d3019563feab98170164f1f9d71
Source-Hash: blake3:0909135361673ec671cac5be323506fce190b3f5ab5125c07835dbb35074d922
Schema-Version: v1
-->

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
