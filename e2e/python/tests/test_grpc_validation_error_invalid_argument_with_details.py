import pytest
from spikard.grpc import GrpcRequest


@pytest.mark.asyncio
async def test_grpc_validation_error_invalid_argument_with_details() -> None:
    """Tests INVALID_ARGUMENT status code with detailed validation error information. Demonstrates how validation failures are communicated.."""

    from app.main import handle_grpc_validation_error_invalid_argument_with_details

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.ValidationService",
        method_name="ValidateInput",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_validation_error_invalid_argument_with_details(request)

    # Verify response
    assert response.status_code == "INVALID_ARGUMENT"
    assert response.metadata is not None
