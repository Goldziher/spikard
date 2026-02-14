from __future__ import annotations

import json

import pytest

from spikard import Spikard
from spikard.request import Request
from spikard.testing import TestClient


@pytest.mark.asyncio
async def test_handler_can_accept_request_object() -> None:
    app = Spikard()

    @app.post("/items/{id}")
    async def handler(request: Request) -> dict[str, object]:
        path_params = request.path_params
        query_params = request.query_params

        id_value = path_params.get("id")
        q = query_params.get("q")
        body = request.body
        # body may be raw bytes for JSON requests; decode if needed
        if isinstance(body, (bytes, bytearray)):
            body = json.loads(body)

        return {
            "method": request.method,
            "path": request.path,
            "id": id_value,
            "q": q,
            "name": body.get("name") if isinstance(body, dict) else None,
            "content_type": request.headers.get("content-type"),
        }

    async with TestClient(app) as client:
        response = await client.post("/items/123", params={"q": 1}, json={"name": "widget"})

    assert response.status_code == 200, response.json()
    data = response.json()
    assert data["method"] == "POST"
    assert data["path"] == "/items/123"
    assert data["id"] == 123
    assert data["q"] == 1
    assert data["name"] == "widget"
    assert "json" in str(data["content_type"]).lower()
