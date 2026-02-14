@pytest.mark.asyncio
async def test_grpc_client_streaming_early_stream_close() -> None:
    """Tests client streaming RPC where client closes stream after sending 3 messages instead of the expected 5. Server should gracefully handle partial stream.."""

    from app.main import handle_grpc_client_streaming_early_stream_close

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.StreamService",
        method_name="SendChunks",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_client_streaming_early_stream_close(request)

    # Verify response
    assert response.status_code == "OK"
    assert (
        response.payload
        == b'{"session_id":"sess-early-001","received_chunks":3,"expected_chunks":5,"status":"INCOMPLETE"}'
    )
    assert response.metadata is not None
