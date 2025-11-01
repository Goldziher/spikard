"""Factory functions for creating test applications based on fixture categories.

This module provides app factories that create Spikard applications configured
to handle different types of test scenarios (query params, cookies, headers, etc.).
"""

from datetime import date, datetime
from enum import Enum
from typing import Any
from uuid import UUID

from pydantic import Field

from spikard import Spikard
from spikard.params import Header
from spikard.routing import delete, get, post


class Status(str, Enum):
    """Status enum for testing."""

    active = "active"
    inactive = "inactive"
    pending = "pending"


class ModelName(str, Enum):
    """Model name enum for testing."""

    alexnet = "alexnet"
    resnet = "resnet"
    lenet = "lenet"


def query_params_app() -> Spikard:
    """Create a Spikard app for query parameter testing."""
    app = Spikard()

    @get("/items")
    def get_items(
        q: str | None = None,
        page: int = 1,
        limit: int | None = None,
        tags: list[str] | None = None,
        active: bool | None = None,
        item_id: UUID | None = None,
        status: Status | None = None,
        created_after: date | None = None,
        updated_at: datetime | None = None,
        min_price: float | None = None,
        search: str | None = Field(None, min_length=3, max_length=50),
        code: str | None = Field(None, pattern=r"^[A-Z]{3}$"),
    ) -> dict[str, Any]:
        """Generic endpoint for query parameter testing."""
        result = {"message": "Query parameters received", "page": page}

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

        return result

    @get("/users/{user_id}")
    def get_user(user_id: int = Field(gt=0)) -> dict[str, int]:
        """Endpoint with validated path param."""
        return {"user_id": user_id}

    return app


def cookies_app() -> Spikard:
    """Create a Spikard app for cookie testing."""
    app = Spikard()

    @get("/items/cookies")
    def get_items_cookies(
        session_id: str,
        fatebook_tracker: str | None = None,
    ) -> dict[str, Any]:
        """Endpoint for cookie testing."""
        result: dict[str, Any] = {"session_id": session_id}
        if fatebook_tracker is not None:
            result["fatebook_tracker"] = fatebook_tracker
        return result

    @get("/users/me/auth")
    def get_users_me_auth(
        key: str,
    ) -> dict[str, Any]:
        """Endpoint for API key cookie authentication."""
        return {"authenticated": "true", "key": key}

    return app


def headers_app() -> Spikard:
    """Create a Spikard app for header testing."""
    app = Spikard()

    @get("/items")
    def get_items_headers(
        user_agent: str | None = None,
        x_api_key: str | None = None,
    ) -> dict[str, Any]:
        """Endpoint for header testing."""
        result: dict[str, Any] = {}
        if user_agent is not None:
            result["user_agent"] = user_agent
        if x_api_key is not None:
            result["x_api_key"] = x_api_key
        return result

    return app


def json_bodies_app() -> Spikard:
    """Create a Spikard app for JSON body testing."""
    app = Spikard()

    @post(
        "/users",
        body_schema={
            "type": "object",
            "required": ["username", "email"],
            "properties": {
                "username": {"type": "string"},
                "email": {"type": "string"},
                "age": {"type": "integer"},
            },
        },
    )
    def post_users(body: dict[str, Any]) -> dict[str, Any]:
        """Endpoint for JSON body testing."""
        result: dict[str, Any] = {"username": body["username"], "email": body["email"]}
        if "age" in body:
            result["age"] = body["age"]
        return result

    @post(
        "/items/",
        body_schema={
            "type": "object",
            "required": ["name", "price"],
            "properties": {
                "name": {"type": "string"},
                "description": {"type": "string"},
                "price": {"type": "number"},
            },
        },
    )
    def post_items(body: dict[str, Any]) -> dict[str, Any]:
        """Endpoint for JSON body testing."""
        result: dict[str, Any] = {"name": body["name"]}
        if "description" in body:
            result["description"] = body["description"]
        if "price" in body:
            result["price"] = body["price"]
        return result

    return app


def path_params_app() -> Spikard:
    """Create a Spikard app for path parameter testing."""
    app = Spikard()

    @get("/items/{item_id}")
    def get_item(item_id: int) -> dict[str, int]:
        """Endpoint for path parameter testing."""
        return {"item_id": item_id}

    @get("/users/{user_id}")
    def get_user(user_id: str) -> dict[str, str]:
        """Endpoint for string path parameter testing."""
        return {"user_id": user_id}

    @get("/products/{product_id}")
    def get_product(product_id: UUID) -> dict[str, str]:
        """Endpoint for UUID path parameter testing."""
        return {"product_id": str(product_id)}

    return app


