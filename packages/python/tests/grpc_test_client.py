"""
gRPC Test Client for executing fixtures against running gRPC server.

This module provides a wrapper for executing gRPC streaming fixtures
in integration tests.
"""

from __future__ import annotations

import json
from typing import AsyncIterator

import grpc
from grpc import aio


class GrpcTestClient:
    """Wrapper for executing gRPC streaming fixtures."""

    def __init__(self, server_address: str = "localhost:50051"):
        """
        Initialize gRPC test client.

        Args:
            server_address: Server address in format "host:port"
        """
        self.server_address = server_address
        self.channel: aio.Channel | None = None

    async def __aenter__(self) -> GrpcTestClient:
        """Async context manager entry."""
        self.channel = aio.insecure_channel(self.server_address)
        return self

    async def __aexit__(self, *args: object) -> None:
        """Async context manager exit."""
        if self.channel:
            await self.channel.close()

    async def execute_unary(
        self, service_name: str, method_name: str, request: dict[str, object]
    ) -> dict[str, object]:
        """
        Execute unary RPC from fixture.

        Args:
            service_name: Fully qualified service name
            method_name: Method name
            request: Request data as dictionary

        Returns:
            Response data as dictionary
        """
        if not self.channel:
            msg = "Channel not initialized. Use async context manager."
            raise RuntimeError(msg)

        method = f"/{service_name}/{method_name}"
        stub = self.channel.unary_unary(
            method,
            request_serializer=lambda x: json.dumps(x).encode(),
            response_deserializer=lambda x: json.loads(x.decode()),
        )

        response: dict[str, object] = await stub(request)
        return response

    async def execute_server_streaming(
        self, service_name: str, method_name: str, request: dict[str, object]
    ) -> list[dict[str, object]]:
        """
        Execute server streaming RPC from fixture.

        Args:
            service_name: Fully qualified service name
            method_name: Method name
            request: Request data as dictionary

        Returns:
            List of response messages
        """
        if not self.channel:
            msg = "Channel not initialized. Use async context manager."
            raise RuntimeError(msg)

        method = f"/{service_name}/{method_name}"
        stub = self.channel.unary_stream(
            method,
            request_serializer=lambda x: json.dumps(x).encode(),
            response_deserializer=lambda x: json.loads(x.decode()),
        )

        responses: list[dict[str, object]] = []
        call = stub(request)

        async for response in call:
            responses.append(response)

        return responses

    async def execute_client_streaming(
        self, service_name: str, method_name: str, requests: list[dict[str, object]]
    ) -> dict[str, object]:
        """
        Execute client streaming RPC from fixture.

        Args:
            service_name: Fully qualified service name
            method_name: Method name
            requests: List of request messages

        Returns:
            Response data as dictionary
        """
        if not self.channel:
            msg = "Channel not initialized. Use async context manager."
            raise RuntimeError(msg)

        method = f"/{service_name}/{method_name}"
        stub = self.channel.stream_unary(
            method,
            request_serializer=lambda x: json.dumps(x).encode(),
            response_deserializer=lambda x: json.loads(x.decode()),
        )

        async def request_iterator() -> AsyncIterator[dict[str, object]]:
            for req in requests:
                yield req

        response: dict[str, object] = await stub(request_iterator())
        return response

    async def execute_bidirectional(
        self, service_name: str, method_name: str, requests: list[dict[str, object]]
    ) -> list[dict[str, object]]:
        """
        Execute bidirectional streaming RPC from fixture.

        Args:
            service_name: Fully qualified service name
            method_name: Method name
            requests: List of request messages

        Returns:
            List of response messages
        """
        if not self.channel:
            msg = "Channel not initialized. Use async context manager."
            raise RuntimeError(msg)

        method = f"/{service_name}/{method_name}"
        stub = self.channel.stream_stream(
            method,
            request_serializer=lambda x: json.dumps(x).encode(),
            response_deserializer=lambda x: json.loads(x.decode()),
        )

        async def request_iterator() -> AsyncIterator[dict[str, object]]:
            for req in requests:
                yield req

        responses: list[dict[str, object]] = []
        call = stub(request_iterator())

        async for response in call:
            responses.append(response)

        return responses
