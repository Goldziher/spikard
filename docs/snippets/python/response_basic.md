```python
@app.get("/health")
async def health() -> dict:
    return {"status": "ok"}
```
