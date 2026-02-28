```python
from spikard import Spikard
from msgspec import Struct

class User(Struct):
    id: int
    name: str
    email: str

app = Spikard()

@app.get("/users/{user_id}")
async def get_user(user_id: int) -> User:
    return User(id=user_id, name="Alice", email="alice@example.com")

@app.post("/users")
async def create_user(user: User) -> User:
    # Automatic validation via msgspec
    return user

if __name__ == "__main__":
    app.run(port=8000)
```

## Core Concepts

**Route Decorators:**
```python
from spikard import Spikard, get, post

app = Spikard()

@app.get("/users/{user_id}")
async def get_user(user_id: int):
    return {"id": user_id}

@post("/users")  # Standalone decorator style
async def create_user(user: User):
    return user
# Note: standalone decorators require app.include_router(get_default_router()) before app.run()
```

**Validation with msgspec (recommended):**
```python
from msgspec import Struct

class User(Struct):
    name: str
    email: str

@app.post("/users")
async def create_user(user: User):
    return user  # Automatic validation
```

**Dependency Injection:**
```python
from spikard.di import Provide

class DatabasePool:
    async def fetch(self, sql: str) -> list: ...

def create_pool() -> DatabasePool:
    return DatabasePool()

app.provide(DatabasePool, Provide(create_pool, singleton=True))

@app.get("/data")
async def get_data(pool: DatabasePool) -> dict:
    return {"data": await pool.fetch("SELECT * FROM items")}
```

**WebSockets:**
```python
from spikard import websocket

@app.websocket("/ws")
async def chat_endpoint(message: dict) -> dict | None:
    return {"echo": message}
```

**Server-Sent Events:**
```python
from spikard import sse

@sse("/events")
async def stream():
    for i in range(10):
        yield {"count": i}
```

**Lifecycle Hooks:**
```python
@app.pre_validation
async def check_auth(request):
    if not request.headers.get("authorization"):
        return Response({"error": "Unauthorized"}, 401)
    return request
```
