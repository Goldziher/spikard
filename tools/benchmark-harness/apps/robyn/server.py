"""Robyn benchmark server (raw + validated endpoints).

Robyn is a Rust-based Python web framework with high performance.
Serves both raw endpoints (no validation) and validated endpoints (/validated/... prefix).
Raw endpoints use Request.json() for minimal overhead.
Validated endpoints use Pydantic for validation.
"""

import inspect
import sys
import urllib.parse
from datetime import date
from typing import Any
from uuid import UUID

from pydantic import BaseModel, ValidationError, field_validator
from robyn import Robyn, Request, Response, jsonify

app = Robyn(__file__)


# ============================================================================
# Pydantic Models for Validation
# ============================================================================


class Image(BaseModel):
    url: str
    name: str


class Country(BaseModel):
    name: str
    code: str


class Address(BaseModel):
    street: str
    city: str
    country: Country


class Seller(BaseModel):
    name: str
    address: Address


class SmallPayload(BaseModel):
    name: str
    description: str
    price: float
    tax: float | None = None


class MediumPayload(BaseModel):
    name: str
    price: float
    image: Image


class LargePayload(BaseModel):
    name: str
    price: float
    seller: Seller


class VeryLargePayload(BaseModel):
    name: str
    tags: list[str]
    images: list[Image]


class SimpleUrlencoded(BaseModel):
    name: str
    email: str
    age: int
    subscribe: bool

    @field_validator("age", mode="before")
    @classmethod
    def validate_age(cls, v):
        if isinstance(v, str):
            return int(v)
        return v

    @field_validator("subscribe", mode="before")
    @classmethod
    def validate_subscribe(cls, v):
        if isinstance(v, str):
            return v.lower() in ("true", "1", "yes", "on")
        return v


class ComplexUrlencoded(BaseModel):
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

    @field_validator("age", mode="before")
    @classmethod
    def validate_age(cls, v):
        if isinstance(v, str):
            return int(v)
        return v

    @field_validator(
        "subscribe",
        "newsletter",
        "terms_accepted",
        "privacy_accepted",
        "marketing_consent",
        "two_factor_enabled",
        mode="before",
    )
    @classmethod
    def validate_bool_fields(cls, v):
        if isinstance(v, str):
            return v.lower() in ("true", "1", "yes", "on")
        return v


class QueryFew(BaseModel):
    q: str
    page: int | None = None
    limit: int | None = None

    @field_validator("page", "limit", mode="before")
    @classmethod
    def validate_int_fields(cls, v):
        if v is None:
            return v
        if isinstance(v, str):
            return int(v)
        return v


class QueryMedium(BaseModel):
    """Query parameters validation schema for medium params."""

    search: str
    category: str | None = None
    sort: str | None = None
    order: str | None = None
    page: int | None = None
    limit: int | None = None
    filter: str | None = None

    @field_validator("page", "limit", mode="before")
    @classmethod
    def validate_int_fields_medium(cls, v):
        if v is None:
            return v
        if isinstance(v, str):
            return int(v)
        return v


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

    @field_validator("page", "limit", "rating", mode="before")
    @classmethod
    def validate_int_fields_many(cls, v):
        if v is None:
            return v
        if isinstance(v, str):
            return int(v)
        return v

    @field_validator("min_price", "max_price", mode="before")
    @classmethod
    def validate_float_fields(cls, v):
        if v is None:
            return v
        if isinstance(v, str):
            return float(v)
        return v

    @field_validator("in_stock", "on_sale", mode="before")
    @classmethod
    def validate_bool_fields_many(cls, v):
        if v is None:
            return v
        if isinstance(v, str):
            return v.lower() in ("true", "1", "yes", "on")
        return v


def _dictish(value: Any) -> dict[str, Any]:
    if value is None:
        return {}
    if isinstance(value, dict):
        return value
    if hasattr(value, "items"):
        return {k: v for k, v in value.items()}
    return {}


# ============================================================================
# Raw Endpoints (No Validation)
# ============================================================================


@app.post("/json/small")
async def post_json_small(request: Request):
    """Small JSON payload (~100 bytes) - no validation."""
    body = request.json()
    return jsonify(body)


@app.post("/json/medium")
async def post_json_medium(request: Request):
    """Medium JSON payload (~1KB) - no validation."""
    body = request.json()
    return jsonify(body)


