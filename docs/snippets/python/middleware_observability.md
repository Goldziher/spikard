```python
import logging
import uuid
from spikard import Spikard

app = Spikard()

@app.on_request
async def observability_middleware(request):
    # Generate or propagate request ID
    request_id = request.get("headers", {}).get("x-request-id", str(uuid.uuid4()))

    # Inject into context for handlers to use
    request["context"] = request.get("context", {})
    request["context"]["request_id"] = request_id

    # Log request with structured data
    logging.info("request_started", extra={
        "request_id": request_id,
        "method": request["method"],
        "path": request["path"],
        "user_agent": request.get("headers", {}).get("user-agent"),
    })

    return request

@app.on_response
async def response_logger(response):
    request_id = response.get("context", {}).get("request_id")

    logging.info("request_completed", extra={
        "request_id": request_id,
        "status": response["status"],
        "duration_ms": response.get("duration_ms"),
    })

    # Propagate request ID in response headers
    response["headers"] = response.get("headers", {})
    response["headers"]["X-Request-ID"] = request_id

    return response
```
