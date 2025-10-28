"""Test application implementing endpoints for all fixture categories.

This app is designed to work with the test fixtures in testing_data/.
It implements endpoints that match the fixture expectations for comprehensive testing.
"""

# mypy: ignore-errors

from datetime import date, datetime
from enum import Enum
from typing import Any
from uuid import UUID

from pydantic import BaseModel, Field

from spikard import Cookie, Header, Query, Response

# ============================================================================
# Query Parameters Models
# ============================================================================


class StatusEnum(str, Enum):
    active = "active"
    inactive = "inactive"
    pending = "pending"


class ModelName(str, Enum):
    alexnet = "alexnet"
    resnet = "resnet"
    lenet = "lenet"


def query_params_app():
    """Create app for query parameter fixtures."""
    from spikard import Spikard

    app = Spikard()

    # Route: /query - required string
    @app.get("/query")
    def query_string(query: str):
        """Required string query parameter."""
        return f"foo bar {query}"

    # Route: /query/int - required int
    @app.get("/query/int")
    def query_int(query: int):
        """Required int query parameter."""
        return f"foo bar {query}"

    # Route: /query/optional - optional string
    @app.get("/query/optional")
    def query_optional_string(query: str | None = None):
        """Optional string query parameter."""
        return f"foo bar {query}"

    # Route: /query/int/optional - optional int
    @app.get("/query/int/optional")
    def query_optional_int(query: int | None = None):
        """Optional int query parameter."""
        return f"foo bar {query}"

    # Route: /query/int/default - int with default
    @app.get("/query/int/default")
    def query_int_default(query: int = 10):
        """Int query parameter with default value."""
        return f"foo bar {query}"

    # Route: /query/list - list query parameters
    #
    # This endpoint has TWO separate parameters testing different scenarios:
    # - device_ids: required list[int] (tests #12, #13 - when missing should 422)
    # - items: optional list[str] with default [] (tests #32, #33)
    #
    # NOTE: The test expectations are contradictory - when NO params provided:
    #   Test #13 (device_ids focus) expects 422 (required param missing)
    #   Test #32 (items focus) expects 200 with {"items": []}
    #
    # Since we can't make both true simultaneously, for now keep the original behavior
    # where both are optional. Test #13 will need fixing at the test fixture level.
    @app.get("/query/list")
    def query_list(device_ids: list[int] | None = None, items: list[str] = Query(default_factory=list)):
        """List query parameters - both optional but tested independently."""
        # When device_ids is provided (tests #12, #13)
        if device_ids is not None:
            return device_ids
        # When items is provided OR empty (tests #32, #33)
        # items has default [], so always returns {"items": [...]}
        return {"items": items}

    # Route: /query/list-default - optional list with default
    @app.get("/query/list-default")
    def query_list_default(tags: list[str] = Query(default_factory=list)):
        """List query parameter with default empty list."""
        return tags

    # Route: /items/ - multiple query params
    @app.get("/items/")
    def get_items(
        q: list[str] | None = None,
        item_query: str | None = Field(None, pattern=r"^fixedquery$"),
    ):
        """Generic endpoint for query parameter testing."""
        result = {}
        if q is not None:
            result["q"] = q
        if item_query is not None:
            result["item_query"] = item_query
        return result

    # Route: /test - special chars
    @app.get("/test")
    def test_special_chars(email: str, special: str):
        """Test special character encoding."""
        return {"email": email, "special": special}

    # Route: /query/bool - bool param
    @app.get("/query/bool")
    def query_bool(flag: bool):
        """Boolean query parameter."""
        return {"flag": flag}

    # Route: /query/uuid - UUID param
    @app.get("/query/uuid")
    def query_uuid(item_id: UUID):
        """UUID query parameter."""
        return {"item_id": str(item_id)}

    # Route: /query/enum - enum param
    @app.get("/query/enum")
    def query_enum(model: ModelName):
        """Enum query parameter."""
        return {"model": model}

    # Route: /query/int-ge - int with ge constraint
    @app.get("/query/int-ge")
    def query_int_ge(value: int = Field(ge=10)):
        """Int with >= constraint."""
        return {"value": value}

    # Route: /query/int-le - int with le constraint
    @app.get("/query/int-le")
    def query_int_le(value: int = Field(le=100)):
        """Int with <= constraint."""
        return {"value": value}

    # Route: /query/int-lt - int with lt constraint
    @app.get("/query/int-lt")
    def query_int_lt(value: int = Field(lt=50)):
        """Int with < constraint."""
        return {"value": value}

    # Route: /query/int-gt - int with gt constraint
    @app.get("/query/int-gt")
    def query_int_gt(value: int = Field(gt=0)):
        """Int with > constraint."""
        return {"value": value}

    # Route: /query/str-max-length - string with max length
    @app.get("/query/str-max-length")
    def query_str_max_length(name: str = Field(max_length=10)):
        """String with max length constraint."""
        return {"name": name}

    # Route: /query/str-min-length - string with min length
    @app.get("/query/str-min-length")
    def query_str_min_length(name: str = Field(min_length=3)):
        """String with min length constraint."""
        return {"name": name}

    # Route: /query/date - date param
    @app.get("/query/date")
    def query_date(event_date: date):
        """Date query parameter."""
        return {"event_date": event_date.isoformat()}

    # Route: /query/datetime - datetime param
    @app.get("/query/datetime")
    def query_datetime(timestamp: datetime):
        """Datetime query parameter."""
        return {"timestamp": timestamp.isoformat()}

    # Route: /query/pattern - regex pattern validation
    @app.get("/query/pattern")
    def query_pattern(code: str = Field(pattern=r"^[0-9]{3,}$")):
        """String with regex pattern constraint."""
        return {"code": code}

    # Route: /query/basic - basic string param
    @app.get("/query/basic")
    def query_basic(name: str):
        """Basic string query parameter."""
        return {"name": name}

    # Route: /query/multi-type - multiple different types
    @app.get("/query/multi-type")
    def query_multi_type(name: str, age: int, active: bool, score: float):
        """Multiple query parameters of different types."""
        return {"name": name, "age": age, "active": active, "score": score}

    # Route: /query/optional-default - optional with default
    @app.get("/query/optional-default")
    def query_optional_default(limit: int = 10):
        """Optional query parameter with default value."""
        return {"limit": limit}

    # Route: /query/float-ge - float with ge constraint
    @app.get("/query/float-ge")
    def query_float_ge(price: float = Field(ge=0.01)):
        """Float with >= constraint."""
        return {"price": price}

    @app.get("/users/{user_id}")
    def get_user(user_id: int = Field(gt=0)):
        """Endpoint with validated path param and query params."""
        return {"user_id": user_id}

    return app


