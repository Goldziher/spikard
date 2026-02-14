@pytest.mark.asyncio
async def test_grpc_grpc_already_exists_status_6() -> None:
    """Tests ALREADY_EXISTS gRPC status code. Returned when trying to create a resource that already exists.."""

    from app.main import handle_grpc_grpc_already_exists_status_6

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.CreateService",
        method_name="Create",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_grpc_already_exists_status_6(request)

    # Verify response
    assert response.status_code == "ALREADY_EXISTS"
    assert response.metadata is not None
