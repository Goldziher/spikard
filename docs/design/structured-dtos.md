# Structured DTO Generation

Spikard's CLI now emits typed request/response models for every language binding. The guiding principles:

- **Default DTOs** – each language has a clear default that mirrors the handwritten bindings:
  - Python → `dataclasses` (slots + kw-only). `--dto msgspec` switches to `msgspec.Struct`.
  - TypeScript → `zod` schemas + inferred `type`s (Node runtime).
  - Ruby → `Dry::Struct` backed by `dry-types`.
- **Per-language flags** – `spikard generate openapi ... --dto <style>` interprets values relative to the `--lang` that was selected. Invalid combinations (e.g. `--lang ruby --dto zod`) are rejected early.
- **Minimal `Any`/`unknown` usage** – structured DTOs are preferred. We only fall back to `dict[str, Any]`/`Record<string, unknown>` for anonymous objects or `additionalProperties` records where the OpenAPI schema does not describe fields.
- **Nullable vs optional** – schema-defined `nullable` becomes `| None`/`| null`/`.nullable()` while property optionality appends `= None`, `| undefined`, `.optional()`, etc.

This design keeps multi-language parity and prepares the CLI for future DTO families (Pydantic v2, msgspec enums, alternative Node schema libs, etc.) without inflating the CLI surface with per-language flags. Test generators and AsyncAPI harnesses continue to call into the same code paths to ensure parity between generated fixtures and the CLI output.