# ============================================================================
# Headers Models and App
# ============================================================================


def headers_app():
    """Create app for header fixtures."""
    import base64

    from spikard import Spikard

    app = Spikard()

    # Route: /items/ - User-Agent, X-Token, strange_header
    @app.get("/items/")
    def get_items_headers(
        user_agent: str | None = Header("testclient"),
        x_token: list[str] | None = Header(None),
        strange_header: str | None = Header(None),
    ):
        """Header testing for /items/ route."""
        result = {}
        if user_agent is not None:
            result["User-Agent"] = user_agent
        if x_token is not None:
            result["X-Token values"] = x_token
        if strange_header is None:
            result["strange_header"] = None
        else:
            result["strange_header"] = strange_header
        return result

    # Route: /users/me - Auth endpoints (X-API-Key required)
    @app.get("/users/me")
    def get_user_me(
        key: str | None = Header(None),
        authorization: str | None = Header(None),
    ):
        """Multiple auth variations for /users/me."""
        # Check for required X-API-Key (test 03, 04)
        if key == "secret":
            return {"username": "secret"}

        # Check for optional X-API-Key (test 05, 06)
        if key is not None and key != "secret":
            return {"msg": f"Hello {key}"}
        if key is None and authorization is None:
            return {"msg": "Hello World"}

        # Check for Authorization header (test 07, 08, 09)
        if authorization:
            if authorization.startswith("Digest "):
                scheme = "Digest"
                credentials = authorization.replace("Digest ", "")
                return {"scheme": scheme, "credentials": credentials}
            if authorization.startswith("Other "):
                return Response(content={"detail": "Invalid authentication credentials"}, status_code=403)
            return Response(content={"detail": "Not authenticated"}, status_code=403)

        # No auth provided
        return Response(content={"detail": "Not authenticated"}, status_code=403)

    # Route: /echo - Case insensitive header access
    @app.get("/echo")
    def echo_headers(
        content_type: str | None = Header(None, alias="content-type"),
    ):
        """Test case insensitive header access."""
        return {
            "content_type_lower": content_type,
            "content_type_upper": content_type,
            "content_type_mixed": content_type,
        }

    # Route: /headers/content-type
    @app.get("/headers/content-type")
    def content_type_header(
        content_type: str = Header(..., alias="Content-Type"),
    ):
        """Content-Type header test."""
        return {"content_type": content_type}

    # Route: /headers/accept
    @app.get("/headers/accept")
    def accept_header(
        accept: str = Header(...),
    ):
        """Accept header test."""
        return {"accept": accept}

    # Route: /headers/bearer-auth
    @app.get("/headers/bearer-auth")
    def bearer_auth(
        authorization: str | None = Header(None),
    ):
        """Bearer token authentication."""
        if not authorization or not authorization.startswith("Bearer "):
            return Response(content={"detail": "Not authenticated"}, status_code=401)
        token = authorization.replace("Bearer ", "")
        return {"token": token}

    # Route: /headers/multiple
    @app.get("/headers/multiple")
    def multiple_headers(
        x_request_id: str = Header(...),
        x_client_version: str = Header(...),
        x_trace_id: str = Header(...),
    ):
        """Multiple custom headers."""
        return {
            "x_request_id": x_request_id,
            "x_client_version": x_client_version,
            "x_trace_id": x_trace_id,
        }

    # Route: /headers/validated
    @app.get("/headers/validated")
    def validated_header(
        x_token: str = Header(..., min_length=3),
    ):
        """Header with validation."""
        return {"x_token": x_token}

    # Route: /headers/accept-language
    @app.get("/headers/accept-language")
    def accept_language_header(
        accept_language: str = Header(...),
    ):
        """Accept-Language header test."""
        return {"accept_language": accept_language}

    # Route: /headers/referer
    @app.get("/headers/referer")
    def referer_header(
        referer: str = Header(...),
    ):
        """Referer header test."""
        return {"referer": referer}

    # Route: /headers/pattern
    @app.get("/headers/pattern")
    def pattern_header(
        x_request_id: str = Header(..., pattern=r"^[0-9]{3,}$"),
    ):
        """Header with regex pattern validation."""
        return {"x_request_id": x_request_id}

    # Route: /headers/host
    @app.get("/headers/host")
    def host_header(
        host: str = Header(...),
    ):
        """Host header test."""
        return {"host": host}

    # Route: /headers/max-length
    @app.get("/headers/max-length")
    def max_length_header(
        x_session_id: str = Header(..., max_length=20),
    ):
        """Header with max length validation."""
        return {"x_session_id": x_session_id}

    # Route: /headers/basic-auth
    @app.get("/headers/basic-auth")
    def basic_auth(
        authorization: str = Header(...),
    ):
        """Basic authentication."""
        if not authorization.startswith("Basic "):
            return Response(content={"detail": "Invalid authentication"}, status_code=401)

        # Decode base64 credentials
        encoded = authorization.replace("Basic ", "")
        decoded = base64.b64decode(encoded).decode("utf-8")
        username, password = decoded.split(":", 1)

        return {"username": username, "password": password}

    # Route: /headers/origin
    @app.get("/headers/origin")
    def origin_header(
        origin: str = Header(...),
    ):
        """Origin header test."""
        return {"origin": origin}

    # Route: /headers/accept-encoding
    @app.get("/headers/accept-encoding")
    def accept_encoding_header(
        accept_encoding: str = Header(...),
    ):
        """Accept-Encoding header test."""
        return {"accept_encoding": accept_encoding}

    # Route: /headers/underscore
    @app.get("/headers/underscore")
    def underscore_header(
        x_token: str = Header(...),
    ):
        """Header with underscore conversion."""
        return {"x_token": x_token}

    return app


