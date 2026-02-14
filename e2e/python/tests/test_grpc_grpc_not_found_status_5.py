@pytest.mark.asyncio
async def test_grpc_grpc_not_found_status_5() -> None:
    """Tests NOT_FOUND gRPC status code. Returned when a requested resource (e.g., user, file) does not exist.."""

    from app.main import handle_grpc_grpc_not_found_status_5

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.ResourceService",
        method_name="Get",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_grpc_not_found_status_5(request)

    # Verify response
    assert response.status_code == "NOT_FOUND"
    assert response.metadata is not None
