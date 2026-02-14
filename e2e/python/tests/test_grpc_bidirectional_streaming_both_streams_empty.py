@pytest.mark.asyncio
async def test_grpc_bidirectional_streaming_both_streams_empty() -> None:
    """Tests bidirectional streaming RPC where both request and response streams are empty.."""

    from app.main import handle_grpc_bidirectional_streaming_both_streams_empty

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.EmptyBothService",
        method_name="Empty",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_bidirectional_streaming_both_streams_empty(request)

    # Verify response
    assert response.status_code == "OK"
    assert response.metadata is not None