# ============================================================================
# JSON Bodies Models and App
# ============================================================================


class Item(BaseModel):
    name: str = Field(min_length=1, max_length=100)
    description: str | None = None
    price: float = Field(ge=0)
    tax: float | None = None
    tags: list[str] = []
    in_stock: bool = True


class User(BaseModel):
    username: str = Field(min_length=3, max_length=50, pattern=r"^[a-zA-Z0-9_]+$")
    email: str
    full_name: str | None = None
    age: int | None = Field(None, ge=0, le=150)


class Address(BaseModel):
    street: str = Field(min_length=3)
    city: str = Field(min_length=3)
    zip_code: str = Field(min_length=5)


class Seller(BaseModel):
    name: str = Field(min_length=3)
    address: Address


class Product(BaseModel):
    name: str
    seller: Seller


class ItemStatus(str, Enum):
    draft = "draft"
    published = "published"
    archived = "archived"


class SimpleItem(BaseModel):
    name: str
    description: str | None = None
    price: float
    tax: float | None = None


class ValidatedItem(BaseModel):
    name: str = Field(min_length=3, max_length=50, pattern=r"^[a-zA-Z0-9 ]+$")
    price: float = Field(ge=0, le=1000)


class ItemWithUUID(BaseModel):
    name: str
    item_id: UUID


