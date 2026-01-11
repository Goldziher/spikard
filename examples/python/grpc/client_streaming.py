"""Client Streaming gRPC Example - Log Aggregation.

This example demonstrates client streaming where a client streams log entries
to the server and receives a summary report after all logs are processed.

Use case: Data uploads, batch processing, telemetry collection

Run:
    python examples/python/grpc/client_streaming.py
"""

from __future__ import annotations

import asyncio
import json
from collections import Counter
from typing import TYPE_CHECKING

from spikard.grpc import GrpcHandler, GrpcRequest, GrpcResponse

if TYPE_CHECKING:
    from collections.abc import AsyncIterator


class LogAggregationHandler(GrpcHandler):
    """Handler for aggregating log entries from clients.

    Implements client streaming RPC where clients stream log entries
    and receive a summary statistics report after all logs are processed.
    """

    async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
        """Unary RPC - Submit single log entry."""
        json.loads(request.payload)

        resp_data = {"status": "accepted", "log_id": "12345"}
        return GrpcResponse(payload=json.dumps(resp_data).encode())

    async def handle_client_stream(self, request_stream: AsyncIterator[GrpcRequest]) -> GrpcResponse:
        """Client streaming RPC - Aggregate log entries.

        Client streams:
            {"level": "INFO", "message": "User login", "timestamp": 1234567890}
            {"level": "ERROR", "message": "Connection failed", "timestamp": 1234567891}
            {"level": "INFO", "message": "Request processed", "timestamp": 1234567892}
            ...

        Server returns summary:
            {
                "total_logs": 100,
                "level_counts": {"INFO": 75, "ERROR": 20, "WARN": 5},
                "time_range": {"start": 1234567890, "end": 1234567990}
            }
        """
        # Aggregate logs
        logs = []
        level_counts = Counter()
        min_timestamp = float("inf")
        max_timestamp = float("-inf")

        async for request in request_stream:
            log_data = json.loads(request.payload)

            level = log_data.get("level", "UNKNOWN")
            log_data.get("message", "")
            timestamp = log_data.get("timestamp", 0)

            logs.append(log_data)
            level_counts[level] += 1

            min_timestamp = min(min_timestamp, timestamp)
            max_timestamp = max(max_timestamp, timestamp)

            # Show progress
            if len(logs) % 10 == 0:
                pass

        # Generate summary report
        summary = {
            "total_logs": len(logs),
            "level_counts": dict(level_counts),
            "time_range": {
                "start": min_timestamp if min_timestamp != float("inf") else 0,
                "end": max_timestamp if max_timestamp != float("-inf") else 0,
            },
            "processed_at": asyncio.get_event_loop().time(),
        }

        return GrpcResponse(payload=json.dumps(summary).encode())


class BatchUploadHandler(GrpcHandler):
    """Handler for batch file uploads with progress tracking."""

    async def handle_client_stream(self, request_stream: AsyncIterator[GrpcRequest]) -> GrpcResponse:
        """Client streaming RPC - Upload file in chunks.

        Client streams:
            {"chunk_id": 0, "data": "base64...", "is_last": false}
            {"chunk_id": 1, "data": "base64...", "is_last": false}
            {"chunk_id": 2, "data": "base64...", "is_last": true}

        Server returns:
            {"file_id": "abc123", "total_chunks": 3, "total_bytes": 1024}
        """
        chunks = []
        total_bytes = 0

        async for request in request_stream:
            chunk_data = json.loads(request.payload)

            chunk_data.get("chunk_id", 0)
            data = chunk_data.get("data", "")
            is_last = chunk_data.get("is_last", False)

            chunk_size = len(data)
            total_bytes += chunk_size
            chunks.append(chunk_data)

            if is_last:
                break

        # Return upload summary
        result = {
            "file_id": f"upload_{asyncio.get_event_loop().time()}",
            "total_chunks": len(chunks),
            "total_bytes": total_bytes,
            "status": "success",
        }

        return GrpcResponse(payload=json.dumps(result).encode())


async def simulate_log_stream() -> AsyncIterator[GrpcRequest]:
    """Simulate a client streaming log entries."""
    log_levels = ["INFO", "WARN", "ERROR", "DEBUG"]
    messages = [
        "User logged in",
        "Request processed",
        "Connection established",
        "Database query executed",
        "Cache hit",
        "API call successful",
    ]

    for i in range(25):
        level = log_levels[i % len(log_levels)]
        message = messages[i % len(messages)]

        log_data = {
            "level": level,
            "message": f"{message} #{i + 1}",
            "timestamp": 1700000000 + i,
        }

        yield GrpcRequest(
            service_name="logging.v1.LogService",
            method_name="UploadLogs",
            payload=json.dumps(log_data).encode(),
            metadata={},
        )

        # Simulate network delay
        await asyncio.sleep(0.05)


async def simulate_file_upload() -> AsyncIterator[GrpcRequest]:
    """Simulate a client uploading a file in chunks."""
    chunk_size = 256
    total_chunks = 5

    for chunk_id in range(total_chunks):
        chunk_data = {
            "chunk_id": chunk_id,
            "data": "x" * chunk_size,  # Simulated data
            "is_last": chunk_id == total_chunks - 1,
        }

        yield GrpcRequest(
            service_name="storage.v1.StorageService",
            method_name="UploadFile",
            payload=json.dumps(chunk_data).encode(),
            metadata={},
        )

        await asyncio.sleep(0.1)


async def example_client_streaming() -> None:
    """Demonstrate client streaming with mock requests."""
    # Example 1: Log aggregation
    handler = LogAggregationHandler()

    request_stream = simulate_log_stream()
    response = await handler.handle_client_stream(request_stream)

    json.loads(response.payload)

    # Example 2: File upload
    upload_handler = BatchUploadHandler()

    upload_stream = simulate_file_upload()
    upload_response = await upload_handler.handle_client_stream(upload_stream)

    json.loads(upload_response.payload)


if __name__ == "__main__":
    # Run examples
    asyncio.run(example_client_streaming())