def validation_errors_app() -> Spikard:
    """Create a Spikard app for validation error testing."""
    app = Spikard()

    # GET /items/ - Main endpoint that requires q and x-token header, accepts many params for testing validations
    # This route is used by most query param validation tests, including fixture 02 (missing required)
    # Fixture 19 tests missing required header
    @get("/items/")
    def get_items_validation(
        q: str = Field(
            ..., min_length=3, max_length=50, pattern=r"^[a-zA-Z0-9_-]+$"
        ),  # REQUIRED - fixture 02 tests missing, 07/08 test length, 09 tests pattern
        x_token: str = Header(...),  # type: ignore[assignment]  # REQUIRED header - fixture 19 tests missing header
        skip: int | None = None,
        limit: int | None = Field(None, le=100),  # fixture 06 tests le constraint
        price: float | None = Field(None, gt=0, le=1000),
        search: str | None = Field(None, min_length=3, max_length=50),
        code: str | None = Field(None, pattern=r"^[A-Z]{3}$"),
        status: Status | None = None,
        tags: list[str] | None = Field(None, min_length=1, max_length=10),
        created_after: date | None = None,
        active: bool | None = None,
        is_active: bool | None = None,  # fixture 13 tests bool validation
    ) -> dict[str, Any]:
        """Endpoint for query parameter validation testing."""
        result: dict[str, Any] = {"message": "Query parameters received", "q": q}
        if skip is not None:
            result["skip"] = skip
        if limit is not None:
            result["limit"] = limit
        if price is not None:
            result["price"] = price
        if search is not None:
            result["search"] = search
        if code is not None:
            result["code"] = code
        if status is not None:
            result["status"] = status
        if tags is not None:
            result["tags"] = tags
        if created_after is not None:
            result["created_after"] = created_after.isoformat()
        if active is not None:
            result["active"] = active
        if is_active is not None:
            result["is_active"] = is_active
        return result

    # GET /items/{item_id} - for UUID path param validation
    @get("/items/{item_id}")
    def get_item_validation(item_id: UUID) -> dict[str, str]:
        """Endpoint for UUID path parameter validation."""
        return {"item_id": str(item_id)}

    # GET /models/{model_name} - for enum path param validation (fixture 10)
    @get("/models/{model_name}")
    def get_model_validation(model_name: ModelName) -> dict[str, str]:
        """Endpoint for enum path parameter validation."""
        return {"model_name": model_name.value}

    # POST /items/ - for body validation errors
    @post(
        "/items/",
        body_schema={
            "type": "object",
            "required": ["name", "price"],
            "properties": {
                "name": {"type": "string", "minLength": 3},
                "description": {"type": "string"},
                "price": {"type": "number", "exclusiveMinimum": 0},
                "quantity": {"type": "integer"},
                "created_at": {"type": "string", "format": "date-time"},
                "tags": {
                    "type": "array",
                    "items": {"type": "string"},
                    "minItems": 1,
                    "maxItems": 10,
                },
                "metadata": {
                    "type": "object",
                    "properties": {
                        "category": {"type": "string"},
                        "rating": {"type": "number"},
                    },
                },
                "seller": {
                    "type": "object",
                    "properties": {
                        "name": {"type": "string", "minLength": 3},
                        "email": {"type": "string", "format": "email"},
                        "address": {
                            "type": "object",
                            "properties": {
                                "city": {"type": "string", "minLength": 3},
                                "zip_code": {"type": "string", "minLength": 5},
                            },
                        },
                    },
                },
            },
        },
    )
    def post_items_validation(body: dict[str, Any]) -> dict[str, Any]:
        """Endpoint for JSON body validation testing."""
        result: dict[str, Any] = {"name": body["name"], "price": body["price"]}
        if "description" in body:
            result["description"] = body["description"]
        if "quantity" in body:
            result["quantity"] = body["quantity"]
        if "created_at" in body:
            result["created_at"] = body["created_at"]
        if "tags" in body:
            result["tags"] = body["tags"]
        if "metadata" in body:
            result["metadata"] = body["metadata"]
        if "seller" in body:
            result["seller"] = body["seller"]
        return result

    # POST /users - for body validation errors (fixture 09)
    @post(
        "/users",
        body_schema={
            "type": "object",
            "required": ["name", "email", "age"],
            "properties": {
                "name": {"type": "string", "minLength": 3},
                "email": {"type": "string", "format": "email"},
                "age": {"type": "integer", "minimum": 18},
            },
        },
    )
    def post_users_validation(body: dict[str, Any]) -> dict[str, Any]:
        """Endpoint for JSON body validation testing."""
        result: dict[str, Any] = {"name": body["name"], "email": body["email"], "age": body["age"]}
        return result

    # POST /profiles - for nested validation testing (fixture 10)
    @post(
        "/profiles",
        body_schema={
            "type": "object",
            "required": ["profile"],
            "properties": {
                "profile": {
                    "type": "object",
                    "required": ["contact"],
                    "properties": {
                        "contact": {
                            "type": "object",
                            "required": ["email"],
                            "properties": {
                                "email": {"type": "string", "format": "email"},
                            },
                        },
                    },
                },
            },
        },
    )
    def post_profiles_validation(body: dict[str, Any]) -> dict[str, Any]:
        """Endpoint for nested validation testing."""
        return {"profile": body["profile"]}

    return app


def status_codes_app() -> Spikard:
    """Create a Spikard app for status code testing."""
    app = Spikard()

    @get("/status-test/{code}")
    def get_status_test(code: int) -> dict[str, int]:
        """Endpoint for status code testing."""
        return {"code": code}

    @post("/items")
    def post_items_status(name: str) -> dict[str, str]:
        """Endpoint for testing 201 Created."""
        return {"name": name, "id": "123"}

    @delete("/items/{item_id}")
    def delete_item(item_id: int) -> dict[str, str]:
        """Endpoint for testing 204 No Content."""
        return {"message": "deleted"}

    return app