class ItemWithEnum(BaseModel):
    name: str
    status: ItemStatus


class ItemWithArrays(BaseModel):
    name: str
    tags: list[str] = []


class ValidatedList(BaseModel):
    items: list[str] = Field(min_length=1)


class NestedAddress(BaseModel):
    street: str
    city: str


class ItemWithNested(BaseModel):
    name: str
    address: NestedAddress


class EventModel(BaseModel):
    name: str
    event_date: date | None = None
    event_datetime: datetime | None = None


class OptionalAllModel(BaseModel):
    name: str | None = None
    description: str | None = None
    price: float | None = None


def json_bodies_app():
    """Create app for JSON body fixtures."""
    from spikard import Spikard

    app = Spikard()

    # Route: POST /items/ - simple item creation
    @app.post("/items/")
    def create_item(item: dict[str, Any]):
        """Create an item from JSON body - echo back as-is."""
        return item

    # Route: POST /items/nested - nested objects
    @app.post("/items/nested")
    def create_nested_item(item: dict[str, Any]):
        """Create item with nested objects."""
        return item

    # Route: POST /items/list - array of objects
    @app.post("/items/list")
    def create_item_list(items: dict[str, Any]):
        """Create list of items."""
        return items

    # Route: POST /items/validated - validated item
    @app.post("/items/validated")
    def create_validated_item(item: ValidatedItem):
        """Create validated item."""
        return item.model_dump()

    # Route: POST /items/optional-all - all optional fields
    @app.post("/items/optional-all")
    def create_optional_item(item: OptionalAllModel):
        """Create item with all optional fields."""
        return item.model_dump()

    # Route: POST /items/list-validated - validated list
    @app.post("/items/list-validated")
    def create_validated_list(data: ValidatedList):
        """Create validated list."""
        return data.model_dump()

    # Route: POST /events/ - events with date/datetime
    @app.post("/events/")
    def create_event(event: EventModel):
        """Create event with date/datetime fields."""
        result = {"name": event.name}
        if event.event_date:
            result["event_date"] = event.event_date.isoformat()
        if event.event_datetime:
            result["event_datetime"] = event.event_datetime.isoformat()
        return result

    # Route: PATCH /items/{item_id} - partial update
    @app.patch("/items/{item_id}")
    def update_item(item_id: int, item: dict[str, Any]):
        """Partial update of an item."""
        return item

    return app


