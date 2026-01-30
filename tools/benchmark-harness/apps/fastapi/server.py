"""FastAPI + Granian benchmark server (raw + validated endpoints).

Serves both raw endpoints (no validation) and validated endpoints (/validated/... prefix).
Raw endpoints use Request.json() for minimal overhead.
Validated endpoints use Pydantic models for type checking and validation.
"""

import sys
import urllib.parse
from datetime import date as DateType
from typing import Any
from uuid import UUID

from fastapi import FastAPI, Request
from fastapi.exceptions import RequestValidationError
from fastapi.responses import ORJSONResponse
from pydantic import BaseModel

app = FastAPI(default_response_class=ORJSONResponse)


@app.exception_handler(RequestValidationError)
async def validation_exception_handler(request: Request, exc: RequestValidationError):
    """Convert FastAPI 422 validation errors to 400 for consistency."""
    return ORJSONResponse(status_code=400, content={"detail": str(exc)})


async def _parse_urlencoded(request: Request) -> dict[str, Any]:
    raw = await request.body()
    text = raw.decode("utf-8", errors="replace")
    parsed = urllib.parse.parse_qs(text, keep_blank_values=True, strict_parsing=False)
    return {k: (v[0] if len(v) == 1 else v) for k, v in parsed.items()}


# ============================================================================
# Pydantic Models for Validation
# ============================================================================


class SmallPayload(BaseModel):
    """Small JSON payload - matches 01_simple_object_success.json."""

    name: str
    description: str
    price: float
    tax: float | None = None


class Image(BaseModel):
    url: str
    name: str


class MediumPayload(BaseModel):
    name: str
    price: float
    image: Image


class Country(BaseModel):
    name: str
    code: str


class Address(BaseModel):
    street: str
    city: str
    country: Country


class SellerWithAddress(BaseModel):
    name: str
    address: Address


class LargePayload(BaseModel):
    name: str
    price: float
    seller: SellerWithAddress


class VeryLargePayload(BaseModel):
    name: str
    tags: list[str]
    images: list[Image]


class UrlencodedSimple(BaseModel):
    """Simple URL-encoded form validation schema."""

    name: str
    email: str
    age: int
    subscribe: bool


class UrlencodedComplex(BaseModel):
    """Complex URL-encoded form validation schema."""

    username: str
    password: str
    email: str
    first_name: str
    last_name: str
    age: int
    country: str
    state: str
    city: str
    zip: str
    phone: str
    company: str
    job_title: str
    subscribe: bool
    newsletter: bool
    terms_accepted: bool
    privacy_accepted: bool
    marketing_consent: bool
    two_factor_enabled: bool


class QueryFew(BaseModel):
    """Query parameters validation schema for few params."""

    q: str
    page: int | None = None
    limit: int | None = None


class QueryMedium(BaseModel):
    """Query parameters validation schema for medium params."""

    search: str
    category: str | None = None
    sort: str | None = None
    order: str | None = None
    page: int | None = None
    limit: int | None = None
    filter: str | None = None


class QueryMany(BaseModel):
    """Query parameters validation schema for many params."""

    q: str
    category: str | None = None
    subcategory: str | None = None
    brand: str | None = None
    min_price: float | None = None
    max_price: float | None = None
    color: str | None = None
    size: str | None = None
    material: str | None = None
    rating: int | None = None
    sort: str | None = None
    order: str | None = None
    page: int | None = None
    limit: int | None = None
    in_stock: bool | None = None
    on_sale: bool | None = None


# ============================================================================
# Raw Endpoints (No Validation)
# ============================================================================


@app.post("/json/small")
async def post_json_small(request: Request) -> dict[str, Any]:
    """Small JSON body (~100 bytes) - no validation."""
    body = await request.json()
    return body


@app.post("/json/medium")
async def post_json_medium(request: Request) -> dict[str, Any]:
    """Medium JSON body (~1KB) - no validation."""
    body = await request.json()
    return body


