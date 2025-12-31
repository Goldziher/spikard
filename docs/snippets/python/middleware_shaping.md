```python
from spikard import Spikard, HTTPError
import time
from collections import defaultdict

app = Spikard()

# Simple in-memory rate limiter (use Redis in production)
rate_limits = defaultdict(list)

@app.on_request
async def request_shaper(request):
    # 1. Rate limiting: 100 requests per minute per IP
    client_ip = request.get("client_ip", "unknown")
    now = time.time()

    # Clean old entries
    rate_limits[client_ip] = [
        ts for ts in rate_limits[client_ip] if now - ts < 60
    ]

    if len(rate_limits[client_ip]) >= 100:
        raise HTTPError(429, "Rate limit exceeded")

    rate_limits[client_ip].append(now)

    # 2. Normalize headers (lowercase keys)
    if "headers" in request:
        request["headers"] = {
            k.lower(): v for k, v in request["headers"].items()
        }

    # 3. Inject tenant from subdomain
    host = request.get("headers", {}).get("host", "")
    tenant = host.split(".")[0] if "." in host else "default"
    request["context"] = request.get("context", {})
    request["context"]["tenant"] = tenant

    # 4. Feature flags from query params or headers
    feature_flags = request.get("query", {}).get("features", "").split(",")
    request["context"]["features"] = set(f for f in feature_flags if f)

    return request

@app.on_response
async def compress_response(response):
    # Response compression for large payloads
    body = response.get("body", "")
    if len(body) > 1024:  # Compress if > 1KB
        import gzip
        response["body"] = gzip.compress(body.encode())
        response["headers"] = response.get("headers", {})
        response["headers"]["content-encoding"] = "gzip"

    return response
```
