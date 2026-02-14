import pytest
from spikard.grpc import GrpcRequest


@pytest.mark.asyncio
async def test_grpc_api_key_authentication() -> None:
    """Tests API key authentication via gRPC metadata. Validates that API keys are properly validated and associated with clients.."""

    from app.main import handle_grpc_api_key_authentication

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "x-api-key": "sk_live_abc123def456",
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.ApiService",
        method_name="FetchResource",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_api_key_authentication(request)

    # Verify response
    assert response.status_code == "OK"
    assert response.payload == b'{"data":"resource_data","client_id":"client-api-001"}'
    assert response.metadata is not None
