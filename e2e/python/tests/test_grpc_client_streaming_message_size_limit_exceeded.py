@pytest.mark.asyncio
async def test_grpc_client_streaming_message_size_limit_exceeded() -> None:
    """Tests client streaming RPC where one message exceeds the max_message_size limit. Server rejects the oversized message and terminates the stream.."""

    from app.main import handle_grpc_client_streaming_message_size_limit_exceeded

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
        "grpc-max-message-size": "4096",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.PayloadService",
        method_name="ProcessPayloads",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_client_streaming_message_size_limit_exceeded(request)

    # Verify response
    assert response.status_code == "RESOURCE_EXHAUSTED"
    assert (
        response.payload
        == b'{"message_id":"payload-002","processed_count":1,"status":"FAILED","error_detail":"Message payload size 10240 exceeds maximum allowed size 4096"}'
    )
    assert response.metadata is not None
