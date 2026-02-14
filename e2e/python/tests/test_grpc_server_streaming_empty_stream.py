@pytest.mark.asyncio
async def test_grpc_server_streaming_empty_stream() -> None:
    """Tests server streaming RPC that returns an empty stream. The server opens the stream but sends no messages before completing successfully.."""

    from app.main import handle_grpc_server_streaming_empty_stream

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.StreamService",
        method_name="GetEmptyStream",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_server_streaming_empty_stream(request)

    # Verify response
    assert response.status_code == "OK"
    assert response.payload == b'"Stream completed with no messages"'
    assert response.metadata is not None
