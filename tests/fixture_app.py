"""Test application implementing endpoints for all fixture categories.

This app is designed to work with the test fixtures in testing_data/.
It implements endpoints that match the fixture expectations for comprehensive testing.
"""

from datetime import date, datetime
from enum import Enum
from typing import Any, Dict, List, Optional
from uuid import UUID

from pydantic import BaseModel, Field, field_validator


# ============================================================================
# Query Parameters Models
# ============================================================================

class StatusEnum(str, Enum):
    active = "active"
    inactive = "inactive"
    pending = "pending"


def query_params_app():
    """Create app for query parameter fixtures."""
    from spikard import Spikard

    app = Spikard()

    @app.get("/items/")
    def get_items(
        q: Optional[str] = None,
        page: int = 1,
        limit: Optional[int] = None,
        tags: Optional[List[str]] = None,
        active: Optional[bool] = None,
        item_id: Optional[UUID] = None,
        status: Optional[StatusEnum] = None,
        created_after: Optional[date] = None,
        updated_at: Optional[datetime] = None,
        min_price: Optional[float] = None,
        search: Optional[str] = Field(None, min_length=3, max_length=50),
        code: Optional[str] = Field(None, pattern=r"^[A-Z]{3}$"),
    ):
        """Generic endpoint for query parameter testing."""
        result = {"message": "Query parameters received"}

        if q is not None:
            result["q"] = q
        if limit is not None:
            result["limit"] = limit
        if tags is not None:
            result["tags"] = tags
        if active is not None:
            result["active"] = active
        if item_id is not None:
            result["item_id"] = str(item_id)
        if status is not None:
            result["status"] = status
        if created_after is not None:
            result["created_after"] = created_after.isoformat()
        if updated_at is not None:
            result["updated_at"] = updated_at.isoformat()
        if min_price is not None:
            result["min_price"] = min_price
        if search is not None:
            result["search"] = search
        if code is not None:
            result["code"] = code

        result["page"] = page
        return result

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
    from spikard import Spikard, Header

    app = Spikard()

    @app.get("/headers/test")
    def test_headers(
        user_agent: Optional[str] = Header(None),
        x_api_key: Optional[str] = Header(None),
        authorization: Optional[str] = Header(None),
        x_token: Optional[str] = Header(None),
        content_type: Optional[str] = Header(None),
        accept: Optional[str] = Header(None),
        accept_language: Optional[str] = Header(None),
        referer: Optional[str] = Header(None),
        host: Optional[str] = Header(None),
        origin: Optional[str] = Header(None),
        accept_encoding: Optional[str] = Header(None),
        x_custom_header: Optional[str] = Header(None, min_length=5, max_length=50),
        x_version: Optional[str] = Header(None, pattern=r"^v\d+\.\d+$"),
    ):
        """Generic endpoint for header testing."""
        result = {"message": "Headers received"}

        if user_agent:
            result["user_agent"] = user_agent
        if x_api_key:
            result["x_api_key"] = x_api_key
        if authorization:
            result["authorization"] = authorization
        if x_token:
            result["x_token"] = x_token
        if content_type:
            result["content_type"] = content_type
        if accept:
            result["accept"] = accept
        if accept_language:
            result["accept_language"] = accept_language
        if referer:
            result["referer"] = referer
        if host:
            result["host"] = host
        if origin:
            result["origin"] = origin
        if accept_encoding:
            result["accept_encoding"] = accept_encoding
        if x_custom_header:
            result["x_custom_header"] = x_custom_header
        if x_version:
            result["x_version"] = x_version

        return result

    @app.get("/auth/required")
    def auth_required(
        x_api_key: str = Header(..., alias="X-API-Key")
    ):
        """Endpoint requiring API key header."""
        return {"authenticated": True, "key": x_api_key}

    @app.get("/auth/bearer")
    def bearer_auth(
        authorization: str = Header(..., pattern=r"^Bearer .+$")
    ):
        """Endpoint requiring Bearer token."""
        token = authorization.replace("Bearer ", "")
        return {"authenticated": True, "token": token}

    return app


# ============================================================================
# JSON Bodies Models and App
# ============================================================================

class Item(BaseModel):
    name: str = Field(min_length=1, max_length=100)
    description: Optional[str] = None
    price: float = Field(ge=0)
    tax: Optional[float] = None
    tags: List[str] = []
    in_stock: bool = True


class User(BaseModel):
    username: str = Field(min_length=3, max_length=50, pattern=r"^[a-zA-Z0-9_]+$")
    email: str
    full_name: Optional[str] = None
    age: Optional[int] = Field(None, ge=0, le=150)


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


