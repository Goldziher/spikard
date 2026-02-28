```python
from spikard.testing import TestClient

async def test_users():
    async with TestClient(app) as client:
        response = await client.get("/users/123")
        assert response.status_code == 200
        assert response.json()["id"] == 123
```

`TestClient` uses in-process Rust testing for speed. `LiveTestClient` starts a real subprocess server for WebSocket/SSE tests.

See the main documentation for WebSocket and SSE testing.
