#!/usr/bin/env bash
set -euo pipefail

# Git filter-repo helper to scrub AI/Claude signatures from commit messages.
# Usage:
#   git filter-repo --force --message-callback "$(pwd)/scripts/filter-strip-ai-signatures.sh"

python3 - <<'PYCODE'
import re
import sys

SIGNATURE_PATTERNS = [
    re.compile(r"(?im)^co-authored-by:.*claude.*$", re.IGNORECASE),
    re.compile(r"(?im)^signed-off-by:.*claude.*$", re.IGNORECASE),
    re.compile(r"(?im)^claude(?:\s+code\s+signature)?[:\-\s].*$", re.IGNORECASE),
]

def scrub(message: str) -> str:
    lines = message.splitlines()
    cleaned = []
    for line in lines:
        if any(pat.search(line) for pat in SIGNATURE_PATTERNS):
            continue
        cleaned.append(line)
    # Trim extra trailing blank lines introduced by removals
    while cleaned and cleaned[-1].strip() == "":
        cleaned.pop()
    return "\n".join(cleaned) + ("\n" if cleaned else "")

for orig in sys.stdin:
    sys.stdout.write(scrub(orig))
PYCODE
