"""Generated E2E test application."""

from dataclasses import asdict, dataclass
from datetime import date, datetime
from typing import Any, NamedTuple, TypedDict
from uuid import UUID

import msgspec
from pydantic import BaseModel

from spikard import Spikard, delete, get, head, options, patch, post, put, trace

app = Spikard()


@post(
    "/cookies/samesite-strict",
    body_schema={
        "additionalProperties": False,
        "properties": {"value": {"type": "string"}},
        "required": ["value"],
        "type": "object",
    },
)
def post_cookies_samesitestrict(
    _body: dict[str, Any],
) -> Any:
    """Handler for POST /cookies/samesite-strict."""
    return {"message": "Cookie set with SameSite=Strict"}


class PostCookiesSetwithdomainBody(TypedDict):
    """Request body type (TypedDict - runtime is dict)."""

    value: str


@post(
    "/cookies/set-with-domain",
    body_schema={
        "additionalProperties": False,
        "properties": {"value": {"type": "string"}},
        "required": ["value"],
        "type": "object",
    },
)
def post_cookies_setwithdomain(
    _body: PostCookiesSetwithdomainBody,
) -> Any:
    """Handler for POST /cookies/set-with-domain."""
    return {"message": "Cookie set with domain"}


@get("/headers/accept-encoding")
def get_headers_acceptencoding(
    _accept_encoding: str,
) -> Any:
    """Handler for GET /headers/accept-encoding."""
    return {"accept_encoding": "gzip, deflate, br"}


@get("/headers/accept-language")
def get_headers_acceptlanguage(
    _accept_language: str,
) -> Any:
    """Handler for GET /headers/accept-language."""
    return {"accept_language": "en-US,en;q=0.9"}


@get("/query/optional-default")
def get_query_optionaldefault(
    _limit: int | None = None,
) -> Any:
    """Handler for GET /query/optional-default."""
    return {"limit": 10}


class PostCookiesSamesitenoneBody(BaseModel):
    """Request body Pydantic model."""

    value: str


@post(
    "/cookies/samesite-none",
    body_schema={
        "additionalProperties": False,
        "properties": {"value": {"type": "string"}},
        "required": ["value"],
        "type": "object",
    },
)
def post_cookies_samesitenone(
    _body: PostCookiesSamesitenoneBody,
) -> Any:
    """Handler for POST /cookies/samesite-none."""
    return {"message": "Cookie set with SameSite=None"}


@post(
    "/cookies/set-with-path",
    body_schema={
        "additionalProperties": False,
        "properties": {"value": {"type": "string"}},
        "required": ["value"],
        "type": "object",
    },
)
def post_cookies_setwithpath(
    _body: dict[str, Any],
) -> Any:
    """Handler for POST /cookies/set-with-path."""
    return {"message": "Cookie set with path"}


@get("/download/document.pdf")
def get_download_documentpdf() -> Any:
    """Handler for GET /download/document.pdf."""
    return "pdf_binary_data"


@dataclass
class PostCookiesSamesitelaxBody:
    """Request body dataclass."""

    value: str


@post(
    "/cookies/samesite-lax",
    body_schema={
        "additionalProperties": False,
        "properties": {"value": {"type": "string"}},
        "required": ["value"],
        "type": "object",
    },
)
def post_cookies_samesitelax(
    _body: PostCookiesSamesitelaxBody,
) -> Any:
    """Handler for POST /cookies/samesite-lax."""
    return {"message": "Cookie set with SameSite=Lax"}


@get("/headers/content-type")
def get_headers_contenttype(
    _content_type: str,
) -> Any:
    """Handler for GET /headers/content-type."""
    return {"content_type": "application/json"}


class PostItemsListvalidatedBody(msgspec.Struct):
    """Request body msgspec.Struct (fast typed)."""

    name: str
    tags: list[str]


@post(
    "/items/list-validated",
    body_schema={
        "additionalProperties": False,
        "properties": {"name": {"type": "string"}, "tags": {"items": {}, "type": "array"}},
        "required": ["name", "tags"],
        "type": "object",
    },
)
def post_items_listvalidated(
    body: PostItemsListvalidatedBody,
) -> Any:
    """Handler for POST /items/list-validated."""
    # Echo back parameters for testing
    result: dict[str, Any] = {}
    if body:
        result.update(msgspec.to_builtins(body))
    return result


@get("/query/str-max-length")
def get_query_strmaxlength(
    name: str,
) -> Any:
    """Handler for GET /query/str-max-length."""
    # Echo back parameters for testing
    result: dict[str, Any] = {}
    if name is not None:
        result["name"] = name
    return result


@get("/query/str-min-length")
def get_query_strminlength(
    name: str,
) -> Any:
    """Handler for GET /query/str-min-length."""
    # Echo back parameters for testing
    result: dict[str, Any] = {}
    if name is not None:
        result["name"] = name
    return result


@get("/headers/bearer-auth")
def get_headers_bearerauth(
    _authorization: str,
) -> Any:
    """Handler for GET /headers/bearer-auth."""
    return {"token": "valid_token_123"}


@get("/cookies/min-length")
def get_cookies_minlength(
    _token: str | None = None,
) -> Any:
    """Handler for GET /cookies/min-length."""
    return {"token": "abc"}


@get("/files/document.pdf")
def get_files_documentpdf() -> Any:
    """Handler for GET /files/document.pdf."""
    return "binary_data_1024_bytes"


@get("/headers/basic-auth")
def get_headers_basicauth(
    _authorization: str,
) -> Any:
    """Handler for GET /headers/basic-auth."""
    return {"password": "password", "username": "username"}


@get("/headers/max-length")
def get_headers_maxlength(
    x_session_id: str,
) -> Any:
    """Handler for GET /headers/max-length."""
    # Echo back parameters for testing
    result: dict[str, Any] = {}
    if x_session_id is not None:
        result["x_session_id"] = x_session_id
    return result


@get("/headers/underscore")
def get_headers_underscore(
    _x_token: str,
) -> Any:
    """Handler for GET /headers/underscore."""
    return {"x_token": "secret123"}


class PostItemsOptionalallBody(TypedDict):
    """Request body type (TypedDict - runtime is dict)."""


@post("/items/optional-all", body_schema={"additionalProperties": False, "properties": {}, "type": "object"})
def post_items_optionalall(
    _body: PostItemsOptionalallBody,
) -> Any:
    """Handler for POST /items/optional-all."""
    return {"description": None, "name": None, "price": None, "tax": None}


@get("/query/int/optional")
def get_query_int_optional(
    _query: int | None = None,
) -> Any:
    """Handler for GET /query/int/optional."""
    return "foo bar None"


@get("/query/list-default")
def get_query_listdefault(
    _tags: list[str] | None = None,
) -> Any:
    """Handler for GET /query/list-default."""
    return []


@get("/cookies/validated")
def get_cookies_validated(
    session_id: str,
) -> Any:
    """Handler for GET /cookies/validated."""
    # Echo back parameters for testing
    result: dict[str, Any] = {}
    if session_id is not None:
        result["session_id"] = session_id
    return result


@get("/download/file.bin")
def get_download_filebin() -> Any:
    """Handler for GET /download/file.bin."""
    return "binary_data_placeholder"


