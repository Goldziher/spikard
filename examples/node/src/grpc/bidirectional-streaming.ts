/**
 * Bidirectional Streaming gRPC Example: Real-time Chat Service
 *
 * This example demonstrates bidirectional streaming RPC where both client
 * and server send streams of messages concurrently.
 *
 * Current Implementation Note:
 * Due to napi-rs limitations, bidirectional streaming uses a collection
 * pattern where the handler consumes all client messages and returns
 * a batch of responses. This is similar to combining client streaming
 * (consume input) with server streaming (pre-collect output).
 *
 * For true bidirectional streaming in production, consider:
 * 1. WebSocket connections for real-time bidirectional communication
 * 2. Server-Sent Events (SSE) with HTTP/2 for unidirectional streaming
 * 3. Callback-based patterns (future enhancement)
 *
 * @see crates/spikard-node/src/grpc/handler.rs for implementation details
 */

import type { GrpcRequest, GrpcResponse } from "@spikard/node";

// Mock protobuf types (in production, use generated proto files)
interface ChatMessage {
	userId: string;
	username: string;
	message: string;
	timestamp: number;
	roomId: string;
}

interface ChatResponse {
	messageId: string;
	status: "delivered" | "failed";
	timestamp: number;
	error?: string;
}

interface ChatStreamResponse {
	responses: ChatResponse[];
}

/**
 * Chat Service Handler
 *
 * Demonstrates bidirectional streaming pattern where the handler:
 * 1. Consumes incoming messages from client stream
 * 2. Processes each message
 * 3. Returns batch of responses (due to napi-rs limitations)
 *
 * In a true bidirectional implementation, responses would be yielded
 * incrementally as messages are processed.
 */
class ChatServiceHandler {
	private messageHistory: Map<string, ChatMessage[]> = new Map();

	/**
	 * Bidirectional Streaming RPC: Real-time chat
	 *
	 * Current implementation uses the collection pattern:
	 * - Consumes all incoming messages from requestStream
	 * - Processes each message
	 * - Returns all responses in a single batch
	 *
	 * @param requestStream AsyncIterator yielding GrpcRequest objects
	 * @returns Promise<GrpcResponse> with batched ChatResponse array
	 */
	async handleBidiStream(requestStream: AsyncIterator<GrpcRequest>): Promise<GrpcResponse> {
		console.log("[Bidirectional Streaming] Chat called");

		const responses: ChatResponse[] = [];
		let messageCount = 0;

		try {
			// Consume all incoming messages
			for await (const request of requestStream) {
				messageCount++;
				console.log(`  Received message ${messageCount}`);

				// Deserialize incoming message
				const chatMsg = JSON.parse(request.payload.toString("utf-8")) as ChatMessage;
				console.log(`    ${chatMsg.username}: ${chatMsg.message}`);

				// Process message and generate response
				try {
					// Store message in room history
					if (!this.messageHistory.has(chatMsg.roomId)) {
						this.messageHistory.set(chatMsg.roomId, []);
					}
					this.messageHistory.get(chatMsg.roomId)!.push(chatMsg);

					// Simulate message processing
					await this.delay(50);

					// Create successful response
					const response: ChatResponse = {
						messageId: this.generateMessageId(chatMsg),
						status: "delivered",
						timestamp: Date.now(),
					};

					responses.push(response);
					console.log(`    ✓ Delivered: ${response.messageId}`);

					// Simulate broadcasting to other users (conceptual)
					await this.broadcastToRoom(chatMsg);
				} catch (error) {
					// Handle message processing error
					const errorResponse: ChatResponse = {
						messageId: this.generateMessageId(chatMsg),
						status: "failed",
						timestamp: Date.now(),
						error: error instanceof Error ? error.message : "Unknown error",
					};

					responses.push(errorResponse);
					console.log(`    ✗ Failed: ${errorResponse.error}`);
				}
			}
		} catch (error) {
			console.error("Error consuming stream:", error);
			throw new Error("INTERNAL: Failed to process chat stream");
		}

		console.log(`\nProcessed ${messageCount} messages, returning ${responses.length} responses`);

		// Return all responses in a batch
		const result: ChatStreamResponse = { responses };
		const responsePayload = Buffer.from(JSON.stringify(result), "utf-8");

		return {
			payload: responsePayload,
			metadata: {
				"content-type": "application/json",
				"x-service": "chat-service",
				"x-message-count": String(messageCount),
				"x-response-count": String(responses.length),
			},
		};
	}

	/**
	 * Generate unique message ID
	 */
	private generateMessageId(msg: ChatMessage): string {
		return `${msg.roomId}-${msg.userId}-${msg.timestamp}`;
	}

	/**
	 * Simulate broadcasting message to room (conceptual)
	 */
	private async broadcastToRoom(msg: ChatMessage): Promise<void> {
		// In production, this would:
		// 1. Look up all connected users in the room
		// 2. Send the message to each user's stream
		// 3. Handle disconnected users

		// Simulate broadcast delay
		await this.delay(20);

		const roomUsers = this.messageHistory.get(msg.roomId)?.length ?? 1;
		console.log(`    📡 Broadcast to ${roomUsers} users in room ${msg.roomId}`);
	}

