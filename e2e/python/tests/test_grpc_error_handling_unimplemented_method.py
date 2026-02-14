@pytest.mark.asyncio
async def test_grpc_error_handling_unimplemented_method() -> None:
    """Tests unary RPC calling an unimplemented method. Validates that UNIMPLEMENTED status is returned when the server does not support the requested RPC method. This fixture ensures proper error handling for feature requests that are not yet available in the current server implementation.."""

    from app.main import handle_grpc_error_handling_unimplemented_method

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.ErrorTestService",
        method_name="FutureFeature",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_error_handling_unimplemented_method(request)

    # Verify response
    assert response.status_code == "UNIMPLEMENTED"
    assert response.metadata is not None
