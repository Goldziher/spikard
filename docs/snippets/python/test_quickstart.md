```python
import pytest
from spikard import Spikard, get
from spikard.testing import TestClient

app = Spikard()

@get("/hello")
async def hello():
    return {"message": "Hello, World!"}

@pytest.mark.asyncio
async def test_hello():
    async with TestClient(app) as client:
        response = await client.get("/hello")
        assert response.status_code == 200
        assert response.json() == {"message": "Hello, World!"}
```
