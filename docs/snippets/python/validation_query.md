```python
from msgspec import Struct

class ListUsersQuery(Struct):
    page: int = 1  # Default value
    limit: int = 10
    sort_by: str | None = None
    min_age: int | None = None

@app.get("/users")
async def list_users(query: ListUsersQuery) -> dict:
    # Validate constraints in handler or use custom validators
    if query.limit > 100:
        raise ValueError("limit cannot exceed 100")
    if query.page < 1:
        raise ValueError("page must be positive")

    return {
        "page": query.page,
        "limit": query.limit,
        "users": []
    }
```