# ============================================================================
# Cookies Models and App
# ============================================================================


def cookies_app():
    """Create app for cookie fixtures."""
    from spikard import Spikard

    app = Spikard()

    # Route: GET /items/ - Optional and required cookies
    @app.get("/items/")
    def get_items_cookies(
        ads_id: str | None = Cookie(None),
        session_id: str | None = Cookie(None),
        fatebook_tracker: str | None = Cookie(None),
        googall_tracker: str | None = Cookie(None),
        tracking_id: str | None = Cookie(None, min_length=10),
    ):
        """Cookie testing for /items/ route."""
        result = {}
        if ads_id is not None:
            result["ads_id"] = ads_id
        if session_id is not None:
            result["session_id"] = session_id
        if fatebook_tracker is not None:
            result["fatebook_tracker"] = fatebook_tracker
        if googall_tracker is not None:
            result["googall_tracker"] = googall_tracker
        if tracking_id is not None:
            result["tracking_id"] = tracking_id
        return result

    # Route: GET /users/me - API key cookie auth
    @app.get("/users/me")
    def get_user_me_cookie(
        key: str | None = Cookie(None),
    ):
        """API key cookie authentication."""
        if key == "secret":
            return {"username": "secret"}
        if key is None:
            return {"msg": "Hello World"}
        return Response(content={"detail": "Not authenticated"}, status_code=403)

    # Route: POST /cookie/ - Set cookie in response
    @app.post("/cookie/")
    def post_cookie():
        """Set cookie in response."""
        response = Response(content={"message": "Cookie set"})
        response.set_cookie("session_id", "abc123")
        return response

    # Route: GET /cookie/set - Set cookie with attributes
    @app.get("/cookie/set")
    def get_cookie_set():
        """Set cookie with attributes in response."""
        response = Response(content={"message": "Cookie set with attributes"})
        response.set_cookie("session_id", "abc123", max_age=3600, secure=True, httponly=True)
        return response

    # Route: GET /cookies/validated - Cookie with max length validation
    @app.get("/cookies/validated")
    def cookies_validated(
        session_id: str = Cookie(..., max_length=20),
    ):
        """Cookie with max length validation."""
        return {"session_id": session_id}

    # Route: GET /cookies/min-length - Cookie with min length
    @app.get("/cookies/min-length")
    def cookies_min_length(
        token: str = Cookie(..., min_length=8),
    ):
        """Cookie with min length validation."""
        return {"token": token}

    # Route: GET /cookies/pattern - Cookie with regex pattern
    @app.get("/cookies/pattern")
    def cookies_pattern(
        tracking_id: str = Cookie(..., pattern=r"^[0-9a-f]{8}$"),
    ):
        """Cookie with regex pattern validation."""
        return {"tracking_id": tracking_id}

    # Route: POST /cookies/delete - Delete cookie
    @app.post("/cookies/delete")
    def cookies_delete(session: str | None = Cookie(None)):
        """Delete cookie in response."""
        response = Response(content={"message": "Cookie deleted"})
        response.delete_cookie("session")
        return response

    # Route: POST /cookies/set-with-domain - Set cookie with domain
    @app.post("/cookies/set-with-domain")
    def cookies_set_with_domain():
        """Set cookie with domain attribute."""
        response = Response(content={"message": "Cookie set with domain"})
        response.set_cookie("session_id", "abc123", domain=".example.com")
        return response

    # Route: POST /cookies/set-with-path - Set cookie with path
    @app.post("/cookies/set-with-path")
    def cookies_set_with_path():
        """Set cookie with path attribute."""
        response = Response(content={"message": "Cookie set with path"})
        response.set_cookie("session_id", "abc123", path="/api")
        return response

    # Route: POST /cookies/session - Set session cookie
    @app.post("/cookies/session")
    def cookies_session():
        """Set session cookie (no max_age)."""
        response = Response(content={"message": "Session cookie set"})
        response.set_cookie("session_id", "abc123")
        return response

    # Route: POST /cookies/samesite-strict - Cookie with SameSite=Strict
    @app.post("/cookies/samesite-strict")
    def cookies_samesite_strict():
        """Set cookie with SameSite=Strict."""
        response = Response(content={"message": "Cookie set with SameSite=Strict"})
        response.set_cookie("session_id", "abc123", samesite="strict")
        return response

    # Route: POST /cookies/samesite-lax - Cookie with SameSite=Lax
    @app.post("/cookies/samesite-lax")
    def cookies_samesite_lax():
        """Set cookie with SameSite=Lax."""
        response = Response(content={"message": "Cookie set with SameSite=Lax"})
        response.set_cookie("session_id", "abc123", samesite="lax")
        return response

    # Route: POST /cookies/samesite-none - Cookie with SameSite=None
    @app.post("/cookies/samesite-none")
    def cookies_samesite_none():
        """Set cookie with SameSite=None."""
        response = Response(content={"message": "Cookie set with SameSite=None"})
        response.set_cookie("session_id", "abc123", samesite="none", secure=True)
        return response

    # Route: POST /cookies/multiple - Set multiple cookies
    @app.post("/cookies/multiple")
    def cookies_multiple():
        """Set multiple cookies in response."""
        response = Response(content={"message": "Multiple cookies set"})
        response.set_cookie("session_id", "abc123")
        response.set_cookie("user_id", "42")
        response.set_cookie("theme", "dark")
        return response

    return app


