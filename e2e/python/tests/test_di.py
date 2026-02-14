"""E2E tests for di."""

import time

from spikard.testing import TestClient
from app.main import (
    create_app_di_async_factory_dependency_success,
    create_app_di_circular_dependency_detection_error,
    create_app_di_dependency_injection_in_lifecycle_hooks_success,
    create_app_di_factory_dependency_success,
    create_app_di_missing_dependency_error,
    create_app_di_mixed_singleton_and_per_request_caching_success,
    create_app_di_multiple_dependencies_with_cleanup_success,
    create_app_di_nested_dependencies_3_levels_success,
    create_app_di_node_js_object_destructuring_injection_success,
    create_app_di_per_request_dependency_caching_success,
    create_app_di_python_parameter_name_based_injection_success,
    create_app_di_python_type_annotation_based_injection_success,
    create_app_di_resource_cleanup_after_request_success,
    create_app_di_route_level_dependency_override_success,
    create_app_di_ruby_keyword_argument_injection_success,
    create_app_di_singleton_dependency_caching_success,
    create_app_di_type_mismatch_in_dependency_resolution_error,
    create_app_di_value_dependency_injection_success,
)


async def test_route_level_dependency_override_success() -> None:
    """Tests route-level dependency override of app-level dependency for testing or special cases."""

    async with TestClient(create_app_di_route_level_dependency_override_success()) as client:
        response = await client.get("/api/override-test")

        assert response.status_code == 200
        response_data = response.json()
        assert "mode" in response_data
        assert response_data["mode"] == "test"
        assert "strict" in response_data
        assert response_data["strict"] == False


async def test_circular_dependency_detection_error() -> None:
    """Tests that circular dependencies (A depends on B, B depends on A) are detected and rejected at registration time."""

    async with TestClient(create_app_di_circular_dependency_detection_error()) as client:
        response = await client.get("/api/circular")

        assert response.status_code == 500
        response_data = response.json()
        assert "type" in response_data
        assert response_data["type"] == "https://spikard.dev/errors/dependency-error"
        assert "title" in response_data
        assert response_data["title"] == "Dependency Resolution Failed"
        assert "status" in response_data
        assert response_data["status"] == 500
        assert "detail" in response_data
        assert response_data["detail"] == "Circular dependency detected"
        assert "errors" in response_data
        assert len(response_data["errors"]) == 1
        assert "type" in response_data["errors"][0]
        assert "msg" in response_data["errors"][0]
        assert "cycle" in response_data["errors"][0]
        assert len(response_data["errors"][0]["cycle"]) == 3
        assert response_data["errors"][0]["cycle"][0] == "service_a"
        assert response_data["errors"][0]["cycle"][1] == "service_b"
        assert response_data["errors"][0]["cycle"][2] == "service_a"


async def test_factory_dependency_success() -> None:
    """Tests factory dependency that creates instances on-demand."""

    async with TestClient(create_app_di_factory_dependency_success()) as client:
        response = await client.get("/api/timestamp")

        assert response.status_code == 200
        response_data = response.json()
        assert "timestamp" in response_data
        assert response_data["timestamp"] == "<<present>>"


async def test_value_dependency_injection_success() -> None:
    """Tests simple value injection (config, constants) into a handler."""

    async with TestClient(create_app_di_value_dependency_injection_success()) as client:
        response = await client.get("/api/config")

        assert response.status_code == 200
        response_data = response.json()
        assert "app_name" in response_data
        assert response_data["app_name"] == "SpikardApp"
        assert "version" in response_data
        assert response_data["version"] == "1.0.0"
        assert "max_connections" in response_data
        assert response_data["max_connections"] == 100


async def test_node_js_object_destructuring_injection_success() -> None:
    """Tests Node.js/TypeScript-specific object destructuring pattern for dependency injection."""

    async with TestClient(create_app_di_node_js_object_destructuring_injection_success()) as client:
        response = await client.get("/api/node-destructure")

        assert response.status_code == 200
        response_data = response.json()
        assert "db_name" in response_data
        assert response_data["db_name"] == "PostgreSQL"
        assert "log_level" in response_data
        assert response_data["log_level"] == "info"