	/**
	 * Get message history for a room
	 */
	getRoomHistory(roomId: string): ChatMessage[] {
		return this.messageHistory.get(roomId) ?? [];
	}

	/**
	 * Utility: Delay for ms milliseconds
	 */
	private delay(ms: number): Promise<void> {
		return new Promise((resolve) => setTimeout(resolve, ms));
	}

	/**
	 * Generate sample chat messages
	 */
	static generateSampleMessages(count: number, roomId: string): ChatMessage[] {
		const users = [
			{ id: "user1", name: "Alice" },
			{ id: "user2", name: "Bob" },
			{ id: "user3", name: "Charlie" },
			{ id: "user4", name: "Diana" },
		];

		const messageTemplates = [
			"Hello everyone!",
			"How are you doing?",
			"Check out this cool feature!",
			"Anyone up for a game?",
			"Thanks for the help!",
			"That makes sense",
			"I agree with that",
			"Interesting point",
			"Could you clarify?",
			"See you later!",
		];

		const messages: ChatMessage[] = [];
		const baseTimestamp = Date.now() - 60000; // Start 1 minute ago

		for (let i = 0; i < count; i++) {
			const user = users[i % users.length];
			const message = messageTemplates[Math.floor(Math.random() * messageTemplates.length)];

			messages.push({
				userId: user.id,
				username: user.name,
				message,
				timestamp: baseTimestamp + i * 2000, // 2 seconds apart
				roomId,
			});
		}

		return messages;
	}
}

/**
 * Mock async generator to simulate client message stream
 */
async function* createChatStream(messages: ChatMessage[]): AsyncGenerator<GrpcRequest> {
	for (const msg of messages) {
		// Simulate typing and network delay
		await new Promise((resolve) => setTimeout(resolve, 100));

		yield {
			serviceName: "chat.v1.ChatService",
			methodName: "StreamChat",
			payload: Buffer.from(JSON.stringify(msg), "utf-8"),
			metadata: {
				"user-id": msg.userId,
				"room-id": msg.roomId,
			},
		};
	}
}

/**
 * Client: Test the chat service
 */
async function runClient() {
	console.log("=== Bidirectional Chat Client ===\n");

	const service = new ChatServiceHandler();
	const roomId = "general";

	// Generate sample chat messages
	const messages = ChatServiceHandler.generateSampleMessages(12, roomId);

	console.log(`Generated ${messages.length} chat messages for room: ${roomId}\n`);
	console.log("Streaming messages to chat service...\n");

	// Create client stream
	const chatStream = createChatStream(messages);

	// Call bidirectional streaming RPC
	const response = await service.handleBidiStream(chatStream);

	// Parse and display results
	const result = JSON.parse(response.payload.toString("utf-8")) as ChatStreamResponse;

	console.log("\n=== Chat Session Results ===");
	console.log(`Messages sent: ${messages.length}`);
	console.log(`Responses received: ${result.responses.length}`);

	// Display delivery status
	const delivered = result.responses.filter((r) => r.status === "delivered").length;
	const failed = result.responses.filter((r) => r.status === "failed").length;

	console.log(`\nDelivery status:`);
	console.log(`  ✓ Delivered: ${delivered}`);
	console.log(`  ✗ Failed: ${failed}`);

	// Display sample responses
	console.log(`\nSample responses:`);
	result.responses.slice(0, 5).forEach((resp, i) => {
		const icon = resp.status === "delivered" ? "✓" : "✗";
		const time = new Date(resp.timestamp).toISOString();
		console.log(`  ${icon} ${resp.messageId} @ ${time}`);
		if (resp.error) {
			console.log(`    Error: ${resp.error}`);
		}
	});

	if (result.responses.length > 5) {
		console.log(`  ... and ${result.responses.length - 5} more`);
	}

	// Display room history
	const history = service.getRoomHistory(roomId);
	console.log(`\nRoom history: ${history.length} messages stored`);

	console.log("\n=== Demo Complete ===");
}

/**
 * Server: Start the gRPC server (when Spikard supports it)
 */
async function runServer() {
	console.log("=== Starting Chat Service ===\n");

	// Note: This is a conceptual example. Actual server implementation
	// depends on Spikard's gRPC server API being finalized.
	//
	// Expected usage:
	// const app = new Spikard();
	// app.registerGrpcHandler('chat.v1.ChatService', new ChatServiceHandler());
	// await app.listen(50051);

	console.log("Server would run on port 50051");
	console.log("Service: chat.v1.ChatService");
	console.log("Methods: StreamChat (bidirectional streaming)\n");

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

export { ChatServiceHandler, type ChatMessage, type ChatResponse, type ChatStreamResponse, createChatStream };
