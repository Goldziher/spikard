```python
from spikard import Spikard, background
from msgspec import Struct

app = Spikard()

class User(Struct):
    id: int
    email: str

async def send_email(user_id: int) -> None:
    # enqueue real email send
    print(f"send email to {user_id}")

@app.post("/signup")
async def signup(user: User) -> User:
    background.run(send_email(user.id))
    return user
```
