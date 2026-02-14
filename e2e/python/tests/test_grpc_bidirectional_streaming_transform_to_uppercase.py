@pytest.mark.asyncio
async def test_grpc_bidirectional_streaming_transform_to_uppercase() -> None:
    """Tests bidirectional streaming RPC where server transforms incoming messages to uppercase.."""

    from app.main import handle_grpc_bidirectional_streaming_transform_to_uppercase

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.TransformService",
        method_name="Transform",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_bidirectional_streaming_transform_to_uppercase(request)

    # Verify response
    assert response.status_code == "OK"
    assert response.metadata is not None
