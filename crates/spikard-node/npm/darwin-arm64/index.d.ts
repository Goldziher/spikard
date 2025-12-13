/* eslint-disable */
/** Node.js wrapper for SSE event */
export declare class SseEvent {
	/** Get the data field of the event */
	getData(): string;
	/** Parse the event data as JSON */
	asJson(): unknown;
}

/** Node.js wrapper for SSE stream */
export declare class SseStream {
	/** Get the raw body of the SSE response */
	body(): string;
	/** Get all events from the stream */
	events(): Array<SseEvent>;
	/** Get events as JSON values */
	eventsAsJson(): Array<unknown>;
}

/** HTTP Response wrapper */
export declare class TestResponse {
	/** Get the HTTP status code */
	get statusCode(): number;
	/** Get response headers as JSON */
	headers(): Record<string, string>;
	/** Get response body as text */
	text(): string;
	/** Parse response body as JSON */
	json(): unknown;
	/** Get raw response body bytes */
	bytes(): Buffer;
}

/** Node.js wrapper for WebSocket messages */
export declare class WebSocketMessage {
	/** Get message as text if it's a text message */
	asText(): string | null;
	/** Get message as JSON if it's a text message containing JSON */
	asJson(): unknown | null;
	/** Get message as binary if it's a binary message */
	asBinary(): Buffer | null;
	/** Check if this is a close message */
	isClose(): boolean;
}

/** Node.js wrapper for WebSocket test client */
export declare class WebSocketTestConnection {
	/** Send a text message */
	sendText(text: string): Promise<void>;
	/** Send a JSON message */
	sendJson(obj: unknown): Promise<void>;
	/** Receive a text message */
	receiveText(): Promise<string>;
	/** Receive and parse a JSON message */
	receiveJson(): Promise<unknown>;
	/** Receive raw bytes */
	receiveBytes(): Promise<Buffer>;
	/** Receive a message and return WebSocketMessage */
	receiveMessage(): Promise<WebSocketMessage>;
	/** Close the WebSocket connection */
	close(): Promise<void>;
}

export declare function backgroundRun(task: () => Promise<undefined>): void;

/**
 * Structured handler input passed to JavaScript handlers
 *
 * This struct replaces JSON string passing, eliminating serialization overhead.
 * Fields are converted from `RequestData` using a direct `From` impl.
 */
export interface HandlerInput {
	/** HTTP method (GET, POST, etc.) */
	method: string;
	/** Request path */
	path: string;
	/** HTTP headers as a map */
	headers: Record<string, string>;
	/** HTTP cookies as a map */
	cookies: Record<string, string>;
	/** Parsed query parameters */
	queryParams: Record<string, unknown>;
	/** Parsed request body */
	body: unknown;
	/** Extracted path parameters */
	pathParams: Record<string, string>;
}

/**
 * Structured handler output returned from JavaScript handlers
 *
 * This struct is returned directly from handlers without JSON serialization,
 * completing the zero-copy request/response pattern.
 */
export interface HandlerOutput {
	/** HTTP status code (e.g., 200, 404, 500) */
	status: number;
	/** Response headers as a map */
	headers?: Record<string, string>;
	/** Response body as JSON value */
	body?: unknown;
}

/**
 * Start the Spikard HTTP server from Node.js
 *
 * Creates an Axum HTTP server in a dedicated background thread with its own Tokio runtime.
 * This ensures the Node.js event loop remains free to process ThreadsafeFunction calls.
 *
 * # Arguments
 *
 * * `app` - Application object containing routes and handler functions
 * * `config` - Optional ServerConfig with all middleware settings
 *
 * # Returns
 *
 * Returns `Ok(())` after the server thread is spawned. Note that this function
 * returns immediately - the server runs in the background.
 *
 * # Errors
 *
 * Returns an error if:
 * - Route metadata is invalid or missing required fields
 * - Handler functions cannot be converted to ThreadsafeFunctions
 * - Socket address is invalid
 * - Route creation fails
 *
 * # Example
 *
 * ```typescript
 * import { Spikard, ServerConfig } from 'spikard';
 *
 * const config: ServerConfig = {
 *   host: '0.0.0.0',
 *   port: 8000,
 *   compression: { quality: 9 },
 *   openapi: {
 *     enabled: true,
 *     title: 'My API',
 *     version: '1.0.0'
 *   }
 * };
 *
 * const app = new Spikard();
 * app.run(config);
 * ```
 */
export declare function runServer(app: object, config?: object | undefined | null): void;

/**
 * Optional configuration for a streaming response.
 *
 * This struct is exposed to JavaScript via napi and provides configuration
 * options when creating streaming responses from async iterators.
 *
 * NOTE: Marked with #[allow(dead_code)] because the #[napi(object)] macro
 * generates access patterns that aren't visible to the Rust dead code checker,
 * though the struct is actually exposed to and used by JavaScript code.
 */
export interface StreamingResponseInit {
	/** HTTP status code for the streaming response (default 200). */
	statusCode?: number;
	/** Headers to attach to the streaming response. */
	headers?: Record<string, string>;
}
