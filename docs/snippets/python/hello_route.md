```python
from spikard import App
from msgspec import Struct

class User(Struct):
    id: int
    name: str

app = App()

@app.get("/users/{id:int}")
async def get_user(id: int) -> User:
    return User(id=id, name="Alice")

if __name__ == "__main__":
    app.run(port=8000)
```
