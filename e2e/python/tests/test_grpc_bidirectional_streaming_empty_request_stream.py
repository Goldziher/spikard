@pytest.mark.asyncio
async def test_grpc_bidirectional_streaming_empty_request_stream() -> None:
    """Tests bidirectional streaming RPC with empty request stream but server sends response.."""

    from app.main import handle_grpc_bidirectional_streaming_empty_request_stream

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.EmptyService",
        method_name="HandleEmpty",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_bidirectional_streaming_empty_request_stream(request)

    # Verify response
    assert response.status_code == "OK"
    assert response.metadata is not None