@app.post("/json/large")
async def post_json_large(request: Request):
    """Large JSON payload (~10KB) - no validation."""
    body = request.json()
    return jsonify(body)


@app.post("/json/very-large")
async def post_json_very_large(request: Request):
    """Very large JSON payload (~100KB) - no validation."""
    body = request.json()
    return jsonify(body)


@app.post("/multipart/small")
async def post_multipart_small(request: Request):
    """Small multipart form (~1KB)."""
    files = getattr(request, "files", {})
    files_received = len(files)
    total_bytes = 0
    for file_list in files.values():
        if isinstance(file_list, list):
            for file_obj in file_list:
                if hasattr(file_obj, "file") and hasattr(file_obj.file, "read"):
                    content = file_obj.file.read()
                    total_bytes += len(content)
        elif hasattr(file_list, "file") and hasattr(file_list.file, "read"):
            content = file_list.file.read()
            total_bytes += len(content)
    return jsonify({"files_received": files_received, "total_bytes": total_bytes})


@app.post("/multipart/medium")
async def post_multipart_medium(request: Request):
    """Medium multipart form (~10KB)."""
    files = getattr(request, "files", {})
    files_received = len(files)
    total_bytes = 0
    for file_list in files.values():
        if isinstance(file_list, list):
            for file_obj in file_list:
                if hasattr(file_obj, "file") and hasattr(file_obj.file, "read"):
                    content = file_obj.file.read()
                    total_bytes += len(content)
        elif hasattr(file_list, "file") and hasattr(file_list.file, "read"):
            content = file_list.file.read()
            total_bytes += len(content)
    return jsonify({"files_received": files_received, "total_bytes": total_bytes})


@app.post("/multipart/large")
async def post_multipart_large(request: Request):
    """Large multipart form (~100KB)."""
    files = getattr(request, "files", {})
    files_received = len(files)
    total_bytes = 0
    for file_list in files.values():
        if isinstance(file_list, list):
            for file_obj in file_list:
                if hasattr(file_obj, "file") and hasattr(file_obj.file, "read"):
                    content = file_obj.file.read()
                    total_bytes += len(content)
        elif hasattr(file_list, "file") and hasattr(file_list.file, "read"):
            content = file_list.file.read()
            total_bytes += len(content)
    return jsonify({"files_received": files_received, "total_bytes": total_bytes})


@app.post("/urlencoded/simple")
async def post_urlencoded_simple(request: Request):
    """Simple URL-encoded form - no validation."""
    raw = getattr(request, "body", b"")
    if inspect.isawaitable(raw):
        raw = await raw
    if isinstance(raw, (bytes, bytearray, memoryview)):
        text = bytes(raw).decode("utf-8", errors="replace")
    else:
        text = str(raw or "")
    parsed = urllib.parse.parse_qs(text, keep_blank_values=True, strict_parsing=False)
    body = {k: (v[0] if len(v) == 1 else v) for k, v in parsed.items()}
    return jsonify(body)


@app.post("/urlencoded/complex")
async def post_urlencoded_complex(request: Request):
    """Complex URL-encoded form - no validation."""
    raw = getattr(request, "body", b"")
    if inspect.isawaitable(raw):
        raw = await raw
    if isinstance(raw, (bytes, bytearray, memoryview)):
        text = bytes(raw).decode("utf-8", errors="replace")
    else:
        text = str(raw or "")
    parsed = urllib.parse.parse_qs(text, keep_blank_values=True, strict_parsing=False)
    body = {k: (v[0] if len(v) == 1 else v) for k, v in parsed.items()}
    return jsonify(body)


@app.get("/path/simple/:id")
async def get_path_simple(request: Request):
    """Single path parameter."""
    return jsonify({"id": request.path_params["id"]})


@app.get("/path/multiple/:user_id/:post_id")
async def get_path_multiple(request: Request):
    """Multiple path parameters."""
    return jsonify(
        {
            "user_id": request.path_params["user_id"],
            "post_id": request.path_params["post_id"],
        }
    )


@app.get("/path/deep/:org/:team/:project/:resource/:id")
async def get_path_deep(request: Request):
    """Deep nested path parameters."""
    return jsonify(
        {
            "org": request.path_params["org"],
            "team": request.path_params["team"],
            "project": request.path_params["project"],
            "resource": request.path_params["resource"],
            "id": request.path_params["id"],
        }
    )