@post(
    "/files/images-only",
    body_schema={
        "additionalProperties": False,
        "properties": {"file": {"format": "binary", "type": "string"}},
        "type": "object",
    },
)
def post_files_imagesonly(
    body: dict[str, Any],
) -> Any:
    """Handler for POST /files/images-only."""
    # Echo back parameters for testing
    result: dict[str, Any] = {}
    if body:
        result.update(body)
    return result


@get("/headers/validated")
def get_headers_validated(
    x_token: str,
) -> Any:
    """Handler for GET /headers/validated."""
    # Echo back parameters for testing
    result: dict[str, Any] = {}
    if x_token is not None:
        result["x_token"] = x_token
    return result


@get("/query/int/default")
def get_query_int_default(
    _query: int | None = None,
) -> Any:
    """Handler for GET /query/int/default."""
    return "foo bar 50"


@get("/api/user/profile")
def get_api_user_profile() -> Any:
    """Handler for GET /api/user/profile."""
    return {"username": "john"}


class PostCookiesMultipleBody(msgspec.Struct):
    """Request body msgspec.Struct (fast typed)."""

    session: str
    user: str


@post(
    "/cookies/multiple",
    body_schema={
        "additionalProperties": False,
        "properties": {"session": {"type": "string"}, "user": {"type": "string"}},
        "required": ["user", "session"],
        "type": "object",
    },
)
def post_cookies_multiple(
    _body: PostCookiesMultipleBody,
) -> Any:
    """Handler for POST /cookies/multiple."""
    return {"message": "Multiple cookies set"}


@get("/headers/multiple")
def get_headers_multiple(
    _x_client_version: str,
    _x_request_id: str,
    _x_trace_id: str,
) -> Any:
    """Handler for GET /headers/multiple."""
    return {"x_client_version": "1.2.3", "x_request_id": "req-12345", "x_trace_id": "trace-abc"}


@get("/images/photo.jpg")
def get_images_photojpg() -> Any:
    """Handler for GET /images/photo.jpg."""
    return "jpeg_binary_data"


@get("/query/multi-type")
def get_query_multitype(
    _active: bool,
    _age: int,
    _name: str,
    _score: float,
) -> Any:
    """Handler for GET /query/multi-type."""
    return {"active": True, "age": 30, "name": "john", "score": 95.5}


@dataclass
class PostApiV1ResourceBody:
    """Request body dataclass."""

    data: str


@post(
    "/api/v1/resource", body_schema={"properties": {"data": {"type": "string"}}, "required": ["data"], "type": "object"}
)
def post_api_v1_resource(
    _body: PostApiV1ResourceBody,
) -> Any:
    """Handler for POST /api/v1/resource."""
    return {"data": "value"}


@get("/cookies/pattern")
def get_cookies_pattern(
    _tracking_id: str,
) -> Any:
    """Handler for GET /cookies/pattern."""
    return {"tracking_id": "ABC12345"}


class PostCookiesSessionBody(msgspec.Struct):
    """Request body msgspec.Struct (fast typed)."""

    value: str


@post(
    "/cookies/session",
    body_schema={
        "additionalProperties": False,
        "properties": {"value": {"type": "string"}},
        "required": ["value"],
        "type": "object",
    },
)
def post_cookies_session(
    _body: PostCookiesSessionBody,
) -> Any:
    """Handler for POST /cookies/session."""
    return {"message": "Session cookie set"}


@get("/export/data.csv")
def get_export_datacsv() -> Any:
    """Handler for GET /export/data.csv."""
    return "id,name,price\n1,Item A,10.0\n2,Item B,20.0"


@post(
    "/files/validated",
    body_schema={
        "additionalProperties": False,
        "properties": {"file": {"format": "binary", "type": "string"}},
        "type": "object",
    },
)
def post_files_validated(
    body: dict[str, Any],
) -> Any:
    """Handler for POST /files/validated."""
    # Echo back parameters for testing
    result: dict[str, Any] = {}
    if body:
        result.update(body)
    return result


@get("/headers/pattern")
def get_headers_pattern(
    _x_request_id: str,
) -> Any:
    """Handler for GET /headers/pattern."""
    return {"x_request_id": "12345"}


@get("/headers/referer")
def get_headers_referer(
    _referer: str,
) -> Any:
    """Handler for GET /headers/referer."""
    return {"referer": "https://example.com/page"}


@get("/images/logo.png")
def get_images_logopng() -> Any:
    """Handler for GET /images/logo.png."""
    return "png_binary_data"


class PostItemsBody(msgspec.Struct):
    """Request body msgspec.Struct (fast typed)."""

    name: str
    price: float


@post(
    "/items/?limit=10",
    body_schema={
        "additionalProperties": False,
        "properties": {"name": {"type": "string"}, "price": {"type": "number"}},
        "required": ["name", "price"],
        "type": "object",
    },
)
def post_items(
    _body: PostItemsBody,
    _limit: int,
) -> Any:
    """Handler for POST /items/?limit=10."""
    return {"item": {"name": "Item", "price": 42.0}, "limit": 10}


class PostItemsValidatedBody(BaseModel):
    """Request body Pydantic model."""

    name: str
    price: float


@post(
    "/items/validated",
    body_schema={
        "additionalProperties": False,
        "properties": {"name": {"type": "string"}, "price": {"type": "number"}},
        "required": ["name", "price"],
        "type": "object",
    },
)
def post_items_validated(
    _body: PostItemsValidatedBody,
) -> Any:
    """Handler for POST /items/validated."""
    return {"name": "Item", "price": 100.0}


@post("/cookies/delete")
def post_cookies_delete(
    _session: str | None = None,
) -> Any:
    """Handler for POST /cookies/delete."""
    return {"message": "Cookie deleted"}


class PostFilesDocumentBody(TypedDict):
    """Request body type (TypedDict - runtime is dict)."""

    document: str


@post(
    "/files/document",
    body_schema={
        "additionalProperties": False,
        "properties": {"document": {"format": "binary", "type": "string"}},
        "required": ["document"],
        "type": "object",
    },
)
def post_files_document(
    _body: PostFilesDocumentBody,
) -> Any:
    """Handler for POST /files/document."""
    return {"content_type": "application/pdf", "filename": "report.pdf", "size": 16}


@dataclass
class PostFilesOptionalBody:
    """Request body dataclass."""

    file: str


@post(
    "/files/optional",
    body_schema={
        "additionalProperties": False,
        "properties": {"file": {"format": "binary", "type": "string"}},
        "required": ["file"],
        "type": "object",
    },
)
def post_files_optional(
    _body: PostFilesOptionalBody,
) -> Any:
    """Handler for POST /files/optional."""
    return {"content_type": "text/plain", "filename": "optional.txt", "size": 21}


class PostFilesRequiredBody(NamedTuple):
    """Request body NamedTuple (immutable)."""

    file: str


@post(
    "/files/required",
    body_schema={
        "additionalProperties": False,
        "properties": {"file": {"format": "binary", "type": "string"}},
        "required": ["file"],
        "type": "object",
    },
)
def post_files_required(
    body: PostFilesRequiredBody,
) -> Any:
    """Handler for POST /files/required."""
    # Echo back parameters for testing
    result: dict[str, Any] = {}
    if body:
        result.update(body._asdict())
    return result


class PostFormValidatedBody(msgspec.Struct):
    """Request body msgspec.Struct (fast typed)."""

    username: str


