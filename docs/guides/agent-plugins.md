# Coding agent plugins

Spikard bundles one public skill with six API references for Claude Code, Codex, Cursor, Gemini, Kimi, OpenCode, Pi, Hermes, Factory, Agent Plugins 1.0 and Copilot. The plugin calls the local MCP-enabled Spikard CLI; framework bindings alone do not install it. Pi installs the public skill and does not expose MCP.

```bash
cargo install spikard-cli --version 0.17.2 --locked
spikard mcp --help
```

The default `mcp` Cargo feature provides stdio. HTTP transport requires `mcp-http`; plugins use stdio. The installed server exposes 13 tools including SQL-to-HTTP generation. Inspect `list_tools` request schemas before supplying arguments. No provider key or HTTP authentication is needed to run the local server.

## Install from Git or a checkout

Claude Code:

```text
/plugin marketplace add Goldziher/spikard
/plugin install spikard@spikard
```

Codex: add `Goldziher/spikard` as a marketplace in its plugin manager, then install `spikard`. Both `.codex-plugin/plugin.json` and portable `plugin.json` are generated. The root marketplace index registers the curated child plugin rather than repository maintainer instructions.

OpenCode uses the package name `@goldziher/opencode-spikard` after publication; for a checkout add the absolute `plugin/.opencode/plugins/spikard.js` path to `opencode.json`'s `plugin` array. It registers the namespaced `spikard/spikard` skill.

Pi:

```bash
pi install /absolute/path/to/spikard/plugin
# After the first npm publication:
pi install npm:@goldziher/opencode-spikard
```

Gemini: `gemini extensions install /absolute/path/to/spikard/plugin`. Hermes: copy `plugin/.hermes/plugins/spikard` into `$HERMES_HOME/plugins/spikard` and enable `spikard` in the Hermes plugin settings, or install the generated wheel. Its `spikard_call` tool accepts `tool` and optional `tool_arguments`; `list_tools` discovers the installed server schemas. Skills and all supporting resources ship with both forms.

Cursor: install the portable plugin from `plugin/.cursor-plugin/plugin.json`, or import the Cursor team marketplace produced by the release workflow. Kimi reads `plugin/kimi.plugin.json`; CLI installation requires a client version that supports plugins. Factory reads `plugin/.factory-plugin/plugin.json`, Copilot and Agent Plugins use `plugin/plugin.json`. Match the client version's native installation UI to the generated manifest, then confirm `spikard` is discovered before invoking it.

Standalone skills: copy `skills/spikard` into the client's skills directory, retaining the entire `references/` folder. This provides instructions only; configure the local MCP server separately if the client supports it.

## Maintain and publish

Author product content in `plugin/.ai-rulez/`; repository governance stays in `.ai-rulez/`. Both use a separate `config.plugins.toml` layer. Run `task plugin:generate`, `task ai:check`, and the public resource/native SDK tests before committing. Public `skills/` bytes are generated from the curated native bundle, with ownership hashes and drift checks.

The [publishing setup](https://github.com/Goldziher/spikard/blob/main/.github/PLUGIN-PUBLISHING.md) describes GitHub artifacts, npm and PyPI trusted publishers. Until the new packages are first published, use the checkout paths above. Registry versions are immutable; a subsequent changed bundle needs a new version.