@app.get("/path/int/:id")
async def get_path_int(request: Request):
    """Path parameter with int type."""
    return jsonify({"id": int(request.path_params["id"])})


@app.get("/path/uuid/:uuid")
async def get_path_uuid(request: Request):
    """Path parameter with UUID."""
    return jsonify({"uuid": request.path_params["uuid"]})


@app.get("/path/date/:date")
async def get_path_date(request: Request):
    """Path parameter with date."""
    return jsonify({"date": request.path_params["date"]})


@app.get("/query/few")
async def get_query_few(request: Request):
    """Few query parameters (1-2)."""
    return jsonify(_dictish(getattr(request, "query_params", None)))


@app.get("/query/medium")
async def get_query_medium(request: Request):
    """Medium query parameters (3-5)."""
    return jsonify(_dictish(getattr(request, "query_params", None)))


@app.get("/query/many")
async def get_query_many(request: Request):
    """Many query parameters (6-10)."""
    return jsonify(_dictish(getattr(request, "query_params", None)))


# ============================================================================
# Validated Endpoints (using Pydantic for validation)
# ============================================================================


@app.post("/validated/json/small")
async def post_json_small_validated(request: Request):
    """Small JSON payload (~100 bytes) with validation."""
    try:
        body = request.json()
        validated = SmallPayload(**body)
        return jsonify(validated.model_dump())
    except ValidationError as e:
        return Response(
            status_code=400,
            headers={"Content-Type": "application/json"},
            description=jsonify({"error": "Validation failed", "details": e.errors()}),
        )


@app.post("/validated/json/medium")
async def post_json_medium_validated(request: Request):
    """Medium JSON payload (~1KB) with validation."""
    try:
        body = request.json()
        validated = MediumPayload(**body)
        return jsonify(validated.model_dump())
    except ValidationError as e:
        return Response(
            status_code=400,
            headers={"Content-Type": "application/json"},
            description=jsonify({"error": "Validation failed", "details": e.errors()}),
        )


@app.post("/validated/json/large")
async def post_json_large_validated(request: Request):
    """Large JSON payload (~10KB) with validation."""
    try:
        body = request.json()
        validated = LargePayload(**body)
        return jsonify(validated.model_dump())
    except ValidationError as e:
        return Response(
            status_code=400,
            headers={"Content-Type": "application/json"},
            description=jsonify({"error": "Validation failed", "details": e.errors()}),
        )


@app.post("/validated/json/very-large")
async def post_json_very_large_validated(request: Request):
    """Very large JSON payload (~100KB) with validation."""
    try:
        body = request.json()
        validated = VeryLargePayload(**body)
        return jsonify(validated.model_dump())
    except ValidationError as e:
        return Response(
            status_code=400,
            headers={"Content-Type": "application/json"},
            description=jsonify({"error": "Validation failed", "details": e.errors()}),
        )


@app.post("/validated/multipart/small")
async def post_multipart_small_validated(request: Request):
    """Small multipart form (~1KB)."""
    files = getattr(request, "files", {})
    files_received = len(files)
    total_bytes = 0
    for file_list in files.values():
        if isinstance(file_list, list):
            for file_obj in file_list:
                if hasattr(file_obj, "file") and hasattr(file_obj.file, "read"):
                    content = file_obj.file.read()
                    total_bytes += len(content)
        elif hasattr(file_list, "file") and hasattr(file_list.file, "read"):
            content = file_list.file.read()
            total_bytes += len(content)
    if files_received == 0:
        return Response(
            status_code=400,
            headers={"Content-Type": "application/json"},
            description=jsonify({"error": "Validation failed", "details": "No files received"}),
        )
    return jsonify({"files_received": files_received, "total_bytes": total_bytes})


@app.post("/validated/multipart/medium")
async def post_multipart_medium_validated(request: Request):
    """Medium multipart form (~10KB)."""
    files = getattr(request, "files", {})
    files_received = len(files)
    total_bytes = 0
    for file_list in files.values():
        if isinstance(file_list, list):
            for file_obj in file_list:
                if hasattr(file_obj, "file") and hasattr(file_obj.file, "read"):
                    content = file_obj.file.read()
                    total_bytes += len(content)
        elif hasattr(file_list, "file") and hasattr(file_list.file, "read"):
            content = file_list.file.read()
            total_bytes += len(content)
    if files_received == 0:
        return Response(
            status_code=400,
            headers={"Content-Type": "application/json"},
            description=jsonify({"error": "Validation failed", "details": "No files received"}),
        )
    return jsonify({"files_received": files_received, "total_bytes": total_bytes})


