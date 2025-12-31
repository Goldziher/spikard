```python
from spikard import Spikard
from spikard.config import ServerConfig

config = ServerConfig(
    host="0.0.0.0",
    port=8080,
    workers=4,
    request_timeout=60,
    max_body_size=5 * 1024 * 1024,  # 5MB
)

app = Spikard(config=config)

@app.get("/health")
async def health():
    return {"status": "ok"}

if __name__ == "__main__":
    app.run()
```
