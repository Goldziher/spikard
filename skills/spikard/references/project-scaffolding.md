# Project Scaffolding

Use `spikard init` or the `init_project` MCP tool to create a new app skeleton.

Example:

```bash
spikard init my_service --lang python --dir .
```

Supported languages:

- `python`
- `typescript`
- `rust`
- `ruby`
- `php`

Behavior:

- The project directory is `DIR/NAME`.
- The response includes the created files and next steps.
- Naming rules are enforced by language-specific scaffolders, so invalid project names should be fixed at the source instead of patched after generation.

Use scaffolding first when:

- starting a new service
- creating language-specific examples
- building a generator regression fixture
- preparing an agent workspace that should stay close to Spikard defaults