async def test_nested_dependencies_3_levels_success() -> None:
    """Tests dependency resolution where auth depends on cache and db, which both depend on config."""

    async with TestClient(create_app_di_nested_dependencies_3_levels_success()) as client:
        response = await client.get("/api/auth-status")

        assert response.status_code == 200
        response_data = response.json()
        assert "auth_enabled" in response_data
        assert response_data["auth_enabled"] == True
        assert "has_db" in response_data
        assert response_data["has_db"] == True
        assert "has_cache" in response_data
        assert response_data["has_cache"] == True


async def test_type_mismatch_in_dependency_resolution_error() -> None:
    """Tests error when handler expects a different type than what the dependency provides."""

    async with TestClient(create_app_di_type_mismatch_in_dependency_resolution_error()) as client:
        response = await client.get("/api/type-mismatch")

        assert response.status_code == 500
        response_data = response.json()
        assert "type" in response_data
        assert response_data["type"] == "https://spikard.dev/errors/dependency-error"
        assert "title" in response_data
        assert response_data["title"] == "Dependency Resolution Failed"
        assert "status" in response_data
        assert response_data["status"] == 500
        assert "detail" in response_data
        assert response_data["detail"] == "Dependency type mismatch"
        assert "errors" in response_data
        assert len(response_data["errors"]) == 1
        assert "type" in response_data["errors"][0]
        assert "msg" in response_data["errors"][0]
        assert "dependency_key" in response_data["errors"][0]
        assert response_data["errors"][0]["dependency_key"] == "config"
        assert "expected_type" in response_data["errors"][0]
        assert response_data["errors"][0]["expected_type"] == "object"
        assert "actual_type" in response_data["errors"][0]
        assert response_data["errors"][0]["actual_type"] == "string"


async def test_missing_dependency_error() -> None:
    """Tests error when handler requires a dependency that was never registered."""

    async with TestClient(create_app_di_missing_dependency_error()) as client:
        response = await client.get("/api/missing-dep")

        assert response.status_code == 500
        response_data = response.json()
        assert "type" in response_data
        assert response_data["type"] == "https://spikard.dev/errors/dependency-error"
        assert "title" in response_data
        assert response_data["title"] == "Dependency Resolution Failed"
        assert "status" in response_data
        assert response_data["status"] == 500
        assert "detail" in response_data
        assert response_data["detail"] == "Required dependency not found"
        assert "errors" in response_data
        assert len(response_data["errors"]) == 1
        assert "type" in response_data["errors"][0]
        assert "msg" in response_data["errors"][0]
        assert "dependency_key" in response_data["errors"][0]
        assert response_data["errors"][0]["dependency_key"] == "non_existent_service"


async def test_python_parameter_name_based_injection_success() -> None:
    """Tests Python-specific parameter name-based dependency injection where dependencies are matched by parameter names."""

    async with TestClient(create_app_di_python_parameter_name_based_injection_success()) as client:
        response = await client.get("/api/python-name-inject")

        assert response.status_code == 200
        response_data = response.json()
        assert "db_status" in response_data
        assert response_data["db_status"] == "connected"
        assert "cache_status" in response_data
        assert response_data["cache_status"] == "ready"


async def test_dependency_injection_in_lifecycle_hooks_success() -> None:
    """Tests accessing dependencies in lifecycle hooks (onRequest, preHandler) for auth, logging, etc.."""

    async with TestClient(create_app_di_dependency_injection_in_lifecycle_hooks_success()) as client:
        headers = {
            "authorization": "Bearer valid_token",
        }
        response = await client.get("/api/hook-di-test", headers=headers)

        assert response.status_code == 200
        response_data = response.json()
        assert "authenticated" in response_data
        assert response_data["authenticated"] == True
        assert "logged" in response_data
        assert response_data["logged"] == True
        response_headers = response.headers
        assert response_headers.get("x-auth-mode") == "strict"
        assert response_headers.get("x-log-level") == "debug"


async def test_ruby_keyword_argument_injection_success() -> None:
    """Tests Ruby-specific keyword argument pattern for dependency injection."""

    async with TestClient(create_app_di_ruby_keyword_argument_injection_success()) as client:
        response = await client.get("/api/ruby-kwargs")

        assert response.status_code == 200
        response_data = response.json()
        assert "adapter" in response_data
        assert response_data["adapter"] == "postgresql"
        assert "user_id" in response_data
        assert response_data["user_id"] == 42


