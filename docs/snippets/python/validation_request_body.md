```python
from msgspec import Struct, ValidationError
from typing import Annotated
from spikard import Body

class CreateUserRequest(Struct):
    email: Annotated[str, "Email address"]
    age: Annotated[int, "User age must be 18+"]
    username: Annotated[str, "Alphanumeric username"]

@app.post("/users")
async def create_user(request: Body[CreateUserRequest]) -> dict:
    # Validation happens automatically before this handler runs
    # If validation fails, returns 400 with error details
    return {
        "id": "usr_123",
        "email": request.email,
        "age": request.age,
        "username": request.username
    }
```
