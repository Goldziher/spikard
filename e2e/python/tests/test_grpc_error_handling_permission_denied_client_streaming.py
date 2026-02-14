import pytest
from spikard.grpc import GrpcRequest


@pytest.mark.asyncio
async def test_grpc_error_handling_permission_denied_client_streaming() -> None:
    """Tests client streaming RPC accessing unauthorized resource. Expects PERMISSION_DENIED status when client sends restricted access level requests. Demonstrates permission validation on streaming upload operations.."""

    from app.main import handle_grpc_error_handling_permission_denied_client_streaming

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.ErrorTestService",
        method_name="UploadRestricted",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_error_handling_permission_denied_client_streaming(request)

    # Verify response
    assert response.status_code == "PERMISSION_DENIED"
    assert response.metadata is not None
