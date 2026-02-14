"""E2E tests for GraphQL operations."""

import pytest
from spikard.testing import TestClient
import app.main as app_main


@pytest.mark.asyncio
async def test_graphql_validation_directive() -> None:
    """Custom @length directive validating string field length constraints at execution time."""

    app_factory = getattr(app_main, "create_app_graphql_validation_directive", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_validation_directive")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="mutation CreateUser($input: CreateUserInput!) {\n  createUser(input: $input) {\n    id\n    name\n    bio\n  }\n}",
            variables={"input": {"name": "a", "bio": None}},
            operation_name=None,
            path="/graphql",
        )

        assert response.status_code == 422
        response_data = response.json()
        assert "errors" in response_data
        assert len(response_data["errors"]) == 1
        error_0 = response_data["errors"][0]
        assert (
            error_0["message"]
            == "Validation error on input field 'name': String length must be between 3 and 50 characters (provided: 1)"
        )
        assert error_0["path"] == ["createUser"]


@pytest.mark.asyncio
async def test_graphql_transform_directive() -> None:
    """Custom @uppercase directive transforming field output to uppercase."""

    app_factory = getattr(app_main, "create_app_graphql_transform_directive", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_transform_directive")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="query {\n  message @uppercase\n  title @uppercase\n}",
            variables=None,
            operation_name=None,
            path="/graphql",
        )

        assert response.status_code == 200
        response_data = response.json()
        assert "data" in response_data
        assert "message" in response_data["data"]
        assert response_data["data"]["message"] == "HELLO FROM GRAPHQL"
        assert "title" in response_data["data"]
        assert response_data["data"]["title"] == "WELCOME TO SPIKARD"
        response_data = response.json()
        assert response_data.get("errors") is None or response_data.get("errors") == []


@pytest.mark.asyncio
async def test_graphql_rate_limit_directive() -> None:
    """Custom @rateLimit directive enforcing request rate limiting on expensive fields."""

    app_factory = getattr(app_main, "create_app_graphql_rate_limit_directive", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_rate_limit_directive")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="query {\n  expensiveQuery\n}", variables=None, operation_name=None, path="/graphql"
        )

        assert response.status_code == 200
        response_data = response.json()
        assert "data" in response_data
        assert "expensiveQuery" in response_data["data"]
        assert response_data["data"]["expensiveQuery"] == "Result from expensive computation"
        response_data = response.json()
        assert response_data.get("errors") is None or response_data.get("errors") == []


@pytest.mark.asyncio
async def test_graphql_cache_directive() -> None:
    """Custom @cacheControl directive setting HTTP caching headers on GraphQL field resolution."""

    app_factory = getattr(app_main, "create_app_graphql_cache_directive", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_cache_directive")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="query GetUser($id: ID!) {\n  user(id: $id) {\n    id\n    name\n    email\n  }\n}",
            variables={"id": "1"},
            operation_name=None,
            path="/graphql",
        )

        assert response.status_code == 200
        response_data = response.json()
        assert "data" in response_data
        assert "user" in response_data["data"]
        assert "id" in response_data["data"]["user"]
        assert response_data["data"]["user"]["id"] == "1"
        assert "name" in response_data["data"]["user"]
        assert response_data["data"]["user"]["name"] == "Alice Smith"
        assert "email" in response_data["data"]["user"]
        assert response_data["data"]["user"]["email"] == "alice@example.com"
        response_data = response.json()
        assert response_data.get("errors") is None or response_data.get("errors") == []


@pytest.mark.asyncio
async def test_graphql_custom_auth_directive() -> None:
    """Custom @auth directive enforcing authorization rules based on user role."""

    app_factory = getattr(app_main, "create_app_graphql_custom_auth_directive", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_custom_auth_directive")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="query {\n  publicData\n  secretData\n  moderatorData\n}",
            variables=None,
            operation_name=None,
            path="/graphql",
        )

        assert response.status_code == 200
        response_data = response.json()
        assert "data" in response_data
        assert "publicData" in response_data["data"]
        assert response_data["data"]["publicData"] == "Anyone can see this"
        assert "secretData" in response_data["data"]
        assert response_data["data"]["secretData"] == None
        assert "moderatorData" in response_data["data"]
        assert response_data["data"]["moderatorData"] == None
        response_data = response.json()
        assert "errors" in response_data
        assert len(response_data["errors"]) == 2
        error_0 = response_data["errors"][0]
        assert error_0["message"] == "Unauthorized: User role USER cannot access ADMIN field"
        assert error_0["path"] == ["secretData"]
        error_1 = response_data["errors"][1]
        assert error_1["message"] == "Unauthorized: User role USER cannot access MODERATOR field"
        assert error_1["path"] == ["moderatorData"]


@pytest.mark.asyncio
async def test_graphql_deprecated_field() -> None:
    """Field with @deprecated directive showing deprecation warning in response extensions."""

    app_factory = getattr(app_main, "create_app_graphql_deprecated_field", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_deprecated_field")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="query {\n  oldField\n  newField\n}", variables=None, operation_name=None, path="/graphql"
        )

        assert response.status_code == 200
        response_data = response.json()
        assert "data" in response_data
        assert "oldField" in response_data["data"]
        assert response_data["data"]["oldField"] == "legacy value"
        assert "newField" in response_data["data"]
        assert response_data["data"]["newField"] == "modern value"
        response_data = response.json()
        assert response_data.get("errors") is None or response_data.get("errors") == []


@pytest.mark.asyncio
async def test_graphql_custom_scalar_invalid() -> None:
    """Custom scalar validation failure - all custom scalars receive invalid values."""

    app_factory = getattr(app_main, "create_app_graphql_custom_scalar_invalid", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_custom_scalar_invalid")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="mutation CreateContact($input: CreateContactInput!) {\n  createContact(input: $input) {\n    id\n    name\n    email\n    website\n    phone\n    createdAt\n  }\n}",
            variables={
                "input": {"name": "Invalid Contact", "email": "not-an-email", "website": "not a url", "phone": "123"}
            },
            operation_name="CreateContact",
            path="/graphql",
        )

        assert response.status_code == 422
        response_data = response.json()
        assert "errors" in response_data
        assert len(response_data["errors"]) == 3
        error_0 = response_data["errors"][0]
        assert error_0["message"] == "Email must be a valid email address"
        assert "locations" in error_0
        assert len(error_0["locations"]) >= 1
        error_1 = response_data["errors"][1]
        assert error_1["message"] == "URL must start with http:// or https://"
        assert "locations" in error_1
        assert len(error_1["locations"]) >= 1
        error_2 = response_data["errors"][2]
        assert error_2["message"] == "PhoneNumber must be a valid E.164 format"
        assert "locations" in error_2
        assert len(error_2["locations"]) >= 1


@pytest.mark.asyncio
async def test_graphql_datetime_scalar() -> None:
    """Custom DateTime scalar type handling with ISO 8601 format validation."""

    app_factory = getattr(app_main, "create_app_graphql_datetime_scalar", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_datetime_scalar")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="query GetEvents($since: DateTime, $until: DateTime) {\n  events(since: $since, until: $until) {\n    id\n    title\n    scheduledAt\n    completedAt\n  }\n}",
            variables={"since": "2025-01-01T00:00:00Z", "until": "2025-12-31T23:59:59Z"},
            operation_name=None,
            path="/graphql",
        )

        assert response.status_code == 200
        response_data = response.json()
        assert "data" in response_data
        assert "events" in response_data["data"]
        assert len(response_data["data"]["events"]) == 2
        assert "id" in response_data["data"]["events"][0]
        assert response_data["data"]["events"][0]["id"] == "event-1"
        assert "title" in response_data["data"]["events"][0]
        assert response_data["data"]["events"][0]["title"] == "Conference"
        assert "scheduledAt" in response_data["data"]["events"][0]
        assert response_data["data"]["events"][0]["scheduledAt"] == "2025-06-15T09:00:00Z"
        assert "completedAt" in response_data["data"]["events"][0]
        assert response_data["data"]["events"][0]["completedAt"] == "2025-06-15T17:00:00Z"
        assert "id" in response_data["data"]["events"][1]
        assert response_data["data"]["events"][1]["id"] == "event-2"
        assert "title" in response_data["data"]["events"][1]
        assert response_data["data"]["events"][1]["title"] == "Hackathon"
        assert "scheduledAt" in response_data["data"]["events"][1]
        assert response_data["data"]["events"][1]["scheduledAt"] == "2025-08-20T10:00:00Z"
        assert "completedAt" in response_data["data"]["events"][1]
        assert response_data["data"]["events"][1]["completedAt"] == None
        response_data = response.json()
        assert response_data.get("errors") is None or response_data.get("errors") == []


@pytest.mark.asyncio
async def test_graphql_custom_scalar_validation() -> None:
    """Multiple custom scalars with validation - Email, URL, and PhoneNumber types."""

    app_factory = getattr(app_main, "create_app_graphql_custom_scalar_validation", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_custom_scalar_validation")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="mutation CreateContact($input: CreateContactInput!) {\n  createContact(input: $input) {\n    id\n    name\n    email\n    website\n    phone\n    createdAt\n  }\n}",
            variables={
                "input": {
                    "name": "Alice Johnson",
                    "email": "alice.johnson@example.com",
                    "website": "https://example.com",
                    "phone": "+1-555-123-4567",
                }
            },
            operation_name="CreateContact",
            path="/graphql",
        )

        assert response.status_code == 200
        response_data = response.json()
        assert "data" in response_data
        assert "createContact" in response_data["data"]
        assert "id" in response_data["data"]["createContact"]
        assert response_data["data"]["createContact"]["id"] == "contact-001"
        assert "name" in response_data["data"]["createContact"]
        assert response_data["data"]["createContact"]["name"] == "Alice Johnson"
        assert "email" in response_data["data"]["createContact"]
        assert response_data["data"]["createContact"]["email"] == "alice.johnson@example.com"
        assert "website" in response_data["data"]["createContact"]
        assert response_data["data"]["createContact"]["website"] == "https://example.com"
        assert "phone" in response_data["data"]["createContact"]
        assert response_data["data"]["createContact"]["phone"] == "+1-555-123-4567"
        assert "createdAt" in response_data["data"]["createContact"]
        assert response_data["data"]["createContact"]["createdAt"] == "2025-12-27T14:30:00Z"
        response_data = response.json()
        assert response_data.get("errors") is None or response_data.get("errors") == []


@pytest.mark.asyncio
async def test_graphql_uuid_scalar() -> None:
    """Custom UUID scalar type validation with RFC 4122 format."""

    app_factory = getattr(app_main, "create_app_graphql_uuid_scalar", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_uuid_scalar")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="query GetResource($id: UUID!) {\n  resource(id: $id) {\n    id\n    parentId\n    name\n    ownerId\n    relatedIds\n  }\n}",
            variables={"id": "550e8400-e29b-41d4-a716-446655440000"},
            operation_name=None,
            path="/graphql",
        )

        assert response.status_code == 200
        response_data = response.json()
        assert "data" in response_data
        assert "resource" in response_data["data"]
        assert "id" in response_data["data"]["resource"]
        assert response_data["data"]["resource"]["id"] == "550e8400-e29b-41d4-a716-446655440000"
        assert "parentId" in response_data["data"]["resource"]
        assert response_data["data"]["resource"]["parentId"] == "6ba7b810-9dad-11d1-80b4-00c04fd430c8"
        assert "name" in response_data["data"]["resource"]
        assert response_data["data"]["resource"]["name"] == "Primary Resource"
        assert "ownerId" in response_data["data"]["resource"]
        assert response_data["data"]["resource"]["ownerId"] == "6ba7b811-9dad-11d1-80b4-00c04fd430c8"
        assert "relatedIds" in response_data["data"]["resource"]
        assert len(response_data["data"]["resource"]["relatedIds"]) == 2
        assert response_data["data"]["resource"]["relatedIds"][0] == "6ba7b812-9dad-11d1-80b4-00c04fd430c8"
        assert response_data["data"]["resource"]["relatedIds"][1] == "6ba7b814-9dad-11d1-80b4-00c04fd430c8"
        response_data = response.json()
        assert response_data.get("errors") is None or response_data.get("errors") == []


@pytest.mark.asyncio
async def test_graphql_json_scalar() -> None:
    """Custom JSON scalar type for arbitrary JSON data structures."""

    app_factory = getattr(app_main, "create_app_graphql_json_scalar", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_json_scalar")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="query GetConfig {\n  configuration {\n    id\n    name\n    settings\n    metadata\n  }\n}",
            variables=None,
            operation_name=None,
            path="/graphql",
        )

        assert response.status_code == 200
        response_data = response.json()
        assert "data" in response_data
        assert "configuration" in response_data["data"]
        assert "id" in response_data["data"]["configuration"]
        assert response_data["data"]["configuration"]["id"] == "config-1"
        assert "name" in response_data["data"]["configuration"]
        assert response_data["data"]["configuration"]["name"] == "Production Config"
        assert "settings" in response_data["data"]["configuration"]
        assert "timeout" in response_data["data"]["configuration"]["settings"]
        assert response_data["data"]["configuration"]["settings"]["timeout"] == 30000
        assert "retries" in response_data["data"]["configuration"]["settings"]
        assert response_data["data"]["configuration"]["settings"]["retries"] == 3
        assert "features" in response_data["data"]["configuration"]["settings"]
        assert "caching" in response_data["data"]["configuration"]["settings"]["features"]
        assert response_data["data"]["configuration"]["settings"]["features"]["caching"] == True
        assert "compression" in response_data["data"]["configuration"]["settings"]["features"]
        assert response_data["data"]["configuration"]["settings"]["features"]["compression"] == True
        assert "tracing" in response_data["data"]["configuration"]["settings"]["features"]
        assert response_data["data"]["configuration"]["settings"]["features"]["tracing"] == False
        assert "endpoints" in response_data["data"]["configuration"]["settings"]
        assert len(response_data["data"]["configuration"]["settings"]["endpoints"]) == 2
        assert response_data["data"]["configuration"]["settings"]["endpoints"][0] == "https://api.example.com"
        assert response_data["data"]["configuration"]["settings"]["endpoints"][1] == "https://api-backup.example.com"
        assert "metadata" in response_data["data"]["configuration"]
        assert "version" in response_data["data"]["configuration"]["metadata"]
        assert response_data["data"]["configuration"]["metadata"]["version"] == "1.0.0"
        assert "environment" in response_data["data"]["configuration"]["metadata"]
        assert response_data["data"]["configuration"]["metadata"]["environment"] == "production"
        assert "lastUpdated" in response_data["data"]["configuration"]["metadata"]
        assert response_data["data"]["configuration"]["metadata"]["lastUpdated"] == "2025-12-27T10:00:00Z"
        assert "author" in response_data["data"]["configuration"]["metadata"]
        assert response_data["data"]["configuration"]["metadata"]["author"] == "DevOps Team"
        response_data = response.json()
        assert response_data.get("errors") is None or response_data.get("errors") == []


@pytest.mark.asyncio
async def test_graphql_create_resource() -> None:
    """Mutation that creates a new resource with input type and returns the created object."""

    app_factory = getattr(app_main, "create_app_graphql_create_resource", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_create_resource")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="mutation CreateUser($input: CreateUserInput!) {\n  createUser(input: $input) {\n    id\n    name\n    email\n    role\n    createdAt\n  }\n}",
            variables={"input": {"name": "John Doe", "email": "john@example.com", "role": "admin"}},
            operation_name="CreateUser",
            path="/graphql",
        )

        assert response.status_code == 200
        response_data = response.json()
        assert "data" in response_data
        assert "createUser" in response_data["data"]
        assert "id" in response_data["data"]["createUser"]
        assert response_data["data"]["createUser"]["id"] == "user-123"
        assert "name" in response_data["data"]["createUser"]
        assert response_data["data"]["createUser"]["name"] == "John Doe"
        assert "email" in response_data["data"]["createUser"]
        assert response_data["data"]["createUser"]["email"] == "john@example.com"
        assert "role" in response_data["data"]["createUser"]
        assert response_data["data"]["createUser"]["role"] == "admin"
        assert "createdAt" in response_data["data"]["createUser"]
        assert response_data["data"]["createUser"]["createdAt"] == "2025-12-27T10:30:00Z"
        response_data = response.json()
        assert response_data.get("errors") is None or response_data.get("errors") == []


@pytest.mark.asyncio
async def test_graphql_delete_resource() -> None:
    """Mutation that deletes a resource by ID and returns success status."""

    app_factory = getattr(app_main, "create_app_graphql_delete_resource", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_delete_resource")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="mutation DeleteUser($id: ID!) {\n  deleteUser(id: $id) {\n    success\n    message\n    deletedId\n  }\n}",
            variables={"id": "user-123"},
            operation_name="DeleteUser",
            path="/graphql",
        )

        assert response.status_code == 200
        response_data = response.json()
        assert "data" in response_data
        assert "deleteUser" in response_data["data"]
        assert "success" in response_data["data"]["deleteUser"]
        assert response_data["data"]["deleteUser"]["success"] == True
        assert "message" in response_data["data"]["deleteUser"]
        assert response_data["data"]["deleteUser"]["message"] == "User successfully deleted"
        assert "deletedId" in response_data["data"]["deleteUser"]
        assert response_data["data"]["deleteUser"]["deletedId"] == "user-123"
        response_data = response.json()
        assert response_data.get("errors") is None or response_data.get("errors") == []


@pytest.mark.asyncio
async def test_graphql_update_resource() -> None:
    """Mutation that updates a resource with partial input fields."""

    app_factory = getattr(app_main, "create_app_graphql_update_resource", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_update_resource")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="mutation UpdateUser($id: ID!, $input: UpdateUserInput!) {\n  updateUser(id: $id, input: $input) {\n    id\n    name\n    email\n    role\n    updatedAt\n  }\n}",
            variables={"id": "user-123", "input": {"email": "john.doe@example.com", "role": "editor"}},
            operation_name="UpdateUser",
            path="/graphql",
        )

        assert response.status_code == 200
        response_data = response.json()
        assert "data" in response_data
        assert "updateUser" in response_data["data"]
        assert "id" in response_data["data"]["updateUser"]
        assert response_data["data"]["updateUser"]["id"] == "user-123"
        assert "name" in response_data["data"]["updateUser"]
        assert response_data["data"]["updateUser"]["name"] == "John Doe"
        assert "email" in response_data["data"]["updateUser"]
        assert response_data["data"]["updateUser"]["email"] == "john.doe@example.com"
        assert "role" in response_data["data"]["updateUser"]
        assert response_data["data"]["updateUser"]["role"] == "editor"
        assert "updatedAt" in response_data["data"]["updateUser"]
        assert response_data["data"]["updateUser"]["updatedAt"] == "2025-12-27T11:45:00Z"
        response_data = response.json()
        assert response_data.get("errors") is None or response_data.get("errors") == []


