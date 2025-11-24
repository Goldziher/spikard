```python
from spikard import background

@background.task
async def send_email(user_id: int) -> None:
    # enqueue real email send
    print(f"send email to {user_id}")

@app.post("/signup")
async def signup(user: User) -> User:
    send_email.enqueue(user.id)
    return user
```
