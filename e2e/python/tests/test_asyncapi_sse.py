"""AsyncAPI SSE tests."""

import json
from pathlib import Path

from spikard.testing import TestClient

ROOT_DIR = Path(__file__).resolve().parents[3]
SSE_FIXTURE_ROOT = ROOT_DIR / "testing_data" / "sse"


def load_async_fixture(root: Path, name: str) -> dict:
    fixture_path = root / f"{name}.json"
    with fixture_path.open() as handle:
        return json.load(handle)


def load_fixture_examples(root: Path, name: str) -> list[str]:
    data = load_async_fixture(root, name)
    examples = data.get("examples", [])
    if not isinstance(examples, list) or not examples:
        return [json.dumps({})]
    return [json.dumps(example) for example in examples]


from app.main import (
    create_app_sse_notifications,
)


async def test_sse_notifications() -> None:
    """SSE channel test for /notifications."""
    async with TestClient(create_app_sse_notifications()) as client:
        response = await client.get("/notifications")
        assert response.status_code == 200
        body = response.text
        normalized = body.replace("\r\n", "\n")
        events = [chunk[5:] for chunk in normalized.split("\n\n") if chunk.startswith("data:")]
        fixture_names = ["systemAlert", "notificationBatch", "userNotification", "statusUpdate"]
        expected = []
        for fixture_name in fixture_names:
            expected.extend(load_fixture_examples(SSE_FIXTURE_ROOT, fixture_name))
        assert len(events) == len(expected)
        for payload, expected_json in zip(events, expected):
            assert json.loads(payload.strip()) == json.loads(expected_json)
