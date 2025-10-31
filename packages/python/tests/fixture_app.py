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


class Status(str, Enum):
    """Status enum for testing."""

    active = "active"
    inactive = "inactive"
    pending = "pending"


def query_params_app() -> Spikard:
    """Create a Spikard app for query parameter testing."""
    app = Spikard()

    @app.get("/items")
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

    @app.get("/users/{user_id}")
    def get_user(user_id: int = Field(gt=0)) -> dict[str, int]:
        """Endpoint with validated path param."""
        return {"user_id": user_id}

    return app


def cookies_app() -> Spikard:
    """Create a Spikard app for cookie testing."""
    app = Spikard()

    @app.get("/items/cookies")
    def get_items_cookies(
        session_id: str,
        fatebook_tracker: str | None = None,
    ) -> dict[str, Any]:
        """Endpoint for cookie testing."""
        result: dict[str, Any] = {"session_id": session_id}
        if fatebook_tracker is not None:
            result["fatebook_tracker"] = fatebook_tracker
        return result

    @app.get("/users/me/auth")
    def get_users_me_auth(
        key: str,
    ) -> dict[str, Any]:
        """Endpoint for API key cookie authentication."""
        return {"authenticated": "true", "key": key}

    return app


def headers_app() -> Spikard:
    """Create a Spikard app for header testing."""
    app = Spikard()

    @app.get("/items")
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

    @app.post("/users")
    def post_users(
        username: str,
        email: str,
        age: int | None = None,
    ) -> dict[str, Any]:
        """Endpoint for JSON body testing."""
        result: dict[str, Any] = {"username": username, "email": email}
        if age is not None:
            result["age"] = age
        return result

    @app.post("/items")
    def post_items(
        name: str,
        description: str | None = None,
        price: float | None = None,
    ) -> dict[str, Any]:
        """Endpoint for JSON body testing."""
        result: dict[str, Any] = {"name": name}
        if description is not None:
            result["description"] = description
        if price is not None:
            result["price"] = price
        return result

    return app


def path_params_app() -> Spikard:
    """Create a Spikard app for path parameter testing."""
    app = Spikard()

    @app.get("/items/{item_id}")
    def get_item(item_id: int) -> dict[str, int]:
        """Endpoint for path parameter testing."""
        return {"item_id": item_id}

    @app.get("/users/{user_id}")
    def get_user(user_id: str) -> dict[str, str]:
        """Endpoint for string path parameter testing."""
        return {"user_id": user_id}

    @app.get("/products/{product_id}")
    def get_product(product_id: UUID) -> dict[str, str]:
        """Endpoint for UUID path parameter testing."""
        return {"product_id": str(product_id)}

    return app


def status_codes_app() -> Spikard:
    """Create a Spikard app for status code testing."""
    app = Spikard()

    @app.get("/status-test/{code}")
    def get_status_test(code: int) -> dict[str, int]:
        """Endpoint for status code testing."""
        return {"code": code}

    @app.post("/items")
    def post_items_status(name: str) -> dict[str, str]:
        """Endpoint for testing 201 Created."""
        return {"name": name, "id": "123"}

    @app.delete("/items/{item_id}")
    def delete_item(item_id: int) -> dict[str, str]:
        """Endpoint for testing 204 No Content."""
        return {"message": "deleted"}

    return app
