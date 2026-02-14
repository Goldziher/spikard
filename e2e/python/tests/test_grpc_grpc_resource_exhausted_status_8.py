@pytest.mark.asyncio
async def test_grpc_grpc_resource_exhausted_status_8() -> None:
    """Tests RESOURCE_EXHAUSTED gRPC status code. Returned when the server has run out of resources (disk space, memory, connections, etc.).."""

    from app.main import handle_grpc_grpc_resource_exhausted_status_8

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.ResourceService",
        method_name="AllocateMemory",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_grpc_resource_exhausted_status_8(request)

    # Verify response
    assert response.status_code == "RESOURCE_EXHAUSTED"
    assert response.metadata is not None