@app.post("/json/large")
async def post_json_large(request: Request) -> dict[str, Any]:
    """Large JSON body (~10KB) - no validation."""
    body = await request.json()
    return body


@app.post("/json/very-large")
async def post_json_very_large(request: Request) -> dict[str, Any]:
    """Very large JSON body (~100KB) - no validation."""
    body = await request.json()
    return body


@app.post("/multipart/small")
async def post_multipart_small(request: Request) -> dict[str, Any]:
    """Small multipart form (~1KB)."""
    form = await request.form()
    files_received = 0
    total_bytes = 0
    for key, value in form.items():
        if hasattr(value, "file"):
            files_received += 1
            content = await value.read()
            total_bytes += len(content)
    return {"files_received": files_received, "total_bytes": total_bytes}


@app.post("/multipart/medium")
async def post_multipart_medium(request: Request) -> dict[str, Any]:
    """Medium multipart form (~10KB)."""
    form = await request.form()
    files_received = 0
    total_bytes = 0
    for key, value in form.items():
        if hasattr(value, "file"):
            files_received += 1
            content = await value.read()
            total_bytes += len(content)
    return {"files_received": files_received, "total_bytes": total_bytes}


@app.post("/multipart/large")
async def post_multipart_large(request: Request) -> dict[str, Any]:
    """Large multipart form (~100KB)."""
    form = await request.form()
    files_received = 0
    total_bytes = 0
    for key, value in form.items():
        if hasattr(value, "file"):
            files_received += 1
            content = await value.read()
            total_bytes += len(content)
    return {"files_received": files_received, "total_bytes": total_bytes}


@app.post("/urlencoded/simple")
async def post_urlencoded_simple(request: Request) -> dict[str, Any]:
    """Simple URL-encoded form."""
    return await _parse_urlencoded(request)


@app.post("/urlencoded/complex")
async def post_urlencoded_complex(request: Request) -> dict[str, Any]:
    """Complex URL-encoded form."""
    return await _parse_urlencoded(request)


@app.get("/path/simple/{id}")
async def get_path_simple(id: str) -> dict[str, Any]:
    """Single path parameter."""
    return {"id": id}


@app.get("/path/multiple/{user_id}/{post_id}")
async def get_path_multiple(user_id: str, post_id: str) -> dict[str, Any]:
    """Multiple path parameters."""
    return {"user_id": user_id, "post_id": post_id}


@app.get("/path/deep/{org}/{team}/{project}/{resource}/{id}")
async def get_path_deep(org: str, team: str, project: str, resource: str, id: str) -> dict[str, Any]:
    """Deep nested path parameters."""
    return {
        "org": org,
        "team": team,
        "project": project,
        "resource": resource,
        "id": id,
    }


@app.get("/path/int/{id}")
async def get_path_int(id: int) -> dict[str, Any]:
    """Path parameter with int type."""
    return {"id": id}


@app.get("/path/uuid/{uuid}")
async def get_path_uuid(uuid: UUID) -> dict[str, Any]:
    """Path parameter with UUID."""
    return {"uuid": str(uuid)}


@app.get("/path/date/{date}")
async def get_path_date(date: DateType) -> dict[str, Any]:
    """Path parameter with date."""
    return {"date": date.isoformat()}


@app.get("/query/few")
async def get_query_few(request: Request) -> dict[str, Any]:
    """Few query parameters (1-2)."""
    return dict(request.query_params)


@app.get("/query/medium")
async def get_query_medium(request: Request) -> dict[str, Any]:
    """Medium query parameters (3-5)."""
    return dict(request.query_params)


@app.get("/query/many")
async def get_query_many(request: Request) -> dict[str, Any]:
    """Many query parameters (6-10)."""
    return dict(request.query_params)


# ============================================================================
# Validated Endpoints (Pydantic Validation)
# ============================================================================


@app.post("/validated/json/small")
async def post_json_small_validated(body: SmallPayload) -> SmallPayload:
    """Small JSON body (~100 bytes) with Pydantic validation."""
    return body


