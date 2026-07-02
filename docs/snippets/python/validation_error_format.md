```python
from msgspec import ValidationError
from spikard import Response

@app.exception_handler(ValidationError)
async def validation_exception_handler(
    request,
    exc: ValidationError
) -> Response:
    return Response.json(
        {
            "error": "validation_failed",
            "message": "Request validation failed",
            "details": [
                {
                    "field": err.get("loc", ["unknown"])[0],
                    "message": err.get("msg", "Invalid value"),
                    "type": err.get("type", "unknown")
                }
                for err in exc.errors()
            ]
        },
        status_code=422
    )
```
