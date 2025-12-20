from __future__ import annotations

from typing import Protocol

import msgspec
import pytest

from spikard import Spikard
from spikard.testing import TestClient


class HandlerRequest(Protocol):
    method: str
    path: str
    path_params: dict[str, object]
    query_params: dict[str, object]
    headers: dict[str, str]
    cookies: dict[str, str]
    body: bytes


@pytest.mark.asyncio
async def test_handler_can_accept_request_object() -> None:
    app = Spikard()

    @app.post("/items/{id}")
    async def handler(request: HandlerRequest):
        decoded: dict[str, object] = msgspec.json.decode(request.body, type=dict[str, object])

        name = decoded.get("name")
        assert isinstance(name, str)

        q = request.query_params.get("q")
        assert isinstance(q, int)

        id_value = request.path_params.get("id")
        assert isinstance(id_value, int)

        return {
            "method": request.method,
            "path": request.path,
            "id": id_value,
            "q": q,
            "name": name,
            "content_type": request.headers.get("content-type"),
        }

    async with TestClient(app) as client:
        response = await client.post("/items/123", params={"q": 1}, json={"name": "widget"})

    assert response.status_code == 200, response.text
    data = response.json()
    assert data["method"] == "POST"
    assert data["path"] == "/items/123"
    assert data["id"] == 123
    assert data["q"] == 1
    assert data["name"] == "widget"
    assert data["content_type"].startswith("application/json")
