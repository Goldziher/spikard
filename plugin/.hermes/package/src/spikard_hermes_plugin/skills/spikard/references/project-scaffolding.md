<!--
AI-RULEZ :: GENERATED FILE — DO NOT EDIT
Content-Hash: blake3:049284776566008882299b28587b5c4a6cdd1aea52b8796c2b2fe11101e5a888
Source-Hash: blake3:6ce2fb06ea35e1723d9c4b97abfb515db05b7e9052ba746196665eeef918c097
Schema-Version: v1
-->

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
- `elixir`

Behavior:

- The project directory is `DIR/NAME`.
- The response includes the created files and next steps.
- Naming rules are enforced by language-specific scaffolders, so invalid project names should be fixed at the source instead of patched after generation.

Use scaffolding first when:

- starting a new service
- creating language-specific examples
- building a generator regression fixture
- preparing an agent workspace that should stay close to Spikard defaults
