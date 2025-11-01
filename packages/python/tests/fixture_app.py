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
from spikard.routing import delete, get, post


class Status(str, Enum):
    """Status enum for testing."""

    active = "active"
    inactive = "inactive"
    pending = "pending"


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

    # GET /items/ - Main endpoint that accepts many optional params for testing different validations
    # Note: q is optional here to allow testing other parameter validations
    # Fixture 02 specifically tests missing required param with this same route
    @get("/items/")
    def get_items_validation(
        q: str | None = Field(None, min_length=3),  # Optional but when provided must be >= 3 chars
        skip: int | None = None,
        price: float | None = Field(None, gt=0, le=1000),
        search: str | None = Field(None, min_length=3, max_length=50),
        code: str | None = Field(None, pattern=r"^[A-Z]{3}$"),
        status: Status | None = None,
        tags: list[str] | None = Field(None, min_length=1, max_length=10),
        created_after: date | None = None,
        active: bool | None = None,
    ) -> dict[str, Any]:
        """Endpoint for query parameter validation testing."""
        result: dict[str, Any] = {"message": "Query parameters received"}
        if q is not None:
            result["q"] = q
        if skip is not None:
            result["skip"] = skip
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
        return result

    # GET /items/{item_id} - for UUID path param validation
    @get("/items/{item_id}")
    def get_item_validation(item_id: UUID) -> dict[str, str]:
        """Endpoint for UUID path parameter validation."""
        return {"item_id": str(item_id)}

    # GET /models/{model_id} - for UUID path param validation
    @get("/models/{model_id}")
    def get_model_validation(model_id: UUID) -> dict[str, str]:
        """Endpoint for UUID path parameter validation."""
        return {"model_id": str(model_id)}

    # POST /items/ - for body validation errors
    @post(
        "/items/",
        body_schema={
            "type": "object",
            "required": ["name", "price"],
            "properties": {
                "name": {"type": "string"},
                "description": {"type": "string"},
                "price": {"type": "number"},
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
            },
        },
    )
    def post_items_validation(body: dict[str, Any]) -> dict[str, Any]:
        """Endpoint for JSON body validation testing."""
        result: dict[str, Any] = {"name": body["name"], "price": body["price"]}
        if "description" in body:
            result["description"] = body["description"]
        if "tags" in body:
            result["tags"] = body["tags"]
        if "metadata" in body:
            result["metadata"] = body["metadata"]
        return result

    # POST /users - for body validation errors
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
    def post_users_validation(body: dict[str, Any]) -> dict[str, Any]:
        """Endpoint for JSON body validation testing."""
        result: dict[str, Any] = {"username": body["username"], "email": body["email"]}
        if "age" in body:
            result["age"] = body["age"]
        return result

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
