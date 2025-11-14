"""Generated E2E test application with per-fixture app factories."""
# ruff: noqa: ARG001, A002
# mypy: ignore-errors

from dataclasses import asdict, dataclass
from datetime import date, datetime
from enum import Enum
from typing import Any, NamedTuple, TypedDict
from uuid import UUID

import asyncio
from pathlib import Path
import json
import msgspec
from pydantic import BaseModel

from collections import defaultdict

BASE_DIR = Path(__file__).parent

from spikard import (
    Response,
    Spikard,
    StreamingResponse,
    delete,
    get,
    head,
    options,
    patch,
    post,
    put,
    background,
    websocket,
    sse,
)
from spikard.config import (
    ServerConfig,
    CompressionConfig,
    RateLimitConfig,
    JwtConfig,
    ApiKeyConfig,
    StaticFilesConfig,
    OpenApiConfig,
    ContactInfo,
    LicenseInfo,
    ServerInfo,
    SecuritySchemeInfo,
)

BACKGROUND_STATE = defaultdict(list)


def compression_compression_payload_below_min_size_is_not_compressed() -> Any:
    """Handler for GET /compression/skip."""
    return Response(
        content={"message": "Small payload", "payload": "tiny"},
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_compression_compression_payload_below_min_size_is_not_compressed() -> Spikard:
    """App factory for fixture: Compression - payload below min_size is not compressed"""
    config = ServerConfig(compression=CompressionConfig(gzip=True, brotli=False, min_size=4096, quality=6))
    app = Spikard(config=config)
    # Register handler with this app instance
    app.register_route("GET", "/compression/skip", body_schema=None, parameter_schema=None, file_params=None)(
        compression_compression_payload_below_min_size_is_not_compressed
    )
    return app


def compression_compression_gzip_applied() -> Any:
    """Handler for GET /compression/gzip."""
    return Response(
        content={
            "message": "Compressed payload",
            "payload": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        },
        status_code=200,
        headers={"vary": "Accept-Encoding", "Content-Type": "application/json"},
    )


def create_app_compression_compression_gzip_applied() -> Spikard:
    """App factory for fixture: Compression - gzip applied"""
    config = ServerConfig(compression=CompressionConfig(gzip=True, brotli=False, min_size=0, quality=4))
    app = Spikard(config=config)
    # Register handler with this app instance
    app.register_route("GET", "/compression/gzip", body_schema=None, parameter_schema=None, file_params=None)(
        compression_compression_gzip_applied
    )
    return app


async def request_timeout_request_exceeds_timeout() -> Any:
    """Handler for GET /timeouts/slow."""
    await asyncio.sleep(1.5)
    return Response(status_code=408)


def create_app_request_timeout_request_exceeds_timeout() -> Spikard:
    """App factory for fixture: Request exceeds timeout"""
    config = ServerConfig(request_timeout=1)
    app = Spikard(config=config)
    # Register handler with this app instance
    app.register_route("GET", "/timeouts/slow", body_schema=None, parameter_schema=None, file_params=None)(
        request_timeout_request_exceeds_timeout
    )
    return app


async def request_timeout_request_completes_before_timeout() -> Any:
    """Handler for GET /timeouts/fast."""
    await asyncio.sleep(0.1)
    return Response(
        content={"duration": "fast", "status": "ok"}, status_code=200, headers={"Content-Type": "application/json"}
    )


def create_app_request_timeout_request_completes_before_timeout() -> Spikard:
    """App factory for fixture: Request completes before timeout"""
    config = ServerConfig(request_timeout=2)
    app = Spikard(config=config)
    # Register handler with this app instance
    app.register_route("GET", "/timeouts/fast", body_schema=None, parameter_schema=None, file_params=None)(
        request_timeout_request_completes_before_timeout
    )
    return app


def json_bodies_uuid_field_invalid_format(
    body: dict[str, Any],
) -> Any:
    """Handler for POST /items/."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "input": "not-a-valid-uuid",
                    "loc": ["body", "item_id"],
                    "msg": "Input should be a valid UUID",
                    "type": "uuid_parsing",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_json_bodies_uuid_field_invalid_format() -> Spikard:
    """App factory for fixture: UUID field - invalid format"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/items/",
        body_schema={
            "additionalProperties": False,
            "properties": {"item_id": {"format": "uuid", "type": "string"}, "name": {"type": "string"}},
            "required": ["name", "item_id"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(json_bodies_uuid_field_invalid_format)
    return app


class JsonBodies44ConstValidationFailureBody(TypedDict):
    """Request body type (TypedDict - runtime is dict)."""

    data: str
    version: str


def json_bodies_44_const_validation_failure(
    body: JsonBodies44ConstValidationFailureBody,
) -> Any:
    """Handler for POST /api/v1/data."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"const": "1.0", "value": "2.0"},
                    "loc": ["body", "version"],
                    "msg": "Value must be exactly '1.0'",
                    "type": "validation_error",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_json_bodies_44_const_validation_failure() -> Spikard:
    """App factory for fixture: 44_const_validation_failure"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/api/v1/data",
        body_schema={
            "properties": {"data": {"type": "string"}, "version": {"const": "1.0", "type": "string"}},
            "required": ["version", "data"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(json_bodies_44_const_validation_failure)
    return app


@dataclass
class JsonBodiesBooleanFieldSuccessBody:
    """Request body dataclass."""

    in_stock: bool
    name: str
    price: float


def json_bodies_boolean_field_success(
    body: JsonBodiesBooleanFieldSuccessBody,
) -> Any:
    """Handler for POST /items/."""
    return Response(
        content={"in_stock": True, "name": "Item", "price": 42.0},
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_json_bodies_boolean_field_success() -> Spikard:
    """App factory for fixture: Boolean field - success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/items/",
        body_schema={
            "additionalProperties": False,
            "properties": {"in_stock": {"type": "boolean"}, "name": {"type": "string"}, "price": {"type": "number"}},
            "required": ["name", "price", "in_stock"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(json_bodies_boolean_field_success)
    return app


class JsonBodiesNumericLeValidationSuccessBody(NamedTuple):
    """Request body NamedTuple (immutable)."""

    name: str
    price: float


def json_bodies_numeric_le_validation_success(
    body: JsonBodiesNumericLeValidationSuccessBody,
) -> Any:
    """Handler for POST /items/validated."""
    return Response(
        content={"name": "Item", "price": 100.0}, status_code=200, headers={"Content-Type": "application/json"}
    )


def create_app_json_bodies_numeric_le_validation_success() -> Spikard:
    """App factory for fixture: Numeric le validation - success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/items/validated",
        body_schema={
            "additionalProperties": False,
            "properties": {"name": {"type": "string"}, "price": {"type": "number"}},
            "required": ["name", "price"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(json_bodies_numeric_le_validation_success)
    return app


class JsonBodiesDeeplyNestedObjectsBody(msgspec.Struct):
    """Request body msgspec.Struct (fast typed)."""

    name: str
    price: float
    seller: dict[str, Any]


def json_bodies_deeply_nested_objects(
    body: JsonBodiesDeeplyNestedObjectsBody,
) -> Any:
    """Handler for POST /items/nested."""
    return Response(
        content={
            "name": "Product",
            "price": 100.0,
            "seller": {
                "address": {"city": "Springfield", "country": {"code": "US", "name": "USA"}, "street": "123 Main St"},
                "name": "John Doe",
            },
        },
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_json_bodies_deeply_nested_objects() -> Spikard:
    """App factory for fixture: Deeply nested objects"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
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
        parameter_schema=None,
        file_params=None,
    )(json_bodies_deeply_nested_objects)
    return app


class JsonBodiesOptionalFieldsOmittedBody(BaseModel):
    """Request body Pydantic model."""

    name: str
    price: float


def json_bodies_optional_fields_omitted(
    body: JsonBodiesOptionalFieldsOmittedBody,
) -> Any:
    """Handler for POST /items/."""
    return Response(
        content={"description": None, "name": "Foo", "price": 35.4, "tax": None},
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_json_bodies_optional_fields_omitted() -> Spikard:
    """App factory for fixture: Optional fields - omitted"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/items/",
        body_schema={
            "additionalProperties": False,
            "properties": {"name": {"type": "string"}, "price": {"type": "number"}},
            "required": ["name", "price"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(json_bodies_optional_fields_omitted)
    return app


def json_bodies_uuid_field_success(
    body: dict[str, Any],
) -> Any:
    """Handler for POST /items/."""
    return Response(
        content={"item_id": "c892496f-b1fd-4b91-bdb8-b46f92df1716", "name": "Item"},
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_json_bodies_uuid_field_success() -> Spikard:
    """App factory for fixture: UUID field - success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/items/",
        body_schema={
            "additionalProperties": False,
            "properties": {"item_id": {"format": "uuid", "type": "string"}, "name": {"type": "string"}},
            "required": ["name", "item_id"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(json_bodies_uuid_field_success)
    return app


class JsonBodiesDateFieldSuccessBody(TypedDict):
    """Request body type (TypedDict - runtime is dict)."""

    event_date: str
    name: str


def json_bodies_date_field_success(
    body: JsonBodiesDateFieldSuccessBody,
) -> Any:
    """Handler for POST /events/."""
    return Response(
        content={"event_date": "2024-03-15", "name": "Conference"},
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_json_bodies_date_field_success() -> Spikard:
    """App factory for fixture: Date field - success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/events/",
        body_schema={
            "additionalProperties": False,
            "properties": {"event_date": {"type": "string"}, "name": {"type": "string"}},
            "required": ["name", "event_date"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(json_bodies_date_field_success)
    return app


@dataclass
class JsonBodies47MaxpropertiesValidationFailureBody:
    """Request body dataclass."""


def json_bodies_47_maxproperties_validation_failure(
    body: JsonBodies47MaxpropertiesValidationFailureBody,
) -> Any:
    """Handler for POST /config."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"actual_properties": 4, "max_properties": 3},
                    "loc": ["body"],
                    "msg": "Object must have at most 3 properties",
                    "type": "validation_error",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_json_bodies_47_maxproperties_validation_failure() -> Spikard:
    """App factory for fixture: 47_maxproperties_validation_failure"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST", "/config", body_schema={"maxProperties": 3, "type": "object"}, parameter_schema=None, file_params=None
    )(json_bodies_47_maxproperties_validation_failure)
    return app


class JsonBodies46MinpropertiesValidationFailureBody(NamedTuple):
    """Request body NamedTuple (immutable)."""


def json_bodies_46_minproperties_validation_failure(
    body: JsonBodies46MinpropertiesValidationFailureBody,
) -> Any:
    """Handler for POST /config."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"actual_properties": 1, "min_properties": 2},
                    "loc": ["body"],
                    "msg": "Object must have at least 2 properties",
                    "type": "validation_error",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_json_bodies_46_minproperties_validation_failure() -> Spikard:
    """App factory for fixture: 46_minproperties_validation_failure"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST", "/config", body_schema={"minProperties": 2, "type": "object"}, parameter_schema=None, file_params=None
    )(json_bodies_46_minproperties_validation_failure)
    return app


class JsonBodiesStringMinLengthValidationFailBody(msgspec.Struct):
    """Request body msgspec.Struct (fast typed)."""

    name: str
    price: float


def json_bodies_string_min_length_validation_fail(
    body: JsonBodiesStringMinLengthValidationFailBody,
) -> Any:
    """Handler for POST /items/validated."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"min_length": 3},
                    "input": "ab",
                    "loc": ["body", "name"],
                    "msg": "String should have at least 3 characters",
                    "type": "string_too_short",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_json_bodies_string_min_length_validation_fail() -> Spikard:
    """App factory for fixture: String min_length validation - fail"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/items/validated",
        body_schema={
            "additionalProperties": False,
            "properties": {"name": {"minLength": 3, "type": "string"}, "price": {"type": "number"}},
            "required": ["name", "price"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(json_bodies_string_min_length_validation_fail)
    return app


class JsonBodiesFieldTypeValidationInvalidTypeBody(BaseModel):
    """Request body Pydantic model."""

    description: str
    name: str
    price: float
    tax: float


def json_bodies_field_type_validation_invalid_type(
    body: JsonBodiesFieldTypeValidationInvalidTypeBody,
) -> Any:
    """Handler for POST /items/."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "input": "not a number",
                    "loc": ["body", "price"],
                    "msg": "Input should be a valid number",
                    "type": "float_parsing",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_json_bodies_field_type_validation_invalid_type() -> Spikard:
    """App factory for fixture: Field type validation - invalid type"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/items/",
        body_schema={
            "additionalProperties": False,
            "properties": {
                "description": {"type": "string"},
                "name": {"type": "string"},
                "price": {"type": "number"},
                "tax": {"type": "number"},
            },
            "required": ["name", "description", "price", "tax"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(json_bodies_field_type_validation_invalid_type)
    return app


def json_bodies_36_oneof_schema_multiple_match_failure(
    body: str,
) -> Any:
    """Handler for POST /payment."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"matched_schemas": 2},
                    "loc": ["body"],
                    "msg": "Must match exactly one schema (oneOf), but matched 2",
                    "type": "validation_error",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_json_bodies_36_oneof_schema_multiple_match_failure() -> Spikard:
    """App factory for fixture: 36_oneof_schema_multiple_match_failure"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
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
        parameter_schema=None,
        file_params=None,
    )(json_bodies_36_oneof_schema_multiple_match_failure)
    return app


class JsonBodiesNestedObjectSuccessBody(TypedDict):
    """Request body type (TypedDict - runtime is dict)."""

    image: dict[str, Any]
    name: str
    price: float


def json_bodies_nested_object_success(
    body: JsonBodiesNestedObjectSuccessBody,
) -> Any:
    """Handler for POST /items/nested."""
    return Response(
        content={
            "image": {"name": "Product Image", "url": "https://example.com/image.jpg"},
            "name": "Foo",
            "price": 42.0,
        },
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_json_bodies_nested_object_success() -> Spikard:
    """App factory for fixture: Nested object - success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/items/nested",
        body_schema={
            "additionalProperties": False,
            "properties": {
                "image": {
                    "additionalProperties": False,
                    "properties": {"name": {"type": "string"}, "url": {"type": "string"}},
                    "required": ["url", "name"],
                    "type": "object",
                },
                "name": {"type": "string"},
                "price": {"type": "number"},
            },
            "required": ["name", "price", "image"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(json_bodies_nested_object_success)
    return app


@dataclass
class JsonBodies41NotSchemaSuccessBody:
    """Request body dataclass."""

    username: str


def json_bodies_41_not_schema_success(
    body: JsonBodies41NotSchemaSuccessBody,
) -> Any:
    """Handler for POST /users."""
    return Response(status_code=201)


def create_app_json_bodies_41_not_schema_success() -> Spikard:
    """App factory for fixture: 41_not_schema_success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/users",
        body_schema={
            "properties": {"username": {"not": {"enum": ["admin", "root", "system"]}, "type": "string"}},
            "required": ["username"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(json_bodies_41_not_schema_success)
    return app


class JsonBodiesStringMaxLengthValidationFailBody(NamedTuple):
    """Request body NamedTuple (immutable)."""

    name: str
    price: float


def json_bodies_string_max_length_validation_fail(
    body: JsonBodiesStringMaxLengthValidationFailBody,
) -> Any:
    """Handler for POST /items/validated."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"max_length": 50},
                    "input": "This is a very long name that exceeds the maximum length",
                    "loc": ["body", "name"],
                    "msg": "String should have at most 50 characters",
                    "type": "string_too_long",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_json_bodies_string_max_length_validation_fail() -> Spikard:
    """App factory for fixture: String max_length validation - fail"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/items/validated",
        body_schema={
            "additionalProperties": False,
            "properties": {"name": {"maxLength": 50, "type": "string"}, "price": {"type": "number"}},
            "required": ["name", "price"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(json_bodies_string_max_length_validation_fail)
    return app


class JsonBodies50DeepNesting4LevelsBody(msgspec.Struct):
    """Request body msgspec.Struct (fast typed)."""

    user: dict[str, Any]


def json_bodies_50_deep_nesting_4_levels(
    body: JsonBodies50DeepNesting4LevelsBody,
) -> Any:
    """Handler for POST /data."""
    return Response(status_code=201)


def create_app_json_bodies_50_deep_nesting_4_levels() -> Spikard:
    """App factory for fixture: 50_deep_nesting_4_levels"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/data",
        body_schema={
            "properties": {
                "user": {
                    "properties": {
                        "profile": {
                            "properties": {
                                "contact": {
                                    "properties": {
                                        "address": {
                                            "properties": {"street": {"type": "string"}},
                                            "required": ["street"],
                                            "type": "object",
                                        }
                                    },
                                    "required": ["address"],
                                    "type": "object",
                                }
                            },
                            "required": ["contact"],
                            "type": "object",
                        }
                    },
                    "required": ["profile"],
                    "type": "object",
                }
            },
            "required": ["user"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(json_bodies_50_deep_nesting_4_levels)
    return app


class JsonBodies48DependenciesValidationSuccessBody(BaseModel):
    """Request body Pydantic model."""

    billing_address: str | None = None
    credit_card: str | None = None
    name: str | None = None


def json_bodies_48_dependencies_validation_success(
    body: JsonBodies48DependenciesValidationSuccessBody,
) -> Any:
    """Handler for POST /billing."""
    return Response(status_code=201)


def create_app_json_bodies_48_dependencies_validation_success() -> Spikard:
    """App factory for fixture: 48_dependencies_validation_success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
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
        parameter_schema=None,
        file_params=None,
    )(json_bodies_48_dependencies_validation_success)
    return app


def json_bodies_patch_partial_update(
    body: dict[str, Any],
    id: str,
) -> Any:
    """Handler for PATCH /items/{id}."""
    return Response(
        content={"description": "Original description", "name": "Original Item", "price": 45.0},
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_json_bodies_patch_partial_update() -> Spikard:
    """App factory for fixture: PATCH partial update"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "PATCH",
        "/items/{id}",
        body_schema={"properties": {"price": {"type": "number"}}, "required": ["price"], "type": "object"},
        parameter_schema={
            "properties": {"id": {"source": "path", "type": "string"}},
            "required": ["id"],
            "type": "object",
        },
        file_params=None,
    )(json_bodies_patch_partial_update)
    return app


class JsonBodies30NestedObjectMissingFieldBody(TypedDict):
    """Request body type (TypedDict - runtime is dict)."""

    profile: dict[str, Any]


def json_bodies_30_nested_object_missing_field(
    body: JsonBodies30NestedObjectMissingFieldBody,
) -> Any:
    """Handler for POST /users."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"required": True},
                    "loc": ["body", "profile", "email"],
                    "msg": "Field required",
                    "type": "validation_error",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_json_bodies_30_nested_object_missing_field() -> Spikard:
    """App factory for fixture: 30_nested_object_missing_field"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/users",
        body_schema={
            "properties": {
                "profile": {
                    "properties": {
                        "email": {"format": "email", "type": "string"},
                        "name": {"minLength": 1, "type": "string"},
                    },
                    "required": ["name", "email"],
                    "type": "object",
                }
            },
            "required": ["profile"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(json_bodies_30_nested_object_missing_field)
    return app


@dataclass
class JsonBodiesDatetimeFieldSuccessBody:
    """Request body dataclass."""

    created_at: datetime
    name: str


def json_bodies_datetime_field_success(
    body: JsonBodiesDatetimeFieldSuccessBody,
) -> Any:
    """Handler for POST /events/."""
    return Response(
        content={"created_at": "2024-03-15T10:30:00Z", "name": "Meeting"},
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_json_bodies_datetime_field_success() -> Spikard:
    """App factory for fixture: Datetime field - success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/events/",
        body_schema={
            "additionalProperties": False,
            "properties": {"created_at": {"format": "date-time", "type": "string"}, "name": {"type": "string"}},
            "required": ["name", "created_at"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(json_bodies_datetime_field_success)
    return app


class JsonBodiesStringPatternValidationSuccessBody(NamedTuple):
    """Request body NamedTuple (immutable)."""

    name: str
    sku: str


def json_bodies_string_pattern_validation_success(
    body: JsonBodiesStringPatternValidationSuccessBody,
) -> Any:
    """Handler for POST /items/validated."""
    return Response(
        content={"name": "Item", "sku": "ABC1234"}, status_code=200, headers={"Content-Type": "application/json"}
    )


def create_app_json_bodies_string_pattern_validation_success() -> Spikard:
    """App factory for fixture: String pattern validation - success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/items/validated",
        body_schema={
            "additionalProperties": False,
            "properties": {"name": {"type": "string"}, "sku": {"type": "string"}},
            "required": ["name", "sku"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(json_bodies_string_pattern_validation_success)
    return app


class JsonBodiesExtraFieldsIgnoredNoAdditionalpropertiesBody(msgspec.Struct):
    """Request body msgspec.Struct (fast typed)."""

    another_extra: int
    extra_field: str
    name: str
    price: float


def json_bodies_extra_fields_ignored_no_additionalproperties(
    body: JsonBodiesExtraFieldsIgnoredNoAdditionalpropertiesBody,
) -> Any:
    """Handler for POST /items/."""
    return Response(
        content={"name": "Item", "price": 42.0}, status_code=200, headers={"Content-Type": "application/json"}
    )


def create_app_json_bodies_extra_fields_ignored_no_additionalproperties() -> Spikard:
    """App factory for fixture: Extra fields ignored (no additionalProperties)"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/items/",
        body_schema={
            "additionalProperties": False,
            "properties": {
                "another_extra": {"type": "integer"},
                "extra_field": {"type": "string"},
                "name": {"type": "string"},
                "price": {"type": "number"},
            },
            "required": ["name", "price", "extra_field", "another_extra"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(json_bodies_extra_fields_ignored_no_additionalproperties)
    return app


class JsonBodies40AnyofSchemaFailureBody(BaseModel):
    """Request body Pydantic model."""

    name: str
    email: str | None = None
    phone: str | None = None


def json_bodies_40_anyof_schema_failure(
    body: JsonBodies40AnyofSchemaFailureBody,
) -> Any:
    """Handler for POST /contact."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"matched_schemas": 0},
                    "loc": ["body"],
                    "msg": "Must match at least one schema (anyOf), but matched 0",
                    "type": "validation_error",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_json_bodies_40_anyof_schema_failure() -> Spikard:
    """App factory for fixture: 40_anyof_schema_failure"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
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
        parameter_schema=None,
        file_params=None,
    )(json_bodies_40_anyof_schema_failure)
    return app


def json_bodies_39_anyof_schema_multiple_match_success(
    body: dict[str, Any],
) -> Any:
    """Handler for POST /contact."""
    return Response(status_code=201)


def create_app_json_bodies_39_anyof_schema_multiple_match_success() -> Spikard:
    """App factory for fixture: 39_anyof_schema_multiple_match_success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
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
        parameter_schema=None,
        file_params=None,
    )(json_bodies_39_anyof_schema_multiple_match_success)
    return app


class JsonBodiesArrayOfPrimitiveValuesBody(TypedDict):
    """Request body type (TypedDict - runtime is dict)."""

    name: str
    ratings: list[float]
    tags: list[str]


def json_bodies_array_of_primitive_values(
    body: JsonBodiesArrayOfPrimitiveValuesBody,
) -> Any:
    """Handler for POST /items/."""
    return Response(
        content={"name": "Product", "ratings": [4.5, 4.8, 5.0, 4.2], "tags": ["electronics", "gadget", "new"]},
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_json_bodies_array_of_primitive_values() -> Spikard:
    """App factory for fixture: Array of primitive values"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/items/",
        body_schema={
            "additionalProperties": False,
            "properties": {
                "name": {"type": "string"},
                "ratings": {"items": {"type": "number"}, "type": "array"},
                "tags": {"items": {"type": "string"}, "type": "array"},
            },
            "required": ["name", "tags", "ratings"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(json_bodies_array_of_primitive_values)
    return app


@dataclass
class JsonBodiesNumericGeValidationFailBody:
    """Request body dataclass."""

    name: str
    price: float


def json_bodies_numeric_ge_validation_fail(
    body: JsonBodiesNumericGeValidationFailBody,
) -> Any:
    """Handler for POST /items/validated."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"ge": 1},
                    "input": 0.5,
                    "loc": ["body", "price"],
                    "msg": "Input should be greater than or equal to 1",
                    "type": "greater_than_equal",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_json_bodies_numeric_ge_validation_fail() -> Spikard:
    """App factory for fixture: Numeric ge validation - fail"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/items/validated",
        body_schema={
            "additionalProperties": False,
            "properties": {"name": {"type": "string"}, "price": {"minimum": 1, "type": "number"}},
            "required": ["name", "price"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(json_bodies_numeric_ge_validation_fail)
    return app


class JsonBodies37OneofSchemaNoMatchFailureBody(NamedTuple):
    """Request body NamedTuple (immutable)."""


def json_bodies_37_oneof_schema_no_match_failure(
    body: JsonBodies37OneofSchemaNoMatchFailureBody,
) -> Any:
    """Handler for POST /payment."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"matched_schemas": 0},
                    "loc": ["body"],
                    "msg": "Must match exactly one schema (oneOf), but matched 0",
                    "type": "validation_error",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_json_bodies_37_oneof_schema_no_match_failure() -> Spikard:
    """App factory for fixture: 37_oneof_schema_no_match_failure"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
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
        parameter_schema=None,
        file_params=None,
    )(json_bodies_37_oneof_schema_no_match_failure)
    return app


class JsonBodiesEmptyArrayValidationFailBody(msgspec.Struct):
    """Request body msgspec.Struct (fast typed)."""

    name: str
    tags: list[str]


def json_bodies_empty_array_validation_fail(
    body: JsonBodiesEmptyArrayValidationFailBody,
) -> Any:
    """Handler for POST /items/list-validated."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"min_length": 1},
                    "input": [],
                    "loc": ["body", "tags"],
                    "msg": "List should have at least 1 item after validation",
                    "type": "too_short",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_json_bodies_empty_array_validation_fail() -> Spikard:
    """App factory for fixture: Empty array validation - fail"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/items/list-validated",
        body_schema={
            "additionalProperties": False,
            "properties": {"name": {"type": "string"}, "tags": {"items": {}, "minItems": 1, "type": "array"}},
            "required": ["name", "tags"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(json_bodies_empty_array_validation_fail)
    return app


class JsonBodies38AnyofSchemaSuccessBody(BaseModel):
    """Request body Pydantic model."""

    name: str


def json_bodies_38_anyof_schema_success(
    body: JsonBodies38AnyofSchemaSuccessBody,
) -> Any:
    """Handler for POST /contact."""
    return Response(status_code=201)


def create_app_json_bodies_38_anyof_schema_success() -> Spikard:
    """App factory for fixture: 38_anyof_schema_success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/contact",
        body_schema={
            "anyOf": [{"required": ["email"]}, {"required": ["phone"]}],
            "properties": {"name": {"type": "string"}},
            "required": ["name"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(json_bodies_38_anyof_schema_success)
    return app


def json_bodies_empty_json_object(
    body: dict[str, Any],
) -> Any:
    """Handler for POST /items/optional-all."""
    return Response(
        content={"description": None, "name": None, "price": None, "tax": None},
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_json_bodies_empty_json_object() -> Spikard:
    """App factory for fixture: Empty JSON object"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/items/optional-all",
        body_schema={"additionalProperties": False, "properties": {}, "type": "object"},
        parameter_schema=None,
        file_params=None,
    )(json_bodies_empty_json_object)
    return app


class JsonBodiesStringPatternValidationFailBody(TypedDict):
    """Request body type (TypedDict - runtime is dict)."""

    name: str
    sku: str


def json_bodies_string_pattern_validation_fail(
    body: JsonBodiesStringPatternValidationFailBody,
) -> Any:
    """Handler for POST /items/validated."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"pattern": "^[A-Z]{3}[0-9]{4}$"},
                    "input": "ABC-123",
                    "loc": ["body", "sku"],
                    "msg": "String should match pattern '^[A-Z]{3}[0-9]{4}$'",
                    "type": "string_pattern_mismatch",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_json_bodies_string_pattern_validation_fail() -> Spikard:
    """App factory for fixture: String pattern validation - fail"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/items/validated",
        body_schema={
            "additionalProperties": False,
            "properties": {"name": {"type": "string"}, "sku": {"pattern": "^[A-Z]{3}[0-9]{4}$", "type": "string"}},
            "required": ["name", "sku"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(json_bodies_string_pattern_validation_fail)
    return app


@dataclass
class JsonBodies49DependenciesValidationFailureBody:
    """Request body dataclass."""

    billing_address: str | None = None
    credit_card: str | None = None
    name: str | None = None


def json_bodies_49_dependencies_validation_failure(
    body: JsonBodies49DependenciesValidationFailureBody,
) -> Any:
    """Handler for POST /billing."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"dependency": "credit_card", "required_fields": ["billing_address"]},
                    "loc": ["body"],
                    "msg": "When 'credit_card' is present, 'billing_address' is required",
                    "type": "validation_error",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_json_bodies_49_dependencies_validation_failure() -> Spikard:
    """App factory for fixture: 49_dependencies_validation_failure"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
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
        parameter_schema=None,
        file_params=None,
    )(json_bodies_49_dependencies_validation_failure)
    return app


class JsonBodiesSimpleJsonObjectSuccessBody(NamedTuple):
    """Request body NamedTuple (immutable)."""

    description: str
    name: str
    price: float
    tax: float


def json_bodies_simple_json_object_success(
    body: JsonBodiesSimpleJsonObjectSuccessBody,
) -> Any:
    """Handler for POST /items/."""
    return Response(
        content={"description": "A very nice Item", "name": "Foo", "price": 35.4, "tax": 3.2},
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_json_bodies_simple_json_object_success() -> Spikard:
    """App factory for fixture: Simple JSON object - success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/items/",
        body_schema={
            "additionalProperties": False,
            "properties": {
                "description": {"type": "string"},
                "name": {"type": "string"},
                "price": {"type": "number"},
                "tax": {"type": "number"},
            },
            "required": ["name", "description", "price", "tax"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(json_bodies_simple_json_object_success)
    return app


class JsonBodiesRequiredFieldMissingValidationErrorBody(msgspec.Struct):
    """Request body msgspec.Struct (fast typed)."""

    description: str
    name: str
    price: float


def json_bodies_required_field_missing_validation_error(
    body: JsonBodiesRequiredFieldMissingValidationErrorBody,
) -> Any:
    """Handler for POST /items/."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [{"input": "", "loc": ["body", "name"], "msg": "Field required", "type": "missing"}],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_json_bodies_required_field_missing_validation_error() -> Spikard:
    """App factory for fixture: Required field missing - validation error"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/items/",
        body_schema={
            "additionalProperties": False,
            "properties": {"description": {"type": "string"}, "name": {"type": "string"}, "price": {"type": "number"}},
            "required": ["description", "price", "name"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(json_bodies_required_field_missing_validation_error)
    return app


class JsonBodies35OneofSchemaSuccessBody(BaseModel):
    """Request body Pydantic model."""


def json_bodies_35_oneof_schema_success(
    body: JsonBodies35OneofSchemaSuccessBody,
) -> Any:
    """Handler for POST /payment."""
    return Response(status_code=201)


def create_app_json_bodies_35_oneof_schema_success() -> Spikard:
    """App factory for fixture: 35_oneof_schema_success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
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
        parameter_schema=None,
        file_params=None,
    )(json_bodies_35_oneof_schema_success)
    return app


def json_bodies_enum_field_invalid_value(
    body: dict[str, Any],
) -> Any:
    """Handler for POST /items/."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"expected": "'electronics', 'clothing' or 'books'"},
                    "input": "furniture",
                    "loc": ["body", "category"],
                    "msg": "Input should be 'electronics', 'clothing' or 'books'",
                    "type": "enum",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_json_bodies_enum_field_invalid_value() -> Spikard:
    """App factory for fixture: Enum field - invalid value"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/items/",
        body_schema={
            "additionalProperties": False,
            "properties": {
                "category": {"enum": ["electronics", "clothing", "books"], "type": "string"},
                "name": {"type": "string"},
            },
            "required": ["name", "category"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(json_bodies_enum_field_invalid_value)
    return app


class JsonBodiesEnumFieldSuccessBody(TypedDict):
    """Request body type (TypedDict - runtime is dict)."""

    category: str
    name: str


def json_bodies_enum_field_success(
    body: JsonBodiesEnumFieldSuccessBody,
) -> Any:
    """Handler for POST /items/."""
    return Response(
        content={"category": "electronics", "name": "Item"},
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_json_bodies_enum_field_success() -> Spikard:
    """App factory for fixture: Enum field - success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/items/",
        body_schema={
            "additionalProperties": False,
            "properties": {"category": {"type": "string"}, "name": {"type": "string"}},
            "required": ["name", "category"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(json_bodies_enum_field_success)
    return app


@dataclass
class JsonBodies33AllofSchemaCompositionBody:
    """Request body dataclass."""


def json_bodies_33_allof_schema_composition(
    body: JsonBodies33AllofSchemaCompositionBody,
) -> Any:
    """Handler for POST /items."""
    return Response(status_code=201)


def create_app_json_bodies_33_allof_schema_composition() -> Spikard:
    """App factory for fixture: 33_allof_schema_composition"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/items",
        body_schema={
            "allOf": [
                {"properties": {"name": {"type": "string"}}, "required": ["name"], "type": "object"},
                {"properties": {"price": {"minimum": 0, "type": "number"}}, "required": ["price"], "type": "object"},
            ]
        },
        parameter_schema=None,
        file_params=None,
    )(json_bodies_33_allof_schema_composition)
    return app


class JsonBodies45MinpropertiesValidationSuccessBody(NamedTuple):
    """Request body NamedTuple (immutable)."""


def json_bodies_45_minproperties_validation_success(
    body: JsonBodies45MinpropertiesValidationSuccessBody,
) -> Any:
    """Handler for POST /config."""
    return Response(status_code=201)


def create_app_json_bodies_45_minproperties_validation_success() -> Spikard:
    """App factory for fixture: 45_minproperties_validation_success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST", "/config", body_schema={"minProperties": 2, "type": "object"}, parameter_schema=None, file_params=None
    )(json_bodies_45_minproperties_validation_success)
    return app


class JsonBodiesBodyWithQueryParametersBody(msgspec.Struct):
    """Request body msgspec.Struct (fast typed)."""

    name: str
    price: float


def json_bodies_body_with_query_parameters(
    body: JsonBodiesBodyWithQueryParametersBody,
    limit: int,
) -> Any:
    """Handler for POST /items/."""
    return Response(
        content={"item": {"name": "Item", "price": 42.0}, "limit": 10},
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_json_bodies_body_with_query_parameters() -> Spikard:
    """App factory for fixture: Body with query parameters"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/items/",
        body_schema={
            "additionalProperties": False,
            "properties": {"name": {"type": "string"}, "price": {"type": "number"}},
            "required": ["name", "price"],
            "type": "object",
        },
        parameter_schema={
            "properties": {"limit": {"source": "query", "type": "integer"}},
            "required": ["limit"],
            "type": "object",
        },
        file_params=None,
    )(json_bodies_body_with_query_parameters)
    return app


class JsonBodies42NotSchemaFailureBody(BaseModel):
    """Request body Pydantic model."""

    username: str


def json_bodies_42_not_schema_failure(
    body: JsonBodies42NotSchemaFailureBody,
) -> Any:
    """Handler for POST /users."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"prohibited_value": "admin"},
                    "loc": ["body", "username"],
                    "msg": "Must not match the schema",
                    "type": "validation_error",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_json_bodies_42_not_schema_failure() -> Spikard:
    """App factory for fixture: 42_not_schema_failure"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/users",
        body_schema={
            "properties": {"username": {"not": {"enum": ["admin", "root", "system"]}, "type": "string"}},
            "required": ["username"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(json_bodies_42_not_schema_failure)
    return app


def json_bodies_43_const_validation_success(
    body: dict[str, Any],
) -> Any:
    """Handler for POST /api/v1/data."""
    return Response(status_code=201)


def create_app_json_bodies_43_const_validation_success() -> Spikard:
    """App factory for fixture: 43_const_validation_success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/api/v1/data",
        body_schema={
            "properties": {"data": {"type": "string"}, "version": {"const": "1.0", "type": "string"}},
            "required": ["version", "data"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(json_bodies_43_const_validation_success)
    return app


class JsonBodies32SchemaRefDefinitionsBody(TypedDict):
    """Request body type (TypedDict - runtime is dict)."""

    product: str


def json_bodies_32_schema_ref_definitions(
    body: JsonBodies32SchemaRefDefinitionsBody,
) -> Any:
    """Handler for POST /products."""
    return Response(status_code=201)


def create_app_json_bodies_32_schema_ref_definitions() -> Spikard:
    """App factory for fixture: 32_schema_ref_definitions"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
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
        parameter_schema=None,
        file_params=None,
    )(json_bodies_32_schema_ref_definitions)
    return app


@dataclass
class JsonBodies29NestedObjectValidationSuccessBody:
    """Request body dataclass."""

    profile: dict[str, Any]


def json_bodies_29_nested_object_validation_success(
    body: JsonBodies29NestedObjectValidationSuccessBody,
) -> Any:
    """Handler for POST /users."""
    return Response(status_code=201)


def create_app_json_bodies_29_nested_object_validation_success() -> Spikard:
    """App factory for fixture: 29_nested_object_validation_success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/users",
        body_schema={
            "properties": {
                "profile": {
                    "properties": {
                        "email": {"format": "email", "type": "string"},
                        "name": {"minLength": 1, "type": "string"},
                    },
                    "required": ["name", "email"],
                    "type": "object",
                }
            },
            "required": ["profile"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(json_bodies_29_nested_object_validation_success)
    return app


class JsonBodies34AdditionalPropertiesFalseBody(NamedTuple):
    """Request body NamedTuple (immutable)."""

    name: str
    email: str | None = None


def json_bodies_34_additional_properties_false(
    body: JsonBodies34AdditionalPropertiesFalseBody,
) -> Any:
    """Handler for POST /users."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"additional_properties": False, "unexpected_field": "extra_field"},
                    "loc": ["body", "extra_field"],
                    "msg": "Additional properties are not allowed",
                    "type": "validation_error",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_json_bodies_34_additional_properties_false() -> Spikard:
    """App factory for fixture: 34_additional_properties_false"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/users",
        body_schema={
            "additionalProperties": False,
            "properties": {"email": {"type": "string"}, "name": {"type": "string"}},
            "required": ["name"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(json_bodies_34_additional_properties_false)
    return app


class JsonBodiesNullValueForOptionalFieldBody(msgspec.Struct):
    """Request body msgspec.Struct (fast typed)."""

    description: Any
    name: str
    price: float
    tax: Any


def json_bodies_null_value_for_optional_field(
    body: JsonBodiesNullValueForOptionalFieldBody,
) -> Any:
    """Handler for POST /items/."""
    return Response(
        content={"description": None, "name": "Item", "price": 42.0, "tax": None},
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_json_bodies_null_value_for_optional_field() -> Spikard:
    """App factory for fixture: Null value for optional field"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/items/",
        body_schema={
            "additionalProperties": False,
            "properties": {
                "description": {"type": "null"},
                "name": {"type": "string"},
                "price": {"type": "number"},
                "tax": {"type": "null"},
            },
            "required": ["name", "price", "description", "tax"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(json_bodies_null_value_for_optional_field)
    return app


class JsonBodies31NullablePropertyNullValueBody(BaseModel):
    """Request body Pydantic model."""

    name: str
    description: str | None = None


def json_bodies_31_nullable_property_null_value(
    body: JsonBodies31NullablePropertyNullValueBody,
) -> Any:
    """Handler for POST /users."""
    return Response(status_code=201)


def create_app_json_bodies_31_nullable_property_null_value() -> Spikard:
    """App factory for fixture: 31_nullable_property_null_value"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/users",
        body_schema={
            "properties": {"description": {"type": ["string", "null"]}, "name": {"type": "string"}},
            "required": ["name"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(json_bodies_31_nullable_property_null_value)
    return app


def json_bodies_array_of_objects_success(
    body: dict[str, Any],
) -> Any:
    """Handler for POST /items/list."""
    return Response(
        content={
            "images": [
                {"name": "Front", "url": "https://example.com/img1.jpg"},
                {"name": "Back", "url": "https://example.com/img2.jpg"},
            ],
            "name": "Product Bundle",
            "tags": ["electronics", "gadget"],
        },
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_json_bodies_array_of_objects_success() -> Spikard:
    """App factory for fixture: Array of objects - success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
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
        parameter_schema=None,
        file_params=None,
    )(json_bodies_array_of_objects_success)
    return app


def cors_07_cors_preflight_header_not_allowed(
    access_control_request_headers: str | None = None,
    access_control_request_method: str | None = None,
    origin: str | None = None,
) -> Any:
    """Handler for POST /api/data."""
    return Response(status_code=403)


def create_app_cors_07_cors_preflight_header_not_allowed() -> Spikard:
    """App factory for fixture: 07_cors_preflight_header_not_allowed"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/api/data",
        body_schema=None,
        parameter_schema={
            "properties": {
                "access_control_request_headers": {"source": "header", "type": "string"},
                "access_control_request_method": {"source": "header", "type": "string"},
                "origin": {"source": "header", "type": "string"},
            },
            "required": [],
            "type": "object",
        },
        file_params=None,
    )(cors_07_cors_preflight_header_not_allowed)
    app.register_route(
        "OPTIONS",
        "/api/data",
        body_schema=None,
        parameter_schema={
            "properties": {
                "access_control_request_headers": {"source": "header", "type": "string"},
                "access_control_request_method": {"source": "header", "type": "string"},
                "origin": {"source": "header", "type": "string"},
            },
            "required": [],
            "type": "object",
        },
        file_params=None,
    )(cors_07_cors_preflight_header_not_allowed)
    return app


def cors_cors_vary_header_for_proper_caching() -> Any:
    """Handler for GET /api/cached-resource."""
    return Response(
        content={"data": "cacheable resource"},
        status_code=200,
        headers={
            "Access-Control-Allow-Origin": "https://app.example.com",
            "Vary": "Origin",
            "Cache-Control": "public, max-age=3600",
            "Content-Type": "application/json",
        },
    )


def create_app_cors_cors_vary_header_for_proper_caching() -> Spikard:
    """App factory for fixture: CORS Vary header for proper caching"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route("GET", "/api/cached-resource", body_schema=None, parameter_schema=None, file_params=None)(
        cors_cors_vary_header_for_proper_caching
    )
    return app


def cors_cors_preflight_for_put_method() -> Any:
    """Handler for OPTIONS /api/resource/123."""
    return Response(
        status_code=204,
        headers={
            "Vary": "Origin",
            "Access-Control-Max-Age": "3600",
            "Access-Control-Allow-Methods": "GET, POST, PUT, PATCH, DELETE",
            "Access-Control-Allow-Headers": "Content-Type, X-Custom-Header",
            "Access-Control-Allow-Origin": "https://app.example.com",
        },
    )


def create_app_cors_cors_preflight_for_put_method() -> Spikard:
    """App factory for fixture: CORS preflight for PUT method"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route("OPTIONS", "/api/resource/123", body_schema=None, parameter_schema=None, file_params=None)(
        cors_cors_preflight_for_put_method
    )
    return app


def cors_cors_preflight_for_delete_method() -> Any:
    """Handler for OPTIONS /api/resource/456."""
    return Response(
        status_code=204,
        headers={
            "Vary": "Origin",
            "Access-Control-Max-Age": "3600",
            "Access-Control-Allow-Origin": "https://app.example.com",
            "Access-Control-Allow-Methods": "GET, POST, PUT, PATCH, DELETE",
        },
    )


def create_app_cors_cors_preflight_for_delete_method() -> Spikard:
    """App factory for fixture: CORS preflight for DELETE method"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route("OPTIONS", "/api/resource/456", body_schema=None, parameter_schema=None, file_params=None)(
        cors_cors_preflight_for_delete_method
    )
    return app


def cors_cors_multiple_allowed_origins() -> Any:
    """Handler for GET /api/data."""
    return Response(
        content={"data": "resource data"},
        status_code=200,
        headers={
            "Vary": "Origin",
            "Content-Type": "application/json",
            "Access-Control-Allow-Origin": "https://admin.example.com",
        },
    )


def create_app_cors_cors_multiple_allowed_origins() -> Spikard:
    """App factory for fixture: CORS multiple allowed origins"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route("GET", "/api/data", body_schema=None, parameter_schema=None, file_params=None)(
        cors_cors_multiple_allowed_origins
    )
    return app


def cors_cors_preflight_request() -> Any:
    """Handler for OPTIONS /items/."""
    return Response(
        status_code=200,
        headers={
            "Access-Control-Allow-Headers": "Content-Type, X-Custom-Header",
            "Access-Control-Max-Age": "600",
            "Access-Control-Allow-Methods": "GET, POST, PUT, DELETE, OPTIONS",
            "Access-Control-Allow-Origin": "https://example.com",
        },
    )


def create_app_cors_cors_preflight_request() -> Spikard:
    """App factory for fixture: CORS preflight request"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route("OPTIONS", "/items/", body_schema=None, parameter_schema=None, file_params=None)(
        cors_cors_preflight_request
    )
    return app


def cors_cors_with_credentials() -> Any:
    """Handler for GET /api/user/profile."""
    return Response(
        content={"username": "john"},
        status_code=200,
        headers={
            "Vary": "Origin",
            "Access-Control-Allow-Credentials": "true",
            "Content-Type": "application/json",
            "Access-Control-Allow-Origin": "https://app.example.com",
        },
    )


def create_app_cors_cors_with_credentials() -> Spikard:
    """App factory for fixture: CORS with credentials"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route("GET", "/api/user/profile", body_schema=None, parameter_schema=None, file_params=None)(
        cors_cors_with_credentials
    )
    return app


def cors_cors_regex_pattern_matching_for_origins() -> Any:
    """Handler for GET /api/data."""
    return Response(
        content={"data": "resource data"},
        status_code=200,
        headers={
            "Access-Control-Allow-Origin": "https://subdomain.example.com",
            "Content-Type": "application/json",
            "Vary": "Origin",
        },
    )


def create_app_cors_cors_regex_pattern_matching_for_origins() -> Spikard:
    """App factory for fixture: CORS regex pattern matching for origins"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route("GET", "/api/data", body_schema=None, parameter_schema=None, file_params=None)(
        cors_cors_regex_pattern_matching_for_origins
    )
    return app


def cors_08_cors_max_age(
    access_control_request_headers: str | None = None,
    access_control_request_method: str | None = None,
    origin: str | None = None,
) -> Any:
    """Handler for POST /api/data."""
    return Response(
        status_code=204,
        headers={
            "Access-Control-Allow-Headers": "Content-Type",
            "Access-Control-Allow-Origin": "https://example.com",
            "Access-Control-Allow-Methods": "POST",
            "Access-Control-Max-Age": "3600",
        },
    )


def create_app_cors_08_cors_max_age() -> Spikard:
    """App factory for fixture: 08_cors_max_age"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/api/data",
        body_schema=None,
        parameter_schema={
            "properties": {
                "access_control_request_headers": {"source": "header", "type": "string"},
                "access_control_request_method": {"source": "header", "type": "string"},
                "origin": {"source": "header", "type": "string"},
            },
            "required": [],
            "type": "object",
        },
        file_params=None,
    )(cors_08_cors_max_age)
    app.register_route(
        "OPTIONS",
        "/api/data",
        body_schema=None,
        parameter_schema={
            "properties": {
                "access_control_request_headers": {"source": "header", "type": "string"},
                "access_control_request_method": {"source": "header", "type": "string"},
                "origin": {"source": "header", "type": "string"},
            },
            "required": [],
            "type": "object",
        },
        file_params=None,
    )(cors_08_cors_max_age)
    return app


def cors_10_cors_origin_null(
    origin: str | None = None,
) -> Any:
    """Handler for GET /api/data."""
    return Response(
        content={"error": "Origin 'null' is not allowed"}, status_code=403, headers={"Content-Type": "application/json"}
    )


def create_app_cors_10_cors_origin_null() -> Spikard:
    """App factory for fixture: 10_cors_origin_null"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/api/data",
        body_schema=None,
        parameter_schema={
            "properties": {"origin": {"source": "header", "type": "string"}},
            "required": [],
            "type": "object",
        },
        file_params=None,
    )(cors_10_cors_origin_null)
    return app


def cors_cors_wildcard_origin() -> Any:
    """Handler for GET /public/data."""
    return Response(
        content={"data": "public"},
        status_code=200,
        headers={"Content-Type": "application/json", "Access-Control-Allow-Origin": "*"},
    )


def create_app_cors_cors_wildcard_origin() -> Spikard:
    """App factory for fixture: CORS wildcard origin"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route("GET", "/public/data", body_schema=None, parameter_schema=None, file_params=None)(
        cors_cors_wildcard_origin
    )
    return app


def cors_cors_safelisted_headers_without_preflight() -> Any:
    """Handler for POST /api/form."""
    return Response(
        content={"message": "Success"},
        status_code=200,
        headers={
            "Access-Control-Allow-Origin": "https://app.example.com",
            "Vary": "Origin",
            "Content-Type": "application/json",
        },
    )


def create_app_cors_cors_safelisted_headers_without_preflight() -> Spikard:
    """App factory for fixture: CORS safelisted headers without preflight"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route("POST", "/api/form", body_schema=None, parameter_schema=None, file_params=None)(
        cors_cors_safelisted_headers_without_preflight
    )
    return app


def cors_cors_private_network_access() -> Any:
    """Handler for OPTIONS /api/local-resource."""
    return Response(
        status_code=204,
        headers={
            "Access-Control-Allow-Private-Network": "true",
            "Vary": "Origin",
            "Access-Control-Allow-Origin": "https://public.example.com",
            "Access-Control-Allow-Methods": "GET, POST",
        },
    )


def create_app_cors_cors_private_network_access() -> Spikard:
    """App factory for fixture: CORS Private Network Access"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route("OPTIONS", "/api/local-resource", body_schema=None, parameter_schema=None, file_params=None)(
        cors_cors_private_network_access
    )
    return app


def cors_cors_origin_case_sensitivity() -> Any:
    """Handler for GET /api/data."""
    return Response(status_code=200, headers={"Vary": "Origin"})


def create_app_cors_cors_origin_case_sensitivity() -> Spikard:
    """App factory for fixture: CORS origin case sensitivity"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route("GET", "/api/data", body_schema=None, parameter_schema=None, file_params=None)(
        cors_cors_origin_case_sensitivity
    )
    return app


def cors_cors_request_blocked(
    origin: str | None = None,
) -> Any:
    """Handler for GET /items/."""
    return Response(
        content={"detail": "CORS request from origin 'https://malicious-site.com' not allowed"},
        status_code=403,
        headers={"Content-Type": "application/json"},
    )


def create_app_cors_cors_request_blocked() -> Spikard:
    """App factory for fixture: CORS request blocked"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/items/",
        body_schema=None,
        parameter_schema={
            "properties": {"origin": {"source": "header", "type": "string"}},
            "required": [],
            "type": "object",
        },
        file_params=None,
    )(cors_cors_request_blocked)
    return app


def cors_simple_cors_request() -> Any:
    """Handler for GET /items/."""
    return Response(
        content={"items": []},
        status_code=200,
        headers={
            "Access-Control-Allow-Origin": "https://example.com",
            "Vary": "Origin",
            "Content-Type": "application/json",
        },
    )


def create_app_cors_simple_cors_request() -> Spikard:
    """App factory for fixture: Simple CORS request"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route("GET", "/items/", body_schema=None, parameter_schema=None, file_params=None)(
        cors_simple_cors_request
    )
    return app


def cors_09_cors_expose_headers(
    origin: str | None = None,
) -> Any:
    """Handler for GET /api/data."""
    return Response(
        status_code=200,
        headers={
            "X-Request-Id": "abc123",
            "Access-Control-Allow-Origin": "https://example.com",
            "Access-Control-Expose-Headers": "X-Total-Count, X-Request-Id",
            "X-Total-Count": "42",
        },
    )


def create_app_cors_09_cors_expose_headers() -> Spikard:
    """App factory for fixture: 09_cors_expose_headers"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/api/data",
        body_schema=None,
        parameter_schema={
            "properties": {"origin": {"source": "header", "type": "string"}},
            "required": [],
            "type": "object",
        },
        file_params=None,
    )(cors_09_cors_expose_headers)
    return app


def cors_06_cors_preflight_method_not_allowed(
    access_control_request_headers: str | None = None,
    access_control_request_method: str | None = None,
    origin: str | None = None,
) -> Any:
    """Handler for GET /api/data."""
    return Response(status_code=403)


def create_app_cors_06_cors_preflight_method_not_allowed() -> Spikard:
    """App factory for fixture: 06_cors_preflight_method_not_allowed"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/api/data",
        body_schema=None,
        parameter_schema={
            "properties": {
                "access_control_request_headers": {"source": "header", "type": "string"},
                "access_control_request_method": {"source": "header", "type": "string"},
                "origin": {"source": "header", "type": "string"},
            },
            "required": [],
            "type": "object",
        },
        file_params=None,
    )(cors_06_cors_preflight_method_not_allowed)
    app.register_route(
        "OPTIONS",
        "/api/data",
        body_schema=None,
        parameter_schema={
            "properties": {
                "access_control_request_headers": {"source": "header", "type": "string"},
                "access_control_request_method": {"source": "header", "type": "string"},
                "origin": {"source": "header", "type": "string"},
            },
            "required": [],
            "type": "object",
        },
        file_params=None,
    )(cors_06_cors_preflight_method_not_allowed)
    return app


def cookies_25_cookie_samesite_lax(
    tracking: str,
) -> Any:
    """Handler for GET /data."""
    # Echo back parameters for testing
    result: dict[str, Any] = {}
    if tracking is not None:
        result["tracking"] = tracking
    return result


def create_app_cookies_25_cookie_samesite_lax() -> Spikard:
    """App factory for fixture: 25_cookie_samesite_lax"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/data",
        body_schema=None,
        parameter_schema={
            "properties": {"tracking": {"samesite": "Lax", "source": "cookie", "type": "string"}},
            "required": ["tracking"],
            "type": "object",
        },
        file_params=None,
    )(cookies_25_cookie_samesite_lax)
    return app


def cookies_optional_cookie_parameter_success(
    ads_id: str | None = None,
) -> Any:
    """Handler for GET /items/."""
    return Response(content={"ads_id": "abc123"}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_cookies_optional_cookie_parameter_success() -> Spikard:
    """App factory for fixture: Optional cookie parameter - success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/items/",
        body_schema=None,
        parameter_schema={
            "properties": {"ads_id": {"source": "cookie", "type": "string"}},
            "required": [],
            "type": "object",
        },
        file_params=None,
    )(cookies_optional_cookie_parameter_success)
    return app


def cookies_cookie_regex_pattern_validation_fail(
    tracking_id: str,
) -> Any:
    """Handler for GET /cookies/pattern."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"pattern": "^[A-Z0-9]{8}$"},
                    "input": "invalid-format",
                    "loc": ["cookie", "tracking_id"],
                    "msg": "String should match pattern '^[A-Z0-9]{8}$'",
                    "type": "string_pattern_mismatch",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_cookies_cookie_regex_pattern_validation_fail() -> Spikard:
    """App factory for fixture: Cookie regex pattern validation - fail"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/cookies/pattern",
        body_schema=None,
        parameter_schema={
            "properties": {"tracking_id": {"pattern": "^[A-Z0-9]{8}$", "source": "cookie", "type": "string"}},
            "required": ["tracking_id"],
            "type": "object",
        },
        file_params=None,
    )(cookies_cookie_regex_pattern_validation_fail)
    return app


class CookiesResponseSessionCookieNoMaxAgeBody(NamedTuple):
    """Request body NamedTuple (immutable)."""

    value: str


def cookies_response_session_cookie_no_max_age(
    body: CookiesResponseSessionCookieNoMaxAgeBody,
) -> Any:
    """Handler for POST /cookies/session."""
    return Response(
        content={"message": "Session cookie set"}, status_code=200, headers={"Content-Type": "application/json"}
    )


def create_app_cookies_response_session_cookie_no_max_age() -> Spikard:
    """App factory for fixture: Response - session cookie (no max_age)"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/cookies/session",
        body_schema={
            "additionalProperties": False,
            "properties": {"value": {"type": "string"}},
            "required": ["value"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(cookies_response_session_cookie_no_max_age)
    return app


def cookies_27_cookie_httponly_flag(
    session: str,
) -> Any:
    """Handler for GET /secure."""
    # Echo back parameters for testing
    result: dict[str, Any] = {}
    if session is not None:
        result["session"] = session
    return result


def create_app_cookies_27_cookie_httponly_flag() -> Spikard:
    """App factory for fixture: 27_cookie_httponly_flag"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/secure",
        body_schema=None,
        parameter_schema={
            "properties": {"session": {"httponly": True, "source": "cookie", "type": "string"}},
            "required": ["session"],
            "type": "object",
        },
        file_params=None,
    )(cookies_27_cookie_httponly_flag)
    return app


def cookies_response_cookie_with_attributes() -> Any:
    """Handler for GET /cookie/set."""
    return Response(content={"message": "Cookie set"}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_cookies_response_cookie_with_attributes() -> Spikard:
    """App factory for fixture: Response cookie with attributes"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route("GET", "/cookie/set", body_schema=None, parameter_schema=None, file_params=None)(
        cookies_response_cookie_with_attributes
    )
    return app


def cookies_24_cookie_samesite_strict(
    session_id: str,
) -> Any:
    """Handler for GET /secure."""
    # Echo back parameters for testing
    result: dict[str, Any] = {}
    if session_id is not None:
        result["session_id"] = session_id
    return result


def create_app_cookies_24_cookie_samesite_strict() -> Spikard:
    """App factory for fixture: 24_cookie_samesite_strict"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/secure",
        body_schema=None,
        parameter_schema={
            "properties": {"session_id": {"samesite": "Strict", "source": "cookie", "type": "string"}},
            "required": ["session_id"],
            "type": "object",
        },
        file_params=None,
    )(cookies_24_cookie_samesite_strict)
    return app


def cookies_apikey_cookie_authentication_success(
    key: str | None = None,
) -> Any:
    """Handler for GET /users/me."""
    return Response(content={"username": "secret"}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_cookies_apikey_cookie_authentication_success() -> Spikard:
    """App factory for fixture: APIKey cookie authentication - success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/users/me",
        body_schema=None,
        parameter_schema={
            "properties": {"key": {"source": "cookie", "type": "string"}},
            "required": [],
            "type": "object",
        },
        file_params=None,
    )(cookies_apikey_cookie_authentication_success)
    return app


def cookies_cookie_validation_min_length_constraint_success(
    token: str | None = None,
) -> Any:
    """Handler for GET /cookies/min-length."""
    return Response(content={"token": "abc"}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_cookies_cookie_validation_min_length_constraint_success() -> Spikard:
    """App factory for fixture: Cookie validation - min_length constraint success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/cookies/min-length",
        body_schema=None,
        parameter_schema={
            "properties": {"token": {"source": "cookie", "type": "string"}},
            "required": [],
            "type": "object",
        },
        file_params=None,
    )(cookies_cookie_validation_min_length_constraint_success)
    return app


def cookies_cookie_validation_min_length_failure(
    tracking_id: str,
) -> Any:
    """Handler for GET /items/."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "input": "",
                    "loc": ["cookie", "tracking_id"],
                    "msg": "String should have at least 3 characters",
                    "type": "string_too_short",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_cookies_cookie_validation_min_length_failure() -> Spikard:
    """App factory for fixture: Cookie validation - min_length failure"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/items/",
        body_schema=None,
        parameter_schema={
            "properties": {"tracking_id": {"minLength": 3, "source": "cookie", "type": "string"}},
            "required": ["tracking_id"],
            "type": "object",
        },
        file_params=None,
    )(cookies_cookie_validation_min_length_failure)
    return app


def cookies_cookie_validation_max_length_constraint_fail(
    session_id: str,
) -> Any:
    """Handler for GET /cookies/validated."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"max_length": 20},
                    "input": "this_cookie_value_is_way_too_long",
                    "loc": ["cookie", "session_id"],
                    "msg": "String should have at most 20 characters",
                    "type": "string_too_long",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_cookies_cookie_validation_max_length_constraint_fail() -> Spikard:
    """App factory for fixture: Cookie validation - max_length constraint fail"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/cookies/validated",
        body_schema=None,
        parameter_schema={
            "properties": {"session_id": {"maxLength": 20, "source": "cookie", "type": "string"}},
            "required": ["session_id"],
            "type": "object",
        },
        file_params=None,
    )(cookies_cookie_validation_max_length_constraint_fail)
    return app


def cookies_required_cookie_missing(
    session_id: str,
    fatebook_tracker: str | None = None,
) -> Any:
    """Handler for GET /items/cookies."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [{"input": "", "loc": ["cookie", "session_id"], "msg": "Field required", "type": "missing"}],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_cookies_required_cookie_missing() -> Spikard:
    """App factory for fixture: Required cookie - missing"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/items/cookies",
        body_schema=None,
        parameter_schema={
            "properties": {
                "fatebook_tracker": {"source": "cookie", "type": "string"},
                "session_id": {"source": "cookie", "type": "string"},
            },
            "required": ["session_id"],
            "type": "object",
        },
        file_params=None,
    )(cookies_required_cookie_missing)
    return app


def cookies_optional_cookie_parameter_missing(
    ads_id: str | None = None,
) -> Any:
    """Handler for GET /items/."""
    return Response(content={"ads_id": None}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_cookies_optional_cookie_parameter_missing() -> Spikard:
    """App factory for fixture: Optional cookie parameter - missing"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/items/",
        body_schema=None,
        parameter_schema={
            "properties": {"ads_id": {"source": "cookie", "type": "string"}},
            "required": [],
            "type": "object",
        },
        file_params=None,
    )(cookies_optional_cookie_parameter_missing)
    return app


def cookies_apikey_cookie_authentication_missing(
    key: str,
) -> Any:
    """Handler for GET /users/me/auth."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [{"input": None, "loc": ["cookie", "key"], "msg": "Field required", "type": "missing"}],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_cookies_apikey_cookie_authentication_missing() -> Spikard:
    """App factory for fixture: APIKey cookie authentication - missing"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/users/me/auth",
        body_schema=None,
        parameter_schema={
            "properties": {"key": {"source": "cookie", "type": "string"}},
            "required": ["key"],
            "type": "object",
        },
        file_params=None,
    )(cookies_apikey_cookie_authentication_missing)
    return app


@dataclass
class CookiesResponseMultipleCookiesBody:
    """Request body dataclass."""

    session: str
    user: str


def cookies_response_multiple_cookies(
    body: CookiesResponseMultipleCookiesBody,
) -> Any:
    """Handler for POST /cookies/multiple."""
    return Response(
        content={"message": "Multiple cookies set"}, status_code=200, headers={"Content-Type": "application/json"}
    )


def create_app_cookies_response_multiple_cookies() -> Spikard:
    """App factory for fixture: Response - multiple cookies"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/cookies/multiple",
        body_schema={
            "additionalProperties": False,
            "properties": {"session": {"type": "string"}, "user": {"type": "string"}},
            "required": ["user", "session"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(cookies_response_multiple_cookies)
    return app


class CookiesResponseCookieWithSamesiteLaxBody(NamedTuple):
    """Request body NamedTuple (immutable)."""

    value: str


def cookies_response_cookie_with_samesite_lax(
    body: CookiesResponseCookieWithSamesiteLaxBody,
) -> Any:
    """Handler for POST /cookies/samesite-lax."""
    return Response(
        content={"message": "Cookie set with SameSite=Lax"},
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_cookies_response_cookie_with_samesite_lax() -> Spikard:
    """App factory for fixture: Response cookie with SameSite=Lax"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/cookies/samesite-lax",
        body_schema={
            "additionalProperties": False,
            "properties": {"value": {"type": "string"}},
            "required": ["value"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(cookies_response_cookie_with_samesite_lax)
    return app


def cookies_response_delete_cookie(
    session: str | None = None,
) -> Any:
    """Handler for POST /cookies/delete."""
    return Response(
        content={"message": "Cookie deleted"}, status_code=200, headers={"Content-Type": "application/json"}
    )


def create_app_cookies_response_delete_cookie() -> Spikard:
    """App factory for fixture: Response - delete cookie"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/cookies/delete",
        body_schema=None,
        parameter_schema={
            "properties": {"session": {"source": "cookie", "type": "string"}},
            "required": [],
            "type": "object",
        },
        file_params=None,
    )(cookies_response_delete_cookie)
    return app


class CookiesResponseCookieWithPathAttributeBody(BaseModel):
    """Request body Pydantic model."""

    value: str


def cookies_response_cookie_with_path_attribute(
    body: CookiesResponseCookieWithPathAttributeBody,
) -> Any:
    """Handler for POST /cookies/set-with-path."""
    return Response(
        content={"message": "Cookie set with path"}, status_code=200, headers={"Content-Type": "application/json"}
    )


def create_app_cookies_response_cookie_with_path_attribute() -> Spikard:
    """App factory for fixture: Response cookie with path attribute"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/cookies/set-with-path",
        body_schema={
            "additionalProperties": False,
            "properties": {"value": {"type": "string"}},
            "required": ["value"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(cookies_response_cookie_with_path_attribute)
    return app


def cookies_optional_apikey_cookie_missing(
    key: str | None = None,
) -> Any:
    """Handler for GET /users/me."""
    return Response(
        content={"msg": "Create an account first"}, status_code=200, headers={"Content-Type": "application/json"}
    )


def create_app_cookies_optional_apikey_cookie_missing() -> Spikard:
    """App factory for fixture: Optional APIKey cookie - missing"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/users/me",
        body_schema=None,
        parameter_schema={
            "properties": {"key": {"source": "cookie", "type": "string"}},
            "required": [],
            "type": "object",
        },
        file_params=None,
    )(cookies_optional_apikey_cookie_missing)
    return app


class CookiesResponseCookieWithSamesiteStrictBody(TypedDict):
    """Request body type (TypedDict - runtime is dict)."""

    value: str


def cookies_response_cookie_with_samesite_strict(
    body: CookiesResponseCookieWithSamesiteStrictBody,
) -> Any:
    """Handler for POST /cookies/samesite-strict."""
    return Response(
        content={"message": "Cookie set with SameSite=Strict"},
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_cookies_response_cookie_with_samesite_strict() -> Spikard:
    """App factory for fixture: Response cookie with SameSite=Strict"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/cookies/samesite-strict",
        body_schema={
            "additionalProperties": False,
            "properties": {"value": {"type": "string"}},
            "required": ["value"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(cookies_response_cookie_with_samesite_strict)
    return app


@dataclass
class CookiesResponseCookieWithSamesiteNoneBody:
    """Request body dataclass."""

    value: str


def cookies_response_cookie_with_samesite_none(
    body: CookiesResponseCookieWithSamesiteNoneBody,
) -> Any:
    """Handler for POST /cookies/samesite-none."""
    return Response(
        content={"message": "Cookie set with SameSite=None"},
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_cookies_response_cookie_with_samesite_none() -> Spikard:
    """App factory for fixture: Response cookie with SameSite=None"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/cookies/samesite-none",
        body_schema={
            "additionalProperties": False,
            "properties": {"value": {"type": "string"}},
            "required": ["value"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(cookies_response_cookie_with_samesite_none)
    return app


def cookies_cookie_regex_pattern_validation_success(
    tracking_id: str | None = None,
) -> Any:
    """Handler for GET /cookies/pattern."""
    return Response(content={"tracking_id": "ABC12345"}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_cookies_cookie_regex_pattern_validation_success() -> Spikard:
    """App factory for fixture: Cookie regex pattern validation - success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/cookies/pattern",
        body_schema=None,
        parameter_schema={
            "properties": {"tracking_id": {"source": "cookie", "type": "string"}},
            "required": [],
            "type": "object",
        },
        file_params=None,
    )(cookies_cookie_regex_pattern_validation_success)
    return app


def cookies_response_set_cookie_basic() -> Any:
    """Handler for POST /cookie/."""
    return Response(
        content={"message": "Come to the dark side, we have cookies"},
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_cookies_response_set_cookie_basic() -> Spikard:
    """App factory for fixture: Response set cookie - basic"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route("POST", "/cookie/", body_schema=None, parameter_schema=None, file_params=None)(
        cookies_response_set_cookie_basic
    )
    return app


def cookies_multiple_cookies_success(
    fatebook_tracker: str | None = None,
    googall_tracker: str | None = None,
    session_id: str | None = None,
) -> Any:
    """Handler for GET /items/."""
    return Response(
        content={"fatebook_tracker": "tracker456", "googall_tracker": "ga789", "session_id": "session123"},
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_cookies_multiple_cookies_success() -> Spikard:
    """App factory for fixture: Multiple cookies - success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/items/",
        body_schema=None,
        parameter_schema={
            "properties": {
                "fatebook_tracker": {"source": "cookie", "type": "string"},
                "googall_tracker": {"source": "cookie", "type": "string"},
                "session_id": {"source": "cookie", "type": "string"},
            },
            "required": [],
            "type": "object",
        },
        file_params=None,
    )(cookies_multiple_cookies_success)
    return app


def cookies_26_cookie_secure_flag(
    auth_token: str,
) -> Any:
    """Handler for GET /secure."""
    # Echo back parameters for testing
    result: dict[str, Any] = {}
    if auth_token is not None:
        result["auth_token"] = auth_token
    return result


def create_app_cookies_26_cookie_secure_flag() -> Spikard:
    """App factory for fixture: 26_cookie_secure_flag"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/secure",
        body_schema=None,
        parameter_schema={
            "properties": {"auth_token": {"secure": True, "source": "cookie", "type": "string"}},
            "required": ["auth_token"],
            "type": "object",
        },
        file_params=None,
    )(cookies_26_cookie_secure_flag)
    return app


class CookiesResponseCookieWithDomainAttributeBody(TypedDict):
    """Request body type (TypedDict - runtime is dict)."""

    value: str


def cookies_response_cookie_with_domain_attribute(
    body: CookiesResponseCookieWithDomainAttributeBody,
) -> Any:
    """Handler for POST /cookies/set-with-domain."""
    return Response(
        content={"message": "Cookie set with domain"}, status_code=200, headers={"Content-Type": "application/json"}
    )


def create_app_cookies_response_cookie_with_domain_attribute() -> Spikard:
    """App factory for fixture: Response cookie with domain attribute"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/cookies/set-with-domain",
        body_schema={
            "additionalProperties": False,
            "properties": {"value": {"type": "string"}},
            "required": ["value"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(cookies_response_cookie_with_domain_attribute)
    return app


def edge_cases_19_emoji_in_strings(
    body: dict[str, Any],
) -> Any:
    """Handler for POST /messages."""
    return Response(
        content={"text": "Hello 👋 World 🌍"}, status_code=201, headers={"Content-Type": "application/json"}
    )


def create_app_edge_cases_19_emoji_in_strings() -> Spikard:
    """App factory for fixture: 19_emoji_in_strings"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/messages",
        body_schema={
            "properties": {"text": {"maxLength": 100, "minLength": 1, "type": "string"}},
            "required": ["text"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(edge_cases_19_emoji_in_strings)
    return app


def edge_cases_12_percent_encoded_special_chars(
    term: str,
) -> Any:
    """Handler for GET /search."""
    return Response(content={"term": "hi there"}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_edge_cases_12_percent_encoded_special_chars() -> Spikard:
    """App factory for fixture: 12_percent_encoded_special_chars"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/search",
        body_schema=None,
        parameter_schema={
            "properties": {"term": {"source": "query", "type": "string"}},
            "required": ["term"],
            "type": "object",
        },
        file_params=None,
    )(edge_cases_12_percent_encoded_special_chars)
    return app


@dataclass
class EdgeCasesSpecialStringValuesAndEscapingBody:
    """Request body dataclass."""

    backslashes: str
    empty_string: str
    quotes: str
    special_chars: str
    tabs_newlines: str
    unicode_escapes: str
    whitespace: str


def edge_cases_special_string_values_and_escaping(
    body: EdgeCasesSpecialStringValuesAndEscapingBody,
) -> Any:
    """Handler for POST /strings/."""
    return Response(
        content={
            "backslashes": "C:\\\\Users\\\\Path",
            "empty_string": "",
            "quotes": "He said \"hello\" and 'goodbye'",
            "special_chars": "!@#$%^&*()_+-=[]{}|;':\",./<>?",
            "tabs_newlines": "line1\n\tline2\r\nline3",
            "unicode_escapes": "Hello",
            "whitespace": "   ",
        },
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_edge_cases_special_string_values_and_escaping() -> Spikard:
    """App factory for fixture: Special string values and escaping"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
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
        parameter_schema=None,
        file_params=None,
    )(edge_cases_special_string_values_and_escaping)
    return app


class EdgeCases15FloatPrecisionPreservationBody(NamedTuple):
    """Request body NamedTuple (immutable)."""

    value: float


def edge_cases_15_float_precision_preservation(
    body: EdgeCases15FloatPrecisionPreservationBody,
) -> Any:
    """Handler for POST /calculate."""
    return Response(content={"value": 3.141592653589793}, status_code=201, headers={"Content-Type": "application/json"})


def create_app_edge_cases_15_float_precision_preservation() -> Spikard:
    """App factory for fixture: 15_float_precision_preservation"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/calculate",
        body_schema={"properties": {"value": {"type": "number"}}, "required": ["value"], "type": "object"},
        parameter_schema=None,
        file_params=None,
    )(edge_cases_15_float_precision_preservation)
    return app


def edge_cases_13_empty_string_query_param_preserved(
    filter: str,
) -> Any:
    """Handler for GET /items."""
    return Response(content={"filter": ""}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_edge_cases_13_empty_string_query_param_preserved() -> Spikard:
    """App factory for fixture: 13_empty_string_query_param_preserved"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/items",
        body_schema=None,
        parameter_schema={
            "properties": {"filter": {"source": "query", "type": "string"}},
            "required": ["filter"],
            "type": "object",
        },
        file_params=None,
    )(edge_cases_13_empty_string_query_param_preserved)
    return app


class EdgeCases24ArrayWithHolesBody(BaseModel):
    """Request body Pydantic model."""

    items: list[str]


def edge_cases_24_array_with_holes(
    body: EdgeCases24ArrayWithHolesBody,
) -> Any:
    """Handler for POST /items."""
    return Response(
        content={"items": ["first", "third", "sixth"]}, status_code=200, headers={"Content-Type": "application/json"}
    )


def create_app_edge_cases_24_array_with_holes() -> Spikard:
    """App factory for fixture: 24_array_with_holes"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/items",
        body_schema={
            "properties": {"items": {"items": {"type": "string"}, "type": "array"}},
            "required": ["items"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(edge_cases_24_array_with_holes)
    return app


def edge_cases_21_scientific_notation_number(
    body: dict[str, Any],
) -> Any:
    """Handler for POST /calculate."""
    return Response(content={"value": 123000}, status_code=201, headers={"Content-Type": "application/json"})


def create_app_edge_cases_21_scientific_notation_number() -> Spikard:
    """App factory for fixture: 21_scientific_notation_number"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/calculate",
        body_schema={
            "properties": {"value": {"minimum": 0, "type": "number"}},
            "required": ["value"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(edge_cases_21_scientific_notation_number)
    return app


class EdgeCasesFloatPrecisionAndRoundingBody(TypedDict):
    """Request body type (TypedDict - runtime is dict)."""

    expected_sum: float
    precise_value: float
    value1: float
    value2: float
    very_large: float
    very_small: float


def edge_cases_float_precision_and_rounding(
    body: EdgeCasesFloatPrecisionAndRoundingBody,
) -> Any:
    """Handler for POST /calculations/."""
    return Response(
        content={
            "precise_value": 3.141592653589793,
            "sum": 0.30000000000000004,
            "very_large": 1.7976931348623157e308,
            "very_small": 1e-10,
        },
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_edge_cases_float_precision_and_rounding() -> Spikard:
    """App factory for fixture: Float precision and rounding"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
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
        parameter_schema=None,
        file_params=None,
    )(edge_cases_float_precision_and_rounding)
    return app


@dataclass
class EdgeCasesUnicodeAndEmojiHandlingBody:
    """Request body dataclass."""

    description: str
    emoji_reactions: str
    name: str
    tags: list[str]


def edge_cases_unicode_and_emoji_handling(
    body: EdgeCasesUnicodeAndEmojiHandlingBody,
) -> Any:
    """Handler for POST /items/."""
    return Response(
        content={
            "description": "Best café in München 🇩🇪",
            "emoji_reactions": "👍❤️😂🎉",
            "id": 1,
            "name": "Coffee Shop ☕",
            "tags": ["食べ物", "音楽", "💰"],
        },
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_edge_cases_unicode_and_emoji_handling() -> Spikard:
    """App factory for fixture: Unicode and emoji handling"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/items/",
        body_schema={
            "additionalProperties": False,
            "properties": {
                "description": {"type": "string"},
                "emoji_reactions": {"type": "string"},
                "name": {"type": "string"},
                "tags": {"items": {"type": "string"}, "type": "array"},
            },
            "required": ["name", "description", "tags", "emoji_reactions"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(edge_cases_unicode_and_emoji_handling)
    return app


class EdgeCases17ExtremelyLongStringBody(NamedTuple):
    """Request body NamedTuple (immutable)."""

    content: str


def edge_cases_17_extremely_long_string(
    body: EdgeCases17ExtremelyLongStringBody,
) -> Any:
    """Handler for POST /text."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"actual_length": 10001, "max_length": 10000},
                    "loc": ["body", "content"],
                    "msg": "String length must not exceed 10000",
                    "type": "validation_error",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_edge_cases_17_extremely_long_string() -> Spikard:
    """App factory for fixture: 17_extremely_long_string"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/text",
        body_schema={
            "properties": {"content": {"maxLength": 10000, "type": "string"}},
            "required": ["content"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(edge_cases_17_extremely_long_string)
    return app


def edge_cases_11_utf8_query_parameter(
    term: str,
) -> Any:
    """Handler for GET /search."""
    return Response(content={"term": "café"}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_edge_cases_11_utf8_query_parameter() -> Spikard:
    """App factory for fixture: 11_utf8_query_parameter"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/search",
        body_schema=None,
        parameter_schema={
            "properties": {"term": {"source": "query", "type": "string"}},
            "required": ["term"],
            "type": "object",
        },
        file_params=None,
    )(edge_cases_11_utf8_query_parameter)
    return app


class EdgeCases18UnicodeNormalizationBody(BaseModel):
    """Request body Pydantic model."""

    name: str


def edge_cases_18_unicode_normalization(
    body: EdgeCases18UnicodeNormalizationBody,
) -> Any:
    """Handler for POST /users."""
    return Response(content={"name": "café"}, status_code=201, headers={"Content-Type": "application/json"})


def create_app_edge_cases_18_unicode_normalization() -> Spikard:
    """App factory for fixture: 18_unicode_normalization"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/users",
        body_schema={
            "properties": {"name": {"minLength": 1, "type": "string"}},
            "required": ["name"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(edge_cases_18_unicode_normalization)
    return app


def edge_cases_20_null_byte_in_string(
    body: dict[str, Any],
) -> Any:
    """Handler for POST /files."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"value": "file\\u0000.txt"},
                    "loc": ["body", "filename"],
                    "msg": "String contains null byte character",
                    "type": "validation_error",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_edge_cases_20_null_byte_in_string() -> Spikard:
    """App factory for fixture: 20_null_byte_in_string"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/files",
        body_schema={
            "properties": {"filename": {"pattern": "^[^\\x00]+$", "type": "string"}},
            "required": ["filename"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(edge_cases_20_null_byte_in_string)
    return app


class EdgeCases23DeeplyNestedJsonLimitBody(TypedDict):
    """Request body type (TypedDict - runtime is dict)."""


def edge_cases_23_deeply_nested_json_limit(
    body: EdgeCases23DeeplyNestedJsonLimitBody,
) -> Any:
    """Handler for POST /data."""
    return Response(
        content={"error": "Request body exceeds maximum nesting depth of 32"},
        status_code=400,
        headers={"Content-Type": "application/json"},
    )


def create_app_edge_cases_23_deeply_nested_json_limit() -> Spikard:
    """App factory for fixture: 23_deeply_nested_json_limit"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route("POST", "/data", body_schema={"type": "object"}, parameter_schema=None, file_params=None)(
        edge_cases_23_deeply_nested_json_limit
    )
    return app


def edge_cases_14_large_integer_boundary(
    id: int,
) -> Any:
    """Handler for GET /items."""
    return Response(content={"id": 9007199254740991}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_edge_cases_14_large_integer_boundary() -> Spikard:
    """App factory for fixture: 14_large_integer_boundary"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/items",
        body_schema=None,
        parameter_schema={
            "properties": {"id": {"source": "query", "type": "integer"}},
            "required": ["id"],
            "type": "object",
        },
        file_params=None,
    )(edge_cases_14_large_integer_boundary)
    return app


def edge_cases_22_leading_zeros_integer(
    value: int,
) -> Any:
    """Handler for GET /data."""
    return Response(content={"value": 123}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_edge_cases_22_leading_zeros_integer() -> Spikard:
    """App factory for fixture: 22_leading_zeros_integer"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/data",
        body_schema=None,
        parameter_schema={
            "properties": {"value": {"annotation": "int", "source": "query", "type": "integer"}},
            "required": ["value"],
            "type": "object",
        },
        file_params=None,
    )(edge_cases_22_leading_zeros_integer)
    return app


class EdgeCasesLargeIntegerBoundaryValuesBody(msgspec.Struct):
    """Request body msgspec.Struct (fast typed)."""

    large_int: int
    max_safe_int: int
    negative_large: int


def edge_cases_large_integer_boundary_values(
    body: EdgeCasesLargeIntegerBoundaryValuesBody,
) -> Any:
    """Handler for POST /numbers/."""
    return Response(
        content={
            "large_int": 9223372036854775807,
            "max_safe_int": 9007199254740991,
            "negative_large": -9223372036854775808,
        },
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_edge_cases_large_integer_boundary_values() -> Spikard:
    """App factory for fixture: Large integer boundary values"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
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
        parameter_schema=None,
        file_params=None,
    )(edge_cases_large_integer_boundary_values)
    return app


class EdgeCasesDeeplyNestedStructure10LevelsBody(BaseModel):
    """Request body Pydantic model."""

    level1: dict[str, Any]


def edge_cases_deeply_nested_structure_10_levels(
    body: EdgeCasesDeeplyNestedStructure10LevelsBody,
) -> Any:
    """Handler for POST /nested/."""
    return Response(
        content={"max_depth": 10, "message": "Processed deeply nested structure", "value_found": "deep"},
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_edge_cases_deeply_nested_structure_10_levels() -> Spikard:
    """App factory for fixture: Deeply nested structure (10+ levels)"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
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
                                                                                                "value": {
                                                                                                    "type": "string"
                                                                                                },
                                                                                            },
                                                                                            "required": [
                                                                                                "value",
                                                                                                "depth",
                                                                                            ],
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
        parameter_schema=None,
        file_params=None,
    )(edge_cases_deeply_nested_structure_10_levels)
    return app


def edge_cases_empty_and_null_value_handling(
    body: dict[str, Any],
) -> Any:
    """Handler for POST /nulls/."""
    return Response(
        content={
            "empty_array_length": 0,
            "empty_object_keys": 0,
            "empty_string_length": 0,
            "explicit_null_is_null": True,
            "false_is_false": True,
            "zero_is_falsy": True,
        },
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_edge_cases_empty_and_null_value_handling() -> Spikard:
    """App factory for fixture: Empty and null value handling"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
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
            "required": [
                "explicit_null",
                "empty_string",
                "empty_array",
                "empty_object",
                "zero_number",
                "false_boolean",
            ],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(edge_cases_empty_and_null_value_handling)
    return app


class EdgeCases16NegativeZeroHandlingBody(TypedDict):
    """Request body type (TypedDict - runtime is dict)."""

    offset: float


def edge_cases_16_negative_zero_handling(
    body: EdgeCases16NegativeZeroHandlingBody,
) -> Any:
    """Handler for POST /data."""
    return Response(content={"offset": 0}, status_code=201, headers={"Content-Type": "application/json"})


def create_app_edge_cases_16_negative_zero_handling() -> Spikard:
    """App factory for fixture: 16_negative_zero_handling"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/data",
        body_schema={"properties": {"offset": {"type": "number"}}, "required": ["offset"], "type": "object"},
        parameter_schema=None,
        file_params=None,
    )(edge_cases_16_negative_zero_handling)
    return app


async def streaming_stream_json_lines() -> Any:
    """Handler for GET /stream/json-lines."""

    async def stream_chunks():
        yield '{"index":0,"payload":"alpha"}\\n'
        yield '{"index":1,"payload":"beta"}\\n'
        yield '{"index":2,"payload":"gamma"}\\n'

    return StreamingResponse(stream_chunks(), status_code=200, headers={"content-type": "application/x-ndjson"})


def create_app_streaming_stream_json_lines() -> Spikard:
    """App factory for fixture: Stream JSON lines"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route("GET", "/stream/json-lines", body_schema=None, parameter_schema=None, file_params=None)(
        streaming_stream_json_lines
    )
    return app


async def streaming_binary_log_download() -> Any:
    """Handler for GET /stream/logfile."""

    async def stream_chunks():
        yield "LOG:"
        yield b"\x00\x01\x02\x03"
        yield "|TAIL|"
        yield b"\x07"
        yield "\\n"

    return StreamingResponse(stream_chunks(), status_code=200, headers={"content-type": "application/octet-stream"})


def create_app_streaming_binary_log_download() -> Spikard:
    """App factory for fixture: Binary log download"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route("GET", "/stream/logfile", body_schema=None, parameter_schema=None, file_params=None)(
        streaming_binary_log_download
    )
    return app


async def streaming_chunked_csv_export() -> Any:
    """Handler for GET /stream/csv-report."""

    async def stream_chunks():
        yield "id,name,value\\n"
        yield "1,Alice,42\\n"
        yield "2,Bob,7\\n"

    return StreamingResponse(stream_chunks(), status_code=200, headers={"content-type": "text/csv"})


def create_app_streaming_chunked_csv_export() -> Spikard:
    """App factory for fixture: Chunked CSV export"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route("GET", "/stream/csv-report", body_schema=None, parameter_schema=None, file_params=None)(
        streaming_chunked_csv_export
    )
    return app


def create_app_static_files_static_file_server_returns_text_file() -> Spikard:
    """App factory for fixture: Static file server returns text file"""
    config = ServerConfig(
        static_files=[
            StaticFilesConfig(
                directory=str(
                    BASE_DIR / "static_assets" / "static_files_static_file_server_returns_text_file" / "public_0"
                ),
                route_prefix="/public",
                cache_control="public, max-age=60",
            )
        ]
    )
    app = Spikard(config=config)
    return app


def create_app_static_files_static_server_returns_index_html_for_directory() -> Spikard:
    """App factory for fixture: Static server returns index.html for directory"""
    config = ServerConfig(
        static_files=[
            StaticFilesConfig(
                directory=str(
                    BASE_DIR / "static_assets" / "static_files_static_server_returns_index_html_for_directory" / "app_0"
                ),
                route_prefix="/app",
            )
        ]
    )
    app = Spikard(config=config)
    return app


def query_params_string_validation_with_regex_success(
    item_query: str,
) -> Any:
    """Handler for GET /items/."""
    return Response(content={"item_query": "fixedquery"}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_query_params_string_validation_with_regex_success() -> Spikard:
    """App factory for fixture: String validation with regex - success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/items/",
        body_schema=None,
        parameter_schema={
            "properties": {
                "item_query": {"annotation": "str", "pattern": "^fixedquery$", "source": "query", "type": "string"}
            },
            "required": ["item_query"],
            "type": "object",
        },
        file_params=None,
    )(query_params_string_validation_with_regex_success)
    return app


def query_params_49_integer_gt_constraint_success(
    limit: int,
) -> Any:
    """Handler for GET /items."""
    return Response(content={"limit": 5}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_query_params_49_integer_gt_constraint_success() -> Spikard:
    """App factory for fixture: 49_integer_gt_constraint_success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/items",
        body_schema=None,
        parameter_schema={
            "properties": {"limit": {"exclusiveMinimum": 0, "source": "query", "type": "integer"}},
            "required": ["limit"],
            "type": "object",
        },
        file_params=None,
    )(query_params_49_integer_gt_constraint_success)
    return app


def query_params_enum_query_parameter_invalid_value(
    model: str,
) -> Any:
    """Handler for GET /query/enum."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"expected": "'alexnet', 'resnet' or 'lenet'"},
                    "input": "vgg16",
                    "loc": ["query", "model"],
                    "msg": "Input should be 'alexnet', 'resnet' or 'lenet'",
                    "type": "enum",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_query_params_enum_query_parameter_invalid_value() -> Spikard:
    """App factory for fixture: Enum query parameter - invalid value"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/query/enum",
        body_schema=None,
        parameter_schema={
            "properties": {
                "model": {
                    "annotation": "str",
                    "enum": ["alexnet", "resnet", "lenet"],
                    "source": "query",
                    "type": "string",
                }
            },
            "required": ["model"],
            "type": "object",
        },
        file_params=None,
    )(query_params_enum_query_parameter_invalid_value)
    return app


def query_params_68_array_uniqueitems_success(
    ids: list[int],
) -> Any:
    """Handler for GET /items."""
    return Response(content={"ids": [1, 2, 3, 4]}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_query_params_68_array_uniqueitems_success() -> Spikard:
    """App factory for fixture: 68_array_uniqueitems_success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/items",
        body_schema=None,
        parameter_schema={
            "properties": {
                "ids": {"items": {"type": "integer"}, "source": "query", "type": "array", "uniqueItems": True}
            },
            "required": ["ids"],
            "type": "object",
        },
        file_params=None,
    )(query_params_68_array_uniqueitems_success)
    return app


def query_params_47_pattern_validation_email_success(
    email: str,
) -> Any:
    """Handler for GET /subscribe."""
    return Response(
        content={"email": "user@example.com"}, status_code=200, headers={"Content-Type": "application/json"}
    )


def create_app_query_params_47_pattern_validation_email_success() -> Spikard:
    """App factory for fixture: 47_pattern_validation_email_success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/subscribe",
        body_schema=None,
        parameter_schema={
            "properties": {
                "email": {
                    "pattern": "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$",
                    "source": "query",
                    "type": "string",
                }
            },
            "required": ["email"],
            "type": "object",
        },
        file_params=None,
    )(query_params_47_pattern_validation_email_success)
    return app


def query_params_required_integer_query_parameter_success(
    query: int,
) -> Any:
    """Handler for GET /query/int."""
    return Response(content="foo bar 42", status_code=200)


def create_app_query_params_required_integer_query_parameter_success() -> Spikard:
    """App factory for fixture: Required integer query parameter - success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/query/int",
        body_schema=None,
        parameter_schema={
            "properties": {"query": {"annotation": "int", "source": "query", "type": "integer"}},
            "required": ["query"],
            "type": "object",
        },
        file_params=None,
    )(query_params_required_integer_query_parameter_success)
    return app


def query_params_required_string_query_parameter_missing(
    query: str,
) -> Any:
    """Handler for GET /query."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [{"input": None, "loc": ["query", "query"], "msg": "Field required", "type": "missing"}],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_query_params_required_string_query_parameter_missing() -> Spikard:
    """App factory for fixture: Required string query parameter - missing"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/query",
        body_schema=None,
        parameter_schema={
            "properties": {"query": {"annotation": "str", "source": "query", "type": "string"}},
            "required": ["query"],
            "type": "object",
        },
        file_params=None,
    )(query_params_required_string_query_parameter_missing)
    return app


def query_params_57_boolean_empty_string_coercion(
    active: bool,
) -> Any:
    """Handler for GET /items."""
    return Response(content={"active": False}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_query_params_57_boolean_empty_string_coercion() -> Spikard:
    """App factory for fixture: 57_boolean_empty_string_coercion"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/items",
        body_schema=None,
        parameter_schema={
            "properties": {"active": {"source": "query", "type": "boolean"}},
            "required": ["active"],
            "type": "object",
        },
        file_params=None,
    )(query_params_57_boolean_empty_string_coercion)
    return app


def query_params_52_integer_le_constraint_boundary(
    limit: int,
) -> Any:
    """Handler for GET /items."""
    return Response(content={"limit": 100}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_query_params_52_integer_le_constraint_boundary() -> Spikard:
    """App factory for fixture: 52_integer_le_constraint_boundary"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/items",
        body_schema=None,
        parameter_schema={
            "properties": {"limit": {"maximum": 100, "source": "query", "type": "integer"}},
            "required": ["limit"],
            "type": "object",
        },
        file_params=None,
    )(query_params_52_integer_le_constraint_boundary)
    return app


def query_params_list_with_default_empty_array_no_values_provided(
    tags: list[str] | None = None,
) -> Any:
    """Handler for GET /query/list-default."""
    # Echo back parameters for testing
    result: dict[str, Any] = {}
    if tags is not None:
        result["tags"] = tags
    return result


def create_app_query_params_list_with_default_empty_array_no_values_provided() -> Spikard:
    """App factory for fixture: List with default empty array - no values provided"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/query/list-default",
        body_schema=None,
        parameter_schema={
            "properties": {
                "tags": {
                    "annotation": "list[str]",
                    "default": [],
                    "items": {"type": "string"},
                    "source": "query",
                    "type": "array",
                }
            },
            "required": [],
            "type": "object",
        },
        file_params=None,
    )(query_params_list_with_default_empty_array_no_values_provided)
    return app


def query_params_date_query_parameter_success(
    event_date: date,
) -> Any:
    """Handler for GET /query/date."""
    return Response(content={"event_date": "2024-01-15"}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_query_params_date_query_parameter_success() -> Spikard:
    """App factory for fixture: Date query parameter - success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/query/date",
        body_schema=None,
        parameter_schema={
            "properties": {"event_date": {"annotation": "str", "format": "date", "source": "query", "type": "string"}},
            "required": ["event_date"],
            "type": "object",
        },
        file_params=None,
    )(query_params_date_query_parameter_success)
    return app


def query_params_string_query_param_with_max_length_constraint_fail(
    name: str,
) -> Any:
    """Handler for GET /query/str-max-length."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"max_length": 10},
                    "input": "this_is_way_too_long",
                    "loc": ["query", "name"],
                    "msg": "String should have at most 10 characters",
                    "type": "string_too_long",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_query_params_string_query_param_with_max_length_constraint_fail() -> Spikard:
    """App factory for fixture: String query param with max_length constraint - fail"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/query/str-max-length",
        body_schema=None,
        parameter_schema={
            "properties": {"name": {"annotation": "str", "maxLength": 10, "source": "query", "type": "string"}},
            "required": ["name"],
            "type": "object",
        },
        file_params=None,
    )(query_params_string_query_param_with_max_length_constraint_fail)
    return app


def query_params_45_string_minlength_validation_failure(
    term: str,
) -> Any:
    """Handler for GET /search."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"actual_length": 2, "min_length": 3},
                    "loc": ["query", "term"],
                    "msg": "String length must be at least 3",
                    "type": "validation_error",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_query_params_45_string_minlength_validation_failure() -> Spikard:
    """App factory for fixture: 45_string_minlength_validation_failure"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/search",
        body_schema=None,
        parameter_schema={
            "properties": {"term": {"minLength": 3, "source": "query", "type": "string"}},
            "required": ["term"],
            "type": "object",
        },
        file_params=None,
    )(query_params_45_string_minlength_validation_failure)
    return app


def query_params_integer_with_default_value_override(
    query: int | None = None,
) -> Any:
    """Handler for GET /query/int/default."""
    return Response(content="foo bar 50", status_code=200)


def create_app_query_params_integer_with_default_value_override() -> Spikard:
    """App factory for fixture: Integer with default value - override"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/query/int/default",
        body_schema=None,
        parameter_schema={
            "properties": {"query": {"annotation": "int", "default": 10, "source": "query", "type": "integer"}},
            "required": [],
            "type": "object",
        },
        file_params=None,
    )(query_params_integer_with_default_value_override)
    return app


def query_params_67_multipleof_constraint_failure(
    quantity: int,
) -> Any:
    """Handler for GET /items."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"multiple_of": 5, "value": 17},
                    "loc": ["query", "quantity"],
                    "msg": "Value must be a multiple of 5",
                    "type": "validation_error",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_query_params_67_multipleof_constraint_failure() -> Spikard:
    """App factory for fixture: 67_multipleof_constraint_failure"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/items",
        body_schema=None,
        parameter_schema={
            "properties": {"quantity": {"multipleOf": 5, "source": "query", "type": "integer"}},
            "required": ["quantity"],
            "type": "object",
        },
        file_params=None,
    )(query_params_67_multipleof_constraint_failure)
    return app


def query_params_58_format_email_success(
    email: str,
) -> Any:
    """Handler for GET /subscribe."""
    return Response(
        content={"email": "user@example.com"}, status_code=200, headers={"Content-Type": "application/json"}
    )


def create_app_query_params_58_format_email_success() -> Spikard:
    """App factory for fixture: 58_format_email_success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/subscribe",
        body_schema=None,
        parameter_schema={
            "properties": {"email": {"format": "email", "source": "query", "type": "string"}},
            "required": ["email"],
            "type": "object",
        },
        file_params=None,
    )(query_params_58_format_email_success)
    return app


def query_params_integer_query_param_with_ge_constraint_boundary(
    value: int,
) -> Any:
    """Handler for GET /query/int-ge."""
    return Response(content={"value": 10}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_query_params_integer_query_param_with_ge_constraint_boundary() -> Spikard:
    """App factory for fixture: Integer query param with ge constraint - boundary"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/query/int-ge",
        body_schema=None,
        parameter_schema={
            "properties": {"value": {"annotation": "int", "minimum": 10, "source": "query", "type": "integer"}},
            "required": ["value"],
            "type": "object",
        },
        file_params=None,
    )(query_params_integer_query_param_with_ge_constraint_boundary)
    return app


def query_params_integer_query_param_with_gt_constraint_valid(
    value: int,
) -> Any:
    """Handler for GET /query/int-gt."""
    return Response(content={"value": 1}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_query_params_integer_query_param_with_gt_constraint_valid() -> Spikard:
    """App factory for fixture: Integer query param with gt constraint - valid"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/query/int-gt",
        body_schema=None,
        parameter_schema={
            "properties": {"value": {"annotation": "int", "exclusiveMinimum": 0, "source": "query", "type": "integer"}},
            "required": ["value"],
            "type": "object",
        },
        file_params=None,
    )(query_params_integer_query_param_with_gt_constraint_valid)
    return app


def query_params_required_integer_query_parameter_invalid_type(
    query: int,
) -> Any:
    """Handler for GET /query/int."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "input": "baz",
                    "loc": ["query", "query"],
                    "msg": "Input should be a valid integer, unable to parse string as an integer",
                    "type": "int_parsing",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_query_params_required_integer_query_parameter_invalid_type() -> Spikard:
    """App factory for fixture: Required integer query parameter - invalid type"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/query/int",
        body_schema=None,
        parameter_schema={
            "properties": {"query": {"annotation": "int", "source": "query", "type": "integer"}},
            "required": ["query"],
            "type": "object",
        },
        file_params=None,
    )(query_params_required_integer_query_parameter_invalid_type)
    return app


def query_params_required_integer_query_parameter_float_value(
    query: int,
) -> Any:
    """Handler for GET /query/int."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "input": 42.5,
                    "loc": ["query", "query"],
                    "msg": "Input should be a valid integer, unable to parse string as an integer",
                    "type": "int_parsing",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_query_params_required_integer_query_parameter_float_value() -> Spikard:
    """App factory for fixture: Required integer query parameter - float value"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/query/int",
        body_schema=None,
        parameter_schema={
            "properties": {"query": {"annotation": "int", "source": "query", "type": "integer"}},
            "required": ["query"],
            "type": "object",
        },
        file_params=None,
    )(query_params_required_integer_query_parameter_float_value)
    return app


def query_params_query_parameter_with_url_encoded_special_characters(
    name: str,
) -> Any:
    """Handler for GET /query/basic."""
    return Response(content={"name": "test&value=123"}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_query_params_query_parameter_with_url_encoded_special_characters() -> Spikard:
    """App factory for fixture: Query parameter with URL encoded special characters"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/query/basic",
        body_schema=None,
        parameter_schema={
            "properties": {"name": {"annotation": "str", "source": "query", "type": "string"}},
            "required": ["name"],
            "type": "object",
        },
        file_params=None,
    )(query_params_query_parameter_with_url_encoded_special_characters)
    return app


def query_params_59_format_email_failure(
    email: str,
) -> Any:
    """Handler for GET /subscribe."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"format": "email", "value": "not-an-email"},
                    "loc": ["query", "email"],
                    "msg": "Invalid email format",
                    "type": "validation_error",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_query_params_59_format_email_failure() -> Spikard:
    """App factory for fixture: 59_format_email_failure"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/subscribe",
        body_schema=None,
        parameter_schema={
            "properties": {"email": {"format": "email", "source": "query", "type": "string"}},
            "required": ["email"],
            "type": "object",
        },
        file_params=None,
    )(query_params_59_format_email_failure)
    return app


def query_params_43_scientific_notation_float(
    threshold: float,
) -> Any:
    """Handler for GET /stats."""
    return Response(content={"threshold": 0.0015}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_query_params_43_scientific_notation_float() -> Spikard:
    """App factory for fixture: 43_scientific_notation_float"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/stats",
        body_schema=None,
        parameter_schema={
            "properties": {"threshold": {"annotation": "float", "source": "query", "type": "number"}},
            "required": ["threshold"],
            "type": "object",
        },
        file_params=None,
    )(query_params_43_scientific_notation_float)
    return app


def query_params_63_format_uri_success(
    url: str,
) -> Any:
    """Handler for GET /redirect."""
    return Response(
        content={"url": "https://example.com/path?query=value"},
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_query_params_63_format_uri_success() -> Spikard:
    """App factory for fixture: 63_format_uri_success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/redirect",
        body_schema=None,
        parameter_schema={
            "properties": {"url": {"format": "uri", "source": "query", "type": "string"}},
            "required": ["url"],
            "type": "object",
        },
        file_params=None,
    )(query_params_63_format_uri_success)
    return app


def query_params_boolean_query_parameter_numeric_1(
    flag: bool,
) -> Any:
    """Handler for GET /query/bool."""
    return Response(content={"flag": True}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_query_params_boolean_query_parameter_numeric_1() -> Spikard:
    """App factory for fixture: Boolean query parameter - numeric 1"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/query/bool",
        body_schema=None,
        parameter_schema={
            "properties": {"flag": {"annotation": "bool", "source": "query", "type": "boolean"}},
            "required": ["flag"],
            "type": "object",
        },
        file_params=None,
    )(query_params_boolean_query_parameter_numeric_1)
    return app


def query_params_string_query_param_with_min_length_constraint_fail(
    name: str,
) -> Any:
    """Handler for GET /query/str-min-length."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"min_length": 3},
                    "input": "ab",
                    "loc": ["query", "name"],
                    "msg": "String should have at least 3 characters",
                    "type": "string_too_short",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_query_params_string_query_param_with_min_length_constraint_fail() -> Spikard:
    """App factory for fixture: String query param with min_length constraint - fail"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/query/str-min-length",
        body_schema=None,
        parameter_schema={
            "properties": {"name": {"annotation": "str", "minLength": 3, "source": "query", "type": "string"}},
            "required": ["name"],
            "type": "object",
        },
        file_params=None,
    )(query_params_string_query_param_with_min_length_constraint_fail)
    return app


def query_params_optional_string_query_parameter_provided(
    query: str | None = None,
) -> Any:
    """Handler for GET /query/optional."""
    return Response(content="foo bar baz", status_code=200)


def create_app_query_params_optional_string_query_parameter_provided() -> Spikard:
    """App factory for fixture: Optional string query parameter - provided"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/query/optional",
        body_schema=None,
        parameter_schema={
            "properties": {"query": {"annotation": "str", "source": "query", "type": "string"}},
            "required": [],
            "type": "object",
        },
        file_params=None,
    )(query_params_optional_string_query_parameter_provided)
    return app


def query_params_list_of_integers_multiple_values(
    device_ids: list[int],
) -> Any:
    """Handler for GET /query/list."""
    return Response(content=[1, 2], status_code=200, headers={"Content-Type": "application/json"})


def create_app_query_params_list_of_integers_multiple_values() -> Spikard:
    """App factory for fixture: List of integers - multiple values"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/query/list",
        body_schema=None,
        parameter_schema={
            "properties": {
                "device_ids": {
                    "annotation": "list[int]",
                    "items": {"type": "integer"},
                    "source": "query",
                    "type": "array",
                }
            },
            "required": ["device_ids"],
            "type": "object",
        },
        file_params=None,
    )(query_params_list_of_integers_multiple_values)
    return app


def query_params_integer_query_param_with_lt_constraint_valid(
    value: int,
) -> Any:
    """Handler for GET /query/int-lt."""
    return Response(content={"value": 49}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_query_params_integer_query_param_with_lt_constraint_valid() -> Spikard:
    """App factory for fixture: Integer query param with lt constraint - valid"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/query/int-lt",
        body_schema=None,
        parameter_schema={
            "properties": {
                "value": {"annotation": "int", "exclusiveMaximum": 50, "source": "query", "type": "integer"}
            },
            "required": ["value"],
            "type": "object",
        },
        file_params=None,
    )(query_params_integer_query_param_with_lt_constraint_valid)
    return app


def query_params_42_negative_integer_query_param(
    offset: int,
) -> Any:
    """Handler for GET /items/negative."""
    return Response(content={"offset": -10}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_query_params_42_negative_integer_query_param() -> Spikard:
    """App factory for fixture: 42_negative_integer_query_param"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/items/negative",
        body_schema=None,
        parameter_schema={
            "properties": {"offset": {"annotation": "int", "source": "query", "type": "integer"}},
            "required": ["offset"],
            "type": "object",
        },
        file_params=None,
    )(query_params_42_negative_integer_query_param)
    return app


def query_params_46_string_maxlength_validation_failure(
    term: str,
) -> Any:
    """Handler for GET /search."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"actual_length": 21, "max_length": 10},
                    "loc": ["query", "term"],
                    "msg": "String length must not exceed 10",
                    "type": "validation_error",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_query_params_46_string_maxlength_validation_failure() -> Spikard:
    """App factory for fixture: 46_string_maxlength_validation_failure"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/search",
        body_schema=None,
        parameter_schema={
            "properties": {"term": {"maxLength": 10, "source": "query", "type": "string"}},
            "required": ["term"],
            "type": "object",
        },
        file_params=None,
    )(query_params_46_string_maxlength_validation_failure)
    return app


def query_params_56_array_maxitems_constraint_failure(
    tags: list[str],
) -> Any:
    """Handler for GET /items."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"actual_items": 6, "max_items": 5},
                    "loc": ["query", "tags"],
                    "msg": "Array must not contain more than 5 items",
                    "type": "validation_error",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_query_params_56_array_maxitems_constraint_failure() -> Spikard:
    """App factory for fixture: 56_array_maxitems_constraint_failure"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/items",
        body_schema=None,
        parameter_schema={
            "properties": {"tags": {"items": {"type": "string"}, "maxItems": 5, "source": "query", "type": "array"}},
            "required": ["tags"],
            "type": "object",
        },
        file_params=None,
    )(query_params_56_array_maxitems_constraint_failure)
    return app


def query_params_string_query_param_with_regex_pattern_fail(
    code: str,
) -> Any:
    """Handler for GET /query/pattern."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"pattern": "^[0-9]{3,}$"},
                    "input": "abc123",
                    "loc": ["query", "code"],
                    "msg": "String should match pattern '^[0-9]{3,}$'",
                    "type": "string_pattern_mismatch",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_query_params_string_query_param_with_regex_pattern_fail() -> Spikard:
    """App factory for fixture: String query param with regex pattern - fail"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/query/pattern",
        body_schema=None,
        parameter_schema={
            "properties": {
                "code": {"annotation": "str", "pattern": "^[0-9]{3,}$", "source": "query", "type": "string"}
            },
            "required": ["code"],
            "type": "object",
        },
        file_params=None,
    )(query_params_string_query_param_with_regex_pattern_fail)
    return app


def query_params_44_string_minlength_validation_success(
    term: str,
) -> Any:
    """Handler for GET /search."""
    return Response(content={"term": "foo"}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_query_params_44_string_minlength_validation_success() -> Spikard:
    """App factory for fixture: 44_string_minlength_validation_success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/search",
        body_schema=None,
        parameter_schema={
            "properties": {"term": {"minLength": 3, "source": "query", "type": "string"}},
            "required": ["term"],
            "type": "object",
        },
        file_params=None,
    )(query_params_44_string_minlength_validation_success)
    return app


def query_params_61_format_ipv4_failure(
    ip: str,
) -> Any:
    """Handler for GET /network."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"format": "ipv4", "value": "999.999.999.999"},
                    "loc": ["query", "ip"],
                    "msg": "Invalid IPv4 address format",
                    "type": "validation_error",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_query_params_61_format_ipv4_failure() -> Spikard:
    """App factory for fixture: 61_format_ipv4_failure"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/network",
        body_schema=None,
        parameter_schema={
            "properties": {"ip": {"format": "ipv4", "source": "query", "type": "string"}},
            "required": ["ip"],
            "type": "object",
        },
        file_params=None,
    )(query_params_61_format_ipv4_failure)
    return app


def query_params_48_pattern_validation_email_failure(
    email: str,
) -> Any:
    """Handler for GET /subscribe."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"pattern": "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$", "value": "invalid-email"},
                    "loc": ["query", "email"],
                    "msg": "String does not match pattern",
                    "type": "validation_error",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_query_params_48_pattern_validation_email_failure() -> Spikard:
    """App factory for fixture: 48_pattern_validation_email_failure"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/subscribe",
        body_schema=None,
        parameter_schema={
            "properties": {
                "email": {
                    "pattern": "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$",
                    "source": "query",
                    "type": "string",
                }
            },
            "required": ["email"],
            "type": "object",
        },
        file_params=None,
    )(query_params_48_pattern_validation_email_failure)
    return app


def query_params_required_integer_query_parameter_missing(
    query: int,
) -> Any:
    """Handler for GET /query/int."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [{"input": None, "loc": ["query", "query"], "msg": "Field required", "type": "missing"}],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_query_params_required_integer_query_parameter_missing() -> Spikard:
    """App factory for fixture: Required integer query parameter - missing"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/query/int",
        body_schema=None,
        parameter_schema={
            "properties": {"query": {"annotation": "int", "source": "query", "type": "integer"}},
            "required": ["query"],
            "type": "object",
        },
        file_params=None,
    )(query_params_required_integer_query_parameter_missing)
    return app


def query_params_query_parameter_with_special_characters_url_encoding(
    email: str,
    special: str,
) -> Any:
    """Handler for GET /test."""
    return Response(
        content={"email": "x@test.com", "special": "&@A.ac"},
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_query_params_query_parameter_with_special_characters_url_encoding() -> Spikard:
    """App factory for fixture: Query parameter with special characters - URL encoding"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/test",
        body_schema=None,
        parameter_schema={
            "properties": {
                "email": {"annotation": "str", "source": "query", "type": "string"},
                "special": {"annotation": "str", "source": "query", "type": "string"},
            },
            "required": ["email", "special"],
            "type": "object",
        },
        file_params=None,
    )(query_params_query_parameter_with_special_characters_url_encoding)
    return app


def query_params_list_query_parameter_required_but_missing(
    device_ids: list[int],
) -> Any:
    """Handler for GET /query/list."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [{"input": None, "loc": ["query", "device_ids"], "msg": "Field required", "type": "missing"}],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_query_params_list_query_parameter_required_but_missing() -> Spikard:
    """App factory for fixture: List query parameter - required but missing"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/query/list",
        body_schema=None,
        parameter_schema={
            "properties": {
                "device_ids": {
                    "annotation": "list[int]",
                    "items": {"type": "integer"},
                    "source": "query",
                    "type": "array",
                }
            },
            "required": ["device_ids"],
            "type": "object",
        },
        file_params=None,
    )(query_params_list_query_parameter_required_but_missing)
    return app


def query_params_required_string_query_parameter_success(
    query: str,
) -> Any:
    """Handler for GET /query."""
    return Response(content="foo bar baz", status_code=200)


def create_app_query_params_required_string_query_parameter_success() -> Spikard:
    """App factory for fixture: Required string query parameter - success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/query",
        body_schema=None,
        parameter_schema={
            "properties": {"query": {"annotation": "str", "source": "query", "type": "string"}},
            "required": ["query"],
            "type": "object",
        },
        file_params=None,
    )(query_params_required_string_query_parameter_success)
    return app


def query_params_66_multipleof_constraint_success(
    quantity: int,
) -> Any:
    """Handler for GET /items."""
    return Response(content={"quantity": 15}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_query_params_66_multipleof_constraint_success() -> Spikard:
    """App factory for fixture: 66_multipleof_constraint_success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/items",
        body_schema=None,
        parameter_schema={
            "properties": {"quantity": {"multipleOf": 5, "source": "query", "type": "integer"}},
            "required": ["quantity"],
            "type": "object",
        },
        file_params=None,
    )(query_params_66_multipleof_constraint_success)
    return app


def query_params_53_integer_le_constraint_failure(
    limit: int,
) -> Any:
    """Handler for GET /items."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"maximum": 100, "value": 101},
                    "loc": ["query", "limit"],
                    "msg": "Value must not exceed 100",
                    "type": "validation_error",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_query_params_53_integer_le_constraint_failure() -> Spikard:
    """App factory for fixture: 53_integer_le_constraint_failure"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/items",
        body_schema=None,
        parameter_schema={
            "properties": {"limit": {"maximum": 100, "source": "query", "type": "integer"}},
            "required": ["limit"],
            "type": "object",
        },
        file_params=None,
    )(query_params_53_integer_le_constraint_failure)
    return app


def query_params_multiple_query_parameters_with_different_types(
    active: bool,
    age: int,
    name: str,
    score: float,
) -> Any:
    """Handler for GET /query/multi-type."""
    return Response(
        content={"active": True, "age": 30, "name": "john", "score": 95.5},
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_query_params_multiple_query_parameters_with_different_types() -> Spikard:
    """App factory for fixture: Multiple query parameters with different types"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/query/multi-type",
        body_schema=None,
        parameter_schema={
            "properties": {
                "active": {"annotation": "bool", "source": "query", "type": "boolean"},
                "age": {"annotation": "int", "source": "query", "type": "integer"},
                "name": {"annotation": "str", "source": "query", "type": "string"},
                "score": {"annotation": "float", "source": "query", "type": "number"},
            },
            "required": ["active", "age", "name", "score"],
            "type": "object",
        },
        file_params=None,
    )(query_params_multiple_query_parameters_with_different_types)
    return app


def query_params_71_array_separator_semicolon(
    colors: list[str],
) -> Any:
    """Handler for GET /items."""
    return Response(
        content={"colors": ["red", "green", "blue"]}, status_code=200, headers={"Content-Type": "application/json"}
    )


def create_app_query_params_71_array_separator_semicolon() -> Spikard:
    """App factory for fixture: 71_array_separator_semicolon"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/items",
        body_schema=None,
        parameter_schema={
            "properties": {
                "colors": {"items": {"type": "string"}, "separator": ";", "source": "query", "type": "array"}
            },
            "required": ["colors"],
            "type": "object",
        },
        file_params=None,
    )(query_params_71_array_separator_semicolon)
    return app


def query_params_70_array_separator_pipe(
    tags: list[str],
) -> Any:
    """Handler for GET /items."""
    return Response(
        content={"tags": ["python", "rust", "typescript"]},
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_query_params_70_array_separator_pipe() -> Spikard:
    """App factory for fixture: 70_array_separator_pipe"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/items",
        body_schema=None,
        parameter_schema={
            "properties": {"tags": {"items": {"type": "string"}, "separator": "|", "source": "query", "type": "array"}},
            "required": ["tags"],
            "type": "object",
        },
        file_params=None,
    )(query_params_70_array_separator_pipe)
    return app


def query_params_integer_with_default_value_not_provided(
    query: int | None = None,
) -> Any:
    """Handler for GET /query/int/default."""
    return Response(content="foo bar 10", status_code=200)


def create_app_query_params_integer_with_default_value_not_provided() -> Spikard:
    """App factory for fixture: Integer with default value - not provided"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/query/int/default",
        body_schema=None,
        parameter_schema={
            "properties": {"query": {"annotation": "int", "default": 10, "source": "query", "type": "integer"}},
            "required": [],
            "type": "object",
        },
        file_params=None,
    )(query_params_integer_with_default_value_not_provided)
    return app


def query_params_boolean_query_parameter_true(
    flag: bool,
) -> Any:
    """Handler for GET /query/bool."""
    return Response(content={"flag": True}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_query_params_boolean_query_parameter_true() -> Spikard:
    """App factory for fixture: Boolean query parameter - true"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/query/bool",
        body_schema=None,
        parameter_schema={
            "properties": {"flag": {"annotation": "bool", "source": "query", "type": "boolean"}},
            "required": ["flag"],
            "type": "object",
        },
        file_params=None,
    )(query_params_boolean_query_parameter_true)
    return app


def query_params_integer_query_param_with_le_constraint_boundary(
    value: int,
) -> Any:
    """Handler for GET /query/int-le."""
    return Response(content={"value": 100}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_query_params_integer_query_param_with_le_constraint_boundary() -> Spikard:
    """App factory for fixture: Integer query param with le constraint - boundary"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/query/int-le",
        body_schema=None,
        parameter_schema={
            "properties": {"value": {"annotation": "int", "maximum": 100, "source": "query", "type": "integer"}},
            "required": ["value"],
            "type": "object",
        },
        file_params=None,
    )(query_params_integer_query_param_with_le_constraint_boundary)
    return app


def query_params_float_query_param_with_ge_constraint_success(
    price: float,
) -> Any:
    """Handler for GET /query/float-ge."""
    return Response(content={"price": 0.01}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_query_params_float_query_param_with_ge_constraint_success() -> Spikard:
    """App factory for fixture: Float query param with ge constraint - success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/query/float-ge",
        body_schema=None,
        parameter_schema={
            "properties": {"price": {"annotation": "float", "minimum": 0.01, "source": "query", "type": "number"}},
            "required": ["price"],
            "type": "object",
        },
        file_params=None,
    )(query_params_float_query_param_with_ge_constraint_success)
    return app


def query_params_51_integer_ge_constraint_boundary(
    offset: int,
) -> Any:
    """Handler for GET /items."""
    return Response(content={"offset": 0}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_query_params_51_integer_ge_constraint_boundary() -> Spikard:
    """App factory for fixture: 51_integer_ge_constraint_boundary"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/items",
        body_schema=None,
        parameter_schema={
            "properties": {"offset": {"minimum": 0, "source": "query", "type": "integer"}},
            "required": ["offset"],
            "type": "object",
        },
        file_params=None,
    )(query_params_51_integer_ge_constraint_boundary)
    return app


def query_params_optional_integer_query_parameter_missing(
    query: int | None = None,
) -> Any:
    """Handler for GET /query/int/optional."""
    return Response(content="foo bar None", status_code=200)


def create_app_query_params_optional_integer_query_parameter_missing() -> Spikard:
    """App factory for fixture: Optional integer query parameter - missing"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/query/int/optional",
        body_schema=None,
        parameter_schema={
            "properties": {"query": {"annotation": "int", "source": "query", "type": "integer"}},
            "required": [],
            "type": "object",
        },
        file_params=None,
    )(query_params_optional_integer_query_parameter_missing)
    return app


def query_params_69_array_uniqueitems_failure(
    ids: list[int],
) -> Any:
    """Handler for GET /items."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"duplicate_index": 2, "duplicate_value": 2, "unique_items": True},
                    "loc": ["query", "ids"],
                    "msg": "Array items must be unique",
                    "type": "validation_error",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_query_params_69_array_uniqueitems_failure() -> Spikard:
    """App factory for fixture: 69_array_uniqueitems_failure"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/items",
        body_schema=None,
        parameter_schema={
            "properties": {
                "ids": {"items": {"type": "integer"}, "source": "query", "type": "array", "uniqueItems": True}
            },
            "required": ["ids"],
            "type": "object",
        },
        file_params=None,
    )(query_params_69_array_uniqueitems_failure)
    return app


def query_params_72_array_separator_space(
    keywords: list[str],
) -> Any:
    """Handler for GET /search."""
    return Response(
        content={"keywords": ["rust", "web", "framework"]},
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_query_params_72_array_separator_space() -> Spikard:
    """App factory for fixture: 72_array_separator_space"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/search",
        body_schema=None,
        parameter_schema={
            "properties": {
                "keywords": {"items": {"type": "string"}, "separator": " ", "source": "query", "type": "array"}
            },
            "required": ["keywords"],
            "type": "object",
        },
        file_params=None,
    )(query_params_72_array_separator_space)
    return app


def query_params_string_validation_with_regex_failure(
    item_query: str,
) -> Any:
    """Handler for GET /items/."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"pattern": "^fixedquery$"},
                    "input": "nonregexquery",
                    "loc": ["query", "item_query"],
                    "msg": "String should match pattern '^fixedquery$'",
                    "type": "string_pattern_mismatch",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_query_params_string_validation_with_regex_failure() -> Spikard:
    """App factory for fixture: String validation with regex - failure"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/items/",
        body_schema=None,
        parameter_schema={
            "properties": {
                "item_query": {"annotation": "str", "pattern": "^fixedquery$", "source": "query", "type": "string"}
            },
            "required": ["item_query"],
            "type": "object",
        },
        file_params=None,
    )(query_params_string_validation_with_regex_failure)
    return app


def query_params_65_format_hostname_success(
    host: str,
) -> Any:
    """Handler for GET /dns."""
    return Response(content={"host": "api.example.com"}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_query_params_65_format_hostname_success() -> Spikard:
    """App factory for fixture: 65_format_hostname_success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/dns",
        body_schema=None,
        parameter_schema={
            "properties": {"host": {"format": "hostname", "source": "query", "type": "string"}},
            "required": ["host"],
            "type": "object",
        },
        file_params=None,
    )(query_params_65_format_hostname_success)
    return app


def query_params_query_parameter_with_url_encoded_space(
    name: str,
) -> Any:
    """Handler for GET /query/basic."""
    return Response(content={"name": "hello world"}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_query_params_query_parameter_with_url_encoded_space() -> Spikard:
    """App factory for fixture: Query parameter with URL encoded space"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/query/basic",
        body_schema=None,
        parameter_schema={
            "properties": {"name": {"annotation": "str", "source": "query", "type": "string"}},
            "required": ["name"],
            "type": "object",
        },
        file_params=None,
    )(query_params_query_parameter_with_url_encoded_space)
    return app


def query_params_list_of_strings_multiple_values(
    q: list[str] | None = None,
) -> Any:
    """Handler for GET /items/."""
    return Response(content={"q": ["foo", "bar"]}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_query_params_list_of_strings_multiple_values() -> Spikard:
    """App factory for fixture: List of strings - multiple values"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/items/",
        body_schema=None,
        parameter_schema={
            "properties": {
                "q": {"annotation": "list[str]", "items": {"type": "string"}, "source": "query", "type": "array"}
            },
            "required": [],
            "type": "object",
        },
        file_params=None,
    )(query_params_list_of_strings_multiple_values)
    return app


def query_params_optional_query_parameter_with_default_value(
    limit: int | None = None,
) -> Any:
    """Handler for GET /query/optional-default."""
    return Response(content={"limit": 10}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_query_params_optional_query_parameter_with_default_value() -> Spikard:
    """App factory for fixture: Optional query parameter with default value"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/query/optional-default",
        body_schema=None,
        parameter_schema={
            "properties": {"limit": {"annotation": "int", "default": 10, "source": "query", "type": "integer"}},
            "required": [],
            "type": "object",
        },
        file_params=None,
    )(query_params_optional_query_parameter_with_default_value)
    return app


def query_params_62_format_ipv6_success(
    ip: str,
) -> Any:
    """Handler for GET /network/ipv6."""
    return Response(
        content={"ip": "2001:0db8:85a3:0000:0000:8a2e:0370:7334"},
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_query_params_62_format_ipv6_success() -> Spikard:
    """App factory for fixture: 62_format_ipv6_success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/network/ipv6",
        body_schema=None,
        parameter_schema={
            "properties": {"ip": {"format": "ipv6", "source": "query", "type": "string"}},
            "required": ["ip"],
            "type": "object",
        },
        file_params=None,
    )(query_params_62_format_ipv6_success)
    return app


def query_params_array_query_parameter_single_value(
    tags: list[str] | None = None,
) -> Any:
    """Handler for GET /query/list-default."""
    return Response(content=["apple"], status_code=200, headers={"Content-Type": "application/json"})


def create_app_query_params_array_query_parameter_single_value() -> Spikard:
    """App factory for fixture: Array query parameter - single value"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/query/list-default",
        body_schema=None,
        parameter_schema={
            "properties": {
                "tags": {
                    "annotation": "list[str]",
                    "default": [],
                    "items": {"type": "string"},
                    "source": "query",
                    "type": "array",
                }
            },
            "required": [],
            "type": "object",
        },
        file_params=None,
    )(query_params_array_query_parameter_single_value)
    return app


def query_params_optional_string_query_parameter_missing(
    query: str | None = None,
) -> Any:
    """Handler for GET /query/optional."""
    return Response(content="foo bar None", status_code=200)


def create_app_query_params_optional_string_query_parameter_missing() -> Spikard:
    """App factory for fixture: Optional string query parameter - missing"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/query/optional",
        body_schema=None,
        parameter_schema={
            "properties": {"query": {"annotation": "str", "source": "query", "type": "string"}},
            "required": [],
            "type": "object",
        },
        file_params=None,
    )(query_params_optional_string_query_parameter_missing)
    return app


def query_params_datetime_query_parameter_success(
    timestamp: datetime,
) -> Any:
    """Handler for GET /query/datetime."""
    return Response(
        content={"timestamp": "2024-01-15T10:30:00Z"}, status_code=200, headers={"Content-Type": "application/json"}
    )


def create_app_query_params_datetime_query_parameter_success() -> Spikard:
    """App factory for fixture: Datetime query parameter - success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/query/datetime",
        body_schema=None,
        parameter_schema={
            "properties": {
                "timestamp": {"annotation": "str", "format": "date-time", "source": "query", "type": "string"}
            },
            "required": ["timestamp"],
            "type": "object",
        },
        file_params=None,
    )(query_params_datetime_query_parameter_success)
    return app


def query_params_uuid_query_parameter_invalid_format(
    item_id: UUID,
) -> Any:
    """Handler for GET /query/uuid."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "input": "not-a-uuid",
                    "loc": ["query", "item_id"],
                    "msg": "Input should be a valid UUID",
                    "type": "uuid_parsing",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_query_params_uuid_query_parameter_invalid_format() -> Spikard:
    """App factory for fixture: UUID query parameter - invalid format"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/query/uuid",
        body_schema=None,
        parameter_schema={
            "properties": {"item_id": {"annotation": "str", "format": "uuid", "source": "query", "type": "string"}},
            "required": ["item_id"],
            "type": "object",
        },
        file_params=None,
    )(query_params_uuid_query_parameter_invalid_format)
    return app


def query_params_array_query_parameter_empty_array(
    tags: list[str] | None = None,
) -> Any:
    """Handler for GET /query/list-default."""
    # Echo back parameters for testing
    result: dict[str, Any] = {}
    if tags is not None:
        result["tags"] = tags
    return result


def create_app_query_params_array_query_parameter_empty_array() -> Spikard:
    """App factory for fixture: Array query parameter - empty array"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/query/list-default",
        body_schema=None,
        parameter_schema={
            "properties": {
                "tags": {
                    "annotation": "list[str]",
                    "default": [],
                    "items": {"type": "string"},
                    "source": "query",
                    "type": "array",
                }
            },
            "required": [],
            "type": "object",
        },
        file_params=None,
    )(query_params_array_query_parameter_empty_array)
    return app


def query_params_enum_query_parameter_success(
    model: str,
) -> Any:
    """Handler for GET /query/enum."""
    return Response(content={"model": "alexnet"}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_query_params_enum_query_parameter_success() -> Spikard:
    """App factory for fixture: Enum query parameter - success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/query/enum",
        body_schema=None,
        parameter_schema={
            "properties": {
                "model": {
                    "annotation": "str",
                    "enum": ["alexnet", "resnet", "lenet"],
                    "source": "query",
                    "type": "string",
                }
            },
            "required": ["model"],
            "type": "object",
        },
        file_params=None,
    )(query_params_enum_query_parameter_success)
    return app


def query_params_uuid_query_parameter_success(
    item_id: UUID,
) -> Any:
    """Handler for GET /query/uuid."""
    return Response(
        content={"item_id": "c892496f-b1fd-4b91-bdb8-b46f92df1716"},
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_query_params_uuid_query_parameter_success() -> Spikard:
    """App factory for fixture: UUID query parameter - success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/query/uuid",
        body_schema=None,
        parameter_schema={
            "properties": {"item_id": {"annotation": "str", "format": "uuid", "source": "query", "type": "string"}},
            "required": ["item_id"],
            "type": "object",
        },
        file_params=None,
    )(query_params_uuid_query_parameter_success)
    return app


def query_params_50_integer_gt_constraint_failure(
    limit: int,
) -> Any:
    """Handler for GET /items."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"exclusive_minimum": 0, "value": 0},
                    "loc": ["query", "limit"],
                    "msg": "Value must be greater than 0",
                    "type": "validation_error",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_query_params_50_integer_gt_constraint_failure() -> Spikard:
    """App factory for fixture: 50_integer_gt_constraint_failure"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/items",
        body_schema=None,
        parameter_schema={
            "properties": {"limit": {"exclusiveMinimum": 0, "source": "query", "type": "integer"}},
            "required": ["limit"],
            "type": "object",
        },
        file_params=None,
    )(query_params_50_integer_gt_constraint_failure)
    return app


def query_params_64_format_uri_failure(
    url: str,
) -> Any:
    """Handler for GET /redirect."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"format": "uri", "value": "not a uri"},
                    "loc": ["query", "url"],
                    "msg": "Invalid URI format",
                    "type": "validation_error",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_query_params_64_format_uri_failure() -> Spikard:
    """App factory for fixture: 64_format_uri_failure"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/redirect",
        body_schema=None,
        parameter_schema={
            "properties": {"url": {"format": "uri", "source": "query", "type": "string"}},
            "required": ["url"],
            "type": "object",
        },
        file_params=None,
    )(query_params_64_format_uri_failure)
    return app


def query_params_54_array_minitems_constraint_success(
    ids: list[int],
) -> Any:
    """Handler for GET /items."""
    return Response(content={"ids": [1, 2, 3]}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_query_params_54_array_minitems_constraint_success() -> Spikard:
    """App factory for fixture: 54_array_minitems_constraint_success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/items",
        body_schema=None,
        parameter_schema={
            "properties": {"ids": {"items": {"type": "integer"}, "minItems": 2, "source": "query", "type": "array"}},
            "required": ["ids"],
            "type": "object",
        },
        file_params=None,
    )(query_params_54_array_minitems_constraint_success)
    return app


def query_params_55_array_minitems_constraint_failure(
    ids: list[int],
) -> Any:
    """Handler for GET /items."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"actual_items": 1, "min_items": 2},
                    "loc": ["query", "ids"],
                    "msg": "Array must contain at least 2 items",
                    "type": "validation_error",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_query_params_55_array_minitems_constraint_failure() -> Spikard:
    """App factory for fixture: 55_array_minitems_constraint_failure"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/items",
        body_schema=None,
        parameter_schema={
            "properties": {"ids": {"items": {"type": "integer"}, "minItems": 2, "source": "query", "type": "array"}},
            "required": ["ids"],
            "type": "object",
        },
        file_params=None,
    )(query_params_55_array_minitems_constraint_failure)
    return app


def query_params_60_format_ipv4_success(
    ip: str,
) -> Any:
    """Handler for GET /network."""
    return Response(content={"ip": "192.168.1.1"}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_query_params_60_format_ipv4_success() -> Spikard:
    """App factory for fixture: 60_format_ipv4_success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/network",
        body_schema=None,
        parameter_schema={
            "properties": {"ip": {"format": "ipv4", "source": "query", "type": "string"}},
            "required": ["ip"],
            "type": "object",
        },
        file_params=None,
    )(query_params_60_format_ipv4_success)
    return app


def rate_limit_rate_limit_below_threshold_succeeds() -> Any:
    """Handler for GET /rate-limit/basic."""
    return Response(
        content={"request": "under-limit", "status": "ok"},
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_rate_limit_rate_limit_below_threshold_succeeds() -> Spikard:
    """App factory for fixture: Rate limit below threshold succeeds"""
    config = ServerConfig(rate_limit=RateLimitConfig(per_second=5, burst=5, ip_based=False))
    app = Spikard(config=config)
    # Register handler with this app instance
    app.register_route("GET", "/rate-limit/basic", body_schema=None, parameter_schema=None, file_params=None)(
        rate_limit_rate_limit_below_threshold_succeeds
    )
    return app


def rate_limit_rate_limit_exceeded_returns_429() -> Any:
    """Handler for GET /rate-limit/exceeded."""
    return Response(status_code=200)


def create_app_rate_limit_rate_limit_exceeded_returns_429() -> Spikard:
    """App factory for fixture: Rate limit exceeded returns 429"""
    config = ServerConfig(rate_limit=RateLimitConfig(per_second=1, burst=1, ip_based=False))
    app = Spikard(config=config)
    # Register handler with this app instance
    app.register_route("GET", "/rate-limit/exceeded", body_schema=None, parameter_schema=None, file_params=None)(
        rate_limit_rate_limit_exceeded_returns_429
    )
    return app


def body_limits_body_under_limit_succeeds(
    body: dict[str, Any],
) -> Any:
    """Handler for POST /body-limit/under."""
    return Response(
        content={"accepted": True, "note": "small"}, status_code=200, headers={"Content-Type": "application/json"}
    )


def create_app_body_limits_body_under_limit_succeeds() -> Spikard:
    """App factory for fixture: Body under limit succeeds"""
    config = ServerConfig(max_body_size=64)
    app = Spikard(config=config)
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/body-limit/under",
        body_schema={
            "additionalProperties": False,
            "properties": {"note": {"type": "string"}},
            "required": ["note"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(body_limits_body_under_limit_succeeds)
    return app


class BodyLimitsBodyOverLimitReturns413Body(TypedDict):
    """Request body type (TypedDict - runtime is dict)."""

    note: str


def body_limits_body_over_limit_returns_413(
    body: BodyLimitsBodyOverLimitReturns413Body,
) -> Any:
    """Handler for POST /body-limit/over."""
    return Response(status_code=413)


def create_app_body_limits_body_over_limit_returns_413() -> Spikard:
    """App factory for fixture: Body over limit returns 413"""
    config = ServerConfig(max_body_size=64)
    app = Spikard(config=config)
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/body-limit/over",
        body_schema={
            "additionalProperties": False,
            "properties": {"note": {"type": "string"}},
            "required": ["note"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(body_limits_body_over_limit_returns_413)
    return app


def auth_jwt_malformed_token_format(
    authorization: str,
) -> Any:
    """Handler for GET /api/protected."""
    return Response(
        content={
            "detail": "Malformed JWT token: expected 3 parts separated by dots, found 2",
            "status": 401,
            "title": "Malformed JWT token",
            "type": "https://spikard.dev/errors/unauthorized",
        },
        status_code=401,
        headers={"Content-Type": "application/json"},
    )


def create_app_auth_jwt_malformed_token_format() -> Spikard:
    """App factory for fixture: JWT malformed token format"""
    config = ServerConfig(
        jwt_auth=JwtConfig(
            secret="test-secret-key-do-not-use-in-production",
            algorithm="HS256",
        ),
    )
    app = Spikard(config=config)
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/api/protected",
        body_schema=None,
        parameter_schema={
            "properties": {
                "authorization": {"description": "JWT token in Bearer format", "source": "header", "type": "string"}
            },
            "required": ["authorization"],
            "type": "object",
        },
        file_params=None,
    )(auth_jwt_malformed_token_format)
    return app


def auth_bearer_token_without_prefix(
    authorization: str,
) -> Any:
    """Handler for GET /api/protected."""
    return Response(
        content={
            "detail": "Authorization header must use Bearer scheme: 'Bearer <token>'",
            "status": 401,
            "title": "Invalid Authorization header format",
            "type": "https://spikard.dev/errors/unauthorized",
        },
        status_code=401,
        headers={"Content-Type": "application/json"},
    )


def create_app_auth_bearer_token_without_prefix() -> Spikard:
    """App factory for fixture: Bearer token without prefix"""
    config = ServerConfig(
        jwt_auth=JwtConfig(
            secret="test-secret-key-do-not-use-in-production",
            algorithm="HS256",
        ),
    )
    app = Spikard(config=config)
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/api/protected",
        body_schema=None,
        parameter_schema={
            "properties": {
                "authorization": {"description": "JWT token in Bearer format", "source": "header", "type": "string"}
            },
            "required": ["authorization"],
            "type": "object",
        },
        file_params=None,
    )(auth_bearer_token_without_prefix)
    return app


def auth_jwt_authentication_valid_token(
    authorization: str,
) -> Any:
    """Handler for GET /protected/user."""
    return Response(
        content={"message": "Access granted", "user_id": "user123"},
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_auth_jwt_authentication_valid_token() -> Spikard:
    """App factory for fixture: JWT authentication - valid token"""
    config = ServerConfig(
        jwt_auth=JwtConfig(
            secret="test-secret-key-do-not-use-in-production",
            algorithm="HS256",
            audience=["https://api.example.com"],
            issuer="https://auth.example.com",
        ),
    )
    app = Spikard(config=config)
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/protected/user",
        body_schema=None,
        parameter_schema={
            "properties": {
                "authorization": {"description": "JWT token in Bearer format", "source": "header", "type": "string"}
            },
            "required": ["authorization"],
            "type": "object",
        },
        file_params=None,
    )(auth_jwt_authentication_valid_token)
    return app


def auth_api_key_rotation_old_key_still_valid(
    x_api_key: str,
) -> Any:
    """Handler for GET /api/data."""
    return Response(
        content={"data": "sensitive information", "message": "Access granted"},
        status_code=200,
        headers={"Content-Type": "application/json", "X-API-Key-Deprecated": "true"},
    )


def create_app_auth_api_key_rotation_old_key_still_valid() -> Spikard:
    """App factory for fixture: API key rotation - old key still valid"""
    config = ServerConfig(
        api_key_auth=ApiKeyConfig(
            keys=["sk_test_old_123456", "sk_test_new_789012"],
            header_name="X-API-Key",
        ),
    )
    app = Spikard(config=config)
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/api/data",
        body_schema=None,
        parameter_schema={
            "properties": {
                "x_api_key": {"description": "API key for authentication", "source": "header", "type": "string"}
            },
            "required": ["x_api_key"],
            "type": "object",
        },
        file_params=None,
    )(auth_api_key_rotation_old_key_still_valid)
    return app


def auth_jwt_invalid_issuer(
    authorization: str,
) -> Any:
    """Handler for GET /api/protected."""
    return Response(
        content={
            "detail": "Token issuer is invalid, expected 'https://auth.example.com'",
            "status": 401,
            "title": "JWT validation failed",
            "type": "https://spikard.dev/errors/unauthorized",
        },
        status_code=401,
        headers={"Content-Type": "application/json"},
    )


def create_app_auth_jwt_invalid_issuer() -> Spikard:
    """App factory for fixture: JWT invalid issuer"""
    config = ServerConfig(
        jwt_auth=JwtConfig(
            secret="test-secret-key-do-not-use-in-production",
            algorithm="HS256",
            issuer="https://auth.example.com",
            leeway=0,
        ),
    )
    app = Spikard(config=config)
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/api/protected",
        body_schema=None,
        parameter_schema={
            "properties": {
                "authorization": {"description": "JWT token in Bearer format", "source": "header", "type": "string"}
            },
            "required": ["authorization"],
            "type": "object",
        },
        file_params=None,
    )(auth_jwt_invalid_issuer)
    return app


def auth_jwt_with_multiple_audiences(
    authorization: str,
) -> Any:
    """Handler for GET /api/protected."""
    return Response(
        content={"message": "Access granted", "user_id": "user123"},
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_auth_jwt_with_multiple_audiences() -> Spikard:
    """App factory for fixture: JWT with multiple audiences"""
    config = ServerConfig(
        jwt_auth=JwtConfig(
            secret="test-secret-key-do-not-use-in-production",
            algorithm="HS256",
            audience=["https://api.example.com"],
            issuer="https://auth.example.com",
        ),
    )
    app = Spikard(config=config)
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/api/protected",
        body_schema=None,
        parameter_schema={
            "properties": {
                "authorization": {"description": "JWT token in Bearer format", "source": "header", "type": "string"}
            },
            "required": ["authorization"],
            "type": "object",
        },
        file_params=None,
    )(auth_jwt_with_multiple_audiences)
    return app


def auth_api_key_in_query_parameter() -> Any:
    """Handler for GET /api/data."""
    return Response(
        content={"data": "sensitive information", "message": "Access granted"},
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_auth_api_key_in_query_parameter() -> Spikard:
    """App factory for fixture: API key in query parameter"""
    config = ServerConfig(
        api_key_auth=ApiKeyConfig(
            keys=["sk_test_123456", "sk_test_789012"],
            header_name="X-API-Key",
        ),
    )
    app = Spikard(config=config)
    # Register handler with this app instance
    app.register_route("GET", "/api/data", body_schema=None, parameter_schema=None, file_params=None)(
        auth_api_key_in_query_parameter
    )
    return app


def auth_jwt_authentication_expired_token(
    authorization: str,
) -> Any:
    """Handler for GET /protected/user."""
    return Response(
        content={
            "detail": "Token has expired",
            "status": 401,
            "title": "JWT validation failed",
            "type": "https://spikard.dev/errors/unauthorized",
        },
        status_code=401,
        headers={"Content-Type": "application/json"},
    )


def create_app_auth_jwt_authentication_expired_token() -> Spikard:
    """App factory for fixture: JWT authentication - expired token"""
    config = ServerConfig(
        jwt_auth=JwtConfig(
            secret="test-secret-key-do-not-use-in-production",
            algorithm="HS256",
        ),
    )
    app = Spikard(config=config)
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/protected/user",
        body_schema=None,
        parameter_schema={
            "properties": {"authorization": {"source": "header", "type": "string"}},
            "required": ["authorization"],
            "type": "object",
        },
        file_params=None,
    )(auth_jwt_authentication_expired_token)
    return app


def auth_api_key_authentication_invalid_key(
    x_api_key: str,
) -> Any:
    """Handler for GET /api/data."""
    return Response(
        content={
            "detail": "The provided API key is not valid",
            "status": 401,
            "title": "Invalid API key",
            "type": "https://spikard.dev/errors/unauthorized",
        },
        status_code=401,
        headers={"Content-Type": "application/json"},
    )


def create_app_auth_api_key_authentication_invalid_key() -> Spikard:
    """App factory for fixture: API key authentication - invalid key"""
    config = ServerConfig(
        api_key_auth=ApiKeyConfig(
            keys=["sk_test_123456", "sk_test_789012"],
            header_name="X-API-Key",
        ),
    )
    app = Spikard(config=config)
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/api/data",
        body_schema=None,
        parameter_schema={
            "properties": {"x_api_key": {"source": "header", "type": "string"}},
            "required": ["x_api_key"],
            "type": "object",
        },
        file_params=None,
    )(auth_api_key_authentication_invalid_key)
    return app


def auth_jwt_not_before_claim_in_future(
    authorization: str,
) -> Any:
    """Handler for GET /api/protected."""
    return Response(
        content={
            "detail": "JWT not valid yet, not before claim is in the future",
            "status": 401,
            "title": "JWT validation failed",
            "type": "https://spikard.dev/errors/unauthorized",
        },
        status_code=401,
        headers={"Content-Type": "application/json"},
    )


def create_app_auth_jwt_not_before_claim_in_future() -> Spikard:
    """App factory for fixture: JWT not before claim in future"""
    config = ServerConfig(
        jwt_auth=JwtConfig(
            secret="test-secret-key-do-not-use-in-production",
            algorithm="HS256",
            leeway=0,
        ),
    )
    app = Spikard(config=config)
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/api/protected",
        body_schema=None,
        parameter_schema={
            "properties": {
                "authorization": {"description": "JWT token in Bearer format", "source": "header", "type": "string"}
            },
            "required": ["authorization"],
            "type": "object",
        },
        file_params=None,
    )(auth_jwt_not_before_claim_in_future)
    return app


def auth_multiple_authentication_schemes_jwt_precedence(
    authorization: str,
    x_api_key: str,
) -> Any:
    """Handler for GET /api/data."""
    return Response(
        content={"auth_method": "jwt", "message": "Access granted", "user_id": "user123"},
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_auth_multiple_authentication_schemes_jwt_precedence() -> Spikard:
    """App factory for fixture: Multiple authentication schemes - JWT precedence"""
    config = ServerConfig(
        jwt_auth=JwtConfig(
            secret="test-secret-key-do-not-use-in-production",
            algorithm="HS256",
            audience=["https://api.example.com"],
            issuer="https://auth.example.com",
        ),
        api_key_auth=ApiKeyConfig(
            keys=["sk_test_123456", "sk_test_789012"],
            header_name="X-API-Key",
        ),
    )
    app = Spikard(config=config)
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/api/data",
        body_schema=None,
        parameter_schema={
            "properties": {
                "authorization": {"description": "JWT token in Bearer format", "source": "header", "type": "string"},
                "x_api_key": {"description": "API key for authentication", "source": "header", "type": "string"},
            },
            "required": ["authorization", "x_api_key"],
            "type": "object",
        },
        file_params=None,
    )(auth_multiple_authentication_schemes_jwt_precedence)
    return app


def auth_jwt_missing_required_custom_claims(
    authorization: str,
) -> Any:
    """Handler for GET /api/admin."""
    return Response(
        content={
            "detail": "Required claims 'role' and 'permissions' missing from JWT",
            "status": 403,
            "title": "Forbidden",
            "type": "https://spikard.dev/errors/forbidden",
        },
        status_code=403,
        headers={"Content-Type": "application/json"},
    )


def create_app_auth_jwt_missing_required_custom_claims() -> Spikard:
    """App factory for fixture: JWT missing required custom claims"""
    config = ServerConfig(
        jwt_auth=JwtConfig(
            secret="test-secret-key-do-not-use-in-production",
            algorithm="HS256",
            audience=["https://api.example.com"],
            issuer="https://auth.example.com",
        ),
    )
    app = Spikard(config=config)
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/api/admin",
        body_schema=None,
        parameter_schema={
            "properties": {
                "authorization": {"description": "JWT token in Bearer format", "source": "header", "type": "string"}
            },
            "required": ["authorization"],
            "type": "object",
        },
        file_params=None,
    )(auth_jwt_missing_required_custom_claims)
    return app


def auth_api_key_authentication_valid_key(
    x_api_key: str,
) -> Any:
    """Handler for GET /api/data."""
    return Response(
        content={"data": "sensitive information", "message": "Access granted"},
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_auth_api_key_authentication_valid_key() -> Spikard:
    """App factory for fixture: API key authentication - valid key"""
    config = ServerConfig(
        api_key_auth=ApiKeyConfig(
            keys=["sk_test_123456", "sk_test_789012"],
            header_name="X-API-Key",
        ),
    )
    app = Spikard(config=config)
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/api/data",
        body_schema=None,
        parameter_schema={
            "properties": {
                "x_api_key": {"description": "API key for authentication", "source": "header", "type": "string"}
            },
            "required": ["x_api_key"],
            "type": "object",
        },
        file_params=None,
    )(auth_api_key_authentication_valid_key)
    return app


def auth_api_key_with_custom_header_name(
    x_api_token: str,
) -> Any:
    """Handler for GET /api/data."""
    return Response(
        content={"data": "sensitive information", "message": "Access granted"},
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_auth_api_key_with_custom_header_name() -> Spikard:
    """App factory for fixture: API key with custom header name"""
    config = ServerConfig(
        api_key_auth=ApiKeyConfig(
            keys=["sk_test_123456", "sk_test_789012"],
            header_name="X-API-Token",
        ),
    )
    app = Spikard(config=config)
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/api/data",
        body_schema=None,
        parameter_schema={
            "properties": {
                "x_api_token": {"description": "API token for authentication", "source": "header", "type": "string"}
            },
            "required": ["x_api_token"],
            "type": "object",
        },
        file_params=None,
    )(auth_api_key_with_custom_header_name)
    return app


def auth_api_key_authentication_missing_header() -> Any:
    """Handler for GET /api/data."""
    return Response(
        content={
            "detail": "Expected 'X-API-Key' header or 'api_key' query parameter with valid API key",
            "status": 401,
            "title": "Missing API key",
            "type": "https://spikard.dev/errors/unauthorized",
        },
        status_code=401,
        headers={"Content-Type": "application/json"},
    )


def create_app_auth_api_key_authentication_missing_header() -> Spikard:
    """App factory for fixture: API key authentication - missing header"""
    config = ServerConfig(
        api_key_auth=ApiKeyConfig(
            keys=["sk_test_123456", "sk_test_789012"],
            header_name="X-API-Key",
        ),
    )
    app = Spikard(config=config)
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/api/data",
        body_schema=None,
        parameter_schema={"properties": {}, "required": [], "type": "object"},
        file_params=None,
    )(auth_api_key_authentication_missing_header)
    return app


def auth_jwt_authentication_invalid_signature(
    authorization: str,
) -> Any:
    """Handler for GET /protected/user."""
    return Response(
        content={
            "detail": "Token signature is invalid",
            "status": 401,
            "title": "JWT validation failed",
            "type": "https://spikard.dev/errors/unauthorized",
        },
        status_code=401,
        headers={"Content-Type": "application/json"},
    )


def create_app_auth_jwt_authentication_invalid_signature() -> Spikard:
    """App factory for fixture: JWT authentication - invalid signature"""
    config = ServerConfig(
        jwt_auth=JwtConfig(
            secret="test-secret-key-do-not-use-in-production",
            algorithm="HS256",
        ),
    )
    app = Spikard(config=config)
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/protected/user",
        body_schema=None,
        parameter_schema={
            "properties": {"authorization": {"source": "header", "type": "string"}},
            "required": ["authorization"],
            "type": "object",
        },
        file_params=None,
    )(auth_jwt_authentication_invalid_signature)
    return app


def auth_jwt_authentication_missing_authorization_header() -> Any:
    """Handler for GET /protected/user."""
    return Response(
        content={
            "detail": "Expected 'Authorization: Bearer <token>'",
            "status": 401,
            "title": "Missing or invalid Authorization header",
            "type": "https://spikard.dev/errors/unauthorized",
        },
        status_code=401,
        headers={"Content-Type": "application/json"},
    )


def create_app_auth_jwt_authentication_missing_authorization_header() -> Spikard:
    """App factory for fixture: JWT authentication - missing Authorization header"""
    config = ServerConfig(
        jwt_auth=JwtConfig(
            secret="test-secret-key-do-not-use-in-production",
            algorithm="HS256",
        ),
    )
    app = Spikard(config=config)
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/protected/user",
        body_schema=None,
        parameter_schema={"properties": {}, "required": [], "type": "object"},
        file_params=None,
    )(auth_jwt_authentication_missing_authorization_header)
    return app


def auth_jwt_authentication_invalid_audience(
    authorization: str,
) -> Any:
    """Handler for GET /protected/user."""
    return Response(
        content={
            "detail": "Token audience is invalid",
            "status": 401,
            "title": "JWT validation failed",
            "type": "https://spikard.dev/errors/unauthorized",
        },
        status_code=401,
        headers={"Content-Type": "application/json"},
    )


def create_app_auth_jwt_authentication_invalid_audience() -> Spikard:
    """App factory for fixture: JWT authentication - invalid audience"""
    config = ServerConfig(
        jwt_auth=JwtConfig(
            secret="test-secret-key-do-not-use-in-production",
            algorithm="HS256",
            audience=["https://api.example.com"],
        ),
    )
    app = Spikard(config=config)
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/protected/user",
        body_schema=None,
        parameter_schema={
            "properties": {"authorization": {"source": "header", "type": "string"}},
            "required": ["authorization"],
            "type": "object",
        },
        file_params=None,
    )(auth_jwt_authentication_invalid_audience)
    return app


def http_methods_options_cors_preflight_request() -> Any:
    """Handler for OPTIONS /items/."""
    return Response(
        status_code=200,
        headers={
            "Access-Control-Max-Age": "86400",
            "Access-Control-Allow-Methods": "GET, POST, PUT, DELETE, OPTIONS",
            "Access-Control-Allow-Origin": "https://example.com",
            "Access-Control-Allow-Headers": "Content-Type",
        },
    )


def create_app_http_methods_options_cors_preflight_request() -> Spikard:
    """App factory for fixture: OPTIONS - CORS preflight request"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route("OPTIONS", "/items/", body_schema=None, parameter_schema=None, file_params=None)(
        http_methods_options_cors_preflight_request
    )
    return app


def http_methods_delete_remove_resource(
    id: str,
) -> Any:
    """Handler for DELETE /items/{id}."""
    # Echo back parameters for testing
    result: dict[str, Any] = {}
    if id is not None:
        result["id"] = id
    return result


def create_app_http_methods_delete_remove_resource() -> Spikard:
    """App factory for fixture: DELETE - Remove resource"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "DELETE",
        "/items/{id}",
        body_schema=None,
        parameter_schema={
            "properties": {"id": {"source": "path", "type": "string"}},
            "required": ["id"],
            "type": "object",
        },
        file_params=None,
    )(http_methods_delete_remove_resource)
    return app


@dataclass
class HttpMethodsPutCreateResourceIfDoesnTExistBody:
    """Request body dataclass."""

    id: int
    name: str
    price: float


def http_methods_put_create_resource_if_doesn_t_exist(
    body: HttpMethodsPutCreateResourceIfDoesnTExistBody,
    id: str,
) -> Any:
    """Handler for PUT /items/{id}."""
    return Response(
        content={"id": 999, "name": "New Item", "price": 49.99},
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_http_methods_put_create_resource_if_doesn_t_exist() -> Spikard:
    """App factory for fixture: PUT - Create resource if doesn't exist"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "PUT",
        "/items/{id}",
        body_schema={
            "properties": {"id": {"type": "integer"}, "name": {"type": "string"}, "price": {"type": "number"}},
            "required": ["id", "name", "price"],
            "type": "object",
        },
        parameter_schema={
            "properties": {"id": {"source": "path", "type": "string"}},
            "required": ["id"],
            "type": "object",
        },
        file_params=None,
    )(http_methods_put_create_resource_if_doesn_t_exist)
    return app


class HttpMethodsPatchUpdateMultipleFieldsBody(NamedTuple):
    """Request body NamedTuple (immutable)."""

    in_stock: bool
    name: str
    price: float


def http_methods_patch_update_multiple_fields(
    body: HttpMethodsPatchUpdateMultipleFieldsBody,
    id: str,
) -> Any:
    """Handler for PATCH /items/{id}."""
    return Response(
        content={"id": 1, "in_stock": False, "name": "Updated Name", "price": 89.99},
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_http_methods_patch_update_multiple_fields() -> Spikard:
    """App factory for fixture: PATCH - Update multiple fields"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "PATCH",
        "/items/{id}",
        body_schema={
            "properties": {"in_stock": {"type": "boolean"}, "name": {"type": "string"}, "price": {"type": "number"}},
            "required": ["in_stock", "name", "price"],
            "type": "object",
        },
        parameter_schema={
            "properties": {"id": {"source": "path", "type": "string"}},
            "required": ["id"],
            "type": "object",
        },
        file_params=None,
    )(http_methods_patch_update_multiple_fields)
    return app


class HttpMethodsPutValidationErrorBody(msgspec.Struct):
    """Request body msgspec.Struct (fast typed)."""

    id: int
    name: str
    price: float


def http_methods_put_validation_error(
    body: HttpMethodsPutValidationErrorBody,
    id: str,
) -> Any:
    """Handler for PUT /items/{id}."""
    return Response(
        content={
            "detail": "2 validation errors in request",
            "errors": [
                {
                    "input": "X",
                    "loc": ["body", "name"],
                    "msg": "String should have at least 3 characters",
                    "type": "string_too_short",
                },
                {
                    "input": -10,
                    "loc": ["body", "price"],
                    "msg": "Input should be greater than 0",
                    "type": "greater_than",
                },
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_http_methods_put_validation_error() -> Spikard:
    """App factory for fixture: PUT - Validation error"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "PUT",
        "/items/{id}",
        body_schema={
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "properties": {
                "id": {"type": "integer"},
                "name": {"minLength": 3, "type": "string"},
                "price": {"exclusiveMinimum": 0, "type": "number"},
            },
            "required": ["id", "name", "price"],
            "type": "object",
        },
        parameter_schema={
            "properties": {"id": {"source": "path", "type": "string"}},
            "required": ["id"],
            "type": "object",
        },
        file_params=None,
    )(http_methods_put_validation_error)
    return app


def http_methods_head_get_metadata_without_body(
    id: str,
) -> Any:
    """Handler for HEAD /items/{id}."""
    return Response(status_code=200, headers={"Content-Length": "85", "Content-Type": "application/json"})


def create_app_http_methods_head_get_metadata_without_body() -> Spikard:
    """App factory for fixture: HEAD - Get metadata without body"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "HEAD",
        "/items/{id}",
        body_schema=None,
        parameter_schema={
            "properties": {"id": {"source": "path", "type": "string"}},
            "required": ["id"],
            "type": "object",
        },
        file_params=None,
    )(http_methods_head_get_metadata_without_body)
    return app


def http_methods_delete_with_response_body(
    id: str,
) -> Any:
    """Handler for DELETE /items/{id}."""
    return Response(
        content={"id": 1, "message": "Item deleted successfully", "name": "Deleted Item"},
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_http_methods_delete_with_response_body() -> Spikard:
    """App factory for fixture: DELETE - With response body"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "DELETE",
        "/items/{id}",
        body_schema=None,
        parameter_schema={
            "properties": {"id": {"source": "path", "type": "string"}},
            "required": ["id"],
            "type": "object",
        },
        file_params=None,
    )(http_methods_delete_with_response_body)
    return app


class HttpMethodsPutMissingRequiredFieldBody(TypedDict):
    """Request body type (TypedDict - runtime is dict)."""

    price: str
    id: int | None
    name: str | None


def http_methods_put_missing_required_field(
    body: HttpMethodsPutMissingRequiredFieldBody,
    id: str,
) -> Any:
    """Handler for PUT /items/{id}."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [{"input": "1", "loc": ["body", "price"], "msg": "Field required", "type": "missing"}],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_http_methods_put_missing_required_field() -> Spikard:
    """App factory for fixture: PUT - Missing required field"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "PUT",
        "/items/{id}",
        body_schema={
            "properties": {"id": {"type": "integer"}, "name": {"type": "string"}, "price": {"type": "string"}},
            "required": ["price"],
            "type": "object",
        },
        parameter_schema={
            "properties": {"id": {"source": "path", "type": "string"}},
            "required": ["id"],
            "type": "object",
        },
        file_params=None,
    )(http_methods_put_missing_required_field)
    return app


@dataclass
class HttpMethodsPatchPartialUpdateBody:
    """Request body dataclass."""

    price: float


def http_methods_patch_partial_update(
    body: HttpMethodsPatchPartialUpdateBody,
    id: str,
) -> Any:
    """Handler for PATCH /items/{id}."""
    return Response(
        content={"id": 1, "in_stock": True, "name": "Existing Item", "price": 79.99},
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_http_methods_patch_partial_update() -> Spikard:
    """App factory for fixture: PATCH - Partial update"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "PATCH",
        "/items/{id}",
        body_schema={"properties": {"price": {"type": "number"}}, "required": ["price"], "type": "object"},
        parameter_schema={
            "properties": {"id": {"source": "path", "type": "string"}},
            "required": ["id"],
            "type": "object",
        },
        file_params=None,
    )(http_methods_patch_partial_update)
    return app


def http_methods_delete_resource_not_found(
    id: str,
) -> Any:
    """Handler for DELETE /items/{id}."""
    # Echo back parameters for testing
    result: dict[str, Any] = {}
    if id is not None:
        result["id"] = id
    return result


def create_app_http_methods_delete_resource_not_found() -> Spikard:
    """App factory for fixture: DELETE - Resource not found"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "DELETE",
        "/items/{id}",
        body_schema=None,
        parameter_schema={
            "properties": {"id": {"source": "path", "type": "string"}},
            "required": ["id"],
            "type": "object",
        },
        file_params=None,
    )(http_methods_delete_resource_not_found)
    return app


class HttpMethodsPutIdempotentOperationBody(msgspec.Struct):
    """Request body msgspec.Struct (fast typed)."""

    id: int
    name: str
    price: float


def http_methods_put_idempotent_operation(
    body: HttpMethodsPutIdempotentOperationBody,
    id: str,
) -> Any:
    """Handler for PUT /items/{id}."""
    return Response(
        content={"id": 1, "name": "Fixed Name", "price": 50.0},
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_http_methods_put_idempotent_operation() -> Spikard:
    """App factory for fixture: PUT - Idempotent operation"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "PUT",
        "/items/{id}",
        body_schema={
            "properties": {"id": {"type": "integer"}, "name": {"type": "string"}, "price": {"type": "number"}},
            "required": ["id", "name", "price"],
            "type": "object",
        },
        parameter_schema={
            "properties": {"id": {"source": "path", "type": "string"}},
            "required": ["id"],
            "type": "object",
        },
        file_params=None,
    )(http_methods_put_idempotent_operation)
    return app


class HttpMethodsPutCompleteResourceReplacementBody(BaseModel):
    """Request body Pydantic model."""

    description: str
    id: int
    in_stock: bool
    name: str
    price: float


def http_methods_put_complete_resource_replacement(
    body: HttpMethodsPutCompleteResourceReplacementBody,
    id: str,
) -> Any:
    """Handler for PUT /items/{id}."""
    return Response(
        content={
            "description": "Completely replaced",
            "id": 1,
            "in_stock": True,
            "name": "Updated Item",
            "price": 99.99,
        },
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_http_methods_put_complete_resource_replacement() -> Spikard:
    """App factory for fixture: PUT - Complete resource replacement"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "PUT",
        "/items/{id}",
        body_schema={
            "properties": {
                "description": {"type": "string"},
                "id": {"type": "integer"},
                "in_stock": {"type": "boolean"},
                "name": {"type": "string"},
                "price": {"type": "number"},
            },
            "required": ["description", "id", "in_stock", "name", "price"],
            "type": "object",
        },
        parameter_schema={
            "properties": {"id": {"source": "path", "type": "string"}},
            "required": ["id"],
            "type": "object",
        },
        file_params=None,
    )(http_methods_put_complete_resource_replacement)
    return app


def path_params_boolean_path_parameter_true(
    item_id: bool,
) -> Any:
    """Handler for GET /path/bool/{item_id}."""
    return Response(content={"item_id": True}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_path_params_boolean_path_parameter_true() -> Spikard:
    """App factory for fixture: Boolean path parameter - True"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/path/bool/{item_id}",
        body_schema=None,
        parameter_schema={
            "properties": {"item_id": {"source": "path", "type": "boolean"}},
            "required": ["item_id"],
            "type": "object",
        },
        file_params=None,
    )(path_params_boolean_path_parameter_true)
    return app


def path_params_29_decimal_path_param_success(
    amount: str,
) -> Any:
    """Handler for GET /prices/{amount}."""
    return Response(content={"amount": "19.99"}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_path_params_29_decimal_path_param_success() -> Spikard:
    """App factory for fixture: 29_decimal_path_param_success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/prices/{amount}",
        body_schema=None,
        parameter_schema={
            "properties": {"amount": {"format": "decimal", "source": "path", "type": "string"}},
            "required": ["amount"],
            "type": "object",
        },
        file_params=None,
    )(path_params_29_decimal_path_param_success)
    return app


def path_params_integer_path_parameter_with_combined_lt_and_gt_constraints_success(
    item_id: int,
) -> Any:
    """Handler for GET /path/param-lt-gt/{item_id}."""
    return Response(content={"item_id": 2}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_path_params_integer_path_parameter_with_combined_lt_and_gt_constraints_success() -> Spikard:
    """App factory for fixture: Integer path parameter with combined lt and gt constraints - success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/path/param-lt-gt/{item_id}",
        body_schema=None,
        parameter_schema={
            "properties": {
                "item_id": {"exclusiveMaximum": 3, "exclusiveMinimum": 1, "source": "path", "type": "integer"}
            },
            "required": ["item_id"],
            "type": "object",
        },
        file_params=None,
    )(path_params_integer_path_parameter_with_combined_lt_and_gt_constraints_success)
    return app


def path_params_33_string_pattern_path_success(
    owner: str,
    repo: str,
) -> Any:
    """Handler for GET /repos/{owner}/{repo}."""
    return Response(
        content={"owner": "spikard-labs", "repo": "spikard-http"},
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_path_params_33_string_pattern_path_success() -> Spikard:
    """App factory for fixture: 33_string_pattern_path_success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/repos/{owner}/{repo}",
        body_schema=None,
        parameter_schema={
            "properties": {
                "owner": {"pattern": "^[a-zA-Z0-9-]+$", "source": "path", "type": "string"},
                "repo": {"pattern": "^[a-zA-Z0-9-_]+$", "source": "path", "type": "string"},
            },
            "required": ["owner", "repo"],
            "type": "object",
        },
        file_params=None,
    )(path_params_33_string_pattern_path_success)
    return app


def path_params_31_string_minlength_path_failure(
    username: str,
) -> Any:
    """Handler for GET /users/{username}."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"actual_length": 2, "min_length": 3},
                    "loc": ["path", "username"],
                    "msg": "String length must be at least 3",
                    "type": "validation_error",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_path_params_31_string_minlength_path_failure() -> Spikard:
    """App factory for fixture: 31_string_minlength_path_failure"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/users/{username}",
        body_schema=None,
        parameter_schema={
            "properties": {"username": {"minLength": 3, "source": "path", "type": "string"}},
            "required": ["username"],
            "type": "object",
        },
        file_params=None,
    )(path_params_31_string_minlength_path_failure)
    return app


def path_params_35_negative_integer_path_param(
    value: int,
) -> Any:
    """Handler for GET /offset/{value}."""
    return Response(content={"value": -100}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_path_params_35_negative_integer_path_param() -> Spikard:
    """App factory for fixture: 35_negative_integer_path_param"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/offset/{value}",
        body_schema=None,
        parameter_schema={
            "properties": {"value": {"source": "path", "type": "integer"}},
            "required": ["value"],
            "type": "object",
        },
        file_params=None,
    )(path_params_35_negative_integer_path_param)
    return app


def path_params_enum_path_parameter_invalid_value(
    model_name: str,
) -> Any:
    """Handler for GET /models/{model_name}."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"expected": "'alexnet', 'resnet' or 'lenet'"},
                    "input": "foo",
                    "loc": ["path", "model_name"],
                    "msg": "Input should be 'alexnet', 'resnet' or 'lenet'",
                    "type": "enum",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_path_params_enum_path_parameter_invalid_value() -> Spikard:
    """App factory for fixture: Enum path parameter - invalid value"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/models/{model_name}",
        body_schema=None,
        parameter_schema={
            "properties": {"model_name": {"enum": ["alexnet", "resnet", "lenet"], "source": "path", "type": "string"}},
            "required": ["model_name"],
            "type": "object",
        },
        file_params=None,
    )(path_params_enum_path_parameter_invalid_value)
    return app


def path_params_27_datetime_format_path_param_success(
    timestamp: datetime,
) -> Any:
    """Handler for GET /bookings/{timestamp}."""
    return Response(
        content={"timestamp": "2025-10-30T14:30:00Z"}, status_code=200, headers={"Content-Type": "application/json"}
    )


def create_app_path_params_27_datetime_format_path_param_success() -> Spikard:
    """App factory for fixture: 27_datetime_format_path_param_success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/bookings/{timestamp}",
        body_schema=None,
        parameter_schema={
            "properties": {"timestamp": {"format": "date-time", "source": "path", "type": "string"}},
            "required": ["timestamp"],
            "type": "object",
        },
        file_params=None,
    )(path_params_27_datetime_format_path_param_success)
    return app


def path_params_25_date_format_invalid_failure(
    date: date,
) -> Any:
    """Handler for GET /events/{date}."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"format": "date", "value": "2025-13-45"},
                    "loc": ["path", "date"],
                    "msg": "Invalid date format",
                    "type": "validation_error",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_path_params_25_date_format_invalid_failure() -> Spikard:
    """App factory for fixture: 25_date_format_invalid_failure"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/events/{date}",
        body_schema=None,
        parameter_schema={
            "properties": {"date": {"format": "date", "source": "path", "type": "string"}},
            "required": ["date"],
            "type": "object",
        },
        file_params=None,
    )(path_params_25_date_format_invalid_failure)
    return app


def path_params_integer_path_parameter_with_lt_constraint_success(
    item_id: int,
) -> Any:
    """Handler for GET /path/param-lt/{item_id}."""
    return Response(content={"item_id": 2}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_path_params_integer_path_parameter_with_lt_constraint_success() -> Spikard:
    """App factory for fixture: Integer path parameter with lt constraint - success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/path/param-lt/{item_id}",
        body_schema=None,
        parameter_schema={
            "properties": {"item_id": {"exclusiveMaximum": 3, "source": "path", "type": "integer"}},
            "required": ["item_id"],
            "type": "object",
        },
        file_params=None,
    )(path_params_integer_path_parameter_with_lt_constraint_success)
    return app


def path_params_integer_path_parameter_with_gt_constraint_success(
    item_id: int,
) -> Any:
    """Handler for GET /path/param-gt/{item_id}."""
    return Response(content={"item_id": 42}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_path_params_integer_path_parameter_with_gt_constraint_success() -> Spikard:
    """App factory for fixture: Integer path parameter with gt constraint - success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/path/param-gt/{item_id}",
        body_schema=None,
        parameter_schema={
            "properties": {"item_id": {"exclusiveMinimum": 3, "source": "path", "type": "integer"}},
            "required": ["item_id"],
            "type": "object",
        },
        file_params=None,
    )(path_params_integer_path_parameter_with_gt_constraint_success)
    return app


def path_params_28_duration_format_path_param_success(
    duration: str,
) -> Any:
    """Handler for GET /delays/{duration}."""
    return Response(content={"duration": "P1DT2H30M"}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_path_params_28_duration_format_path_param_success() -> Spikard:
    """App factory for fixture: 28_duration_format_path_param_success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/delays/{duration}",
        body_schema=None,
        parameter_schema={
            "properties": {"duration": {"format": "duration", "source": "path", "type": "string"}},
            "required": ["duration"],
            "type": "object",
        },
        file_params=None,
    )(path_params_28_duration_format_path_param_success)
    return app


def path_params_path_parameter_type_syntax_with_override(
    count: int,
) -> Any:
    """Handler for GET /type-syntax/items-count/{count:int}."""
    return Response(content={"count": "50"}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_path_params_path_parameter_type_syntax_with_override() -> Spikard:
    """App factory for fixture: Path parameter type syntax with override"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/type-syntax/items-count/{count:int}",
        body_schema=None,
        parameter_schema={
            "properties": {"count": {"maximum": 100, "minimum": 1, "source": "path", "type": "integer"}},
            "required": ["count"],
            "type": "object",
        },
        file_params=None,
    )(path_params_path_parameter_type_syntax_with_override)
    return app


def path_params_20_uuid_v3_path_param_success(
    id: UUID,
) -> Any:
    """Handler for GET /items/{id}."""
    return Response(
        content={"id": "e8b5a51d-11c8-3310-a6ab-367563f20686"},
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_path_params_20_uuid_v3_path_param_success() -> Spikard:
    """App factory for fixture: 20_uuid_v3_path_param_success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/items/{id}",
        body_schema=None,
        parameter_schema={
            "properties": {"id": {"format": "uuid", "source": "path", "type": "string", "uuidVersion": "3"}},
            "required": ["id"],
            "type": "object",
        },
        file_params=None,
    )(path_params_20_uuid_v3_path_param_success)
    return app


def path_params_integer_path_parameter_invalid_string(
    item_id: int,
) -> Any:
    """Handler for GET /path/int/{item_id}."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "input": "foobar",
                    "loc": ["path", "item_id"],
                    "msg": "Input should be a valid integer, unable to parse string as an integer",
                    "type": "int_parsing",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_path_params_integer_path_parameter_invalid_string() -> Spikard:
    """App factory for fixture: Integer path parameter - invalid string"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/path/int/{item_id}",
        body_schema=None,
        parameter_schema={
            "properties": {"item_id": {"source": "path", "type": "integer"}},
            "required": ["item_id"],
            "type": "object",
        },
        file_params=None,
    )(path_params_integer_path_parameter_invalid_string)
    return app


def path_params_30_string_minlength_path_success(
    username: str,
) -> Any:
    """Handler for GET /users/{username}."""
    return Response(content={"username": "alice"}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_path_params_30_string_minlength_path_success() -> Spikard:
    """App factory for fixture: 30_string_minlength_path_success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/users/{username}",
        body_schema=None,
        parameter_schema={
            "properties": {"username": {"minLength": 3, "source": "path", "type": "string"}},
            "required": ["username"],
            "type": "object",
        },
        file_params=None,
    )(path_params_30_string_minlength_path_success)
    return app


def path_params_integer_path_parameter_with_le_constraint_success(
    item_id: int,
) -> Any:
    """Handler for GET /path/param-le/{item_id}."""
    return Response(content={"item_id": 3}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_path_params_integer_path_parameter_with_le_constraint_success() -> Spikard:
    """App factory for fixture: Integer path parameter with le constraint - success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/path/param-le/{item_id}",
        body_schema=None,
        parameter_schema={
            "properties": {"item_id": {"maximum": 3, "source": "path", "type": "integer"}},
            "required": ["item_id"],
            "type": "object",
        },
        file_params=None,
    )(path_params_integer_path_parameter_with_le_constraint_success)
    return app


def path_params_path_parameter_type_syntax_invalid_uuid() -> Any:
    """Handler for GET /type-syntax/items/{id:uuid}."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "input": "not-a-uuid",
                    "loc": ["path", "id"],
                    "msg": "Input should be a valid UUID",
                    "type": "uuid_parsing",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_path_params_path_parameter_type_syntax_invalid_uuid() -> Spikard:
    """App factory for fixture: Path parameter type syntax - invalid UUID"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET", "/type-syntax/items/{id:uuid}", body_schema=None, parameter_schema=None, file_params=None
    )(path_params_path_parameter_type_syntax_invalid_uuid)
    return app


def path_params_path_type_parameter_file_path(
    file_path: str,
) -> Any:
    """Handler for GET /files/{file_path:path}."""
    return Response(
        content={"file_path": "home/johndoe/myfile.txt"}, status_code=200, headers={"Content-Type": "application/json"}
    )


def create_app_path_params_path_type_parameter_file_path() -> Spikard:
    """App factory for fixture: Path type parameter - file path"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/files/{file_path:path}",
        body_schema=None,
        parameter_schema={
            "properties": {"file_path": {"source": "path", "type": "string"}},
            "required": ["file_path"],
            "type": "object",
        },
        file_params=None,
    )(path_params_path_type_parameter_file_path)
    return app


def path_params_path_parameter_with_type_syntax_uuid() -> Any:
    """Handler for GET /type-syntax/items/{id:uuid}."""
    return Response(
        content={"id": "550e8400-e29b-41d4-a716-446655440000"},
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_path_params_path_parameter_with_type_syntax_uuid() -> Spikard:
    """App factory for fixture: Path parameter with type syntax - UUID"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET", "/type-syntax/items/{id:uuid}", body_schema=None, parameter_schema=None, file_params=None
    )(path_params_path_parameter_with_type_syntax_uuid)
    return app


def path_params_32_string_maxlength_path_failure(
    username: str,
) -> Any:
    """Handler for GET /users/{username}."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"actual_length": 42, "max_length": 20},
                    "loc": ["path", "username"],
                    "msg": "String length must not exceed 20",
                    "type": "validation_error",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_path_params_32_string_maxlength_path_failure() -> Spikard:
    """App factory for fixture: 32_string_maxlength_path_failure"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/users/{username}",
        body_schema=None,
        parameter_schema={
            "properties": {"username": {"maxLength": 20, "source": "path", "type": "string"}},
            "required": ["username"],
            "type": "object",
        },
        file_params=None,
    )(path_params_32_string_maxlength_path_failure)
    return app


def path_params_integer_path_parameter_success(
    item_id: int,
) -> Any:
    """Handler for GET /path/int/{item_id}."""
    return Response(content={"item_id": 42}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_path_params_integer_path_parameter_success() -> Spikard:
    """App factory for fixture: Integer path parameter - success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/path/int/{item_id}",
        body_schema=None,
        parameter_schema={
            "properties": {"item_id": {"source": "path", "type": "integer"}},
            "required": ["item_id"],
            "type": "object",
        },
        file_params=None,
    )(path_params_integer_path_parameter_success)
    return app


def path_params_34_string_pattern_path_failure(
    owner: str,
) -> Any:
    """Handler for GET /repos/{owner}."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"pattern": "^[a-zA-Z0-9-]+$", "value": "invalid@owner"},
                    "loc": ["path", "owner"],
                    "msg": "String does not match pattern",
                    "type": "validation_error",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_path_params_34_string_pattern_path_failure() -> Spikard:
    """App factory for fixture: 34_string_pattern_path_failure"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/repos/{owner}",
        body_schema=None,
        parameter_schema={
            "properties": {"owner": {"pattern": "^[a-zA-Z0-9-]+$", "source": "path", "type": "string"}},
            "required": ["owner"],
            "type": "object",
        },
        file_params=None,
    )(path_params_34_string_pattern_path_failure)
    return app


def path_params_21_uuid_v5_path_param_success(
    id: UUID,
) -> Any:
    """Handler for GET /items/{id}."""
    return Response(
        content={"id": "630eb68f-e0fa-5ecc-887a-7c7a62614681"},
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_path_params_21_uuid_v5_path_param_success() -> Spikard:
    """App factory for fixture: 21_uuid_v5_path_param_success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/items/{id}",
        body_schema=None,
        parameter_schema={
            "properties": {"id": {"format": "uuid", "source": "path", "type": "string", "uuidVersion": "5"}},
            "required": ["id"],
            "type": "object",
        },
        file_params=None,
    )(path_params_21_uuid_v5_path_param_success)
    return app


def path_params_string_path_parameter_with_max_length_failure(
    item_id: str,
) -> Any:
    """Handler for GET /path/param-maxlength/{item_id}."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"max_length": 3},
                    "input": "foobar",
                    "loc": ["path", "item_id"],
                    "msg": "String should have at most 3 characters",
                    "type": "string_too_long",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_path_params_string_path_parameter_with_max_length_failure() -> Spikard:
    """App factory for fixture: String path parameter with max_length - failure"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/path/param-maxlength/{item_id}",
        body_schema=None,
        parameter_schema={
            "properties": {"item_id": {"maxLength": 3, "source": "path", "type": "string"}},
            "required": ["item_id"],
            "type": "object",
        },
        file_params=None,
    )(path_params_string_path_parameter_with_max_length_failure)
    return app


def path_params_string_path_parameter_with_min_length_failure(
    item_id: str,
) -> Any:
    """Handler for GET /path/param-minlength/{item_id}."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"min_length": 3},
                    "input": "fo",
                    "loc": ["path", "item_id"],
                    "msg": "String should have at least 3 characters",
                    "type": "string_too_short",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_path_params_string_path_parameter_with_min_length_failure() -> Spikard:
    """App factory for fixture: String path parameter with min_length - failure"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/path/param-minlength/{item_id}",
        body_schema=None,
        parameter_schema={
            "properties": {"item_id": {"minLength": 3, "source": "path", "type": "string"}},
            "required": ["item_id"],
            "type": "object",
        },
        file_params=None,
    )(path_params_string_path_parameter_with_min_length_failure)
    return app


def path_params_multiple_path_parameters_success(
    order_id: UUID,
    service_id: int,
    user_id: str,
    version: float,
) -> Any:
    """Handler for GET /{version}/{service_id}/{user_id}/{order_id}."""
    return Response(
        content={"order_id": "c892496f-b1fd-4b91-bdb8-b46f92df1716", "service_id": 1, "user_id": "abc", "version": 1.0},
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_path_params_multiple_path_parameters_success() -> Spikard:
    """App factory for fixture: Multiple path parameters - success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/{version}/{service_id}/{user_id}/{order_id}",
        body_schema=None,
        parameter_schema={
            "properties": {
                "order_id": {"format": "uuid", "source": "path", "type": "string"},
                "service_id": {"source": "path", "type": "integer"},
                "user_id": {"source": "path", "type": "string"},
                "version": {"source": "path", "type": "number"},
            },
            "required": ["order_id", "service_id", "user_id", "version"],
            "type": "object",
        },
        file_params=None,
    )(path_params_multiple_path_parameters_success)
    return app


def path_params_date_path_parameter_success(
    date_param: date,
) -> Any:
    """Handler for GET /date/{date_param}."""
    return Response(content={"date_param": "2023-07-15"}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_path_params_date_path_parameter_success() -> Spikard:
    """App factory for fixture: Date path parameter - success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/date/{date_param}",
        body_schema=None,
        parameter_schema={
            "properties": {"date_param": {"format": "date", "source": "path", "type": "string"}},
            "required": ["date_param"],
            "type": "object",
        },
        file_params=None,
    )(path_params_date_path_parameter_success)
    return app


def path_params_integer_path_parameter_with_gt_constraint_failure(
    item_id: int,
) -> Any:
    """Handler for GET /path/param-gt/{item_id}."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"gt": 3},
                    "input": 2,
                    "loc": ["path", "item_id"],
                    "msg": "Input should be greater than 3",
                    "type": "greater_than",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_path_params_integer_path_parameter_with_gt_constraint_failure() -> Spikard:
    """App factory for fixture: Integer path parameter with gt constraint - failure"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/path/param-gt/{item_id}",
        body_schema=None,
        parameter_schema={
            "properties": {"item_id": {"exclusiveMinimum": 3, "source": "path", "type": "integer"}},
            "required": ["item_id"],
            "type": "object",
        },
        file_params=None,
    )(path_params_integer_path_parameter_with_gt_constraint_failure)
    return app


def path_params_24_date_format_path_param_success(
    date: date,
) -> Any:
    """Handler for GET /events/{date}."""
    return Response(content={"date": "2025-10-30"}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_path_params_24_date_format_path_param_success() -> Spikard:
    """App factory for fixture: 24_date_format_path_param_success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/events/{date}",
        body_schema=None,
        parameter_schema={
            "properties": {"date": {"format": "date", "source": "path", "type": "string"}},
            "required": ["date"],
            "type": "object",
        },
        file_params=None,
    )(path_params_24_date_format_path_param_success)
    return app


def path_params_float_path_parameter_success(
    item_id: float,
) -> Any:
    """Handler for GET /path/float/{item_id}."""
    return Response(content={"item_id": 42.5}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_path_params_float_path_parameter_success() -> Spikard:
    """App factory for fixture: Float path parameter - success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/path/float/{item_id}",
        body_schema=None,
        parameter_schema={
            "properties": {"item_id": {"source": "path", "type": "number"}},
            "required": ["item_id"],
            "type": "object",
        },
        file_params=None,
    )(path_params_float_path_parameter_success)
    return app


def path_params_path_parameter_with_type_syntax_integer() -> Any:
    """Handler for GET /type-syntax/users/{user_id:int}."""
    return Response(content={"user_id": "42"}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_path_params_path_parameter_with_type_syntax_integer() -> Spikard:
    """App factory for fixture: Path parameter with type syntax - integer"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET", "/type-syntax/users/{user_id:int}", body_schema=None, parameter_schema=None, file_params=None
    )(path_params_path_parameter_with_type_syntax_integer)
    return app


def path_params_string_path_parameter_success(
    item_id: str,
) -> Any:
    """Handler for GET /path/str/{item_id}."""
    return Response(content={"item_id": "foobar"}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_path_params_string_path_parameter_success() -> Spikard:
    """App factory for fixture: String path parameter - success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/path/str/{item_id}",
        body_schema=None,
        parameter_schema={
            "properties": {"item_id": {"source": "path", "type": "string"}},
            "required": ["item_id"],
            "type": "object",
        },
        file_params=None,
    )(path_params_string_path_parameter_success)
    return app


def path_params_uuid_path_parameter_success(
    item_id: UUID,
) -> Any:
    """Handler for GET /items/{item_id}."""
    return Response(
        content={"item_id": "ec38df32-ceda-4cfa-9b4a-1aeb94ad551a"},
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_path_params_uuid_path_parameter_success() -> Spikard:
    """App factory for fixture: UUID path parameter - success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/items/{item_id}",
        body_schema=None,
        parameter_schema={
            "properties": {"item_id": {"format": "uuid", "source": "path", "type": "string"}},
            "required": ["item_id"],
            "type": "object",
        },
        file_params=None,
    )(path_params_uuid_path_parameter_success)
    return app


def path_params_integer_path_parameter_with_ge_constraint_success(
    item_id: int,
) -> Any:
    """Handler for GET /path/param-ge/{item_id}."""
    return Response(content={"item_id": 3}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_path_params_integer_path_parameter_with_ge_constraint_success() -> Spikard:
    """App factory for fixture: Integer path parameter with ge constraint - success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/path/param-ge/{item_id}",
        body_schema=None,
        parameter_schema={
            "properties": {"item_id": {"minimum": 3, "source": "path", "type": "integer"}},
            "required": ["item_id"],
            "type": "object",
        },
        file_params=None,
    )(path_params_integer_path_parameter_with_ge_constraint_success)
    return app


def path_params_enum_path_parameter_success(
    model_name: str,
) -> Any:
    """Handler for GET /models/{model_name}."""
    return Response(content={"model_name": "alexnet"}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_path_params_enum_path_parameter_success() -> Spikard:
    """App factory for fixture: Enum path parameter - success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/models/{model_name}",
        body_schema=None,
        parameter_schema={
            "properties": {"model_name": {"enum": ["alexnet", "lenet", "resnet"], "source": "path", "type": "string"}},
            "required": ["model_name"],
            "type": "object",
        },
        file_params=None,
    )(path_params_enum_path_parameter_success)
    return app


def path_params_boolean_path_parameter_numeric_1(
    item_id: bool,
) -> Any:
    """Handler for GET /path/bool/{item_id}."""
    return Response(content={"item_id": True}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_path_params_boolean_path_parameter_numeric_1() -> Spikard:
    """App factory for fixture: Boolean path parameter - numeric 1"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/path/bool/{item_id}",
        body_schema=None,
        parameter_schema={
            "properties": {"item_id": {"source": "path", "type": "boolean"}},
            "required": ["item_id"],
            "type": "object",
        },
        file_params=None,
    )(path_params_boolean_path_parameter_numeric_1)
    return app


def request_id_request_id_header_is_preserved() -> Any:
    """Handler for GET /request-id/preserved."""
    return Response(
        content={"echo": "trace-123", "status": "preserved"},
        status_code=200,
        headers={"Content-Type": "application/json", "x-request-id": "trace-123"},
    )


def create_app_request_id_request_id_header_is_preserved() -> Spikard:
    """App factory for fixture: Request ID header is preserved"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route("GET", "/request-id/preserved", body_schema=None, parameter_schema=None, file_params=None)(
        request_id_request_id_header_is_preserved
    )
    return app


def request_id_request_id_middleware_can_be_disabled() -> Any:
    """Handler for GET /request-id/disabled."""
    return Response(content={"status": "no-request-id"}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_request_id_request_id_middleware_can_be_disabled() -> Spikard:
    """App factory for fixture: Request ID middleware can be disabled"""
    config = ServerConfig(enable_request_id=False)
    app = Spikard(config=config)
    # Register handler with this app instance
    app.register_route("GET", "/request-id/disabled", body_schema=None, parameter_schema=None, file_params=None)(
        request_id_request_id_middleware_can_be_disabled
    )
    return app


def request_id_request_id_is_generated_when_not_provided() -> Any:
    """Handler for GET /request-id/generated."""
    return Response(
        content={"status": "generated"},
        status_code=200,
        headers={"x-request-id": "00000000-0000-4000-8000-000000000000", "Content-Type": "application/json"},
    )


def create_app_request_id_request_id_is_generated_when_not_provided() -> Spikard:
    """App factory for fixture: Request ID is generated when not provided"""
    config = ServerConfig(enable_request_id=True)
    app = Spikard(config=config)
    # Register handler with this app instance
    app.register_route("GET", "/request-id/generated", body_schema=None, parameter_schema=None, file_params=None)(
        request_id_request_id_is_generated_when_not_provided
    )
    return app


def headers_header_regex_validation_success(
    x_request_id: str,
) -> Any:
    """Handler for GET /headers/pattern."""
    return Response(content={"x_request_id": "12345"}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_headers_header_regex_validation_success() -> Spikard:
    """App factory for fixture: Header regex validation - success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/headers/pattern",
        body_schema=None,
        parameter_schema={
            "properties": {
                "x_request_id": {"annotation": "str", "pattern": "^[0-9]{3,}$", "source": "header", "type": "string"}
            },
            "required": ["x_request_id"],
            "type": "object",
        },
        file_params=None,
    )(headers_header_regex_validation_success)
    return app


def headers_33_api_key_header_valid(
    x_api_key: str,
) -> Any:
    """Handler for GET /api/data."""
    # Echo back parameters for testing
    result: dict[str, Any] = {}
    if x_api_key is not None:
        result["x_api_key"] = x_api_key
    return result


def create_app_headers_33_api_key_header_valid() -> Spikard:
    """App factory for fixture: 33_api_key_header_valid"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/api/data",
        body_schema=None,
        parameter_schema={
            "properties": {"x_api_key": {"pattern": "^[a-f0-9]{32}$", "source": "header", "type": "string"}},
            "required": ["x_api_key"],
            "type": "object",
        },
        file_params=None,
    )(headers_33_api_key_header_valid)
    return app


def headers_content_type_header_application_json(
    content_type: str,
) -> Any:
    """Handler for GET /headers/content-type."""
    return Response(
        content={"content_type": "application/json"}, status_code=200, headers={"Content-Type": "application/json"}
    )


def create_app_headers_content_type_header_application_json() -> Spikard:
    """App factory for fixture: Content-Type header - application/json"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/headers/content-type",
        body_schema=None,
        parameter_schema={
            "properties": {"content_type": {"annotation": "str", "source": "header", "type": "string"}},
            "required": ["content_type"],
            "type": "object",
        },
        file_params=None,
    )(headers_content_type_header_application_json)
    return app


def headers_accept_language_header(
    accept_language: str,
) -> Any:
    """Handler for GET /headers/accept-language."""
    return Response(
        content={"accept_language": "en-US,en;q=0.9"}, status_code=200, headers={"Content-Type": "application/json"}
    )


def create_app_headers_accept_language_header() -> Spikard:
    """App factory for fixture: Accept-Language header"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/headers/accept-language",
        body_schema=None,
        parameter_schema={
            "properties": {"accept_language": {"annotation": "str", "source": "header", "type": "string"}},
            "required": ["accept_language"],
            "type": "object",
        },
        file_params=None,
    )(headers_accept_language_header)
    return app


def headers_x_api_key_required_header_success(
    key: str,
) -> Any:
    """Handler for GET /users/me."""
    return Response(content={"username": "secret"}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_headers_x_api_key_required_header_success() -> Spikard:
    """App factory for fixture: X-API-Key required header - success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/users/me",
        body_schema=None,
        parameter_schema={
            "properties": {"key": {"annotation": "str", "source": "header", "type": "string"}},
            "required": ["key"],
            "type": "object",
        },
        file_params=None,
    )(headers_x_api_key_required_header_success)
    return app


def headers_header_validation_max_length_constraint_fail(
    x_session_id: str,
) -> Any:
    """Handler for GET /headers/max-length."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"max_length": 20},
                    "input": "this_is_way_too_long_for_validation",
                    "loc": ["headers", "x-session-id"],
                    "msg": "String should have at most 20 characters",
                    "type": "string_too_long",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_headers_header_validation_max_length_constraint_fail() -> Spikard:
    """App factory for fixture: Header validation - max_length constraint fail"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/headers/max-length",
        body_schema=None,
        parameter_schema={
            "properties": {
                "x_session_id": {"annotation": "str", "maxLength": 20, "source": "header", "type": "string"}
            },
            "required": ["x_session_id"],
            "type": "object",
        },
        file_params=None,
    )(headers_header_validation_max_length_constraint_fail)
    return app


def headers_x_api_key_required_header_missing(
    x_api_key: str,
) -> Any:
    """Handler for GET /users/me."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [{"input": None, "loc": ["headers", "x-api-key"], "msg": "Field required", "type": "missing"}],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_headers_x_api_key_required_header_missing() -> Spikard:
    """App factory for fixture: X-API-Key required header - missing"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/users/me",
        body_schema=None,
        parameter_schema={
            "properties": {"x_api_key": {"annotation": "str", "source": "header", "type": "string"}},
            "required": ["x_api_key"],
            "type": "object",
        },
        file_params=None,
    )(headers_x_api_key_required_header_missing)
    return app


def headers_origin_header(
    origin: str,
) -> Any:
    """Handler for GET /headers/origin."""
    return Response(
        content={"origin": "https://example.com"}, status_code=200, headers={"Content-Type": "application/json"}
    )


def create_app_headers_origin_header() -> Spikard:
    """App factory for fixture: Origin header"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/headers/origin",
        body_schema=None,
        parameter_schema={
            "properties": {"origin": {"annotation": "str", "source": "header", "type": "string"}},
            "required": ["origin"],
            "type": "object",
        },
        file_params=None,
    )(headers_origin_header)
    return app


def headers_user_agent_header_default_value(
    user_agent: str | None = None,
) -> Any:
    """Handler for GET /items/."""
    return Response(content={"User-Agent": "testclient"}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_headers_user_agent_header_default_value() -> Spikard:
    """App factory for fixture: User-Agent header - default value"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/items/",
        body_schema=None,
        parameter_schema={
            "properties": {
                "user_agent": {"annotation": "str", "default": "testclient", "source": "header", "type": "string"}
            },
            "required": [],
            "type": "object",
        },
        file_params=None,
    )(headers_user_agent_header_default_value)
    return app


def headers_32_bearer_token_missing_prefix(
    authorization: str,
) -> Any:
    """Handler for GET /protected."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {
                        "pattern": "^Bearer [A-Za-z0-9-._~+/]+=*$",
                        "value": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9",
                    },
                    "loc": ["headers", "authorization"],
                    "msg": "Invalid Bearer token format",
                    "type": "validation_error",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_headers_32_bearer_token_missing_prefix() -> Spikard:
    """App factory for fixture: 32_bearer_token_missing_prefix"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/protected",
        body_schema=None,
        parameter_schema={
            "properties": {
                "authorization": {"pattern": "^Bearer [A-Za-z0-9-._~+/]+=*$", "source": "header", "type": "string"}
            },
            "required": ["authorization"],
            "type": "object",
        },
        file_params=None,
    )(headers_32_bearer_token_missing_prefix)
    return app


def headers_optional_header_with_none_default_missing(
    strange_header: str | None = None,
) -> Any:
    """Handler for GET /items/."""
    return Response(content={"strange_header": None}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_headers_optional_header_with_none_default_missing() -> Spikard:
    """App factory for fixture: Optional header with None default - missing"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/items/",
        body_schema=None,
        parameter_schema={
            "properties": {
                "strange_header": {"annotation": "str", "default": None, "source": "header", "type": "string"}
            },
            "required": [],
            "type": "object",
        },
        file_params=None,
    )(headers_optional_header_with_none_default_missing)
    return app


def headers_header_regex_validation_fail(
    x_request_id: str,
) -> Any:
    """Handler for GET /headers/pattern."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"pattern": "^[0-9]{3,}$"},
                    "input": "invalid-format",
                    "loc": ["headers", "x-request-id"],
                    "msg": "String should match pattern '^[0-9]{3,}$'",
                    "type": "string_pattern_mismatch",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_headers_header_regex_validation_fail() -> Spikard:
    """App factory for fixture: Header regex validation - fail"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/headers/pattern",
        body_schema=None,
        parameter_schema={
            "properties": {
                "x_request_id": {"annotation": "str", "pattern": "^[0-9]{3,}$", "source": "header", "type": "string"}
            },
            "required": ["x_request_id"],
            "type": "object",
        },
        file_params=None,
    )(headers_header_regex_validation_fail)
    return app


def headers_31_bearer_token_format_invalid(
    authorization: str,
) -> Any:
    """Handler for GET /protected."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"pattern": "^Bearer [A-Za-z0-9-._~+/]+=*$", "value": "Bearer invalid token with spaces"},
                    "loc": ["headers", "authorization"],
                    "msg": "Invalid Bearer token format",
                    "type": "validation_error",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_headers_31_bearer_token_format_invalid() -> Spikard:
    """App factory for fixture: 31_bearer_token_format_invalid"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/protected",
        body_schema=None,
        parameter_schema={
            "properties": {
                "authorization": {"pattern": "^Bearer [A-Za-z0-9-._~+/]+=*$", "source": "header", "type": "string"}
            },
            "required": ["authorization"],
            "type": "object",
        },
        file_params=None,
    )(headers_31_bearer_token_format_invalid)
    return app


def headers_x_api_key_optional_header_success(
    key: str | None = None,
) -> Any:
    """Handler for GET /users/me."""
    return Response(content={"msg": "Hello secret"}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_headers_x_api_key_optional_header_success() -> Spikard:
    """App factory for fixture: X-API-Key optional header - success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/users/me",
        body_schema=None,
        parameter_schema={
            "properties": {"key": {"annotation": "str", "source": "header", "type": "string"}},
            "required": [],
            "type": "object",
        },
        file_params=None,
    )(headers_x_api_key_optional_header_success)
    return app


def headers_authorization_header_success(
    authorization: str,
) -> Any:
    """Handler for GET /users/me."""
    return Response(
        content={"credentials": "foobar", "scheme": "Digest"},
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_headers_authorization_header_success() -> Spikard:
    """App factory for fixture: Authorization header - success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/users/me",
        body_schema=None,
        parameter_schema={
            "properties": {"authorization": {"annotation": "str", "source": "header", "type": "string"}},
            "required": ["authorization"],
            "type": "object",
        },
        file_params=None,
    )(headers_authorization_header_success)
    return app


def headers_30_bearer_token_format_valid(
    authorization: str,
) -> Any:
    """Handler for GET /protected."""
    # Echo back parameters for testing
    result: dict[str, Any] = {}
    if authorization is not None:
        result["authorization"] = authorization
    return result


def create_app_headers_30_bearer_token_format_valid() -> Spikard:
    """App factory for fixture: 30_bearer_token_format_valid"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/protected",
        body_schema=None,
        parameter_schema={
            "properties": {
                "authorization": {"pattern": "^Bearer [A-Za-z0-9-._~+/]+=*$", "source": "header", "type": "string"}
            },
            "required": ["authorization"],
            "type": "object",
        },
        file_params=None,
    )(headers_30_bearer_token_format_valid)
    return app


def headers_authorization_header_missing(
    authorization: str,
) -> Any:
    """Handler for GET /users/me."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {"input": None, "loc": ["headers", "authorization"], "msg": "Field required", "type": "missing"}
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_headers_authorization_header_missing() -> Spikard:
    """App factory for fixture: Authorization header - missing"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/users/me",
        body_schema=None,
        parameter_schema={
            "properties": {"authorization": {"annotation": "str", "source": "header", "type": "string"}},
            "required": ["authorization"],
            "type": "object",
        },
        file_params=None,
    )(headers_authorization_header_missing)
    return app


def headers_accept_header_json(
    accept: str,
) -> Any:
    """Handler for GET /headers/accept."""
    return Response(
        content={"accept": "application/json"}, status_code=200, headers={"Content-Type": "application/json"}
    )


def create_app_headers_accept_header_json() -> Spikard:
    """App factory for fixture: Accept header - JSON"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/headers/accept",
        body_schema=None,
        parameter_schema={
            "properties": {"accept": {"annotation": "str", "source": "header", "type": "string"}},
            "required": ["accept"],
            "type": "object",
        },
        file_params=None,
    )(headers_accept_header_json)
    return app


def headers_accept_encoding_header(
    accept_encoding: str,
) -> Any:
    """Handler for GET /headers/accept-encoding."""
    return Response(
        content={"accept_encoding": "gzip, deflate, br"}, status_code=200, headers={"Content-Type": "application/json"}
    )


def create_app_headers_accept_encoding_header() -> Spikard:
    """App factory for fixture: Accept-Encoding header"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/headers/accept-encoding",
        body_schema=None,
        parameter_schema={
            "properties": {"accept_encoding": {"annotation": "str", "source": "header", "type": "string"}},
            "required": ["accept_encoding"],
            "type": "object",
        },
        file_params=None,
    )(headers_accept_encoding_header)
    return app


def headers_authorization_header_wrong_scheme(
    authorization: str,
) -> Any:
    """Handler for GET /users/me."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "input": "Other invalidauthorization",
                    "loc": ["headers", "authorization"],
                    "msg": "String should match pattern '^Digest .+'",
                    "type": "string_pattern_mismatch",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_headers_authorization_header_wrong_scheme() -> Spikard:
    """App factory for fixture: Authorization header - wrong scheme"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/users/me",
        body_schema=None,
        parameter_schema={
            "properties": {
                "authorization": {"annotation": "str", "pattern": "^Digest .+", "source": "header", "type": "string"}
            },
            "required": ["authorization"],
            "type": "object",
        },
        file_params=None,
    )(headers_authorization_header_wrong_scheme)
    return app


def headers_header_validation_min_length_constraint(
    x_token: str,
) -> Any:
    """Handler for GET /headers/validated."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"min_length": 3},
                    "input": "ab",
                    "loc": ["headers", "x-token"],
                    "msg": "String should have at least 3 characters",
                    "type": "string_too_short",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_headers_header_validation_min_length_constraint() -> Spikard:
    """App factory for fixture: Header validation - min_length constraint"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/headers/validated",
        body_schema=None,
        parameter_schema={
            "properties": {"x_token": {"annotation": "str", "minLength": 3, "source": "header", "type": "string"}},
            "required": ["x_token"],
            "type": "object",
        },
        file_params=None,
    )(headers_header_validation_min_length_constraint)
    return app


def headers_basic_authentication_success(
    authorization: str,
) -> Any:
    """Handler for GET /headers/basic-auth."""
    return Response(
        content={"password": "password", "username": "username"},
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_headers_basic_authentication_success() -> Spikard:
    """App factory for fixture: Basic authentication - success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/headers/basic-auth",
        body_schema=None,
        parameter_schema={
            "properties": {"authorization": {"annotation": "str", "source": "header", "type": "string"}},
            "required": ["authorization"],
            "type": "object",
        },
        file_params=None,
    )(headers_basic_authentication_success)
    return app


def headers_bearer_token_authentication_missing(
    authorization: str,
) -> Any:
    """Handler for GET /headers/bearer-auth."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {"input": None, "loc": ["headers", "authorization"], "msg": "Field required", "type": "missing"}
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_headers_bearer_token_authentication_missing() -> Spikard:
    """App factory for fixture: Bearer token authentication - missing"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/headers/bearer-auth",
        body_schema=None,
        parameter_schema={
            "properties": {
                "authorization": {"annotation": "str", "pattern": "^Bearer .+", "source": "header", "type": "string"}
            },
            "required": ["authorization"],
            "type": "object",
        },
        file_params=None,
    )(headers_bearer_token_authentication_missing)
    return app


def headers_x_api_key_optional_header_missing(
    key: str | None = None,
) -> Any:
    """Handler for GET /users/me."""
    return Response(content={"msg": "Hello World"}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_headers_x_api_key_optional_header_missing() -> Spikard:
    """App factory for fixture: X-API-Key optional header - missing"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/users/me",
        body_schema=None,
        parameter_schema={
            "properties": {"key": {"annotation": "str", "source": "header", "type": "string"}},
            "required": [],
            "type": "object",
        },
        file_params=None,
    )(headers_x_api_key_optional_header_missing)
    return app


def headers_multiple_header_values_x_token(
    x_token: str,
) -> Any:
    """Handler for GET /items/."""
    return Response(
        content={"X-Token values": ["foo", "bar"]}, status_code=200, headers={"Content-Type": "application/json"}
    )


def create_app_headers_multiple_header_values_x_token() -> Spikard:
    """App factory for fixture: Multiple header values - X-Token"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/items/",
        body_schema=None,
        parameter_schema={
            "properties": {"x_token": {"annotation": "str", "source": "header", "type": "string"}},
            "required": ["x_token"],
            "type": "object",
        },
        file_params=None,
    )(headers_multiple_header_values_x_token)
    return app


def headers_multiple_custom_headers(
    x_client_version: str,
    x_request_id: str,
    x_trace_id: str,
) -> Any:
    """Handler for GET /headers/multiple."""
    return Response(
        content={"x_client_version": "1.2.3", "x_request_id": "req-12345", "x_trace_id": "trace-abc"},
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_headers_multiple_custom_headers() -> Spikard:
    """App factory for fixture: Multiple custom headers"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/headers/multiple",
        body_schema=None,
        parameter_schema={
            "properties": {
                "x_client_version": {"annotation": "str", "source": "header", "type": "string"},
                "x_request_id": {"annotation": "str", "source": "header", "type": "string"},
                "x_trace_id": {"annotation": "str", "source": "header", "type": "string"},
            },
            "required": ["x_client_version", "x_request_id", "x_trace_id"],
            "type": "object",
        },
        file_params=None,
    )(headers_multiple_custom_headers)
    return app


def headers_34_api_key_header_invalid(
    x_api_key: str,
) -> Any:
    """Handler for GET /api/data."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"pattern": "^[a-f0-9]{32}$", "value": "invalid-key"},
                    "loc": ["headers", "x-api-key"],
                    "msg": "Invalid API key format",
                    "type": "validation_error",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_headers_34_api_key_header_invalid() -> Spikard:
    """App factory for fixture: 34_api_key_header_invalid"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/api/data",
        body_schema=None,
        parameter_schema={
            "properties": {"x_api_key": {"pattern": "^[a-f0-9]{32}$", "source": "header", "type": "string"}},
            "required": ["x_api_key"],
            "type": "object",
        },
        file_params=None,
    )(headers_34_api_key_header_invalid)
    return app


def headers_bearer_token_authentication_success(
    authorization: str,
) -> Any:
    """Handler for GET /headers/bearer-auth."""
    return Response(content={"token": "valid_token_123"}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_headers_bearer_token_authentication_success() -> Spikard:
    """App factory for fixture: Bearer token authentication - success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/headers/bearer-auth",
        body_schema=None,
        parameter_schema={
            "properties": {"authorization": {"annotation": "str", "source": "header", "type": "string"}},
            "required": ["authorization"],
            "type": "object",
        },
        file_params=None,
    )(headers_bearer_token_authentication_success)
    return app


def headers_host_header(
    host: str,
) -> Any:
    """Handler for GET /headers/host."""
    return Response(content={"host": "example.com:8080"}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_headers_host_header() -> Spikard:
    """App factory for fixture: Host header"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/headers/host",
        body_schema=None,
        parameter_schema={
            "properties": {"host": {"annotation": "str", "source": "header", "type": "string"}},
            "required": ["host"],
            "type": "object",
        },
        file_params=None,
    )(headers_host_header)
    return app


def headers_referer_header(
    referer: str,
) -> Any:
    """Handler for GET /headers/referer."""
    return Response(
        content={"referer": "https://example.com/page"}, status_code=200, headers={"Content-Type": "application/json"}
    )


def create_app_headers_referer_header() -> Spikard:
    """App factory for fixture: Referer header"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/headers/referer",
        body_schema=None,
        parameter_schema={
            "properties": {"referer": {"annotation": "str", "source": "header", "type": "string"}},
            "required": ["referer"],
            "type": "object",
        },
        file_params=None,
    )(headers_referer_header)
    return app


def headers_header_with_underscore_conversion_explicit(
    x_token: str,
) -> Any:
    """Handler for GET /headers/underscore."""
    return Response(content={"x_token": "secret123"}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_headers_header_with_underscore_conversion_explicit() -> Spikard:
    """App factory for fixture: Header with underscore conversion - explicit"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/headers/underscore",
        body_schema=None,
        parameter_schema={
            "properties": {"x_token": {"annotation": "str", "source": "header", "type": "string"}},
            "required": ["x_token"],
            "type": "object",
        },
        file_params=None,
    )(headers_header_with_underscore_conversion_explicit)
    return app


class HeadersHeaderCaseInsensitivityAccessBody(TypedDict):
    """Request body type (TypedDict - runtime is dict)."""

    test: str


def headers_header_case_insensitivity_access(
    body: HeadersHeaderCaseInsensitivityAccessBody,
) -> Any:
    """Handler for POST /echo."""
    return Response(
        content={
            "content_type_lower": "application/json",
            "content_type_mixed": "application/json",
            "content_type_upper": "application/json",
        },
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_headers_header_case_insensitivity_access() -> Spikard:
    """App factory for fixture: Header case insensitivity - access"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/echo",
        body_schema={
            "additionalProperties": False,
            "properties": {"test": {"type": "string"}},
            "required": ["test"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(headers_header_case_insensitivity_access)
    return app


def headers_user_agent_header_custom_value(
    user_agent: str,
) -> Any:
    """Handler for GET /items/."""
    return Response(
        content={"User-Agent": "Mozilla/5.0 Custom Browser"},
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_headers_user_agent_header_custom_value() -> Spikard:
    """App factory for fixture: User-Agent header - custom value"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/items/",
        body_schema=None,
        parameter_schema={
            "properties": {"user_agent": {"annotation": "str", "source": "header", "type": "string"}},
            "required": ["user_agent"],
            "type": "object",
        },
        file_params=None,
    )(headers_user_agent_header_custom_value)
    return app


def status_codes_408_request_timeout(
    body: dict[str, Any],
) -> Any:
    """Handler for POST /slow-endpoint."""
    return Response(
        content={"detail": "Request timeout"},
        status_code=408,
        headers={"Connection": "close", "Content-Type": "application/json"},
    )


def create_app_status_codes_408_request_timeout() -> Spikard:
    """App factory for fixture: 408 Request Timeout"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/slow-endpoint",
        body_schema={
            "additionalProperties": False,
            "properties": {"data": {"type": "string"}},
            "required": ["data"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(status_codes_408_request_timeout)
    return app


def status_codes_404_not_found_resource_not_found(
    code: str,
) -> Any:
    """Handler for GET /status-test/{code}."""
    return Response(content={"detail": "Item not found"}, status_code=404, headers={"Content-Type": "application/json"})


def create_app_status_codes_404_not_found_resource_not_found() -> Spikard:
    """App factory for fixture: 404 Not Found - Resource not found"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/status-test/{code}",
        body_schema=None,
        parameter_schema={
            "properties": {"code": {"source": "path", "type": "string"}},
            "required": ["code"],
            "type": "object",
        },
        file_params=None,
    )(status_codes_404_not_found_resource_not_found)
    return app


def status_codes_503_service_unavailable_server_overload() -> Any:
    """Handler for GET /health."""
    return Response(
        content={"detail": "Service temporarily unavailable"},
        status_code=503,
        headers={"retry-after": "120", "Content-Type": "application/json"},
    )


def create_app_status_codes_503_service_unavailable_server_overload() -> Spikard:
    """App factory for fixture: 503 Service Unavailable - Server overload"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route("GET", "/health", body_schema=None, parameter_schema=None, file_params=None)(
        status_codes_503_service_unavailable_server_overload
    )
    return app


class StatusCodes422UnprocessableEntityValidationErrorBody(NamedTuple):
    """Request body NamedTuple (immutable)."""

    name: str
    price: str


def status_codes_422_unprocessable_entity_validation_error(
    body: StatusCodes422UnprocessableEntityValidationErrorBody,
) -> Any:
    """Handler for POST /items/."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [{"input": "", "loc": ["body", "name"], "msg": "Field required", "type": "missing"}],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_status_codes_422_unprocessable_entity_validation_error() -> Spikard:
    """App factory for fixture: 422 Unprocessable Entity - Validation error"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/items/",
        body_schema={
            "additionalProperties": False,
            "properties": {"name": {"type": "string"}, "price": {"type": "string"}},
            "required": ["price", "name"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(status_codes_422_unprocessable_entity_validation_error)
    return app


def status_codes_302_found_temporary_redirect() -> Any:
    """Handler for GET /temp-redirect."""
    return Response(status_code=302, headers={"location": "/target-path"})


def create_app_status_codes_302_found_temporary_redirect() -> Spikard:
    """App factory for fixture: 302 Found - Temporary redirect"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route("GET", "/temp-redirect", body_schema=None, parameter_schema=None, file_params=None)(
        status_codes_302_found_temporary_redirect
    )
    return app


def status_codes_304_not_modified_cached_content_valid(
    code: str,
    if_none_match: str | None = None,
) -> Any:
    """Handler for GET /status-test/{code}."""
    return Response(status_code=304)


def create_app_status_codes_304_not_modified_cached_content_valid() -> Spikard:
    """App factory for fixture: 304 Not Modified - Cached content valid"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/status-test/{code}",
        body_schema=None,
        parameter_schema={
            "properties": {
                "code": {"source": "path", "type": "string"},
                "if_none_match": {"source": "header", "type": "string"},
            },
            "required": ["code"],
            "type": "object",
        },
        file_params=None,
    )(status_codes_304_not_modified_cached_content_valid)
    return app


def status_codes_400_bad_request_invalid_request(
    body: str,
) -> Any:
    """Handler for POST /items/."""
    return Response(
        content={"detail": "Invalid request format"}, status_code=400, headers={"Content-Type": "application/json"}
    )


def create_app_status_codes_400_bad_request_invalid_request() -> Spikard:
    """App factory for fixture: 400 Bad Request - Invalid request"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route("POST", "/items/", body_schema={"type": "string"}, parameter_schema=None, file_params=None)(
        status_codes_400_bad_request_invalid_request
    )
    return app


def status_codes_22_501_not_implemented() -> Any:
    """Handler for GET /data."""
    return Response(status_code=405)


def create_app_status_codes_22_501_not_implemented() -> Spikard:
    """App factory for fixture: 22_501_not_implemented"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route("GET", "/data", body_schema=None, parameter_schema=None, file_params=None)(
        status_codes_22_501_not_implemented
    )
    return app


def status_codes_204_no_content_success_with_no_body(
    code: str,
) -> Any:
    """Handler for DELETE /status-test/{code}."""
    return Response(status_code=204)


def create_app_status_codes_204_no_content_success_with_no_body() -> Spikard:
    """App factory for fixture: 204 No Content - Success with no body"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "DELETE",
        "/status-test/{code}",
        body_schema=None,
        parameter_schema={
            "properties": {"code": {"source": "path", "type": "string"}},
            "required": ["code"],
            "type": "object",
        },
        file_params=None,
    )(status_codes_204_no_content_success_with_no_body)
    return app


def status_codes_301_moved_permanently_permanent_redirect() -> Any:
    """Handler for GET /old-path."""
    return Response(status_code=301, headers={"location": "/new-path"})


def create_app_status_codes_301_moved_permanently_permanent_redirect() -> Spikard:
    """App factory for fixture: 301 Moved Permanently - Permanent redirect"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route("GET", "/old-path", body_schema=None, parameter_schema=None, file_params=None)(
        status_codes_301_moved_permanently_permanent_redirect
    )
    return app


class StatusCodes201CreatedResourceCreatedBody(msgspec.Struct):
    """Request body msgspec.Struct (fast typed)."""

    name: str


def status_codes_201_created_resource_created(
    body: StatusCodes201CreatedResourceCreatedBody,
) -> Any:
    """Handler for POST /items/."""
    return Response(
        content={"id": 1, "name": "New Item"}, status_code=201, headers={"Content-Type": "application/json"}
    )


def create_app_status_codes_201_created_resource_created() -> Spikard:
    """App factory for fixture: 201 Created - Resource created"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/items/",
        body_schema={
            "additionalProperties": False,
            "properties": {"name": {"type": "string"}},
            "required": ["name"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(status_codes_201_created_resource_created)
    return app


class StatusCodes202AcceptedRequestAcceptedForProcessingBody(BaseModel):
    """Request body Pydantic model."""

    task: str


def status_codes_202_accepted_request_accepted_for_processing(
    body: StatusCodes202AcceptedRequestAcceptedForProcessingBody,
) -> Any:
    """Handler for POST /tasks/."""
    return Response(
        content={"message": "Task accepted for processing", "task_id": "abc123"},
        status_code=202,
        headers={"Content-Type": "application/json"},
    )


def create_app_status_codes_202_accepted_request_accepted_for_processing() -> Spikard:
    """App factory for fixture: 202 Accepted - Request accepted for processing"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/tasks/",
        body_schema={
            "additionalProperties": False,
            "properties": {"task": {"type": "string"}},
            "required": ["task"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(status_codes_202_accepted_request_accepted_for_processing)
    return app


def status_codes_307_temporary_redirect_method_preserved(
    body: dict[str, Any],
) -> Any:
    """Handler for POST /redirect-post."""
    return Response(status_code=307, headers={"location": "/target-post", "Content-Type": "application/json"})


def create_app_status_codes_307_temporary_redirect_method_preserved() -> Spikard:
    """App factory for fixture: 307 Temporary Redirect - Method preserved"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/redirect-post",
        body_schema={"additionalProperties": False, "properties": {}, "type": "object"},
        parameter_schema=None,
        file_params=None,
    )(status_codes_307_temporary_redirect_method_preserved)
    return app


def status_codes_500_internal_server_error_server_error() -> Any:
    """Handler for GET /error."""
    return Response(
        content={
            "detail": "Internal server error",
            "status": 500,
            "title": "Internal Server Error",
            "type": "https://spikard.dev/errors/internal-server-error",
        },
        status_code=500,
        headers={"Content-Type": "application/json"},
    )


def create_app_status_codes_500_internal_server_error_server_error() -> Spikard:
    """App factory for fixture: 500 Internal Server Error - Server error"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route("GET", "/error", body_schema=None, parameter_schema=None, file_params=None)(
        status_codes_500_internal_server_error_server_error
    )
    return app


def status_codes_20_414_uri_too_long() -> Any:
    """Handler for GET /data."""
    return Response(status_code=200, headers={"Content-Type": "application/json"})


def create_app_status_codes_20_414_uri_too_long() -> Spikard:
    """App factory for fixture: 20_414_uri_too_long"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route("GET", "/data", body_schema=None, parameter_schema=None, file_params=None)(
        status_codes_20_414_uri_too_long
    )
    return app


def status_codes_401_unauthorized_missing_authentication() -> Any:
    """Handler for GET /users/me."""
    return Response(
        content={"detail": "Not authenticated"},
        status_code=401,
        headers={"Content-Type": "application/json", "www-authenticate": "Bearer"},
    )


def create_app_status_codes_401_unauthorized_missing_authentication() -> Spikard:
    """App factory for fixture: 401 Unauthorized - Missing authentication"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route("GET", "/users/me", body_schema=None, parameter_schema=None, file_params=None)(
        status_codes_401_unauthorized_missing_authentication
    )
    return app


def status_codes_23_503_service_unavailable() -> Any:
    """Handler for GET /data."""
    return Response(
        content={
            "error": "Service Unavailable",
            "message": "The service is temporarily unavailable. Please try again later.",
        },
        status_code=503,
        headers={"Retry-After": "60", "Content-Type": "application/json"},
    )


def create_app_status_codes_23_503_service_unavailable() -> Spikard:
    """App factory for fixture: 23_503_service_unavailable"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route("GET", "/data", body_schema=None, parameter_schema=None, file_params=None)(
        status_codes_23_503_service_unavailable
    )
    return app


class StatusCodes19413PayloadTooLargeBody(BaseModel):
    """Request body Pydantic model."""

    data: str | None = None


def status_codes_19_413_payload_too_large(
    body: StatusCodes19413PayloadTooLargeBody,
) -> Any:
    """Handler for POST /upload."""
    return Response(
        content={
            "error": "Payload Too Large",
            "message": "Request body size exceeds maximum allowed size of 1024 bytes",
        },
        status_code=413,
        headers={"Content-Type": "application/json"},
    )


def create_app_status_codes_19_413_payload_too_large() -> Spikard:
    """App factory for fixture: 19_413_payload_too_large"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/upload",
        body_schema={"properties": {"data": {"type": "string"}}, "type": "object"},
        parameter_schema=None,
        file_params=None,
    )(status_codes_19_413_payload_too_large)
    return app


def status_codes_403_forbidden_insufficient_permissions() -> Any:
    """Handler for GET /admin/users."""
    return Response(
        content={"detail": "Not enough permissions"}, status_code=403, headers={"Content-Type": "application/json"}
    )


def create_app_status_codes_403_forbidden_insufficient_permissions() -> Spikard:
    """App factory for fixture: 403 Forbidden - Insufficient permissions"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route("GET", "/admin/users", body_schema=None, parameter_schema=None, file_params=None)(
        status_codes_403_forbidden_insufficient_permissions
    )
    return app


def status_codes_21_431_request_header_fields_too_large(
    x_large_header: str | None = None,
) -> Any:
    """Handler for GET /data."""
    return Response(
        content={
            "error": "Request Header Fields Too Large",
            "message": "Request headers exceed maximum allowed size of 8192 bytes",
        },
        status_code=431,
        headers={"Content-Type": "application/json"},
    )


def create_app_status_codes_21_431_request_header_fields_too_large() -> Spikard:
    """App factory for fixture: 21_431_request_header_fields_too_large"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/data",
        body_schema=None,
        parameter_schema={
            "properties": {"x_large_header": {"source": "header", "type": "string"}},
            "required": [],
            "type": "object",
        },
        file_params=None,
    )(status_codes_21_431_request_header_fields_too_large)
    return app


def status_codes_429_too_many_requests() -> Any:
    """Handler for GET /api/resource."""
    return Response(
        content={"detail": "Rate limit exceeded. Try again in 60 seconds."},
        status_code=429,
        headers={
            "X-RateLimit-Limit": "100",
            "X-RateLimit-Remaining": "0",
            "Content-Type": "application/json",
            "X-RateLimit-Reset": "1609459200",
            "Retry-After": "60",
        },
    )


def create_app_status_codes_429_too_many_requests() -> Spikard:
    """App factory for fixture: 429 Too Many Requests"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route("GET", "/api/resource", body_schema=None, parameter_schema=None, file_params=None)(
        status_codes_429_too_many_requests
    )
    return app


def status_codes_200_ok_success(
    code: str,
) -> Any:
    """Handler for GET /status-test/{code}."""
    return Response(content={"id": 1, "name": "Item 1"}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_status_codes_200_ok_success() -> Spikard:
    """App factory for fixture: 200 OK - Success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/status-test/{code}",
        body_schema=None,
        parameter_schema={
            "properties": {"code": {"source": "path", "type": "string"}},
            "required": ["code"],
            "type": "object",
        },
        file_params=None,
    )(status_codes_200_ok_success)
    return app


def status_codes_206_partial_content() -> Any:
    """Handler for GET /files/document.pdf."""
    return Response(
        content="binary_data_1024_bytes",
        status_code=206,
        headers={
            "Content-Type": "application/pdf",
            "Content-Range": "bytes 0-1023/5000",
            "Content-Length": "1024",
            "Accept-Ranges": "bytes",
        },
    )


def create_app_status_codes_206_partial_content() -> Spikard:
    """App factory for fixture: 206 Partial Content"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route("GET", "/files/document.pdf", body_schema=None, parameter_schema=None, file_params=None)(
        status_codes_206_partial_content
    )
    return app


def background_background_event_logging_second_payload(
    body: dict[str, Any],
) -> Any:
    """Handler for POST /background/events."""
    state = BACKGROUND_STATE.setdefault("background_background_event_logging_second_payload", [])
    value = body.get("event") if body is not None else None
    if value is None:
        raise ValueError("background task requires request body value")

    async def _background_task() -> None:
        state.append(value)

    background.run(_background_task())
    return Response(status_code=202, headers={"content-type": "application/json"})


def background_background_event_logging_second_payload_background_state() -> Any:
    """Background state endpoint."""
    state = BACKGROUND_STATE.get("background_background_event_logging_second_payload", [])
    return {"events": state}


def create_app_background_background_event_logging_second_payload() -> Spikard:
    """App factory for fixture: Background event logging - second payload"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/background/events",
        body_schema={
            "additionalProperties": False,
            "properties": {"event": {"type": "string"}},
            "required": ["event"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(background_background_event_logging_second_payload)
    app.register_route("GET", "/background/events", body_schema=None, parameter_schema=None, file_params=None)(
        background_background_event_logging_second_payload_background_state
    )
    return app


def background_background_event_logging(
    body: dict[str, Any],
) -> Any:
    """Handler for POST /background/events."""
    state = BACKGROUND_STATE.setdefault("background_background_event_logging", [])
    value = body.get("event") if body is not None else None
    if value is None:
        raise ValueError("background task requires request body value")

    async def _background_task() -> None:
        state.append(value)

    background.run(_background_task())
    return Response(status_code=202, headers={"content-type": "application/json"})


def background_background_event_logging_background_state() -> Any:
    """Background state endpoint."""
    state = BACKGROUND_STATE.get("background_background_event_logging", [])
    return {"events": state}


def create_app_background_background_event_logging() -> Spikard:
    """App factory for fixture: Background event logging"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/background/events",
        body_schema={
            "additionalProperties": False,
            "properties": {"event": {"type": "string"}},
            "required": ["event"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(background_background_event_logging)
    app.register_route("GET", "/background/events", body_schema=None, parameter_schema=None, file_params=None)(
        background_background_event_logging_background_state
    )
    return app


def validation_errors_invalid_uuid_format(
    item_id: UUID,
) -> Any:
    """Handler for GET /items/{item_id}."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "input": "not-a-uuid",
                    "loc": ["path", "item_id"],
                    "msg": "Input should be a valid UUID, invalid character: expected an optional prefix of `urn:uuid:` followed by [0-9a-fA-F-], found `n` at 0",
                    "type": "uuid_parsing",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_validation_errors_invalid_uuid_format() -> Spikard:
    """App factory for fixture: Invalid UUID format"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/items/{item_id}",
        body_schema=None,
        parameter_schema={
            "properties": {"item_id": {"format": "uuid", "source": "path", "type": "string"}},
            "required": ["item_id"],
            "type": "object",
        },
        file_params=None,
    )(validation_errors_invalid_uuid_format)
    return app


def validation_errors_invalid_boolean_value(
    is_active: bool,
    q: str,
) -> Any:
    """Handler for GET /items/."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "input": "maybe",
                    "loc": ["query", "is_active"],
                    "msg": "Input should be a valid boolean, unable to interpret input",
                    "type": "bool_parsing",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_validation_errors_invalid_boolean_value() -> Spikard:
    """App factory for fixture: Invalid boolean value"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/items/",
        body_schema=None,
        parameter_schema={
            "properties": {
                "is_active": {"source": "query", "type": "boolean"},
                "q": {"source": "query", "type": "string"},
            },
            "required": ["is_active", "q"],
            "type": "object",
        },
        file_params=None,
    )(validation_errors_invalid_boolean_value)
    return app


def validation_errors_missing_required_query_parameter(
    q: str,
) -> Any:
    """Handler for GET /items/."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [{"input": None, "loc": ["query", "q"], "msg": "Field required", "type": "missing"}],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_validation_errors_missing_required_query_parameter() -> Spikard:
    """App factory for fixture: Missing required query parameter"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/items/",
        body_schema=None,
        parameter_schema={
            "properties": {"q": {"source": "query", "type": "string"}},
            "required": ["q"],
            "type": "object",
        },
        file_params=None,
    )(validation_errors_missing_required_query_parameter)
    return app


class ValidationErrorsArrayMaxItemsConstraintViolationBody(NamedTuple):
    """Request body NamedTuple (immutable)."""

    name: str
    price: float
    tags: list[str]


def validation_errors_array_max_items_constraint_violation(
    body: ValidationErrorsArrayMaxItemsConstraintViolationBody,
) -> Any:
    """Handler for POST /items/."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "input": ["tag1", "tag2", "tag3", "tag4", "tag5", "tag6", "tag7", "tag8", "tag9", "tag10", "tag11"],
                    "loc": ["body", "tags"],
                    "msg": '["tag1","tag2","tag3","tag4","tag5","tag6","tag7","tag8","tag9","tag10","tag11"] has more than 10 items',
                    "type": "validation_error",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_validation_errors_array_max_items_constraint_violation() -> Spikard:
    """App factory for fixture: Array max_items constraint violation"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/items/",
        body_schema={
            "additionalProperties": False,
            "properties": {
                "name": {"type": "string"},
                "price": {"type": "number"},
                "tags": {"items": {"type": "string"}, "maxItems": 10, "type": "array"},
            },
            "required": ["name", "price", "tags"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(validation_errors_array_max_items_constraint_violation)
    return app


def validation_errors_numeric_constraint_violation_gt_greater_than(
    price: float,
    q: str,
) -> Any:
    """Handler for GET /items/."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"gt": 0},
                    "input": "0",
                    "loc": ["query", "price"],
                    "msg": "Input should be greater than 0",
                    "type": "greater_than",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_validation_errors_numeric_constraint_violation_gt_greater_than() -> Spikard:
    """App factory for fixture: Numeric constraint violation - gt (greater than)"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/items/",
        body_schema=None,
        parameter_schema={
            "properties": {
                "price": {"exclusiveMinimum": 0, "source": "query", "type": "number"},
                "q": {"source": "query", "type": "string"},
            },
            "required": ["price", "q"],
            "type": "object",
        },
        file_params=None,
    )(validation_errors_numeric_constraint_violation_gt_greater_than)
    return app


def validation_errors_string_regex_pattern_mismatch(
    q: str,
) -> Any:
    """Handler for GET /items/."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"pattern": "^[a-zA-Z0-9_-]+$"},
                    "input": "invalid!",
                    "loc": ["query", "q"],
                    "msg": "String should match pattern '^[a-zA-Z0-9_-]+$'",
                    "type": "string_pattern_mismatch",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_validation_errors_string_regex_pattern_mismatch() -> Spikard:
    """App factory for fixture: String regex pattern mismatch"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/items/",
        body_schema=None,
        parameter_schema={
            "properties": {"q": {"pattern": "^[a-zA-Z0-9_-]+$", "source": "query", "type": "string"}},
            "required": ["q"],
            "type": "object",
        },
        file_params=None,
    )(validation_errors_string_regex_pattern_mismatch)
    return app


def validation_errors_invalid_enum_value(
    model_name: str,
) -> Any:
    """Handler for GET /models/{model_name}."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"expected": "'alexnet', 'resnet' or 'lenet'"},
                    "input": "invalid_model",
                    "loc": ["path", "model_name"],
                    "msg": "Input should be 'alexnet', 'resnet' or 'lenet'",
                    "type": "enum",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_validation_errors_invalid_enum_value() -> Spikard:
    """App factory for fixture: Invalid enum value"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/models/{model_name}",
        body_schema=None,
        parameter_schema={
            "properties": {"model_name": {"enum": ["alexnet", "resnet", "lenet"], "source": "path", "type": "string"}},
            "required": ["model_name"],
            "type": "object",
        },
        file_params=None,
    )(validation_errors_invalid_enum_value)
    return app


def validation_errors_string_min_length_constraint_violation(
    q: str,
) -> Any:
    """Handler for GET /items/."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"min_length": 3},
                    "input": "ab",
                    "loc": ["query", "q"],
                    "msg": "String should have at least 3 characters",
                    "type": "string_too_short",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_validation_errors_string_min_length_constraint_violation() -> Spikard:
    """App factory for fixture: String min_length constraint violation"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/items/",
        body_schema=None,
        parameter_schema={
            "properties": {"q": {"minLength": 3, "source": "query", "type": "string"}},
            "required": ["q"],
            "type": "object",
        },
        file_params=None,
    )(validation_errors_string_min_length_constraint_violation)
    return app


@dataclass
class ValidationErrorsMultipleValidationErrorsBody:
    """Request body dataclass."""

    name: str
    price: int
    quantity: int


def validation_errors_multiple_validation_errors(
    body: ValidationErrorsMultipleValidationErrorsBody,
) -> Any:
    """Handler for POST /items/."""
    return Response(
        content={
            "detail": "3 validation errors in request",
            "errors": [
                {
                    "ctx": {"min_length": 3},
                    "input": "X",
                    "loc": ["body", "name"],
                    "msg": "String should have at least 3 characters",
                    "type": "string_too_short",
                },
                {
                    "ctx": {"gt": 0},
                    "input": -10,
                    "loc": ["body", "price"],
                    "msg": "Input should be greater than 0",
                    "type": "greater_than",
                },
                {
                    "input": "not_a_number",
                    "loc": ["body", "quantity"],
                    "msg": "Input should be a valid integer, unable to parse string as an integer",
                    "type": "int_parsing",
                },
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_validation_errors_multiple_validation_errors() -> Spikard:
    """App factory for fixture: Multiple validation errors"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/items/",
        body_schema={
            "additionalProperties": False,
            "properties": {
                "name": {"minLength": 3, "type": "string"},
                "price": {"exclusiveMinimum": 0, "type": "integer"},
                "quantity": {"type": "integer"},
            },
            "required": ["name", "price", "quantity"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(validation_errors_multiple_validation_errors)
    return app


def validation_errors_string_max_length_constraint_violation(
    q: str,
) -> Any:
    """Handler for GET /items/."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"max_length": 50},
                    "input": "this_is_a_very_long_query_string_that_exceeds_maximum_length_limit_for_this_parameter",
                    "loc": ["query", "q"],
                    "msg": "String should have at most 50 characters",
                    "type": "string_too_long",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_validation_errors_string_max_length_constraint_violation() -> Spikard:
    """App factory for fixture: String max_length constraint violation"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/items/",
        body_schema=None,
        parameter_schema={
            "properties": {"q": {"maxLength": 50, "source": "query", "type": "string"}},
            "required": ["q"],
            "type": "object",
        },
        file_params=None,
    )(validation_errors_string_max_length_constraint_violation)
    return app


class ValidationErrorsNestedObjectValidationErrorBody(msgspec.Struct):
    """Request body msgspec.Struct (fast typed)."""

    name: str
    price: float
    seller: dict[str, Any]


def validation_errors_nested_object_validation_error(
    body: ValidationErrorsNestedObjectValidationErrorBody,
) -> Any:
    """Handler for POST /items/."""
    return Response(
        content={
            "detail": "3 validation errors in request",
            "errors": [
                {
                    "ctx": {"min_length": 3},
                    "input": "SF",
                    "loc": ["body", "seller", "address", "city"],
                    "msg": "String should have at least 3 characters",
                    "type": "string_too_short",
                },
                {
                    "ctx": {"min_length": 5},
                    "input": "123",
                    "loc": ["body", "seller", "address", "zip_code"],
                    "msg": "String should have at least 5 characters",
                    "type": "string_too_short",
                },
                {
                    "ctx": {"min_length": 3},
                    "input": "Jo",
                    "loc": ["body", "seller", "name"],
                    "msg": "String should have at least 3 characters",
                    "type": "string_too_short",
                },
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_validation_errors_nested_object_validation_error() -> Spikard:
    """App factory for fixture: Nested object validation error"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/items/",
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
                                "city": {"minLength": 3, "type": "string"},
                                "zip_code": {"minLength": 5, "type": "string"},
                            },
                            "required": ["city", "zip_code"],
                            "type": "object",
                        },
                        "name": {"minLength": 3, "type": "string"},
                    },
                    "required": ["name", "address"],
                    "type": "object",
                },
            },
            "required": ["name", "price", "seller"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(validation_errors_nested_object_validation_error)
    return app


class ValidationErrors10NestedErrorPathBody(BaseModel):
    """Request body Pydantic model."""

    profile: dict[str, Any]


def validation_errors_10_nested_error_path(
    body: ValidationErrors10NestedErrorPathBody,
) -> Any:
    """Handler for POST /profiles."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"pattern": "^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\\.[a-zA-Z0-9-.]+$"},
                    "input": "invalid",
                    "loc": ["body", "profile", "contact", "email"],
                    "msg": "String should match pattern '^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\\.[a-zA-Z0-9-.]+$'",
                    "type": "string_pattern_mismatch",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_validation_errors_10_nested_error_path() -> Spikard:
    """App factory for fixture: 10_nested_error_path"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
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
        parameter_schema=None,
        file_params=None,
    )(validation_errors_10_nested_error_path)
    return app


def validation_errors_invalid_datetime_format(
    body: dict[str, Any],
) -> Any:
    """Handler for POST /items/."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "input": "not-a-datetime",
                    "loc": ["body", "created_at"],
                    "msg": "Input should be a valid datetime",
                    "type": "datetime_parsing",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_validation_errors_invalid_datetime_format() -> Spikard:
    """App factory for fixture: Invalid datetime format"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/items/",
        body_schema={
            "additionalProperties": False,
            "properties": {
                "created_at": {"format": "date-time", "type": "string"},
                "name": {"type": "string"},
                "price": {"type": "number"},
            },
            "required": ["name", "price", "created_at"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(validation_errors_invalid_datetime_format)
    return app


class ValidationErrorsArrayItemValidationErrorBody(TypedDict):
    """Request body type (TypedDict - runtime is dict)."""

    name: str
    price: float
    tags: list[str]


def validation_errors_array_item_validation_error(
    body: ValidationErrorsArrayItemValidationErrorBody,
) -> Any:
    """Handler for POST /items/."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "input": 123,
                    "loc": ["body", "tags", "2"],
                    "msg": "Input should be a valid unknown",
                    "type": "type_error",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_validation_errors_array_item_validation_error() -> Spikard:
    """App factory for fixture: Array item validation error"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/items/",
        body_schema={
            "additionalProperties": False,
            "properties": {
                "name": {"type": "string"},
                "price": {"type": "number"},
                "tags": {"items": {"type": "string"}, "type": "array"},
            },
            "required": ["name", "price", "tags"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(validation_errors_array_item_validation_error)
    return app


@dataclass
class ValidationErrorsMissingRequiredBodyFieldBody:
    """Request body dataclass."""

    name: str
    price: str


def validation_errors_missing_required_body_field(
    body: ValidationErrorsMissingRequiredBodyFieldBody,
) -> Any:
    """Handler for POST /items/."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {"input": {"name": "Item"}, "loc": ["body", "price"], "msg": "Field required", "type": "missing"}
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_validation_errors_missing_required_body_field() -> Spikard:
    """App factory for fixture: Missing required body field"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/items/",
        body_schema={
            "additionalProperties": False,
            "properties": {"name": {"type": "string"}, "price": {"type": "string"}},
            "required": ["name", "price"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(validation_errors_missing_required_body_field)
    return app


class ValidationErrorsBodyFieldTypeErrorStringForFloatBody(NamedTuple):
    """Request body NamedTuple (immutable)."""

    name: str
    price: float


def validation_errors_body_field_type_error_string_for_float(
    body: ValidationErrorsBodyFieldTypeErrorStringForFloatBody,
) -> Any:
    """Handler for POST /items/."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "input": "not_a_float",
                    "loc": ["body", "price"],
                    "msg": "Input should be a valid number, unable to parse string as a number",
                    "type": "float_parsing",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_validation_errors_body_field_type_error_string_for_float() -> Spikard:
    """App factory for fixture: Body field type error - string for float"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/items/",
        body_schema={
            "additionalProperties": False,
            "properties": {"name": {"type": "string"}, "price": {"type": "number"}},
            "required": ["name", "price"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(validation_errors_body_field_type_error_string_for_float)
    return app


def validation_errors_malformed_json_body(
    body: str,
) -> Any:
    """Handler for POST /items/."""
    return Response(
        content={"detail": "Invalid request format"}, status_code=400, headers={"Content-Type": "application/json"}
    )


def create_app_validation_errors_malformed_json_body() -> Spikard:
    """App factory for fixture: Malformed JSON body"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route("POST", "/items/", body_schema={"type": "string"}, parameter_schema=None, file_params=None)(
        validation_errors_malformed_json_body
    )
    return app


def validation_errors_query_param_type_error_string_provided_for_int(
    q: str,
    skip: int,
) -> Any:
    """Handler for GET /items/."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "input": "not_a_number",
                    "loc": ["query", "skip"],
                    "msg": "Input should be a valid integer, unable to parse string as an integer",
                    "type": "int_parsing",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_validation_errors_query_param_type_error_string_provided_for_int() -> Spikard:
    """App factory for fixture: Query param type error - string provided for int"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/items/",
        body_schema=None,
        parameter_schema={
            "properties": {"q": {"source": "query", "type": "string"}, "skip": {"source": "query", "type": "integer"}},
            "required": ["q", "skip"],
            "type": "object",
        },
        file_params=None,
    )(validation_errors_query_param_type_error_string_provided_for_int)
    return app


def validation_errors_header_validation_error(
    q: str,
    x_token: str,
) -> Any:
    """Handler for GET /items/."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [{"input": None, "loc": ["headers", "x-token"], "msg": "Field required", "type": "missing"}],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_validation_errors_header_validation_error() -> Spikard:
    """App factory for fixture: Header validation error"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/items/",
        body_schema=None,
        parameter_schema={
            "properties": {
                "q": {"source": "query", "type": "string"},
                "x_token": {"source": "header", "type": "string"},
            },
            "required": ["q", "x_token"],
            "type": "object",
        },
        file_params=None,
    )(validation_errors_header_validation_error)
    return app


class ValidationErrors09MultipleValidationErrorsBody(TypedDict):
    """Request body type (TypedDict - runtime is dict)."""

    age: int
    email: str
    name: str


def validation_errors_09_multiple_validation_errors(
    body: ValidationErrors09MultipleValidationErrorsBody,
) -> Any:
    """Handler for POST /users."""
    return Response(
        content={
            "detail": "3 validation errors in request",
            "errors": [
                {
                    "ctx": {"ge": 18},
                    "input": 15,
                    "loc": ["body", "age"],
                    "msg": "Input should be greater than or equal to 18",
                    "type": "greater_than_equal",
                },
                {
                    "ctx": {"pattern": "^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\\.[a-zA-Z0-9-.]+$"},
                    "input": "invalid-email",
                    "loc": ["body", "email"],
                    "msg": "String should match pattern '^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\\.[a-zA-Z0-9-.]+$'",
                    "type": "string_pattern_mismatch",
                },
                {
                    "ctx": {"min_length": 3},
                    "input": "ab",
                    "loc": ["body", "name"],
                    "msg": "String should have at least 3 characters",
                    "type": "string_too_short",
                },
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_validation_errors_09_multiple_validation_errors() -> Spikard:
    """App factory for fixture: 09_multiple_validation_errors"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/users",
        body_schema={
            "properties": {
                "age": {"minimum": 18, "type": "integer"},
                "email": {"format": "email", "type": "string"},
                "name": {"minLength": 3, "type": "string"},
            },
            "required": ["name", "email", "age"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(validation_errors_09_multiple_validation_errors)
    return app


def validation_errors_numeric_constraint_violation_le_less_than_or_equal(
    limit: int,
    q: str,
) -> Any:
    """Handler for GET /items/."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"le": 100},
                    "input": "101",
                    "loc": ["query", "limit"],
                    "msg": "Input should be less than or equal to 100",
                    "type": "less_than_equal",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_validation_errors_numeric_constraint_violation_le_less_than_or_equal() -> Spikard:
    """App factory for fixture: Numeric constraint violation - le (less than or equal)"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/items/",
        body_schema=None,
        parameter_schema={
            "properties": {
                "limit": {"maximum": 100, "source": "query", "type": "integer"},
                "q": {"source": "query", "type": "string"},
            },
            "required": ["limit", "q"],
            "type": "object",
        },
        file_params=None,
    )(validation_errors_numeric_constraint_violation_le_less_than_or_equal)
    return app


class ValidationErrorsArrayMinItemsConstraintViolationBody(NamedTuple):
    """Request body NamedTuple (immutable)."""

    name: str
    price: float
    tags: list[str]


def validation_errors_array_min_items_constraint_violation(
    body: ValidationErrorsArrayMinItemsConstraintViolationBody,
) -> Any:
    """Handler for POST /items/."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {"input": [], "loc": ["body", "tags"], "msg": "[] has less than 1 item", "type": "validation_error"}
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_validation_errors_array_min_items_constraint_violation() -> Spikard:
    """App factory for fixture: Array min_items constraint violation"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/items/",
        body_schema={
            "additionalProperties": False,
            "properties": {
                "name": {"type": "string"},
                "price": {"type": "number"},
                "tags": {"items": {}, "minItems": 1, "type": "array"},
            },
            "required": ["name", "price", "tags"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(validation_errors_array_min_items_constraint_violation)
    return app


def content_types_415_unsupported_media_type(
    body: str,
) -> Any:
    """Handler for POST /items/."""
    return Response(
        content={"detail": "Unsupported media type"}, status_code=415, headers={"Content-Type": "application/json"}
    )


def create_app_content_types_415_unsupported_media_type() -> Spikard:
    """App factory for fixture: 415 Unsupported Media Type"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route("POST", "/items/", body_schema={"type": "string"}, parameter_schema=None, file_params=None)(
        content_types_415_unsupported_media_type
    )
    return app


def content_types_xml_response_application_xml() -> Any:
    """Handler for GET /xml."""
    return Response(
        content='<?xml version="1.0"?><item><name>Item</name><price>42.0</price></item>',
        status_code=200,
        headers={"content-type": "application/xml"},
    )


def create_app_content_types_xml_response_application_xml() -> Spikard:
    """App factory for fixture: XML response - application/xml"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route("GET", "/xml", body_schema=None, parameter_schema=None, file_params=None)(
        content_types_xml_response_application_xml
    )
    return app


@dataclass
class ContentTypes14ContentTypeCaseInsensitiveBody:
    """Request body dataclass."""

    name: str


def content_types_14_content_type_case_insensitive(
    body: ContentTypes14ContentTypeCaseInsensitiveBody,
) -> Any:
    """Handler for POST /data."""
    return Response(content={"name": "test"}, status_code=201, headers={"Content-Type": "application/json"})


def create_app_content_types_14_content_type_case_insensitive() -> Spikard:
    """App factory for fixture: 14_content_type_case_insensitive"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/data",
        body_schema={"properties": {"name": {"type": "string"}}, "required": ["name"], "type": "object"},
        parameter_schema=None,
        file_params=None,
    )(content_types_14_content_type_case_insensitive)
    return app


def content_types_json_with_utf_8_charset() -> Any:
    """Handler for GET /items/unicode."""
    return Response(
        content={"emoji": "☕", "name": "Café"},
        status_code=200,
        headers={"content-type": "application/json; charset=utf-8"},
    )


def create_app_content_types_json_with_utf_8_charset() -> Spikard:
    """App factory for fixture: JSON with UTF-8 charset"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route("GET", "/items/unicode", body_schema=None, parameter_schema=None, file_params=None)(
        content_types_json_with_utf_8_charset
    )
    return app


class ContentTypes16TextPlainNotAcceptedBody(msgspec.Struct):
    """Request body msgspec.Struct (fast typed)."""

    data: str


def content_types_16_text_plain_not_accepted(
    body: ContentTypes16TextPlainNotAcceptedBody,
) -> Any:
    """Handler for POST /data."""
    return Response(
        content={"error": "Unsupported Media Type. Expected application/json"},
        status_code=415,
        headers={"Content-Type": "application/json"},
    )


def create_app_content_types_16_text_plain_not_accepted() -> Spikard:
    """App factory for fixture: 16_text_plain_not_accepted"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/data",
        body_schema={"properties": {"data": {"type": "string"}}, "required": ["data"], "type": "object"},
        parameter_schema=None,
        file_params=None,
    )(content_types_16_text_plain_not_accepted)
    return app


def content_types_pdf_response_application_pdf() -> Any:
    """Handler for GET /download/document.pdf."""
    return Response(
        content="pdf_binary_data",
        status_code=200,
        headers={"content-disposition": "attachment; filename=document.pdf", "content-type": "application/pdf"},
    )


def create_app_content_types_pdf_response_application_pdf() -> Spikard:
    """App factory for fixture: PDF response - application/pdf"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route("GET", "/download/document.pdf", body_schema=None, parameter_schema=None, file_params=None)(
        content_types_pdf_response_application_pdf
    )
    return app


def content_types_20_content_length_mismatch(
    body: dict[str, Any],
    content_length: str | None = None,
) -> Any:
    """Handler for POST /data."""
    return Response(
        content={"error": "Content-Length header does not match actual body size"},
        status_code=400,
        headers={"Content-Type": "application/json"},
    )


def create_app_content_types_20_content_length_mismatch() -> Spikard:
    """App factory for fixture: 20_content_length_mismatch"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/data",
        body_schema={"properties": {"value": {"type": "string"}}, "type": "object"},
        parameter_schema={
            "properties": {"content_length": {"source": "header", "type": "string"}},
            "required": [],
            "type": "object",
        },
        file_params=None,
    )(content_types_20_content_length_mismatch)
    return app


class ContentTypes17VendorJsonAcceptedBody(TypedDict):
    """Request body type (TypedDict - runtime is dict)."""

    data: str


def content_types_17_vendor_json_accepted(
    body: ContentTypes17VendorJsonAcceptedBody,
) -> Any:
    """Handler for POST /api/v1/resource."""
    return Response(content={"data": "value"}, status_code=201, headers={"Content-Type": "application/json"})


def create_app_content_types_17_vendor_json_accepted() -> Spikard:
    """App factory for fixture: 17_vendor_json_accepted"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/api/v1/resource",
        body_schema={"properties": {"data": {"type": "string"}}, "required": ["data"], "type": "object"},
        parameter_schema=None,
        file_params=None,
    )(content_types_17_vendor_json_accepted)
    return app


@dataclass
class ContentTypes13JsonWithCharsetUtf16Body:
    """Request body dataclass."""

    value: str | None = None


def content_types_13_json_with_charset_utf16(
    body: ContentTypes13JsonWithCharsetUtf16Body,
) -> Any:
    """Handler for POST /data."""
    return Response(
        content={"error": "Unsupported charset 'utf-16' for JSON. Only UTF-8 is supported."},
        status_code=415,
        headers={"Content-Type": "application/json"},
    )


def create_app_content_types_13_json_with_charset_utf16() -> Spikard:
    """App factory for fixture: 13_json_with_charset_utf16"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/data",
        body_schema={"properties": {"value": {"type": "string"}}, "type": "object"},
        parameter_schema=None,
        file_params=None,
    )(content_types_13_json_with_charset_utf16)
    return app


def content_types_json_response_application_json() -> Any:
    """Handler for GET /items/json."""
    return Response(
        content={"name": "Item", "price": 42.0}, status_code=200, headers={"content-type": "application/json"}
    )


def create_app_content_types_json_response_application_json() -> Spikard:
    """App factory for fixture: JSON response - application/json"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route("GET", "/items/json", body_schema=None, parameter_schema=None, file_params=None)(
        content_types_json_response_application_json
    )
    return app


def content_types_15_multipart_boundary_required() -> Any:
    """Handler for POST /upload."""
    return Response(
        content={"error": "multipart/form-data requires 'boundary' parameter"},
        status_code=400,
        headers={"Content-Type": "application/json"},
    )


def create_app_content_types_15_multipart_boundary_required() -> Spikard:
    """App factory for fixture: 15_multipart_boundary_required"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/upload",
        body_schema=None,
        parameter_schema={"properties": {}, "required": [], "type": "object"},
        file_params={"document": {"required": True}},
    )(content_types_15_multipart_boundary_required)
    return app


def content_types_content_negotiation_accept_header(
    id: str,
) -> Any:
    """Handler for GET /accept-test/{id}."""
    return Response(content={"id": 1, "name": "Item"}, status_code=200, headers={"content-type": "application/json"})


def create_app_content_types_content_negotiation_accept_header() -> Spikard:
    """App factory for fixture: Content negotiation - Accept header"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET",
        "/accept-test/{id}",
        body_schema=None,
        parameter_schema={
            "properties": {"id": {"source": "path", "type": "string"}},
            "required": ["id"],
            "type": "object",
        },
        file_params=None,
    )(content_types_content_negotiation_accept_header)
    return app


def content_types_html_response_text_html() -> Any:
    """Handler for GET /html."""
    return Response(
        content="<html><body><h1>Hello</h1></body></html>",
        status_code=200,
        headers={"content-type": "text/html; charset=utf-8"},
    )


def create_app_content_types_html_response_text_html() -> Spikard:
    """App factory for fixture: HTML response - text/html"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route("GET", "/html", body_schema=None, parameter_schema=None, file_params=None)(
        content_types_html_response_text_html
    )
    return app


def content_types_jpeg_image_response_image_jpeg() -> Any:
    """Handler for GET /images/photo.jpg."""
    return Response(content="jpeg_binary_data", status_code=200, headers={"content-type": "image/jpeg"})


def create_app_content_types_jpeg_image_response_image_jpeg() -> Spikard:
    """App factory for fixture: JPEG image response - image/jpeg"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route("GET", "/images/photo.jpg", body_schema=None, parameter_schema=None, file_params=None)(
        content_types_jpeg_image_response_image_jpeg
    )
    return app


@dataclass
class ContentTypes19MissingContentTypeDefaultJsonBody:
    """Request body dataclass."""

    name: str


def content_types_19_missing_content_type_default_json(
    body: ContentTypes19MissingContentTypeDefaultJsonBody,
) -> Any:
    """Handler for POST /data."""
    return Response(content={"name": "test"}, status_code=201, headers={"Content-Type": "application/json"})


def create_app_content_types_19_missing_content_type_default_json() -> Spikard:
    """App factory for fixture: 19_missing_content_type_default_json"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/data",
        body_schema={"properties": {"name": {"type": "string"}}, "required": ["name"], "type": "object"},
        parameter_schema=None,
        file_params=None,
    )(content_types_19_missing_content_type_default_json)
    return app


def content_types_png_image_response_image_png() -> Any:
    """Handler for GET /images/logo.png."""
    return Response(content="png_binary_data", status_code=200, headers={"content-type": "image/png"})


def create_app_content_types_png_image_response_image_png() -> Spikard:
    """App factory for fixture: PNG image response - image/png"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route("GET", "/images/logo.png", body_schema=None, parameter_schema=None, file_params=None)(
        content_types_png_image_response_image_png
    )
    return app


def content_types_plain_text_response_text_plain() -> Any:
    """Handler for GET /text."""
    return Response(content="Hello, World!", status_code=200, headers={"content-type": "text/plain; charset=utf-8"})


def create_app_content_types_plain_text_response_text_plain() -> Spikard:
    """App factory for fixture: Plain text response - text/plain"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route("GET", "/text", body_schema=None, parameter_schema=None, file_params=None)(
        content_types_plain_text_response_text_plain
    )
    return app


class ContentTypes18ContentTypeWithMultipleParamsBody(BaseModel):
    """Request body Pydantic model."""

    value: str | None = None


def content_types_18_content_type_with_multiple_params(
    body: ContentTypes18ContentTypeWithMultipleParamsBody,
) -> Any:
    """Handler for POST /data."""
    return Response(content={"value": "test"}, status_code=201, headers={"Content-Type": "application/json"})


def create_app_content_types_18_content_type_with_multiple_params() -> Spikard:
    """App factory for fixture: 18_content_type_with_multiple_params"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/data",
        body_schema={"properties": {"value": {"type": "string"}}, "type": "object"},
        parameter_schema=None,
        file_params=None,
    )(content_types_18_content_type_with_multiple_params)
    return app


def content_types_csv_response_text_csv() -> Any:
    """Handler for GET /export/data.csv."""
    return Response(
        content="id,name,price\n1,Item A,10.0\n2,Item B,20.0",
        status_code=200,
        headers={"content-disposition": "attachment; filename=data.csv", "content-type": "text/csv; charset=utf-8"},
    )


def create_app_content_types_csv_response_text_csv() -> Spikard:
    """App factory for fixture: CSV response - text/csv"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route("GET", "/export/data.csv", body_schema=None, parameter_schema=None, file_params=None)(
        content_types_csv_response_text_csv
    )
    return app


def content_types_binary_response_application_octet_stream() -> Any:
    """Handler for GET /download/file.bin."""
    return Response(
        content="binary_data_placeholder",
        status_code=200,
        headers={"content-type": "application/octet-stream", "content-disposition": "attachment; filename=file.bin"},
    )


def create_app_content_types_binary_response_application_octet_stream() -> Spikard:
    """App factory for fixture: Binary response - application/octet-stream"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route("GET", "/download/file.bin", body_schema=None, parameter_schema=None, file_params=None)(
        content_types_binary_response_application_octet_stream
    )
    return app


def url_encoded_simple_form_submission_success(
    body: dict[str, Any],
) -> Any:
    """Handler for POST /login/."""
    return Response(content={"username": "johndoe"}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_url_encoded_simple_form_submission_success() -> Spikard:
    """App factory for fixture: Simple form submission - success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/login/",
        body_schema={
            "properties": {"password": {"type": "string"}, "username": {"type": "string"}},
            "required": ["username", "password"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(url_encoded_simple_form_submission_success)
    return app


class UrlEncoded15SpecialCharactersFieldNamesBody(TypedDict):
    """Request body type (TypedDict - runtime is dict)."""

    contact_email: str | None
    user_name: str | None


def url_encoded_15_special_characters_field_names(
    body: UrlEncoded15SpecialCharactersFieldNamesBody,
) -> Any:
    """Handler for POST /data."""
    return Response(
        content={"contact.email": "john@example.com", "user-name": "JohnDoe"},
        status_code=201,
        headers={"Content-Type": "application/json"},
    )


def create_app_url_encoded_15_special_characters_field_names() -> Spikard:
    """App factory for fixture: 15_special_characters_field_names"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/data",
        body_schema={
            "properties": {"contact.email": {"format": "email", "type": "string"}, "user-name": {"type": "string"}},
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(url_encoded_15_special_characters_field_names)
    return app


@dataclass
class UrlEncodedPatternValidationFailBody:
    """Request body dataclass."""

    username: str


def url_encoded_pattern_validation_fail(
    body: UrlEncodedPatternValidationFailBody,
) -> Any:
    """Handler for POST /form/validated."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"pattern": "^[a-z0-9_]+$"},
                    "input": "john doe",
                    "loc": ["body", "username"],
                    "msg": "String should match pattern '^[a-z0-9_]+$'",
                    "type": "string_pattern_mismatch",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_url_encoded_pattern_validation_fail() -> Spikard:
    """App factory for fixture: Pattern validation - fail"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/form/validated",
        body_schema={
            "properties": {"username": {"pattern": "^[a-z0-9_]+$", "type": "string"}},
            "required": ["username"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(url_encoded_pattern_validation_fail)
    return app


class UrlEncoded22AdditionalPropertiesStrictFailureBody(NamedTuple):
    """Request body NamedTuple (immutable)."""

    theme: str


def url_encoded_22_additional_properties_strict_failure(
    body: UrlEncoded22AdditionalPropertiesStrictFailureBody,
) -> Any:
    """Handler for POST /settings."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"property": "unknown_field"},
                    "loc": ["body", "unknown_field"],
                    "msg": "Additional properties are not allowed",
                    "type": "validation_error",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_url_encoded_22_additional_properties_strict_failure() -> Spikard:
    """App factory for fixture: 22_additional_properties_strict_failure"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/settings",
        body_schema={
            "additionalProperties": False,
            "properties": {"theme": {"enum": ["light", "dark"], "type": "string"}},
            "required": ["theme"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(url_encoded_22_additional_properties_strict_failure)
    return app


class UrlEncoded17PatternValidationFailureBody(msgspec.Struct):
    """Request body msgspec.Struct (fast typed)."""

    account_id: str


def url_encoded_17_pattern_validation_failure(
    body: UrlEncoded17PatternValidationFailureBody,
) -> Any:
    """Handler for POST /accounts."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"pattern": "^ACC-[0-9]{6}$", "value": "INVALID123"},
                    "loc": ["body", "account_id"],
                    "msg": "String does not match pattern '^ACC-[0-9]{6}$'",
                    "type": "validation_error",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_url_encoded_17_pattern_validation_failure() -> Spikard:
    """App factory for fixture: 17_pattern_validation_failure"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/accounts",
        body_schema={
            "properties": {"account_id": {"pattern": "^ACC-[0-9]{6}$", "type": "string"}},
            "required": ["account_id"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(url_encoded_17_pattern_validation_failure)
    return app


class UrlEncoded20FormatEmailValidationFailureBody(BaseModel):
    """Request body Pydantic model."""

    email: str


def url_encoded_20_format_email_validation_failure(
    body: UrlEncoded20FormatEmailValidationFailureBody,
) -> Any:
    """Handler for POST /subscribe."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"format": "email", "value": "not-an-email"},
                    "loc": ["body", "email"],
                    "msg": "Invalid email format",
                    "type": "validation_error",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_url_encoded_20_format_email_validation_failure() -> Spikard:
    """App factory for fixture: 20_format_email_validation_failure"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/subscribe",
        body_schema={
            "properties": {"email": {"format": "email", "type": "string"}},
            "required": ["email"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(url_encoded_20_format_email_validation_failure)
    return app


def url_encoded_multiple_values_for_same_field(
    body: dict[str, Any],
) -> Any:
    """Handler for POST /form/tags."""
    return Response(
        content={"tags": ["python", "fastapi", "web"]}, status_code=200, headers={"Content-Type": "application/json"}
    )


def create_app_url_encoded_multiple_values_for_same_field() -> Spikard:
    """App factory for fixture: Multiple values for same field"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/form/tags",
        body_schema={
            "properties": {"tags": {"items": {"type": "string"}, "type": "array"}},
            "required": ["tags"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(url_encoded_multiple_values_for_same_field)
    return app


class UrlEncodedRequiredFieldMissingValidationErrorBody(TypedDict):
    """Request body type (TypedDict - runtime is dict)."""

    password: str
    username: str


def url_encoded_required_field_missing_validation_error(
    body: UrlEncodedRequiredFieldMissingValidationErrorBody,
) -> Any:
    """Handler for POST /login/."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [{"input": "", "loc": ["body", "username"], "msg": "Field required", "type": "missing"}],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_url_encoded_required_field_missing_validation_error() -> Spikard:
    """App factory for fixture: Required field missing - validation error"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/login/",
        body_schema={
            "properties": {"password": {"type": "string"}, "username": {"type": "string"}},
            "required": ["username", "password"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(url_encoded_required_field_missing_validation_error)
    return app


@dataclass
class UrlEncoded13ArrayFieldSuccessBody:
    """Request body dataclass."""

    tags: list[str]


def url_encoded_13_array_field_success(
    body: UrlEncoded13ArrayFieldSuccessBody,
) -> Any:
    """Handler for POST /register."""
    return Response(
        content={"tags": ["python", "rust", "typescript"]},
        status_code=201,
        headers={"Content-Type": "application/json"},
    )


def create_app_url_encoded_13_array_field_success() -> Spikard:
    """App factory for fixture: 13_array_field_success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/register",
        body_schema={
            "properties": {"tags": {"items": {"type": "string"}, "minItems": 1, "type": "array"}},
            "required": ["tags"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(url_encoded_13_array_field_success)
    return app


class UrlEncodedNumericFieldTypeConversionBody(NamedTuple):
    """Request body NamedTuple (immutable)."""

    username: str
    age: int | None = None


def url_encoded_numeric_field_type_conversion(
    body: UrlEncodedNumericFieldTypeConversionBody,
) -> Any:
    """Handler for POST /form/."""
    return Response(
        content={"age": 30, "username": "johndoe"}, status_code=200, headers={"Content-Type": "application/json"}
    )


def create_app_url_encoded_numeric_field_type_conversion() -> Spikard:
    """App factory for fixture: Numeric field type conversion"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/form/",
        body_schema={
            "properties": {"age": {"type": "integer"}, "username": {"type": "string"}},
            "required": ["username"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(url_encoded_numeric_field_type_conversion)
    return app


class UrlEncodedSpecialCharactersEncodingBody(msgspec.Struct):
    """Request body msgspec.Struct (fast typed)."""

    name: str
    description: str | None = None


def url_encoded_special_characters_encoding(
    body: UrlEncodedSpecialCharactersEncodingBody,
) -> Any:
    """Handler for POST /form/."""
    return Response(
        content={"description": "Test & Development", "name": "John Doe"},
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_url_encoded_special_characters_encoding() -> Spikard:
    """App factory for fixture: Special characters encoding"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/form/",
        body_schema={
            "properties": {"description": {"type": "string"}, "name": {"type": "string"}},
            "required": ["name"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(url_encoded_special_characters_encoding)
    return app


class UrlEncodedBooleanFieldConversionBody(BaseModel):
    """Request body Pydantic model."""

    username: str
    subscribe: bool | None = None


def url_encoded_boolean_field_conversion(
    body: UrlEncodedBooleanFieldConversionBody,
) -> Any:
    """Handler for POST /form/."""
    return Response(
        content={"subscribe": True, "username": "johndoe"},
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_url_encoded_boolean_field_conversion() -> Spikard:
    """App factory for fixture: Boolean field conversion"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/form/",
        body_schema={
            "properties": {"subscribe": {"type": "boolean"}, "username": {"type": "string"}},
            "required": ["username"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(url_encoded_boolean_field_conversion)
    return app


def url_encoded_empty_string_value(
    body: dict[str, Any],
) -> Any:
    """Handler for POST /form/."""
    return Response(
        content={"description": "", "username": "johndoe"},
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_url_encoded_empty_string_value() -> Spikard:
    """App factory for fixture: Empty string value"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/form/",
        body_schema={
            "properties": {"description": {"type": "string"}, "username": {"type": "string"}},
            "required": ["username"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(url_encoded_empty_string_value)
    return app


class UrlEncodedOauth2PasswordGrantFlowBody(TypedDict):
    """Request body type (TypedDict - runtime is dict)."""

    grant_type: str
    password: str
    username: str
    scope: str | None


def url_encoded_oauth2_password_grant_flow(
    body: UrlEncodedOauth2PasswordGrantFlowBody,
) -> Any:
    """Handler for POST /token."""
    return Response(
        content={"access_token": "johndoe", "token_type": "bearer"},
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_url_encoded_oauth2_password_grant_flow() -> Spikard:
    """App factory for fixture: OAuth2 password grant flow"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
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
        parameter_schema=None,
        file_params=None,
    )(url_encoded_oauth2_password_grant_flow)
    return app


@dataclass
class UrlEncoded19ArrayMinitemsValidationFailureBody:
    """Request body dataclass."""

    tags: list[str]


def url_encoded_19_array_minitems_validation_failure(
    body: UrlEncoded19ArrayMinitemsValidationFailureBody,
) -> Any:
    """Handler for POST /tags."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"actual_items": 1, "min_items": 2},
                    "loc": ["body", "tags"],
                    "msg": "Array must contain at least 2 items",
                    "type": "validation_error",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_url_encoded_19_array_minitems_validation_failure() -> Spikard:
    """App factory for fixture: 19_array_minitems_validation_failure"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/tags",
        body_schema={
            "properties": {"tags": {"items": {"type": "string"}, "minItems": 2, "type": "array"}},
            "required": ["tags"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(url_encoded_19_array_minitems_validation_failure)
    return app


class UrlEncodedOptionalFieldMissingSuccessBody(NamedTuple):
    """Request body NamedTuple (immutable)."""

    password: str
    username: str
    email: str | None = None


def url_encoded_optional_field_missing_success(
    body: UrlEncodedOptionalFieldMissingSuccessBody,
) -> Any:
    """Handler for POST /register/."""
    return Response(
        content={"email": None, "username": "johndoe"}, status_code=200, headers={"Content-Type": "application/json"}
    )


def create_app_url_encoded_optional_field_missing_success() -> Spikard:
    """App factory for fixture: Optional field missing - success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
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
        parameter_schema=None,
        file_params=None,
    )(url_encoded_optional_field_missing_success)
    return app


class UrlEncoded14NestedObjectBracketNotationBody(msgspec.Struct):
    """Request body msgspec.Struct (fast typed)."""

    user: dict[str, Any]


def url_encoded_14_nested_object_bracket_notation(
    body: UrlEncoded14NestedObjectBracketNotationBody,
) -> Any:
    """Handler for POST /profile."""
    return Response(
        content={"user": {"age": 30, "email": "john@example.com", "name": "John Doe"}},
        status_code=201,
        headers={"Content-Type": "application/json"},
    )


def create_app_url_encoded_14_nested_object_bracket_notation() -> Spikard:
    """App factory for fixture: 14_nested_object_bracket_notation"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
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
        parameter_schema=None,
        file_params=None,
    )(url_encoded_14_nested_object_bracket_notation)
    return app


class UrlEncodedStringMaxLengthValidationFailBody(BaseModel):
    """Request body Pydantic model."""

    username: str


def url_encoded_string_max_length_validation_fail(
    body: UrlEncodedStringMaxLengthValidationFailBody,
) -> Any:
    """Handler for POST /form/validated."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"max_length": 20},
                    "input": "this_is_a_very_long_username_that_exceeds_limit",
                    "loc": ["body", "username"],
                    "msg": "String should have at most 20 characters",
                    "type": "string_too_long",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_url_encoded_string_max_length_validation_fail() -> Spikard:
    """App factory for fixture: String max_length validation - fail"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/form/validated",
        body_schema={
            "properties": {"username": {"maxLength": 20, "type": "string"}},
            "required": ["username"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(url_encoded_string_max_length_validation_fail)
    return app


def url_encoded_18_integer_minimum_validation_failure(
    body: dict[str, Any],
) -> Any:
    """Handler for POST /products."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"actual_value": 0, "minimum": 1},
                    "loc": ["body", "quantity"],
                    "msg": "Value must be at least 1",
                    "type": "validation_error",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_url_encoded_18_integer_minimum_validation_failure() -> Spikard:
    """App factory for fixture: 18_integer_minimum_validation_failure"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/products",
        body_schema={
            "properties": {"quantity": {"minimum": 1, "type": "integer"}},
            "required": ["quantity"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(url_encoded_18_integer_minimum_validation_failure)
    return app


class UrlEncoded21IntegerTypeCoercionFailureBody(TypedDict):
    """Request body type (TypedDict - runtime is dict)."""

    price: int


def url_encoded_21_integer_type_coercion_failure(
    body: UrlEncoded21IntegerTypeCoercionFailureBody,
) -> Any:
    """Handler for POST /products."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"value": "not-a-number"},
                    "loc": ["body", "price"],
                    "msg": "Value is not a valid integer",
                    "type": "validation_error",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_url_encoded_21_integer_type_coercion_failure() -> Spikard:
    """App factory for fixture: 21_integer_type_coercion_failure"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/products",
        body_schema={"properties": {"price": {"type": "integer"}}, "required": ["price"], "type": "object"},
        parameter_schema=None,
        file_params=None,
    )(url_encoded_21_integer_type_coercion_failure)
    return app


@dataclass
class UrlEncoded16MinlengthValidationFailureBody:
    """Request body dataclass."""

    username: str


def url_encoded_16_minlength_validation_failure(
    body: UrlEncoded16MinlengthValidationFailureBody,
) -> Any:
    """Handler for POST /users."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"actual_length": 2, "min_length": 3, "value": "ab"},
                    "loc": ["body", "username"],
                    "msg": "String length must be at least 3",
                    "type": "validation_error",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_url_encoded_16_minlength_validation_failure() -> Spikard:
    """App factory for fixture: 16_minlength_validation_failure"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/users",
        body_schema={
            "properties": {"username": {"minLength": 3, "type": "string"}},
            "required": ["username"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(url_encoded_16_minlength_validation_failure)
    return app


class UrlEncodedStringMinLengthValidationFailBody(NamedTuple):
    """Request body NamedTuple (immutable)."""

    username: str


def url_encoded_string_min_length_validation_fail(
    body: UrlEncodedStringMinLengthValidationFailBody,
) -> Any:
    """Handler for POST /form/validated."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"min_length": 3},
                    "input": "ab",
                    "loc": ["body", "username"],
                    "msg": "String should have at least 3 characters",
                    "type": "string_too_short",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_url_encoded_string_min_length_validation_fail() -> Spikard:
    """App factory for fixture: String min_length validation - fail"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/form/validated",
        body_schema={
            "properties": {"username": {"minLength": 3, "type": "string"}},
            "required": ["username"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(url_encoded_string_min_length_validation_fail)
    return app


def multipart_multiple_values_for_same_field_name(
    body: dict[str, Any],
) -> Any:
    """Handler for POST /."""
    return Response(
        content={
            "files": [
                {"content": "first file", "content_type": "text/plain", "filename": "file1.txt", "size": 10},
                {"content": "second file", "content_type": "text/plain", "filename": "file2.txt", "size": 11},
            ],
            "tags": ["python", "rust", "web"],
        },
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_multipart_multiple_values_for_same_field_name() -> Spikard:
    """App factory for fixture: Multiple values for same field name"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
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
        parameter_schema=None,
        file_params=None,
    )(multipart_multiple_values_for_same_field_name)
    return app


def multipart_19_file_mime_spoofing_png_as_jpeg() -> Any:
    """Handler for POST /upload."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {
                        "declared_mime": "image/jpeg",
                        "detected_type": "image/png",
                        "magic_bytes": "89504e470d0a1a0a",
                    },
                    "loc": ["files", "image"],
                    "msg": "File type mismatch: MIME type is image/jpeg but magic numbers indicate image/png",
                    "type": "validation_error",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_multipart_19_file_mime_spoofing_png_as_jpeg() -> Spikard:
    """App factory for fixture: 19_file_mime_spoofing_png_as_jpeg"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/upload",
        body_schema=None,
        parameter_schema={"properties": {}, "required": [], "type": "object"},
        file_params={"image": {"content_type": ["image/jpeg"], "required": True, "validate_magic_numbers": True}},
    )(multipart_19_file_mime_spoofing_png_as_jpeg)
    return app


def multipart_20_file_mime_spoofing_jpeg_as_png() -> Any:
    """Handler for POST /upload."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"declared_mime": "image/png", "detected_type": "image/jpeg", "magic_bytes": "ffd8ffe0"},
                    "loc": ["files", "image"],
                    "msg": "File type mismatch: MIME type is image/png but magic numbers indicate image/jpeg",
                    "type": "validation_error",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_multipart_20_file_mime_spoofing_jpeg_as_png() -> Spikard:
    """App factory for fixture: 20_file_mime_spoofing_jpeg_as_png"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/upload",
        body_schema=None,
        parameter_schema={"properties": {}, "required": [], "type": "object"},
        file_params={"image": {"content_type": ["image/png"], "required": True, "validate_magic_numbers": True}},
    )(multipart_20_file_mime_spoofing_jpeg_as_png)
    return app


def multipart_21_file_pdf_magic_number_success() -> Any:
    """Handler for POST /upload."""
    return Response(status_code=201)


def create_app_multipart_21_file_pdf_magic_number_success() -> Spikard:
    """App factory for fixture: 21_file_pdf_magic_number_success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/upload",
        body_schema=None,
        parameter_schema={"properties": {}, "required": [], "type": "object"},
        file_params={
            "document": {"content_type": ["application/pdf"], "required": True, "validate_magic_numbers": True}
        },
    )(multipart_21_file_pdf_magic_number_success)
    return app


class MultipartContentTypeValidationInvalidTypeBody(msgspec.Struct):
    """Request body msgspec.Struct (fast typed)."""

    file: str | None = None


def multipart_content_type_validation_invalid_type(
    body: MultipartContentTypeValidationInvalidTypeBody,
) -> Any:
    """Handler for POST /files/images-only."""
    return Response(
        content={
            "errors": [
                {
                    "loc": ["files", "file"],
                    "msg": "Invalid content type 'application/x-sh'. Allowed types: image/jpeg, image/png, image/gif",
                    "type": "validation_error",
                }
            ]
        },
        status_code=422,
    )


def create_app_multipart_content_type_validation_invalid_type() -> Spikard:
    """App factory for fixture: Content-Type validation - invalid type"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/files/images-only",
        body_schema={
            "additionalProperties": False,
            "properties": {"file": {"format": "binary", "type": "string"}},
            "type": "object",
        },
        parameter_schema={"properties": {}, "required": [], "type": "object"},
        file_params={"file": {"content_type": ["image/jpeg", "image/png", "image/gif"], "required": True}},
    )(multipart_content_type_validation_invalid_type)
    return app


class MultipartPdfFileUploadBody(BaseModel):
    """Request body Pydantic model."""

    document: str


def multipart_pdf_file_upload(
    body: MultipartPdfFileUploadBody,
) -> Any:
    """Handler for POST /files/document."""
    return Response(
        content={"content_type": "application/pdf", "filename": "report.pdf", "size": 16},
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_multipart_pdf_file_upload() -> Spikard:
    """App factory for fixture: PDF file upload"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/files/document",
        body_schema={
            "additionalProperties": False,
            "properties": {"document": {"format": "binary", "type": "string"}},
            "required": ["document"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(multipart_pdf_file_upload)
    return app


def multipart_file_list_upload_array_of_files(
    body: dict[str, Any],
) -> Any:
    """Handler for POST /files/list."""
    return Response(
        content={"filenames": ["file1.txt", "file2.txt"], "total_size": 35},
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_multipart_file_list_upload_array_of_files() -> Spikard:
    """App factory for fixture: File list upload (array of files)"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/files/list",
        body_schema={
            "additionalProperties": False,
            "properties": {"files": {"items": {"format": "binary", "type": "string"}, "type": "array"}},
            "required": ["files"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(multipart_file_list_upload_array_of_files)
    return app


class MultipartOptionalFileUploadProvidedBody(TypedDict):
    """Request body type (TypedDict - runtime is dict)."""

    file: str


def multipart_optional_file_upload_provided(
    body: MultipartOptionalFileUploadProvidedBody,
) -> Any:
    """Handler for POST /files/optional."""
    return Response(
        content={"content_type": "text/plain", "filename": "optional.txt", "size": 21},
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_multipart_optional_file_upload_provided() -> Spikard:
    """App factory for fixture: Optional file upload - provided"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/files/optional",
        body_schema={
            "additionalProperties": False,
            "properties": {"file": {"format": "binary", "type": "string"}},
            "required": ["file"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(multipart_optional_file_upload_provided)
    return app


@dataclass
class MultipartFileSizeValidationTooLargeBody:
    """Request body dataclass."""

    file: str | None = None


def multipart_file_size_validation_too_large(
    body: MultipartFileSizeValidationTooLargeBody,
) -> Any:
    """Handler for POST /files/validated."""
    return Response(
        content={"detail": "File too large. Maximum size is 1MB"},
        status_code=413,
        headers={"Content-Type": "application/json"},
    )


def create_app_multipart_file_size_validation_too_large() -> Spikard:
    """App factory for fixture: File size validation - too large"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/files/validated",
        body_schema={
            "additionalProperties": False,
            "properties": {"file": {"format": "binary", "type": "string"}},
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(multipart_file_size_validation_too_large)
    return app


class MultipartMixedFilesAndFormDataBody(NamedTuple):
    """Request body NamedTuple (immutable)."""

    file: str
    active: str | None = None
    age: str | None = None
    username: str | None = None


def multipart_mixed_files_and_form_data(
    body: MultipartMixedFilesAndFormDataBody,
) -> Any:
    """Handler for POST /."""
    return Response(
        content={
            "active": "true",
            "age": "25",
            "file": {"content": "file data here", "content_type": "text/plain", "filename": "upload.txt", "size": 14},
            "username": "testuser",
        },
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_multipart_mixed_files_and_form_data() -> Spikard:
    """App factory for fixture: Mixed files and form data"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/",
        body_schema={
            "additionalProperties": False,
            "properties": {
                "active": {"type": "string"},
                "age": {"type": "string"},
                "file": {"format": "binary", "type": "string"},
                "username": {"type": "string"},
            },
            "required": ["file"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(multipart_mixed_files_and_form_data)
    return app


class MultipartSimpleFileUploadBody(msgspec.Struct):
    """Request body msgspec.Struct (fast typed)."""

    test: str


def multipart_simple_file_upload(
    body: MultipartSimpleFileUploadBody,
) -> Any:
    """Handler for POST /."""
    return Response(
        content={
            "test": {"content": "<file content>", "content_type": "text/plain", "filename": "test.txt", "size": 14}
        },
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_multipart_simple_file_upload() -> Spikard:
    """App factory for fixture: Simple file upload"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/",
        body_schema={
            "additionalProperties": False,
            "properties": {"test": {"format": "binary", "type": "string"}},
            "required": ["test"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(multipart_simple_file_upload)
    return app


class MultipartEmptyFileUploadBody(BaseModel):
    """Request body Pydantic model."""

    file: str


def multipart_empty_file_upload(
    body: MultipartEmptyFileUploadBody,
) -> Any:
    """Handler for POST /files/upload."""
    return Response(
        content={"filename": "empty.txt", "size": 0}, status_code=200, headers={"Content-Type": "application/json"}
    )


def create_app_multipart_empty_file_upload() -> Spikard:
    """App factory for fixture: Empty file upload"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/files/upload",
        body_schema={
            "additionalProperties": False,
            "properties": {"file": {"format": "binary", "type": "string"}},
            "required": ["file"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(multipart_empty_file_upload)
    return app


def multipart_optional_file_upload_missing(
    body: dict[str, Any],
) -> Any:
    """Handler for POST /files/optional."""
    return Response(content={"file": None}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_multipart_optional_file_upload_missing() -> Spikard:
    """App factory for fixture: Optional file upload - missing"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/files/optional",
        body_schema={"additionalProperties": False, "properties": {}, "type": "object"},
        parameter_schema=None,
        file_params=None,
    )(multipart_optional_file_upload_missing)
    return app


class MultipartFileUploadWithoutFilenameBody(TypedDict):
    """Request body type (TypedDict - runtime is dict)."""

    test1: str


def multipart_file_upload_without_filename(
    body: MultipartFileUploadWithoutFilenameBody,
) -> Any:
    """Handler for POST /."""
    return Response(content={"test1": "<file1 content>"}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_multipart_file_upload_without_filename() -> Spikard:
    """App factory for fixture: File upload without filename"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/",
        body_schema={
            "additionalProperties": False,
            "properties": {"test1": {"format": "binary", "type": "string"}},
            "required": ["test1"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(multipart_file_upload_without_filename)
    return app


def multipart_18_file_magic_number_jpeg_success() -> Any:
    """Handler for POST /upload."""
    return Response(status_code=201)


def create_app_multipart_18_file_magic_number_jpeg_success() -> Spikard:
    """App factory for fixture: 18_file_magic_number_jpeg_success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/upload",
        body_schema=None,
        parameter_schema={"properties": {}, "required": [], "type": "object"},
        file_params={"image": {"content_type": ["image/jpeg"], "required": True, "validate_magic_numbers": True}},
    )(multipart_18_file_magic_number_jpeg_success)
    return app


def multipart_22_file_empty_buffer() -> Any:
    """Handler for POST /upload."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [
                {
                    "ctx": {"buffer_size": 0},
                    "loc": ["files", "file"],
                    "msg": "File buffer is empty",
                    "type": "validation_error",
                }
            ],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_multipart_22_file_empty_buffer() -> Spikard:
    """App factory for fixture: 22_file_empty_buffer"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/upload",
        body_schema=None,
        parameter_schema={"properties": {}, "required": [], "type": "object"},
        file_params={"file": {"required": True, "validate_magic_numbers": True}},
    )(multipart_22_file_empty_buffer)
    return app


def multipart_17_file_magic_number_png_success() -> Any:
    """Handler for POST /upload."""
    return Response(status_code=201)


def create_app_multipart_17_file_magic_number_png_success() -> Spikard:
    """App factory for fixture: 17_file_magic_number_png_success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/upload",
        body_schema=None,
        parameter_schema={"properties": {}, "required": [], "type": "object"},
        file_params={"image": {"content_type": ["image/png"], "required": True, "validate_magic_numbers": True}},
    )(multipart_17_file_magic_number_png_success)
    return app


class MultipartFormDataWithoutFilesBody(BaseModel):
    """Request body Pydantic model."""

    some: str | None = None


def multipart_form_data_without_files(
    body: MultipartFormDataWithoutFilesBody,
) -> Any:
    """Handler for POST /."""
    return Response(content={"some": "data"}, status_code=200, headers={"Content-Type": "application/json"})


def create_app_multipart_form_data_without_files() -> Spikard:
    """App factory for fixture: Form data without files"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/",
        body_schema={"additionalProperties": False, "properties": {"some": {"type": "string"}}, "type": "object"},
        parameter_schema=None,
        file_params=None,
    )(multipart_form_data_without_files)
    return app


def multipart_multiple_file_uploads(
    body: dict[str, Any],
) -> Any:
    """Handler for POST /."""
    return Response(
        content={
            "test1": {"content": "<file1 content>", "content_type": "text/plain", "filename": "test1.txt", "size": 15},
            "test2": {"content": "<file2 content>", "content_type": "text/plain", "filename": "test2.txt", "size": 15},
        },
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_multipart_multiple_file_uploads() -> Spikard:
    """App factory for fixture: Multiple file uploads"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/",
        body_schema={
            "additionalProperties": False,
            "properties": {
                "test1": {"format": "binary", "type": "string"},
                "test2": {"format": "binary", "type": "string"},
            },
            "required": ["test1", "test2"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(multipart_multiple_file_uploads)
    return app


class MultipartFileUploadWithCustomHeadersBody(TypedDict):
    """Request body type (TypedDict - runtime is dict)."""

    test2: str


def multipart_file_upload_with_custom_headers(
    body: MultipartFileUploadWithCustomHeadersBody,
) -> Any:
    """Handler for POST /."""
    return Response(
        content={
            "test2": {
                "content": "<file2 content>",
                "content_type": "text/plain",
                "filename": "test2.txt",
                "headers": [
                    ["content-disposition", 'form-data; name="test2"; filename="test2.txt"'],
                    ["content-type", "text/plain"],
                    ["x-custom", "f2"],
                ],
                "size": 15,
            }
        },
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_multipart_file_upload_with_custom_headers() -> Spikard:
    """App factory for fixture: File upload with custom headers"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/",
        body_schema={
            "additionalProperties": False,
            "properties": {"test2": {"format": "binary", "type": "string"}},
            "required": ["test2"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(multipart_file_upload_with_custom_headers)
    return app


@dataclass
class MultipartRequiredFileUploadMissingBody:
    """Request body dataclass."""

    file: str


def multipart_required_file_upload_missing(
    body: MultipartRequiredFileUploadMissingBody,
) -> Any:
    """Handler for POST /files/required."""
    return Response(
        content={
            "detail": "1 validation error in request",
            "errors": [{"input": "required", "loc": ["body", "file"], "msg": "Field required", "type": "missing"}],
            "status": 422,
            "title": "Request Validation Failed",
            "type": "https://spikard.dev/errors/validation-error",
        },
        status_code=422,
        headers={"Content-Type": "application/json"},
    )


def create_app_multipart_required_file_upload_missing() -> Spikard:
    """App factory for fixture: Required file upload - missing"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/files/required",
        body_schema={
            "additionalProperties": False,
            "properties": {"file": {"format": "binary", "type": "string"}},
            "required": ["file"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(multipart_required_file_upload_missing)
    return app


class MultipartImageFileUploadBody(NamedTuple):
    """Request body NamedTuple (immutable)."""

    image: str


def multipart_image_file_upload(
    body: MultipartImageFileUploadBody,
) -> Any:
    """Handler for POST /files/image."""
    return Response(
        content={"content_type": "image/jpeg", "filename": "photo.jpg", "size": 22},
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_multipart_image_file_upload() -> Spikard:
    """App factory for fixture: Image file upload"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/files/image",
        body_schema={
            "additionalProperties": False,
            "properties": {"image": {"format": "binary", "type": "string"}},
            "required": ["image"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(multipart_image_file_upload)
    return app


async def lifecycle_hooks_onresponse_security_headers_security_headers_on_response_0(response: Any) -> Any:
    """onResponse hook: security_headers - Adds security headers"""
    if hasattr(response, "headers"):
        response.headers["X-Content-Type-Options"] = "nosniff"
        response.headers["X-Frame-Options"] = "DENY"
        response.headers["X-XSS-Protection"] = "1; mode=block"
        response.headers["Strict-Transport-Security"] = "max-age=31536000; includeSubDomains"
    return response


def lifecycle_hooks_onresponse_security_headers() -> Any:
    """Handler for GET /api/test-security-headers."""
    return Response(
        content={"message": "Response with security headers"},
        status_code=200,
        headers={
            "X-XSS-Protection": "1; mode=block",
            "X-Frame-Options": "DENY",
            "Content-Type": "application/json",
            "X-Content-Type-Options": "nosniff",
            "Strict-Transport-Security": "max-age=31536000; includeSubDomains",
        },
    )


def create_app_lifecycle_hooks_onresponse_security_headers() -> Spikard:
    """App factory for fixture: onResponse - Security Headers"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route("GET", "/api/test-security-headers", body_schema=None, parameter_schema=None, file_params=None)(
        lifecycle_hooks_onresponse_security_headers
    )
    # Register lifecycle hooks
    app.on_response(lifecycle_hooks_onresponse_security_headers_security_headers_on_response_0)
    return app


async def lifecycle_hooks_prehandler_authentication_failed_short_circuit_authenticator_pre_handler_0(
    request: Any,
) -> Any:
    """preHandler hook: authenticator - Short circuits with 401"""
    from spikard import Response

    return Response(
        content={"error": "Unauthorized", "message": "Invalid or expired authentication token"}, status_code=401
    )


def lifecycle_hooks_prehandler_authentication_failed_short_circuit() -> Any:
    """Handler for GET /api/protected-resource-fail."""
    return Response(
        content={"error": "Unauthorized", "message": "Invalid or expired authentication token"},
        status_code=401,
        headers={"Content-Type": "application/json"},
    )


def create_app_lifecycle_hooks_prehandler_authentication_failed_short_circuit() -> Spikard:
    """App factory for fixture: preHandler - Authentication Failed (Short Circuit)"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "GET", "/api/protected-resource-fail", body_schema=None, parameter_schema=None, file_params=None
    )(lifecycle_hooks_prehandler_authentication_failed_short_circuit)
    # Register lifecycle hooks
    app.pre_handler(lifecycle_hooks_prehandler_authentication_failed_short_circuit_authenticator_pre_handler_0)
    return app


async def lifecycle_hooks_prehandler_authorization_check_authenticator_pre_handler_0(request: Any) -> Any:
    """preHandler hook: authenticator"""
    # Mock implementation for testing
    return request


async def lifecycle_hooks_prehandler_authorization_check_authorizer_pre_handler_1(request: Any) -> Any:
    """preHandler hook: authorizer"""
    # Mock implementation for testing
    return request


def lifecycle_hooks_prehandler_authorization_check() -> Any:
    """Handler for GET /api/admin-only."""
    return Response(
        content={"message": "Admin access granted", "role": "admin", "user_id": "admin-456"},
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_lifecycle_hooks_prehandler_authorization_check() -> Spikard:
    """App factory for fixture: preHandler - Authorization Check"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route("GET", "/api/admin-only", body_schema=None, parameter_schema=None, file_params=None)(
        lifecycle_hooks_prehandler_authorization_check
    )
    # Register lifecycle hooks
    app.pre_handler(lifecycle_hooks_prehandler_authorization_check_authenticator_pre_handler_0)
    app.pre_handler(lifecycle_hooks_prehandler_authorization_check_authorizer_pre_handler_1)
    return app


async def lifecycle_hooks_prehandler_authentication_success_authenticator_pre_handler_0(request: Any) -> Any:
    """preHandler hook: authenticator"""
    # Mock implementation for testing
    return request


def lifecycle_hooks_prehandler_authentication_success() -> Any:
    """Handler for GET /api/protected-resource."""
    return Response(
        content={"authenticated": True, "message": "Access granted", "user_id": "user-123"},
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_lifecycle_hooks_prehandler_authentication_success() -> Spikard:
    """App factory for fixture: preHandler - Authentication Success"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route("GET", "/api/protected-resource", body_schema=None, parameter_schema=None, file_params=None)(
        lifecycle_hooks_prehandler_authentication_success
    )
    # Register lifecycle hooks
    app.pre_handler(lifecycle_hooks_prehandler_authentication_success_authenticator_pre_handler_0)
    return app


class LifecycleHooksPrevalidationRateLimitExceededShortCircuitBody(msgspec.Struct):
    """Request body msgspec.Struct (fast typed)."""

    data: str


async def lifecycle_hooks_prevalidation_rate_limit_exceeded_short_circuit_rate_limiter_pre_validation_0(
    request: Any,
) -> Any:
    """preValidation hook: rate_limiter - Short circuits with 429"""
    from spikard import Response

    return Response(
        content={"error": "Rate limit exceeded", "message": "Too many requests, please try again later"},
        status_code=429,
        headers={"Retry-After": "60"},
    )


def lifecycle_hooks_prevalidation_rate_limit_exceeded_short_circuit(
    body: LifecycleHooksPrevalidationRateLimitExceededShortCircuitBody,
) -> Any:
    """Handler for POST /api/test-rate-limit-exceeded."""
    return Response(
        content={"error": "Rate limit exceeded", "message": "Too many requests, please try again later"},
        status_code=429,
        headers={"Content-Type": "application/json", "Retry-After": "60"},
    )


def create_app_lifecycle_hooks_prevalidation_rate_limit_exceeded_short_circuit() -> Spikard:
    """App factory for fixture: preValidation - Rate Limit Exceeded (Short Circuit)"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/api/test-rate-limit-exceeded",
        body_schema={"properties": {"data": {"type": "string"}}, "required": ["data"], "type": "object"},
        parameter_schema=None,
        file_params=None,
    )(lifecycle_hooks_prevalidation_rate_limit_exceeded_short_circuit)
    # Register lifecycle hooks
    app.pre_validation(lifecycle_hooks_prevalidation_rate_limit_exceeded_short_circuit_rate_limiter_pre_validation_0)
    return app


async def lifecycle_hooks_onerror_error_logging_error_logger_on_error_0(response: Any) -> Any:
    """onError hook: error_logger"""
    # Mock implementation for testing - format error response
    if hasattr(response, "headers"):
        response.headers["Content-Type"] = "application/json"
    return response


async def lifecycle_hooks_onerror_error_logging_error_formatter_on_error_1(response: Any) -> Any:
    """onError hook: error_formatter"""
    # Mock implementation for testing - format error response
    if hasattr(response, "headers"):
        response.headers["Content-Type"] = "application/json"
    return response


def lifecycle_hooks_onerror_error_logging() -> Any:
    """Handler for GET /api/test-error."""
    return Response(
        content={"error": "Internal Server Error", "error_id": ".*", "message": "An unexpected error occurred"},
        status_code=500,
        headers={"Content-Type": "application/json"},
    )


def create_app_lifecycle_hooks_onerror_error_logging() -> Spikard:
    """App factory for fixture: onError - Error Logging"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route("GET", "/api/test-error", body_schema=None, parameter_schema=None, file_params=None)(
        lifecycle_hooks_onerror_error_logging
    )
    # Register lifecycle hooks
    app.on_error(lifecycle_hooks_onerror_error_logging_error_logger_on_error_0)
    app.on_error(lifecycle_hooks_onerror_error_logging_error_formatter_on_error_1)
    return app


async def lifecycle_hooks_multiple_hooks_all_phases_request_logger_on_request_0(request: Any) -> Any:
    """onRequest hook: request_logger"""
    # Mock implementation for testing
    return request


async def lifecycle_hooks_multiple_hooks_all_phases_request_id_generator_on_request_1(request: Any) -> Any:
    """onRequest hook: request_id_generator"""
    # Mock implementation for testing
    return request


async def lifecycle_hooks_multiple_hooks_all_phases_rate_limiter_pre_validation_0(request: Any) -> Any:
    """preValidation hook: rate_limiter"""
    # Mock implementation for testing
    return request


async def lifecycle_hooks_multiple_hooks_all_phases_authenticator_pre_handler_0(request: Any) -> Any:
    """preHandler hook: authenticator"""
    # Mock implementation for testing
    return request


async def lifecycle_hooks_multiple_hooks_all_phases_authorizer_pre_handler_1(request: Any) -> Any:
    """preHandler hook: authorizer"""
    # Mock implementation for testing
    return request


async def lifecycle_hooks_multiple_hooks_all_phases_security_headers_on_response_0(response: Any) -> Any:
    """onResponse hook: security_headers - Adds security headers"""
    if hasattr(response, "headers"):
        response.headers["X-Content-Type-Options"] = "nosniff"
        response.headers["X-Frame-Options"] = "DENY"
        response.headers["X-XSS-Protection"] = "1; mode=block"
        response.headers["Strict-Transport-Security"] = "max-age=31536000; includeSubDomains"
    return response


async def lifecycle_hooks_multiple_hooks_all_phases_response_timer_on_response_1(response: Any) -> Any:
    """onResponse hook: response_timer - Adds timing header"""
    if hasattr(response, "headers"):
        response.headers["X-Response-Time"] = "0ms"
    return response


async def lifecycle_hooks_multiple_hooks_all_phases_audit_logger_on_response_2(response: Any) -> Any:
    """onResponse hook: audit_logger"""
    # Mock implementation for testing
    return response


async def lifecycle_hooks_multiple_hooks_all_phases_error_logger_on_error_0(response: Any) -> Any:
    """onError hook: error_logger"""
    # Mock implementation for testing - format error response
    if hasattr(response, "headers"):
        response.headers["Content-Type"] = "application/json"
    return response


def lifecycle_hooks_multiple_hooks_all_phases(
    body: dict[str, Any],
) -> Any:
    """Handler for POST /api/full-lifecycle."""
    return Response(
        content={
            "action": "update_profile",
            "message": "Action completed successfully",
            "request_id": ".*",
            "user_id": "user-123",
        },
        status_code=200,
        headers={
            "Content-Type": "application/json",
            "X-Response-Time": ".*ms",
            "X-Frame-Options": "DENY",
            "X-Request-ID": ".*",
            "X-Content-Type-Options": "nosniff",
        },
    )


def create_app_lifecycle_hooks_multiple_hooks_all_phases() -> Spikard:
    """App factory for fixture: Multiple Hooks - All Phases"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/api/full-lifecycle",
        body_schema={
            "properties": {"action": {"type": "string"}, "user_id": {"type": "string"}},
            "required": ["user_id", "action"],
            "type": "object",
        },
        parameter_schema=None,
        file_params=None,
    )(lifecycle_hooks_multiple_hooks_all_phases)
    # Register lifecycle hooks
    app.on_request(lifecycle_hooks_multiple_hooks_all_phases_request_logger_on_request_0)
    app.on_request(lifecycle_hooks_multiple_hooks_all_phases_request_id_generator_on_request_1)
    app.pre_validation(lifecycle_hooks_multiple_hooks_all_phases_rate_limiter_pre_validation_0)
    app.pre_handler(lifecycle_hooks_multiple_hooks_all_phases_authenticator_pre_handler_0)
    app.pre_handler(lifecycle_hooks_multiple_hooks_all_phases_authorizer_pre_handler_1)
    app.on_response(lifecycle_hooks_multiple_hooks_all_phases_security_headers_on_response_0)
    app.on_response(lifecycle_hooks_multiple_hooks_all_phases_response_timer_on_response_1)
    app.on_response(lifecycle_hooks_multiple_hooks_all_phases_audit_logger_on_response_2)
    app.on_error(lifecycle_hooks_multiple_hooks_all_phases_error_logger_on_error_0)
    return app


async def lifecycle_hooks_hook_execution_order_first_hook_on_request_0(request: Any) -> Any:
    """onRequest hook: first_hook"""
    # Mock implementation for testing
    return request


async def lifecycle_hooks_hook_execution_order_second_hook_on_request_1(request: Any) -> Any:
    """onRequest hook: second_hook"""
    # Mock implementation for testing
    return request


async def lifecycle_hooks_hook_execution_order_third_hook_on_request_2(request: Any) -> Any:
    """onRequest hook: third_hook"""
    # Mock implementation for testing
    return request


def lifecycle_hooks_hook_execution_order() -> Any:
    """Handler for GET /api/test-hook-order."""
    return Response(
        content={"execution_order": ["first_hook", "second_hook", "third_hook"], "message": "Hooks executed in order"},
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_lifecycle_hooks_hook_execution_order() -> Spikard:
    """App factory for fixture: Hook Execution Order"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route("GET", "/api/test-hook-order", body_schema=None, parameter_schema=None, file_params=None)(
        lifecycle_hooks_hook_execution_order
    )
    # Register lifecycle hooks
    app.on_request(lifecycle_hooks_hook_execution_order_first_hook_on_request_0)
    app.on_request(lifecycle_hooks_hook_execution_order_second_hook_on_request_1)
    app.on_request(lifecycle_hooks_hook_execution_order_third_hook_on_request_2)
    return app


async def lifecycle_hooks_onresponse_response_timing_start_timer_on_request_0(request: Any) -> Any:
    """onRequest hook: start_timer"""
    # Mock implementation for testing
    return request


async def lifecycle_hooks_onresponse_response_timing_response_timer_on_response_0(response: Any) -> Any:
    """onResponse hook: response_timer - Adds timing header"""
    if hasattr(response, "headers"):
        response.headers["X-Response-Time"] = "0ms"
    return response


def lifecycle_hooks_onresponse_response_timing() -> Any:
    """Handler for GET /api/test-timing."""
    return Response(
        content={"message": "Response with timing info"},
        status_code=200,
        headers={"Content-Type": "application/json", "X-Response-Time": ".*ms"},
    )


def create_app_lifecycle_hooks_onresponse_response_timing() -> Spikard:
    """App factory for fixture: onResponse - Response Timing"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route("GET", "/api/test-timing", body_schema=None, parameter_schema=None, file_params=None)(
        lifecycle_hooks_onresponse_response_timing
    )
    # Register lifecycle hooks
    app.on_request(lifecycle_hooks_onresponse_response_timing_start_timer_on_request_0)
    app.on_response(lifecycle_hooks_onresponse_response_timing_response_timer_on_response_0)
    return app


async def lifecycle_hooks_prehandler_authorization_forbidden_short_circuit_authenticator_pre_handler_0(
    request: Any,
) -> Any:
    """preHandler hook: authenticator - Short circuits with 403"""
    from spikard import Response

    return Response(content={"error": "Forbidden", "message": "Admin role required for this endpoint"}, status_code=403)


async def lifecycle_hooks_prehandler_authorization_forbidden_short_circuit_authorizer_pre_handler_1(
    request: Any,
) -> Any:
    """preHandler hook: authorizer - Short circuits with 403"""
    from spikard import Response

    return Response(content={"error": "Forbidden", "message": "Admin role required for this endpoint"}, status_code=403)


def lifecycle_hooks_prehandler_authorization_forbidden_short_circuit() -> Any:
    """Handler for GET /api/admin-only-forbidden."""
    return Response(
        content={"error": "Forbidden", "message": "Admin role required for this endpoint"},
        status_code=403,
        headers={"Content-Type": "application/json"},
    )


def create_app_lifecycle_hooks_prehandler_authorization_forbidden_short_circuit() -> Spikard:
    """App factory for fixture: preHandler - Authorization Forbidden (Short Circuit)"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route("GET", "/api/admin-only-forbidden", body_schema=None, parameter_schema=None, file_params=None)(
        lifecycle_hooks_prehandler_authorization_forbidden_short_circuit
    )
    # Register lifecycle hooks
    app.pre_handler(lifecycle_hooks_prehandler_authorization_forbidden_short_circuit_authenticator_pre_handler_0)
    app.pre_handler(lifecycle_hooks_prehandler_authorization_forbidden_short_circuit_authorizer_pre_handler_1)
    return app


async def lifecycle_hooks_onrequest_request_logging_request_logger_on_request_0(request: Any) -> Any:
    """onRequest hook: request_logger"""
    # Mock implementation for testing
    return request


async def lifecycle_hooks_onrequest_request_logging_request_id_generator_on_request_1(request: Any) -> Any:
    """onRequest hook: request_id_generator"""
    # Mock implementation for testing
    return request


def lifecycle_hooks_onrequest_request_logging() -> Any:
    """Handler for GET /api/test-on-request."""
    return Response(
        content={"has_request_id": True, "message": "onRequest hooks executed", "request_logged": True},
        status_code=200,
        headers={"X-Request-ID": ".*", "Content-Type": "application/json"},
    )


def create_app_lifecycle_hooks_onrequest_request_logging() -> Spikard:
    """App factory for fixture: onRequest - Request Logging"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route("GET", "/api/test-on-request", body_schema=None, parameter_schema=None, file_params=None)(
        lifecycle_hooks_onrequest_request_logging
    )
    # Register lifecycle hooks
    app.on_request(lifecycle_hooks_onrequest_request_logging_request_logger_on_request_0)
    app.on_request(lifecycle_hooks_onrequest_request_logging_request_id_generator_on_request_1)
    return app


class LifecycleHooksPrevalidationRateLimitingBody(BaseModel):
    """Request body Pydantic model."""

    data: str


async def lifecycle_hooks_prevalidation_rate_limiting_rate_limiter_pre_validation_0(request: Any) -> Any:
    """preValidation hook: rate_limiter"""
    # Mock implementation for testing
    return request


def lifecycle_hooks_prevalidation_rate_limiting(
    body: LifecycleHooksPrevalidationRateLimitingBody,
) -> Any:
    """Handler for POST /api/test-rate-limit."""
    return Response(
        content={"message": "Request accepted", "rate_limit_checked": True},
        status_code=200,
        headers={"Content-Type": "application/json"},
    )


def create_app_lifecycle_hooks_prevalidation_rate_limiting() -> Spikard:
    """App factory for fixture: preValidation - Rate Limiting"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route(
        "POST",
        "/api/test-rate-limit",
        body_schema={"properties": {"data": {"type": "string"}}, "required": ["data"], "type": "object"},
        parameter_schema=None,
        file_params=None,
    )(lifecycle_hooks_prevalidation_rate_limiting)
    # Register lifecycle hooks
    app.pre_validation(lifecycle_hooks_prevalidation_rate_limiting_rate_limiter_pre_validation_0)
    return app


def create_app_sse_notifications() -> Spikard:
    """SSE channel for /notifications"""
    app = Spikard()

    @sse("/notifications")
    async def sse_handler_notifications():
        """SSE event stream for /notifications."""
        events = [
            '{"level":"example_level","message":"example_message","source":"example_source","timestamp":"2024-01-15T10:30:00Z","type":"system_alert"}',
            '{"body":"example_body","priority":"example_priority","timestamp":"2024-01-15T10:30:00Z","title":"example_title","type":"user_notification","userId":"example_userId"}',
            '{"message":"example_message","metadata":{},"service":"example_service","status":"example_status","timestamp":"2024-01-15T10:30:00Z","type":"status_update"}',
        ]
        for event_data in events:
            yield json.loads(event_data)

    return app


def create_app_websocket_chat() -> Spikard:
    """WebSocket channel for /chat"""
    app = Spikard()

    @websocket("/chat")
    async def websocket_handler_chat(message: dict) -> dict | None:
        """WebSocket handler for /chat - echoes messages with validation."""
        message["validated"] = True
        return message

    return app


# App factory functions:
# - create_app_compression_compression_payload_below_min_size_is_not_compressed() for compression / Compression - payload below min_size is not compressed
# - create_app_compression_compression_gzip_applied() for compression / Compression - gzip applied
# - create_app_request_timeout_request_exceeds_timeout() for request_timeout / Request exceeds timeout
# - create_app_request_timeout_request_completes_before_timeout() for request_timeout / Request completes before timeout
# - create_app_json_bodies_uuid_field_invalid_format() for json_bodies / UUID field - invalid format
# - create_app_json_bodies_44_const_validation_failure() for json_bodies / 44_const_validation_failure
# - create_app_json_bodies_boolean_field_success() for json_bodies / Boolean field - success
# - create_app_json_bodies_numeric_le_validation_success() for json_bodies / Numeric le validation - success
# - create_app_json_bodies_deeply_nested_objects() for json_bodies / Deeply nested objects
# - create_app_json_bodies_optional_fields_omitted() for json_bodies / Optional fields - omitted
# - create_app_json_bodies_uuid_field_success() for json_bodies / UUID field - success
# - create_app_json_bodies_date_field_success() for json_bodies / Date field - success
# - create_app_json_bodies_47_maxproperties_validation_failure() for json_bodies / 47_maxproperties_validation_failure
# - create_app_json_bodies_46_minproperties_validation_failure() for json_bodies / 46_minproperties_validation_failure
# - create_app_json_bodies_string_min_length_validation_fail() for json_bodies / String min_length validation - fail
# - create_app_json_bodies_field_type_validation_invalid_type() for json_bodies / Field type validation - invalid type
# - create_app_json_bodies_36_oneof_schema_multiple_match_failure() for json_bodies / 36_oneof_schema_multiple_match_failure
# - create_app_json_bodies_nested_object_success() for json_bodies / Nested object - success
# - create_app_json_bodies_41_not_schema_success() for json_bodies / 41_not_schema_success
# - create_app_json_bodies_string_max_length_validation_fail() for json_bodies / String max_length validation - fail
# - create_app_json_bodies_50_deep_nesting_4_levels() for json_bodies / 50_deep_nesting_4_levels
# - create_app_json_bodies_48_dependencies_validation_success() for json_bodies / 48_dependencies_validation_success
# - create_app_json_bodies_patch_partial_update() for json_bodies / PATCH partial update
# - create_app_json_bodies_30_nested_object_missing_field() for json_bodies / 30_nested_object_missing_field
# - create_app_json_bodies_datetime_field_success() for json_bodies / Datetime field - success
# - create_app_json_bodies_string_pattern_validation_success() for json_bodies / String pattern validation - success
# - create_app_json_bodies_extra_fields_ignored_no_additionalproperties() for json_bodies / Extra fields ignored (no additionalProperties)
# - create_app_json_bodies_40_anyof_schema_failure() for json_bodies / 40_anyof_schema_failure
# - create_app_json_bodies_39_anyof_schema_multiple_match_success() for json_bodies / 39_anyof_schema_multiple_match_success
# - create_app_json_bodies_array_of_primitive_values() for json_bodies / Array of primitive values
# - create_app_json_bodies_numeric_ge_validation_fail() for json_bodies / Numeric ge validation - fail
# - create_app_json_bodies_37_oneof_schema_no_match_failure() for json_bodies / 37_oneof_schema_no_match_failure
# - create_app_json_bodies_empty_array_validation_fail() for json_bodies / Empty array validation - fail
# - create_app_json_bodies_38_anyof_schema_success() for json_bodies / 38_anyof_schema_success
# - create_app_json_bodies_empty_json_object() for json_bodies / Empty JSON object
# - create_app_json_bodies_string_pattern_validation_fail() for json_bodies / String pattern validation - fail
# - create_app_json_bodies_49_dependencies_validation_failure() for json_bodies / 49_dependencies_validation_failure
# - create_app_json_bodies_simple_json_object_success() for json_bodies / Simple JSON object - success
# - create_app_json_bodies_required_field_missing_validation_error() for json_bodies / Required field missing - validation error
# - create_app_json_bodies_35_oneof_schema_success() for json_bodies / 35_oneof_schema_success
# - create_app_json_bodies_enum_field_invalid_value() for json_bodies / Enum field - invalid value
# - create_app_json_bodies_enum_field_success() for json_bodies / Enum field - success
# - create_app_json_bodies_33_allof_schema_composition() for json_bodies / 33_allof_schema_composition
# - create_app_json_bodies_45_minproperties_validation_success() for json_bodies / 45_minproperties_validation_success
# - create_app_json_bodies_body_with_query_parameters() for json_bodies / Body with query parameters
# - create_app_json_bodies_42_not_schema_failure() for json_bodies / 42_not_schema_failure
# - create_app_json_bodies_43_const_validation_success() for json_bodies / 43_const_validation_success
# - create_app_json_bodies_32_schema_ref_definitions() for json_bodies / 32_schema_ref_definitions
# - create_app_json_bodies_29_nested_object_validation_success() for json_bodies / 29_nested_object_validation_success
# - create_app_json_bodies_34_additional_properties_false() for json_bodies / 34_additional_properties_false
# - create_app_json_bodies_null_value_for_optional_field() for json_bodies / Null value for optional field
# - create_app_json_bodies_31_nullable_property_null_value() for json_bodies / 31_nullable_property_null_value
# - create_app_json_bodies_array_of_objects_success() for json_bodies / Array of objects - success
# - create_app_cors_07_cors_preflight_header_not_allowed() for cors / 07_cors_preflight_header_not_allowed
# - create_app_cors_cors_vary_header_for_proper_caching() for cors / CORS Vary header for proper caching
# - create_app_cors_cors_preflight_for_put_method() for cors / CORS preflight for PUT method
# - create_app_cors_cors_preflight_for_delete_method() for cors / CORS preflight for DELETE method
# - create_app_cors_cors_multiple_allowed_origins() for cors / CORS multiple allowed origins
# - create_app_cors_cors_preflight_request() for cors / CORS preflight request
# - create_app_cors_cors_with_credentials() for cors / CORS with credentials
# - create_app_cors_cors_regex_pattern_matching_for_origins() for cors / CORS regex pattern matching for origins
# - create_app_cors_08_cors_max_age() for cors / 08_cors_max_age
# - create_app_cors_10_cors_origin_null() for cors / 10_cors_origin_null
# - create_app_cors_cors_wildcard_origin() for cors / CORS wildcard origin
# - create_app_cors_cors_safelisted_headers_without_preflight() for cors / CORS safelisted headers without preflight
# - create_app_cors_cors_private_network_access() for cors / CORS Private Network Access
# - create_app_cors_cors_origin_case_sensitivity() for cors / CORS origin case sensitivity
# - create_app_cors_cors_request_blocked() for cors / CORS request blocked
# - create_app_cors_simple_cors_request() for cors / Simple CORS request
# - create_app_cors_09_cors_expose_headers() for cors / 09_cors_expose_headers
# - create_app_cors_06_cors_preflight_method_not_allowed() for cors / 06_cors_preflight_method_not_allowed
# - create_app_cookies_25_cookie_samesite_lax() for cookies / 25_cookie_samesite_lax
# - create_app_cookies_optional_cookie_parameter_success() for cookies / Optional cookie parameter - success
# - create_app_cookies_cookie_regex_pattern_validation_fail() for cookies / Cookie regex pattern validation - fail
# - create_app_cookies_response_session_cookie_no_max_age() for cookies / Response - session cookie (no max_age)
# - create_app_cookies_27_cookie_httponly_flag() for cookies / 27_cookie_httponly_flag
# - create_app_cookies_response_cookie_with_attributes() for cookies / Response cookie with attributes
# - create_app_cookies_24_cookie_samesite_strict() for cookies / 24_cookie_samesite_strict
# - create_app_cookies_apikey_cookie_authentication_success() for cookies / APIKey cookie authentication - success
# - create_app_cookies_cookie_validation_min_length_constraint_success() for cookies / Cookie validation - min_length constraint success
# - create_app_cookies_cookie_validation_min_length_failure() for cookies / Cookie validation - min_length failure
# - create_app_cookies_cookie_validation_max_length_constraint_fail() for cookies / Cookie validation - max_length constraint fail
# - create_app_cookies_required_cookie_missing() for cookies / Required cookie - missing
# - create_app_cookies_optional_cookie_parameter_missing() for cookies / Optional cookie parameter - missing
# - create_app_cookies_apikey_cookie_authentication_missing() for cookies / APIKey cookie authentication - missing
# - create_app_cookies_response_multiple_cookies() for cookies / Response - multiple cookies
# - create_app_cookies_response_cookie_with_samesite_lax() for cookies / Response cookie with SameSite=Lax
# - create_app_cookies_response_delete_cookie() for cookies / Response - delete cookie
# - create_app_cookies_response_cookie_with_path_attribute() for cookies / Response cookie with path attribute
# - create_app_cookies_optional_apikey_cookie_missing() for cookies / Optional APIKey cookie - missing
# - create_app_cookies_response_cookie_with_samesite_strict() for cookies / Response cookie with SameSite=Strict
# - create_app_cookies_response_cookie_with_samesite_none() for cookies / Response cookie with SameSite=None
# - create_app_cookies_cookie_regex_pattern_validation_success() for cookies / Cookie regex pattern validation - success
# - create_app_cookies_response_set_cookie_basic() for cookies / Response set cookie - basic
# - create_app_cookies_multiple_cookies_success() for cookies / Multiple cookies - success
# - create_app_cookies_26_cookie_secure_flag() for cookies / 26_cookie_secure_flag
# - create_app_cookies_response_cookie_with_domain_attribute() for cookies / Response cookie with domain attribute
# - create_app_edge_cases_19_emoji_in_strings() for edge_cases / 19_emoji_in_strings
# - create_app_edge_cases_12_percent_encoded_special_chars() for edge_cases / 12_percent_encoded_special_chars
# - create_app_edge_cases_special_string_values_and_escaping() for edge_cases / Special string values and escaping
# - create_app_edge_cases_15_float_precision_preservation() for edge_cases / 15_float_precision_preservation
# - create_app_edge_cases_13_empty_string_query_param_preserved() for edge_cases / 13_empty_string_query_param_preserved
# - create_app_edge_cases_24_array_with_holes() for edge_cases / 24_array_with_holes
# - create_app_edge_cases_21_scientific_notation_number() for edge_cases / 21_scientific_notation_number
# - create_app_edge_cases_float_precision_and_rounding() for edge_cases / Float precision and rounding
# - create_app_edge_cases_unicode_and_emoji_handling() for edge_cases / Unicode and emoji handling
# - create_app_edge_cases_17_extremely_long_string() for edge_cases / 17_extremely_long_string
# - create_app_edge_cases_11_utf8_query_parameter() for edge_cases / 11_utf8_query_parameter
# - create_app_edge_cases_18_unicode_normalization() for edge_cases / 18_unicode_normalization
# - create_app_edge_cases_20_null_byte_in_string() for edge_cases / 20_null_byte_in_string
# - create_app_edge_cases_23_deeply_nested_json_limit() for edge_cases / 23_deeply_nested_json_limit
# - create_app_edge_cases_14_large_integer_boundary() for edge_cases / 14_large_integer_boundary
# - create_app_edge_cases_22_leading_zeros_integer() for edge_cases / 22_leading_zeros_integer
# - create_app_edge_cases_large_integer_boundary_values() for edge_cases / Large integer boundary values
# - create_app_edge_cases_deeply_nested_structure_10_levels() for edge_cases / Deeply nested structure (10+ levels)
# - create_app_edge_cases_empty_and_null_value_handling() for edge_cases / Empty and null value handling
# - create_app_edge_cases_16_negative_zero_handling() for edge_cases / 16_negative_zero_handling
# - create_app_streaming_stream_json_lines() for streaming / Stream JSON lines
# - create_app_streaming_binary_log_download() for streaming / Binary log download
# - create_app_streaming_chunked_csv_export() for streaming / Chunked CSV export
# - create_app_static_files_static_file_server_returns_text_file() for static_files / Static file server returns text file
# - create_app_static_files_static_server_returns_index_html_for_directory() for static_files / Static server returns index.html for directory
# - create_app_query_params_string_validation_with_regex_success() for query_params / String validation with regex - success
# - create_app_query_params_49_integer_gt_constraint_success() for query_params / 49_integer_gt_constraint_success
# - create_app_query_params_enum_query_parameter_invalid_value() for query_params / Enum query parameter - invalid value
# - create_app_query_params_68_array_uniqueitems_success() for query_params / 68_array_uniqueitems_success
# - create_app_query_params_47_pattern_validation_email_success() for query_params / 47_pattern_validation_email_success
# - create_app_query_params_required_integer_query_parameter_success() for query_params / Required integer query parameter - success
# - create_app_query_params_required_string_query_parameter_missing() for query_params / Required string query parameter - missing
# - create_app_query_params_57_boolean_empty_string_coercion() for query_params / 57_boolean_empty_string_coercion
# - create_app_query_params_52_integer_le_constraint_boundary() for query_params / 52_integer_le_constraint_boundary
# - create_app_query_params_list_with_default_empty_array_no_values_provided() for query_params / List with default empty array - no values provided
# - create_app_query_params_date_query_parameter_success() for query_params / Date query parameter - success
# - create_app_query_params_string_query_param_with_max_length_constraint_fail() for query_params / String query param with max_length constraint - fail
# - create_app_query_params_45_string_minlength_validation_failure() for query_params / 45_string_minlength_validation_failure
# - create_app_query_params_integer_with_default_value_override() for query_params / Integer with default value - override
# - create_app_query_params_67_multipleof_constraint_failure() for query_params / 67_multipleof_constraint_failure
# - create_app_query_params_58_format_email_success() for query_params / 58_format_email_success
# - create_app_query_params_integer_query_param_with_ge_constraint_boundary() for query_params / Integer query param with ge constraint - boundary
# - create_app_query_params_integer_query_param_with_gt_constraint_valid() for query_params / Integer query param with gt constraint - valid
# - create_app_query_params_required_integer_query_parameter_invalid_type() for query_params / Required integer query parameter - invalid type
# - create_app_query_params_required_integer_query_parameter_float_value() for query_params / Required integer query parameter - float value
# - create_app_query_params_query_parameter_with_url_encoded_special_characters() for query_params / Query parameter with URL encoded special characters
# - create_app_query_params_59_format_email_failure() for query_params / 59_format_email_failure
# - create_app_query_params_43_scientific_notation_float() for query_params / 43_scientific_notation_float
# - create_app_query_params_63_format_uri_success() for query_params / 63_format_uri_success
# - create_app_query_params_boolean_query_parameter_numeric_1() for query_params / Boolean query parameter - numeric 1
# - create_app_query_params_string_query_param_with_min_length_constraint_fail() for query_params / String query param with min_length constraint - fail
# - create_app_query_params_optional_string_query_parameter_provided() for query_params / Optional string query parameter - provided
# - create_app_query_params_list_of_integers_multiple_values() for query_params / List of integers - multiple values
# - create_app_query_params_integer_query_param_with_lt_constraint_valid() for query_params / Integer query param with lt constraint - valid
# - create_app_query_params_42_negative_integer_query_param() for query_params / 42_negative_integer_query_param
# - create_app_query_params_46_string_maxlength_validation_failure() for query_params / 46_string_maxlength_validation_failure
# - create_app_query_params_56_array_maxitems_constraint_failure() for query_params / 56_array_maxitems_constraint_failure
# - create_app_query_params_string_query_param_with_regex_pattern_fail() for query_params / String query param with regex pattern - fail
# - create_app_query_params_44_string_minlength_validation_success() for query_params / 44_string_minlength_validation_success
# - create_app_query_params_61_format_ipv4_failure() for query_params / 61_format_ipv4_failure
# - create_app_query_params_48_pattern_validation_email_failure() for query_params / 48_pattern_validation_email_failure
# - create_app_query_params_required_integer_query_parameter_missing() for query_params / Required integer query parameter - missing
# - create_app_query_params_query_parameter_with_special_characters_url_encoding() for query_params / Query parameter with special characters - URL encoding
# - create_app_query_params_list_query_parameter_required_but_missing() for query_params / List query parameter - required but missing
# - create_app_query_params_required_string_query_parameter_success() for query_params / Required string query parameter - success
# - create_app_query_params_66_multipleof_constraint_success() for query_params / 66_multipleof_constraint_success
# - create_app_query_params_53_integer_le_constraint_failure() for query_params / 53_integer_le_constraint_failure
# - create_app_query_params_multiple_query_parameters_with_different_types() for query_params / Multiple query parameters with different types
# - create_app_query_params_71_array_separator_semicolon() for query_params / 71_array_separator_semicolon
# - create_app_query_params_70_array_separator_pipe() for query_params / 70_array_separator_pipe
# - create_app_query_params_integer_with_default_value_not_provided() for query_params / Integer with default value - not provided
# - create_app_query_params_boolean_query_parameter_true() for query_params / Boolean query parameter - true
# - create_app_query_params_integer_query_param_with_le_constraint_boundary() for query_params / Integer query param with le constraint - boundary
# - create_app_query_params_float_query_param_with_ge_constraint_success() for query_params / Float query param with ge constraint - success
# - create_app_query_params_51_integer_ge_constraint_boundary() for query_params / 51_integer_ge_constraint_boundary
# - create_app_query_params_optional_integer_query_parameter_missing() for query_params / Optional integer query parameter - missing
# - create_app_query_params_69_array_uniqueitems_failure() for query_params / 69_array_uniqueitems_failure
# - create_app_query_params_72_array_separator_space() for query_params / 72_array_separator_space
# - create_app_query_params_string_validation_with_regex_failure() for query_params / String validation with regex - failure
# - create_app_query_params_65_format_hostname_success() for query_params / 65_format_hostname_success
# - create_app_query_params_query_parameter_with_url_encoded_space() for query_params / Query parameter with URL encoded space
# - create_app_query_params_list_of_strings_multiple_values() for query_params / List of strings - multiple values
# - create_app_query_params_optional_query_parameter_with_default_value() for query_params / Optional query parameter with default value
# - create_app_query_params_62_format_ipv6_success() for query_params / 62_format_ipv6_success
# - create_app_query_params_array_query_parameter_single_value() for query_params / Array query parameter - single value
# - create_app_query_params_optional_string_query_parameter_missing() for query_params / Optional string query parameter - missing
# - create_app_query_params_datetime_query_parameter_success() for query_params / Datetime query parameter - success
# - create_app_query_params_uuid_query_parameter_invalid_format() for query_params / UUID query parameter - invalid format
# - create_app_query_params_array_query_parameter_empty_array() for query_params / Array query parameter - empty array
# - create_app_query_params_enum_query_parameter_success() for query_params / Enum query parameter - success
# - create_app_query_params_uuid_query_parameter_success() for query_params / UUID query parameter - success
# - create_app_query_params_50_integer_gt_constraint_failure() for query_params / 50_integer_gt_constraint_failure
# - create_app_query_params_64_format_uri_failure() for query_params / 64_format_uri_failure
# - create_app_query_params_54_array_minitems_constraint_success() for query_params / 54_array_minitems_constraint_success
# - create_app_query_params_55_array_minitems_constraint_failure() for query_params / 55_array_minitems_constraint_failure
# - create_app_query_params_60_format_ipv4_success() for query_params / 60_format_ipv4_success
# - create_app_rate_limit_rate_limit_below_threshold_succeeds() for rate_limit / Rate limit below threshold succeeds
# - create_app_rate_limit_rate_limit_exceeded_returns_429() for rate_limit / Rate limit exceeded returns 429
# - create_app_body_limits_body_under_limit_succeeds() for body_limits / Body under limit succeeds
# - create_app_body_limits_body_over_limit_returns_413() for body_limits / Body over limit returns 413
# - create_app_auth_jwt_malformed_token_format() for auth / JWT malformed token format
# - create_app_auth_bearer_token_without_prefix() for auth / Bearer token without prefix
# - create_app_auth_jwt_authentication_valid_token() for auth / JWT authentication - valid token
# - create_app_auth_api_key_rotation_old_key_still_valid() for auth / API key rotation - old key still valid
# - create_app_auth_jwt_invalid_issuer() for auth / JWT invalid issuer
# - create_app_auth_jwt_with_multiple_audiences() for auth / JWT with multiple audiences
# - create_app_auth_api_key_in_query_parameter() for auth / API key in query parameter
# - create_app_auth_jwt_authentication_expired_token() for auth / JWT authentication - expired token
# - create_app_auth_api_key_authentication_invalid_key() for auth / API key authentication - invalid key
# - create_app_auth_jwt_not_before_claim_in_future() for auth / JWT not before claim in future
# - create_app_auth_multiple_authentication_schemes_jwt_precedence() for auth / Multiple authentication schemes - JWT precedence
# - create_app_auth_jwt_missing_required_custom_claims() for auth / JWT missing required custom claims
# - create_app_auth_api_key_authentication_valid_key() for auth / API key authentication - valid key
# - create_app_auth_api_key_with_custom_header_name() for auth / API key with custom header name
# - create_app_auth_api_key_authentication_missing_header() for auth / API key authentication - missing header
# - create_app_auth_jwt_authentication_invalid_signature() for auth / JWT authentication - invalid signature
# - create_app_auth_jwt_authentication_missing_authorization_header() for auth / JWT authentication - missing Authorization header
# - create_app_auth_jwt_authentication_invalid_audience() for auth / JWT authentication - invalid audience
# - create_app_http_methods_options_cors_preflight_request() for http_methods / OPTIONS - CORS preflight request
# - create_app_http_methods_delete_remove_resource() for http_methods / DELETE - Remove resource
# - create_app_http_methods_put_create_resource_if_doesn_t_exist() for http_methods / PUT - Create resource if doesn't exist
# - create_app_http_methods_patch_update_multiple_fields() for http_methods / PATCH - Update multiple fields
# - create_app_http_methods_put_validation_error() for http_methods / PUT - Validation error
# - create_app_http_methods_head_get_metadata_without_body() for http_methods / HEAD - Get metadata without body
# - create_app_http_methods_delete_with_response_body() for http_methods / DELETE - With response body
# - create_app_http_methods_put_missing_required_field() for http_methods / PUT - Missing required field
# - create_app_http_methods_patch_partial_update() for http_methods / PATCH - Partial update
# - create_app_http_methods_delete_resource_not_found() for http_methods / DELETE - Resource not found
# - create_app_http_methods_put_idempotent_operation() for http_methods / PUT - Idempotent operation
# - create_app_http_methods_put_complete_resource_replacement() for http_methods / PUT - Complete resource replacement
# - create_app_path_params_boolean_path_parameter_true() for path_params / Boolean path parameter - True
# - create_app_path_params_29_decimal_path_param_success() for path_params / 29_decimal_path_param_success
# - create_app_path_params_integer_path_parameter_with_combined_lt_and_gt_constraints_success() for path_params / Integer path parameter with combined lt and gt constraints - success
# - create_app_path_params_33_string_pattern_path_success() for path_params / 33_string_pattern_path_success
# - create_app_path_params_31_string_minlength_path_failure() for path_params / 31_string_minlength_path_failure
# - create_app_path_params_35_negative_integer_path_param() for path_params / 35_negative_integer_path_param
# - create_app_path_params_enum_path_parameter_invalid_value() for path_params / Enum path parameter - invalid value
# - create_app_path_params_27_datetime_format_path_param_success() for path_params / 27_datetime_format_path_param_success
# - create_app_path_params_25_date_format_invalid_failure() for path_params / 25_date_format_invalid_failure
# - create_app_path_params_integer_path_parameter_with_lt_constraint_success() for path_params / Integer path parameter with lt constraint - success
# - create_app_path_params_integer_path_parameter_with_gt_constraint_success() for path_params / Integer path parameter with gt constraint - success
# - create_app_path_params_28_duration_format_path_param_success() for path_params / 28_duration_format_path_param_success
# - create_app_path_params_path_parameter_type_syntax_with_override() for path_params / Path parameter type syntax with override
# - create_app_path_params_20_uuid_v3_path_param_success() for path_params / 20_uuid_v3_path_param_success
# - create_app_path_params_integer_path_parameter_invalid_string() for path_params / Integer path parameter - invalid string
# - create_app_path_params_30_string_minlength_path_success() for path_params / 30_string_minlength_path_success
# - create_app_path_params_integer_path_parameter_with_le_constraint_success() for path_params / Integer path parameter with le constraint - success
# - create_app_path_params_path_parameter_type_syntax_invalid_uuid() for path_params / Path parameter type syntax - invalid UUID
# - create_app_path_params_path_type_parameter_file_path() for path_params / Path type parameter - file path
# - create_app_path_params_path_parameter_with_type_syntax_uuid() for path_params / Path parameter with type syntax - UUID
# - create_app_path_params_32_string_maxlength_path_failure() for path_params / 32_string_maxlength_path_failure
# - create_app_path_params_integer_path_parameter_success() for path_params / Integer path parameter - success
# - create_app_path_params_34_string_pattern_path_failure() for path_params / 34_string_pattern_path_failure
# - create_app_path_params_21_uuid_v5_path_param_success() for path_params / 21_uuid_v5_path_param_success
# - create_app_path_params_string_path_parameter_with_max_length_failure() for path_params / String path parameter with max_length - failure
# - create_app_path_params_string_path_parameter_with_min_length_failure() for path_params / String path parameter with min_length - failure
# - create_app_path_params_multiple_path_parameters_success() for path_params / Multiple path parameters - success
# - create_app_path_params_date_path_parameter_success() for path_params / Date path parameter - success
# - create_app_path_params_integer_path_parameter_with_gt_constraint_failure() for path_params / Integer path parameter with gt constraint - failure
# - create_app_path_params_24_date_format_path_param_success() for path_params / 24_date_format_path_param_success
# - create_app_path_params_float_path_parameter_success() for path_params / Float path parameter - success
# - create_app_path_params_path_parameter_with_type_syntax_integer() for path_params / Path parameter with type syntax - integer
# - create_app_path_params_string_path_parameter_success() for path_params / String path parameter - success
# - create_app_path_params_uuid_path_parameter_success() for path_params / UUID path parameter - success
# - create_app_path_params_integer_path_parameter_with_ge_constraint_success() for path_params / Integer path parameter with ge constraint - success
# - create_app_path_params_enum_path_parameter_success() for path_params / Enum path parameter - success
# - create_app_path_params_boolean_path_parameter_numeric_1() for path_params / Boolean path parameter - numeric 1
# - create_app_request_id_request_id_header_is_preserved() for request_id / Request ID header is preserved
# - create_app_request_id_request_id_middleware_can_be_disabled() for request_id / Request ID middleware can be disabled
# - create_app_request_id_request_id_is_generated_when_not_provided() for request_id / Request ID is generated when not provided
# - create_app_headers_header_regex_validation_success() for headers / Header regex validation - success
# - create_app_headers_33_api_key_header_valid() for headers / 33_api_key_header_valid
# - create_app_headers_content_type_header_application_json() for headers / Content-Type header - application/json
# - create_app_headers_accept_language_header() for headers / Accept-Language header
# - create_app_headers_x_api_key_required_header_success() for headers / X-API-Key required header - success
# - create_app_headers_header_validation_max_length_constraint_fail() for headers / Header validation - max_length constraint fail
# - create_app_headers_x_api_key_required_header_missing() for headers / X-API-Key required header - missing
# - create_app_headers_origin_header() for headers / Origin header
# - create_app_headers_user_agent_header_default_value() for headers / User-Agent header - default value
# - create_app_headers_32_bearer_token_missing_prefix() for headers / 32_bearer_token_missing_prefix
# - create_app_headers_optional_header_with_none_default_missing() for headers / Optional header with None default - missing
# - create_app_headers_header_regex_validation_fail() for headers / Header regex validation - fail
# - create_app_headers_31_bearer_token_format_invalid() for headers / 31_bearer_token_format_invalid
# - create_app_headers_x_api_key_optional_header_success() for headers / X-API-Key optional header - success
# - create_app_headers_authorization_header_success() for headers / Authorization header - success
# - create_app_headers_30_bearer_token_format_valid() for headers / 30_bearer_token_format_valid
# - create_app_headers_authorization_header_missing() for headers / Authorization header - missing
# - create_app_headers_accept_header_json() for headers / Accept header - JSON
# - create_app_headers_accept_encoding_header() for headers / Accept-Encoding header
# - create_app_headers_authorization_header_wrong_scheme() for headers / Authorization header - wrong scheme
# - create_app_headers_header_validation_min_length_constraint() for headers / Header validation - min_length constraint
# - create_app_headers_basic_authentication_success() for headers / Basic authentication - success
# - create_app_headers_bearer_token_authentication_missing() for headers / Bearer token authentication - missing
# - create_app_headers_x_api_key_optional_header_missing() for headers / X-API-Key optional header - missing
# - create_app_headers_multiple_header_values_x_token() for headers / Multiple header values - X-Token
# - create_app_headers_multiple_custom_headers() for headers / Multiple custom headers
# - create_app_headers_34_api_key_header_invalid() for headers / 34_api_key_header_invalid
# - create_app_headers_bearer_token_authentication_success() for headers / Bearer token authentication - success
# - create_app_headers_host_header() for headers / Host header
# - create_app_headers_referer_header() for headers / Referer header
# - create_app_headers_header_with_underscore_conversion_explicit() for headers / Header with underscore conversion - explicit
# - create_app_headers_header_case_insensitivity_access() for headers / Header case insensitivity - access
# - create_app_headers_user_agent_header_custom_value() for headers / User-Agent header - custom value
# - create_app_status_codes_408_request_timeout() for status_codes / 408 Request Timeout
# - create_app_status_codes_404_not_found_resource_not_found() for status_codes / 404 Not Found - Resource not found
# - create_app_status_codes_503_service_unavailable_server_overload() for status_codes / 503 Service Unavailable - Server overload
# - create_app_status_codes_422_unprocessable_entity_validation_error() for status_codes / 422 Unprocessable Entity - Validation error
# - create_app_status_codes_302_found_temporary_redirect() for status_codes / 302 Found - Temporary redirect
# - create_app_status_codes_304_not_modified_cached_content_valid() for status_codes / 304 Not Modified - Cached content valid
# - create_app_status_codes_400_bad_request_invalid_request() for status_codes / 400 Bad Request - Invalid request
# - create_app_status_codes_22_501_not_implemented() for status_codes / 22_501_not_implemented
# - create_app_status_codes_204_no_content_success_with_no_body() for status_codes / 204 No Content - Success with no body
# - create_app_status_codes_301_moved_permanently_permanent_redirect() for status_codes / 301 Moved Permanently - Permanent redirect
# - create_app_status_codes_201_created_resource_created() for status_codes / 201 Created - Resource created
# - create_app_status_codes_202_accepted_request_accepted_for_processing() for status_codes / 202 Accepted - Request accepted for processing
# - create_app_status_codes_307_temporary_redirect_method_preserved() for status_codes / 307 Temporary Redirect - Method preserved
# - create_app_status_codes_500_internal_server_error_server_error() for status_codes / 500 Internal Server Error - Server error
# - create_app_status_codes_20_414_uri_too_long() for status_codes / 20_414_uri_too_long
# - create_app_status_codes_401_unauthorized_missing_authentication() for status_codes / 401 Unauthorized - Missing authentication
# - create_app_status_codes_23_503_service_unavailable() for status_codes / 23_503_service_unavailable
# - create_app_status_codes_19_413_payload_too_large() for status_codes / 19_413_payload_too_large
# - create_app_status_codes_403_forbidden_insufficient_permissions() for status_codes / 403 Forbidden - Insufficient permissions
# - create_app_status_codes_21_431_request_header_fields_too_large() for status_codes / 21_431_request_header_fields_too_large
# - create_app_status_codes_429_too_many_requests() for status_codes / 429 Too Many Requests
# - create_app_status_codes_200_ok_success() for status_codes / 200 OK - Success
# - create_app_status_codes_206_partial_content() for status_codes / 206 Partial Content
# - create_app_background_background_event_logging_second_payload() for background / Background event logging - second payload
# - create_app_background_background_event_logging() for background / Background event logging
# - create_app_validation_errors_invalid_uuid_format() for validation_errors / Invalid UUID format
# - create_app_validation_errors_invalid_boolean_value() for validation_errors / Invalid boolean value
# - create_app_validation_errors_missing_required_query_parameter() for validation_errors / Missing required query parameter
# - create_app_validation_errors_array_max_items_constraint_violation() for validation_errors / Array max_items constraint violation
# - create_app_validation_errors_numeric_constraint_violation_gt_greater_than() for validation_errors / Numeric constraint violation - gt (greater than)
# - create_app_validation_errors_string_regex_pattern_mismatch() for validation_errors / String regex pattern mismatch
# - create_app_validation_errors_invalid_enum_value() for validation_errors / Invalid enum value
# - create_app_validation_errors_string_min_length_constraint_violation() for validation_errors / String min_length constraint violation
# - create_app_validation_errors_multiple_validation_errors() for validation_errors / Multiple validation errors
# - create_app_validation_errors_string_max_length_constraint_violation() for validation_errors / String max_length constraint violation
# - create_app_validation_errors_nested_object_validation_error() for validation_errors / Nested object validation error
# - create_app_validation_errors_10_nested_error_path() for validation_errors / 10_nested_error_path
# - create_app_validation_errors_invalid_datetime_format() for validation_errors / Invalid datetime format
# - create_app_validation_errors_array_item_validation_error() for validation_errors / Array item validation error
# - create_app_validation_errors_missing_required_body_field() for validation_errors / Missing required body field
# - create_app_validation_errors_body_field_type_error_string_for_float() for validation_errors / Body field type error - string for float
# - create_app_validation_errors_malformed_json_body() for validation_errors / Malformed JSON body
# - create_app_validation_errors_query_param_type_error_string_provided_for_int() for validation_errors / Query param type error - string provided for int
# - create_app_validation_errors_header_validation_error() for validation_errors / Header validation error
# - create_app_validation_errors_09_multiple_validation_errors() for validation_errors / 09_multiple_validation_errors
# - create_app_validation_errors_numeric_constraint_violation_le_less_than_or_equal() for validation_errors / Numeric constraint violation - le (less than or equal)
# - create_app_validation_errors_array_min_items_constraint_violation() for validation_errors / Array min_items constraint violation
# - create_app_content_types_415_unsupported_media_type() for content_types / 415 Unsupported Media Type
# - create_app_content_types_xml_response_application_xml() for content_types / XML response - application/xml
# - create_app_content_types_14_content_type_case_insensitive() for content_types / 14_content_type_case_insensitive
# - create_app_content_types_json_with_utf_8_charset() for content_types / JSON with UTF-8 charset
# - create_app_content_types_16_text_plain_not_accepted() for content_types / 16_text_plain_not_accepted
# - create_app_content_types_pdf_response_application_pdf() for content_types / PDF response - application/pdf
# - create_app_content_types_20_content_length_mismatch() for content_types / 20_content_length_mismatch
# - create_app_content_types_17_vendor_json_accepted() for content_types / 17_vendor_json_accepted
# - create_app_content_types_13_json_with_charset_utf16() for content_types / 13_json_with_charset_utf16
# - create_app_content_types_json_response_application_json() for content_types / JSON response - application/json
# - create_app_content_types_15_multipart_boundary_required() for content_types / 15_multipart_boundary_required
# - create_app_content_types_content_negotiation_accept_header() for content_types / Content negotiation - Accept header
# - create_app_content_types_html_response_text_html() for content_types / HTML response - text/html
# - create_app_content_types_jpeg_image_response_image_jpeg() for content_types / JPEG image response - image/jpeg
# - create_app_content_types_19_missing_content_type_default_json() for content_types / 19_missing_content_type_default_json
# - create_app_content_types_png_image_response_image_png() for content_types / PNG image response - image/png
# - create_app_content_types_plain_text_response_text_plain() for content_types / Plain text response - text/plain
# - create_app_content_types_18_content_type_with_multiple_params() for content_types / 18_content_type_with_multiple_params
# - create_app_content_types_csv_response_text_csv() for content_types / CSV response - text/csv
# - create_app_content_types_binary_response_application_octet_stream() for content_types / Binary response - application/octet-stream
# - create_app_url_encoded_simple_form_submission_success() for url_encoded / Simple form submission - success
# - create_app_url_encoded_15_special_characters_field_names() for url_encoded / 15_special_characters_field_names
# - create_app_url_encoded_pattern_validation_fail() for url_encoded / Pattern validation - fail
# - create_app_url_encoded_22_additional_properties_strict_failure() for url_encoded / 22_additional_properties_strict_failure
# - create_app_url_encoded_17_pattern_validation_failure() for url_encoded / 17_pattern_validation_failure
# - create_app_url_encoded_20_format_email_validation_failure() for url_encoded / 20_format_email_validation_failure
# - create_app_url_encoded_multiple_values_for_same_field() for url_encoded / Multiple values for same field
# - create_app_url_encoded_required_field_missing_validation_error() for url_encoded / Required field missing - validation error
# - create_app_url_encoded_13_array_field_success() for url_encoded / 13_array_field_success
# - create_app_url_encoded_numeric_field_type_conversion() for url_encoded / Numeric field type conversion
# - create_app_url_encoded_special_characters_encoding() for url_encoded / Special characters encoding
# - create_app_url_encoded_boolean_field_conversion() for url_encoded / Boolean field conversion
# - create_app_url_encoded_empty_string_value() for url_encoded / Empty string value
# - create_app_url_encoded_oauth2_password_grant_flow() for url_encoded / OAuth2 password grant flow
# - create_app_url_encoded_19_array_minitems_validation_failure() for url_encoded / 19_array_minitems_validation_failure
# - create_app_url_encoded_optional_field_missing_success() for url_encoded / Optional field missing - success
# - create_app_url_encoded_14_nested_object_bracket_notation() for url_encoded / 14_nested_object_bracket_notation
# - create_app_url_encoded_string_max_length_validation_fail() for url_encoded / String max_length validation - fail
# - create_app_url_encoded_18_integer_minimum_validation_failure() for url_encoded / 18_integer_minimum_validation_failure
# - create_app_url_encoded_21_integer_type_coercion_failure() for url_encoded / 21_integer_type_coercion_failure
# - create_app_url_encoded_16_minlength_validation_failure() for url_encoded / 16_minlength_validation_failure
# - create_app_url_encoded_string_min_length_validation_fail() for url_encoded / String min_length validation - fail
# - create_app_multipart_multiple_values_for_same_field_name() for multipart / Multiple values for same field name
# - create_app_multipart_19_file_mime_spoofing_png_as_jpeg() for multipart / 19_file_mime_spoofing_png_as_jpeg
# - create_app_multipart_20_file_mime_spoofing_jpeg_as_png() for multipart / 20_file_mime_spoofing_jpeg_as_png
# - create_app_multipart_21_file_pdf_magic_number_success() for multipart / 21_file_pdf_magic_number_success
# - create_app_multipart_content_type_validation_invalid_type() for multipart / Content-Type validation - invalid type
# - create_app_multipart_pdf_file_upload() for multipart / PDF file upload
# - create_app_multipart_file_list_upload_array_of_files() for multipart / File list upload (array of files)
# - create_app_multipart_optional_file_upload_provided() for multipart / Optional file upload - provided
# - create_app_multipart_file_size_validation_too_large() for multipart / File size validation - too large
# - create_app_multipart_mixed_files_and_form_data() for multipart / Mixed files and form data
# - create_app_multipart_simple_file_upload() for multipart / Simple file upload
# - create_app_multipart_empty_file_upload() for multipart / Empty file upload
# - create_app_multipart_optional_file_upload_missing() for multipart / Optional file upload - missing
# - create_app_multipart_file_upload_without_filename() for multipart / File upload without filename
# - create_app_multipart_18_file_magic_number_jpeg_success() for multipart / 18_file_magic_number_jpeg_success
# - create_app_multipart_22_file_empty_buffer() for multipart / 22_file_empty_buffer
# - create_app_multipart_17_file_magic_number_png_success() for multipart / 17_file_magic_number_png_success
# - create_app_multipart_form_data_without_files() for multipart / Form data without files
# - create_app_multipart_multiple_file_uploads() for multipart / Multiple file uploads
# - create_app_multipart_file_upload_with_custom_headers() for multipart / File upload with custom headers
# - create_app_multipart_required_file_upload_missing() for multipart / Required file upload - missing
# - create_app_multipart_image_file_upload() for multipart / Image file upload
# - create_app_lifecycle_hooks_onresponse_security_headers() for lifecycle_hooks / onResponse - Security Headers
# - create_app_lifecycle_hooks_prehandler_authentication_failed_short_circuit() for lifecycle_hooks / preHandler - Authentication Failed (Short Circuit)
# - create_app_lifecycle_hooks_prehandler_authorization_check() for lifecycle_hooks / preHandler - Authorization Check
# - create_app_lifecycle_hooks_prehandler_authentication_success() for lifecycle_hooks / preHandler - Authentication Success
# - create_app_lifecycle_hooks_prevalidation_rate_limit_exceeded_short_circuit() for lifecycle_hooks / preValidation - Rate Limit Exceeded (Short Circuit)
# - create_app_lifecycle_hooks_onerror_error_logging() for lifecycle_hooks / onError - Error Logging
# - create_app_lifecycle_hooks_multiple_hooks_all_phases() for lifecycle_hooks / Multiple Hooks - All Phases
# - create_app_lifecycle_hooks_hook_execution_order() for lifecycle_hooks / Hook Execution Order
# - create_app_lifecycle_hooks_onresponse_response_timing() for lifecycle_hooks / onResponse - Response Timing
# - create_app_lifecycle_hooks_prehandler_authorization_forbidden_short_circuit() for lifecycle_hooks / preHandler - Authorization Forbidden (Short Circuit)
# - create_app_lifecycle_hooks_onrequest_request_logging() for lifecycle_hooks / onRequest - Request Logging
# - create_app_lifecycle_hooks_prevalidation_rate_limiting() for lifecycle_hooks / preValidation - Rate Limiting
# - create_app_sse_notifications() for sse / /notifications
# - create_app_websocket_chat() for websocket / /chat
