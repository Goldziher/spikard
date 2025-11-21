#!/usr/bin/env python3
"""Test to profile JSON handling performance in Spikard-Python."""

from typing import Any

from spikard import Spikard, post

app = Spikard()


@post("/echo")
async def echo_json(body: dict[str, Any]) -> dict[str, Any]:
    """Echo JSON body back."""
    return body


@post("/no-body")
async def no_body() -> dict[str, Any]:
    """Return empty dict, no body processing."""
    return {}


if __name__ == "__main__":
    import sys

    port = int(sys.argv[1]) if len(sys.argv) > 1 else 8000
    app.run(host="0.0.0.0", port=port)
