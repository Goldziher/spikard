import pytest
from spikard.grpc import GrpcRequest


@pytest.mark.asyncio
async def test_grpc_maximum_field_number_536870911() -> None:
    """Tests protobuf messages using the maximum allowed field number (536870911). Validates proper field number encoding in varint format.."""

    from app.main import handle_grpc_maximum_field_number_536870911

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.MaxFieldService",
        method_name="TestMaxField",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_maximum_field_number_536870911(request)

    # Verify response
    assert response.payload == b'{"id":42,"received_max":"Testing maximum field number"}'
    assert response.metadata is not None