@post(
    "/form/validated",
    body_schema={
        "properties": {"username": {"pattern": "^[a-z0-9_]+$", "type": "string"}},
        "required": ["username"],
        "type": "object",
    },
)
def post_form_validated(
    body: PostFormValidatedBody,
) -> Any:
    """Handler for POST /form/validated."""
    # Echo back parameters for testing
    result: dict[str, Any] = {}
    if body:
        result.update(msgspec.to_builtins(body))
    return result


@get("/headers/accept")
def get_headers_accept(
    _accept: str,
) -> Any:
    """Handler for GET /headers/accept."""
    return {"accept": "application/json"}


@get("/headers/origin")
def get_headers_origin(
    _origin: str,
) -> Any:
    """Handler for GET /headers/origin."""
    return {"origin": "https://example.com"}


@get("/items/negative")
def get_items_negative(
    _offset: int,
) -> Any:
    """Handler for GET /items/negative."""
    return {"offset": -10}


@get("/query/datetime")
def get_query_datetime(
    _timestamp: datetime,
) -> Any:
    """Handler for GET /query/datetime."""
    return {"timestamp": "2024-01-15T10:30:00Z"}


@get("/query/float-ge")
def get_query_floatge(
    _price: float,
) -> Any:
    """Handler for GET /query/float-ge."""
    return {"price": 0.01}


@get("/query/optional")
def get_query_optional(
    _query: str | None = None,
) -> Any:
    """Handler for GET /query/optional."""
    return "foo bar baz"


class PostCalculationsBody(BaseModel):
    """Request body Pydantic model."""

    expected_sum: float
    precise_value: float
    value1: float
    value2: float
    very_large: float
    very_small: float


@post(
    "/calculations/",
    body_schema={
        "additionalProperties": False,
        "properties": {
            "expected_sum": {"type": "number"},
            "precise_value": {"type": "number"},
            "value1": {"type": "number"},
            "value2": {"type": "number"},
            "very_large": {"type": "number"},
            "very_small": {"type": "number"},
        },
        "required": ["value1", "value2", "expected_sum", "precise_value", "very_small", "very_large"],
        "type": "object",
    },
)
def post_calculations(
    _body: PostCalculationsBody,
) -> Any:
    """Handler for POST /calculations/."""
    return {
        "precise_value": 3.141592653589793,
        "sum": 0.30000000000000004,
        "very_large": 1.7976931348623157e308,
        "very_small": 1e-10,
    }


@get("/items/cookies")
def get_items_cookies(
    session_id: str,
    fatebook_tracker: str | None = None,
) -> Any:
    """Handler for GET /items/cookies."""
    # Echo back parameters for testing
    result: dict[str, Any] = {}
    if fatebook_tracker is not None:
        result["fatebook_tracker"] = fatebook_tracker
    if session_id is not None:
        result["session_id"] = session_id
    return result


@get("/items/unicode")
def get_items_unicode() -> Any:
    """Handler for GET /items/unicode."""
    return {"emoji": "☕", "name": "Café"}


@get("/query/pattern")
def get_query_pattern(
    code: str,
) -> Any:
    """Handler for GET /query/pattern."""
    # Echo back parameters for testing
    result: dict[str, Any] = {}
    if code is not None:
        result["code"] = code
    return result


class PostRedirectpostBody(NamedTuple):
    """Request body NamedTuple (immutable)."""


@post("/redirect-post", body_schema={"additionalProperties": False, "properties": {}, "type": "object"})
def post_redirectpost(
    body: PostRedirectpostBody,
) -> Any:
    """Handler for POST /redirect-post."""
    # Echo back parameters for testing
    result: dict[str, Any] = {}
    if body:
        result.update(body._asdict())
    return result


class PostSlowendpointBody(msgspec.Struct):
    """Request body msgspec.Struct (fast typed)."""

    data: str


@post(
    "/slow-endpoint",
    body_schema={
        "additionalProperties": False,
        "properties": {"data": {"type": "string"}},
        "required": ["data"],
        "type": "object",
    },
)
def post_slowendpoint(
    body: PostSlowendpointBody,
) -> Any:
    """Handler for POST /slow-endpoint."""
    # Echo back parameters for testing
    result: dict[str, Any] = {}
    if body:
        result.update(msgspec.to_builtins(body))
    return result


@get("/temp-redirect")
def get_tempredirect() -> Any:
    """Handler for GET /temp-redirect."""
    # Echo back parameters for testing
    result: dict[str, Any] = {}
    return result


@get("/users/me/auth")
def get_users_me_auth(
    key: str,
) -> Any:
    """Handler for GET /users/me/auth."""
    # Echo back parameters for testing
    result: dict[str, Any] = {}
    if key is not None:
        result["key"] = key
    return result


@get("/api/resource")
def get_api_resource() -> Any:
    """Handler for GET /api/resource."""
    # Echo back parameters for testing
    result: dict[str, Any] = {}
    return result


@dataclass
class PostFilesUploadBody:
    """Request body dataclass."""

    file: str


@post(
    "/files/upload",
    body_schema={
        "additionalProperties": False,
        "properties": {"file": {"format": "binary", "type": "string"}},
        "required": ["file"],
        "type": "object",
    },
)
def post_files_upload(
    _body: PostFilesUploadBody,
) -> Any:
    """Handler for POST /files/upload."""
    return {"filename": "empty.txt", "size": 0}


@get("/headers/host")
def get_headers_host(
    _host: str,
) -> Any:
    """Handler for GET /headers/host."""
    return {"host": "example.com:8080"}


class PostItemsNestedBody(msgspec.Struct):
    """Request body msgspec.Struct (fast typed)."""

    name: str
    price: float
    seller: dict[str, Any]


@post(
    "/items/nested",
    body_schema={
        "additionalProperties": False,
        "properties": {
            "name": {"type": "string"},
            "price": {"type": "number"},
            "seller": {
                "additionalProperties": False,
                "properties": {
                    "address": {
                        "additionalProperties": False,
                        "properties": {
                            "city": {"type": "string"},
                            "country": {
                                "additionalProperties": False,
                                "properties": {"code": {"type": "string"}, "name": {"type": "string"}},
                                "required": ["name", "code"],
                                "type": "object",
                            },
                            "street": {"type": "string"},
                        },
                        "required": ["street", "city", "country"],
                        "type": "object",
                    },
                    "name": {"type": "string"},
                },
                "required": ["name", "address"],
                "type": "object",
            },
        },
        "required": ["name", "price", "seller"],
        "type": "object",
    },
)
def post_items_nested(
    _body: PostItemsNestedBody,
) -> Any:
    """Handler for POST /items/nested."""
    return {
        "name": "Product",
        "price": 100.0,
        "seller": {
            "address": {"city": "Springfield", "country": {"code": "US", "name": "USA"}, "street": "123 Main St"},
            "name": "John Doe",
        },
    }


@get("/network/ipv6")
def get_network_ipv6(
    _ip: str,
) -> Any:
    """Handler for GET /network/ipv6."""
    return {"ip": "2001:0db8:85a3:0000:0000:8a2e:0370:7334"}


@get("/query/int-ge")
def get_query_intge(
    _value: int,
) -> Any:
    """Handler for GET /query/int-ge."""
    return {"value": 10}


@get("/query/int-gt")
def get_query_intgt(
    _value: int,
) -> Any:
    """Handler for GET /query/int-gt."""
    return {"value": 1}


@get("/query/int-le")
def get_query_intle(
    _value: int,
) -> Any:
    """Handler for GET /query/int-le."""
    return {"value": 100}


