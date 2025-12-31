```python
from pydantic import BaseModel

class UserCreate(BaseModel):
    name: str
    age: int

@pytest.mark.asyncio
async def test_validation_failure():
    app = Spikard()

    @app.post("/users")
    async def create_user(user: UserCreate):
        return {"name": user.name, "age": user.age}

    async with TestClient(app) as client:
        # Invalid: age is string
        response = await client.post(
            "/users",
            json={"name": "Bob", "age": "not a number"}
        )
        assert response.status_code == 400
        error = response.json()
        assert "validation" in str(error).lower()
```
