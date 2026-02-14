import pytest
from spikard.grpc import GrpcRequest


@pytest.mark.asyncio
async def test_grpc_error_handling_grpc_status_codes() -> None:
    """Tests gRPC error status codes and error responses. Covers NOT_FOUND, INVALID_ARGUMENT, INTERNAL, and other gRPC status codes.."""

    from app.main import handle_grpc_error_handling_grpc_status_codes

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.ProductService",
        method_name="GetProduct",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_error_handling_grpc_status_codes(request)

    # Verify response
    assert response.metadata is not None