@get("/query/int-lt")
def get_query_intlt(
    _value: int,
) -> Any:
    """Handler for GET /query/int-lt."""
    return {"value": 49}


@get("/admin/users")
def get_admin_users() -> Any:
    """Handler for GET /admin/users."""
    # Echo back parameters for testing
    result: dict[str, Any] = {}
    return result


class PostApiV1DataBody(BaseModel):
    """Request body Pydantic model."""

    data: str
    version: str


@post(
    "/api/v1/data",
    body_schema={
        "properties": {"data": {"type": "string"}, "version": {"const": "1.0", "type": "string"}},
        "required": ["version", "data"],
        "type": "object",
    },
)
def post_api_v1_data(
    body: PostApiV1DataBody,
) -> Any:
    """Handler for POST /api/v1/data."""
    # Echo back parameters for testing
    result: dict[str, Any] = {}
    if body:
        result.update(body.model_dump())
    return result


@post(
    "/files/image",
    body_schema={
        "additionalProperties": False,
        "properties": {"image": {"format": "binary", "type": "string"}},
        "required": ["image"],
        "type": "object",
    },
)
def post_files_image(
    _body: dict[str, Any],
) -> Any:
    """Handler for POST /files/image."""
    return {"content_type": "image/jpeg", "filename": "photo.jpg", "size": 22}


@get("/public/data")
def get_public_data() -> Any:
    """Handler for GET /public/data."""
    return {"data": "public"}


@get("/query/basic")
def get_query_basic(
    _name: str,
) -> Any:
    """Handler for GET /query/basic."""
    return {"name": "test&value=123"}


@get("/cookie/set")
def get_cookie_set() -> Any:
    """Handler for GET /cookie/set."""
    return {"message": "Cookie set"}


class PostFilesListBody(msgspec.Struct):
    """Request body msgspec.Struct (fast typed)."""

    files: list[str]


@post(
    "/files/list",
    body_schema={
        "additionalProperties": False,
        "properties": {"files": {"items": {"format": "binary", "type": "string"}, "type": "array"}},
        "required": ["files"],
        "type": "object",
    },
)
def post_files_list(
    _body: PostFilesListBody,
) -> Any:
    """Handler for POST /files/list."""
    return {"filenames": ["file1.txt", "file2.txt"], "total_size": 35}


@get("/items/json")
def get_items_json() -> Any:
    """Handler for GET /items/json."""
    return {"name": "Item", "price": 42.0}


@post(
    "/items/list",
    body_schema={
        "additionalProperties": False,
        "properties": {
            "images": {
                "items": {
                    "additionalProperties": False,
                    "properties": {"name": {"type": "string"}, "url": {"type": "string"}},
                    "required": ["url", "name"],
                    "type": "object",
                },
                "type": "array",
            },
            "name": {"type": "string"},
            "tags": {"items": {"type": "string"}, "type": "array"},
        },
        "required": ["name", "tags", "images"],
        "type": "object",
    },
)
def post_items_list(
    _body: dict[str, Any],
) -> Any:
    """Handler for POST /items/list."""
    return {
        "images": [
            {"name": "Front", "url": "https://example.com/img1.jpg"},
            {"name": "Back", "url": "https://example.com/img2.jpg"},
        ],
        "name": "Product Bundle",
        "tags": ["electronics", "gadget"],
    }


@get("/query/bool")
def get_query_bool(
    _flag: bool,
) -> Any:
    """Handler for GET /query/bool."""
    return {"flag": True}


@get("/query/date")
def get_query_date(
    _event_date: date,
) -> Any:
    """Handler for GET /query/date."""
    return {"event_date": "2024-01-15"}


@get("/query/enum")
def get_query_enum(
    _model: str,
) -> Any:
    """Handler for GET /query/enum."""
    return {"model": "alexnet"}


@get("/query/list")
def get_query_list(
    _device_ids: list[int],
) -> Any:
    """Handler for GET /query/list."""
    return [1, 2]


@get("/query/uuid")
def get_query_uuid(
    _item_id: UUID,
) -> Any:
    """Handler for GET /query/uuid."""
    return {"item_id": "c892496f-b1fd-4b91-bdb8-b46f92df1716"}


@post("/calculate", body_schema={"properties": {"value": {"type": "number"}}, "required": ["value"], "type": "object"})
def post_calculate(
    _body: dict[str, Any],
) -> Any:
    """Handler for POST /calculate."""
    return {"value": 3.141592653589793}


class PostFormTagsBody(TypedDict):
    """Request body type (TypedDict - runtime is dict)."""

    tags: list[str]


@post(
    "/form/tags",
    body_schema={
        "properties": {"tags": {"items": {"type": "string"}, "type": "array"}},
        "required": ["tags"],
        "type": "object",
    },
)
def post_form_tags(
    _body: PostFormTagsBody,
) -> Any:
    """Handler for POST /form/tags."""
    return {"tags": ["python", "fastapi", "web"]}


@get("/protected")
def get_protected(
    authorization: str,
) -> Any:
    """Handler for GET /protected."""
    # Echo back parameters for testing
    result: dict[str, Any] = {}
    if authorization is not None:
        result["authorization"] = authorization
    return result


@get("/query/int")
def get_query_int(
    _query: int,
) -> Any:
    """Handler for GET /query/int."""
    return "foo bar 42"


class PostRegisterBody(msgspec.Struct):
    """Request body msgspec.Struct (fast typed)."""

    password: str
    username: str
    email: str | None = None


@post(
    "/register/",
    body_schema={
        "properties": {
            "email": {"format": "email", "type": ["string", "null"]},
            "password": {"type": "string"},
            "username": {"type": "string"},
        },
        "required": ["username", "password"],
        "type": "object",
    },
)
def post_register(
    _body: PostRegisterBody,
) -> Any:
    """Handler for POST /register/."""
    return {"email": None, "username": "johndoe"}


@get("/subscribe")
def get_subscribe(
    _email: str,
) -> Any:
    """Handler for GET /subscribe."""
    return {"email": "user@example.com"}


@post(
    "/subscribe",
    body_schema={
        "properties": {"email": {"format": "email", "type": "string"}},
        "required": ["email"],
        "type": "object",
    },
)
def post_subscribe(
    body: dict[str, Any],
) -> Any:
    """Handler for POST /subscribe."""
    # Echo back parameters for testing
    result: dict[str, Any] = {}
    if body:
        result.update(body)
    return result


class PostAccountsBody(TypedDict):
    """Request body type (TypedDict - runtime is dict)."""

    account_id: str


@post(
    "/accounts",
    body_schema={
        "properties": {"account_id": {"pattern": "^ACC-[0-9]{6}$", "type": "string"}},
        "required": ["account_id"],
        "type": "object",
    },
)
def post_accounts(
    body: PostAccountsBody,
) -> Any:
    """Handler for POST /accounts."""
    # Echo back parameters for testing
    result: dict[str, Any] = {}
    if body:
        result.update(body)
    return result


@get("/api/data")
def get_api_data(
    x_api_key: str,
) -> Any:
    """Handler for GET /api/data."""
    # Echo back parameters for testing
    result: dict[str, Any] = {}
    if x_api_key is not None:
        result["x_api_key"] = x_api_key
    return result


