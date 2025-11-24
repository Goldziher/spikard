```python
from spikard import background

async def send_email(user_id: int) -> None:
    # enqueue real email send
    print(f"send email to {user_id}")

@app.post("/signup")
async def signup(user: User) -> User:
    background.run(send_email(user.id))
    return user
```
