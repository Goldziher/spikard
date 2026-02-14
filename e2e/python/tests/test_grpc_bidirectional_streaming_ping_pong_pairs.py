@pytest.mark.asyncio
async def test_grpc_bidirectional_streaming_ping_pong_pairs() -> None:
    """Tests bidirectional streaming RPC with request-response pairs (ping-pong pattern).."""

    from app.main import handle_grpc_bidirectional_streaming_ping_pong_pairs

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.PingService",
        method_name="PingPong",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_bidirectional_streaming_ping_pong_pairs(request)

    # Verify response
    assert response.status_code == "OK"
    assert response.metadata is not None
