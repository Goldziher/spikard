"""Test that protobuf fixtures load correctly."""


def test_protobuf_server_fixtures_load(protobuf_server_fixtures: list[dict[str, object]]) -> None:
    """Verify server streaming fixtures load correctly."""
    assert isinstance(protobuf_server_fixtures, list)
    assert len(protobuf_server_fixtures) == 10
    for fixture in protobuf_server_fixtures:
        assert "name" in fixture
        assert "description" in fixture
        assert "category" in fixture
        assert fixture["category"] == "server_streaming"


def test_protobuf_client_fixtures_load(protobuf_client_fixtures: list[dict[str, object]]) -> None:
    """Verify client streaming fixtures load correctly."""
    assert isinstance(protobuf_client_fixtures, list)
    assert len(protobuf_client_fixtures) == 10
    for fixture in protobuf_client_fixtures:
        assert "name" in fixture
        assert "description" in fixture
        assert "category" in fixture
        assert fixture["category"] == "client_streaming"


def test_protobuf_bidirectional_fixtures_load(protobuf_bidirectional_fixtures: list[dict[str, object]]) -> None:
    """Verify bidirectional streaming fixtures load correctly."""
    assert isinstance(protobuf_bidirectional_fixtures, list)
    assert len(protobuf_bidirectional_fixtures) == 10
    for fixture in protobuf_bidirectional_fixtures:
        assert "name" in fixture
        assert "description" in fixture
        assert "category" in fixture
        assert fixture["category"] == "bidirectional_streaming"


def test_protobuf_error_fixtures_load(protobuf_error_fixtures: list[dict[str, object]]) -> None:
    """Verify error handling fixtures load correctly."""
    assert isinstance(protobuf_error_fixtures, list)
    # Error fixtures are now part of the streaming suite
    assert len(protobuf_error_fixtures) == 8


def test_protobuf_fixtures_all(protobuf_fixtures: dict[str, list[dict[str, object]]]) -> None:
    """Verify all protobuf fixtures are organized correctly."""
    assert isinstance(protobuf_fixtures, dict)
    assert set(protobuf_fixtures.keys()) == {"server", "client", "bidirectional", "errors"}
    assert len(protobuf_fixtures["server"]) == 10
    assert len(protobuf_fixtures["client"]) == 10
    assert len(protobuf_fixtures["bidirectional"]) == 10
    assert len(protobuf_fixtures["errors"]) == 8
