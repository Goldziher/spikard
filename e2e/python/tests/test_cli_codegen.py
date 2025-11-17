"""
CLI DTO/AsyncAPI generation smoke tests executed via the Python e2e suite.

These tests invoke `spikard-cli` directly to ensure the OpenAPI/AsyncAPI
generators produce runnable artifacts for all supported DTO configurations.
"""

from __future__ import annotations

import os
import subprocess
import sys
import tempfile
import textwrap
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
CLI_MANIFEST = ROOT / "crates" / "spikard-cli" / "Cargo.toml"

OPENAPI_SPEC = textwrap.dedent(
    """
    openapi: 3.1.0
    info:
      title: DTO Smoke API
      version: "1.0.0"
    paths:
      /hello:
        post:
          operationId: sayHello
          requestBody:
            required: true
            content:
              application/json:
                schema:
                  $ref: "#/components/schemas/HelloRequest"
          responses:
            "200":
              description: OK
              content:
                application/json:
                  schema:
                    $ref: "#/components/schemas/HelloResponse"
    components:
      schemas:
        HelloRequest:
          type: object
          properties:
            message:
              type: string
          required:
            - message
        HelloResponse:
          type: object
          properties:
            reply:
              type: string
          required:
            - reply
    """
)

ASYNCAPI_SPEC = textwrap.dedent(
    """
    asyncapi: "3.0.0"
    info:
      title: Chat DTO Smoke
      version: "1.0.0"
    servers:
      ws:
        host: chat.example.com
        protocol: ws
    channels:
      /chat:
        messages:
          chatMessage:
            payload:
              type: object
              properties:
                type:
                  const: chatMessage
                body:
                  type: string
              required:
                - type
                - body
    """
)


def run_cli(args: list[str], cwd: Path | None = None) -> None:
    """Invoke `cargo run -p spikard-cli -- <args>`."""
    cmd = ["cargo", "run", "--manifest-path", str(CLI_MANIFEST), "--", *args]
    subprocess.run(cmd, cwd=cwd or ROOT, check=True)


def create_python_stub(root: Path) -> Path:
    """Create lightweight `spikard` stubs so generated apps can import symbols."""
    stub_root = root / "stubs"
    (stub_root / "spikard").mkdir(parents=True, exist_ok=True)
    (stub_root / "spikard/__init__.py").write_text(
        """
class _Param:
    def __class_getitem__(cls, _item):
        return cls

Body = Path = Query = _Param

class Request:
    ...

class Spikard:
    def __init__(self):
        self._routes = []

    def route(self, *args, **kwargs):
        def decorator(fn):
            self._routes.append((args, kwargs, fn))
            return fn
        return decorator

def route(*_args, **_kwargs):
    def decorator(fn):
        return fn
    return decorator
"""
    )

    (stub_root / "_spikard.py").write_text(
        """
class Response:
    ...

class StreamingResponse:
    ...
"""
    )

    return stub_root


def import_generated_module(module_path: Path, stub_root: Path, class_name: str) -> None:
    """Load generated Python module and instantiate `class_name`."""
    script = textwrap.dedent(
        f"""
        import importlib.util
        import sys
        sys.path.insert(0, r"{stub_root}")
        spec = importlib.util.spec_from_file_location("generated_app", r"{module_path}")
        module = importlib.util.module_from_spec(spec)
        sys.modules["generated_app"] = module
        spec.loader.exec_module(module)
        instance = getattr(module, "{class_name}")(message="hello")
        assert instance.message == "hello"
        """
    )

    env = os.environ.copy()
    env["PYTHONPATH"] = f"{stub_root}:{env.get('PYTHONPATH', '')}"
    subprocess.run(["python3", "-c", script], check=True, env=env)


def test_cli_generates_python_dataclass_dto() -> None:
    with tempfile.TemporaryDirectory() as tmpdir:
        tmp = Path(tmpdir)
        spec_path = tmp / "openapi.yaml"
        output = tmp / "generated_app.py"
        spec_path.write_text(OPENAPI_SPEC)

        run_cli(
            [
                "generate",
                "openapi",
                str(spec_path),
                "--lang",
                "python",
                "--dto",
                "dataclass",
                "--output",
                str(output),
            ]
        )

        stub_root = create_python_stub(tmp)
        import_generated_module(output, stub_root, "HelloRequest")