@app.post("/validated/multipart/large")
async def post_multipart_large_validated(request: Request):
    """Large multipart form (~100KB)."""
    files = getattr(request, "files", {})
    files_received = len(files)
    total_bytes = 0
    for file_list in files.values():
        if isinstance(file_list, list):
            for file_obj in file_list:
                if hasattr(file_obj, "file") and hasattr(file_obj.file, "read"):
                    content = file_obj.file.read()
                    total_bytes += len(content)
        elif hasattr(file_list, "file") and hasattr(file_list.file, "read"):
            content = file_list.file.read()
            total_bytes += len(content)
    if files_received == 0:
        return Response(
            status_code=400,
            headers={"Content-Type": "application/json"},
            description=jsonify({"error": "Validation failed", "details": "No files received"}),
        )
    return jsonify({"files_received": files_received, "total_bytes": total_bytes})


@app.post("/validated/urlencoded/simple")
async def post_urlencoded_simple_validated(request: Request):
    """Simple URL-encoded form with validation."""
    try:
        raw = getattr(request, "body", b"")
        if inspect.isawaitable(raw):
            raw = await raw
        if isinstance(raw, (bytes, bytearray, memoryview)):
            text = bytes(raw).decode("utf-8", errors="replace")
        else:
            text = str(raw or "")
        parsed = urllib.parse.parse_qs(text, keep_blank_values=True, strict_parsing=False)
        body = {k: (v[0] if len(v) == 1 else v) for k, v in parsed.items()}
        validated = SimpleUrlencoded(**body)
        return jsonify(validated.model_dump())
    except ValidationError as e:
        return Response(
            status_code=400,
            headers={"Content-Type": "application/json"},
            description=jsonify({"error": "Validation failed", "details": e.errors()}),
        )


@app.post("/validated/urlencoded/complex")
async def post_urlencoded_complex_validated(request: Request):
    """Complex URL-encoded form with validation."""
    try:
        raw = getattr(request, "body", b"")
        if inspect.isawaitable(raw):
            raw = await raw
        if isinstance(raw, (bytes, bytearray, memoryview)):
            text = bytes(raw).decode("utf-8", errors="replace")
        else:
            text = str(raw or "")
        parsed = urllib.parse.parse_qs(text, keep_blank_values=True, strict_parsing=False)
        body = {k: (v[0] if len(v) == 1 else v) for k, v in parsed.items()}
        validated = ComplexUrlencoded(**body)
        return jsonify(validated.model_dump())
    except ValidationError as e:
        return Response(
            status_code=400,
            headers={"Content-Type": "application/json"},
            description=jsonify({"error": "Validation failed", "details": e.errors()}),
        )


@app.get("/validated/path/simple/:id")
async def get_path_simple_validated(request: Request):
    """Single path parameter with validation."""
    id_value = request.path_params["id"]
    if not id_value or len(id_value) > 255 or not id_value.replace("-", "").replace("_", "").isalnum():
        return Response(
            status_code=400,
            headers={"Content-Type": "application/json"},
            description=jsonify({"error": "Validation failed", "details": "Path parameter 'id' must be non-empty, alphanumeric (with - or _), and max 255 characters"}),
        )
    return jsonify({"id": id_value})


@app.get("/validated/path/multiple/:user_id/:post_id")
async def get_path_multiple_validated(request: Request):
    """Multiple path parameters with validation."""
    user_id = request.path_params["user_id"]
    post_id = request.path_params["post_id"]
    if not user_id or len(user_id) > 255 or not user_id.replace("-", "").replace("_", "").isalnum():
        return Response(
            status_code=400,
            headers={"Content-Type": "application/json"},
            description=jsonify({"error": "Validation failed", "details": "Path parameter 'user_id' must be non-empty, alphanumeric (with - or _), and max 255 characters"}),
        )
    if not post_id or len(post_id) > 255 or not post_id.replace("-", "").replace("_", "").isalnum():
        return Response(
            status_code=400,
            headers={"Content-Type": "application/json"},
            description=jsonify({"error": "Validation failed", "details": "Path parameter 'post_id' must be non-empty, alphanumeric (with - or _), and max 255 characters"}),
        )
    return jsonify(
        {
            "user_id": user_id,
            "post_id": post_id,
        }
    )


