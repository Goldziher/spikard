```python
@pytest.mark.asyncio
async def test_auth_middleware():
    app = Spikard()

    @app.pre_handler
    async def check_auth(request: dict) -> dict | tuple:
        token = request.get("headers", {}).get("authorization", "")
        if not token.startswith("Bearer "):
            return {"error": "Unauthorized"}, 401
        return request

    @app.get("/protected")
    async def protected():
        return {"data": "secret"}

    async with TestClient(app) as client:
        # Without auth
        response = await client.get("/protected")
        assert response.status_code == 401

        # With auth
        response = await client.get(
            "/protected",
            headers={"authorization": "Bearer token123"}
        )
        assert response.status_code == 200
```
