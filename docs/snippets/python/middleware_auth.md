```python
from spikard import Spikard, HTTPError
import jwt

app = Spikard()

@app.on_request
async def auth_guard(request):
    # Extract token from Authorization header
    auth_header = request.get("headers", {}).get("authorization", "")
    if not auth_header.startswith("Bearer "):
        raise HTTPError(401, "Missing or invalid authorization header")

    token = auth_header[7:]  # Strip "Bearer "

    try:
        # Verify and decode JWT
        payload = jwt.decode(token, "your-secret-key", algorithms=["HS256"])

        # Enrich context with authenticated user
        request["context"] = request.get("context", {})
        request["context"]["user_id"] = payload["sub"]
        request["context"]["roles"] = payload.get("roles", [])

        return request
    except jwt.InvalidTokenError:
        raise HTTPError(401, "Invalid token")
```
