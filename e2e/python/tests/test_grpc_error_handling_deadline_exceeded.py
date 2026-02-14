@pytest.mark.asyncio
async def test_grpc_error_handling_deadline_exceeded() -> None:
    """Tests server streaming RPC that exceeds deadline. Expects DEADLINE_EXCEEDED status when RPC time exceeds configured timeout.."""

    from app.main import handle_grpc_error_handling_deadline_exceeded

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.ErrorTestService",
        method_name="SlowStream",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_error_handling_deadline_exceeded(request)

    # Verify response
    assert response.status_code == "DEADLINE_EXCEEDED"
    assert response.metadata is not None
