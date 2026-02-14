@pytest.mark.asyncio
async def test_grpc_grpc_aborted_status_10() -> None:
    """Tests ABORTED gRPC status code. Returned when an operation was aborted, typically due to a concurrency issue like conflict.."""

    from app.main import handle_grpc_grpc_aborted_status_10

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.TransactionService",
        method_name="Commit",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_grpc_aborted_status_10(request)

    # Verify response
    assert response.status_code == "ABORTED"
    assert response.metadata is not None
