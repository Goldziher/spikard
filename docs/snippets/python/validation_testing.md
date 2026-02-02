```python
import pytest
from spikard.testing import TestClient

@pytest.mark.asyncio
async def test_user_creation_validation(client: TestClient):
    # Valid request succeeds
    response = await client.post("/users", json={
        "email": "test@example.com",
        "age": 25,
        "username": "testuser"
    })
    assert response.status_code == 200

    # Invalid email rejected
    response = await client.post("/users", json={
        "email": "not-an-email",
        "age": 25,
        "username": "testuser"
    })
    assert response.status_code == 422
    assert "email" in response.json()["details"][0]["field"]

    # Age below minimum rejected
    response = await client.post("/users", json={
        "email": "test@example.com",
        "age": 16,
        "username": "testuser"
    })
    assert response.status_code == 422

    # Missing required field rejected
    response = await client.post("/users", json={
        "email": "test@example.com",
        "age": 25
    })
    assert response.status_code == 422
```
