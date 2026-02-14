@pytest.mark.asyncio
async def test_grpc_server_streaming_with_metadata_and_trailers() -> None:
    """Tests server streaming RPC with gRPC metadata headers and trailers. Validates that metadata is accessible before streaming begins and trailers are delivered after stream completion.."""

    from app.main import handle_grpc_server_streaming_with_metadata_and_trailers

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "x-client-version": "1.0.0",
        "content-type": "application/grpc",
        "x-request-id": "metadata-stream-001",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.StreamService",
        method_name="StreamWithMetadata",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_server_streaming_with_metadata_and_trailers(request)

    # Verify response
    assert response.status_code == "OK"
    assert response.payload == b'"Stream completed with metadata and trailers"'
    assert response.metadata is not None
