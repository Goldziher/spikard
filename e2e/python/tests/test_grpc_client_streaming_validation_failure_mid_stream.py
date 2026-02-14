import pytest
from spikard.grpc import GrpcRequest


@pytest.mark.asyncio
async def test_grpc_client_streaming_validation_failure_mid_stream() -> None:
    """Tests client streaming RPC where a message fails validation in the middle of the stream. Server rejects the stream and returns error.."""

    from app.main import handle_grpc_client_streaming_validation_failure_mid_stream

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.ValidationService",
        method_name="ValidateUsers",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_client_streaming_validation_failure_mid_stream(request)

    # Verify response
    assert (
        response.payload
        == b'{"processed":2,"status":"VALIDATION_FAILED","error_message":"Invalid email format at message index 2: invalid-email"}'
    )
    assert response.metadata is not None
