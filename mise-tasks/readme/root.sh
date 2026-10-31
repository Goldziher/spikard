#!/usr/bin/env bash
#MISE description="Render the root README from templates/readme/root.md"
#MISE hide=true
set -euo pipefail

python3 - <<'PY'
from pathlib import Path

template = Path("templates/readme/root.md").read_text()
badges = Path("templates/readme/partials/badges.html.jinja").read_text()
rendered = template.replace("{% include 'partials/badges.html.jinja' %}", badges.rstrip())
Path("README.md").write_text(rendered)
PY