@options("/api/data")
def options_api_data(
    access_control_request_headers: str | None = None,
    access_control_request_method: str | None = None,
    origin: str | None = None,
) -> Any:
    """Handler for OPTIONS /api/data."""
    # Echo back parameters for testing
    result: dict[str, Any] = {}
    if access_control_request_headers is not None:
        result["access_control_request_headers"] = access_control_request_headers
    if access_control_request_method is not None:
        result["access_control_request_method"] = access_control_request_method
    if origin is not None:
        result["origin"] = origin
    return result


class PostMessagesBody(msgspec.Struct):
    """Request body msgspec.Struct (fast typed)."""

    text: str


@post(
    "/messages",
    body_schema={
        "properties": {"text": {"maxLength": 100, "minLength": 1, "type": "string"}},
        "required": ["text"],
        "type": "object",
    },
)
def post_messages(
    _body: PostMessagesBody,
) -> Any:
    """Handler for POST /messages."""
    return {"text": "Hello 👋 World 🌍"}


class PostNumbersBody(BaseModel):
    """Request body Pydantic model."""

    large_int: int
    max_safe_int: int
    negative_large: int


@post(
    "/numbers/",
    body_schema={
        "additionalProperties": False,
        "properties": {
            "large_int": {"type": "integer"},
            "max_safe_int": {"type": "integer"},
            "negative_large": {"type": "integer"},
        },
        "required": ["max_safe_int", "large_int", "negative_large"],
        "type": "object",
    },
)
def post_numbers(
    _body: PostNumbersBody,
) -> Any:
    """Handler for POST /numbers/."""
    return {"large_int": 9223372036854775807, "max_safe_int": 9007199254740991, "negative_large": -9223372036854775808}


@get("/old-path")
def get_oldpath() -> Any:
    """Handler for GET /old-path."""
    # Echo back parameters for testing
    result: dict[str, Any] = {}
    return result


class PostProductsBody(TypedDict):
    """Request body type (TypedDict - runtime is dict)."""

    product: str


@post(
    "/products",
    body_schema={
        "definitions": {
            "Product": {
                "properties": {"name": {"type": "string"}, "price": {"minimum": 0, "type": "number"}},
                "required": ["name", "price"],
                "type": "object",
            }
        },
        "properties": {"product": {"$ref": "#/definitions/Product"}},
        "required": ["product"],
        "type": "object",
    },
)
def post_products(
    body: PostProductsBody,
) -> Any:
    """Handler for POST /products."""
    # Echo back parameters for testing
    result: dict[str, Any] = {}
    if body:
        result.update(body)
    return result


@dataclass
class PostProfilesBody:
    """Request body dataclass."""

    profile: dict[str, Any]


@post(
    "/profiles",
    body_schema={
        "properties": {
            "profile": {
                "properties": {
                    "contact": {
                        "properties": {"email": {"format": "email", "type": "string"}},
                        "required": ["email"],
                        "type": "object",
                    }
                },
                "required": ["contact"],
                "type": "object",
            }
        },
        "required": ["profile"],
        "type": "object",
    },
)
def post_profiles(
    body: PostProfilesBody,
) -> Any:
    """Handler for POST /profiles."""
    # Echo back parameters for testing
    result: dict[str, Any] = {}
    if body:
        result.update(asdict(body))
    return result


@get("/redirect")
def get_redirect(
    _url: str,
) -> Any:
    """Handler for GET /redirect."""
    return {"url": "https://example.com/path?query=value"}


class PostRegister1Body(msgspec.Struct):
    """Request body msgspec.Struct (fast typed)."""

    tags: list[str]


@post(
    "/register",
    body_schema={
        "properties": {"tags": {"items": {"type": "string"}, "minItems": 1, "type": "array"}},
        "required": ["tags"],
        "type": "object",
    },
)
def post_register_1(
    _body: PostRegister1Body,
) -> Any:
    """Handler for POST /register."""
    return {"tags": ["python", "rust", "typescript"]}


class PostSettingsBody(BaseModel):
    """Request body Pydantic model."""

    theme: str


@post(
    "/settings",
    body_schema={
        "additionalProperties": False,
        "properties": {"theme": {"enum": ["light", "dark"], "type": "string"}},
        "required": ["theme"],
        "type": "object",
    },
)
def post_settings(
    body: PostSettingsBody,
) -> Any:
    """Handler for POST /settings."""
    # Echo back parameters for testing
    result: dict[str, Any] = {}
    if body:
        result.update(body.model_dump())
    return result


@post(
    "/strings/",
    body_schema={
        "additionalProperties": False,
        "properties": {
            "backslashes": {"type": "string"},
            "empty_string": {"type": "string"},
            "quotes": {"type": "string"},
            "special_chars": {"type": "string"},
            "tabs_newlines": {"type": "string"},
            "unicode_escapes": {"type": "string"},
            "whitespace": {"type": "string"},
        },
        "required": [
            "empty_string",
            "whitespace",
            "tabs_newlines",
            "quotes",
            "backslashes",
            "unicode_escapes",
            "special_chars",
        ],
        "type": "object",
    },
)
def post_strings(
    _body: dict[str, Any],
) -> Any:
    """Handler for POST /strings/."""
    return {
        "backslashes": "C:\\\\Users\\\\Path",
        "empty_string": "",
        "quotes": "He said \"hello\" and 'goodbye'",
        "special_chars": "!@#$%^&*()_+-=[]{}|;':\",./<>?",
        "tabs_newlines": "line1\n\tline2\r\nline3",
        "unicode_escapes": "Hello",
        "whitespace": "   ",
    }


@get("/users/me")
def get_users_me(
    _key: str,
) -> Any:
    """Handler for GET /users/me."""
    return {"username": "secret"}


@dataclass
class PostBillingBody:
    """Request body dataclass."""

    billing_address: str | None = None
    credit_card: str | None = None
    name: str | None = None


@post(
    "/billing",
    body_schema={
        "dependencies": {"credit_card": ["billing_address"]},
        "properties": {
            "billing_address": {"type": "string"},
            "credit_card": {"type": "string"},
            "name": {"type": "string"},
        },
        "type": "object",
    },
)
def post_billing(
    body: PostBillingBody,
) -> Any:
    """Handler for POST /billing."""
    # Echo back parameters for testing
    result: dict[str, Any] = {}
    if body:
        result.update(asdict(body))
    return result


class PostContactBody(NamedTuple):
    """Request body NamedTuple (immutable)."""

    name: str
    email: str | None = None
    phone: str | None = None


@post(
    "/contact",
    body_schema={
        "anyOf": [{"required": ["email"]}, {"required": ["phone"]}],
        "properties": {
            "email": {"format": "email", "type": "string"},
            "name": {"type": "string"},
            "phone": {"type": "string"},
        },
        "required": ["name"],
        "type": "object",
    },
)
def post_contact(
    body: PostContactBody,
) -> Any:
    """Handler for POST /contact."""
    # Echo back parameters for testing
    result: dict[str, Any] = {}
    if body:
        result.update(body._asdict())
    return result


@post("/cookie/")
def post_cookie() -> Any:
    """Handler for POST /cookie/."""
    return {"message": "Come to the dark side, we have cookies"}


class PostEventsBody(BaseModel):
    """Request body Pydantic model."""

    event_date: str
    name: str


@post(
    "/events/",
    body_schema={
        "additionalProperties": False,
        "properties": {"event_date": {"type": "string"}, "name": {"type": "string"}},
        "required": ["name", "event_date"],
        "type": "object",
    },
)
def post_events(
    _body: PostEventsBody,
) -> Any:
    """Handler for POST /events/."""
    return {"event_date": "2024-03-15", "name": "Conference"}


