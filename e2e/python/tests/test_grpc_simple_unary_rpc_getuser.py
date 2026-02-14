@pytest.mark.asyncio
async def test_grpc_simple_unary_rpc_getuser() -> None:
    """Tests basic unary gRPC call with scalar types (int32, string). Covers fundamental request-response pattern.."""

    from app.main import handle_grpc_simple_unary_rpc_getuser

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "authorization": "Bearer test-token",
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.UserService",
        method_name="GetUser",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_simple_unary_rpc_getuser(request)

    # Verify response
    assert response.status_code == "OK"
    assert response.payload == b'{"id":123,"name":"Alice Johnson","email":"alice@example.com"}'
    assert response.metadata is not None
