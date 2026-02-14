import pytest
from spikard.grpc import GrpcRequest


@pytest.mark.asyncio
async def test_grpc_error_handling_resource_not_found() -> None:
    """Tests NOT_FOUND gRPC status code. Returned when the requested resource does not exist. Validates unary RPC requesting non-existent resource.."""

    from app.main import handle_grpc_error_handling_resource_not_found

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.ErrorTestService",
        method_name="GetResource",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_error_handling_resource_not_found(request)

    # Verify response
    assert response.status_code == "NOT_FOUND"
    assert response.metadata is not None