async def test_multiple_dependencies_with_cleanup_success() -> None:
    """Tests cleanup for multiple dependencies with generator pattern, ensuring all are cleaned up in reverse resolution order."""

    async with TestClient(create_app_di_multiple_dependencies_with_cleanup_success()) as client:
        response = await client.get("/api/multi-cleanup-test")

        assert response.status_code == 200
        state_response = await client.get("/api/multi-cleanup-state")
        assert state_response.status_code == 200
        assert state_response.json() == {
            "cleanup_order": [
                "db_opened",
                "cache_opened",
                "session_opened",
                "session_closed",
                "cache_closed",
                "db_closed",
            ]
        }


async def test_mixed_singleton_and_per_request_caching_success() -> None:
    """Tests mixing singleton dependencies (shared across requests) with per-request dependencies (cached within request)."""

    async with TestClient(create_app_di_mixed_singleton_and_per_request_caching_success()) as client:
        response = await client.get("/api/mixed-caching")

        assert response.status_code == 200

        # Second request to verify singleton caching
        response2 = await client.get("/api/mixed-caching")
        assert response2.status_code == 200
        data1 = response.json()
        data2 = response2.json()

        # pool_id is singleton; context_id is per-request
        assert "pool_id" in data1 and "pool_id" in data2
        assert data1["pool_id"] == data2["pool_id"]
        assert "context_id" in data1 and "context_id" in data2
        assert data1["context_id"] != data2["context_id"]


async def test_resource_cleanup_after_request_success() -> None:
    """Tests generator pattern cleanup where dependency resources are cleaned up after handler completes."""

    async with TestClient(create_app_di_resource_cleanup_after_request_success()) as client:
        response = await client.get("/api/cleanup-test")

        assert response.status_code == 200
        state_response = await client.get("/api/cleanup-state")
        assert state_response.status_code == 200
        assert state_response.json() == {"cleanup_events": ["session_opened", "session_closed"]}


async def test_python_type_annotation_based_injection_success() -> None:
    """Tests Python-specific type annotation-based dependency injection where dependencies are matched by type hints."""

    async with TestClient(create_app_di_python_type_annotation_based_injection_success()) as client:
        response = await client.get("/api/python-type-inject")

        assert response.status_code == 200
        response_data = response.json()
        assert "pool_type" in response_data
        assert response_data["pool_type"] == "PostgreSQL"
        assert "cache_type" in response_data
        assert response_data["cache_type"] == "Redis"


async def test_per_request_dependency_caching_success() -> None:
    """Tests per-request caching where dependency is created once per request but shared by multiple usages within that request."""

    async with TestClient(create_app_di_per_request_dependency_caching_success()) as client:
        response = await client.get("/api/request-id")

        assert response.status_code == 200
        response_data = response.json()
        assert "first_id" in response_data
        assert response_data["first_id"] == "<<uuid>>"
        assert "second_id" in response_data
        assert response_data["second_id"] == "<<same_as:first_id>>"


async def test_singleton_dependency_caching_success() -> None:
    """Tests singleton dependency that is created once and shared across all requests."""

    async with TestClient(create_app_di_singleton_dependency_caching_success()) as client:
        response = await client.get("/api/app-counter")

        assert response.status_code == 200

        # Second request to verify singleton caching
        response2 = await client.get("/api/app-counter")
        assert response2.status_code == 200
        data1 = response.json()
        data2 = response2.json()

        # Singleton counter should have stable counter_id and incremented count
        assert "counter_id" in data1 and "counter_id" in data2
        assert data1["counter_id"] == data2["counter_id"]
        assert data2["count"] > data1["count"]


async def test_async_factory_dependency_success() -> None:
    """Tests async factory that creates database pool asynchronously."""

    async with TestClient(create_app_di_async_factory_dependency_success()) as client:
        response = await client.get("/api/db-status")

        assert response.status_code == 200
        response_data = response.json()
        assert "pool_status" in response_data
        assert response_data["pool_status"] == "connected"
        assert "max_size" in response_data
        assert response_data["max_size"] == 10
