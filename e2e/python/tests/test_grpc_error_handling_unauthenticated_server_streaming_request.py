@pytest.mark.asyncio
async def test_grpc_error_handling_unauthenticated_server_streaming_request() -> None:
    """Tests server streaming RPC without required auth metadata. Expects UNAUTHENTICATED status when authorization header is missing.."""

    from app.main import handle_grpc_error_handling_unauthenticated_server_streaming_request

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.ErrorTestService",
        method_name="SecureStream",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_error_handling_unauthenticated_server_streaming_request(request)

    # Verify response
    assert response.status_code == "UNAUTHENTICATED"
    assert response.metadata is not None
