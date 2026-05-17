---
id: python_response_basic
language: python
title: Response Basic
tags:
  - python
---

```python
@app.get("/health")
async def health() -> dict:
    return {"status": "ok"}
```
