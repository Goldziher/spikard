#!/usr/bin/env bash
set -euo pipefail

# Enable corepack for Node.js package manager management
if ! corepack enable; then
	echo "Error: Failed to enable corepack" >&2
	exit 1
fi

# Prepare and activate latest pnpm version
if ! corepack prepare pnpm@latest --activate; then
	echo "Error: Failed to prepare and activate pnpm@latest" >&2
	exit 1
fi

echo "pnpm setup completed successfully"