@post(
    "/nested/",
    body_schema={
        "additionalProperties": False,
        "properties": {
            "level1": {
                "additionalProperties": False,
                "properties": {
                    "level2": {
                        "additionalProperties": False,
                        "properties": {
                            "level3": {
                                "additionalProperties": False,
                                "properties": {
                                    "level4": {
                                        "additionalProperties": False,
                                        "properties": {
                                            "level5": {
                                                "additionalProperties": False,
                                                "properties": {
                                                    "level6": {
                                                        "additionalProperties": False,
                                                        "properties": {
                                                            "level7": {
                                                                "additionalProperties": False,
                                                                "properties": {
                                                                    "level8": {
                                                                        "additionalProperties": False,
                                                                        "properties": {
                                                                            "level9": {
                                                                                "additionalProperties": False,
                                                                                "properties": {
                                                                                    "level10": {
                                                                                        "additionalProperties": False,
                                                                                        "properties": {
                                                                                            "depth": {
                                                                                                "type": "integer"
                                                                                            },
                                                                                            "value": {"type": "string"},
                                                                                        },
                                                                                        "required": ["value", "depth"],
                                                                                        "type": "object",
                                                                                    }
                                                                                },
                                                                                "required": ["level10"],
                                                                                "type": "object",
                                                                            }
                                                                        },
                                                                        "required": ["level9"],
                                                                        "type": "object",
                                                                    }
                                                                },
                                                                "required": ["level8"],
                                                                "type": "object",
                                                            }
                                                        },
                                                        "required": ["level7"],
                                                        "type": "object",
                                                    }
                                                },
                                                "required": ["level6"],
                                                "type": "object",
                                            }
                                        },
                                        "required": ["level5"],
                                        "type": "object",
                                    }
                                },
                                "required": ["level4"],
                                "type": "object",
                            }
                        },
                        "required": ["level3"],
                        "type": "object",
                    }
                },
                "required": ["level2"],
                "type": "object",
            }
        },
        "required": ["level1"],
        "type": "object",
    },
)
def post_nested(
    _body: dict[str, Any],
) -> Any:
    """Handler for POST /nested/."""
    return {"max_depth": 10, "message": "Processed deeply nested structure", "value_found": "deep"}


@get("/network")
def get_network(
    _ip: str,
) -> Any:
    """Handler for GET /network."""
    return {"ip": "192.168.1.1"}


@dataclass
class PostPaymentBody:
    """Request body dataclass."""


@post(
    "/payment",
    body_schema={
        "oneOf": [
            {
                "properties": {"credit_card": {"pattern": "^[0-9]{16}$", "type": "string"}},
                "required": ["credit_card"],
                "type": "object",
            },
            {
                "properties": {"paypal_email": {"format": "email", "type": "string"}},
                "required": ["paypal_email"],
                "type": "object",
            },
        ]
    },
)
def post_payment(
    body: PostPaymentBody,
) -> Any:
    """Handler for POST /payment."""
    # Echo back parameters for testing
    result: dict[str, Any] = {}
    if body:
        result.update(asdict(body))
    return result


class PostProfileBody(NamedTuple):
    """Request body NamedTuple (immutable)."""

    user: dict[str, Any]


@post(
    "/profile",
    body_schema={
        "properties": {
            "user": {
                "properties": {
                    "age": {"minimum": 0, "type": "integer"},
                    "email": {"format": "email", "type": "string"},
                    "name": {"minLength": 1, "type": "string"},
                },
                "required": ["name", "email"],
                "type": "object",
            }
        },
        "required": ["user"],
        "type": "object",
    },
)
def post_profile(
    _body: PostProfileBody,
) -> Any:
    """Handler for POST /profile."""
    return {"user": {"age": 30, "email": "john@example.com", "name": "John Doe"}}


class PostConfigBody(msgspec.Struct):
    """Request body msgspec.Struct (fast typed)."""


@post("/config", body_schema={"maxProperties": 3, "type": "object"})
def post_config(
    body: PostConfigBody,
) -> Any:
    """Handler for POST /config."""
    # Echo back parameters for testing
    result: dict[str, Any] = {}
    if body:
        result.update(msgspec.to_builtins(body))
    return result


@get("/health")
def get_health() -> Any:
    """Handler for GET /health."""
    # Echo back parameters for testing
    result: dict[str, Any] = {}
    return result


@get("/items/")
def get_items(
    _item_query: str,
) -> Any:
    """Handler for GET /items/."""
    return {"item_query": "fixedquery"}


@options("/items/")
def options_items() -> Any:
    """Handler for OPTIONS /items/."""
    # Echo back parameters for testing
    result: dict[str, Any] = {}
    return result


@dataclass
class PostItems1Body:
    """Request body dataclass."""


@post("/items/", body_schema={"type": "string"})
def post_items_1(
    _body: PostItems1Body,
) -> Any:
    """Handler for POST /items/."""
    return {"in_stock": True, "name": "Item", "price": 42.0}


class PostLoginBody(NamedTuple):
    """Request body NamedTuple (immutable)."""

    password: str
    username: str


@post(
    "/login/",
    body_schema={
        "properties": {"password": {"type": "string"}, "username": {"type": "string"}},
        "required": ["username", "password"],
        "type": "object",
    },
)
def post_login(
    _body: PostLoginBody,
) -> Any:
    """Handler for POST /login/."""
    return {"username": "johndoe"}


class PostNullsBody(msgspec.Struct):
    """Request body msgspec.Struct (fast typed)."""

    empty_array: list[str]
    empty_object: dict[str, Any]
    empty_string: str
    explicit_null: Any
    false_boolean: bool
    zero_number: int


@post(
    "/nulls/",
    body_schema={
        "additionalProperties": False,
        "properties": {
            "empty_array": {"items": {}, "type": "array"},
            "empty_object": {"additionalProperties": False, "properties": {}, "type": "object"},
            "empty_string": {"type": "string"},
            "explicit_null": {"type": "null"},
            "false_boolean": {"type": "boolean"},
            "zero_number": {"type": "integer"},
        },
        "required": ["explicit_null", "empty_string", "empty_array", "empty_object", "zero_number", "false_boolean"],
        "type": "object",
    },
)
def post_nulls(
    _body: PostNullsBody,
) -> Any:
    """Handler for POST /nulls/."""
    return {
        "empty_array_length": 0,
        "empty_object_keys": 0,
        "empty_string_length": 0,
        "explicit_null_is_null": True,
        "false_is_false": True,
        "zero_is_falsy": True,
    }


@get("/search")
def get_search(
    _term: str,
) -> Any:
    """Handler for GET /search."""
    return {"term": "foo"}


@get("/secure")
def get_secure(
    session: str,
) -> Any:
    """Handler for GET /secure."""
    # Echo back parameters for testing
    result: dict[str, Any] = {}
    if session is not None:
        result["session"] = session
    return result


class PostTasksBody(TypedDict):
    """Request body type (TypedDict - runtime is dict)."""

    task: str


@post(
    "/tasks/",
    body_schema={
        "additionalProperties": False,
        "properties": {"task": {"type": "string"}},
        "required": ["task"],
        "type": "object",
    },
)
def post_tasks(
    _body: PostTasksBody,
) -> Any:
    """Handler for POST /tasks/."""
    return {"message": "Task accepted for processing", "task_id": "abc123"}


