---
id: python_middleware_basic
language: python
title: Middleware Basic
tags:
  - python
---

```python
from spikard import Spikard

app = Spikard()

@app.on_request
async def logging_hook(request):
    print(f"{request['method']} {request['path']}")
    return request
```
