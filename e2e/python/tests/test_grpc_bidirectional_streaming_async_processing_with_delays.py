@pytest.mark.asyncio
async def test_grpc_bidirectional_streaming_async_processing_with_delays() -> None:
    """Tests bidirectional streaming RPC with asynchronous message processing.."""

    from app.main import handle_grpc_bidirectional_streaming_async_processing_with_delays

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.AsyncService",
        method_name="ProcessAsync",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_bidirectional_streaming_async_processing_with_delays(request)

    # Verify response
    assert response.status_code == "OK"
    assert response.metadata is not None
