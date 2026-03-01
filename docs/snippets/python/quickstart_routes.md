```python
from spikard import Spikard
from spikard.config import ServerConfig
from msgspec import Struct

class User(Struct):
    id: int
    name: str

app = Spikard()

@app.get("/users/{id:int}")
async def get_user(id: int) -> User:
    return User(id=id, name="Alice")

@app.post("/users")
async def create_user(user: User) -> User:
    return user

if __name__ == "__main__":
    app.run(config=ServerConfig(port=8000))
```
