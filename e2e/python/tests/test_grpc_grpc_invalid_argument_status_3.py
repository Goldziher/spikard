import pytest
from spikard.grpc import GrpcRequest


@pytest.mark.asyncio
async def test_grpc_grpc_invalid_argument_status_3() -> None:
    """Tests INVALID_ARGUMENT gRPC status code. Indicates that the client provided an invalid or malformed argument.."""

    from app.main import handle_grpc_grpc_invalid_argument_status_3

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.ArgService",
        method_name="Validate",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_grpc_invalid_argument_status_3(request)

    # Verify response
    assert response.metadata is not None
