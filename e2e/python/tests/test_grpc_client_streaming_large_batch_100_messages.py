@pytest.mark.asyncio
async def test_grpc_client_streaming_large_batch_100_messages() -> None:
    """Tests client streaming RPC with 100 messages in the stream. Validates performance with large batch aggregation.."""

    from app.main import handle_grpc_client_streaming_large_batch_100_messages

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.BatchService",
        method_name="ProcessBatch",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_client_streaming_large_batch_100_messages(request)

    # Verify response
    assert response.status_code == "OK"
    assert (
        response.payload
        == b'{"batch_id":"batch-large-001","total_items":100,"total_value":5050,"average_value":50.5,"status":"PROCESSED"}'
    )
    assert response.metadata is not None
