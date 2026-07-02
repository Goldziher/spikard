```python
from uuid import UUID

@app.get("/users/{user_id}/posts/{post_id}")
async def get_user_post(user_id: UUID, post_id: int) -> dict:
    # Type validation happens automatically
    # user_id must be valid UUID format
    # post_id must be valid integer
    return {
        "user_id": str(user_id),
        "post_id": post_id,
        "title": "Sample Post"
    }
```