@app.post("/validated/json/medium")
async def post_json_medium_validated(body: MediumPayload) -> MediumPayload:
    """Medium JSON body (~1KB) with Pydantic validation."""
    return body


@app.post("/validated/json/large")
async def post_json_large_validated(body: LargePayload) -> LargePayload:
    """Large JSON body (~10KB) with Pydantic validation."""
    return body


@app.post("/validated/json/very-large")
async def post_json_very_large_validated(body: VeryLargePayload) -> VeryLargePayload:
    """Very large JSON body (~100KB) with Pydantic validation."""
    return body


@app.post("/validated/multipart/small")
async def post_multipart_small_validated(request: Request) -> dict[str, Any]:
    """Small multipart form (~1KB)."""
    form = await request.form()
    files_received = 0
    total_bytes = 0
    for key, value in form.items():
        if hasattr(value, "file"):
            files_received += 1
            content = await value.read()
            total_bytes += len(content)
    if files_received == 0:
        from fastapi import HTTPException

        raise HTTPException(status_code=400, detail="No files received")
    return {"files_received": files_received, "total_bytes": total_bytes}


@app.post("/validated/multipart/medium")
async def post_multipart_medium_validated(request: Request) -> dict[str, Any]:
    """Medium multipart form (~10KB)."""
    form = await request.form()
    files_received = 0
    total_bytes = 0
    for key, value in form.items():
        if hasattr(value, "file"):
            files_received += 1
            content = await value.read()
            total_bytes += len(content)
    if files_received == 0:
        from fastapi import HTTPException

        raise HTTPException(status_code=400, detail="No files received")
    return {"files_received": files_received, "total_bytes": total_bytes}


@app.post("/validated/multipart/large")
async def post_multipart_large_validated(request: Request) -> dict[str, Any]:
    """Large multipart form (~100KB)."""
    form = await request.form()
    files_received = 0
    total_bytes = 0
    for key, value in form.items():
        if hasattr(value, "file"):
            files_received += 1
            content = await value.read()
            total_bytes += len(content)
    if files_received == 0:
        from fastapi import HTTPException

        raise HTTPException(status_code=400, detail="No files received")
    return {"files_received": files_received, "total_bytes": total_bytes}


@app.post("/validated/urlencoded/simple")
async def post_urlencoded_simple_validated(request: Request) -> dict[str, Any]:
    """Simple URL-encoded form with Pydantic validation."""
    raw_data = await _parse_urlencoded(request)
    try:
        validated = UrlencodedSimple(**raw_data)
        return validated.model_dump()
    except Exception as e:
        from fastapi import HTTPException

        raise HTTPException(status_code=400, detail=str(e))


@app.post("/validated/urlencoded/complex")
async def post_urlencoded_complex_validated(request: Request) -> dict[str, Any]:
    """Complex URL-encoded form with Pydantic validation."""
    raw_data = await _parse_urlencoded(request)
    try:
        validated = UrlencodedComplex(**raw_data)
        return validated.model_dump()
    except Exception as e:
        from fastapi import HTTPException

        raise HTTPException(status_code=400, detail=str(e))


@app.get("/validated/path/simple/{id}")
async def get_path_simple_validated(id: str) -> dict[str, Any]:
    """Single path parameter with validation."""
    if not id or len(id) > 255 or not id.replace("-", "").replace("_", "").isalnum():
        from fastapi import HTTPException

        raise HTTPException(
            status_code=400,
            detail="Path parameter 'id' must be non-empty, alphanumeric (with - or _), and max 255 characters",
        )
    return {"id": id}


