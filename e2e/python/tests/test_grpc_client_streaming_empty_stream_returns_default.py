@pytest.mark.asyncio
async def test_grpc_client_streaming_empty_stream_returns_default() -> None:
    """Tests client streaming RPC where client sends no messages (empty stream). Server gracefully handles empty input and returns default response.."""

    from app.main import handle_grpc_client_streaming_empty_stream_returns_default

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.OptionalService",
        method_name="ProcessOptional",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_client_streaming_empty_stream_returns_default(request)

    # Verify response
    assert response.status_code == "OK"
    assert (
        response.payload
        == b'{"request_id":"empty-stream-req","message_count":0,"result":"DEFAULT_RESULT","is_default":true}'
    )
    assert response.metadata is not None
