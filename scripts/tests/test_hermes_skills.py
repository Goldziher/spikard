"""Verify skill discovery through the actual Hermes Agent plugin manager."""

import os
import shutil
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
PROBE = """
import json
from hermes_cli.plugins import PluginManager
from tools.registry import registry
manager = PluginManager()
manager.discover_and_load()
names = manager.list_plugin_skills("spikard")
assert names == ["spikard"], f"registered skill count/name mismatch: {names}"
skill = manager.find_plugin_skill("spikard:spikard")
assert skill is not None and skill.is_file(), "registered skill is unreadable"
references = sorted((skill.parent / "references").glob("*.md"))
assert len(references) == 6, f"expected six skill references, received {len(references)}"
assert all(path.stat().st_size > 100 for path in references), "empty skill reference"
result = registry.dispatch("spikard_call", {"tool": "unknown"})
assert isinstance(result, str), "unsupported Hermes result contract"
assert "Choose a supported tool" in json.loads(result)["error"], result
print("Hermes native discovery: one skill, six readable references")
"""


class HermesSkillTests(unittest.TestCase):
    """Exercise the installed plugin bytes with isolated Hermes state."""

    def probe(self, remove_skill: bool) -> subprocess.CompletedProcess[str]:
        """Use a fresh plugin installation and SDK subprocess for each test."""
        with tempfile.TemporaryDirectory(prefix="spikard-hermes-contract-") as temporary:
            home = Path(temporary)
            plugin = home / "plugins/spikard"
            shutil.copytree(ROOT / "plugin/.hermes/plugins/spikard", plugin)
            (home / "config.yaml").write_text("plugins:\n  enabled: [spikard]\n")
            bundled = home / "empty-bundled"
            bundled.mkdir()
            if remove_skill:
                (plugin / "skills/spikard/SKILL.md").unlink()
            environment = os.environ | {
                "HERMES_HOME": str(home),
                "HERMES_BUNDLED_PLUGINS": str(bundled),
                "HERMES_ENABLE_PROJECT_PLUGINS": "true",
            }
            return subprocess.run(
                [sys.executable, "-c", PROBE], cwd=home, env=environment, text=True, capture_output=True, check=False
            )

    def test_should_discover_installed_spikard_skill_and_references(self) -> None:
        """A native registration succeeds with all bundled resources readable."""
        result = self.probe(remove_skill=False)
        assert result.returncode == 0, result.stdout + result.stderr
        assert "one skill, six readable references" in result.stdout

    def test_should_reject_an_installation_missing_its_registered_skill(self) -> None:
        """The contract fails for missing installed bytes rather than passing empty."""
        result = self.probe(remove_skill=True)
        assert result.returncode != 0
        assert "registered skill count/name mismatch" in result.stderr


if __name__ == "__main__":
    unittest.main()
