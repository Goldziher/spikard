```python
from spikard import App, ServerConfig
from msgspec import Struct

class User(Struct):
    id: int
    name: str

app = App()

@app.get_decorator("/users/{id:int}")
async def get_user(id: int) -> User:
    return User(id=id, name="Alice")

@app.post_decorator("/users")
async def create_user(user: User) -> User:
    return user

if __name__ == "__main__":
    app.config(ServerConfig(port=8000))
    app.run()
```
