import pytest
from spikard.grpc import GrpcRequest


@pytest.mark.asyncio
async def test_grpc_grpc_ok_status_0_successful_response() -> None:
    """Tests successful gRPC response with OK status code. Validates basic request-response completion.."""

    from app.main import handle_grpc_grpc_ok_status_0_successful_response

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.StatusService",
        method_name="CheckStatus",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_grpc_ok_status_0_successful_response(request)

    # Verify response
    assert response.payload == b'{"request_id":"status-ok-001","status":"success"}'
    assert response.metadata is not None
