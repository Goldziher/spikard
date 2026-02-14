@pytest.mark.asyncio
async def test_grpc_grpc_deadline_exceeded_status_4() -> None:
    """Tests DEADLINE_EXCEEDED gRPC status code. Returned when the RPC does not complete within the specified time limit.."""

    from app.main import handle_grpc_grpc_deadline_exceeded_status_4

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.TimeoutService",
        method_name="SlowOp",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_grpc_deadline_exceeded_status_4(request)

    # Verify response
    assert response.status_code == "DEADLINE_EXCEEDED"
    assert response.metadata is not None
