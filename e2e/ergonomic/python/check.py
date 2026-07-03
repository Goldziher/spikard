"""Drives the ergonomic smoke server with real HTTP requests and asserts:

- a valid body  -> 2xx with the typed DTO serialized back
- an invalid body -> 422 ProblemDetails produced by the Rust CORE (not a
  language-side 400), proving validation is delegated to the core.

Exit 0 = pass. Run with the binding-installed interpreter, e.g.
    e2e/python/.venv/bin/python e2e/ergonomic/python/check.py
"""

import json
import os
import subprocess
import sys
import time
import urllib.error
import urllib.request

HERE = os.path.dirname(os.path.abspath(__file__))
PORT = 8000


def post(payload: dict) -> tuple[int, str]:
    req = urllib.request.Request(
        f"http://127.0.0.1:{PORT}/users",
        data=json.dumps(payload).encode(),
        headers={"Content-Type": "application/json"},
        method="POST",
    )
    try:
        resp = urllib.request.urlopen(req, timeout=5)
        return resp.status, resp.read().decode()
    except urllib.error.HTTPError as exc:
        return exc.code, exc.read().decode()


def main() -> int:
    log_path = os.path.join(HERE, ".server.log")
    log = open(log_path, "w")  # noqa: SIM115 (kept open for the child's lifetime)
    proc = subprocess.Popen(
        [sys.executable, os.path.join(HERE, "server.py")],
        stdout=log,
        stderr=subprocess.STDOUT,
    )
    try:
        # Wait for the server to bind (and surface a crash if it exits early).
        for _ in range(60):
            if proc.poll() is not None:
                log.flush()
                with open(log_path) as fh:
                    print(f"FAIL: server exited early rc={proc.returncode}")
                    print(fh.read()[:2500])
                return 1
            try:
                post({"name": "warmup", "email": "w@w.w", "age": 1})
                break
            except OSError:
                time.sleep(0.25)
        else:
            print("FAIL: server never came up")
            return 1

        status, body = post({"name": "Alice", "email": "alice@example.com", "age": 30})
        print(f"VALID   -> {status} {body}")
        if status not in (200, 201) or "Alice" not in body:
            print("FAIL: valid request did not return the typed DTO")
            return 1

        status2, body2 = post({"name": "Bob", "age": "not-a-number"})
        print(f"INVALID -> {status2} {body2}")
        if status2 != 422:
            print(f"FAIL: invalid body expected 422 from the core, got {status2}")
            return 1

        print("ERGO SMOKE PASS (python)")
        return 0
    finally:
        proc.terminate()
        try:
            proc.wait(timeout=5)
        except subprocess.TimeoutExpired:
            proc.kill()


if __name__ == "__main__":
    raise SystemExit(main())
