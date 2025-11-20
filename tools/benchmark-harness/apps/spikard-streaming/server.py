"""Streaming benchmark server backed by AsyncAPI fixtures."""

from __future__ import annotations

import json
import sys
from pathlib import Path

ROOT_DIR = Path(__file__).resolve().parents[4]
sys.path.append(str(ROOT_DIR / "packages" / "python"))

import argparse

from spikard import Spikard, get, sse, websocket
from spikard.config import ServerConfig

FIXTURE_ROOT = ROOT_DIR / "testing_data"
sys.path.append(str(ROOT_DIR / "packages" / "python"))

app = Spikard()


def load_fixture_examples(protocol: str, fixture_name: str) -> list[object]:
    fixture_path = FIXTURE_ROOT / protocol / f"{fixture_name}.json"
    data = json.loads(fixture_path.read_text())
    examples = data.get("examples", [])
    if not isinstance(examples, list) or not examples:
        return [{}]
    return examples


@sse("/notifications")
async def notifications_stream():
    fixture_names = ["systemAlert", "userNotification", "statusUpdate", "notificationBatch"]
    for name in fixture_names:
        for event in load_fixture_examples("sse", name):
            yield event


ACK_PAYLOAD = load_fixture_examples("websockets", "chatAck")[0]


@websocket("/chat")
async def chat_socket(message: dict) -> dict:
    if message.get("type") == "message":
        return ACK_PAYLOAD
    message["validated"] = True
    return message


@get("/health")
async def health_check() -> dict[str, str]:
    return {"status": "ok"}


def main() -> None:
    parser = argparse.ArgumentParser(description="Spikard streaming harness server")
    parser.add_argument("--port", type=int, default=8000)
    args = parser.parse_args()

    config = ServerConfig(port=args.port, host="127.0.0.1")
    app.run(config=config)


if __name__ == "__main__":
    main()
