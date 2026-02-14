@pytest.mark.asyncio
async def test_grpc_grpc_failed_precondition_status_9() -> None:
    """Tests FAILED_PRECONDITION gRPC status code. Returned when the RPC failed because the system is not in the required state.."""

    from app.main import handle_grpc_grpc_failed_precondition_status_9

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.StateService",
        method_name="Proceed",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_grpc_failed_precondition_status_9(request)

    # Verify response
    assert response.status_code == "FAILED_PRECONDITION"
    assert response.metadata is not None