@app.get("/validated/path/multiple/{user_id}/{post_id}")
async def get_path_multiple_validated(user_id: str, post_id: str) -> dict[str, Any]:
    """Multiple path parameters with validation."""
    if not user_id or len(user_id) > 255 or not user_id.replace("-", "").replace("_", "").isalnum():
        from fastapi import HTTPException

        raise HTTPException(
            status_code=400,
            detail="Path parameter 'user_id' must be non-empty, alphanumeric (with - or _), and max 255 characters",
        )
    if not post_id or len(post_id) > 255 or not post_id.replace("-", "").replace("_", "").isalnum():
        from fastapi import HTTPException

        raise HTTPException(
            status_code=400,
            detail="Path parameter 'post_id' must be non-empty, alphanumeric (with - or _), and max 255 characters",
        )
    return {"user_id": user_id, "post_id": post_id}


@app.get("/validated/path/deep/{org}/{team}/{project}/{resource}/{id}")
async def get_path_deep_validated(org: str, team: str, project: str, resource: str, id: str) -> dict[str, Any]:
    """Deep nested path parameters with validation."""
    from fastapi import HTTPException

    for param_name, param_value in [
        ("org", org),
        ("team", team),
        ("project", project),
        ("resource", resource),
        ("id", id),
    ]:
        if not param_value or len(param_value) > 255 or not param_value.replace("-", "").replace("_", "").isalnum():
            raise HTTPException(
                status_code=400,
                detail=f"Path parameter '{param_name}' must be non-empty, alphanumeric (with - or _), and max 255 characters",
            )
    return {
        "org": org,
        "team": team,
        "project": project,
        "resource": resource,
        "id": id,
    }


@app.get("/validated/path/int/{id}")
async def get_path_int_validated(id: int) -> dict[str, Any]:
    """Path parameter with int type."""
    return {"id": id}


@app.get("/validated/path/uuid/{uuid}")
async def get_path_uuid_validated(uuid: UUID) -> dict[str, Any]:
    """Path parameter with UUID."""
    return {"uuid": str(uuid)}


@app.get("/validated/path/date/{date}")
async def get_path_date_validated(date: DateType) -> dict[str, Any]:
    """Path parameter with date."""
    return {"date": date.isoformat()}


@app.get("/validated/query/few")
async def get_query_few_validated(request: Request) -> dict[str, Any]:
    """Few query parameters (1-2) with Pydantic validation."""
    try:
        validated = QueryFew(**dict(request.query_params))
        return validated.model_dump(exclude_none=True)
    except Exception as e:
        from fastapi import HTTPException

        raise HTTPException(status_code=400, detail=str(e))


@app.get("/validated/query/medium")
async def get_query_medium_validated(request: Request) -> dict[str, Any]:
    """Medium query parameters (3-5) with Pydantic validation."""
    try:
        validated = QueryMedium(**dict(request.query_params))
        return validated.model_dump(exclude_none=True)
    except Exception as e:
        from fastapi import HTTPException

        raise HTTPException(status_code=400, detail=str(e))


@app.get("/validated/query/many")
async def get_query_many_validated(request: Request) -> dict[str, Any]:
    """Many query parameters (6-10) with Pydantic validation."""
    try:
        validated = QueryMany(**dict(request.query_params))
        return validated.model_dump(exclude_none=True)
    except Exception as e:
        from fastapi import HTTPException

        raise HTTPException(status_code=400, detail=str(e))


# ============================================================================
# Health & Root Endpoints
# ============================================================================


@app.get("/health")
async def health() -> dict[str, Any]:
    """Health check endpoint."""
    return {"status": "ok"}


@app.get("/")
async def root() -> dict[str, Any]:
    """Root endpoint."""
    return {"status": "ok"}


if __name__ == "__main__":
    from granian import Granian
    from granian.constants import Interfaces

    port = int(sys.argv[1]) if len(sys.argv) > 1 else 8000
    print(
        f"FastAPI + Granian workload server (raw + validated) starting on port {port}",
        file=sys.stderr,
        flush=True,
    )

    Granian(
        "__main__:app",
        address="0.0.0.0",
        port=port,
        interface=Interfaces.ASGI,
        workers=1,
        log_level="error",
    ).serve()
