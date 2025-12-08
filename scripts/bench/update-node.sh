#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../.." && pwd -P)"

cd "${REPO_ROOT}"

for app in elysia-dto express-dto express-raw fastify-dto fastify-raw hono-dto hono-raw morojs-dto spikard-node spikard-wasm; do
	echo "Updating $app..."
	cd "tools/benchmark-harness/apps/$app" && pnpm up --latest
	cd - >/dev/null
done