def test_cli_generates_python_msgspec_dto() -> None:
    with tempfile.TemporaryDirectory() as tmpdir:
        tmp = Path(tmpdir)
        spec_path = tmp / "openapi.yaml"
        output = tmp / "generated_app.py"
        spec_path.write_text(OPENAPI_SPEC)

        run_cli(
            [
                "generate",
                "openapi",
                str(spec_path),
                "--lang",
                "python",
                "--dto",
                "msgspec",
                "--output",
                str(output),
            ]
        )

        stub_root = create_python_stub(tmp)
        import_generated_module(output, stub_root, "HelloRequest")


def test_cli_generates_typescript_zod_dto() -> None:
    with tempfile.TemporaryDirectory() as tmpdir:
        tmp = Path(tmpdir)
        spec_path = tmp / "openapi.yaml"
        output = tmp / "app.ts"
        spec_path.write_text(OPENAPI_SPEC)

        run_cli(
            [
                "generate",
                "openapi",
                str(spec_path),
                "--lang",
                "typescript",
                "--dto",
                "zod",
                "--output",
                str(output),
            ]
        )

        contents = output.read_text()
        assert 'import { z } from "zod"' in contents


def test_cli_generates_ruby_dry_struct_dto() -> None:
    with tempfile.TemporaryDirectory() as tmpdir:
        tmp = Path(tmpdir)
        spec_path = tmp / "openapi.yaml"
        output = tmp / "app.rb"
        spec_path.write_text(OPENAPI_SPEC)

        run_cli(
            [
                "generate",
                "openapi",
                str(spec_path),
                "--lang",
                "ruby",
                "--dto",
                "dry-schema",
                "--output",
                str(output),
            ]
        )

        subprocess.run(["ruby", "-c", str(output)], check=True)


def test_cli_generates_asyncapi_python_app() -> None:
    with tempfile.TemporaryDirectory() as tmpdir:
        tmp = Path(tmpdir)
        spec_path = tmp / "asyncapi.yaml"
        output = tmp / "ws_app.py"
        spec_path.write_text(ASYNCAPI_SPEC)

        run_cli(
            [
                "testing",
                "asyncapi",
                "test-app",
                str(spec_path),
                "--lang",
                "python",
                "--output",
                str(output),
            ]
        )

        stub_root = create_python_stub(tmp)
        env = os.environ.copy()
        env["PYTHONPATH"] = f"{stub_root}:{env.get('PYTHONPATH', '')}"
        subprocess.run(["python3", "-m", "py_compile", str(output)], check=True, env=env)


def test_cli_generates_asyncapi_python_handler() -> None:
    with tempfile.TemporaryDirectory() as tmpdir:
        tmp = Path(tmpdir)
        spec_path = tmp / "asyncapi.yaml"
        output = tmp / "handler.py"
        spec_path.write_text(ASYNCAPI_SPEC)

        run_cli(
            [
                "generate",
                "asyncapi",
                str(spec_path),
                "--lang",
                "python",
                "--output",
                str(output),
            ]
        )

        stub_root = create_python_stub(tmp)
        env = os.environ.copy()
        env["PYTHONPATH"] = f"{stub_root}:{env.get('PYTHONPATH', '')}"
        subprocess.run(["python3", "-m", "py_compile", str(output)], check=True, env=env)


def test_cli_generates_asyncapi_rust_handler() -> None:
    with tempfile.TemporaryDirectory() as tmpdir:
        tmp = Path(tmpdir)
        spec_path = tmp / "asyncapi.yaml"
        output = tmp / "handler.rs"
        spec_path.write_text(ASYNCAPI_SPEC)

        run_cli(
            [
                "generate",
                "asyncapi",
                str(spec_path),
                "--lang",
                "rust",
                "--output",
                str(output),
            ]
        )

        contents = output.read_text()
        assert "use spikard::{App, AppError, WebSocketHandler};" in contents
        assert 'app.websocket("/chat", ChatWebSocketHandler);' in contents


def test_cli_generates_asyncapi_php_handler() -> None:
    with tempfile.TemporaryDirectory() as tmpdir:
        tmp = Path(tmpdir)
        spec_path = tmp / "asyncapi.yaml"
        output = tmp / "handler.php"
        spec_path.write_text(ASYNCAPI_SPEC)

        run_cli(
            [
                "generate",
                "asyncapi",
                str(spec_path),
                "--lang",
                "php",
                "--output",
                str(output),
            ]
        )

        contents = output.read_text()
        assert contents.startswith("<?php")
