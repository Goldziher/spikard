import pytest
from spikard.grpc import GrpcRequest


@pytest.mark.asyncio
async def test_grpc_optional_fields() -> None:
    """Tests optional field handling with presence semantics. Covers optional fields with and without values.."""

    from app.main import handle_grpc_optional_fields

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.UserService",
        method_name="UpdateProfile",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_optional_fields(request)

    # Verify response
    assert response.status_code == "OK"
    assert (
        response.payload
        == b'{"user_id":42,"username":"charlie_dev","bio":"Software engineer and gRPC enthusiast","updated_at":1704067200000}'
    )
    assert response.metadata is not None
