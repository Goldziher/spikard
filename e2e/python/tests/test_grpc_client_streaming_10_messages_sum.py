@pytest.mark.asyncio
async def test_grpc_client_streaming_10_messages_sum() -> None:
    """Tests client streaming RPC where client sends 10 integer values. Server sums all values and returns result.."""

    from app.main import handle_grpc_client_streaming_10_messages_sum

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.MathService",
        method_name="SumNumbers",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_client_streaming_10_messages_sum(request)

    # Verify response
    assert response.status_code == "OK"
    assert response.payload == b'{"sequence_id":"seq-001","count":10,"sum":550,"status":"COMPLETE"}'
    assert response.metadata is not None
