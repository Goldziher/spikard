```python
@pytest.mark.asyncio
async def test_user_creation():
    app = Spikard()

    @app.post("/users")
    async def create_user(name: str, email: str):
        return {"id": 1, "name": name, "email": email}

    async with TestClient(app) as client:
        response = await client.post(
            "/users",
            json={"name": "Alice", "email": "alice@example.com"}
        )
        assert response.status_code == 200
        data = response.json()
        assert data["name"] == "Alice"
        assert data["email"] == "alice@example.com"
```
