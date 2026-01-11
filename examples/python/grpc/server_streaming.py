"""Server Streaming gRPC Example - Stock Price Ticker.

This example demonstrates server streaming where a client requests stock price
updates and the server continuously streams price data back.

Use case: Real-time data feeds, progress updates, live metrics

Run:
    python examples/python/grpc/server_streaming.py
"""

# ruff: noqa: S311
# S311: Using standard pseudo-random generators for non-cryptographic use cases
# (generating mock stock prices in this example code, not cryptographic purposes)

from __future__ import annotations

import asyncio
import json
import random
from typing import TYPE_CHECKING

from spikard.grpc import GrpcHandler, GrpcRequest, GrpcResponse

if TYPE_CHECKING:
    from collections.abc import AsyncGenerator


class StockPriceHandler(GrpcHandler):
    """Handler for streaming stock price updates.

    Implements server streaming RPC where clients request stock prices
    and receive continuous updates until the stream completes or client
    disconnects.
    """

    async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
        """Unary RPC - Get single stock price snapshot."""
        # Parse request (in real app, use protobuf)
        req_data = json.loads(request.payload)
        symbol = req_data.get("symbol", "UNKNOWN")

        # Generate single price
        price = self._generate_price(symbol)

        # Return response
        resp_data = {
            "symbol": symbol,
            "price": price,
            "timestamp": asyncio.get_event_loop().time(),
        }
        return GrpcResponse(payload=json.dumps(resp_data).encode())

    async def handle_server_stream(self, request: GrpcRequest) -> AsyncGenerator[GrpcResponse]:
        """Server streaming RPC - Stream stock price updates.

        Client sends:
            {"symbol": "AAPL", "count": 10}

        Server yields:
            {"symbol": "AAPL", "price": 150.25, "timestamp": 1234567890}
            {"symbol": "AAPL", "price": 150.50, "timestamp": 1234567891}
            ...
        """
        # Parse request
        req_data = json.loads(request.payload)
        symbol = req_data.get("symbol", "AAPL")
        count = req_data.get("count", 10)

        # Stream price updates
        for i in range(count):
            # Simulate price fluctuation
            price = self._generate_price(symbol, i)

            # Create response
            resp_data = {
                "symbol": symbol,
                "price": price,
                "timestamp": asyncio.get_event_loop().time(),
                "sequence": i + 1,
            }

            yield GrpcResponse(payload=json.dumps(resp_data).encode())

            # Simulate real-time delay
            await asyncio.sleep(0.1)

    def _generate_price(self, symbol: str, sequence: int = 0) -> float:
        """Generate realistic stock price with random walk."""
        # Base prices for different symbols
        base_prices = {
            "AAPL": 150.0,
            "GOOGL": 140.0,
            "MSFT": 380.0,
            "TSLA": 250.0,
        }

        base = base_prices.get(symbol, 100.0)

        # Random walk with small changes
        change = random.uniform(-2.0, 2.0) + (sequence * 0.1)
        return round(base + change, 2)


class StockTickerAdvanced(GrpcHandler):
    """Advanced handler with multiple stock symbols and real-time updates."""

    async def handle_server_stream(self, request: GrpcRequest) -> AsyncGenerator[GrpcResponse]:
        """Stream multiple stock prices concurrently.

        Client sends:
            {"symbols": ["AAPL", "GOOGL", "MSFT"], "duration_seconds": 5}

        Server yields updates for all symbols as they become available.
        """
        req_data = json.loads(request.payload)
        symbols = req_data.get("symbols", ["AAPL"])
        duration = req_data.get("duration_seconds", 5)

        start_time = asyncio.get_event_loop().time()
        sequence = 0

        while (asyncio.get_event_loop().time() - start_time) < duration:
            # Update all symbols
            for symbol in symbols:
                price = self._generate_price(symbol, sequence)

                resp_data = {
                    "symbol": symbol,
                    "price": price,
                    "timestamp": asyncio.get_event_loop().time(),
                    "sequence": sequence,
                }

                yield GrpcResponse(payload=json.dumps(resp_data).encode())

            sequence += 1
            await asyncio.sleep(0.5)

    def _generate_price(self, symbol: str, sequence: int) -> float:
        """Generate stock price with trend."""
        base_prices = {"AAPL": 150.0, "GOOGL": 140.0, "MSFT": 380.0}
        base = base_prices.get(symbol, 100.0)

        # Combine random walk with slight upward trend
        trend = sequence * 0.05
        noise = random.uniform(-1.0, 1.0)
        return round(base + trend + noise, 2)


async def example_server_streaming() -> None:
    """Demonstrate server streaming with mock requests."""
    handler = StockPriceHandler()

    # Example 1: Stream 5 updates for AAPL
    request = GrpcRequest(
        service_name="stock.v1.StockService",
        method_name="StreamPrices",
        payload=json.dumps({"symbol": "AAPL", "count": 5}).encode(),
        metadata={},
    )

    async for response in handler.handle_server_stream(request):
        json.loads(response.payload)
        # Client would display: "AAPL: $150.25 (sequence #1)"

    # Example 2: Multiple stocks with advanced handler
    advanced_handler = StockTickerAdvanced()

    request2 = GrpcRequest(
        service_name="stock.v1.StockService",
        method_name="StreamMultiplePrices",
        payload=json.dumps({"symbols": ["AAPL", "GOOGL"], "duration_seconds": 2}).encode(),
        metadata={},
    )

    async for response in advanced_handler.handle_server_stream(request2):
        json.loads(response.payload)
        # Client would process updates for all symbols


if __name__ == "__main__":
    # Run examples
    asyncio.run(example_server_streaming())
