import pytest
from spikard.grpc import GrpcRequest


@pytest.mark.asyncio
async def test_grpc_oneof_field_handling() -> None:
    """Tests oneof fields where only one field in the group can be set at a time. Validates proper mutual exclusivity and serialization.."""

    from app.main import handle_grpc_oneof_field_handling

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.OneofService",
        method_name="ProcessOneof",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_oneof_field_handling(request)

    # Verify response
    assert response.payload == b'{"received_type":"text_data","data_present":true}'
    assert response.metadata is not None
