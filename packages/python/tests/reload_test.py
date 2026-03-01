"""Unit tests for the Python auto-reload helpers."""

from __future__ import annotations

from pathlib import Path

from spikard._internal.reload import detect_changes, iter_python_files, normalize_path, snapshot_paths


def _touch(path: Path, content: str) -> None:
    path.write_text(content, encoding="utf-8")


def test_detect_changes_for_modified_file(tmp_path: Path) -> None:
    """Modified source files trigger a reload."""
    path = tmp_path / "app.py"
    _touch(path, "print(1)\n")
    initial = snapshot_paths({path})
    _touch(path, "print(2)\n")

    assert detect_changes(initial, {path})


def test_iter_python_files_skips_ignored_directories(tmp_path: Path) -> None:
    """Ignored build/cache directories are excluded from reload watching."""
    src = tmp_path / "src"
    cache = tmp_path / "__pycache__"
    src.mkdir()
    cache.mkdir()
    _touch(src / "main.py", "print('ok')\n")
    _touch(cache / "ignored.py", "print('skip')\n")

    names = {path.name for path in iter_python_files(tmp_path)}
    assert "main.py" in names
    assert "ignored.py" not in names


def test_normalize_path_maps_pyc_to_source(tmp_path: Path) -> None:
    """Bytecode module paths normalize back to their source file."""
    source = tmp_path / "module.py"
    pyc = tmp_path / "module.pyc"
    _touch(source, "print('ok')\n")
    _touch(pyc, "")

    assert normalize_path(str(pyc)) == source.resolve()