@pytest.mark.asyncio
async def test_graphql_complex_query() -> None:
    """Query with high complexity score testing multiple fields, aliases, and fragments."""

    app_factory = getattr(app_main, "create_app_graphql_complex_query", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_complex_query")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="query ComplexSearch($searchTerm: String!, $userLimit: Int!, $postLimit: Int!) {\n  search(term: $searchTerm) {\n    total\n    users(limit: $userLimit) {\n      id\n      name\n      email\n      profile {\n        bio\n        avatar\n        joinedAt\n      }\n      recentPosts: posts(limit: 3) {\n        id\n        title\n        likes\n      }\n      followerCount: followers(limit: 100) {\n        id\n      }\n    }\n    posts(limit: $postLimit) {\n      id\n      title\n      content\n      likes\n      author {\n        id\n        name\n        profile {\n          avatar\n        }\n      }\n      topComments: comments(limit: 5) {\n        id\n        text\n        likes\n        author {\n          id\n          name\n        }\n      }\n    }\n  }\n}",
            variables={"searchTerm": "graphql", "userLimit": 5, "postLimit": 10},
            operation_name="ComplexSearch",
            path="/graphql",
        )

        assert response.status_code == 200
        response_data = response.json()
        assert "data" in response_data
        assert "search" in response_data["data"]
        assert "total" in response_data["data"]["search"]
        assert response_data["data"]["search"]["total"] == 42
        assert "users" in response_data["data"]["search"]
        assert len(response_data["data"]["search"]["users"]) == 2
        assert "id" in response_data["data"]["search"]["users"][0]
        assert response_data["data"]["search"]["users"][0]["id"] == "user-1"
        assert "name" in response_data["data"]["search"]["users"][0]
        assert response_data["data"]["search"]["users"][0]["name"] == "GraphQL Expert"
        assert "email" in response_data["data"]["search"]["users"][0]
        assert response_data["data"]["search"]["users"][0]["email"] == "expert@example.com"
        assert "profile" in response_data["data"]["search"]["users"][0]
        assert "bio" in response_data["data"]["search"]["users"][0]["profile"]
        assert response_data["data"]["search"]["users"][0]["profile"]["bio"] == "GraphQL enthusiast and API designer"
        assert "avatar" in response_data["data"]["search"]["users"][0]["profile"]
        assert (
            response_data["data"]["search"]["users"][0]["profile"]["avatar"] == "https://example.com/avatars/expert.jpg"
        )
        assert "joinedAt" in response_data["data"]["search"]["users"][0]["profile"]
        assert response_data["data"]["search"]["users"][0]["profile"]["joinedAt"] == "2024-01-15T08:30:00Z"
        assert "recentPosts" in response_data["data"]["search"]["users"][0]
        assert len(response_data["data"]["search"]["users"][0]["recentPosts"]) == 3
        assert "id" in response_data["data"]["search"]["users"][0]["recentPosts"][0]
        assert response_data["data"]["search"]["users"][0]["recentPosts"][0]["id"] == "post-101"
        assert "title" in response_data["data"]["search"]["users"][0]["recentPosts"][0]
        assert response_data["data"]["search"]["users"][0]["recentPosts"][0]["title"] == "GraphQL Best Practices"
        assert "likes" in response_data["data"]["search"]["users"][0]["recentPosts"][0]
        assert response_data["data"]["search"]["users"][0]["recentPosts"][0]["likes"] == 234
        assert "id" in response_data["data"]["search"]["users"][0]["recentPosts"][1]
        assert response_data["data"]["search"]["users"][0]["recentPosts"][1]["id"] == "post-102"
        assert "title" in response_data["data"]["search"]["users"][0]["recentPosts"][1]
        assert response_data["data"]["search"]["users"][0]["recentPosts"][1]["title"] == "Schema Design Patterns"
        assert "likes" in response_data["data"]["search"]["users"][0]["recentPosts"][1]
        assert response_data["data"]["search"]["users"][0]["recentPosts"][1]["likes"] == 189
        assert "id" in response_data["data"]["search"]["users"][0]["recentPosts"][2]
        assert response_data["data"]["search"]["users"][0]["recentPosts"][2]["id"] == "post-103"
        assert "title" in response_data["data"]["search"]["users"][0]["recentPosts"][2]
        assert response_data["data"]["search"]["users"][0]["recentPosts"][2]["title"] == "Performance Optimization"
        assert "likes" in response_data["data"]["search"]["users"][0]["recentPosts"][2]
        assert response_data["data"]["search"]["users"][0]["recentPosts"][2]["likes"] == 156
        assert "followerCount" in response_data["data"]["search"]["users"][0]
        assert len(response_data["data"]["search"]["users"][0]["followerCount"]) == 2
        assert "id" in response_data["data"]["search"]["users"][0]["followerCount"][0]
        assert response_data["data"]["search"]["users"][0]["followerCount"][0]["id"] == "user-2"
        assert "id" in response_data["data"]["search"]["users"][0]["followerCount"][1]
        assert response_data["data"]["search"]["users"][0]["followerCount"][1]["id"] == "user-3"
        assert "id" in response_data["data"]["search"]["users"][1]
        assert response_data["data"]["search"]["users"][1]["id"] == "user-2"
        assert "name" in response_data["data"]["search"]["users"][1]
        assert response_data["data"]["search"]["users"][1]["name"] == "API Developer"
        assert "email" in response_data["data"]["search"]["users"][1]
        assert response_data["data"]["search"]["users"][1]["email"] == "developer@example.com"
        assert "profile" in response_data["data"]["search"]["users"][1]
        assert "bio" in response_data["data"]["search"]["users"][1]["profile"]
        assert response_data["data"]["search"]["users"][1]["profile"]["bio"] == "Building scalable APIs"
        assert "avatar" in response_data["data"]["search"]["users"][1]["profile"]
        assert (
            response_data["data"]["search"]["users"][1]["profile"]["avatar"]
            == "https://example.com/avatars/developer.jpg"
        )
        assert "joinedAt" in response_data["data"]["search"]["users"][1]["profile"]
        assert response_data["data"]["search"]["users"][1]["profile"]["joinedAt"] == "2024-02-20T10:15:00Z"
        assert "recentPosts" in response_data["data"]["search"]["users"][1]
        assert len(response_data["data"]["search"]["users"][1]["recentPosts"]) == 1
        assert "id" in response_data["data"]["search"]["users"][1]["recentPosts"][0]
        assert response_data["data"]["search"]["users"][1]["recentPosts"][0]["id"] == "post-201"
        assert "title" in response_data["data"]["search"]["users"][1]["recentPosts"][0]
        assert response_data["data"]["search"]["users"][1]["recentPosts"][0]["title"] == "GraphQL vs REST"
        assert "likes" in response_data["data"]["search"]["users"][1]["recentPosts"][0]
        assert response_data["data"]["search"]["users"][1]["recentPosts"][0]["likes"] == 145
        assert "followerCount" in response_data["data"]["search"]["users"][1]
        assert len(response_data["data"]["search"]["users"][1]["followerCount"]) == 1
        assert "id" in response_data["data"]["search"]["users"][1]["followerCount"][0]
        assert response_data["data"]["search"]["users"][1]["followerCount"][0]["id"] == "user-1"
        assert "posts" in response_data["data"]["search"]
        assert len(response_data["data"]["search"]["posts"]) == 2
        assert "id" in response_data["data"]["search"]["posts"][0]
        assert response_data["data"]["search"]["posts"][0]["id"] == "post-101"
        assert "title" in response_data["data"]["search"]["posts"][0]
        assert response_data["data"]["search"]["posts"][0]["title"] == "GraphQL Best Practices"
        assert "content" in response_data["data"]["search"]["posts"][0]
        assert (
            response_data["data"]["search"]["posts"][0]["content"]
            == "A comprehensive guide to GraphQL best practices and patterns..."
        )
        assert "likes" in response_data["data"]["search"]["posts"][0]
        assert response_data["data"]["search"]["posts"][0]["likes"] == 234
        assert "author" in response_data["data"]["search"]["posts"][0]
        assert "id" in response_data["data"]["search"]["posts"][0]["author"]
        assert response_data["data"]["search"]["posts"][0]["author"]["id"] == "user-1"
        assert "name" in response_data["data"]["search"]["posts"][0]["author"]
        assert response_data["data"]["search"]["posts"][0]["author"]["name"] == "GraphQL Expert"
        assert "profile" in response_data["data"]["search"]["posts"][0]["author"]
        assert "avatar" in response_data["data"]["search"]["posts"][0]["author"]["profile"]
        assert (
            response_data["data"]["search"]["posts"][0]["author"]["profile"]["avatar"]
            == "https://example.com/avatars/expert.jpg"
        )
        assert "topComments" in response_data["data"]["search"]["posts"][0]
        assert len(response_data["data"]["search"]["posts"][0]["topComments"]) == 2
        assert "id" in response_data["data"]["search"]["posts"][0]["topComments"][0]
        assert response_data["data"]["search"]["posts"][0]["topComments"][0]["id"] == "comment-1"
        assert "text" in response_data["data"]["search"]["posts"][0]["topComments"][0]
        assert response_data["data"]["search"]["posts"][0]["topComments"][0]["text"] == "Great post, very helpful!"
        assert "likes" in response_data["data"]["search"]["posts"][0]["topComments"][0]
        assert response_data["data"]["search"]["posts"][0]["topComments"][0]["likes"] == 45
        assert "author" in response_data["data"]["search"]["posts"][0]["topComments"][0]
        assert "id" in response_data["data"]["search"]["posts"][0]["topComments"][0]["author"]
        assert response_data["data"]["search"]["posts"][0]["topComments"][0]["author"]["id"] == "user-2"
        assert "name" in response_data["data"]["search"]["posts"][0]["topComments"][0]["author"]
        assert response_data["data"]["search"]["posts"][0]["topComments"][0]["author"]["name"] == "API Developer"
        assert "id" in response_data["data"]["search"]["posts"][0]["topComments"][1]
        assert response_data["data"]["search"]["posts"][0]["topComments"][1]["id"] == "comment-2"
        assert "text" in response_data["data"]["search"]["posts"][0]["topComments"][1]
        assert (
            response_data["data"]["search"]["posts"][0]["topComments"][1]["text"] == "Could you elaborate on caching?"
        )
        assert "likes" in response_data["data"]["search"]["posts"][0]["topComments"][1]
        assert response_data["data"]["search"]["posts"][0]["topComments"][1]["likes"] == 32
        assert "author" in response_data["data"]["search"]["posts"][0]["topComments"][1]
        assert "id" in response_data["data"]["search"]["posts"][0]["topComments"][1]["author"]
        assert response_data["data"]["search"]["posts"][0]["topComments"][1]["author"]["id"] == "user-3"
        assert "name" in response_data["data"]["search"]["posts"][0]["topComments"][1]["author"]
        assert response_data["data"]["search"]["posts"][0]["topComments"][1]["author"]["name"] == "Data Scientist"
        assert "id" in response_data["data"]["search"]["posts"][1]
        assert response_data["data"]["search"]["posts"][1]["id"] == "post-102"
        assert "title" in response_data["data"]["search"]["posts"][1]
        assert response_data["data"]["search"]["posts"][1]["title"] == "Schema Design Patterns"
        assert "content" in response_data["data"]["search"]["posts"][1]
        assert (
            response_data["data"]["search"]["posts"][1]["content"]
            == "Exploring common patterns for designing GraphQL schemas..."
        )
        assert "likes" in response_data["data"]["search"]["posts"][1]
        assert response_data["data"]["search"]["posts"][1]["likes"] == 189
        assert "author" in response_data["data"]["search"]["posts"][1]
        assert "id" in response_data["data"]["search"]["posts"][1]["author"]
        assert response_data["data"]["search"]["posts"][1]["author"]["id"] == "user-1"
        assert "name" in response_data["data"]["search"]["posts"][1]["author"]
        assert response_data["data"]["search"]["posts"][1]["author"]["name"] == "GraphQL Expert"
        assert "profile" in response_data["data"]["search"]["posts"][1]["author"]
        assert "avatar" in response_data["data"]["search"]["posts"][1]["author"]["profile"]
        assert (
            response_data["data"]["search"]["posts"][1]["author"]["profile"]["avatar"]
            == "https://example.com/avatars/expert.jpg"
        )
        assert "topComments" in response_data["data"]["search"]["posts"][1]
        assert len(response_data["data"]["search"]["posts"][1]["topComments"]) == 1
        assert "id" in response_data["data"]["search"]["posts"][1]["topComments"][0]
        assert response_data["data"]["search"]["posts"][1]["topComments"][0]["id"] == "comment-3"
        assert "text" in response_data["data"]["search"]["posts"][1]["topComments"][0]
        assert response_data["data"]["search"]["posts"][1]["topComments"][0]["text"] == "Excellent breakdown"
        assert "likes" in response_data["data"]["search"]["posts"][1]["topComments"][0]
        assert response_data["data"]["search"]["posts"][1]["topComments"][0]["likes"] == 28
        assert "author" in response_data["data"]["search"]["posts"][1]["topComments"][0]
        assert "id" in response_data["data"]["search"]["posts"][1]["topComments"][0]["author"]
        assert response_data["data"]["search"]["posts"][1]["topComments"][0]["author"]["id"] == "user-4"
        assert "name" in response_data["data"]["search"]["posts"][1]["topComments"][0]["author"]
        assert response_data["data"]["search"]["posts"][1]["topComments"][0]["author"]["name"] == "Backend Engineer"
        response_data = response.json()
        assert response_data.get("errors") is None or response_data.get("errors") == []


@pytest.mark.asyncio
async def test_graphql_deeply_nested_query() -> None:
    """Query with 5+ levels of nesting to test query depth limits and resolver chain performance."""

    app_factory = getattr(app_main, "create_app_graphql_deeply_nested_query", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_deeply_nested_query")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="query GetUserDeepNested($userId: String!) {\n  user(id: $userId) {\n    id\n    name\n    profile {\n      bio\n      settings {\n        preferences {\n          theme\n          language\n          timezone {\n            name\n            offset\n          }\n        }\n        notifications {\n          email\n          push\n        }\n      }\n    }\n  }\n}",
            variables={"userId": "user-deep-001"},
            operation_name=None,
            path="/graphql",
        )

        assert response.status_code == 200
        response_data = response.json()
        assert "data" in response_data
        assert "user" in response_data["data"]
        assert "id" in response_data["data"]["user"]
        assert response_data["data"]["user"]["id"] == "user-deep-001"
        assert "name" in response_data["data"]["user"]
        assert response_data["data"]["user"]["name"] == "Alice Cooper"
        assert "profile" in response_data["data"]["user"]
        assert "bio" in response_data["data"]["user"]["profile"]
        assert response_data["data"]["user"]["profile"]["bio"] == "DevOps engineer passionate about scalability"
        assert "settings" in response_data["data"]["user"]["profile"]
        assert "preferences" in response_data["data"]["user"]["profile"]["settings"]
        assert "theme" in response_data["data"]["user"]["profile"]["settings"]["preferences"]
        assert response_data["data"]["user"]["profile"]["settings"]["preferences"]["theme"] == "dark"
        assert "language" in response_data["data"]["user"]["profile"]["settings"]["preferences"]
        assert response_data["data"]["user"]["profile"]["settings"]["preferences"]["language"] == "en-US"
        assert "timezone" in response_data["data"]["user"]["profile"]["settings"]["preferences"]
        assert "name" in response_data["data"]["user"]["profile"]["settings"]["preferences"]["timezone"]
        assert (
            response_data["data"]["user"]["profile"]["settings"]["preferences"]["timezone"]["name"]
            == "America/Los_Angeles"
        )
        assert "offset" in response_data["data"]["user"]["profile"]["settings"]["preferences"]["timezone"]
        assert response_data["data"]["user"]["profile"]["settings"]["preferences"]["timezone"]["offset"] == -480
        assert "notifications" in response_data["data"]["user"]["profile"]["settings"]
        assert "email" in response_data["data"]["user"]["profile"]["settings"]["notifications"]
        assert response_data["data"]["user"]["profile"]["settings"]["notifications"]["email"] == True
        assert "push" in response_data["data"]["user"]["profile"]["settings"]["notifications"]
        assert response_data["data"]["user"]["profile"]["settings"]["notifications"]["push"] == False
        response_data = response.json()
        assert response_data.get("errors") is None or response_data.get("errors") == []


