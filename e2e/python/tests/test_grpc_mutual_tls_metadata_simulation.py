import pytest
from spikard.grpc import GrpcRequest


@pytest.mark.asyncio
async def test_grpc_mutual_tls_metadata_simulation() -> None:
    """Tests mutual TLS authentication by validating client certificate metadata. Simulates mTLS handshake verification.."""

    from app.main import handle_grpc_mutual_tls_metadata_simulation

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
        "x-client-cert-cn": "client.example.com",
        "x-client-cert-fingerprint": "AB:CD:EF:12:34:56:78:90",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.MtlsService",
        method_name="VerifyClient",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_mutual_tls_metadata_simulation(request)

    # Verify response
    assert response.payload == b'{"verified":true,"client_cn":"client.example.com"}'
    assert response.metadata is not None
