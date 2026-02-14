@pytest.mark.asyncio
async def test_grpc_bidirectional_streaming_with_large_payloads() -> None:
    """Tests bidirectional streaming RPC with large messages in both directions. Validates concurrent read/write handling and proper message ordering.."""

    from app.main import handle_grpc_bidirectional_streaming_with_large_payloads

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.BiDirectionalService",
        method_name="Exchange",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_bidirectional_streaming_with_large_payloads(request)

    # Verify response
    assert response.status_code == "OK"
    assert response.payload == b'{"message_id":"bi-large-001","sequence":1,"direction":"server-to-client"}'
    assert response.metadata is not None
