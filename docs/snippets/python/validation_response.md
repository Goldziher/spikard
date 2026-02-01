```python
from msgspec import Struct
from typing import List

class User(Struct):
    id: str
    email: str
    age: int

class UserListResponse(Struct):
    users: List[User]
    total: int
    page: int

@app.get("/users", response_schema=UserListResponse)
async def list_users() -> UserListResponse:
    # Response will be validated against UserListResponse schema
    # Any field mismatch or type error returns 500 with details
    users = [
        User(id="usr_1", email="alice@example.com", age=30),
        User(id="usr_2", email="bob@example.com", age=25)
    ]

    response = UserListResponse(
        users=users,
        total=len(users),
        page=1
    )

    # Validation happens here before sending response
    return response

# Example error: missing required field
@app.get("/invalid", response_schema=User)
async def invalid_response() -> dict:
    # This will fail validation - missing 'age' field
    # Returns 500: {"error": "Response validation failed: missing field 'age'"}
    return {"id": "usr_1", "email": "test@example.com"}
```
