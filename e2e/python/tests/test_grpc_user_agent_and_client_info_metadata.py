import pytest
from spikard.grpc import GrpcRequest


@pytest.mark.asyncio
async def test_grpc_user_agent_and_client_info_metadata() -> None:
    """Tests User-Agent header handling and client identification. Validates proper user-agent parsing and logging.."""

    from app.main import handle_grpc_user_agent_and_client_info_metadata

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "user-agent": "grpc-client/1.2.3 (linux; amd64)",
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.ClientService",
        method_name="Identify",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_user_agent_and_client_info_metadata(request)

    # Verify response
    assert response.status_code == "OK"
    assert response.payload == b'{"client_type":"grpc-client","client_version":"1.2.3"}'
    assert response.metadata is not None