def json_bodies_app():
    """Create app for JSON body fixtures."""
    from spikard import Spikard

    app = Spikard()

    @app.post("/items/")
    def create_item(item: Item):
        """Create an item from JSON body."""
        return {"id": 1, **item.model_dump()}

    @app.post("/users/")
    def create_user(user: User):
        """Create a user from JSON body."""
        return {"id": 1, **user.model_dump()}

    @app.post("/products/")
    def create_product(product: Product):
        """Create a product with nested validation."""
        return {"id": 1, **product.model_dump()}

    @app.patch("/items/{item_id}")
    def update_item(item_id: int, item: Item):
        """Partial update of an item."""
        return {"id": item_id, **item.model_dump()}

    @app.post("/nulls/")
    def handle_nulls(data: Dict[str, Any]):
        """Test null, empty, and falsy value handling."""
        result = {}

        if "explicit_null" in data:
            result["explicit_null_is_null"] = data["explicit_null"] is None
        if "empty_string" in data:
            result["empty_string_length"] = len(data["empty_string"])
        if "empty_array" in data:
            result["empty_array_length"] = len(data["empty_array"])
        if "empty_object" in data:
            result["empty_object_keys"] = len(data["empty_object"])
        if "zero_number" in data:
            result["zero_is_falsy"] = not bool(data["zero_number"])
        if "false_boolean" in data:
            result["false_is_false"] = data["false_boolean"] is False

        return result

    return app


# ============================================================================
# Cookies Models and App
# ============================================================================

def cookies_app():
    """Create app for cookie fixtures."""
    from spikard import Spikard, Cookie, Response

    app = Spikard()

    @app.get("/cookies/test")
    def test_cookies(
        session_id: Optional[str] = Cookie(None),
        user_id: Optional[str] = Cookie(None),
        api_key: Optional[str] = Cookie(None, min_length=10),
        token: Optional[str] = Cookie(None, pattern=r"^[A-Za-z0-9]{32}$"),
    ):
        """Generic endpoint for cookie testing."""
        result = {"message": "Cookies received"}

        if session_id:
            result["session_id"] = session_id
        if user_id:
            result["user_id"] = user_id
        if api_key:
            result["api_key"] = api_key
        if token:
            result["token"] = token

        return result

    @app.get("/cookies/set")
    def set_cookies():
        """Set response cookies."""
        response = Response(content={"message": "Cookies set"})
        response.set_cookie("session_id", "abc123")
        response.set_cookie("user_id", "42", max_age=3600, secure=True, httponly=True)
        return response

    @app.get("/auth/cookie")
    def cookie_auth(
        api_key: str = Cookie(..., alias="apiKey")
    ):
        """Endpoint requiring API key cookie."""
        return {"authenticated": True, "key": api_key}

    return app


# ============================================================================
# Path Parameters App
# ============================================================================

class ItemType(str, Enum):
    book = "book"
    electronic = "electronic"
    clothing = "clothing"


def path_params_app():
    """Create app for path parameter fixtures."""
    from spikard import Spikard

    app = Spikard()

    @app.get("/items/{item_id}")
    def get_item(item_id: int = Field(gt=0)):
        """Get item by ID with validation."""
        return {"item_id": item_id, "name": "Test Item"}

    @app.get("/users/{user_id}")
    def get_user_by_id(user_id: UUID):
        """Get user by UUID."""
        return {"user_id": str(user_id), "name": "Test User"}

    @app.get("/products/{product_id}")
    def get_product(product_id: str = Field(min_length=3, max_length=10)):
        """Get product by string ID with validation."""
        return {"product_id": product_id, "name": "Test Product"}

    @app.get("/categories/{category_type}")
    def get_category(category_type: ItemType):
        """Get category with enum validation."""
        return {"category": category_type, "count": 10}

    @app.get("/posts/{year}/{month}/{day}")
    def get_posts_by_date(
        year: int = Field(gt=2000, lt=2100),
        month: int = Field(ge=1, le=12),
        day: int = Field(ge=1, le=31),
    ):
        """Get posts by date with multiple path params."""
        return {"year": year, "month": month, "day": day, "posts": []}

    @app.get("/files/{file_path:path}")
    def get_file(file_path: str):
        """Get file with path parameter."""
        return {"file_path": file_path, "exists": True}

    return app


# ============================================================================
# Status Codes App
# ============================================================================

def status_codes_app():
    """Create app for status code fixtures."""
    from spikard import Spikard, Response

    app = Spikard()

    @app.get("/status/200")
    def ok():
        """Return 200 OK."""
        return {"message": "Success"}

    @app.post("/status/201")
    def created():
        """Return 201 Created."""
        return Response(content={"id": 1, "message": "Resource created"}, status_code=201)

    @app.get("/status/204")
    def no_content():
        """Return 204 No Content."""
        return Response(content=None, status_code=204)

    @app.get("/status/400")
    def bad_request():
        """Return 400 Bad Request."""
        return Response(content={"error": "Bad request"}, status_code=400)

    @app.get("/status/404")
    def not_found():
        """Return 404 Not Found."""
        return Response(content={"error": "Not found"}, status_code=404)

    @app.get("/status/500")
    def internal_error():
        """Return 500 Internal Server Error."""
        return Response(content={"error": "Internal server error"}, status_code=500)

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
        q: Optional[str] = None,
        page: int = 1,
        limit: Optional[int] = None,
        tags: Optional[List[str]] = None,
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
        x_api_key: Optional[str] = None,
    ):
        return {"x_api_key": x_api_key}

    # JSON body
    @app.post("/api/items/")
    def create_item(item: Item):
        return {"id": 1, **item.model_dump()}

    return app
