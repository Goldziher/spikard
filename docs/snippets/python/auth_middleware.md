```python
from spikard import Spikard

app = Spikard()

@app.pre_handler
async def auth_hook(request: dict[str, object]):
    headers = request.get("headers", {}) if isinstance(request, dict) else {}
    token = headers.get("authorization") if isinstance(headers, dict) else None
    if token != "Bearer dev-token":
        return {"error": "unauthorized"}, 401
    return request
```
