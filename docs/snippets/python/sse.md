---
id: python_sse
language: python
title: Sse
tags:
  - python
---

```python
from spikard import Spikard, SseEvent, sse

app = Spikard()

@sse("/events")
async def events():
    for i in range(3):
        yield SseEvent(data={"tick": i})
```
