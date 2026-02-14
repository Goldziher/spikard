import pytest
from spikard.grpc import GrpcRequest


@pytest.mark.asyncio
async def test_grpc_error_handling_invalid_request_payload() -> None:
    """Tests server streaming RPC with invalid request payload. Validates that INVALID_ARGUMENT status is returned when required field is missing from the request message. The server should reject the malformed payload before beginning the stream.."""

    from app.main import handle_grpc_error_handling_invalid_request_payload

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.ErrorTestService",
        method_name="ValidateRequest",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_error_handling_invalid_request_payload(request)

    # Verify response
    assert response.status_code == "INVALID_ARGUMENT"
    assert response.metadata is not None
