"""Verify real Spikard MCP tools and code generation against an explicit executable."""

import importlib.util
import json
import os
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


class NativeMcpTests(unittest.IsolatedAsyncioTestCase):
    """Exercise the generated adapter and actual product process."""

    def setUp(self) -> None:
        """Require the fixture rather than letting an absent binary pass quietly."""
        executable = os.environ.get("SPIKARD_TEST_BIN")
        assert executable and Path(executable).is_file(), "Set SPIKARD_TEST_BIN to a verified MCP executable"
        spec = importlib.util.spec_from_file_location(
            "native_adapter", ROOT / "plugin/.hermes/plugins/spikard/hermes.py"
        )
        assert spec is not None and spec.loader is not None
        self.adapter = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(self.adapter)
        self.adapter.CLI_COMMAND = executable

    async def test_should_expose_all_thirteen_actual_tool_schemas(self) -> None:
        """Require exact names, count and SQL request fields from the live server."""
        result = await self.adapter.call_tool({"tool": "list_tools"})
        assert sorted(tool["name"] for tool in result.get("tools", [])) == sorted(self.adapter.TOOL_NAMES), result
        assert len(result["tools"]) == 13
        sql = next(tool for tool in result["tools"] if tool["name"] == "generate_sql")
        assert sorted(sql["inputSchema"]["required"]) == ["lang", "queries", "schema"]

    async def test_should_scaffold_a_real_python_project_via_mcp(self) -> None:
        """Read the generated files rather than counting a successful response alone."""
        with tempfile.TemporaryDirectory(prefix="spikard-native-mcp-") as temporary:
            result = json.loads(
                await self.adapter.handle_tool(
                    {
                        "tool": "init_project",
                        "tool_arguments": {"name": "native_probe", "language": "python", "directory": temporary},
                    }
                )
            )
            assert not result.get("isError", False), result
            files = sorted(path for path in Path(temporary).rglob("*") if path.is_file())  # noqa: ASYNC240 ~keep Inspect files after the completed MCP call.
            assert len(files) >= 3, files
            assert "pyproject.toml" in [path.name for path in files]
            assert all(path.stat().st_size > 0 for path in files), files


if __name__ == "__main__":
    unittest.main()
