@pytest.mark.asyncio
async def test_grpc_server_streaming_with_large_response_data() -> None:
    """Tests server streaming RPC that yields multiple large messages. Validates proper streaming protocol handling and backpressure management.."""

    from app.main import handle_grpc_server_streaming_with_large_response_data

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.StreamingService",
        method_name="StreamLargeData",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_server_streaming_with_large_response_data(request)

    # Verify response
    assert response.status_code == "OK"
    assert response.payload == b'{"stream_id":"stream-large-001","chunk_number":1,"is_final":false}'
    assert response.metadata is not None
