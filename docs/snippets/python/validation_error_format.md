```python
from msgspec import ValidationError
from fastapi import Request, status
from fastapi.responses import JSONResponse

@app.exception_handler(ValidationError)
async def validation_exception_handler(
    request: Request,
    exc: ValidationError
) -> JSONResponse:
    return JSONResponse(
        status_code=status.HTTP_422_UNPROCESSABLE_ENTITY,
        content={
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
        }
    )
```
