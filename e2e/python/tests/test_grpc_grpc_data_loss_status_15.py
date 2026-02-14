import pytest
from spikard.grpc import GrpcRequest


@pytest.mark.asyncio
async def test_grpc_grpc_data_loss_status_15() -> None:
    """Tests DATA_LOSS gRPC status code. Returned when unrecoverable data loss or corruption occurred.."""

    from app.main import handle_grpc_grpc_data_loss_status_15

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.DataService",
        method_name="Process",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_grpc_data_loss_status_15(request)

    # Verify response
    assert response.status_code == "DATA_LOSS"
    assert response.metadata is not None
