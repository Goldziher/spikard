```python
@pytest.mark.asyncio
async def test_user_workflow():
    """Test complete user creation and retrieval workflow."""
    app = Spikard()
    users_db = {}

    @app.post("/users")
    async def create_user(name: str):
        user_id = len(users_db) + 1
        users_db[user_id] = {"id": user_id, "name": name}
        return users_db[user_id]

    @app.get("/users/{user_id}")
    async def get_user(user_id: int):
        return users_db.get(user_id, {"error": "Not found"})

    async with TestClient(app) as client:
        # Create user
        create_response = await client.post(
            "/users",
            json={"name": "Alice"}
        )
        assert create_response.status_code == 200
        user = create_response.json()
        assert user["name"] == "Alice"

        # Retrieve user
        get_response = await client.get(f"/users/{user['id']}")
        assert get_response.status_code == 200
        retrieved = get_response.json()
        assert retrieved == user
```
