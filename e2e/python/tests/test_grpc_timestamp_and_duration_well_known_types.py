@pytest.mark.asyncio
async def test_grpc_timestamp_and_duration_well_known_types() -> None:
    """Tests usage of google.protobuf.Timestamp and Duration types. Validates RFC 3339 timestamp serialization and duration calculations.."""

    from app.main import handle_grpc_timestamp_and_duration_well_known_types

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.EventService",
        method_name="LogEvent",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_timestamp_and_duration_well_known_types(request)

    # Verify response
    assert response.status_code == "OK"
    assert (
        response.payload
        == b'{"event_id":"event-001","processed_at":"2024-01-15T10:31:45.123Z","processing_time_ms":1000}'
    )
    assert response.metadata is not None