@post("/upload")
def post_upload() -> Any:
    """Handler for POST /upload."""
    # Echo back parameters for testing
    result: dict[str, Any] = {}
    return result


@get("/error")
def get_error() -> Any:
    """Handler for GET /error."""
    # Echo back parameters for testing
    result: dict[str, Any] = {}
    return result


class PostFilesBody(msgspec.Struct):
    """Request body msgspec.Struct (fast typed)."""

    filename: str


@post(
    "/files",
    body_schema={
        "properties": {"filename": {"pattern": "^[^\\x00]+$", "type": "string"}},
        "required": ["filename"],
        "type": "object",
    },
)
def post_files(
    body: PostFilesBody,
) -> Any:
    """Handler for POST /files."""
    # Echo back parameters for testing
    result: dict[str, Any] = {}
    if body:
        result.update(msgspec.to_builtins(body))
    return result


class PostFormBody(BaseModel):
    """Request body Pydantic model."""

    username: str
    age: int | None = None


@post(
    "/form/",
    body_schema={
        "properties": {"age": {"type": "integer"}, "username": {"type": "string"}},
        "required": ["username"],
        "type": "object",
    },
)
def post_form(
    _body: PostFormBody,
) -> Any:
    """Handler for POST /form/."""
    return {"age": 30, "username": "johndoe"}


@get("/items")
def get_items_1(
    _limit: int,
) -> Any:
    """Handler for GET /items."""
    return {"limit": 5}


class PostItems2Body(TypedDict):
    """Request body type (TypedDict - runtime is dict)."""


@post(
    "/items",
    body_schema={
        "allOf": [
            {"properties": {"name": {"type": "string"}}, "required": ["name"], "type": "object"},
            {"properties": {"price": {"minimum": 0, "type": "number"}}, "required": ["price"], "type": "object"},
        ]
    },
)
def post_items_2(
    body: PostItems2Body,
) -> Any:
    """Handler for POST /items."""
    # Echo back parameters for testing
    result: dict[str, Any] = {}
    if body:
        result.update(body)
    return result


@get("/query")
def get_query(
    _query: str,
) -> Any:
    """Handler for GET /query."""
    return "foo bar baz"


@get("/stats")
def get_stats(
    _threshold: float,
) -> Any:
    """Handler for GET /stats."""
    return {"threshold": 0.0015}


class PostTokenBody(msgspec.Struct):
    """Request body msgspec.Struct (fast typed)."""

    grant_type: str
    password: str
    username: str
    scope: str | None = None


@post(
    "/token",
    body_schema={
        "properties": {
            "grant_type": {"type": "string"},
            "password": {"type": "string"},
            "scope": {"type": "string"},
            "username": {"type": "string"},
        },
        "required": ["username", "password", "grant_type"],
        "type": "object",
    },
)
def post_token(
    _body: PostTokenBody,
) -> Any:
    """Handler for POST /token."""
    return {"access_token": "johndoe", "token_type": "bearer"}


class PostUsersBody(BaseModel):
    """Request body Pydantic model."""

    username: str


@post(
    "/users",
    body_schema={
        "properties": {"username": {"not": {"enum": ["admin", "root", "system"]}, "type": "string"}},
        "required": ["username"],
        "type": "object",
    },
)
def post_users(
    _body: PostUsersBody,
) -> Any:
    """Handler for POST /users."""
    return {"name": "café"}


@get("/data")
def get_data(
    _tracking: str,
) -> Any:
    """Handler for GET /data."""
    return {"value": 123}


class PostDataBody(TypedDict):
    """Request body type (TypedDict - runtime is dict)."""

    name: str


@post("/data", body_schema={"properties": {"name": {"type": "string"}}, "required": ["name"], "type": "object"})
def post_data(
    _body: PostDataBody,
) -> Any:
    """Handler for POST /data."""
    return {"name": "test"}


@trace("/data")
def trace_data() -> Any:
    """Handler for TRACE /data."""
    # Echo back parameters for testing
    result: dict[str, Any] = {}
    return result


class PostEchoBody(NamedTuple):
    """Request body NamedTuple (immutable)."""

    test: str


@post(
    "/echo",
    body_schema={
        "additionalProperties": False,
        "properties": {"test": {"type": "string"}},
        "required": ["test"],
        "type": "object",
    },
)
def post_echo(
    _body: PostEchoBody,
) -> Any:
    """Handler for POST /echo."""
    return {
        "content_type_lower": "application/json",
        "content_type_mixed": "application/json",
        "content_type_upper": "application/json",
    }


@get("/html")
def get_html() -> Any:
    """Handler for GET /html."""
    return "<html><body><h1>Hello</h1></body></html>"


class PostTagsBody(BaseModel):
    """Request body Pydantic model."""

    tags: list[str]


@post(
    "/tags",
    body_schema={
        "properties": {"tags": {"items": {"type": "string"}, "minItems": 2, "type": "array"}},
        "required": ["tags"],
        "type": "object",
    },
)
def post_tags(
    body: PostTagsBody,
) -> Any:
    """Handler for POST /tags."""
    # Echo back parameters for testing
    result: dict[str, Any] = {}
    if body:
        result.update(body.model_dump())
    return result


@get("/test")
def get_test(
    _email: str,
    _special: str,
) -> Any:
    """Handler for GET /test."""
    return {"email": "x@test.com", "special": "&@A.ac"}


@get("/text")
def get_text() -> Any:
    """Handler for GET /text."""
    return "Hello, World!"


@dataclass
class PostTextBody:
    """Request body dataclass."""

    content: str


@post(
    "/text",
    body_schema={
        "properties": {"content": {"maxLength": 10000, "type": "string"}},
        "required": ["content"],
        "type": "object",
    },
)
def post_text(
    body: PostTextBody,
) -> Any:
    """Handler for POST /text."""
    # Echo back parameters for testing
    result: dict[str, Any] = {}
    if body:
        result.update(asdict(body))
    return result


@get("/dns")
def get_dns(
    _host: str,
) -> Any:
    """Handler for GET /dns."""
    return {"host": "api.example.com"}


@get("/xml")
def get_xml() -> Any:
    """Handler for GET /xml."""
    return '<?xml version="1.0"?><item><name>Item</name><price>42.0</price></item>'


class PostRootBody(BaseModel):
    """Request body Pydantic model."""

    files: list[str]
    tags: list[str] | None = None


@post(
    "/",
    body_schema={
        "additionalProperties": False,
        "properties": {
            "files": {"items": {"format": "binary", "type": "string"}, "type": "array"},
            "tags": {"items": {"type": "string"}, "type": "array"},
        },
        "required": ["files"],
        "type": "object",
    },
)
def post_root(
    _body: PostRootBody,
) -> Any:
    """Handler for POST /."""
    return {
        "files": [
            {"content": "first file", "content_type": "text/plain", "filename": "file1.txt", "size": 10},
            {"content": "second file", "content_type": "text/plain", "filename": "file2.txt", "size": 11},
        ],
        "tags": ["python", "rust", "web"],
    }


@get("/type-syntax/items-count/{id}")
def get_typesyntax_itemscount_id(
    _count: int,
) -> Any:
    """Handler for GET /type-syntax/items-count/{id}."""
    return {"count": "50"}


