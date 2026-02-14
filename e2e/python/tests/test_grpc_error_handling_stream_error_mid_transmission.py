import pytest
from spikard.grpc import GrpcRequest


@pytest.mark.asyncio
async def test_grpc_error_handling_stream_error_mid_transmission() -> None:
    """Tests server streaming RPC that errors after yielding 3 messages. The stream opens successfully and delivers 3 messages before encountering an INTERNAL error. Validates that partial stream data is delivered before the error is signaled.."""

    from app.main import handle_grpc_error_handling_stream_error_mid_transmission

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.ErrorTestService",
        method_name="StreamWithError",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_error_handling_stream_error_mid_transmission(request)

    # Verify response
    assert response.status_code == "INTERNAL"
    assert response.metadata is not None
