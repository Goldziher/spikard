"""Process-based auto-reload support for the Python package."""

from __future__ import annotations

import os
import subprocess
import sys
import time
from pathlib import Path

from spikard.config import ServerConfig

RELOAD_CHILD_ENV = "SPIKARD_RELOAD_CHILD"
DEFAULT_RELOAD_INTERVAL = 0.5
IGNORED_DIR_NAMES = {
    ".git",
    ".hg",
    ".mypy_cache",
    ".pytest_cache",
    ".ruff_cache",
    ".tox",
    ".venv",
    "__pycache__",
    "dist",
    "node_modules",
    "target",
    "venv",
}


def is_reload_child() -> bool:
    """Return True when the current process is the child server process."""
    return os.environ.get(RELOAD_CHILD_ENV) == "1"


def run_with_reload(app: object, config: ServerConfig) -> None:
    """Run the server under a parent reloader process."""
    if is_reload_child():
        _run_server_once(app, config)
        return

    if not sys.argv:
        msg = "reload mode requires a real Python entrypoint"
        raise RuntimeError(msg)

    while True:
        child = _spawn_reload_child()
        snapshots = snapshot_paths(collect_reload_paths())
        reload_requested = False

        try:
            while child.poll() is None:
                current_paths = collect_reload_paths()
                if detect_changes(snapshots, current_paths):
                    reload_requested = True
                    break
                time.sleep(DEFAULT_RELOAD_INTERVAL)
        except KeyboardInterrupt:
            _terminate_child(child)
            raise

        if not reload_requested:
            raise SystemExit(child.returncode or 0)

        _terminate_child(child)


def collect_reload_paths() -> set[Path]:
    """Collect Python source files to watch for reload."""
    paths: set[Path] = set()

    for module in tuple(sys.modules.values()):
        module_path = getattr(module, "__file__", None)
        if not module_path:
            continue
        path = normalize_path(module_path)
        if path is not None:
            paths.add(path)

    for path in iter_python_files(Path.cwd()):
        paths.add(path)

    return paths


def iter_python_files(root: Path) -> set[Path]:
    """Collect Python files under the current working tree."""
    paths: set[Path] = set()
    if not root.exists():
        return paths

    for path in root.rglob("*.py"):
        if any(part in IGNORED_DIR_NAMES for part in path.parts):
            continue
        paths.add(path.resolve())

    return paths


def normalize_path(raw_path: str) -> Path | None:
    """Normalize module paths to source files."""
    path = Path(raw_path)
    if path.suffix in {".py", ".pyi"} and path.exists():
        return path.resolve()
    if path.suffix == ".pyc":
        source = path.with_suffix(".py")
        if source.exists():
            return source.resolve()
    return None


def snapshot_paths(paths: set[Path]) -> dict[Path, int | None]:
    """Capture mtimes for watched files."""
    return {path: file_mtime(path) for path in paths}


def detect_changes(previous: dict[Path, int | None], current_paths: set[Path]) -> bool:
    """Return True when watched files were added, removed, or modified."""
    current = snapshot_paths(current_paths)
    if previous.keys() != current.keys():
        return True
    return any(previous[path] != current[path] for path in current)


def file_mtime(path: Path) -> int | None:
    """Return nanosecond mtime, or None when the file no longer exists."""
    try:
        return path.stat().st_mtime_ns
    except FileNotFoundError:
        return None


def _spawn_reload_child() -> subprocess.Popen[bytes]:
    env = os.environ.copy()
    env[RELOAD_CHILD_ENV] = "1"
    return subprocess.Popen([sys.executable, *sys.argv], env=env)


def _terminate_child(child: subprocess.Popen[bytes]) -> None:
    child.terminate()
    try:
        child.wait(timeout=5)
    except subprocess.TimeoutExpired:
        child.kill()
        child.wait(timeout=5)


def _run_server_once(app: object, config: ServerConfig) -> None:
    from _spikard import run_server  # type: ignore[attr-defined] # noqa: PLC0415

    run_server(app, config=config)