@get("/path/param-maxlength/{id}")
def get_path_parammaxlength_id(
    item_id: str,
) -> Any:
    """Handler for GET /path/param-maxlength/{id}."""
    # Echo back parameters for testing
    result: dict[str, Any] = {}
    if item_id is not None:
        result["item_id"] = item_id
    return result


@get("/path/param-minlength/{id}")
def get_path_paramminlength_id(
    item_id: str,
) -> Any:
    """Handler for GET /path/param-minlength/{id}."""
    # Echo back parameters for testing
    result: dict[str, Any] = {}
    if item_id is not None:
        result["item_id"] = item_id
    return result


@get("/type-syntax/items/{id}")
def get_typesyntax_items_id(
    _id_: str,
) -> Any:
    """Handler for GET /type-syntax/items/{id}."""
    return {"id": "550e8400-e29b-41d4-a716-446655440000"}


@get("/type-syntax/users/{id}")
def get_typesyntax_users_id(
    _user_id: str,
) -> Any:
    """Handler for GET /type-syntax/users/{id}."""
    return {"user_id": "42"}


@get("/path/param-lt-gt/{id}")
def get_path_paramltgt_id(
    _item_id: int,
) -> Any:
    """Handler for GET /path/param-lt-gt/{id}."""
    return {"item_id": 2}


@get("/{id}/{id}/{id}/{id}")
def get_id_id_id_id(
    _order_id: UUID,
    _service_id: int,
    _user_id: str,
    _version: float,
) -> Any:
    """Handler for GET /{id}/{id}/{id}/{id}."""
    return {"order_id": "c892496f-b1fd-4b91-bdb8-b46f92df1716", "service_id": 1, "user_id": "abc", "version": 1.0}


@get("/path/param-ge/{id}")
def get_path_paramge_id(
    _item_id: int,
) -> Any:
    """Handler for GET /path/param-ge/{id}."""
    return {"item_id": 3}


@get("/path/param-gt/{id}")
def get_path_paramgt_id(
    _item_id: int,
) -> Any:
    """Handler for GET /path/param-gt/{id}."""
    return {"item_id": 42}


@get("/path/param-le/{id}")
def get_path_paramle_id(
    _item_id: int,
) -> Any:
    """Handler for GET /path/param-le/{id}."""
    return {"item_id": 3}


@get("/path/param-lt/{id}")
def get_path_paramlt_id(
    _item_id: int,
) -> Any:
    """Handler for GET /path/param-lt/{id}."""
    return {"item_id": 2}


@get("/accept-test/{id}")
def get_accepttest_id(
    _id_: str,
) -> Any:
    """Handler for GET /accept-test/{id}."""
    return {"id": 1, "name": "Item"}


@delete("/status-test/{id}")
def delete_statustest_id(
    code: str,
) -> Any:
    """Handler for DELETE /status-test/{id}."""
    # Echo back parameters for testing
    result: dict[str, Any] = {}
    if code is not None:
        result["code"] = code
    return result


@get("/status-test/{id}")
def get_statustest_id(
    _code: str,
) -> Any:
    """Handler for GET /status-test/{id}."""
    return {"id": 1, "name": "Item 1"}


@get("/path/float/{id}")
def get_path_float_id(
    _item_id: float,
) -> Any:
    """Handler for GET /path/float/{id}."""
    return {"item_id": 42.5}


@get("/repos/{id}/{id}")
def get_repos_id_id(
    _owner: str,
    _repo: str,
) -> Any:
    """Handler for GET /repos/{id}/{id}."""
    return {"owner": "spikard-labs", "repo": "spikard-http"}


@get("/path/bool/{id}")
def get_path_bool_id(
    _item_id: bool,
) -> Any:
    """Handler for GET /path/bool/{id}."""
    return {"item_id": True}


@get("/bookings/{id}")
def get_bookings_id(
    _timestamp: datetime,
) -> Any:
    """Handler for GET /bookings/{id}."""
    return {"timestamp": "2025-10-30T14:30:00Z"}


@get("/path/int/{id}")
def get_path_int_id(
    _item_id: int,
) -> Any:
    """Handler for GET /path/int/{id}."""
    return {"item_id": 42}


@get("/path/str/{id}")
def get_path_str_id(
    _item_id: str,
) -> Any:
    """Handler for GET /path/str/{id}."""
    return {"item_id": "foobar"}


@get("/delays/{id}")
def get_delays_id(
    _duration: str,
) -> Any:
    """Handler for GET /delays/{id}."""
    return {"duration": "P1DT2H30M"}


@get("/events/{id}")
def get_events_id(
    _date: date,
) -> Any:
    """Handler for GET /events/{id}."""
    return {"date": "2025-10-30"}


@get("/models/{id}")
def get_models_id(
    _model_name: str,
) -> Any:
    """Handler for GET /models/{id}."""
    return {"model_name": "alexnet"}


@get("/offset/{id}")
def get_offset_id(
    _value: int,
) -> Any:
    """Handler for GET /offset/{id}."""
    return {"value": -100}


@get("/prices/{id}")
def get_prices_id(
    _amount: str,
) -> Any:
    """Handler for GET /prices/{id}."""
    return {"amount": "19.99"}


@get("/files/{id}")
def get_files_id(
    _file_path: str,
) -> Any:
    """Handler for GET /files/{id}."""
    return {"file_path": "home/johndoe/myfile.txt"}


@delete("/items/{id}")
def delete_items_id(
    _id_: str,
) -> Any:
    """Handler for DELETE /items/{id}."""
    return {}


@get("/items/{id}")
def get_items_id(
    _id_: UUID,
) -> Any:
    """Handler for GET /items/{id}."""
    return {"id": "e8b5a51d-11c8-3310-a6ab-367563f20686"}


@head("/items/{id}")
def head_items_id(
    id_: str,
) -> Any:
    """Handler for HEAD /items/{id}."""
    # Echo back parameters for testing
    result: dict[str, Any] = {}
    if id_ is not None:
        result["id_"] = id_
    return result


class PatchItemsIdBody(BaseModel):
    """Request body Pydantic model."""

    price: float


@patch(
    "/items/{id}", body_schema={"properties": {"price": {"type": "number"}}, "required": ["price"], "type": "object"}
)
def patch_items_id(
    _body: PatchItemsIdBody,
    _id_: str,
) -> Any:
    """Handler for PATCH /items/{id}."""
    return {"description": "Original description", "name": "Original Item", "price": 45.0}


@put(
    "/items/{id}",
    body_schema={
        "properties": {"id": {"type": "integer"}, "name": {"type": "string"}, "price": {"type": "number"}},
        "required": ["id", "name", "price"],
        "type": "object",
    },
)
def put_items_id(
    _body: dict[str, Any],
    _id_: str,
) -> Any:
    """Handler for PUT /items/{id}."""
    return {"id": 999, "name": "New Item", "price": 49.99}


@get("/repos/{id}")
def get_repos_id(
    owner: str,
) -> Any:
    """Handler for GET /repos/{id}."""
    # Echo back parameters for testing
    result: dict[str, Any] = {}
    if owner is not None:
        result["owner"] = owner
    return result


@get("/users/{id}")
def get_users_id(
    _username: str,
) -> Any:
    """Handler for GET /users/{id}."""
    return {"username": "alice"}


@get("/date/{id}")
def get_date_id(
    _date_param: date,
) -> Any:
    """Handler for GET /date/{id}."""
    return {"date_param": "2023-07-15"}


if __name__ == "__main__":
    app.run()
