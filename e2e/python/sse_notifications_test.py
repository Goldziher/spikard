#!/usr/bin/env python3
"""Test application generated from AsyncAPI specification"""

import asyncio
import json
from pathlib import Path
from typing import Any, Dict

import aiohttp

# Load test fixtures
FIXTURES_DIR = Path(__file__).parent.parent / "testing_data" / "sse"


def load_fixture(name: str) -> Dict[str, Any]:
    """Load a test fixture by name"""
    fixture_path = FIXTURES_DIR / f"{name}.json"
    if not fixture_path.exists():
        raise FileNotFoundError(f"Fixture not found: {fixture_path}")
    with open(fixture_path) as f:
        return json.load(f)


async def handle_sse(url: str) -> None:
    """Connect to SSE endpoint and handle events"""
    print(f"Connecting to {url}...")

    async with aiohttp.ClientSession() as session:
        async with session.get(url) as response:
            print("✓ Connected")
            async for line in response.content:
                line = line.decode("utf-8").strip()
                if line.startswith("data:"):
                    data = line[5:].strip()
                    try:
                        message = json.loads(data)
                        print(f"Received event: {message}")
                    except json.JSONDecodeError:
                        print(f"Received: {data}")


async def main() -> None:
    """Main entry point"""
    # Default SSE URI - override with environment variable SSE_URI
    import os

    url = os.getenv("SSE_URI", "http://localhost:8000/notifications")
    await handle_sse(url)


if __name__ == "__main__":
    asyncio.run(main())
