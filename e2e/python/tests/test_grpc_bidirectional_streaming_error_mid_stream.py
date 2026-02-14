@pytest.mark.asyncio
async def test_grpc_bidirectional_streaming_error_mid_stream() -> None:
    """Tests bidirectional streaming RPC where server returns error after processing some messages.."""

    from app.main import handle_grpc_bidirectional_streaming_error_mid_stream

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.ErrorService",
        method_name="ProcessWithError",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_bidirectional_streaming_error_mid_stream(request)

    # Verify response
    assert response.status_code == "INTERNAL"
    assert response.payload == b'"Error after processing 2 messages"'
    assert response.metadata is not None
