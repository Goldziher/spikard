#!/usr/bin/env python3

"""Check if a RubyGem version exists and emit output for GitHub Actions."""

from __future__ import annotations

import json
import os
import sys
import urllib.request
from pathlib import Path


def write_output(line: str) -> None:
    """Append a single line to GITHUB_OUTPUT if available."""
    output_path = os.environ.get("GITHUB_OUTPUT")
    if not output_path:
        return
    output_file = Path(output_path)
    with output_file.open("a", encoding="utf-8") as handle:
        handle.write(f"{line}\n")


def main() -> int:
    """Main entrypoint."""
    if len(sys.argv) < 2:
        return 1

    version = sys.argv[1]
    try:
        request = urllib.request.Request("https://rubygems.org/api/v1/versions/spikard.json", method="GET")
        with urllib.request.urlopen(request, timeout=10) as resp:  # noqa: S310
            data = json.load(resp)
        exists = any(entry.get("number") == version for entry in data)
    except Exception:
        exists = False

    write_output(f"exists={'true' if exists else 'false'}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
