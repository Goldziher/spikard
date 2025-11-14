"""Pytest configuration for E2E tests with real server."""

import asyncio
import os
import signal
import socket
import subprocess
import sys
import time
from pathlib import Path
from typing import AsyncIterator

import httpx
import pytest
from httpx_sse import aconnect_sse


def start_test_server(port: int = 8765) -> subprocess.Popen:
    """Start a Spikard test server in a subprocess.

    Args:
        port: Port to run the server on

    Returns:
        subprocess.Popen: The server process
    """
    # Find the test server script
    current_dir = Path(__file__).parent
    server_script = current_dir / "test_server.py"

    # Start the server process
    env = os.environ.copy()
    env["PYTHONPATH"] = str(Path(__file__).parent.parent.parent / "packages" / "python")

    process = subprocess.Popen(
        [sys.executable, str(server_script), str(port)],
        env=env,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        preexec_fn=os.setsid if hasattr(os, "setsid") else None,
    )

    # Wait for server to be ready
    timeout = 10
    start_time = time.time()

    while time.time() - start_time < timeout:
        try:
            sock = socket.create_connection(("127.0.0.1", port), timeout=1)
            sock.close()
            # Server is ready
            time.sleep(0.5)  # Give it a moment to fully initialize
            return process
        except (ConnectionRefusedError, OSError):
            time.sleep(0.1)

        # Check if process died
        if process.poll() is not None:
            stdout, stderr = process.communicate()
            raise RuntimeError(
                f"Server process died during startup:\nSTDOUT: {stdout.decode()}\nSTDERR: {stderr.decode()}"
            )

    # Timeout - kill the process
    try:
        os.killpg(os.getpgid(process.pid), signal.SIGKILL)
    except (ProcessLookupError, AttributeError):
        process.kill()

    raise TimeoutError(f"Server did not start within {timeout} seconds")


def kill_test_server(process: subprocess.Popen) -> None:
    """Kill the test server process.

    Args:
        process: The server process to kill
    """
    try:
        if hasattr(os, "killpg"):
            os.killpg(os.getpgid(process.pid), signal.SIGTERM)
        else:
            process.terminate()

        # Wait for graceful shutdown
        try:
            process.wait(timeout=5)
        except subprocess.TimeoutExpired:
            # Force kill if graceful shutdown fails
            if hasattr(os, "killpg"):
                os.killpg(os.getpgid(process.pid), signal.SIGKILL)
            else:
                process.kill()
    except (ProcessLookupError, AttributeError):
        pass


@pytest.fixture(scope="session")
def test_server():
    """Start a test server for the session and tear it down after."""
    port = 8765
    os.environ["TEST_SERVER_URL"] = f"http://127.0.0.1:{port}"
    os.environ["TEST_WS_URL"] = f"ws://127.0.0.1:{port}"

    process = start_test_server(port)

    yield {
        "http_url": f"http://127.0.0.1:{port}",
        "ws_url": f"ws://127.0.0.1:{port}",
        "port": port,
    }

    kill_test_server(process)


@pytest.fixture(scope="function")
def event_loop():
    """Create an event loop for each test function."""
    loop = asyncio.new_event_loop()
    yield loop
    loop.close()


@pytest.fixture
async def http_client(test_server) -> AsyncIterator[httpx.AsyncClient]:
    """Provide an async HTTP client for tests."""
    async with httpx.AsyncClient(base_url=test_server["http_url"], timeout=30.0) as client:
        yield client
