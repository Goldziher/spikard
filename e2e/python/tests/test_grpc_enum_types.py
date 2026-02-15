import pytest
from spikard.grpc import GrpcRequest


@pytest.mark.asyncio
async def test_grpc_enum_types() -> None:
    """Tests enum definitions and serialization. Covers enum fields with named constants.."""

    from app.main import handle_grpc_enum_types

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.OrderService",
        method_name="CreateOrder",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_enum_types(request)

    # Verify response
    assert response.payload == b'{"id":1001,"product_name":"Laptop","quantity":2,"status":"PENDING","priority":"HIGH"}'
    assert response.metadata is not None