@app.get("/validated/path/deep/:org/:team/:project/:resource/:id")
async def get_path_deep_validated(request: Request):
    """Deep nested path parameters with validation."""
    org = request.path_params["org"]
    team = request.path_params["team"]
    project = request.path_params["project"]
    resource = request.path_params["resource"]
    id_value = request.path_params["id"]

    for param_name, param_value in [("org", org), ("team", team), ("project", project), ("resource", resource), ("id", id_value)]:
        if not param_value or len(param_value) > 255 or not param_value.replace("-", "").replace("_", "").isalnum():
            return Response(
                status_code=400,
                headers={"Content-Type": "application/json"},
                description=jsonify({"error": "Validation failed", "details": f"Path parameter '{param_name}' must be non-empty, alphanumeric (with - or _), and max 255 characters"}),
            )

    return jsonify(
        {
            "org": org,
            "team": team,
            "project": project,
            "resource": resource,
            "id": id_value,
        }
    )


@app.get("/validated/path/int/:id")
async def get_path_int_validated(request: Request):
    """Path parameter with int type and validation."""
    try:
        id_value = int(request.path_params["id"])
        return jsonify({"id": id_value})
    except ValueError:
        return Response(
            status_code=400,
            headers={"Content-Type": "application/json"},
            description=jsonify(
                {"error": "Validation failed", "details": "Invalid integer format"}
            ),
        )


@app.get("/validated/path/uuid/:uuid")
async def get_path_uuid_validated(request: Request):
    """Path parameter with UUID and validation."""
    try:
        uuid_value = UUID(request.path_params["uuid"])
        return jsonify({"uuid": str(uuid_value)})
    except ValueError:
        return Response(
            status_code=400,
            headers={"Content-Type": "application/json"},
            description=jsonify(
                {"error": "Validation failed", "details": "Invalid UUID format"}
            ),
        )


@app.get("/validated/path/date/:date")
async def get_path_date_validated(request: Request):
    """Path parameter with date and validation."""
    try:
        date_value = date.fromisoformat(request.path_params["date"])
        return jsonify({"date": date_value.isoformat()})
    except ValueError:
        return Response(
            status_code=400,
            headers={"Content-Type": "application/json"},
            description=jsonify(
                {"error": "Validation failed", "details": "Invalid ISO date format"}
            ),
        )


@app.get("/validated/query/few")
async def get_query_few_validated(request: Request):
    """Few query parameters (1-2) with validation."""
    try:
        params = _dictish(getattr(request, "query_params", None))
        validated = QueryFew(**params)
        return jsonify(validated.model_dump(exclude_none=True))
    except ValidationError as e:
        return Response(
            status_code=400,
            headers={"Content-Type": "application/json"},
            description=jsonify({"error": "Validation failed", "details": e.errors()}),
        )


@app.get("/validated/query/medium")
async def get_query_medium_validated(request: Request):
    """Medium query parameters (3-5) with Pydantic validation."""
    try:
        params = _dictish(getattr(request, "query_params", None))
        validated = QueryMedium(**params)
        return jsonify(validated.model_dump(exclude_none=True))
    except ValidationError as e:
        return Response(
            status_code=400,
            headers={"Content-Type": "application/json"},
            description=jsonify({"error": "Validation failed", "details": e.errors()}),
        )


@app.get("/validated/query/many")
async def get_query_many_validated(request: Request):
    """Many query parameters (6-10) with Pydantic validation."""
    try:
        params = _dictish(getattr(request, "query_params", None))
        validated = QueryMany(**params)
        return jsonify(validated.model_dump(exclude_none=True))
    except ValidationError as e:
        return Response(
            status_code=400,
            headers={"Content-Type": "application/json"},
            description=jsonify({"error": "Validation failed", "details": e.errors()}),
        )


# ============================================================================
# Health & Root Endpoints
# ============================================================================


@app.get("/health")
async def health():
    """Health check endpoint."""
    return jsonify({"status": "ok"})


@app.get("/")
async def root():
    """Root endpoint."""
    return jsonify({"status": "ok"})


if __name__ == "__main__":
    port = int(sys.argv[1]) if len(sys.argv) > 1 else 8000
    print(f"[robyn] Starting server on port {port}", file=sys.stderr)
    app.start(host="0.0.0.0", port=port)
