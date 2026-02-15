import pytest
from spikard.grpc import GrpcRequest


@pytest.mark.asyncio
async def test_grpc_custom_authentication_scheme_header() -> None:
    """Tests custom authentication header scheme. Validates that custom auth headers are properly extracted and validated.."""

    from app.main import handle_grpc_custom_authentication_scheme_header

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
        "x-custom-auth": "CustomScheme token_value_123",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.CustomAuthService",
        method_name="Execute",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_custom_authentication_scheme_header(request)

    # Verify response
    assert response.payload == b'{"success":true}'
    assert response.metadata is not None