# ============================================================================
# Path Parameters App
# ============================================================================


class ItemType(str, Enum):
    book = "book"
    electronic = "electronic"
    clothing = "clothing"


def path_params_app():
    """Create app for path parameter fixtures.

    Handles all 20 path_params fixtures with proper validation:
    - String, int, float, bool path params
    - UUID and enum path params
    - Validation constraints (gt, ge, lt, le, min_length, max_length)
    - Multiple path params in single route
    - Date and file path params
    """
    from datetime import date

    from spikard import Spikard

    app = Spikard()

    # Route: /path/str/{item_id} - simple string path param
    @app.get("/path/str/{item_id}")
    def path_str(item_id: str):
        """String path parameter - returns the string directly."""
        return item_id

    # Route: /path/int/{item_id} - simple int path param
    @app.get("/path/int/{item_id}")
    def path_int(item_id: int):
        """Integer path parameter - returns the int directly."""
        return item_id

    # Route: /path/float/{item_id} - simple float path param
    @app.get("/path/float/{item_id}")
    def path_float(item_id: float):
        """Float path parameter - returns the float directly."""
        return item_id

    # Route: /path/bool/{value} - bool path param
    @app.get("/path/bool/{value}")
    def path_bool(value: bool):
        """Bool path parameter - returns the bool directly."""
        return value

    # Route: /items/{item_id} - UUID path param
    @app.get("/items/{item_id}")
    def items_uuid(item_id: UUID):
        """UUID path parameter."""
        return {"item_id": str(item_id)}

    # Route: /models/{model_name} - enum path param
    @app.get("/models/{model_name}")
    def models_enum(model_name: ModelName):
        """Enum path parameter - model_name can be alexnet, resnet, or lenet."""
        return {"model_name": model_name, "message": "Deep Learning FTW!"}

    # Route: /path/param-gt/{item_id} - int with gt constraint
    @app.get("/path/param-gt/{item_id}")
    def path_param_gt(item_id: int = Field(gt=3)):
        """Integer path parameter with gt (greater than) constraint."""
        return item_id

    # Route: /path/param-ge/{item_id} - int with ge constraint
    @app.get("/path/param-ge/{item_id}")
    def path_param_ge(item_id: int = Field(ge=3)):
        """Integer path parameter with ge (greater than or equal) constraint."""
        return item_id

    # Route: /path/param-le/{item_id} - int with le constraint
    @app.get("/path/param-le/{item_id}")
    def path_param_le(item_id: int = Field(le=3)):
        """Integer path parameter with le (less than or equal) constraint."""
        return item_id

    # Route: /path/param-lt/{item_id} - int with lt constraint
    @app.get("/path/param-lt/{item_id}")
    def path_param_lt(item_id: int = Field(lt=3)):
        """Integer path parameter with lt (less than) constraint."""
        return item_id

    # Route: /path/param-lt-gt/{item_id} - int with combined constraints
    @app.get("/path/param-lt-gt/{item_id}")
    def path_param_lt_gt(item_id: int = Field(gt=1, lt=3)):
        """Integer path parameter with combined lt and gt constraints."""
        return item_id

    # Route: /path/param-minlength/{item_id} - string with min_length
    @app.get("/path/param-minlength/{item_id}")
    def path_param_minlength(item_id: str = Field(min_length=3)):
        """String path parameter with min_length constraint."""
        return item_id

    # Route: /path/param-maxlength/{item_id} - string with max_length
    @app.get("/path/param-maxlength/{item_id}")
    def path_param_maxlength(item_id: str = Field(max_length=3)):
        """String path parameter with max_length constraint."""
        return item_id

    # Route: /{version}/{service_id}/{user_id}/{order_id} - multiple path params
    @app.get("/{version}/{service_id}/{user_id}/{order_id}")
    def multiple_path_params(
        version: float,
        service_id: int,
        user_id: str,
        order_id: UUID,
    ):
        """Multiple path parameters with different types."""
        return {
            "version": version,
            "service_id": service_id,
            "user_id": user_id,
            "order_id": str(order_id),
        }

    # Route: /date/{date_param} - date path param
    @app.get("/date/{date_param}")
    def date_path_param(date_param: date):
        """Date path parameter - ISO 8601 format (YYYY-MM-DD)."""
        return {"date_param": date_param.isoformat()}

    # Route: /files/{file_path:path} - file path param (path converter)
    @app.get("/files/{file_path:path}")
    def files_path(file_path: str):
        """File path parameter - accepts path with slashes."""
        return {"file_path": file_path}

    return app


