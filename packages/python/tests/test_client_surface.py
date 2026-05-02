"""Smoke tests: verify TestClient surface is importable and has expected methods."""


def test_test_client_is_importable() -> None:
    """TestClient class is accessible from the spikard binding."""
    import _spikard

    assert hasattr(_spikard, "TestClient"), "TestClient must be exported from _spikard"


def test_test_client_has_http_methods() -> None:
    """TestClient exposes the expected HTTP request methods."""
    import _spikard

    client_cls = _spikard.TestClient
    expected_methods = ["get", "post", "put", "patch", "delete", "options", "head"]
    for method_name in expected_methods:
        assert hasattr(client_cls, method_name), f"TestClient missing method: {method_name}"


def test_test_client_has_graphql_methods() -> None:
    """TestClient exposes GraphQL query/subscription helpers."""
    import _spikard

    client_cls = _spikard.TestClient
    for method_name in ("graphql", "graphql_at", "graphql_subscription", "graphql_subscription_at"):
        assert hasattr(client_cls, method_name), f"TestClient missing method: {method_name}"


def test_response_snapshot_is_importable() -> None:
    """ResponseSnapshot class is accessible from the spikard binding."""
    import _spikard

    assert hasattr(_spikard, "ResponseSnapshot"), "ResponseSnapshot must be exported"
    snap_cls = _spikard.ResponseSnapshot
    for attr in ("status", "text", "json", "graphql_data", "graphql_errors"):
        assert hasattr(snap_cls, attr), f"ResponseSnapshot missing attribute: {attr}"


def test_snapshot_error_is_importable() -> None:
    """SnapshotError is accessible from the spikard binding."""
    import _spikard

    assert hasattr(_spikard, "SnapshotError"), "SnapshotError must be exported"


def test_graphql_subscription_snapshot_is_importable() -> None:
    """GraphQLSubscriptionSnapshot is accessible from the spikard binding."""
    import _spikard

    assert hasattr(_spikard, "GraphQLSubscriptionSnapshot"), "GraphQLSubscriptionSnapshot must be exported"


def test_websocket_message_enum_is_importable() -> None:
    """WebSocketMessage enum variants are accessible from the spikard binding."""
    import _spikard

    assert hasattr(_spikard, "WebSocketMessage"), "WebSocketMessage must be exported"
