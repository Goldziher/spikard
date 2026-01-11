/**
 * Server Streaming gRPC Example: Real-time Stock Price Ticker
 *
 * This example demonstrates server streaming RPC where a client requests
 * stock price updates and the server streams price ticks in real-time.
 *
 * Current Implementation Note:
 * Due to napi-rs limitations with async generators, this example demonstrates
 * the recommended "pre-collection" pattern where the handler collects all
 * messages and returns them in a single response.
 *
 * For true streaming in production, consider:
 * 1. Using callback-based patterns (future enhancement)
 * 2. Implementing stream registry with getter/setter functions
 * 3. WebSocket or SSE for true real-time updates
 *
 * @see crates/spikard-node/src/grpc/handler.rs for implementation details
 */

import { Spikard } from "@spikard/node";
import type { GrpcRequest, GrpcResponse } from "@spikard/node";

// Mock protobuf types (in production, use generated proto files)
interface StockPriceRequest {
	symbol: string;
	count: number;
}

interface StockPrice {
	symbol: string;
	price: number;
	timestamp: number;
}

interface StockPriceStreamResponse {
	prices: StockPrice[];
}

/**
 * Stock Price Service Handler
 *
 * Demonstrates server streaming pattern with pre-collected messages
 */
class StockPriceServiceHandler {
	/**
	 * Unary RPC: Get current stock price
	 */
	async handleRequest(request: GrpcRequest): Promise<GrpcResponse> {
		console.log(`[Unary] ${request.methodName} called`);

		// Deserialize request (mock - use real protobuf in production)
		const payload = JSON.parse(request.payload.toString("utf-8")) as StockPriceRequest;

		// Generate single price
		const price: StockPrice = {
			symbol: payload.symbol,
			price: this.generatePrice(payload.symbol),
			timestamp: Date.now(),
		};

		// Serialize response
		const responsePayload = Buffer.from(JSON.stringify(price), "utf-8");

		return {
			payload: responsePayload,
			metadata: {
				"content-type": "application/json",
				"x-service": "stock-price-service",
			},
		};
	}

	/**
	 * Server Streaming RPC: Stream stock price updates
	 *
	 * Note: Due to napi-rs limitations, this uses the pre-collection pattern
	 * where all messages are collected before returning a single response.
	 *
	 * In production:
	 * - Use repeated fields in protobuf
	 * - Or implement callback-based streaming (future enhancement)
	 */
	async handleServerStream(request: GrpcRequest): Promise<GrpcResponse> {
		console.log(`[Server Streaming] ${request.methodName} called`);

		// Deserialize request
		const payload = JSON.parse(request.payload.toString("utf-8")) as StockPriceRequest;
		const { symbol, count = 10 } = payload;

		console.log(`Collecting ${count} price updates for ${symbol}...`);

		// Pre-collect all price updates
		const prices: StockPrice[] = [];
		for (let i = 0; i < count; i++) {
			const price: StockPrice = {
				symbol,
				price: this.generatePrice(symbol),
				timestamp: Date.now() + i * 1000, // Simulate 1-second intervals
			};

			prices.push(price);
			console.log(`  [${i + 1}/${count}] ${symbol}: $${price.price.toFixed(2)}`);

			// Simulate price generation delay
			await this.delay(100);
		}

		// Return all prices in single response
		const response: StockPriceStreamResponse = { prices };
		const responsePayload = Buffer.from(JSON.stringify(response), "utf-8");

		console.log(`Returning ${prices.length} prices in batch`);

		return {
			payload: responsePayload,
			metadata: {
				"content-type": "application/json",
				"x-service": "stock-price-service",
				"x-message-count": String(prices.length),
			},
		};
	}

	/**
	 * Generate realistic stock price with random walk
	 */
	private generatePrice(symbol: string): number {
		const basePrice = this.getBasePrice(symbol);
		const volatility = 0.02; // 2% volatility
		const change = (Math.random() - 0.5) * 2 * volatility;
		return basePrice * (1 + change);
	}

	/**
	 * Get base price for known symbols
	 */
	private getBasePrice(symbol: string): number {
		const basePrices: Record<string, number> = {
			AAPL: 150.0,
			GOOGL: 2800.0,
			TSLA: 800.0,
			MSFT: 300.0,
			AMZN: 3300.0,
		};

		return basePrices[symbol] ?? 100.0;
	}

	/**
	 * Utility: Delay for ms milliseconds
	 */
	private delay(ms: number): Promise<void> {
		return new Promise((resolve) => setTimeout(resolve, ms));
	}
}

/**
 * Client: Test the stock price streaming service
 */
async function runClient() {
	console.log("=== Stock Price Streaming Client ===\n");

	// Create mock client (in production, use @grpc/grpc-js client)
	const service = new StockPriceServiceHandler();

	// Test 1: Unary RPC
	console.log("Test 1: Unary RPC - Get current price");
	console.log("---------------------------------------");
	const unaryRequest: GrpcRequest = {
		serviceName: "stock.v1.StockService",
		methodName: "GetPrice",
		payload: Buffer.from(JSON.stringify({ symbol: "AAPL", count: 1 }), "utf-8"),
		metadata: {},
	};

	const unaryResponse = await service.handleRequest(unaryRequest);
	const unaryPrice = JSON.parse(unaryResponse.payload.toString("utf-8")) as StockPrice;
	console.log(`✓ Current price: ${unaryPrice.symbol} = $${unaryPrice.price.toFixed(2)}\n`);

	// Test 2: Server Streaming RPC (pre-collection pattern)
	console.log("Test 2: Server Streaming RPC - Stream price updates");
	console.log("-----------------------------------------------------");
	const streamRequest: GrpcRequest = {
		serviceName: "stock.v1.StockService",
		methodName: "StreamPrices",
		payload: Buffer.from(JSON.stringify({ symbol: "TSLA", count: 5 }), "utf-8"),
		metadata: {},
	};

	const streamResponse = await service.handleServerStream(streamRequest);
	const streamData = JSON.parse(streamResponse.payload.toString("utf-8")) as StockPriceStreamResponse;

	console.log(`\n✓ Received ${streamData.prices.length} price updates:`);
	streamData.prices.forEach((price, i) => {
		console.log(
			`  [${i + 1}] ${price.symbol}: $${price.price.toFixed(2)} @ ${new Date(price.timestamp).toISOString()}`,
		);
	});

	console.log("\n=== Demo Complete ===");
}

/**
 * Server: Start the gRPC server (when Spikard supports it)
 */
async function runServer() {
	console.log("=== Starting Stock Price Service ===\n");

	// Note: This is a conceptual example. Actual server implementation
	// depends on Spikard's gRPC server API being finalized.
	//
	// Expected usage:
	// const app = new Spikard();
	// app.registerGrpcHandler('stock.v1.StockService', new StockPriceServiceHandler());
	// await app.listen(50051);

	console.log("Server would run on port 50051");
	console.log("Service: stock.v1.StockService");
	console.log("Methods: GetPrice (unary), StreamPrices (server streaming)\n");

	console.log("⚠️  Server implementation pending Spikard gRPC server API");
}

/**
 * Main entry point
 */
async function main() {
	const mode = process.argv[2] || "client";

	if (mode === "server") {
		await runServer();
	} else {
		await runClient();
	}
}

// Run if executed directly
if (require.main === module) {
	main().catch((error) => {
		console.error("Error:", error);
		process.exit(1);
	});
}

export { StockPriceServiceHandler, type StockPriceRequest, type StockPrice, type StockPriceStreamResponse };