# ============================================================================
# Status Codes App
# ============================================================================


def status_codes_app():
    """Create app for status code fixtures.

    Handles all 18 status_codes fixtures testing different HTTP status codes:
    - 2xx: Success responses (200, 201, 202, 204, 206)
    - 3xx: Redirects (301, 302, 304, 307)
    - 4xx: Client errors (400, 401, 403, 404, 408, 422, 429)
    - 5xx: Server errors (500, 503)

    NOTE: Many tests will fail because Response class doesn't exist yet.
    This is a simplified implementation that just returns dicts.
    """
    from spikard import Spikard

    app = Spikard()

    # Route: GET /items/1 - returns 200 or 304
    @app.get("/items/{item_id}")
    def get_item(item_id: int, request):
        """Get item - can return 200 or 304 based on If-None-Match header."""
        # Check for 304 Not Modified scenario
        if_none_match = request.headers.get("If-None-Match")
        if if_none_match == '"etag-123"':
            return Response(content=None, status_code=304, headers={"ETag": '"etag-123"'})

        # Handle different item IDs
        if item_id == 999:
            return Response(content={"detail": "Item not found"}, status_code=404)

        # Normal 200 response
        return {"id": item_id, "name": f"Item {item_id}"}

    # Route: POST /items/ - can return 201, 400, or 422
    @app.post("/items/")
    def create_item(request):
        """Create item - returns 201, 400 (bad JSON), or 422 (validation error)."""
        # Try to get the body
        try:
            body = request.body

            # Check if it's a string (invalid JSON scenario)
            if isinstance(body, str):
                return Response(content={"detail": "Invalid request format"}, status_code=400)

            # Check for validation errors (missing required field 'name')
            if not isinstance(body, dict) or "name" not in body:
                return Response(
                    content={"detail": [{"type": "missing", "loc": ["body", "name"], "msg": "Field required"}]},
                    status_code=422,
                )

            # Valid request - return 201 Created
            return Response(content={"id": 1, "name": body["name"]}, status_code=201)
        except Exception:
            # Malformed JSON
            return Response(content={"detail": "Invalid request format"}, status_code=400)

    # Route: POST /tasks/ - returns 202 Accepted
    @app.post("/tasks/")
    def create_task():
        """Create task - returns 202 Accepted (async processing)."""
        return Response(content={"id": 1, "status": "processing"}, status_code=202)

    # Route: DELETE /items/1 - returns 204 No Content
    @app.delete("/items/{item_id}")
    def delete_item(item_id: int):
        """Delete item - returns 204 No Content."""
        return Response(content=None, status_code=204)

    # Route: GET /old-path - returns 301 Moved Permanently
    @app.get("/old-path")
    def old_path():
        """Permanent redirect - returns 301."""
        return Response(content=None, status_code=301, headers={"location": "/new-path"})

    # Route: GET /temp-redirect - returns 302 Found
    @app.get("/temp-redirect")
    def temp_redirect():
        """Temporary redirect - returns 302."""
        return Response(content=None, status_code=302, headers={"location": "/new-location"})

    # Route: POST /redirect-post - returns 307 Temporary Redirect
    @app.post("/redirect-post")
    def redirect_post():
        """POST redirect - returns 307 (method preserved)."""
        return Response(content=None, status_code=307, headers={"location": "/new-post-location"})

    # Route: GET /users/me - returns 401 Unauthorized
    @app.get("/users/me")
    def get_user_unauthorized():
        """Unauthorized access - returns 401."""
        return Response(
            content={"detail": "Not authenticated"}, status_code=401, headers={"WWW-Authenticate": "Bearer"}
        )

    # Route: GET /admin/users - returns 403 Forbidden
    @app.get("/admin/users")
    def admin_forbidden():
        """Forbidden access - returns 403."""
        return Response(content={"detail": "Forbidden"}, status_code=403)

    # Route: GET /error - returns 500 Internal Server Error
    @app.get("/error")
    def internal_error():
        """Server error - returns 500."""
        return Response(content={"detail": "Internal server error"}, status_code=500)

    # Route: GET /health - returns 503 Service Unavailable
    @app.get("/health")
    def health_unavailable():
        """Service unavailable - returns 503."""
        return Response(
            content={"detail": "Service temporarily unavailable"}, status_code=503, headers={"Retry-After": "120"}
        )

    # Route: GET /files/document.pdf - returns 206 Partial Content
    @app.get("/files/{filename}")
    def get_file_partial(filename: str, request):
        """Partial content - returns 206."""
        range_header = request.headers.get("Range")
        if range_header:
            return Response(
                content=b"partial content data",
                status_code=206,
                headers={"Content-Range": "bytes 0-1023/5000", "Content-Length": "1024"},
            )
        return {"filename": filename}

    # Route: POST /slow-endpoint - returns 408 Request Timeout
    @app.post("/slow-endpoint")
    def request_timeout():
        """Request timeout - returns 408."""
        return Response(content={"detail": "Request timeout"}, status_code=408)

    # Route: GET /api/resource - returns 429 Too Many Requests
    @app.get("/api/resource")
    def rate_limited():
        """Rate limited - returns 429."""
        return Response(content={"detail": "Too many requests"}, status_code=429, headers={"Retry-After": "60"})

    return app


# ============================================================================
# Main Unified App
# ============================================================================


def create_fixture_test_app():
    """Create a unified app with all fixture endpoints.

    This combines all category-specific endpoints into a single application
    for comprehensive fixture testing.
    """
    from spikard import Spikard

    app = Spikard()

    # Mount all category-specific apps
    # For now, we'll implement the most important endpoints inline

    # Query parameters
    @app.get("/api/items/")
    def get_items(
        q: str | None = None,
        page: int = 1,
        limit: int | None = None,
        tags: list[str] | None = None,
    ):
        return {
            "page": page,
            "q": q,
            "limit": limit,
            "tags": tags or [],
        }

    # Headers
    @app.get("/api/headers/")
    def test_headers(
        x_api_key: str | None = None,
    ):
        return {"x_api_key": x_api_key}

    # JSON body
    @app.post("/api/items/")
    def create_item(item: Item):
        return {"id": 1, **item.model_dump()}

    return app
