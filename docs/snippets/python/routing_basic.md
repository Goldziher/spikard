---
id: python_routing_basic
language: python
title: Routing Basic
tags:
  - python
---

```python
from spikard import Spikard
from msgspec import Struct


class User(Struct):
    id: int
    name: str

app = Spikard()

@app.get("/health")
async def health() -> dict:
    return {"status": "ok"}

@app.post("/users")
async def create_user(user: User) -> User:
    return user
```
