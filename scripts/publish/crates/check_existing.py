"""Check whether crates are already published and emit outputs for GitHub Actions."""

from __future__ import annotations

import json
import os
import sys
import urllib.request
from pathlib import Path
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from collections.abc import Iterable


def write_outputs(lines: Iterable[str]) -> None:
    """Append output lines to GITHUB_OUTPUT if available."""
    output_path = os.environ.get("GITHUB_OUTPUT")
    if not output_path:
        return
    output_file = Path(output_path)
    with output_file.open("a", encoding="utf-8") as handle:
        handle.writelines(f"{line}\n" for line in lines)


def fetch_versions(crate: str) -> list[str]:
    """Fetch published versions for a crate from crates.io."""
    url = f"https://crates.io/api/v1/crates/{crate}"
    request = urllib.request.Request(url=url, method="GET")  # noqa: S310
    with urllib.request.urlopen(request, timeout=10) as resp:  # noqa: S310
        data = json.load(resp)
    return [item.get("num", "") for item in data.get("versions", [])]


def main() -> int:
    """Main entrypoint."""
    if len(sys.argv) < 2:
        return 1

    version = sys.argv[1]
    crates = [
        ("spikard-core", "spikard_core_exists"),
        ("spikard", "spikard_exists"),
        ("spikard-http", "http_exists"),
    ]

    outputs: list[str] = []
    for crate, key in crates:
        try:
            versions = fetch_versions(crate)
        except Exception:
            exists = False
        else:
            exists = version in versions

        outputs.append(f"{key}={'true' if exists else 'false'}")

    write_outputs(outputs)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
