```python
from spikard import App

app = App()

@app.get("/health")
async def health() -> dict:
    return {"status": "ok"}

@app.post("/users")
async def create_user(user: dict) -> dict:
    return user
```
