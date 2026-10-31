---
type: Reference
title: Build
description: Build Spikard core and optional language bindings
x-ai-rulez:
  kind: command
  id: build
  metadata:
    priority: medium
    usage: /build
---

# Build

Build the project using the standard task runner (mise).

1. Run `mise run build` for the core build
2. Run `mise run build:bindings` or `mise run build:all` explicitly when bindings are needed
3. Report any build errors with context
4. Suggest fixes for any compilation failures
