#!/usr/bin/env python3
"""Recompute and inject dependency hashes into ``build.zig.zon`` before ``zig build``.

The published spikard zig package's ``.hash`` in ``build.zig.zon`` is a placeholder:
the real hash can only be computed from the release tarball, which does not exist
until after the GitHub release publishes. This strips any stale/placeholder
``.hash`` lines and runs ``zig fetch`` per dependency URL to inject the correct
hash. Mirrors alef's canonical test-app zig run prelude so local and CI runs of the
published package succeed without hand-patching the generated ``build.zig.zon``.

Run from the directory containing ``build.zig.zon`` (``test_apps/zig``).
"""

from __future__ import annotations

import pathlib
import re
import subprocess

# Match `.hash = "..."` (leading newline + indentation) so we can drop the stale
# placeholder before recomputing. zig honors the LAST `.hash` key per dep, so a
# leftover placeholder alongside a fresh hash would win and break the fetch.
HASH_LINE = re.compile(r'\n[ \t]*\.hash\s*=\s*"[^"]*",')

# Capture each dependency name and its tarball URL from the `.dependencies` block.
# `[^{}]` (not `[^}]`) keeps the match from crossing into a nested `.{ ... }`, so
# it captures the dependency name (e.g. `spikard`) rather than the enclosing
# `dependencies` block.
DEP = re.compile(r'\.([a-z_0-9]+)\s*=\s*\.\{[^{}]*?\.url\s*=\s*"([^"]+)"', re.DOTALL)


def _inject(content: str, name: str, url: str, digest: str) -> str:
    pattern = re.compile(
        r"(\." + re.escape(name) + r'\s*=\s*\.\{[^{}]*?\.url\s*=\s*"' + re.escape(url) + r'",)(\s*\n)(\s*)',
        re.DOTALL,
    )
    return pattern.sub(
        lambda m: f'{m.group(1)}{m.group(2)}{m.group(3)}.hash = "{digest}",\n{m.group(3)}',
        content,
        count=1,
    )


def main() -> None:
    """Strip stale dependency hashes from build.zig.zon and inject freshly fetched ones."""
    zon = pathlib.Path("build.zig.zon")
    content = HASH_LINE.sub("", zon.read_text())

    deps = DEP.findall(content)
    for name, url in deps:
        result = subprocess.run(["zig", "fetch", url], capture_output=True, text=True, check=True)  # noqa: S603, S607
        content = _inject(content, name, url, result.stdout.strip())

    zon.write_text(content)
    print(f"Injected hashes for {len(deps)} dependency(ies) into build.zig.zon")  # noqa: T201


if __name__ == "__main__":
    main()