@pytest.mark.asyncio
async def test_graphql_persisted_query_allowlist() -> None:
    """Persisted query allowlist enforcement - server rejects unknown persisted queries."""

    app_factory = getattr(app_main, "create_app_graphql_persisted_query_allowlist", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_persisted_query_allowlist")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(query="", variables={}, operation_name=None, path="/graphql")

        assert response.status_code == 403
        response_data = response.json()
        assert "errors" in response_data
        assert len(response_data["errors"]) == 1
        error_0 = response_data["errors"][0]
        assert error_0["message"] == "Query not in allowlist"


@pytest.mark.asyncio
async def test_graphql_persisted_query_hash_mismatch() -> None:
    """Hash mismatch - query string provided but hash does not match the query content."""

    app_factory = getattr(app_main, "create_app_graphql_persisted_query_hash_mismatch", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_persisted_query_hash_mismatch")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="query GetUser($id: ID!) {\n  user(id: $id) {\n    id\n    name\n    email\n  }\n}",
            variables={"id": "user-999"},
            operation_name="GetUser",
            path="/graphql",
        )

        assert response.status_code == 400
        response_data = response.json()
        assert "errors" in response_data
        assert len(response_data["errors"]) == 1
        error_0 = response_data["errors"][0]
        assert error_0["message"] == "Hash mismatch"


@pytest.mark.asyncio
async def test_graphql_persisted_query_registration() -> None:
    """Register new persisted query - first request with both query string and hash, server caches it."""

    app_factory = getattr(app_main, "create_app_graphql_persisted_query_registration", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_persisted_query_registration")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="query GetUserPosts($userId: ID!) {\n  posts(userId: $userId) {\n    id\n    title\n    content\n    author {\n      id\n      name\n    }\n  }\n}",
            variables={"userId": "user-789"},
            operation_name="GetUserPosts",
            path="/graphql",
        )

        assert response.status_code == 200
        response_data = response.json()
        assert "data" in response_data
        assert "posts" in response_data["data"]
        assert len(response_data["data"]["posts"]) == 2
        assert "id" in response_data["data"]["posts"][0]
        assert response_data["data"]["posts"][0]["id"] == "post-1"
        assert "title" in response_data["data"]["posts"][0]
        assert response_data["data"]["posts"][0]["title"] == "GraphQL Best Practices"
        assert "content" in response_data["data"]["posts"][0]
        assert response_data["data"]["posts"][0]["content"] == "Understanding GraphQL query optimization..."
        assert "author" in response_data["data"]["posts"][0]
        assert "id" in response_data["data"]["posts"][0]["author"]
        assert response_data["data"]["posts"][0]["author"]["id"] == "user-789"
        assert "name" in response_data["data"]["posts"][0]["author"]
        assert response_data["data"]["posts"][0]["author"]["name"] == "Bob Johnson"
        assert "id" in response_data["data"]["posts"][1]
        assert response_data["data"]["posts"][1]["id"] == "post-2"
        assert "title" in response_data["data"]["posts"][1]
        assert response_data["data"]["posts"][1]["title"] == "Persisted Queries Guide"
        assert "content" in response_data["data"]["posts"][1]
        assert response_data["data"]["posts"][1]["content"] == "How to implement persisted queries for performance..."
        assert "author" in response_data["data"]["posts"][1]
        assert "id" in response_data["data"]["posts"][1]["author"]
        assert response_data["data"]["posts"][1]["author"]["id"] == "user-789"
        assert "name" in response_data["data"]["posts"][1]["author"]
        assert response_data["data"]["posts"][1]["author"]["name"] == "Bob Johnson"
        response_data = response.json()
        assert response_data.get("errors") is None or response_data.get("errors") == []


@pytest.mark.asyncio
async def test_graphql_persisted_query_hit() -> None:
    """Persisted query cache hit - query already cached on server, hash only in request."""

    app_factory = getattr(app_main, "create_app_graphql_persisted_query_hit", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_persisted_query_hit")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(query="", variables={"id": "user-123"}, operation_name=None, path="/graphql")

        assert response.status_code == 200
        response_data = response.json()
        assert "data" in response_data
        assert "user" in response_data["data"]
        assert "id" in response_data["data"]["user"]
        assert response_data["data"]["user"]["id"] == "user-123"
        assert "name" in response_data["data"]["user"]
        assert response_data["data"]["user"]["name"] == "Alice Smith"
        assert "email" in response_data["data"]["user"]
        assert response_data["data"]["user"]["email"] == "alice@example.com"
        response_data = response.json()
        assert response_data.get("errors") is None or response_data.get("errors") == []


@pytest.mark.asyncio
async def test_graphql_persisted_query_miss() -> None:
    """Persisted query cache miss - server does not have cached query for given hash."""

    app_factory = getattr(app_main, "create_app_graphql_persisted_query_miss", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_persisted_query_miss")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(query="", variables={"id": "user-456"}, operation_name=None, path="/graphql")

        assert response.status_code == 400
        response_data = response.json()
        assert "errors" in response_data
        assert len(response_data["errors"]) == 1
        error_0 = response_data["errors"][0]
        assert error_0["message"] == "PersistedQueryNotFound"


@pytest.mark.asyncio
async def test_graphql_persisted_query_automatic_persisted() -> None:
    """Automatic Persisted Queries (APQ) - Step 1 of 3: Client sends hash only, receives miss, must retry with full query."""

    app_factory = getattr(app_main, "create_app_graphql_persisted_query_automatic_persisted", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_persisted_query_automatic_persisted")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(query="", variables={"q": "GraphQL"}, operation_name=None, path="/graphql")

        assert response.status_code == 400
        response_data = response.json()
        assert "errors" in response_data
        assert len(response_data["errors"]) == 1
        error_0 = response_data["errors"][0]
        assert error_0["message"] == "PersistedQueryNotFound"


@pytest.mark.asyncio
async def test_graphql_with_arguments() -> None:
    """Query with scalar arguments and variable substitution."""

    app_factory = getattr(app_main, "create_app_graphql_with_arguments", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_with_arguments")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="query Greet($name: String!) {\n  greet(name: $name)\n}",
            variables={"name": "Alice"},
            operation_name="Greet",
            path="/graphql",
        )

        assert response.status_code == 200
        response_data = response.json()
        assert "data" in response_data
        assert "greet" in response_data["data"]
        assert response_data["data"]["greet"] == "Hello, Alice!"
        response_data = response.json()
        assert response_data.get("errors") is None or response_data.get("errors") == []


@pytest.mark.asyncio
async def test_graphql_nested_objects() -> None:
    """Query with nested object selection and traversal."""

    app_factory = getattr(app_main, "create_app_graphql_nested_objects", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_nested_objects")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="query GetUser($userId: String!) {\n  user(id: $userId) {\n    id\n    name\n    email\n    profile {\n      bio\n      location\n    }\n  }\n}",
            variables={"userId": "550e8400-e29b-41d4-a716-446655440000"},
            operation_name=None,
            path="/graphql",
        )

        assert response.status_code == 200
        response_data = response.json()
        assert "data" in response_data
        assert "user" in response_data["data"]
        assert "id" in response_data["data"]["user"]
        assert response_data["data"]["user"]["id"] == "550e8400-e29b-41d4-a716-446655440000"
        assert "name" in response_data["data"]["user"]
        assert response_data["data"]["user"]["name"] == "Alice Johnson"
        assert "email" in response_data["data"]["user"]
        assert response_data["data"]["user"]["email"] == "alice@example.com"
        assert "profile" in response_data["data"]["user"]
        assert "bio" in response_data["data"]["user"]["profile"]
        assert response_data["data"]["user"]["profile"]["bio"] == "Software engineer and open source enthusiast"
        assert "location" in response_data["data"]["user"]["profile"]
        assert response_data["data"]["user"]["profile"]["location"] == "San Francisco, CA"
        response_data = response.json()
        assert response_data.get("errors") is None or response_data.get("errors") == []


@pytest.mark.asyncio
async def test_graphql_simple_field() -> None:
    """Basic single-field query with scalar return type."""

    app_factory = getattr(app_main, "create_app_graphql_simple_field", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_simple_field")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="query {\n  hello\n}", variables=None, operation_name=None, path="/graphql"
        )

        assert response.status_code == 200
        response_data = response.json()
        assert "data" in response_data
        assert "hello" in response_data["data"]
        assert response_data["data"]["hello"] == "Hello, World!"
        response_data = response.json()
        assert response_data.get("errors") is None or response_data.get("errors") == []


@pytest.mark.asyncio
async def test_graphql_introspection_disabled() -> None:
    """Introspection query rejected when introspection is disabled in production."""

    app_factory = getattr(app_main, "create_app_graphql_introspection_disabled", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_introspection_disabled")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(query="", variables=None, operation_name=None, path="/graphql")

        assert response.status_code == 400
        response_data = response.json()
        assert "errors" in response_data
        assert len(response_data["errors"]) == 1
        error_0 = response_data["errors"][0]
        assert error_0["message"] == "Introspection is disabled"


@pytest.mark.asyncio
async def test_graphql_full_schema_introspection() -> None:
    """Complete __schema introspection query returning full schema metadata."""

    app_factory = getattr(app_main, "create_app_graphql_full_schema_introspection", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_full_schema_introspection")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(query="", variables=None, operation_name=None, path="/graphql")

        assert response.status_code == 200
        response_data = response.json()
        assert "data" in response_data
        assert "__schema" in response_data["data"]
        assert "queryType" in response_data["data"]["__schema"]
        assert "name" in response_data["data"]["__schema"]["queryType"]
        assert response_data["data"]["__schema"]["queryType"]["name"] == "Query"
        assert "mutationType" in response_data["data"]["__schema"]
        assert "name" in response_data["data"]["__schema"]["mutationType"]
        assert response_data["data"]["__schema"]["mutationType"]["name"] == "Mutation"
        assert "subscriptionType" in response_data["data"]["__schema"]
        assert response_data["data"]["__schema"]["subscriptionType"] == None
        assert "types" in response_data["data"]["__schema"]
        assert len(response_data["data"]["__schema"]["types"]) == 7
        assert "kind" in response_data["data"]["__schema"]["types"][0]
        assert response_data["data"]["__schema"]["types"][0]["kind"] == "SCALAR"
        assert "name" in response_data["data"]["__schema"]["types"][0]
        assert response_data["data"]["__schema"]["types"][0]["name"] == "DateTime"
        assert "description" in response_data["data"]["__schema"]["types"][0]
        assert response_data["data"]["__schema"]["types"][0]["description"] == "ISO 8601 DateTime scalar"
        assert "fields" in response_data["data"]["__schema"]["types"][0]
        assert response_data["data"]["__schema"]["types"][0]["fields"] == None
        assert "inputFields" in response_data["data"]["__schema"]["types"][0]
        assert response_data["data"]["__schema"]["types"][0]["inputFields"] == None
        assert "interfaces" in response_data["data"]["__schema"]["types"][0]
        assert response_data["data"]["__schema"]["types"][0]["interfaces"] == None
        assert "enumValues" in response_data["data"]["__schema"]["types"][0]
        assert response_data["data"]["__schema"]["types"][0]["enumValues"] == None
        assert "possibleTypes" in response_data["data"]["__schema"]["types"][0]
        assert response_data["data"]["__schema"]["types"][0]["possibleTypes"] == None
        assert "kind" in response_data["data"]["__schema"]["types"][1]
        assert response_data["data"]["__schema"]["types"][1]["kind"] == "SCALAR"
        assert "name" in response_data["data"]["__schema"]["types"][1]
        assert response_data["data"]["__schema"]["types"][1]["name"] == "UUID"
        assert "description" in response_data["data"]["__schema"]["types"][1]
        assert response_data["data"]["__schema"]["types"][1]["description"] == "UUID scalar type"
        assert "fields" in response_data["data"]["__schema"]["types"][1]
        assert response_data["data"]["__schema"]["types"][1]["fields"] == None
        assert "inputFields" in response_data["data"]["__schema"]["types"][1]
        assert response_data["data"]["__schema"]["types"][1]["inputFields"] == None
        assert "interfaces" in response_data["data"]["__schema"]["types"][1]
        assert response_data["data"]["__schema"]["types"][1]["interfaces"] == None
        assert "enumValues" in response_data["data"]["__schema"]["types"][1]
        assert response_data["data"]["__schema"]["types"][1]["enumValues"] == None
        assert "possibleTypes" in response_data["data"]["__schema"]["types"][1]
        assert response_data["data"]["__schema"]["types"][1]["possibleTypes"] == None
        assert "kind" in response_data["data"]["__schema"]["types"][2]
        assert response_data["data"]["__schema"]["types"][2]["kind"] == "OBJECT"
        assert "name" in response_data["data"]["__schema"]["types"][2]
        assert response_data["data"]["__schema"]["types"][2]["name"] == "Query"
        assert "description" in response_data["data"]["__schema"]["types"][2]
        assert response_data["data"]["__schema"]["types"][2]["description"] == "Root query type"
        assert "fields" in response_data["data"]["__schema"]["types"][2]
        assert len(response_data["data"]["__schema"]["types"][2]["fields"]) == 4
        assert "name" in response_data["data"]["__schema"]["types"][2]["fields"][0]
        assert response_data["data"]["__schema"]["types"][2]["fields"][0]["name"] == "hello"
        assert "description" in response_data["data"]["__schema"]["types"][2]["fields"][0]
        assert response_data["data"]["__schema"]["types"][2]["fields"][0]["description"] == "Greeting message"
        assert "args" in response_data["data"]["__schema"]["types"][2]["fields"][0]
        assert len(response_data["data"]["__schema"]["types"][2]["fields"][0]["args"]) == 0
        assert "type" in response_data["data"]["__schema"]["types"][2]["fields"][0]
        assert "kind" in response_data["data"]["__schema"]["types"][2]["fields"][0]["type"]
        assert response_data["data"]["__schema"]["types"][2]["fields"][0]["type"]["kind"] == "NON_NULL"
        assert "name" in response_data["data"]["__schema"]["types"][2]["fields"][0]["type"]
        assert response_data["data"]["__schema"]["types"][2]["fields"][0]["type"]["name"] == None
        assert "ofType" in response_data["data"]["__schema"]["types"][2]["fields"][0]["type"]
        assert "kind" in response_data["data"]["__schema"]["types"][2]["fields"][0]["type"]["ofType"]
        assert response_data["data"]["__schema"]["types"][2]["fields"][0]["type"]["ofType"]["kind"] == "SCALAR"
        assert "name" in response_data["data"]["__schema"]["types"][2]["fields"][0]["type"]["ofType"]
        assert response_data["data"]["__schema"]["types"][2]["fields"][0]["type"]["ofType"]["name"] == "String"
        assert "isDeprecated" in response_data["data"]["__schema"]["types"][2]["fields"][0]
        assert response_data["data"]["__schema"]["types"][2]["fields"][0]["isDeprecated"] == False
        assert "deprecationReason" in response_data["data"]["__schema"]["types"][2]["fields"][0]
        assert response_data["data"]["__schema"]["types"][2]["fields"][0]["deprecationReason"] == None
        assert "name" in response_data["data"]["__schema"]["types"][2]["fields"][1]
        assert response_data["data"]["__schema"]["types"][2]["fields"][1]["name"] == "version"
        assert "description" in response_data["data"]["__schema"]["types"][2]["fields"][1]
        assert response_data["data"]["__schema"]["types"][2]["fields"][1]["description"] == "API version"
        assert "args" in response_data["data"]["__schema"]["types"][2]["fields"][1]
        assert len(response_data["data"]["__schema"]["types"][2]["fields"][1]["args"]) == 0
        assert "type" in response_data["data"]["__schema"]["types"][2]["fields"][1]
        assert "kind" in response_data["data"]["__schema"]["types"][2]["fields"][1]["type"]
        assert response_data["data"]["__schema"]["types"][2]["fields"][1]["type"]["kind"] == "NON_NULL"
        assert "name" in response_data["data"]["__schema"]["types"][2]["fields"][1]["type"]
        assert response_data["data"]["__schema"]["types"][2]["fields"][1]["type"]["name"] == None
        assert "ofType" in response_data["data"]["__schema"]["types"][2]["fields"][1]["type"]
        assert "kind" in response_data["data"]["__schema"]["types"][2]["fields"][1]["type"]["ofType"]
        assert response_data["data"]["__schema"]["types"][2]["fields"][1]["type"]["ofType"]["kind"] == "SCALAR"
        assert "name" in response_data["data"]["__schema"]["types"][2]["fields"][1]["type"]["ofType"]
        assert response_data["data"]["__schema"]["types"][2]["fields"][1]["type"]["ofType"]["name"] == "String"
        assert "isDeprecated" in response_data["data"]["__schema"]["types"][2]["fields"][1]
        assert response_data["data"]["__schema"]["types"][2]["fields"][1]["isDeprecated"] == False
        assert "deprecationReason" in response_data["data"]["__schema"]["types"][2]["fields"][1]
        assert response_data["data"]["__schema"]["types"][2]["fields"][1]["deprecationReason"] == None
        assert "name" in response_data["data"]["__schema"]["types"][2]["fields"][2]
        assert response_data["data"]["__schema"]["types"][2]["fields"][2]["name"] == "user"
        assert "description" in response_data["data"]["__schema"]["types"][2]["fields"][2]
        assert response_data["data"]["__schema"]["types"][2]["fields"][2]["description"] == "Get user by ID"
        assert "args" in response_data["data"]["__schema"]["types"][2]["fields"][2]
        assert len(response_data["data"]["__schema"]["types"][2]["fields"][2]["args"]) == 1
        assert "name" in response_data["data"]["__schema"]["types"][2]["fields"][2]["args"][0]
        assert response_data["data"]["__schema"]["types"][2]["fields"][2]["args"][0]["name"] == "id"
        assert "description" in response_data["data"]["__schema"]["types"][2]["fields"][2]["args"][0]
        assert response_data["data"]["__schema"]["types"][2]["fields"][2]["args"][0]["description"] == "User ID"
        assert "type" in response_data["data"]["__schema"]["types"][2]["fields"][2]["args"][0]
        assert "kind" in response_data["data"]["__schema"]["types"][2]["fields"][2]["args"][0]["type"]
        assert response_data["data"]["__schema"]["types"][2]["fields"][2]["args"][0]["type"]["kind"] == "NON_NULL"
        assert "name" in response_data["data"]["__schema"]["types"][2]["fields"][2]["args"][0]["type"]
        assert response_data["data"]["__schema"]["types"][2]["fields"][2]["args"][0]["type"]["name"] == None
        assert "ofType" in response_data["data"]["__schema"]["types"][2]["fields"][2]["args"][0]["type"]
        assert "kind" in response_data["data"]["__schema"]["types"][2]["fields"][2]["args"][0]["type"]["ofType"]
        assert (
            response_data["data"]["__schema"]["types"][2]["fields"][2]["args"][0]["type"]["ofType"]["kind"] == "SCALAR"
        )
        assert "name" in response_data["data"]["__schema"]["types"][2]["fields"][2]["args"][0]["type"]["ofType"]
        assert response_data["data"]["__schema"]["types"][2]["fields"][2]["args"][0]["type"]["ofType"]["name"] == "UUID"
        assert "defaultValue" in response_data["data"]["__schema"]["types"][2]["fields"][2]["args"][0]
        assert response_data["data"]["__schema"]["types"][2]["fields"][2]["args"][0]["defaultValue"] == None
        assert "type" in response_data["data"]["__schema"]["types"][2]["fields"][2]
        assert "kind" in response_data["data"]["__schema"]["types"][2]["fields"][2]["type"]
        assert response_data["data"]["__schema"]["types"][2]["fields"][2]["type"]["kind"] == "OBJECT"
        assert "name" in response_data["data"]["__schema"]["types"][2]["fields"][2]["type"]
        assert response_data["data"]["__schema"]["types"][2]["fields"][2]["type"]["name"] == "User"
        assert "isDeprecated" in response_data["data"]["__schema"]["types"][2]["fields"][2]
        assert response_data["data"]["__schema"]["types"][2]["fields"][2]["isDeprecated"] == False
        assert "deprecationReason" in response_data["data"]["__schema"]["types"][2]["fields"][2]
        assert response_data["data"]["__schema"]["types"][2]["fields"][2]["deprecationReason"] == None
        assert "name" in response_data["data"]["__schema"]["types"][2]["fields"][3]
        assert response_data["data"]["__schema"]["types"][2]["fields"][3]["name"] == "users"
        assert "description" in response_data["data"]["__schema"]["types"][2]["fields"][3]
        assert (
            response_data["data"]["__schema"]["types"][2]["fields"][3]["description"] == "Get all users with pagination"
        )
        assert "args" in response_data["data"]["__schema"]["types"][2]["fields"][3]
        assert len(response_data["data"]["__schema"]["types"][2]["fields"][3]["args"]) == 2
        assert "name" in response_data["data"]["__schema"]["types"][2]["fields"][3]["args"][0]
        assert response_data["data"]["__schema"]["types"][2]["fields"][3]["args"][0]["name"] == "limit"
        assert "description" in response_data["data"]["__schema"]["types"][2]["fields"][3]["args"][0]
        assert (
            response_data["data"]["__schema"]["types"][2]["fields"][3]["args"][0]["description"]
            == "Maximum number of results"
        )
        assert "type" in response_data["data"]["__schema"]["types"][2]["fields"][3]["args"][0]
        assert "kind" in response_data["data"]["__schema"]["types"][2]["fields"][3]["args"][0]["type"]
        assert response_data["data"]["__schema"]["types"][2]["fields"][3]["args"][0]["type"]["kind"] == "SCALAR"
        assert "name" in response_data["data"]["__schema"]["types"][2]["fields"][3]["args"][0]["type"]
        assert response_data["data"]["__schema"]["types"][2]["fields"][3]["args"][0]["type"]["name"] == "Int"
        assert "defaultValue" in response_data["data"]["__schema"]["types"][2]["fields"][3]["args"][0]
        assert response_data["data"]["__schema"]["types"][2]["fields"][3]["args"][0]["defaultValue"] == "10"
        assert "name" in response_data["data"]["__schema"]["types"][2]["fields"][3]["args"][1]
        assert response_data["data"]["__schema"]["types"][2]["fields"][3]["args"][1]["name"] == "offset"
        assert "description" in response_data["data"]["__schema"]["types"][2]["fields"][3]["args"][1]
        assert (
            response_data["data"]["__schema"]["types"][2]["fields"][3]["args"][1]["description"]
            == "Number of results to skip"
        )
        assert "type" in response_data["data"]["__schema"]["types"][2]["fields"][3]["args"][1]
        assert "kind" in response_data["data"]["__schema"]["types"][2]["fields"][3]["args"][1]["type"]
        assert response_data["data"]["__schema"]["types"][2]["fields"][3]["args"][1]["type"]["kind"] == "SCALAR"
        assert "name" in response_data["data"]["__schema"]["types"][2]["fields"][3]["args"][1]["type"]
        assert response_data["data"]["__schema"]["types"][2]["fields"][3]["args"][1]["type"]["name"] == "Int"
        assert "defaultValue" in response_data["data"]["__schema"]["types"][2]["fields"][3]["args"][1]
        assert response_data["data"]["__schema"]["types"][2]["fields"][3]["args"][1]["defaultValue"] == "0"
        assert "type" in response_data["data"]["__schema"]["types"][2]["fields"][3]
        assert "kind" in response_data["data"]["__schema"]["types"][2]["fields"][3]["type"]
        assert response_data["data"]["__schema"]["types"][2]["fields"][3]["type"]["kind"] == "NON_NULL"
        assert "name" in response_data["data"]["__schema"]["types"][2]["fields"][3]["type"]
        assert response_data["data"]["__schema"]["types"][2]["fields"][3]["type"]["name"] == None
        assert "ofType" in response_data["data"]["__schema"]["types"][2]["fields"][3]["type"]
        assert "kind" in response_data["data"]["__schema"]["types"][2]["fields"][3]["type"]["ofType"]
        assert response_data["data"]["__schema"]["types"][2]["fields"][3]["type"]["ofType"]["kind"] == "LIST"
        assert "name" in response_data["data"]["__schema"]["types"][2]["fields"][3]["type"]["ofType"]
        assert response_data["data"]["__schema"]["types"][2]["fields"][3]["type"]["ofType"]["name"] == None
        assert "ofType" in response_data["data"]["__schema"]["types"][2]["fields"][3]["type"]["ofType"]
        assert "kind" in response_data["data"]["__schema"]["types"][2]["fields"][3]["type"]["ofType"]["ofType"]
        assert (
            response_data["data"]["__schema"]["types"][2]["fields"][3]["type"]["ofType"]["ofType"]["kind"] == "NON_NULL"
        )
        assert "name" in response_data["data"]["__schema"]["types"][2]["fields"][3]["type"]["ofType"]["ofType"]
        assert response_data["data"]["__schema"]["types"][2]["fields"][3]["type"]["ofType"]["ofType"]["name"] == None
        assert "ofType" in response_data["data"]["__schema"]["types"][2]["fields"][3]["type"]["ofType"]["ofType"]
        assert (
            "kind" in response_data["data"]["__schema"]["types"][2]["fields"][3]["type"]["ofType"]["ofType"]["ofType"]
        )
        assert (
            response_data["data"]["__schema"]["types"][2]["fields"][3]["type"]["ofType"]["ofType"]["ofType"]["kind"]
            == "OBJECT"
        )
        assert (
            "name" in response_data["data"]["__schema"]["types"][2]["fields"][3]["type"]["ofType"]["ofType"]["ofType"]
        )
        assert (
            response_data["data"]["__schema"]["types"][2]["fields"][3]["type"]["ofType"]["ofType"]["ofType"]["name"]
            == "User"
        )
        assert "isDeprecated" in response_data["data"]["__schema"]["types"][2]["fields"][3]
        assert response_data["data"]["__schema"]["types"][2]["fields"][3]["isDeprecated"] == False
        assert "deprecationReason" in response_data["data"]["__schema"]["types"][2]["fields"][3]
        assert response_data["data"]["__schema"]["types"][2]["fields"][3]["deprecationReason"] == None
        assert "inputFields" in response_data["data"]["__schema"]["types"][2]
        assert response_data["data"]["__schema"]["types"][2]["inputFields"] == None
        assert "interfaces" in response_data["data"]["__schema"]["types"][2]
        assert len(response_data["data"]["__schema"]["types"][2]["interfaces"]) == 0
        assert "enumValues" in response_data["data"]["__schema"]["types"][2]
        assert response_data["data"]["__schema"]["types"][2]["enumValues"] == None
        assert "possibleTypes" in response_data["data"]["__schema"]["types"][2]
        assert response_data["data"]["__schema"]["types"][2]["possibleTypes"] == None
        assert "kind" in response_data["data"]["__schema"]["types"][3]
        assert response_data["data"]["__schema"]["types"][3]["kind"] == "OBJECT"
        assert "name" in response_data["data"]["__schema"]["types"][3]
        assert response_data["data"]["__schema"]["types"][3]["name"] == "Mutation"
        assert "description" in response_data["data"]["__schema"]["types"][3]
        assert response_data["data"]["__schema"]["types"][3]["description"] == "Root mutation type"
        assert "fields" in response_data["data"]["__schema"]["types"][3]
        assert len(response_data["data"]["__schema"]["types"][3]["fields"]) == 3
        assert "name" in response_data["data"]["__schema"]["types"][3]["fields"][0]
        assert response_data["data"]["__schema"]["types"][3]["fields"][0]["name"] == "createPost"
        assert "description" in response_data["data"]["__schema"]["types"][3]["fields"][0]
        assert response_data["data"]["__schema"]["types"][3]["fields"][0]["description"] == "Create a new post"
        assert "args" in response_data["data"]["__schema"]["types"][3]["fields"][0]
        assert len(response_data["data"]["__schema"]["types"][3]["fields"][0]["args"]) == 1
        assert "name" in response_data["data"]["__schema"]["types"][3]["fields"][0]["args"][0]
        assert response_data["data"]["__schema"]["types"][3]["fields"][0]["args"][0]["name"] == "input"
        assert "description" in response_data["data"]["__schema"]["types"][3]["fields"][0]["args"][0]
        assert (
            response_data["data"]["__schema"]["types"][3]["fields"][0]["args"][0]["description"]
            == "Post creation input"
        )
        assert "type" in response_data["data"]["__schema"]["types"][3]["fields"][0]["args"][0]
        assert "kind" in response_data["data"]["__schema"]["types"][3]["fields"][0]["args"][0]["type"]
        assert response_data["data"]["__schema"]["types"][3]["fields"][0]["args"][0]["type"]["kind"] == "NON_NULL"
        assert "name" in response_data["data"]["__schema"]["types"][3]["fields"][0]["args"][0]["type"]
        assert response_data["data"]["__schema"]["types"][3]["fields"][0]["args"][0]["type"]["name"] == None
        assert "ofType" in response_data["data"]["__schema"]["types"][3]["fields"][0]["args"][0]["type"]
        assert "kind" in response_data["data"]["__schema"]["types"][3]["fields"][0]["args"][0]["type"]["ofType"]
        assert (
            response_data["data"]["__schema"]["types"][3]["fields"][0]["args"][0]["type"]["ofType"]["kind"]
            == "INPUT_OBJECT"
        )
        assert "name" in response_data["data"]["__schema"]["types"][3]["fields"][0]["args"][0]["type"]["ofType"]
        assert (
            response_data["data"]["__schema"]["types"][3]["fields"][0]["args"][0]["type"]["ofType"]["name"]
            == "CreatePostInput"
        )
        assert "defaultValue" in response_data["data"]["__schema"]["types"][3]["fields"][0]["args"][0]
        assert response_data["data"]["__schema"]["types"][3]["fields"][0]["args"][0]["defaultValue"] == None
        assert "type" in response_data["data"]["__schema"]["types"][3]["fields"][0]
        assert "kind" in response_data["data"]["__schema"]["types"][3]["fields"][0]["type"]
        assert response_data["data"]["__schema"]["types"][3]["fields"][0]["type"]["kind"] == "NON_NULL"
        assert "name" in response_data["data"]["__schema"]["types"][3]["fields"][0]["type"]
        assert response_data["data"]["__schema"]["types"][3]["fields"][0]["type"]["name"] == None
        assert "ofType" in response_data["data"]["__schema"]["types"][3]["fields"][0]["type"]
        assert "kind" in response_data["data"]["__schema"]["types"][3]["fields"][0]["type"]["ofType"]
        assert response_data["data"]["__schema"]["types"][3]["fields"][0]["type"]["ofType"]["kind"] == "OBJECT"
        assert "name" in response_data["data"]["__schema"]["types"][3]["fields"][0]["type"]["ofType"]
        assert response_data["data"]["__schema"]["types"][3]["fields"][0]["type"]["ofType"]["name"] == "Post"
        assert "isDeprecated" in response_data["data"]["__schema"]["types"][3]["fields"][0]
        assert response_data["data"]["__schema"]["types"][3]["fields"][0]["isDeprecated"] == False
        assert "deprecationReason" in response_data["data"]["__schema"]["types"][3]["fields"][0]
        assert response_data["data"]["__schema"]["types"][3]["fields"][0]["deprecationReason"] == None
        assert "name" in response_data["data"]["__schema"]["types"][3]["fields"][1]
        assert response_data["data"]["__schema"]["types"][3]["fields"][1]["name"] == "updateUser"
        assert "description" in response_data["data"]["__schema"]["types"][3]["fields"][1]
        assert response_data["data"]["__schema"]["types"][3]["fields"][1]["description"] == "Update user information"
        assert "args" in response_data["data"]["__schema"]["types"][3]["fields"][1]
        assert len(response_data["data"]["__schema"]["types"][3]["fields"][1]["args"]) == 2
        assert "name" in response_data["data"]["__schema"]["types"][3]["fields"][1]["args"][0]
        assert response_data["data"]["__schema"]["types"][3]["fields"][1]["args"][0]["name"] == "id"
        assert "description" in response_data["data"]["__schema"]["types"][3]["fields"][1]["args"][0]
        assert response_data["data"]["__schema"]["types"][3]["fields"][1]["args"][0]["description"] == "User ID"
        assert "type" in response_data["data"]["__schema"]["types"][3]["fields"][1]["args"][0]
        assert "kind" in response_data["data"]["__schema"]["types"][3]["fields"][1]["args"][0]["type"]
        assert response_data["data"]["__schema"]["types"][3]["fields"][1]["args"][0]["type"]["kind"] == "NON_NULL"
        assert "name" in response_data["data"]["__schema"]["types"][3]["fields"][1]["args"][0]["type"]
        assert response_data["data"]["__schema"]["types"][3]["fields"][1]["args"][0]["type"]["name"] == None
        assert "ofType" in response_data["data"]["__schema"]["types"][3]["fields"][1]["args"][0]["type"]
        assert "kind" in response_data["data"]["__schema"]["types"][3]["fields"][1]["args"][0]["type"]["ofType"]
        assert (
            response_data["data"]["__schema"]["types"][3]["fields"][1]["args"][0]["type"]["ofType"]["kind"] == "SCALAR"
        )
        assert "name" in response_data["data"]["__schema"]["types"][3]["fields"][1]["args"][0]["type"]["ofType"]
        assert response_data["data"]["__schema"]["types"][3]["fields"][1]["args"][0]["type"]["ofType"]["name"] == "UUID"
        assert "defaultValue" in response_data["data"]["__schema"]["types"][3]["fields"][1]["args"][0]
        assert response_data["data"]["__schema"]["types"][3]["fields"][1]["args"][0]["defaultValue"] == None
        assert "name" in response_data["data"]["__schema"]["types"][3]["fields"][1]["args"][1]
        assert response_data["data"]["__schema"]["types"][3]["fields"][1]["args"][1]["name"] == "name"
        assert "description" in response_data["data"]["__schema"]["types"][3]["fields"][1]["args"][1]
        assert response_data["data"]["__schema"]["types"][3]["fields"][1]["args"][1]["description"] == "New user name"
        assert "type" in response_data["data"]["__schema"]["types"][3]["fields"][1]["args"][1]
        assert "kind" in response_data["data"]["__schema"]["types"][3]["fields"][1]["args"][1]["type"]
        assert response_data["data"]["__schema"]["types"][3]["fields"][1]["args"][1]["type"]["kind"] == "NON_NULL"
        assert "name" in response_data["data"]["__schema"]["types"][3]["fields"][1]["args"][1]["type"]
        assert response_data["data"]["__schema"]["types"][3]["fields"][1]["args"][1]["type"]["name"] == None
        assert "ofType" in response_data["data"]["__schema"]["types"][3]["fields"][1]["args"][1]["type"]
        assert "kind" in response_data["data"]["__schema"]["types"][3]["fields"][1]["args"][1]["type"]["ofType"]
        assert (
            response_data["data"]["__schema"]["types"][3]["fields"][1]["args"][1]["type"]["ofType"]["kind"] == "SCALAR"
        )
        assert "name" in response_data["data"]["__schema"]["types"][3]["fields"][1]["args"][1]["type"]["ofType"]
        assert (
            response_data["data"]["__schema"]["types"][3]["fields"][1]["args"][1]["type"]["ofType"]["name"] == "String"
        )
        assert "defaultValue" in response_data["data"]["__schema"]["types"][3]["fields"][1]["args"][1]
        assert response_data["data"]["__schema"]["types"][3]["fields"][1]["args"][1]["defaultValue"] == None
        assert "type" in response_data["data"]["__schema"]["types"][3]["fields"][1]
        assert "kind" in response_data["data"]["__schema"]["types"][3]["fields"][1]["type"]
        assert response_data["data"]["__schema"]["types"][3]["fields"][1]["type"]["kind"] == "NON_NULL"
        assert "name" in response_data["data"]["__schema"]["types"][3]["fields"][1]["type"]
        assert response_data["data"]["__schema"]["types"][3]["fields"][1]["type"]["name"] == None
        assert "ofType" in response_data["data"]["__schema"]["types"][3]["fields"][1]["type"]
        assert "kind" in response_data["data"]["__schema"]["types"][3]["fields"][1]["type"]["ofType"]
        assert response_data["data"]["__schema"]["types"][3]["fields"][1]["type"]["ofType"]["kind"] == "OBJECT"
        assert "name" in response_data["data"]["__schema"]["types"][3]["fields"][1]["type"]["ofType"]
        assert response_data["data"]["__schema"]["types"][3]["fields"][1]["type"]["ofType"]["name"] == "User"
        assert "isDeprecated" in response_data["data"]["__schema"]["types"][3]["fields"][1]
        assert response_data["data"]["__schema"]["types"][3]["fields"][1]["isDeprecated"] == False
        assert "deprecationReason" in response_data["data"]["__schema"]["types"][3]["fields"][1]
        assert response_data["data"]["__schema"]["types"][3]["fields"][1]["deprecationReason"] == None
        assert "name" in response_data["data"]["__schema"]["types"][3]["fields"][2]
        assert response_data["data"]["__schema"]["types"][3]["fields"][2]["name"] == "deletePost"
        assert "description" in response_data["data"]["__schema"]["types"][3]["fields"][2]
        assert response_data["data"]["__schema"]["types"][3]["fields"][2]["description"] == "Delete a post"
        assert "args" in response_data["data"]["__schema"]["types"][3]["fields"][2]
        assert len(response_data["data"]["__schema"]["types"][3]["fields"][2]["args"]) == 1
        assert "name" in response_data["data"]["__schema"]["types"][3]["fields"][2]["args"][0]
        assert response_data["data"]["__schema"]["types"][3]["fields"][2]["args"][0]["name"] == "id"
        assert "description" in response_data["data"]["__schema"]["types"][3]["fields"][2]["args"][0]
        assert response_data["data"]["__schema"]["types"][3]["fields"][2]["args"][0]["description"] == "Post ID"
        assert "type" in response_data["data"]["__schema"]["types"][3]["fields"][2]["args"][0]
        assert "kind" in response_data["data"]["__schema"]["types"][3]["fields"][2]["args"][0]["type"]
        assert response_data["data"]["__schema"]["types"][3]["fields"][2]["args"][0]["type"]["kind"] == "NON_NULL"
        assert "name" in response_data["data"]["__schema"]["types"][3]["fields"][2]["args"][0]["type"]
        assert response_data["data"]["__schema"]["types"][3]["fields"][2]["args"][0]["type"]["name"] == None
        assert "ofType" in response_data["data"]["__schema"]["types"][3]["fields"][2]["args"][0]["type"]
        assert "kind" in response_data["data"]["__schema"]["types"][3]["fields"][2]["args"][0]["type"]["ofType"]
        assert (
            response_data["data"]["__schema"]["types"][3]["fields"][2]["args"][0]["type"]["ofType"]["kind"] == "SCALAR"
        )
        assert "name" in response_data["data"]["__schema"]["types"][3]["fields"][2]["args"][0]["type"]["ofType"]
        assert response_data["data"]["__schema"]["types"][3]["fields"][2]["args"][0]["type"]["ofType"]["name"] == "UUID"
        assert "defaultValue" in response_data["data"]["__schema"]["types"][3]["fields"][2]["args"][0]
        assert response_data["data"]["__schema"]["types"][3]["fields"][2]["args"][0]["defaultValue"] == None
        assert "type" in response_data["data"]["__schema"]["types"][3]["fields"][2]
        assert "kind" in response_data["data"]["__schema"]["types"][3]["fields"][2]["type"]
        assert response_data["data"]["__schema"]["types"][3]["fields"][2]["type"]["kind"] == "NON_NULL"
        assert "name" in response_data["data"]["__schema"]["types"][3]["fields"][2]["type"]
        assert response_data["data"]["__schema"]["types"][3]["fields"][2]["type"]["name"] == None
        assert "ofType" in response_data["data"]["__schema"]["types"][3]["fields"][2]["type"]
        assert "kind" in response_data["data"]["__schema"]["types"][3]["fields"][2]["type"]["ofType"]
        assert response_data["data"]["__schema"]["types"][3]["fields"][2]["type"]["ofType"]["kind"] == "SCALAR"
        assert "name" in response_data["data"]["__schema"]["types"][3]["fields"][2]["type"]["ofType"]
        assert response_data["data"]["__schema"]["types"][3]["fields"][2]["type"]["ofType"]["name"] == "Boolean"
        assert "isDeprecated" in response_data["data"]["__schema"]["types"][3]["fields"][2]
        assert response_data["data"]["__schema"]["types"][3]["fields"][2]["isDeprecated"] == False
        assert "deprecationReason" in response_data["data"]["__schema"]["types"][3]["fields"][2]
        assert response_data["data"]["__schema"]["types"][3]["fields"][2]["deprecationReason"] == None
        assert "inputFields" in response_data["data"]["__schema"]["types"][3]
        assert response_data["data"]["__schema"]["types"][3]["inputFields"] == None
        assert "interfaces" in response_data["data"]["__schema"]["types"][3]
        assert len(response_data["data"]["__schema"]["types"][3]["interfaces"]) == 0
        assert "enumValues" in response_data["data"]["__schema"]["types"][3]
        assert response_data["data"]["__schema"]["types"][3]["enumValues"] == None
        assert "possibleTypes" in response_data["data"]["__schema"]["types"][3]
        assert response_data["data"]["__schema"]["types"][3]["possibleTypes"] == None
        assert "kind" in response_data["data"]["__schema"]["types"][4]
        assert response_data["data"]["__schema"]["types"][4]["kind"] == "OBJECT"
        assert "name" in response_data["data"]["__schema"]["types"][4]
        assert response_data["data"]["__schema"]["types"][4]["name"] == "User"
        assert "description" in response_data["data"]["__schema"]["types"][4]
        assert response_data["data"]["__schema"]["types"][4]["description"] == "User entity"
        assert "fields" in response_data["data"]["__schema"]["types"][4]
        assert len(response_data["data"]["__schema"]["types"][4]["fields"]) == 5
        assert "name" in response_data["data"]["__schema"]["types"][4]["fields"][0]
        assert response_data["data"]["__schema"]["types"][4]["fields"][0]["name"] == "id"
        assert "description" in response_data["data"]["__schema"]["types"][4]["fields"][0]
        assert response_data["data"]["__schema"]["types"][4]["fields"][0]["description"] == "Unique identifier"
        assert "args" in response_data["data"]["__schema"]["types"][4]["fields"][0]
        assert len(response_data["data"]["__schema"]["types"][4]["fields"][0]["args"]) == 0
        assert "type" in response_data["data"]["__schema"]["types"][4]["fields"][0]
        assert "kind" in response_data["data"]["__schema"]["types"][4]["fields"][0]["type"]
        assert response_data["data"]["__schema"]["types"][4]["fields"][0]["type"]["kind"] == "NON_NULL"
        assert "name" in response_data["data"]["__schema"]["types"][4]["fields"][0]["type"]
        assert response_data["data"]["__schema"]["types"][4]["fields"][0]["type"]["name"] == None
        assert "ofType" in response_data["data"]["__schema"]["types"][4]["fields"][0]["type"]
        assert "kind" in response_data["data"]["__schema"]["types"][4]["fields"][0]["type"]["ofType"]
        assert response_data["data"]["__schema"]["types"][4]["fields"][0]["type"]["ofType"]["kind"] == "SCALAR"
        assert "name" in response_data["data"]["__schema"]["types"][4]["fields"][0]["type"]["ofType"]
        assert response_data["data"]["__schema"]["types"][4]["fields"][0]["type"]["ofType"]["name"] == "UUID"
        assert "isDeprecated" in response_data["data"]["__schema"]["types"][4]["fields"][0]
        assert response_data["data"]["__schema"]["types"][4]["fields"][0]["isDeprecated"] == False
        assert "deprecationReason" in response_data["data"]["__schema"]["types"][4]["fields"][0]
        assert response_data["data"]["__schema"]["types"][4]["fields"][0]["deprecationReason"] == None
        assert "name" in response_data["data"]["__schema"]["types"][4]["fields"][1]
        assert response_data["data"]["__schema"]["types"][4]["fields"][1]["name"] == "name"
        assert "description" in response_data["data"]["__schema"]["types"][4]["fields"][1]
        assert response_data["data"]["__schema"]["types"][4]["fields"][1]["description"] == "User's full name"
        assert "args" in response_data["data"]["__schema"]["types"][4]["fields"][1]
        assert len(response_data["data"]["__schema"]["types"][4]["fields"][1]["args"]) == 0
        assert "type" in response_data["data"]["__schema"]["types"][4]["fields"][1]
        assert "kind" in response_data["data"]["__schema"]["types"][4]["fields"][1]["type"]
        assert response_data["data"]["__schema"]["types"][4]["fields"][1]["type"]["kind"] == "NON_NULL"
        assert "name" in response_data["data"]["__schema"]["types"][4]["fields"][1]["type"]
        assert response_data["data"]["__schema"]["types"][4]["fields"][1]["type"]["name"] == None
        assert "ofType" in response_data["data"]["__schema"]["types"][4]["fields"][1]["type"]
        assert "kind" in response_data["data"]["__schema"]["types"][4]["fields"][1]["type"]["ofType"]
        assert response_data["data"]["__schema"]["types"][4]["fields"][1]["type"]["ofType"]["kind"] == "SCALAR"
        assert "name" in response_data["data"]["__schema"]["types"][4]["fields"][1]["type"]["ofType"]
        assert response_data["data"]["__schema"]["types"][4]["fields"][1]["type"]["ofType"]["name"] == "String"
        assert "isDeprecated" in response_data["data"]["__schema"]["types"][4]["fields"][1]
        assert response_data["data"]["__schema"]["types"][4]["fields"][1]["isDeprecated"] == False
        assert "deprecationReason" in response_data["data"]["__schema"]["types"][4]["fields"][1]
        assert response_data["data"]["__schema"]["types"][4]["fields"][1]["deprecationReason"] == None
        assert "name" in response_data["data"]["__schema"]["types"][4]["fields"][2]
        assert response_data["data"]["__schema"]["types"][4]["fields"][2]["name"] == "email"
        assert "description" in response_data["data"]["__schema"]["types"][4]["fields"][2]
        assert response_data["data"]["__schema"]["types"][4]["fields"][2]["description"] == "User's email address"
        assert "args" in response_data["data"]["__schema"]["types"][4]["fields"][2]
        assert len(response_data["data"]["__schema"]["types"][4]["fields"][2]["args"]) == 0
        assert "type" in response_data["data"]["__schema"]["types"][4]["fields"][2]
        assert "kind" in response_data["data"]["__schema"]["types"][4]["fields"][2]["type"]
        assert response_data["data"]["__schema"]["types"][4]["fields"][2]["type"]["kind"] == "NON_NULL"
        assert "name" in response_data["data"]["__schema"]["types"][4]["fields"][2]["type"]
        assert response_data["data"]["__schema"]["types"][4]["fields"][2]["type"]["name"] == None
        assert "ofType" in response_data["data"]["__schema"]["types"][4]["fields"][2]["type"]
        assert "kind" in response_data["data"]["__schema"]["types"][4]["fields"][2]["type"]["ofType"]
        assert response_data["data"]["__schema"]["types"][4]["fields"][2]["type"]["ofType"]["kind"] == "SCALAR"
        assert "name" in response_data["data"]["__schema"]["types"][4]["fields"][2]["type"]["ofType"]
        assert response_data["data"]["__schema"]["types"][4]["fields"][2]["type"]["ofType"]["name"] == "String"
        assert "isDeprecated" in response_data["data"]["__schema"]["types"][4]["fields"][2]
        assert response_data["data"]["__schema"]["types"][4]["fields"][2]["isDeprecated"] == False
        assert "deprecationReason" in response_data["data"]["__schema"]["types"][4]["fields"][2]
        assert response_data["data"]["__schema"]["types"][4]["fields"][2]["deprecationReason"] == None
        assert "name" in response_data["data"]["__schema"]["types"][4]["fields"][3]
        assert response_data["data"]["__schema"]["types"][4]["fields"][3]["name"] == "createdAt"
        assert "description" in response_data["data"]["__schema"]["types"][4]["fields"][3]
        assert response_data["data"]["__schema"]["types"][4]["fields"][3]["description"] == "Creation timestamp"
        assert "args" in response_data["data"]["__schema"]["types"][4]["fields"][3]
        assert len(response_data["data"]["__schema"]["types"][4]["fields"][3]["args"]) == 0
        assert "type" in response_data["data"]["__schema"]["types"][4]["fields"][3]
        assert "kind" in response_data["data"]["__schema"]["types"][4]["fields"][3]["type"]
        assert response_data["data"]["__schema"]["types"][4]["fields"][3]["type"]["kind"] == "NON_NULL"
        assert "name" in response_data["data"]["__schema"]["types"][4]["fields"][3]["type"]
        assert response_data["data"]["__schema"]["types"][4]["fields"][3]["type"]["name"] == None
        assert "ofType" in response_data["data"]["__schema"]["types"][4]["fields"][3]["type"]
        assert "kind" in response_data["data"]["__schema"]["types"][4]["fields"][3]["type"]["ofType"]
        assert response_data["data"]["__schema"]["types"][4]["fields"][3]["type"]["ofType"]["kind"] == "SCALAR"
        assert "name" in response_data["data"]["__schema"]["types"][4]["fields"][3]["type"]["ofType"]
        assert response_data["data"]["__schema"]["types"][4]["fields"][3]["type"]["ofType"]["name"] == "DateTime"
        assert "isDeprecated" in response_data["data"]["__schema"]["types"][4]["fields"][3]
        assert response_data["data"]["__schema"]["types"][4]["fields"][3]["isDeprecated"] == False
        assert "deprecationReason" in response_data["data"]["__schema"]["types"][4]["fields"][3]
        assert response_data["data"]["__schema"]["types"][4]["fields"][3]["deprecationReason"] == None
        assert "name" in response_data["data"]["__schema"]["types"][4]["fields"][4]
        assert response_data["data"]["__schema"]["types"][4]["fields"][4]["name"] == "posts"
        assert "description" in response_data["data"]["__schema"]["types"][4]["fields"][4]
        assert response_data["data"]["__schema"]["types"][4]["fields"][4]["description"] == "User's posts"
        assert "args" in response_data["data"]["__schema"]["types"][4]["fields"][4]
        assert len(response_data["data"]["__schema"]["types"][4]["fields"][4]["args"]) == 0
        assert "type" in response_data["data"]["__schema"]["types"][4]["fields"][4]
        assert "kind" in response_data["data"]["__schema"]["types"][4]["fields"][4]["type"]
        assert response_data["data"]["__schema"]["types"][4]["fields"][4]["type"]["kind"] == "NON_NULL"
        assert "name" in response_data["data"]["__schema"]["types"][4]["fields"][4]["type"]
        assert response_data["data"]["__schema"]["types"][4]["fields"][4]["type"]["name"] == None
        assert "ofType" in response_data["data"]["__schema"]["types"][4]["fields"][4]["type"]
        assert "kind" in response_data["data"]["__schema"]["types"][4]["fields"][4]["type"]["ofType"]
        assert response_data["data"]["__schema"]["types"][4]["fields"][4]["type"]["ofType"]["kind"] == "LIST"
        assert "name" in response_data["data"]["__schema"]["types"][4]["fields"][4]["type"]["ofType"]
        assert response_data["data"]["__schema"]["types"][4]["fields"][4]["type"]["ofType"]["name"] == None
        assert "ofType" in response_data["data"]["__schema"]["types"][4]["fields"][4]["type"]["ofType"]
        assert "kind" in response_data["data"]["__schema"]["types"][4]["fields"][4]["type"]["ofType"]["ofType"]
        assert (
            response_data["data"]["__schema"]["types"][4]["fields"][4]["type"]["ofType"]["ofType"]["kind"] == "NON_NULL"
        )
        assert "name" in response_data["data"]["__schema"]["types"][4]["fields"][4]["type"]["ofType"]["ofType"]
        assert response_data["data"]["__schema"]["types"][4]["fields"][4]["type"]["ofType"]["ofType"]["name"] == None
        assert "ofType" in response_data["data"]["__schema"]["types"][4]["fields"][4]["type"]["ofType"]["ofType"]
        assert (
            "kind" in response_data["data"]["__schema"]["types"][4]["fields"][4]["type"]["ofType"]["ofType"]["ofType"]
        )
        assert (
            response_data["data"]["__schema"]["types"][4]["fields"][4]["type"]["ofType"]["ofType"]["ofType"]["kind"]
            == "OBJECT"
        )
        assert (
            "name" in response_data["data"]["__schema"]["types"][4]["fields"][4]["type"]["ofType"]["ofType"]["ofType"]
        )
        assert (
            response_data["data"]["__schema"]["types"][4]["fields"][4]["type"]["ofType"]["ofType"]["ofType"]["name"]
            == "Post"
        )
        assert "isDeprecated" in response_data["data"]["__schema"]["types"][4]["fields"][4]
        assert response_data["data"]["__schema"]["types"][4]["fields"][4]["isDeprecated"] == False
        assert "deprecationReason" in response_data["data"]["__schema"]["types"][4]["fields"][4]
        assert response_data["data"]["__schema"]["types"][4]["fields"][4]["deprecationReason"] == None
        assert "inputFields" in response_data["data"]["__schema"]["types"][4]
        assert response_data["data"]["__schema"]["types"][4]["inputFields"] == None
        assert "interfaces" in response_data["data"]["__schema"]["types"][4]
        assert len(response_data["data"]["__schema"]["types"][4]["interfaces"]) == 0
        assert "enumValues" in response_data["data"]["__schema"]["types"][4]
        assert response_data["data"]["__schema"]["types"][4]["enumValues"] == None
        assert "possibleTypes" in response_data["data"]["__schema"]["types"][4]
        assert response_data["data"]["__schema"]["types"][4]["possibleTypes"] == None
        assert "kind" in response_data["data"]["__schema"]["types"][5]
        assert response_data["data"]["__schema"]["types"][5]["kind"] == "OBJECT"
        assert "name" in response_data["data"]["__schema"]["types"][5]
        assert response_data["data"]["__schema"]["types"][5]["name"] == "Post"
        assert "description" in response_data["data"]["__schema"]["types"][5]
        assert response_data["data"]["__schema"]["types"][5]["description"] == "Blog post entity"
        assert "fields" in response_data["data"]["__schema"]["types"][5]
        assert len(response_data["data"]["__schema"]["types"][5]["fields"]) == 7
        assert "name" in response_data["data"]["__schema"]["types"][5]["fields"][0]
        assert response_data["data"]["__schema"]["types"][5]["fields"][0]["name"] == "id"
        assert "description" in response_data["data"]["__schema"]["types"][5]["fields"][0]
        assert response_data["data"]["__schema"]["types"][5]["fields"][0]["description"] == "Unique identifier"
        assert "args" in response_data["data"]["__schema"]["types"][5]["fields"][0]
        assert len(response_data["data"]["__schema"]["types"][5]["fields"][0]["args"]) == 0
        assert "type" in response_data["data"]["__schema"]["types"][5]["fields"][0]
        assert "kind" in response_data["data"]["__schema"]["types"][5]["fields"][0]["type"]
        assert response_data["data"]["__schema"]["types"][5]["fields"][0]["type"]["kind"] == "NON_NULL"
        assert "name" in response_data["data"]["__schema"]["types"][5]["fields"][0]["type"]
        assert response_data["data"]["__schema"]["types"][5]["fields"][0]["type"]["name"] == None
        assert "ofType" in response_data["data"]["__schema"]["types"][5]["fields"][0]["type"]
        assert "kind" in response_data["data"]["__schema"]["types"][5]["fields"][0]["type"]["ofType"]
        assert response_data["data"]["__schema"]["types"][5]["fields"][0]["type"]["ofType"]["kind"] == "SCALAR"
        assert "name" in response_data["data"]["__schema"]["types"][5]["fields"][0]["type"]["ofType"]
        assert response_data["data"]["__schema"]["types"][5]["fields"][0]["type"]["ofType"]["name"] == "UUID"
        assert "isDeprecated" in response_data["data"]["__schema"]["types"][5]["fields"][0]
        assert response_data["data"]["__schema"]["types"][5]["fields"][0]["isDeprecated"] == False
        assert "deprecationReason" in response_data["data"]["__schema"]["types"][5]["fields"][0]
        assert response_data["data"]["__schema"]["types"][5]["fields"][0]["deprecationReason"] == None
        assert "name" in response_data["data"]["__schema"]["types"][5]["fields"][1]
        assert response_data["data"]["__schema"]["types"][5]["fields"][1]["name"] == "title"
        assert "description" in response_data["data"]["__schema"]["types"][5]["fields"][1]
        assert response_data["data"]["__schema"]["types"][5]["fields"][1]["description"] == "Post title"
        assert "args" in response_data["data"]["__schema"]["types"][5]["fields"][1]
        assert len(response_data["data"]["__schema"]["types"][5]["fields"][1]["args"]) == 0
        assert "type" in response_data["data"]["__schema"]["types"][5]["fields"][1]
        assert "kind" in response_data["data"]["__schema"]["types"][5]["fields"][1]["type"]
        assert response_data["data"]["__schema"]["types"][5]["fields"][1]["type"]["kind"] == "NON_NULL"
        assert "name" in response_data["data"]["__schema"]["types"][5]["fields"][1]["type"]
        assert response_data["data"]["__schema"]["types"][5]["fields"][1]["type"]["name"] == None
        assert "ofType" in response_data["data"]["__schema"]["types"][5]["fields"][1]["type"]
        assert "kind" in response_data["data"]["__schema"]["types"][5]["fields"][1]["type"]["ofType"]
        assert response_data["data"]["__schema"]["types"][5]["fields"][1]["type"]["ofType"]["kind"] == "SCALAR"
        assert "name" in response_data["data"]["__schema"]["types"][5]["fields"][1]["type"]["ofType"]
        assert response_data["data"]["__schema"]["types"][5]["fields"][1]["type"]["ofType"]["name"] == "String"
        assert "isDeprecated" in response_data["data"]["__schema"]["types"][5]["fields"][1]
        assert response_data["data"]["__schema"]["types"][5]["fields"][1]["isDeprecated"] == False
        assert "deprecationReason" in response_data["data"]["__schema"]["types"][5]["fields"][1]
        assert response_data["data"]["__schema"]["types"][5]["fields"][1]["deprecationReason"] == None
        assert "name" in response_data["data"]["__schema"]["types"][5]["fields"][2]
        assert response_data["data"]["__schema"]["types"][5]["fields"][2]["name"] == "content"
        assert "description" in response_data["data"]["__schema"]["types"][5]["fields"][2]
        assert response_data["data"]["__schema"]["types"][5]["fields"][2]["description"] == "Post content"
        assert "args" in response_data["data"]["__schema"]["types"][5]["fields"][2]
        assert len(response_data["data"]["__schema"]["types"][5]["fields"][2]["args"]) == 0
        assert "type" in response_data["data"]["__schema"]["types"][5]["fields"][2]
        assert "kind" in response_data["data"]["__schema"]["types"][5]["fields"][2]["type"]
        assert response_data["data"]["__schema"]["types"][5]["fields"][2]["type"]["kind"] == "NON_NULL"
        assert "name" in response_data["data"]["__schema"]["types"][5]["fields"][2]["type"]
        assert response_data["data"]["__schema"]["types"][5]["fields"][2]["type"]["name"] == None
        assert "ofType" in response_data["data"]["__schema"]["types"][5]["fields"][2]["type"]
        assert "kind" in response_data["data"]["__schema"]["types"][5]["fields"][2]["type"]["ofType"]
        assert response_data["data"]["__schema"]["types"][5]["fields"][2]["type"]["ofType"]["kind"] == "SCALAR"
        assert "name" in response_data["data"]["__schema"]["types"][5]["fields"][2]["type"]["ofType"]
        assert response_data["data"]["__schema"]["types"][5]["fields"][2]["type"]["ofType"]["name"] == "String"
        assert "isDeprecated" in response_data["data"]["__schema"]["types"][5]["fields"][2]
        assert response_data["data"]["__schema"]["types"][5]["fields"][2]["isDeprecated"] == False
        assert "deprecationReason" in response_data["data"]["__schema"]["types"][5]["fields"][2]
        assert response_data["data"]["__schema"]["types"][5]["fields"][2]["deprecationReason"] == None
        assert "name" in response_data["data"]["__schema"]["types"][5]["fields"][3]
        assert response_data["data"]["__schema"]["types"][5]["fields"][3]["name"] == "authorId"
        assert "description" in response_data["data"]["__schema"]["types"][5]["fields"][3]
        assert response_data["data"]["__schema"]["types"][5]["fields"][3]["description"] == "Author's ID"
        assert "args" in response_data["data"]["__schema"]["types"][5]["fields"][3]
        assert len(response_data["data"]["__schema"]["types"][5]["fields"][3]["args"]) == 0
        assert "type" in response_data["data"]["__schema"]["types"][5]["fields"][3]
        assert "kind" in response_data["data"]["__schema"]["types"][5]["fields"][3]["type"]
        assert response_data["data"]["__schema"]["types"][5]["fields"][3]["type"]["kind"] == "NON_NULL"
        assert "name" in response_data["data"]["__schema"]["types"][5]["fields"][3]["type"]
        assert response_data["data"]["__schema"]["types"][5]["fields"][3]["type"]["name"] == None
        assert "ofType" in response_data["data"]["__schema"]["types"][5]["fields"][3]["type"]
        assert "kind" in response_data["data"]["__schema"]["types"][5]["fields"][3]["type"]["ofType"]
        assert response_data["data"]["__schema"]["types"][5]["fields"][3]["type"]["ofType"]["kind"] == "SCALAR"
        assert "name" in response_data["data"]["__schema"]["types"][5]["fields"][3]["type"]["ofType"]
        assert response_data["data"]["__schema"]["types"][5]["fields"][3]["type"]["ofType"]["name"] == "UUID"
        assert "isDeprecated" in response_data["data"]["__schema"]["types"][5]["fields"][3]
        assert response_data["data"]["__schema"]["types"][5]["fields"][3]["isDeprecated"] == False
        assert "deprecationReason" in response_data["data"]["__schema"]["types"][5]["fields"][3]
        assert response_data["data"]["__schema"]["types"][5]["fields"][3]["deprecationReason"] == None
        assert "name" in response_data["data"]["__schema"]["types"][5]["fields"][4]
        assert response_data["data"]["__schema"]["types"][5]["fields"][4]["name"] == "author"
        assert "description" in response_data["data"]["__schema"]["types"][5]["fields"][4]
        assert response_data["data"]["__schema"]["types"][5]["fields"][4]["description"] == "Post author"
        assert "args" in response_data["data"]["__schema"]["types"][5]["fields"][4]
        assert len(response_data["data"]["__schema"]["types"][5]["fields"][4]["args"]) == 0
        assert "type" in response_data["data"]["__schema"]["types"][5]["fields"][4]
        assert "kind" in response_data["data"]["__schema"]["types"][5]["fields"][4]["type"]
        assert response_data["data"]["__schema"]["types"][5]["fields"][4]["type"]["kind"] == "NON_NULL"
        assert "name" in response_data["data"]["__schema"]["types"][5]["fields"][4]["type"]
        assert response_data["data"]["__schema"]["types"][5]["fields"][4]["type"]["name"] == None
        assert "ofType" in response_data["data"]["__schema"]["types"][5]["fields"][4]["type"]
        assert "kind" in response_data["data"]["__schema"]["types"][5]["fields"][4]["type"]["ofType"]
        assert response_data["data"]["__schema"]["types"][5]["fields"][4]["type"]["ofType"]["kind"] == "OBJECT"
        assert "name" in response_data["data"]["__schema"]["types"][5]["fields"][4]["type"]["ofType"]
        assert response_data["data"]["__schema"]["types"][5]["fields"][4]["type"]["ofType"]["name"] == "User"
        assert "isDeprecated" in response_data["data"]["__schema"]["types"][5]["fields"][4]
        assert response_data["data"]["__schema"]["types"][5]["fields"][4]["isDeprecated"] == False
        assert "deprecationReason" in response_data["data"]["__schema"]["types"][5]["fields"][4]
        assert response_data["data"]["__schema"]["types"][5]["fields"][4]["deprecationReason"] == None
        assert "name" in response_data["data"]["__schema"]["types"][5]["fields"][5]
        assert response_data["data"]["__schema"]["types"][5]["fields"][5]["name"] == "createdAt"
        assert "description" in response_data["data"]["__schema"]["types"][5]["fields"][5]
        assert response_data["data"]["__schema"]["types"][5]["fields"][5]["description"] == "Creation timestamp"
        assert "args" in response_data["data"]["__schema"]["types"][5]["fields"][5]
        assert len(response_data["data"]["__schema"]["types"][5]["fields"][5]["args"]) == 0
        assert "type" in response_data["data"]["__schema"]["types"][5]["fields"][5]
        assert "kind" in response_data["data"]["__schema"]["types"][5]["fields"][5]["type"]
        assert response_data["data"]["__schema"]["types"][5]["fields"][5]["type"]["kind"] == "NON_NULL"
        assert "name" in response_data["data"]["__schema"]["types"][5]["fields"][5]["type"]
        assert response_data["data"]["__schema"]["types"][5]["fields"][5]["type"]["name"] == None
        assert "ofType" in response_data["data"]["__schema"]["types"][5]["fields"][5]["type"]
        assert "kind" in response_data["data"]["__schema"]["types"][5]["fields"][5]["type"]["ofType"]
        assert response_data["data"]["__schema"]["types"][5]["fields"][5]["type"]["ofType"]["kind"] == "SCALAR"
        assert "name" in response_data["data"]["__schema"]["types"][5]["fields"][5]["type"]["ofType"]
        assert response_data["data"]["__schema"]["types"][5]["fields"][5]["type"]["ofType"]["name"] == "DateTime"
        assert "isDeprecated" in response_data["data"]["__schema"]["types"][5]["fields"][5]
        assert response_data["data"]["__schema"]["types"][5]["fields"][5]["isDeprecated"] == False
        assert "deprecationReason" in response_data["data"]["__schema"]["types"][5]["fields"][5]
        assert response_data["data"]["__schema"]["types"][5]["fields"][5]["deprecationReason"] == None
        assert "name" in response_data["data"]["__schema"]["types"][5]["fields"][6]
        assert response_data["data"]["__schema"]["types"][5]["fields"][6]["name"] == "updatedAt"
        assert "description" in response_data["data"]["__schema"]["types"][5]["fields"][6]
        assert response_data["data"]["__schema"]["types"][5]["fields"][6]["description"] == "Last update timestamp"
        assert "args" in response_data["data"]["__schema"]["types"][5]["fields"][6]
        assert len(response_data["data"]["__schema"]["types"][5]["fields"][6]["args"]) == 0
        assert "type" in response_data["data"]["__schema"]["types"][5]["fields"][6]
        assert "kind" in response_data["data"]["__schema"]["types"][5]["fields"][6]["type"]
        assert response_data["data"]["__schema"]["types"][5]["fields"][6]["type"]["kind"] == "NON_NULL"
        assert "name" in response_data["data"]["__schema"]["types"][5]["fields"][6]["type"]
        assert response_data["data"]["__schema"]["types"][5]["fields"][6]["type"]["name"] == None
        assert "ofType" in response_data["data"]["__schema"]["types"][5]["fields"][6]["type"]
        assert "kind" in response_data["data"]["__schema"]["types"][5]["fields"][6]["type"]["ofType"]
        assert response_data["data"]["__schema"]["types"][5]["fields"][6]["type"]["ofType"]["kind"] == "SCALAR"
        assert "name" in response_data["data"]["__schema"]["types"][5]["fields"][6]["type"]["ofType"]
        assert response_data["data"]["__schema"]["types"][5]["fields"][6]["type"]["ofType"]["name"] == "DateTime"
        assert "isDeprecated" in response_data["data"]["__schema"]["types"][5]["fields"][6]
        assert response_data["data"]["__schema"]["types"][5]["fields"][6]["isDeprecated"] == False
        assert "deprecationReason" in response_data["data"]["__schema"]["types"][5]["fields"][6]
        assert response_data["data"]["__schema"]["types"][5]["fields"][6]["deprecationReason"] == None
        assert "inputFields" in response_data["data"]["__schema"]["types"][5]
        assert response_data["data"]["__schema"]["types"][5]["inputFields"] == None
        assert "interfaces" in response_data["data"]["__schema"]["types"][5]
        assert len(response_data["data"]["__schema"]["types"][5]["interfaces"]) == 0
        assert "enumValues" in response_data["data"]["__schema"]["types"][5]
        assert response_data["data"]["__schema"]["types"][5]["enumValues"] == None
        assert "possibleTypes" in response_data["data"]["__schema"]["types"][5]
        assert response_data["data"]["__schema"]["types"][5]["possibleTypes"] == None
        assert "kind" in response_data["data"]["__schema"]["types"][6]
        assert response_data["data"]["__schema"]["types"][6]["kind"] == "INPUT_OBJECT"
        assert "name" in response_data["data"]["__schema"]["types"][6]
        assert response_data["data"]["__schema"]["types"][6]["name"] == "CreatePostInput"
        assert "description" in response_data["data"]["__schema"]["types"][6]
        assert response_data["data"]["__schema"]["types"][6]["description"] == "Input for creating posts"
        assert "fields" in response_data["data"]["__schema"]["types"][6]
        assert response_data["data"]["__schema"]["types"][6]["fields"] == None
        assert "inputFields" in response_data["data"]["__schema"]["types"][6]
        assert len(response_data["data"]["__schema"]["types"][6]["inputFields"]) == 3
        assert "name" in response_data["data"]["__schema"]["types"][6]["inputFields"][0]
        assert response_data["data"]["__schema"]["types"][6]["inputFields"][0]["name"] == "title"
        assert "description" in response_data["data"]["__schema"]["types"][6]["inputFields"][0]
        assert response_data["data"]["__schema"]["types"][6]["inputFields"][0]["description"] == "Post title"
        assert "type" in response_data["data"]["__schema"]["types"][6]["inputFields"][0]
        assert "kind" in response_data["data"]["__schema"]["types"][6]["inputFields"][0]["type"]
        assert response_data["data"]["__schema"]["types"][6]["inputFields"][0]["type"]["kind"] == "NON_NULL"
        assert "name" in response_data["data"]["__schema"]["types"][6]["inputFields"][0]["type"]
        assert response_data["data"]["__schema"]["types"][6]["inputFields"][0]["type"]["name"] == None
        assert "ofType" in response_data["data"]["__schema"]["types"][6]["inputFields"][0]["type"]
        assert "kind" in response_data["data"]["__schema"]["types"][6]["inputFields"][0]["type"]["ofType"]
        assert response_data["data"]["__schema"]["types"][6]["inputFields"][0]["type"]["ofType"]["kind"] == "SCALAR"
        assert "name" in response_data["data"]["__schema"]["types"][6]["inputFields"][0]["type"]["ofType"]
        assert response_data["data"]["__schema"]["types"][6]["inputFields"][0]["type"]["ofType"]["name"] == "String"
        assert "defaultValue" in response_data["data"]["__schema"]["types"][6]["inputFields"][0]
        assert response_data["data"]["__schema"]["types"][6]["inputFields"][0]["defaultValue"] == None
        assert "name" in response_data["data"]["__schema"]["types"][6]["inputFields"][1]
        assert response_data["data"]["__schema"]["types"][6]["inputFields"][1]["name"] == "content"
        assert "description" in response_data["data"]["__schema"]["types"][6]["inputFields"][1]
        assert response_data["data"]["__schema"]["types"][6]["inputFields"][1]["description"] == "Post content"
        assert "type" in response_data["data"]["__schema"]["types"][6]["inputFields"][1]
        assert "kind" in response_data["data"]["__schema"]["types"][6]["inputFields"][1]["type"]
        assert response_data["data"]["__schema"]["types"][6]["inputFields"][1]["type"]["kind"] == "NON_NULL"
        assert "name" in response_data["data"]["__schema"]["types"][6]["inputFields"][1]["type"]
        assert response_data["data"]["__schema"]["types"][6]["inputFields"][1]["type"]["name"] == None
        assert "ofType" in response_data["data"]["__schema"]["types"][6]["inputFields"][1]["type"]
        assert "kind" in response_data["data"]["__schema"]["types"][6]["inputFields"][1]["type"]["ofType"]
        assert response_data["data"]["__schema"]["types"][6]["inputFields"][1]["type"]["ofType"]["kind"] == "SCALAR"
        assert "name" in response_data["data"]["__schema"]["types"][6]["inputFields"][1]["type"]["ofType"]
        assert response_data["data"]["__schema"]["types"][6]["inputFields"][1]["type"]["ofType"]["name"] == "String"
        assert "defaultValue" in response_data["data"]["__schema"]["types"][6]["inputFields"][1]
        assert response_data["data"]["__schema"]["types"][6]["inputFields"][1]["defaultValue"] == None
        assert "name" in response_data["data"]["__schema"]["types"][6]["inputFields"][2]
        assert response_data["data"]["__schema"]["types"][6]["inputFields"][2]["name"] == "authorId"
        assert "description" in response_data["data"]["__schema"]["types"][6]["inputFields"][2]
        assert response_data["data"]["__schema"]["types"][6]["inputFields"][2]["description"] == "Author ID"
        assert "type" in response_data["data"]["__schema"]["types"][6]["inputFields"][2]
        assert "kind" in response_data["data"]["__schema"]["types"][6]["inputFields"][2]["type"]
        assert response_data["data"]["__schema"]["types"][6]["inputFields"][2]["type"]["kind"] == "NON_NULL"
        assert "name" in response_data["data"]["__schema"]["types"][6]["inputFields"][2]["type"]
        assert response_data["data"]["__schema"]["types"][6]["inputFields"][2]["type"]["name"] == None
        assert "ofType" in response_data["data"]["__schema"]["types"][6]["inputFields"][2]["type"]
        assert "kind" in response_data["data"]["__schema"]["types"][6]["inputFields"][2]["type"]["ofType"]
        assert response_data["data"]["__schema"]["types"][6]["inputFields"][2]["type"]["ofType"]["kind"] == "SCALAR"
        assert "name" in response_data["data"]["__schema"]["types"][6]["inputFields"][2]["type"]["ofType"]
        assert response_data["data"]["__schema"]["types"][6]["inputFields"][2]["type"]["ofType"]["name"] == "UUID"
        assert "defaultValue" in response_data["data"]["__schema"]["types"][6]["inputFields"][2]
        assert response_data["data"]["__schema"]["types"][6]["inputFields"][2]["defaultValue"] == None
        assert "interfaces" in response_data["data"]["__schema"]["types"][6]
        assert response_data["data"]["__schema"]["types"][6]["interfaces"] == None
        assert "enumValues" in response_data["data"]["__schema"]["types"][6]
        assert response_data["data"]["__schema"]["types"][6]["enumValues"] == None
        assert "possibleTypes" in response_data["data"]["__schema"]["types"][6]
        assert response_data["data"]["__schema"]["types"][6]["possibleTypes"] == None
        assert "directives" in response_data["data"]["__schema"]
        assert len(response_data["data"]["__schema"]["directives"]) == 3
        assert "name" in response_data["data"]["__schema"]["directives"][0]
        assert response_data["data"]["__schema"]["directives"][0]["name"] == "skip"
        assert "description" in response_data["data"]["__schema"]["directives"][0]
        assert (
            response_data["data"]["__schema"]["directives"][0]["description"]
            == "Directs the executor to skip this field or fragment when the `if` argument is true."
        )
        assert "locations" in response_data["data"]["__schema"]["directives"][0]
        assert len(response_data["data"]["__schema"]["directives"][0]["locations"]) == 3
        assert response_data["data"]["__schema"]["directives"][0]["locations"][0] == "FIELD"
        assert response_data["data"]["__schema"]["directives"][0]["locations"][1] == "FRAGMENT_SPREAD"
        assert response_data["data"]["__schema"]["directives"][0]["locations"][2] == "INLINE_FRAGMENT"
        assert "args" in response_data["data"]["__schema"]["directives"][0]
        assert len(response_data["data"]["__schema"]["directives"][0]["args"]) == 1
        assert "name" in response_data["data"]["__schema"]["directives"][0]["args"][0]
        assert response_data["data"]["__schema"]["directives"][0]["args"][0]["name"] == "if"
        assert "description" in response_data["data"]["__schema"]["directives"][0]["args"][0]
        assert response_data["data"]["__schema"]["directives"][0]["args"][0]["description"] == "Skipped when true"
        assert "type" in response_data["data"]["__schema"]["directives"][0]["args"][0]
        assert "kind" in response_data["data"]["__schema"]["directives"][0]["args"][0]["type"]
        assert response_data["data"]["__schema"]["directives"][0]["args"][0]["type"]["kind"] == "NON_NULL"
        assert "name" in response_data["data"]["__schema"]["directives"][0]["args"][0]["type"]
        assert response_data["data"]["__schema"]["directives"][0]["args"][0]["type"]["name"] == None
        assert "ofType" in response_data["data"]["__schema"]["directives"][0]["args"][0]["type"]
        assert "kind" in response_data["data"]["__schema"]["directives"][0]["args"][0]["type"]["ofType"]
        assert response_data["data"]["__schema"]["directives"][0]["args"][0]["type"]["ofType"]["kind"] == "SCALAR"
        assert "name" in response_data["data"]["__schema"]["directives"][0]["args"][0]["type"]["ofType"]
        assert response_data["data"]["__schema"]["directives"][0]["args"][0]["type"]["ofType"]["name"] == "Boolean"
        assert "defaultValue" in response_data["data"]["__schema"]["directives"][0]["args"][0]
        assert response_data["data"]["__schema"]["directives"][0]["args"][0]["defaultValue"] == None
        assert "name" in response_data["data"]["__schema"]["directives"][1]
        assert response_data["data"]["__schema"]["directives"][1]["name"] == "include"
        assert "description" in response_data["data"]["__schema"]["directives"][1]
        assert (
            response_data["data"]["__schema"]["directives"][1]["description"]
            == "Directs the executor to include this field or fragment when the `if` argument is true."
        )
        assert "locations" in response_data["data"]["__schema"]["directives"][1]
        assert len(response_data["data"]["__schema"]["directives"][1]["locations"]) == 3
        assert response_data["data"]["__schema"]["directives"][1]["locations"][0] == "FIELD"
        assert response_data["data"]["__schema"]["directives"][1]["locations"][1] == "FRAGMENT_SPREAD"
        assert response_data["data"]["__schema"]["directives"][1]["locations"][2] == "INLINE_FRAGMENT"
        assert "args" in response_data["data"]["__schema"]["directives"][1]
        assert len(response_data["data"]["__schema"]["directives"][1]["args"]) == 1
        assert "name" in response_data["data"]["__schema"]["directives"][1]["args"][0]
        assert response_data["data"]["__schema"]["directives"][1]["args"][0]["name"] == "if"
        assert "description" in response_data["data"]["__schema"]["directives"][1]["args"][0]
        assert response_data["data"]["__schema"]["directives"][1]["args"][0]["description"] == "Included when true"
        assert "type" in response_data["data"]["__schema"]["directives"][1]["args"][0]
        assert "kind" in response_data["data"]["__schema"]["directives"][1]["args"][0]["type"]
        assert response_data["data"]["__schema"]["directives"][1]["args"][0]["type"]["kind"] == "NON_NULL"
        assert "name" in response_data["data"]["__schema"]["directives"][1]["args"][0]["type"]
        assert response_data["data"]["__schema"]["directives"][1]["args"][0]["type"]["name"] == None
        assert "ofType" in response_data["data"]["__schema"]["directives"][1]["args"][0]["type"]
        assert "kind" in response_data["data"]["__schema"]["directives"][1]["args"][0]["type"]["ofType"]
        assert response_data["data"]["__schema"]["directives"][1]["args"][0]["type"]["ofType"]["kind"] == "SCALAR"
        assert "name" in response_data["data"]["__schema"]["directives"][1]["args"][0]["type"]["ofType"]
        assert response_data["data"]["__schema"]["directives"][1]["args"][0]["type"]["ofType"]["name"] == "Boolean"
        assert "defaultValue" in response_data["data"]["__schema"]["directives"][1]["args"][0]
        assert response_data["data"]["__schema"]["directives"][1]["args"][0]["defaultValue"] == None
        assert "name" in response_data["data"]["__schema"]["directives"][2]
        assert response_data["data"]["__schema"]["directives"][2]["name"] == "deprecated"
        assert "description" in response_data["data"]["__schema"]["directives"][2]
        assert (
            response_data["data"]["__schema"]["directives"][2]["description"]
            == "Marks an element of a GraphQL schema as no longer supported."
        )
        assert "locations" in response_data["data"]["__schema"]["directives"][2]
        assert len(response_data["data"]["__schema"]["directives"][2]["locations"]) == 2
        assert response_data["data"]["__schema"]["directives"][2]["locations"][0] == "FIELD_DEFINITION"
        assert response_data["data"]["__schema"]["directives"][2]["locations"][1] == "ENUM_VALUE"
        assert "args" in response_data["data"]["__schema"]["directives"][2]
        assert len(response_data["data"]["__schema"]["directives"][2]["args"]) == 1
        assert "name" in response_data["data"]["__schema"]["directives"][2]["args"][0]
        assert response_data["data"]["__schema"]["directives"][2]["args"][0]["name"] == "reason"
        assert "description" in response_data["data"]["__schema"]["directives"][2]["args"][0]
        assert (
            response_data["data"]["__schema"]["directives"][2]["args"][0]["description"]
            == "Explains why this element was deprecated"
        )
        assert "type" in response_data["data"]["__schema"]["directives"][2]["args"][0]
        assert "kind" in response_data["data"]["__schema"]["directives"][2]["args"][0]["type"]
        assert response_data["data"]["__schema"]["directives"][2]["args"][0]["type"]["kind"] == "SCALAR"
        assert "name" in response_data["data"]["__schema"]["directives"][2]["args"][0]["type"]
        assert response_data["data"]["__schema"]["directives"][2]["args"][0]["type"]["name"] == "String"
        assert "defaultValue" in response_data["data"]["__schema"]["directives"][2]["args"][0]
        assert response_data["data"]["__schema"]["directives"][2]["args"][0]["defaultValue"] == "No longer supported"
        response_data = response.json()
        assert response_data.get("errors") is None or response_data.get("errors") == []


@pytest.mark.asyncio
async def test_graphql_subscription_error() -> None:
    """Subscription to non-existent subscription field returns GraphQL error with proper error format."""

    app_factory = getattr(app_main, "create_app_graphql_subscription_error", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_subscription_error")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="subscription {\n  invalidSubscription {\n    id\n    data\n  }\n}",
            variables=None,
            operation_name=None,
            path="/graphql",
        )

        assert response.status_code == 400
        response_data = response.json()
        assert response_data.get("errors") is None or response_data.get("errors") == []


@pytest.mark.asyncio
async def test_graphql_subscription_unsubscribe() -> None:
    """Subscription lifecycle test: subscribe to ticker, receive events, then unsubscribe and verify no more events are received."""

    app_factory = getattr(app_main, "create_app_graphql_subscription_unsubscribe", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_subscription_unsubscribe")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="subscription OnTick {\n  ticker {\n    id\n    symbol\n    price\n    timestamp\n  }\n}",
            variables=None,
            operation_name="OnTick",
            path="/graphql",
        )

        assert response.status_code == 200
        response_data = response.json()
        assert "data" in response_data
        assert "ticker" in response_data["data"]
        assert "id" in response_data["data"]["ticker"]
        assert response_data["data"]["ticker"]["id"] == "tick-1"
        assert "symbol" in response_data["data"]["ticker"]
        assert response_data["data"]["ticker"]["symbol"] == "AAPL"
        assert "price" in response_data["data"]["ticker"]
        assert response_data["data"]["ticker"]["price"] == 195.45
        assert "timestamp" in response_data["data"]["ticker"]
        assert response_data["data"]["ticker"]["timestamp"] == "2025-12-27T15:00:00Z"
        response_data = response.json()
        assert response_data.get("errors") is None or response_data.get("errors") == []


@pytest.mark.asyncio
async def test_graphql_subscription_connection_params() -> None:
    """Subscription with connection initialization parameters containing auth token for WebSocket subprotocol graphql-ws."""

    app_factory = getattr(app_main, "create_app_graphql_subscription_connection_params", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_subscription_connection_params")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="subscription {\n  secureStream {\n    id\n    data\n    timestamp\n  }\n}",
            variables=None,
            operation_name=None,
            path="/graphql",
        )

        assert response.status_code == 101
        response_data = response.json()
        assert "data" in response_data
        assert "secureStream" in response_data["data"]
        assert "id" in response_data["data"]["secureStream"]
        assert response_data["data"]["secureStream"]["id"] == "stream-1"
        assert "data" in response_data["data"]["secureStream"]
        assert response_data["data"]["secureStream"]["data"] == "Connection established"
        assert "timestamp" in response_data["data"]["secureStream"]
        assert response_data["data"]["secureStream"]["timestamp"] == "2025-12-27T14:00:00Z"
        response_data = response.json()
        assert response_data.get("errors") is None or response_data.get("errors") == []


@pytest.mark.asyncio
async def test_graphql_simple_subscription() -> None:
    """Basic subscription to a single event stream returning scalar and timestamp fields."""

    app_factory = getattr(app_main, "create_app_graphql_simple_subscription", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_simple_subscription")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="subscription {\n  messageAdded {\n    id\n    text\n    timestamp\n  }\n}",
            variables=None,
            operation_name=None,
            path="/graphql",
        )

        assert response.status_code == 200
        response_data = response.json()
        assert "data" in response_data
        assert "messageAdded" in response_data["data"]
        assert "id" in response_data["data"]["messageAdded"]
        assert response_data["data"]["messageAdded"]["id"] == "msg-1"
        assert "text" in response_data["data"]["messageAdded"]
        assert response_data["data"]["messageAdded"]["text"] == "Hello, WebSocket!"
        assert "timestamp" in response_data["data"]["messageAdded"]
        assert response_data["data"]["messageAdded"]["timestamp"] == "2025-12-27T10:00:00Z"
        response_data = response.json()
        assert response_data.get("errors") is None or response_data.get("errors") == []


@pytest.mark.asyncio
async def test_graphql_subscription_multiple_fields() -> None:
    """Multiple concurrent subscription fields in single query to test multiplexing and parallel event delivery."""

    app_factory = getattr(app_main, "create_app_graphql_subscription_multiple_fields", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_subscription_multiple_fields")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="subscription MultiStream {\n  messageAdded {\n    id\n    text\n    author\n  }\n  userOnline {\n    userId\n    username\n    isOnline\n    lastSeen\n  }\n}",
            variables=None,
            operation_name="MultiStream",
            path="/graphql",
        )

        assert response.status_code == 200
        response_data = response.json()
        assert "data" in response_data
        assert "messageAdded" in response_data["data"]
        assert "id" in response_data["data"]["messageAdded"]
        assert response_data["data"]["messageAdded"]["id"] == "msg-101"
        assert "text" in response_data["data"]["messageAdded"]
        assert response_data["data"]["messageAdded"]["text"] == "Hey everyone!"
        assert "author" in response_data["data"]["messageAdded"]
        assert response_data["data"]["messageAdded"]["author"] == "alice"
        assert "userOnline" in response_data["data"]
        assert "userId" in response_data["data"]["userOnline"]
        assert response_data["data"]["userOnline"]["userId"] == "user-42"
        assert "username" in response_data["data"]["userOnline"]
        assert response_data["data"]["userOnline"]["username"] == "bob"
        assert "isOnline" in response_data["data"]["userOnline"]
        assert response_data["data"]["userOnline"]["isOnline"] == True
        assert "lastSeen" in response_data["data"]["userOnline"]
        assert response_data["data"]["userOnline"]["lastSeen"] == "2025-12-27T13:00:00Z"
        response_data = response.json()
        assert response_data.get("errors") is None or response_data.get("errors") == []


@pytest.mark.asyncio
async def test_graphql_subscription_with_variables() -> None:
    """Subscription using GraphQL variables to filter events by user ID with required variable."""

    app_factory = getattr(app_main, "create_app_graphql_subscription_with_variables", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_subscription_with_variables")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="subscription OnUserActivity($userId: ID!) {\n  userActivity(userId: $userId) {\n    id\n    userId\n    action\n    description\n    timestamp\n  }\n}",
            variables={"userId": "user123"},
            operation_name="OnUserActivity",
            path="/graphql",
        )

        assert response.status_code == 200
        response_data = response.json()
        assert "data" in response_data
        assert "userActivity" in response_data["data"]
        assert "id" in response_data["data"]["userActivity"]
        assert response_data["data"]["userActivity"]["id"] == "event-789"
        assert "userId" in response_data["data"]["userActivity"]
        assert response_data["data"]["userActivity"]["userId"] == "user123"
        assert "action" in response_data["data"]["userActivity"]
        assert response_data["data"]["userActivity"]["action"] == "LOGIN"
        assert "description" in response_data["data"]["userActivity"]
        assert response_data["data"]["userActivity"]["description"] == "User logged in from browser"
        assert "timestamp" in response_data["data"]["userActivity"]
        assert response_data["data"]["userActivity"]["timestamp"] == "2025-12-27T12:15:00Z"
        response_data = response.json()
        assert response_data.get("errors") is None or response_data.get("errors") == []


@pytest.mark.asyncio
async def test_graphql_subscription_with_auth_middleware() -> None:
    """Subscription requiring JWT authentication in connection params via graphql-ws protocol, validates auth during WebSocket handshake with middleware."""

    app_factory = getattr(app_main, "create_app_graphql_subscription_with_auth_middleware", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_subscription_with_auth_middleware")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="subscription {\n  privateNotifications {\n    id\n    userId\n    type\n    message\n    priority\n    createdAt\n  }\n}",
            variables=None,
            operation_name=None,
            path="/graphql",
        )

        assert response.status_code == 101
        response_data = response.json()
        assert "data" in response_data
        assert "privateNotifications" in response_data["data"]
        assert "id" in response_data["data"]["privateNotifications"]
        assert response_data["data"]["privateNotifications"]["id"] == "notif-456"
        assert "userId" in response_data["data"]["privateNotifications"]
        assert response_data["data"]["privateNotifications"]["userId"] == "user123"
        assert "type" in response_data["data"]["privateNotifications"]
        assert response_data["data"]["privateNotifications"]["type"] == "ALERT"
        assert "message" in response_data["data"]["privateNotifications"]
        assert response_data["data"]["privateNotifications"]["message"] == "Your subscription is about to expire"
        assert "priority" in response_data["data"]["privateNotifications"]
        assert response_data["data"]["privateNotifications"]["priority"] == "HIGH"
        assert "createdAt" in response_data["data"]["privateNotifications"]
        assert response_data["data"]["privateNotifications"]["createdAt"] == "2025-12-27T16:20:00Z"
        response_data = response.json()
        assert response_data.get("errors") is None or response_data.get("errors") == []


@pytest.mark.asyncio
async def test_graphql_subscription_rate_limited() -> None:
    """Subscription with rate limiting middleware enforcing maximum message throughput per subscription, returns 429 when threshold exceeded."""

    app_factory = getattr(app_main, "create_app_graphql_subscription_rate_limited", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_subscription_rate_limited")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="subscription OnStockUpdate($symbol: String!) {\n  stockTicker(symbol: $symbol) {\n    id\n    symbol\n    price\n    change\n    changePercent\n    timestamp\n    volume\n  }\n}",
            variables={"symbol": "AAPL"},
            operation_name="OnStockUpdate",
            path="/graphql",
        )

        assert response.status_code == 200
        response_data = response.json()
        assert "data" in response_data
        assert "stockTicker" in response_data["data"]
        assert "id" in response_data["data"]["stockTicker"]
        assert response_data["data"]["stockTicker"]["id"] == "stock-aapl-1"
        assert "symbol" in response_data["data"]["stockTicker"]
        assert response_data["data"]["stockTicker"]["symbol"] == "AAPL"
        assert "price" in response_data["data"]["stockTicker"]
        assert response_data["data"]["stockTicker"]["price"] == 238.45
        assert "change" in response_data["data"]["stockTicker"]
        assert response_data["data"]["stockTicker"]["change"] == 2.15
        assert "changePercent" in response_data["data"]["stockTicker"]
        assert response_data["data"]["stockTicker"]["changePercent"] == 0.91
        assert "timestamp" in response_data["data"]["stockTicker"]
        assert response_data["data"]["stockTicker"]["timestamp"] == "2025-12-27T17:00:00Z"
        assert "volume" in response_data["data"]["stockTicker"]
        assert response_data["data"]["stockTicker"]["volume"] == 52345678
        response_data = response.json()
        assert response_data.get("errors") is None or response_data.get("errors") == []


@pytest.mark.asyncio
async def test_graphql_subscription_authentication() -> None:
    """Subscription to private messages that requires authentication token in headers, returns 401 when auth is missing."""

    app_factory = getattr(app_main, "create_app_graphql_subscription_authentication", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_subscription_authentication")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="subscription {\n  privateMessages {\n    id\n    from\n    content\n    isPrivate\n  }\n}",
            variables=None,
            operation_name=None,
            path="/graphql",
        )

        assert response.status_code == 401
        response_data = response.json()
        assert response_data.get("errors") is None or response_data.get("errors") == []


@pytest.mark.asyncio
async def test_graphql_subscription_with_filtering() -> None:
    """Advanced subscription with complex multi-field filtering on post updates, returning only posts matching multiple criteria."""

    app_factory = getattr(app_main, "create_app_graphql_subscription_with_filtering", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_subscription_with_filtering")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="subscription OnPostUpdated($authorId: ID!, $statuses: [PostStatus!]!, $tagFilter: String, $scoreThreshold: Int) {\n  postUpdated(filter: {\n    authorId: $authorId\n    status: $statuses\n    tags_contains: $tagFilter\n    minScore: $scoreThreshold\n  }) {\n    id\n    title\n    authorId\n    content\n    status\n    tags\n    score\n    updatedAt\n  }\n}",
            variables={
                "authorId": "123",
                "statuses": ["PUBLISHED", "DRAFT"],
                "tagFilter": "graphql",
                "scoreThreshold": 50,
            },
            operation_name="OnPostUpdated",
            path="/graphql",
        )

        assert response.status_code == 200
        response_data = response.json()
        assert "data" in response_data
        assert "postUpdated" in response_data["data"]
        assert "id" in response_data["data"]["postUpdated"]
        assert response_data["data"]["postUpdated"]["id"] == "post-789"
        assert "title" in response_data["data"]["postUpdated"]
        assert response_data["data"]["postUpdated"]["title"] == "Advanced GraphQL Patterns"
        assert "authorId" in response_data["data"]["postUpdated"]
        assert response_data["data"]["postUpdated"]["authorId"] == "123"
        assert "content" in response_data["data"]["postUpdated"]
        assert (
            response_data["data"]["postUpdated"]["content"]
            == "A comprehensive guide to GraphQL subscriptions with advanced filtering techniques..."
        )
        assert "status" in response_data["data"]["postUpdated"]
        assert response_data["data"]["postUpdated"]["status"] == "PUBLISHED"
        assert "tags" in response_data["data"]["postUpdated"]
        assert len(response_data["data"]["postUpdated"]["tags"]) == 3
        assert response_data["data"]["postUpdated"]["tags"][0] == "graphql"
        assert response_data["data"]["postUpdated"]["tags"][1] == "subscriptions"
        assert response_data["data"]["postUpdated"]["tags"][2] == "real-time"
        assert "score" in response_data["data"]["postUpdated"]
        assert response_data["data"]["postUpdated"]["score"] == 95
        assert "updatedAt" in response_data["data"]["postUpdated"]
        assert response_data["data"]["postUpdated"]["updatedAt"] == "2025-12-27T15:45:00Z"
        response_data = response.json()
        assert response_data.get("errors") is None or response_data.get("errors") == []


@pytest.mark.asyncio
async def test_graphql_filtered_subscription() -> None:
    """Subscription with filter arguments to receive events matching specific status values."""

    app_factory = getattr(app_main, "create_app_graphql_filtered_subscription", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_filtered_subscription")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="subscription OnOrderUpdated($status: OrderStatus) {\n  orderUpdated(status: $status) {\n    id\n    orderId\n    status\n    amount\n    updatedAt\n  }\n}",
            variables={"status": "SHIPPED"},
            operation_name="OnOrderUpdated",
            path="/graphql",
        )

        assert response.status_code == 200
        response_data = response.json()
        assert "data" in response_data
        assert "orderUpdated" in response_data["data"]
        assert "id" in response_data["data"]["orderUpdated"]
        assert response_data["data"]["orderUpdated"]["id"] == "order-456"
        assert "orderId" in response_data["data"]["orderUpdated"]
        assert response_data["data"]["orderUpdated"]["orderId"] == "ORD-2025-00123"
        assert "status" in response_data["data"]["orderUpdated"]
        assert response_data["data"]["orderUpdated"]["status"] == "SHIPPED"
        assert "amount" in response_data["data"]["orderUpdated"]
        assert response_data["data"]["orderUpdated"]["amount"] == 149.99
        assert "updatedAt" in response_data["data"]["orderUpdated"]
        assert response_data["data"]["orderUpdated"]["updatedAt"] == "2025-12-27T11:30:00Z"
        response_data = response.json()
        assert response_data.get("errors") is None or response_data.get("errors") == []


@pytest.mark.asyncio
async def test_graphql_entity_with_key() -> None:
    """Entity with @key directive for federation subgraph key definition."""

    app_factory = getattr(app_main, "create_app_graphql_entity_with_key", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_entity_with_key")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query='query {\n  _entities(representations: [{__typename: "User", id: "42"}]) {\n    ... on User {\n      id\n      name\n      username\n      profile {\n        bio\n        avatar\n        joinDate\n      }\n    }\n  }\n}',
            variables=None,
            operation_name=None,
            path="/graphql",
        )

        assert response.status_code == 200
        response_data = response.json()
        assert "data" in response_data
        assert "_entities" in response_data["data"]
        assert len(response_data["data"]["_entities"]) == 1
        assert "id" in response_data["data"]["_entities"][0]
        assert response_data["data"]["_entities"][0]["id"] == "42"
        assert "name" in response_data["data"]["_entities"][0]
        assert response_data["data"]["_entities"][0]["name"] == "Bob Smith"
        assert "username" in response_data["data"]["_entities"][0]
        assert response_data["data"]["_entities"][0]["username"] == "bobsmith"
        assert "profile" in response_data["data"]["_entities"][0]
        assert "bio" in response_data["data"]["_entities"][0]["profile"]
        assert response_data["data"]["_entities"][0]["profile"]["bio"] == "Software engineer and open source enthusiast"
        assert "avatar" in response_data["data"]["_entities"][0]["profile"]
        assert response_data["data"]["_entities"][0]["profile"]["avatar"] == "https://example.com/avatars/bob.jpg"
        assert "joinDate" in response_data["data"]["_entities"][0]["profile"]
        assert response_data["data"]["_entities"][0]["profile"]["joinDate"] == "2020-03-15"
        response_data = response.json()
        assert response_data.get("errors") is None or response_data.get("errors") == []


@pytest.mark.asyncio
async def test_graphql_requires_directive() -> None:
    """Field with @requires directive for dependent field resolution."""

    app_factory = getattr(app_main, "create_app_graphql_requires_directive", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_requires_directive")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query='query {\n  _entities(representations: [{__typename: "Shipment", id: "ship-001", weight: 5.5, destination: "NYC"}]) {\n    ... on Shipment {\n      id\n      weight\n      destination\n      shippingEstimate\n    }\n  }\n}',
            variables=None,
            operation_name=None,
            path="/graphql",
        )

        assert response.status_code == 200
        response_data = response.json()
        assert "data" in response_data
        assert "_entities" in response_data["data"]
        assert len(response_data["data"]["_entities"]) == 1
        assert "id" in response_data["data"]["_entities"][0]
        assert response_data["data"]["_entities"][0]["id"] == "ship-001"
        assert "weight" in response_data["data"]["_entities"][0]
        assert response_data["data"]["_entities"][0]["weight"] == 5.5
        assert "destination" in response_data["data"]["_entities"][0]
        assert response_data["data"]["_entities"][0]["destination"] == "NYC"
        assert "shippingEstimate" in response_data["data"]["_entities"][0]
        assert response_data["data"]["_entities"][0]["shippingEstimate"] == 24.75
        response_data = response.json()
        assert response_data.get("errors") is None or response_data.get("errors") == []


@pytest.mark.asyncio
async def test_graphql_cross_subgraph_query() -> None:
    """Query spanning multiple subgraphs with federation entity references."""

    app_factory = getattr(app_main, "create_app_graphql_cross_subgraph_query", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_cross_subgraph_query")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query='query {\n  user(id: "usr-42") {\n    id\n    name\n    email\n    orders {\n      id\n      orderId\n      total\n      status\n      createdAt\n    }\n  }\n}',
            variables=None,
            operation_name=None,
            path="/graphql",
        )

        assert response.status_code == 200
        response_data = response.json()
        assert "data" in response_data
        assert "user" in response_data["data"]
        assert "id" in response_data["data"]["user"]
        assert response_data["data"]["user"]["id"] == "usr-42"
        assert "name" in response_data["data"]["user"]
        assert response_data["data"]["user"]["name"] == "Emma Wilson"
        assert "email" in response_data["data"]["user"]
        assert response_data["data"]["user"]["email"] == "emma@example.com"
        assert "orders" in response_data["data"]["user"]
        assert len(response_data["data"]["user"]["orders"]) == 2
        assert "id" in response_data["data"]["user"]["orders"][0]
        assert response_data["data"]["user"]["orders"][0]["id"] == "order-101"
        assert "orderId" in response_data["data"]["user"]["orders"][0]
        assert response_data["data"]["user"]["orders"][0]["orderId"] == "ORD-2024-001"
        assert "total" in response_data["data"]["user"]["orders"][0]
        assert response_data["data"]["user"]["orders"][0]["total"] == 149.99
        assert "status" in response_data["data"]["user"]["orders"][0]
        assert response_data["data"]["user"]["orders"][0]["status"] == "DELIVERED"
        assert "createdAt" in response_data["data"]["user"]["orders"][0]
        assert response_data["data"]["user"]["orders"][0]["createdAt"] == "2024-01-15T10:30:00Z"
        assert "id" in response_data["data"]["user"]["orders"][1]
        assert response_data["data"]["user"]["orders"][1]["id"] == "order-102"
        assert "orderId" in response_data["data"]["user"]["orders"][1]
        assert response_data["data"]["user"]["orders"][1]["orderId"] == "ORD-2024-002"
        assert "total" in response_data["data"]["user"]["orders"][1]
        assert response_data["data"]["user"]["orders"][1]["total"] == 89.5
        assert "status" in response_data["data"]["user"]["orders"][1]
        assert response_data["data"]["user"]["orders"][1]["status"] == "PROCESSING"
        assert "createdAt" in response_data["data"]["user"]["orders"][1]
        assert response_data["data"]["user"]["orders"][1]["createdAt"] == "2024-12-20T14:22:00Z"
        response_data = response.json()
        assert response_data.get("errors") is None or response_data.get("errors") == []


@pytest.mark.asyncio
async def test_graphql_provides_directive() -> None:
    """Field with @provides directive for optimized nested field resolution."""

    app_factory = getattr(app_main, "create_app_graphql_provides_directive", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_provides_directive")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query='query {\n  _entities(representations: [{__typename: "Post", id: "post-123"}]) {\n    ... on Post {\n      id\n      title\n      content\n      reviews {\n        id\n        rating\n        text\n        author {\n          id\n          name\n        }\n      }\n    }\n  }\n}',
            variables=None,
            operation_name=None,
            path="/graphql",
        )

        assert response.status_code == 200
        response_data = response.json()
        assert "data" in response_data
        assert "_entities" in response_data["data"]
        assert len(response_data["data"]["_entities"]) == 1
        assert "id" in response_data["data"]["_entities"][0]
        assert response_data["data"]["_entities"][0]["id"] == "post-123"
        assert "title" in response_data["data"]["_entities"][0]
        assert response_data["data"]["_entities"][0]["title"] == "Getting Started with GraphQL Federation"
        assert "content" in response_data["data"]["_entities"][0]
        assert response_data["data"]["_entities"][0]["content"] == "Learn how to build scalable microservices..."
        assert "reviews" in response_data["data"]["_entities"][0]
        assert len(response_data["data"]["_entities"][0]["reviews"]) == 2
        assert "id" in response_data["data"]["_entities"][0]["reviews"][0]
        assert response_data["data"]["_entities"][0]["reviews"][0]["id"] == "rev-001"
        assert "rating" in response_data["data"]["_entities"][0]["reviews"][0]
        assert response_data["data"]["_entities"][0]["reviews"][0]["rating"] == 5
        assert "text" in response_data["data"]["_entities"][0]["reviews"][0]
        assert response_data["data"]["_entities"][0]["reviews"][0]["text"] == "Excellent post!"
        assert "author" in response_data["data"]["_entities"][0]["reviews"][0]
        assert "id" in response_data["data"]["_entities"][0]["reviews"][0]["author"]
        assert response_data["data"]["_entities"][0]["reviews"][0]["author"]["id"] == "user-1"
        assert "name" in response_data["data"]["_entities"][0]["reviews"][0]["author"]
        assert response_data["data"]["_entities"][0]["reviews"][0]["author"]["name"] == "Charlie Brown"
        assert "id" in response_data["data"]["_entities"][0]["reviews"][1]
        assert response_data["data"]["_entities"][0]["reviews"][1]["id"] == "rev-002"
        assert "rating" in response_data["data"]["_entities"][0]["reviews"][1]
        assert response_data["data"]["_entities"][0]["reviews"][1]["rating"] == 4
        assert "text" in response_data["data"]["_entities"][0]["reviews"][1]
        assert response_data["data"]["_entities"][0]["reviews"][1]["text"] == "Very helpful"
        assert "author" in response_data["data"]["_entities"][0]["reviews"][1]
        assert "id" in response_data["data"]["_entities"][0]["reviews"][1]["author"]
        assert response_data["data"]["_entities"][0]["reviews"][1]["author"]["id"] == "user-2"
        assert "name" in response_data["data"]["_entities"][0]["reviews"][1]["author"]
        assert response_data["data"]["_entities"][0]["reviews"][1]["author"]["name"] == "Diana Prince"
        response_data = response.json()
        assert response_data.get("errors") is None or response_data.get("errors") == []


@pytest.mark.asyncio
async def test_graphql_external_field() -> None:
    """Reference to @external field used by @requires in another subgraph."""

    app_factory = getattr(app_main, "create_app_graphql_external_field", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_external_field")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query='query {\n  _entities(representations: [{__typename: "Parcel", id: "parcel-x1", weight: 2.5, dimensions: "10x8x6"}]) {\n    ... on Parcel {\n      id\n      weight\n      dimensions\n      label\n    }\n  }\n}',
            variables=None,
            operation_name=None,
            path="/graphql",
        )

        assert response.status_code == 200
        response_data = response.json()
        assert "data" in response_data
        assert "_entities" in response_data["data"]
        assert len(response_data["data"]["_entities"]) == 1
        assert "id" in response_data["data"]["_entities"][0]
        assert response_data["data"]["_entities"][0]["id"] == "parcel-x1"
        assert "weight" in response_data["data"]["_entities"][0]
        assert response_data["data"]["_entities"][0]["weight"] == 2.5
        assert "dimensions" in response_data["data"]["_entities"][0]
        assert response_data["data"]["_entities"][0]["dimensions"] == "10x8x6"
        assert "label" in response_data["data"]["_entities"][0]
        assert response_data["data"]["_entities"][0]["label"] == "SMALL_PACKAGE_2.5KG"
        response_data = response.json()
        assert response_data.get("errors") is None or response_data.get("errors") == []


@pytest.mark.asyncio
async def test_graphql_inaccessible_directive() -> None:
    """Field with @inaccessible directive - internal-only fields hidden from public schema."""

    app_factory = getattr(app_main, "create_app_graphql_inaccessible_directive", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_inaccessible_directive")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query='query {\n  user(id: "user-42") {\n    id\n    name\n    email\n    internalScore\n    publicStatus\n  }\n}',
            variables=None,
            operation_name=None,
            path="/graphql",
        )

        assert response.status_code == 400
        response_data = response.json()
        assert "errors" in response_data
        assert len(response_data["errors"]) == 1
        error_0 = response_data["errors"][0]
        assert (
            error_0["message"]
            == "Cannot query field 'internalScore' on type 'User'. This field is @inaccessible and not available in the public schema."
        )


@pytest.mark.asyncio
async def test_graphql_subgraph_introspection() -> None:
    """Federation _service query returning subgraph Schema Definition Language."""

    app_factory = getattr(app_main, "create_app_graphql_subgraph_introspection", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_subgraph_introspection")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="query {\n  _service {\n    sdl\n  }\n}", variables=None, operation_name=None, path="/graphql"
        )

        assert response.status_code == 200
        response_data = response.json()
        assert "data" in response_data
        assert "_service" in response_data["data"]
        assert "sdl" in response_data["data"]["_service"]
        assert (
            response_data["data"]["_service"]["sdl"]
            == 'type Account @key(fields: "accountId") {\n  accountId: ID!\n  accountName: String!\n  tier: String!\n  createdAt: String!\n}\n\ntype Query {\n  account(accountId: ID!): Account\n}'
        )
        response_data = response.json()
        assert response_data.get("errors") is None or response_data.get("errors") == []


@pytest.mark.asyncio
async def test_graphql_shareable_directive() -> None:
    """Field with @shareable directive - multiple subgraphs can contribute to the same field."""

    app_factory = getattr(app_main, "create_app_graphql_shareable_directive", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_shareable_directive")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query='query {\n  _entities(representations: [{__typename: "Product", id: "prod-001"}]) {\n    ... on Product {\n      id\n      name\n      description\n      category\n      price\n    }\n  }\n}',
            variables=None,
            operation_name=None,
            path="/graphql",
        )

        assert response.status_code == 200
        response_data = response.json()
        assert "data" in response_data
        assert "_entities" in response_data["data"]
        assert len(response_data["data"]["_entities"]) == 1
        assert "id" in response_data["data"]["_entities"][0]
        assert response_data["data"]["_entities"][0]["id"] == "prod-001"
        assert "name" in response_data["data"]["_entities"][0]
        assert response_data["data"]["_entities"][0]["name"] == "Wireless Headphones"
        assert "description" in response_data["data"]["_entities"][0]
        assert (
            response_data["data"]["_entities"][0]["description"]
            == "Premium noise-canceling headphones with 30-hour battery life"
        )
        assert "category" in response_data["data"]["_entities"][0]
        assert response_data["data"]["_entities"][0]["category"] == "Electronics"
        assert "price" in response_data["data"]["_entities"][0]
        assert response_data["data"]["_entities"][0]["price"] == 199.99
        response_data = response.json()
        assert response_data.get("errors") is None or response_data.get("errors") == []


@pytest.mark.asyncio
async def test_graphql_entity_resolution_basic() -> None:
    """Basic _entities query for Apollo Federation entity resolution."""

    app_factory = getattr(app_main, "create_app_graphql_entity_resolution_basic", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_entity_resolution_basic")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query='query {\n  _entities(representations: [{__typename: "User", id: "1"}]) {\n    ... on User {\n      id\n      name\n      email\n    }\n  }\n}',
            variables=None,
            operation_name=None,
            path="/graphql",
        )

        assert response.status_code == 200
        response_data = response.json()
        assert "data" in response_data
        assert "_entities" in response_data["data"]
        assert len(response_data["data"]["_entities"]) == 1
        assert "id" in response_data["data"]["_entities"][0]
        assert response_data["data"]["_entities"][0]["id"] == "1"
        assert "name" in response_data["data"]["_entities"][0]
        assert response_data["data"]["_entities"][0]["name"] == "Alice Johnson"
        assert "email" in response_data["data"]["_entities"][0]
        assert response_data["data"]["_entities"][0]["email"] == "alice@example.com"
        response_data = response.json()
        assert response_data.get("errors") is None or response_data.get("errors") == []


@pytest.mark.asyncio
async def test_graphql_override_directive() -> None:
    """Field with @override directive for progressive field ownership migration between subgraphs."""

    app_factory = getattr(app_main, "create_app_graphql_override_directive", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_override_directive")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query='query {\n  user(id: "user-789") {\n    id\n    username\n    email\n    profile {\n      bio\n      joinDate\n      location\n    }\n  }\n}',
            variables=None,
            operation_name=None,
            path="/graphql",
        )

        assert response.status_code == 200
        response_data = response.json()
        assert "data" in response_data
        assert "user" in response_data["data"]
        assert "id" in response_data["data"]["user"]
        assert response_data["data"]["user"]["id"] == "user-789"
        assert "username" in response_data["data"]["user"]
        assert response_data["data"]["user"]["username"] == "johndoe"
        assert "email" in response_data["data"]["user"]
        assert response_data["data"]["user"]["email"] == "john.doe@example.com"
        assert "profile" in response_data["data"]["user"]
        assert "bio" in response_data["data"]["user"]["profile"]
        assert response_data["data"]["user"]["profile"]["bio"] == "Software developer and tech enthusiast"
        assert "joinDate" in response_data["data"]["user"]["profile"]
        assert response_data["data"]["user"]["profile"]["joinDate"] == "2021-06-12"
        assert "location" in response_data["data"]["user"]["profile"]
        assert response_data["data"]["user"]["profile"]["location"] == "San Francisco, CA"
        response_data = response.json()
        assert response_data.get("errors") is None or response_data.get("errors") == []


@pytest.mark.asyncio
async def test_graphql_federation_type_mismatch() -> None:
    """Wrong __typename in entity representation - returns 400 error."""

    app_factory = getattr(app_main, "create_app_graphql_federation_type_mismatch", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_federation_type_mismatch")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query='query {\n  _entities(representations: [{__typename: "InvalidType", id: "1"}]) {\n    ... on Article {\n      id\n      title\n      content\n      author\n    }\n  }\n}',
            variables=None,
            operation_name=None,
            path="/graphql",
        )

        assert response.status_code == 400
        response_data = response.json()
        assert "errors" in response_data
        assert len(response_data["errors"]) == 1
        error_0 = response_data["errors"][0]
        assert error_0["message"] == "Unknown type 'InvalidType' in entity representation"


@pytest.mark.asyncio
async def test_graphql_entity_with_compound_key() -> None:
    """Entity with compound @key directive spanning multiple fields."""

    app_factory = getattr(app_main, "create_app_graphql_entity_with_compound_key", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_entity_with_compound_key")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query='query {\n  _entities(representations: [{__typename: "Product", sku: "ABC123", category: "electronics"}]) {\n    ... on Product {\n      sku\n      category\n      name\n      description\n      price\n      stock\n    }\n  }\n}',
            variables=None,
            operation_name=None,
            path="/graphql",
        )

        assert response.status_code == 200
        response_data = response.json()
        assert "data" in response_data
        assert "_entities" in response_data["data"]
        assert len(response_data["data"]["_entities"]) == 1
        assert "sku" in response_data["data"]["_entities"][0]
        assert response_data["data"]["_entities"][0]["sku"] == "ABC123"
        assert "category" in response_data["data"]["_entities"][0]
        assert response_data["data"]["_entities"][0]["category"] == "electronics"
        assert "name" in response_data["data"]["_entities"][0]
        assert response_data["data"]["_entities"][0]["name"] == "Wireless Headphones"
        assert "description" in response_data["data"]["_entities"][0]
        assert response_data["data"]["_entities"][0]["description"] == "Premium noise-cancelling wireless headphones"
        assert "price" in response_data["data"]["_entities"][0]
        assert response_data["data"]["_entities"][0]["price"] == 199.99
        assert "stock" in response_data["data"]["_entities"][0]
        assert response_data["data"]["_entities"][0]["stock"] == 45
        response_data = response.json()
        assert response_data.get("errors") is None or response_data.get("errors") == []


@pytest.mark.asyncio
async def test_graphql_federation_error_missing_entity() -> None:
    """Entity not found - returns null in _entities array per Apollo Federation spec."""

    app_factory = getattr(app_main, "create_app_graphql_federation_error_missing_entity", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_federation_error_missing_entity")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query='query {\n  _entities(representations: [{__typename: "Customer", id: "999999"}]) {\n    ... on Customer {\n      id\n      firstName\n      lastName\n      email\n    }\n  }\n}',
            variables=None,
            operation_name=None,
            path="/graphql",
        )

        assert response.status_code == 200
        response_data = response.json()
        assert "data" in response_data
        assert "_entities" in response_data["data"]
        assert len(response_data["data"]["_entities"]) == 1
        assert response_data["data"]["_entities"][0] == None
        response_data = response.json()
        assert response_data.get("errors") is None or response_data.get("errors") == []


@pytest.mark.asyncio
async def test_graphql_field_error() -> None:
    """Query requesting non-existent field with error path and location information."""

    app_factory = getattr(app_main, "create_app_graphql_field_error", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_field_error")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="query GetUser($id: ID!) {\n  user(id: $id) {\n    id\n    name\n    invalidField\n  }\n}",
            variables={"id": "user-123"},
            operation_name=None,
            path="/graphql",
        )

        assert response.status_code == 400
        response_data = response.json()
        assert "errors" in response_data
        assert len(response_data["errors"]) == 1
        error_0 = response_data["errors"][0]
        assert (
            error_0["message"]
            == 'Cannot query field "invalidField" on type "User". Did you mean "id", "name", or "email"?'
        )
        assert error_0["path"] == ["user", "invalidField"]
        assert "locations" in error_0
        assert len(error_0["locations"]) >= 1


@pytest.mark.asyncio
async def test_graphql_syntax_error() -> None:
    """GraphQL document with invalid syntax - unterminated string and missing closing brace."""

    app_factory = getattr(app_main, "create_app_graphql_syntax_error", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_syntax_error")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query='query {\n  user(id: "123\n}', variables=None, operation_name=None, path="/graphql"
        )

        assert response.status_code == 400
        response_data = response.json()
        assert "errors" in response_data
        assert len(response_data["errors"]) == 1
        error_0 = response_data["errors"][0]
        assert error_0["message"] == "Syntax Error in GraphQL query at line 2, column 17: Unterminated string."
        assert "locations" in error_0
        assert len(error_0["locations"]) >= 1


@pytest.mark.asyncio
async def test_graphql_type_error() -> None:
    """Argument type mismatch - passing string instead of required integer ID."""

    app_factory = getattr(app_main, "create_app_graphql_type_error", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_type_error")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="query GetPost($id: ID!) {\n  post(id: $id) {\n    id\n    title\n    content\n  }\n}",
            variables={"id": True},
            operation_name=None,
            path="/graphql",
        )

        assert response.status_code == 400
        response_data = response.json()
        assert "errors" in response_data
        assert len(response_data["errors"]) == 1
        error_0 = response_data["errors"][0]
        assert error_0["message"] == 'Variable "$id" of type "ID!" was provided invalid value.'
        assert "locations" in error_0
        assert len(error_0["locations"]) >= 1


@pytest.mark.asyncio
async def test_graphql_validation_error() -> None:
    """Constraint violation - required input field is missing."""

    app_factory = getattr(app_main, "create_app_graphql_validation_error", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_validation_error")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="mutation CreatePost($input: CreatePostInput!) {\n  createPost(input: $input) {\n    id\n    title\n    content\n    tags\n    createdAt\n  }\n}",
            variables={"input": {"title": "My Post", "content": "This is a post"}},
            operation_name=None,
            path="/graphql",
        )

        assert response.status_code == 400
        response_data = response.json()
        assert "errors" in response_data
        assert len(response_data["errors"]) == 1
        error_0 = response_data["errors"][0]
        assert error_0["message"] == 'Field "CreatePostInput.tags" of required type "[String!]!" was not provided.'
        assert "locations" in error_0
        assert len(error_0["locations"]) >= 1


@pytest.mark.asyncio
async def test_graphql_query_batching() -> None:
    """Batched query execution with multiple queries in a single request, executed in parallel for optimal performance."""

    app_factory = getattr(app_main, "create_app_graphql_query_batching", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_query_batching")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(query="", variables=None, operation_name=None, path="/graphql")

        assert response.status_code == 200
        response_data = response.json()
        assert "data" in response_data
        assert len(response_data["data"]) == 3
        assert "user" in response_data["data"][0]
        assert "id" in response_data["data"][0]["user"]
        assert response_data["data"][0]["user"]["id"] == "user-1"
        assert "name" in response_data["data"][0]["user"]
        assert response_data["data"][0]["user"]["name"] == "Alice Johnson"
        assert "email" in response_data["data"][0]["user"]
        assert response_data["data"][0]["user"]["email"] == "alice@example.com"
        assert "user" in response_data["data"][1]
        assert "id" in response_data["data"][1]["user"]
        assert response_data["data"][1]["user"]["id"] == "user-2"
        assert "name" in response_data["data"][1]["user"]
        assert response_data["data"][1]["user"]["name"] == "Bob Smith"
        assert "email" in response_data["data"][1]["user"]
        assert response_data["data"][1]["user"]["email"] == "bob@example.com"
        assert "post" in response_data["data"][2]
        assert "id" in response_data["data"][2]["post"]
        assert response_data["data"][2]["post"]["id"] == "post-1"
        assert "title" in response_data["data"][2]["post"]
        assert response_data["data"][2]["post"]["title"] == "GraphQL Performance Tips"
        assert "author_id" in response_data["data"][2]["post"]
        assert response_data["data"][2]["post"]["author_id"] == "user-1"
        response_data = response.json()
        assert response_data.get("errors") is None or response_data.get("errors") == []


@pytest.mark.asyncio
async def test_graphql_response_streaming() -> None:
    """Response streaming with @defer and @stream directives for progressive data delivery and improved perceived performance."""

    app_factory = getattr(app_main, "create_app_graphql_response_streaming", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_response_streaming")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query='query GetUserWithDeferred($userId: String!) {\n  user(id: $userId) {\n    id\n    name\n    email\n    ...DeferredPosts @defer(label: "userPosts")\n    ...DeferredFollowers @defer(label: "userFollowers")\n  }\n}\n\nfragment DeferredPosts on User {\n  posts @stream(initialCount: 1, label: "postsStream") {\n    id\n    title\n    published_at\n  }\n}\n\nfragment DeferredFollowers on User {\n  followers @stream(initialCount: 2, label: "followersStream") {\n    id\n    name\n  }\n}',
            variables={"userId": "user-123"},
            operation_name=None,
            path="/graphql",
        )

        assert response.status_code == 200
        response_data = response.json()
        assert response_data.get("errors") is None or response_data.get("errors") == []


@pytest.mark.asyncio
async def test_graphql_field_level_permissions() -> None:
    """Field-level authorization - user can access id and email but not privateData."""

    app_factory = getattr(app_main, "create_app_graphql_field_level_permissions", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_field_level_permissions")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query='query {\n  user(id: "user123") {\n    id\n    email\n    privateData\n  }\n}',
            variables={"userId": "user123"},
            operation_name=None,
            path="/graphql",
        )

        assert response.status_code == 200
        response_data = response.json()
        assert "errors" in response_data
        assert len(response_data["errors"]) == 1
        error_0 = response_data["errors"][0]
        assert error_0["message"] == "Field 'privateData' requires elevated permissions"
        assert error_0["path"] == ["user", "privateData"]


@pytest.mark.asyncio
async def test_graphql_role_admin_allowed() -> None:
    """Admin-only query accessed with admin role - allowed."""

    app_factory = getattr(app_main, "create_app_graphql_role_admin_allowed", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_role_admin_allowed")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="query {\n  adminPanel {\n    stats {\n      totalUsers\n      activeUsers\n      totalRevenue\n    }\n  }\n}",
            variables=None,
            operation_name=None,
            path="/graphql",
        )

        assert response.status_code == 200
        response_data = response.json()
        assert "data" in response_data
        assert "adminPanel" in response_data["data"]
        assert "stats" in response_data["data"]["adminPanel"]
        assert "totalUsers" in response_data["data"]["adminPanel"]["stats"]
        assert response_data["data"]["adminPanel"]["stats"]["totalUsers"] == 1250
        assert "activeUsers" in response_data["data"]["adminPanel"]["stats"]
        assert response_data["data"]["adminPanel"]["stats"]["activeUsers"] == 856
        assert "totalRevenue" in response_data["data"]["adminPanel"]["stats"]
        assert response_data["data"]["adminPanel"]["stats"]["totalRevenue"] == 125000.5
        response_data = response.json()
        assert response_data.get("errors") is None or response_data.get("errors") == []


@pytest.mark.asyncio
async def test_graphql_mutation_permission_check() -> None:
    """Mutation requiring specific permission - user has READ but not DELETE."""

    app_factory = getattr(app_main, "create_app_graphql_mutation_permission_check", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_mutation_permission_check")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="mutation DeleteUser($userId: String!) {\n  deleteUser(id: $userId) {\n    success\n    message\n  }\n}",
            variables={"userId": "user123"},
            operation_name=None,
            path="/graphql",
        )

        assert response.status_code == 403
        response_data = response.json()
        assert "errors" in response_data
        assert len(response_data["errors"]) == 1
        error_0 = response_data["errors"][0]
        assert error_0["message"] == "Missing required permission: DELETE"


@pytest.mark.asyncio
async def test_graphql_dynamic_authorization() -> None:
    """Authorization based on resource state - only post author or admin can approve."""

    app_factory = getattr(app_main, "create_app_graphql_dynamic_authorization", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_dynamic_authorization")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="mutation ApprovePost($postId: String!) {\n  approvePost(id: $postId) {\n    success\n    postId\n    status\n  }\n}",
            variables={"postId": "post123"},
            operation_name=None,
            path="/graphql",
        )

        assert response.status_code == 403
        response_data = response.json()
        assert "errors" in response_data
        assert len(response_data["errors"]) == 1
        error_0 = response_data["errors"][0]
        assert error_0["message"] == "Only post author or admin can approve posts"


@pytest.mark.asyncio
async def test_graphql_resource_owner_allowed() -> None:
    """User accessing their own resource - allowed."""

    app_factory = getattr(app_main, "create_app_graphql_resource_owner_allowed", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_resource_owner_allowed")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="query GetUserProfile($userId: String!) {\n  user(id: $userId) {\n    id\n    profile {\n      bio\n      website\n      joinDate\n    }\n  }\n}",
            variables={"userId": "user123"},
            operation_name=None,
            path="/graphql",
        )

        assert response.status_code == 200
        response_data = response.json()
        assert "data" in response_data
        assert "user" in response_data["data"]
        assert "id" in response_data["data"]["user"]
        assert response_data["data"]["user"]["id"] == "user123"
        assert "profile" in response_data["data"]["user"]
        assert "bio" in response_data["data"]["user"]["profile"]
        assert response_data["data"]["user"]["profile"]["bio"] == "Software engineer from San Francisco"
        assert "website" in response_data["data"]["user"]["profile"]
        assert response_data["data"]["user"]["profile"]["website"] == "https://example.com"
        assert "joinDate" in response_data["data"]["user"]["profile"]
        assert response_data["data"]["user"]["profile"]["joinDate"] == "2020-01-15"
        response_data = response.json()
        assert response_data.get("errors") is None or response_data.get("errors") == []


@pytest.mark.asyncio
async def test_graphql_permission_chain() -> None:
    """Multiple permission checks in nested resolvers - partial data with errors."""

    app_factory = getattr(app_main, "create_app_graphql_permission_chain", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_permission_chain")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="query {\n  dashboard {\n    id\n    publicMetrics {\n      pageViews\n      uniqueVisitors\n    }\n    privateMetrics {\n      pageViews\n      uniqueVisitors\n    }\n    adminSettings {\n      apiKey\n      webhookUrl\n    }\n  }\n}",
            variables=None,
            operation_name=None,
            path="/graphql",
        )

        assert response.status_code == 200
        response_data = response.json()
        assert "errors" in response_data
        assert len(response_data["errors"]) == 2
        error_0 = response_data["errors"][0]
        assert error_0["message"] == "Insufficient permissions to access privateMetrics"
        assert error_0["path"] == ["dashboard", "privateMetrics"]
        error_1 = response_data["errors"][1]
        assert error_1["message"] == "Insufficient permissions to access adminSettings"
        assert error_1["path"] == ["dashboard", "adminSettings"]


@pytest.mark.asyncio
async def test_graphql_resource_owner_denied() -> None:
    """User accessing another user's resource - denied."""

    app_factory = getattr(app_main, "create_app_graphql_resource_owner_denied", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_resource_owner_denied")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="query GetUserProfile($userId: String!) {\n  user(id: $userId) {\n    id\n    profile {\n      bio\n      website\n    }\n  }\n}",
            variables={"userId": "user456"},
            operation_name=None,
            path="/graphql",
        )

        assert response.status_code == 403
        response_data = response.json()
        assert "errors" in response_data
        assert len(response_data["errors"]) == 1
        error_0 = response_data["errors"][0]
        assert error_0["message"] == "Not authorized to access this resource"


@pytest.mark.asyncio
async def test_graphql_role_user_denied() -> None:
    """Admin-only query accessed with user role - denied."""

    app_factory = getattr(app_main, "create_app_graphql_role_user_denied", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_role_user_denied")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="query {\n  adminPanel {\n    stats {\n      totalUsers\n      activeUsers\n    }\n  }\n}",
            variables=None,
            operation_name=None,
            path="/graphql",
        )

        assert response.status_code == 403
        response_data = response.json()
        assert "errors" in response_data
        assert len(response_data["errors"]) == 1
        error_0 = response_data["errors"][0]
        assert error_0["message"] == "Insufficient permissions to access adminPanel"


@pytest.mark.asyncio
async def test_graphql_jwt_valid() -> None:
    """Query with valid JWT token in Authorization header."""

    app_factory = getattr(app_main, "create_app_graphql_jwt_valid", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_jwt_valid")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="query {\n  currentUser {\n    id\n    email\n    name\n  }\n}",
            variables=None,
            operation_name=None,
            path="/graphql",
        )

        assert response.status_code == 200
        response_data = response.json()
        assert "data" in response_data
        assert "currentUser" in response_data["data"]
        assert "id" in response_data["data"]["currentUser"]
        assert response_data["data"]["currentUser"]["id"] == "user123"
        assert "email" in response_data["data"]["currentUser"]
        assert response_data["data"]["currentUser"]["email"] == "john@example.com"
        assert "name" in response_data["data"]["currentUser"]
        assert response_data["data"]["currentUser"]["name"] == "John Doe"
        response_data = response.json()
        assert response_data.get("errors") is None or response_data.get("errors") == []


@pytest.mark.asyncio
async def test_graphql_api_key_invalid() -> None:
    """Query with invalid API key."""

    app_factory = getattr(app_main, "create_app_graphql_api_key_invalid", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_api_key_invalid")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="query {\n  secureData\n}", variables=None, operation_name=None, path="/graphql"
        )

        assert response.status_code == 401
        response_data = response.json()
        assert "errors" in response_data
        assert len(response_data["errors"]) == 1
        error_0 = response_data["errors"][0]
        assert error_0["message"] == "Invalid API key"


@pytest.mark.asyncio
async def test_graphql_jwt_expired() -> None:
    """Query with expired JWT token."""

    app_factory = getattr(app_main, "create_app_graphql_jwt_expired", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_jwt_expired")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="query {\n  currentUser {\n    id\n    email\n  }\n}",
            variables=None,
            operation_name=None,
            path="/graphql",
        )

        assert response.status_code == 401
        response_data = response.json()
        assert "errors" in response_data
        assert len(response_data["errors"]) == 1
        error_0 = response_data["errors"][0]
        assert error_0["message"] == "Token expired"


@pytest.mark.asyncio
async def test_graphql_jwt_invalid_signature() -> None:
    """Query with JWT token having invalid signature."""

    app_factory = getattr(app_main, "create_app_graphql_jwt_invalid_signature", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_jwt_invalid_signature")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="query {\n  currentUser {\n    id\n    email\n  }\n}",
            variables=None,
            operation_name=None,
            path="/graphql",
        )

        assert response.status_code == 401
        response_data = response.json()
        assert "errors" in response_data
        assert len(response_data["errors"]) == 1
        error_0 = response_data["errors"][0]
        assert error_0["message"] == "Invalid token signature"


@pytest.mark.asyncio
async def test_graphql_no_authentication() -> None:
    """Query requiring authentication without any credentials."""

    app_factory = getattr(app_main, "create_app_graphql_no_authentication", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_no_authentication")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="query {\n  protectedQuery\n}", variables=None, operation_name=None, path="/graphql"
        )

        assert response.status_code == 401
        response_data = response.json()
        assert "errors" in response_data
        assert len(response_data["errors"]) == 1
        error_0 = response_data["errors"][0]
        assert error_0["message"] == "Authentication required"


@pytest.mark.asyncio
async def test_graphql_session_cookie_valid() -> None:
    """Query with valid session cookie."""

    app_factory = getattr(app_main, "create_app_graphql_session_cookie_valid", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_session_cookie_valid")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="query {\n  userProfile {\n    id\n    username\n    email\n  }\n}",
            variables=None,
            operation_name=None,
            path="/graphql",
        )

        assert response.status_code == 200
        response_data = response.json()
        assert "data" in response_data
        assert "userProfile" in response_data["data"]
        assert "id" in response_data["data"]["userProfile"]
        assert response_data["data"]["userProfile"]["id"] == "user456"
        assert "username" in response_data["data"]["userProfile"]
        assert response_data["data"]["userProfile"]["username"] == "alice_smith"
        assert "email" in response_data["data"]["userProfile"]
        assert response_data["data"]["userProfile"]["email"] == "alice@example.com"
        response_data = response.json()
        assert response_data.get("errors") is None or response_data.get("errors") == []


@pytest.mark.asyncio
async def test_graphql_multiple_auth_methods() -> None:
    """Query with multiple auth headers present (JWT and API key) - should use JWT."""

    app_factory = getattr(app_main, "create_app_graphql_multiple_auth_methods", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_multiple_auth_methods")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="query {\n  currentUser {\n    id\n    email\n    authMethod\n  }\n}",
            variables=None,
            operation_name=None,
            path="/graphql",
        )

        assert response.status_code == 200
        response_data = response.json()
        assert "data" in response_data
        assert "currentUser" in response_data["data"]
        assert "id" in response_data["data"]["currentUser"]
        assert response_data["data"]["currentUser"]["id"] == "user123"
        assert "email" in response_data["data"]["currentUser"]
        assert response_data["data"]["currentUser"]["email"] == "john@example.com"
        assert "authMethod" in response_data["data"]["currentUser"]
        assert response_data["data"]["currentUser"]["authMethod"] == "jwt"
        response_data = response.json()
        assert response_data.get("errors") is None or response_data.get("errors") == []


@pytest.mark.asyncio
async def test_graphql_api_key_valid() -> None:
    """Query with valid API key in X-API-Key header."""

    app_factory = getattr(app_main, "create_app_graphql_api_key_valid", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_api_key_valid")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="query {\n  secureData\n}", variables=None, operation_name=None, path="/graphql"
        )

        assert response.status_code == 200
        response_data = response.json()
        assert "data" in response_data
        assert "secureData" in response_data["data"]
        assert response_data["data"]["secureData"] == "Protected data from API key authentication"
        response_data = response.json()
        assert response_data.get("errors") is None or response_data.get("errors") == []


@pytest.mark.asyncio
async def test_graphql_required_fields() -> None:
    """Input validation for required fields in mutations and input types."""

    app_factory = getattr(app_main, "create_app_graphql_required_fields", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_required_fields")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="mutation Register($input: UserRegistrationInput!) {\n  registerUser(input: $input) {\n    success\n    userId\n    message\n  }\n}",
            variables={"input": {"username": "johndoe", "email": "john@example.com"}},
            operation_name=None,
            path="/graphql",
        )

        assert response.status_code == 400
        response_data = response.json()
        assert "errors" in response_data
        assert len(response_data["errors"]) == 1
        error_0 = response_data["errors"][0]
        assert (
            error_0["message"] == 'Field "UserRegistrationInput.password" of required type "String!" was not provided.'
        )
        assert "locations" in error_0
        assert len(error_0["locations"]) >= 1


@pytest.mark.asyncio
async def test_graphql_invalid_types() -> None:
    """Input validation for wrong types in arguments and input fields."""

    app_factory = getattr(app_main, "create_app_graphql_invalid_types", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_invalid_types")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="query SearchUsers($limit: Int!, $offset: Int) {\n  searchUsers(limit: $limit, offset: $offset) {\n    id\n    name\n    email\n  }\n}",
            variables={"limit": "not_an_integer", "offset": 10},
            operation_name=None,
            path="/graphql",
        )

        assert response.status_code == 400
        response_data = response.json()
        assert "errors" in response_data
        assert len(response_data["errors"]) == 1
        error_0 = response_data["errors"][0]
        assert error_0["message"] == 'Variable "$limit" of type "Int!" was provided invalid value.'
        assert "locations" in error_0
        assert len(error_0["locations"]) >= 1


@pytest.mark.asyncio
async def test_graphql_file_upload_validation_type() -> None:
    """File upload with invalid file type (expects image, receives text)."""

    app_factory = getattr(app_main, "create_app_graphql_file_upload_validation_type", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_file_upload_validation_type")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="mutation UploadImage($file: Upload!) {\n  uploadImage(file: $file) {\n    id\n    filename\n    mimetype\n    width\n    height\n  }\n}",
            variables={"file": None},
            operation_name="UploadImage",
            path="/graphql",
        )

        assert response.status_code == 400
        response_data = response.json()
        assert "errors" in response_data
        assert len(response_data["errors"]) == 1
        error_0 = response_data["errors"][0]
        assert error_0["message"] == "Invalid file type"


@pytest.mark.asyncio
async def test_graphql_multiple_files_upload() -> None:
    """Upload multiple files in a single GraphQL multipart request."""

    app_factory = getattr(app_main, "create_app_graphql_multiple_files_upload", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_multiple_files_upload")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="mutation MultipleUpload($files: [Upload!]!) {\n  multipleUpload(files: $files) {\n    id\n    filename\n    mimetype\n    size\n  }\n}",
            variables={"files": [None, None, None]},
            operation_name="MultipleUpload",
            path="/graphql",
        )

        assert response.status_code == 200
        response_data = response.json()
        assert "data" in response_data
        assert "multipleUpload" in response_data["data"]
        assert len(response_data["data"]["multipleUpload"]) == 3
        assert "id" in response_data["data"]["multipleUpload"][0]
        assert response_data["data"]["multipleUpload"][0]["id"] == "file-002"
        assert "filename" in response_data["data"]["multipleUpload"][0]
        assert response_data["data"]["multipleUpload"][0]["filename"] == "document.pdf"
        assert "mimetype" in response_data["data"]["multipleUpload"][0]
        assert response_data["data"]["multipleUpload"][0]["mimetype"] == "application/pdf"
        assert "size" in response_data["data"]["multipleUpload"][0]
        assert response_data["data"]["multipleUpload"][0]["size"] == 32
        assert "id" in response_data["data"]["multipleUpload"][1]
        assert response_data["data"]["multipleUpload"][1]["id"] == "file-003"
        assert "filename" in response_data["data"]["multipleUpload"][1]
        assert response_data["data"]["multipleUpload"][1]["filename"] == "image.png"
        assert "mimetype" in response_data["data"]["multipleUpload"][1]
        assert response_data["data"]["multipleUpload"][1]["mimetype"] == "image/png"
        assert "size" in response_data["data"]["multipleUpload"][1]
        assert response_data["data"]["multipleUpload"][1]["size"] == 24
        assert "id" in response_data["data"]["multipleUpload"][2]
        assert response_data["data"]["multipleUpload"][2]["id"] == "file-004"
        assert "filename" in response_data["data"]["multipleUpload"][2]
        assert response_data["data"]["multipleUpload"][2]["filename"] == "data.csv"
        assert "mimetype" in response_data["data"]["multipleUpload"][2]
        assert response_data["data"]["multipleUpload"][2]["mimetype"] == "text/csv"
        assert "size" in response_data["data"]["multipleUpload"][2]
        assert response_data["data"]["multipleUpload"][2]["size"] == 68
        response_data = response.json()
        assert response_data.get("errors") is None or response_data.get("errors") == []


@pytest.mark.asyncio
async def test_graphql_file_upload_multipart_spec() -> None:
    """GraphQL multipart request spec compliance test (RFC 2388, graphql-multipart-request-spec)."""

    app_factory = getattr(app_main, "create_app_graphql_file_upload_multipart_spec", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_file_upload_multipart_spec")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="mutation UploadDocument($title: String!, $files: [Upload!]!) {\n  uploadDocument(title: $title, files: $files) {\n    id\n    title\n    files {\n      id\n      filename\n      mimetype\n      size\n    }\n    uploadedAt\n  }\n}",
            variables={"title": "Important Documents", "files": [None, None]},
            operation_name="UploadDocument",
            path="/graphql",
        )

        assert response.status_code == 200
        response_data = response.json()
        assert "data" in response_data
        assert "uploadDocument" in response_data["data"]
        assert "id" in response_data["data"]["uploadDocument"]
        assert response_data["data"]["uploadDocument"]["id"] == "doc-001"
        assert "title" in response_data["data"]["uploadDocument"]
        assert response_data["data"]["uploadDocument"]["title"] == "Important Documents"
        assert "files" in response_data["data"]["uploadDocument"]
        assert len(response_data["data"]["uploadDocument"]["files"]) == 2
        assert "id" in response_data["data"]["uploadDocument"]["files"][0]
        assert response_data["data"]["uploadDocument"]["files"][0]["id"] == "file-006"
        assert "filename" in response_data["data"]["uploadDocument"]["files"][0]
        assert response_data["data"]["uploadDocument"]["files"][0]["filename"] == "contract.pdf"
        assert "mimetype" in response_data["data"]["uploadDocument"]["files"][0]
        assert response_data["data"]["uploadDocument"]["files"][0]["mimetype"] == "application/pdf"
        assert "size" in response_data["data"]["uploadDocument"]["files"][0]
        assert response_data["data"]["uploadDocument"]["files"][0]["size"] == 88
        assert "id" in response_data["data"]["uploadDocument"]["files"][1]
        assert response_data["data"]["uploadDocument"]["files"][1]["id"] == "file-007"
        assert "filename" in response_data["data"]["uploadDocument"]["files"][1]
        assert response_data["data"]["uploadDocument"]["files"][1]["filename"] == "summary.txt"
        assert "mimetype" in response_data["data"]["uploadDocument"]["files"][1]
        assert response_data["data"]["uploadDocument"]["files"][1]["mimetype"] == "text/plain"
        assert "size" in response_data["data"]["uploadDocument"]["files"][1]
        assert response_data["data"]["uploadDocument"]["files"][1]["size"] == 65
        assert "uploadedAt" in response_data["data"]["uploadDocument"]
        assert response_data["data"]["uploadDocument"]["uploadedAt"] == "2025-12-27T14:30:00Z"
        response_data = response.json()
        assert response_data.get("errors") is None or response_data.get("errors") == []


@pytest.mark.asyncio
async def test_graphql_file_upload_validation_size() -> None:
    """File upload that exceeds maximum file size limit."""

    app_factory = getattr(app_main, "create_app_graphql_file_upload_validation_size", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_file_upload_validation_size")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="mutation Upload($file: Upload!) {\n  singleUpload(file: $file) {\n    id\n    filename\n    mimetype\n    size\n  }\n}",
            variables={"file": None},
            operation_name="Upload",
            path="/graphql",
        )

        assert response.status_code == 400
        response_data = response.json()
        assert "errors" in response_data
        assert len(response_data["errors"]) == 1
        error_0 = response_data["errors"][0]
        assert error_0["message"] == "File too large"


@pytest.mark.asyncio
async def test_graphql_single_file_upload() -> None:
    """Upload a single file via GraphQL multipart request."""

    app_factory = getattr(app_main, "create_app_graphql_single_file_upload", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_single_file_upload")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="mutation Upload($file: Upload!) {\n  singleUpload(file: $file) {\n    id\n    filename\n    mimetype\n    size\n  }\n}",
            variables={"file": None},
            operation_name="Upload",
            path="/graphql",
        )

        assert response.status_code == 200
        response_data = response.json()
        assert "data" in response_data
        assert "singleUpload" in response_data["data"]
        assert "id" in response_data["data"]["singleUpload"]
        assert response_data["data"]["singleUpload"]["id"] == "file-001"
        assert "filename" in response_data["data"]["singleUpload"]
        assert response_data["data"]["singleUpload"]["filename"] == "test.txt"
        assert "mimetype" in response_data["data"]["singleUpload"]
        assert response_data["data"]["singleUpload"]["mimetype"] == "text/plain"
        assert "size" in response_data["data"]["singleUpload"]
        assert response_data["data"]["singleUpload"]["size"] == 39
        response_data = response.json()
        assert response_data.get("errors") is None or response_data.get("errors") == []


@pytest.mark.asyncio
async def test_graphql_file_upload_with_variables() -> None:
    """Upload file with additional scalar variables in the same request."""

    app_factory = getattr(app_main, "create_app_graphql_file_upload_with_variables", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_file_upload_with_variables")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="mutation UploadProfile($userId: ID!, $file: Upload!) {\n  uploadProfilePicture(userId: $userId, file: $file) {\n    id\n    name\n    email\n    profilePicture {\n      id\n      filename\n      mimetype\n      size\n    }\n  }\n}",
            variables={"userId": "user-123", "file": None},
            operation_name="UploadProfile",
            path="/graphql",
        )

        assert response.status_code == 200
        response_data = response.json()
        assert "data" in response_data
        assert "uploadProfilePicture" in response_data["data"]
        assert "id" in response_data["data"]["uploadProfilePicture"]
        assert response_data["data"]["uploadProfilePicture"]["id"] == "user-123"
        assert "name" in response_data["data"]["uploadProfilePicture"]
        assert response_data["data"]["uploadProfilePicture"]["name"] == "John Doe"
        assert "email" in response_data["data"]["uploadProfilePicture"]
        assert response_data["data"]["uploadProfilePicture"]["email"] == "john@example.com"
        assert "profilePicture" in response_data["data"]["uploadProfilePicture"]
        assert "id" in response_data["data"]["uploadProfilePicture"]["profilePicture"]
        assert response_data["data"]["uploadProfilePicture"]["profilePicture"]["id"] == "file-005"
        assert "filename" in response_data["data"]["uploadProfilePicture"]["profilePicture"]
        assert response_data["data"]["uploadProfilePicture"]["profilePicture"]["filename"] == "profile.jpg"
        assert "mimetype" in response_data["data"]["uploadProfilePicture"]["profilePicture"]
        assert response_data["data"]["uploadProfilePicture"]["profilePicture"]["mimetype"] == "image/jpeg"
        assert "size" in response_data["data"]["uploadProfilePicture"]["profilePicture"]
        assert response_data["data"]["uploadProfilePicture"]["profilePicture"]["size"] == 24568
        response_data = response.json()
        assert response_data.get("errors") is None or response_data.get("errors") == []


@pytest.mark.asyncio
async def test_graphql_dataloader_cache_hit() -> None:
    """DataLoader cache hits when same entity is requested multiple times in single request."""

    app_factory = getattr(app_main, "create_app_graphql_dataloader_cache_hit", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_dataloader_cache_hit")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query='query {\n  user1: user(id: "1") {\n    id\n    name\n    email\n  }\n  user2: user(id: "1") {\n    id\n    name\n    username\n  }\n  user3: user(id: "2") {\n    id\n    name\n    email\n  }\n}',
            variables=None,
            operation_name=None,
            path="/graphql",
        )

        assert response.status_code == 200
        response_data = response.json()
        assert "data" in response_data
        assert "user1" in response_data["data"]
        assert "id" in response_data["data"]["user1"]
        assert response_data["data"]["user1"]["id"] == "1"
        assert "name" in response_data["data"]["user1"]
        assert response_data["data"]["user1"]["name"] == "Alice Smith"
        assert "email" in response_data["data"]["user1"]
        assert response_data["data"]["user1"]["email"] == "alice@example.com"
        assert "user2" in response_data["data"]
        assert "id" in response_data["data"]["user2"]
        assert response_data["data"]["user2"]["id"] == "1"
        assert "name" in response_data["data"]["user2"]
        assert response_data["data"]["user2"]["name"] == "Alice Smith"
        assert "username" in response_data["data"]["user2"]
        assert response_data["data"]["user2"]["username"] == "alice_smith"
        assert "user3" in response_data["data"]
        assert "id" in response_data["data"]["user3"]
        assert response_data["data"]["user3"]["id"] == "2"
        assert "name" in response_data["data"]["user3"]
        assert response_data["data"]["user3"]["name"] == "Bob Johnson"
        assert "email" in response_data["data"]["user3"]
        assert response_data["data"]["user3"]["email"] == "bob@example.com"
        response_data = response.json()
        assert response_data.get("errors") is None or response_data.get("errors") == []


@pytest.mark.asyncio
async def test_graphql_dataloader_with_variables() -> None:
    """DataLoader batch loading with GraphQL query variables and parameterized IDs."""

    app_factory = getattr(app_main, "create_app_graphql_dataloader_with_variables", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_dataloader_with_variables")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="query GetPosts($ids: [ID!]!) {\n  posts(ids: $ids) {\n    id\n    title\n    slug\n    publishedAt\n    tags\n  }\n}",
            variables={"ids": ["1", "2", "3"]},
            operation_name=None,
            path="/graphql",
        )

        assert response.status_code == 200
        response_data = response.json()
        assert "data" in response_data
        assert "posts" in response_data["data"]
        assert len(response_data["data"]["posts"]) == 3
        assert "id" in response_data["data"]["posts"][0]
        assert response_data["data"]["posts"][0]["id"] == "1"
        assert "title" in response_data["data"]["posts"][0]
        assert response_data["data"]["posts"][0]["title"] == "Getting Started with GraphQL"
        assert "slug" in response_data["data"]["posts"][0]
        assert response_data["data"]["posts"][0]["slug"] == "getting-started-graphql"
        assert "publishedAt" in response_data["data"]["posts"][0]
        assert response_data["data"]["posts"][0]["publishedAt"] == "2025-01-10T08:00:00Z"
        assert "tags" in response_data["data"]["posts"][0]
        assert len(response_data["data"]["posts"][0]["tags"]) == 2
        assert response_data["data"]["posts"][0]["tags"][0] == "graphql"
        assert response_data["data"]["posts"][0]["tags"][1] == "tutorial"
        assert "id" in response_data["data"]["posts"][1]
        assert response_data["data"]["posts"][1]["id"] == "2"
        assert "title" in response_data["data"]["posts"][1]
        assert response_data["data"]["posts"][1]["title"] == "Mastering DataLoader"
        assert "slug" in response_data["data"]["posts"][1]
        assert response_data["data"]["posts"][1]["slug"] == "mastering-dataloader"
        assert "publishedAt" in response_data["data"]["posts"][1]
        assert response_data["data"]["posts"][1]["publishedAt"] == "2025-01-15T10:30:00Z"
        assert "tags" in response_data["data"]["posts"][1]
        assert len(response_data["data"]["posts"][1]["tags"]) == 3
        assert response_data["data"]["posts"][1]["tags"][0] == "dataloader"
        assert response_data["data"]["posts"][1]["tags"][1] == "performance"
        assert response_data["data"]["posts"][1]["tags"][2] == "optimization"
        assert "id" in response_data["data"]["posts"][2]
        assert response_data["data"]["posts"][2]["id"] == "3"
        assert "title" in response_data["data"]["posts"][2]
        assert response_data["data"]["posts"][2]["title"] == "GraphQL Best Practices"
        assert "slug" in response_data["data"]["posts"][2]
        assert response_data["data"]["posts"][2]["slug"] == "graphql-best-practices"
        assert "publishedAt" in response_data["data"]["posts"][2]
        assert response_data["data"]["posts"][2]["publishedAt"] == "2025-01-20T14:45:00Z"
        assert "tags" in response_data["data"]["posts"][2]
        assert len(response_data["data"]["posts"][2]["tags"]) == 3
        assert response_data["data"]["posts"][2]["tags"][0] == "graphql"
        assert response_data["data"]["posts"][2]["tags"][1] == "best-practices"
        assert response_data["data"]["posts"][2]["tags"][2] == "patterns"
        response_data = response.json()
        assert response_data.get("errors") is None or response_data.get("errors") == []


@pytest.mark.asyncio
async def test_graphql_dataloader_batch_users() -> None:
    """DataLoader batch loading multiple users in a single database call."""

    app_factory = getattr(app_main, "create_app_graphql_dataloader_batch_users", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_dataloader_batch_users")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="query GetUsers($ids: [ID!]!) {\n  users(ids: $ids) {\n    id\n    name\n    email\n    age\n  }\n}",
            variables={"ids": ["1", "2", "3"]},
            operation_name=None,
            path="/graphql",
        )

        assert response.status_code == 200
        response_data = response.json()
        assert "data" in response_data
        assert "users" in response_data["data"]
        assert len(response_data["data"]["users"]) == 3
        assert "id" in response_data["data"]["users"][0]
        assert response_data["data"]["users"][0]["id"] == "1"
        assert "name" in response_data["data"]["users"][0]
        assert response_data["data"]["users"][0]["name"] == "Alice Johnson"
        assert "email" in response_data["data"]["users"][0]
        assert response_data["data"]["users"][0]["email"] == "alice@example.com"
        assert "age" in response_data["data"]["users"][0]
        assert response_data["data"]["users"][0]["age"] == 28
        assert "id" in response_data["data"]["users"][1]
        assert response_data["data"]["users"][1]["id"] == "2"
        assert "name" in response_data["data"]["users"][1]
        assert response_data["data"]["users"][1]["name"] == "Bob Smith"
        assert "email" in response_data["data"]["users"][1]
        assert response_data["data"]["users"][1]["email"] == "bob@example.com"
        assert "age" in response_data["data"]["users"][1]
        assert response_data["data"]["users"][1]["age"] == 34
        assert "id" in response_data["data"]["users"][2]
        assert response_data["data"]["users"][2]["id"] == "3"
        assert "name" in response_data["data"]["users"][2]
        assert response_data["data"]["users"][2]["name"] == "Carol Davis"
        assert "email" in response_data["data"]["users"][2]
        assert response_data["data"]["users"][2]["email"] == "carol@example.com"
        assert "age" in response_data["data"]["users"][2]
        assert response_data["data"]["users"][2]["age"] == 26
        response_data = response.json()
        assert response_data.get("errors") is None or response_data.get("errors") == []


@pytest.mark.asyncio
async def test_graphql_dataloader_error_handling() -> None:
    """DataLoader handling partial errors in batch loads where some items don't exist."""

    app_factory = getattr(app_main, "create_app_graphql_dataloader_error_handling", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_dataloader_error_handling")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="query GetUsers($ids: [ID!]!) {\n  users(ids: $ids) {\n    id\n    name\n    email\n  }\n}",
            variables={"ids": ["1", "999", "2"]},
            operation_name=None,
            path="/graphql",
        )

        assert response.status_code == 200
        response_data = response.json()
        assert "data" in response_data
        assert "users" in response_data["data"]
        assert len(response_data["data"]["users"]) == 3
        assert "id" in response_data["data"]["users"][0]
        assert response_data["data"]["users"][0]["id"] == "1"
        assert "name" in response_data["data"]["users"][0]
        assert response_data["data"]["users"][0]["name"] == "Alice Johnson"
        assert "email" in response_data["data"]["users"][0]
        assert response_data["data"]["users"][0]["email"] == "alice@example.com"
        assert response_data["data"]["users"][1] == None
        assert "id" in response_data["data"]["users"][2]
        assert response_data["data"]["users"][2]["id"] == "2"
        assert "name" in response_data["data"]["users"][2]
        assert response_data["data"]["users"][2]["name"] == "Bob Smith"
        assert "email" in response_data["data"]["users"][2]
        assert response_data["data"]["users"][2]["email"] == "bob@example.com"
        response_data = response.json()
        assert "errors" in response_data
        assert len(response_data["errors"]) == 1
        error_0 = response_data["errors"][0]
        assert error_0["message"] == "User not found with id '999'"
        assert error_0["path"] == ["users", 1]


@pytest.mark.asyncio
async def test_graphql_dataloader_custom_key() -> None:
    """DataLoader using custom cache key (slug) instead of traditional ID for lookup."""

    app_factory = getattr(app_main, "create_app_graphql_dataloader_custom_key", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_dataloader_custom_key")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="query GetProduct($slug: String!) {\n  productBySlug(slug: $slug) {\n    id\n    name\n    slug\n    price\n    category\n    description\n  }\n}",
            variables={"slug": "laptop-pro-2025"},
            operation_name=None,
            path="/graphql",
        )

        assert response.status_code == 200
        response_data = response.json()
        assert "data" in response_data
        assert "productBySlug" in response_data["data"]
        assert "id" in response_data["data"]["productBySlug"]
        assert response_data["data"]["productBySlug"]["id"] == "prod-1"
        assert "name" in response_data["data"]["productBySlug"]
        assert response_data["data"]["productBySlug"]["name"] == "Professional Laptop"
        assert "slug" in response_data["data"]["productBySlug"]
        assert response_data["data"]["productBySlug"]["slug"] == "laptop-pro-2025"
        assert "price" in response_data["data"]["productBySlug"]
        assert response_data["data"]["productBySlug"]["price"] == 1299.99
        assert "category" in response_data["data"]["productBySlug"]
        assert response_data["data"]["productBySlug"]["category"] == "electronics"
        assert "description" in response_data["data"]["productBySlug"]
        assert response_data["data"]["productBySlug"]["description"] == "High-performance laptop for professionals"
        response_data = response.json()
        assert response_data.get("errors") is None or response_data.get("errors") == []


@pytest.mark.asyncio
async def test_graphql_dataloader_nested_batching() -> None:
    """Multi-level DataLoader batching with three nested queries optimized independently."""

    app_factory = getattr(app_main, "create_app_graphql_dataloader_nested_batching", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_dataloader_nested_batching")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="query {\n  posts {\n    id\n    title\n    comments {\n      id\n      text\n      author {\n        id\n        name\n        email\n      }\n    }\n  }\n}",
            variables=None,
            operation_name=None,
            path="/graphql",
        )

        assert response.status_code == 200
        response_data = response.json()
        assert "data" in response_data
        assert "posts" in response_data["data"]
        assert len(response_data["data"]["posts"]) == 2
        assert "id" in response_data["data"]["posts"][0]
        assert response_data["data"]["posts"][0]["id"] == "post-1"
        assert "title" in response_data["data"]["posts"][0]
        assert response_data["data"]["posts"][0]["title"] == "GraphQL Introduction"
        assert "comments" in response_data["data"]["posts"][0]
        assert len(response_data["data"]["posts"][0]["comments"]) == 2
        assert "id" in response_data["data"]["posts"][0]["comments"][0]
        assert response_data["data"]["posts"][0]["comments"][0]["id"] == "comment-1"
        assert "text" in response_data["data"]["posts"][0]["comments"][0]
        assert response_data["data"]["posts"][0]["comments"][0]["text"] == "Great article!"
        assert "author" in response_data["data"]["posts"][0]["comments"][0]
        assert "id" in response_data["data"]["posts"][0]["comments"][0]["author"]
        assert response_data["data"]["posts"][0]["comments"][0]["author"]["id"] == "user-1"
        assert "name" in response_data["data"]["posts"][0]["comments"][0]["author"]
        assert response_data["data"]["posts"][0]["comments"][0]["author"]["name"] == "Alice Johnson"
        assert "email" in response_data["data"]["posts"][0]["comments"][0]["author"]
        assert response_data["data"]["posts"][0]["comments"][0]["author"]["email"] == "alice@example.com"
        assert "id" in response_data["data"]["posts"][0]["comments"][1]
        assert response_data["data"]["posts"][0]["comments"][1]["id"] == "comment-2"
        assert "text" in response_data["data"]["posts"][0]["comments"][1]
        assert response_data["data"]["posts"][0]["comments"][1]["text"] == "Very helpful"
        assert "author" in response_data["data"]["posts"][0]["comments"][1]
        assert "id" in response_data["data"]["posts"][0]["comments"][1]["author"]
        assert response_data["data"]["posts"][0]["comments"][1]["author"]["id"] == "user-2"
        assert "name" in response_data["data"]["posts"][0]["comments"][1]["author"]
        assert response_data["data"]["posts"][0]["comments"][1]["author"]["name"] == "Bob Smith"
        assert "email" in response_data["data"]["posts"][0]["comments"][1]["author"]
        assert response_data["data"]["posts"][0]["comments"][1]["author"]["email"] == "bob@example.com"
        assert "id" in response_data["data"]["posts"][1]
        assert response_data["data"]["posts"][1]["id"] == "post-2"
        assert "title" in response_data["data"]["posts"][1]
        assert response_data["data"]["posts"][1]["title"] == "Advanced Patterns"
        assert "comments" in response_data["data"]["posts"][1]
        assert len(response_data["data"]["posts"][1]["comments"]) == 1
        assert "id" in response_data["data"]["posts"][1]["comments"][0]
        assert response_data["data"]["posts"][1]["comments"][0]["id"] == "comment-3"
        assert "text" in response_data["data"]["posts"][1]["comments"][0]
        assert response_data["data"]["posts"][1]["comments"][0]["text"] == "Excellent explanation"
        assert "author" in response_data["data"]["posts"][1]["comments"][0]
        assert "id" in response_data["data"]["posts"][1]["comments"][0]["author"]
        assert response_data["data"]["posts"][1]["comments"][0]["author"]["id"] == "user-1"
        assert "name" in response_data["data"]["posts"][1]["comments"][0]["author"]
        assert response_data["data"]["posts"][1]["comments"][0]["author"]["name"] == "Alice Johnson"
        assert "email" in response_data["data"]["posts"][1]["comments"][0]["author"]
        assert response_data["data"]["posts"][1]["comments"][0]["author"]["email"] == "alice@example.com"
        response_data = response.json()
        assert response_data.get("errors") is None or response_data.get("errors") == []


@pytest.mark.asyncio
async def test_graphql_dataloader_priming() -> None:
    """DataLoader cache priming where initial batch load primes cache for subsequent individual lookups."""

    app_factory = getattr(app_main, "create_app_graphql_dataloader_priming", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_dataloader_priming")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="query {\n  userList {\n    id\n    name\n    email\n    role\n  }\n}",
            variables=None,
            operation_name=None,
            path="/graphql",
        )

        assert response.status_code == 200
        response_data = response.json()
        assert "data" in response_data
        assert "userList" in response_data["data"]
        assert len(response_data["data"]["userList"]) == 3
        assert "id" in response_data["data"]["userList"][0]
        assert response_data["data"]["userList"][0]["id"] == "user-1"
        assert "name" in response_data["data"]["userList"][0]
        assert response_data["data"]["userList"][0]["name"] == "Alice Johnson"
        assert "email" in response_data["data"]["userList"][0]
        assert response_data["data"]["userList"][0]["email"] == "alice@example.com"
        assert "role" in response_data["data"]["userList"][0]
        assert response_data["data"]["userList"][0]["role"] == "admin"
        assert "id" in response_data["data"]["userList"][1]
        assert response_data["data"]["userList"][1]["id"] == "user-2"
        assert "name" in response_data["data"]["userList"][1]
        assert response_data["data"]["userList"][1]["name"] == "Bob Smith"
        assert "email" in response_data["data"]["userList"][1]
        assert response_data["data"]["userList"][1]["email"] == "bob@example.com"
        assert "role" in response_data["data"]["userList"][1]
        assert response_data["data"]["userList"][1]["role"] == "user"
        assert "id" in response_data["data"]["userList"][2]
        assert response_data["data"]["userList"][2]["id"] == "user-3"
        assert "name" in response_data["data"]["userList"][2]
        assert response_data["data"]["userList"][2]["name"] == "Carol Davis"
        assert "email" in response_data["data"]["userList"][2]
        assert response_data["data"]["userList"][2]["email"] == "carol@example.com"
        assert "role" in response_data["data"]["userList"][2]
        assert response_data["data"]["userList"][2]["role"] == "moderator"
        response_data = response.json()
        assert response_data.get("errors") is None or response_data.get("errors") == []


@pytest.mark.asyncio
async def test_graphql_dataloader_n_plus_one_prevention() -> None:
    """DataLoader preventing N+1 query problem by batching nested author loads."""

    app_factory = getattr(app_main, "create_app_graphql_dataloader_n_plus_one_prevention", None)
    if app_factory is None:
        pytest.skip("Missing generated app factory: create_app_graphql_dataloader_n_plus_one_prevention")
    async with TestClient(app_factory()) as client:
        response = await client.graphql(
            query="query {\n  posts {\n    id\n    title\n    content\n    author {\n      id\n      name\n      email\n    }\n  }\n}",
            variables=None,
            operation_name=None,
            path="/graphql",
        )

        assert response.status_code == 200
        response_data = response.json()
        assert "data" in response_data
        assert "posts" in response_data["data"]
        assert len(response_data["data"]["posts"]) == 3
        assert "id" in response_data["data"]["posts"][0]
        assert response_data["data"]["posts"][0]["id"] == "post-1"
        assert "title" in response_data["data"]["posts"][0]
        assert response_data["data"]["posts"][0]["title"] == "GraphQL Basics"
        assert "content" in response_data["data"]["posts"][0]
        assert response_data["data"]["posts"][0]["content"] == "Introduction to GraphQL..."
        assert "author" in response_data["data"]["posts"][0]
        assert "id" in response_data["data"]["posts"][0]["author"]
        assert response_data["data"]["posts"][0]["author"]["id"] == "user-1"
        assert "name" in response_data["data"]["posts"][0]["author"]
        assert response_data["data"]["posts"][0]["author"]["name"] == "Alice Johnson"
        assert "email" in response_data["data"]["posts"][0]["author"]
        assert response_data["data"]["posts"][0]["author"]["email"] == "alice@example.com"
        assert "id" in response_data["data"]["posts"][1]
        assert response_data["data"]["posts"][1]["id"] == "post-2"
        assert "title" in response_data["data"]["posts"][1]
        assert response_data["data"]["posts"][1]["title"] == "DataLoader Patterns"
        assert "content" in response_data["data"]["posts"][1]
        assert response_data["data"]["posts"][1]["content"] == "Optimizing GraphQL queries..."
        assert "author" in response_data["data"]["posts"][1]
        assert "id" in response_data["data"]["posts"][1]["author"]
        assert response_data["data"]["posts"][1]["author"]["id"] == "user-2"
        assert "name" in response_data["data"]["posts"][1]["author"]
        assert response_data["data"]["posts"][1]["author"]["name"] == "Bob Smith"
        assert "email" in response_data["data"]["posts"][1]["author"]
        assert response_data["data"]["posts"][1]["author"]["email"] == "bob@example.com"
        assert "id" in response_data["data"]["posts"][2]
        assert response_data["data"]["posts"][2]["id"] == "post-3"
        assert "title" in response_data["data"]["posts"][2]
        assert response_data["data"]["posts"][2]["title"] == "Advanced GraphQL"
        assert "content" in response_data["data"]["posts"][2]
        assert response_data["data"]["posts"][2]["content"] == "Custom directives and more..."
        assert "author" in response_data["data"]["posts"][2]
        assert "id" in response_data["data"]["posts"][2]["author"]
        assert response_data["data"]["posts"][2]["author"]["id"] == "user-1"
        assert "name" in response_data["data"]["posts"][2]["author"]
        assert response_data["data"]["posts"][2]["author"]["name"] == "Alice Johnson"
        assert "email" in response_data["data"]["posts"][2]["author"]
        assert response_data["data"]["posts"][2]["author"]["email"] == "alice@example.com"
        response_data = response.json()
        assert response_data.get("errors") is None or response_data.get("errors") == []
