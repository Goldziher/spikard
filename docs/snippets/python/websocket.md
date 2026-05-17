---
id: python_websocket
language: python
title: Websocket
tags:
  - python
---

```python
from spikard import Spikard, websocket

app = Spikard()

@websocket("/ws")
async def echo(message: dict) -> dict | None:
    return {"echo": message}
```
