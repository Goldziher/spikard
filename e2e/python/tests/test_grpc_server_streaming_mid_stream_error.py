@pytest.mark.asyncio
async def test_grpc_server_streaming_mid_stream_error() -> None:
    """Tests server streaming RPC that sends 5 messages successfully, then encounters an error before completing the stream. Validates partial stream delivery and error handling.."""

    from app.main import handle_grpc_server_streaming_mid_stream_error

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.StreamService",
        method_name="StreamData",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_server_streaming_mid_stream_error(request)

    # Verify response
    assert response.status_code == "INTERNAL"
    assert response.metadata is not None
