import type { Request } from "./request";
import type { StreamingResponse } from "./streaming";

export type JsonPrimitive = string | number | boolean | null;

export type JsonValue = JsonPrimitive | JsonValue[] | { [Key in string]: JsonValue };

export type JsonRecord = Record<string, JsonValue>;

export type BinaryLike = ArrayBuffer | ArrayBufferView | Uint8Array | Blob;

export type MaybePromise<T> = T | Promise<T>;

export interface AbortSignalLike {
	readonly aborted: boolean;
	addEventListener(type: "abort", listener: () => void, options?: { once?: boolean } | boolean): void;
	removeEventListener(type: "abort", listener: () => void): void;
}

export interface HandlerContext {
	signal?: AbortSignalLike;
}

export interface Base64EncodedBody {
	__spikard_base64__: string;
}

export type HandlerBody = JsonValue | Base64EncodedBody | null;

export interface StructuredHandlerResponse {
	status?: number;
	statusCode?: number;
	headers?: Record<string, string>;
	body?: HandlerBody;
}

export type HandlerPayload = Request | JsonValue | string | BinaryLike | null | undefined;

export type HandlerResult = StructuredHandlerResponse | JsonValue | StreamingResponse | undefined;

export type HandlerFunction<TReturn extends HandlerResult = HandlerResult> = (
	payload: HandlerPayload,
	context?: HandlerContext,
) => MaybePromise<TReturn>;

export interface WebSocketServerSocket {
	sendText(message: string): MaybePromise<void>;
	sendJson(payload: JsonValue): MaybePromise<void>;
	sendBytes?(payload: BinaryLike): MaybePromise<void>;
	close(code?: number, reason?: string): MaybePromise<void>;
	broadcast?(payload: JsonValue | string): MaybePromise<void>;
	isClosed(): boolean;
}

export interface WebSocketHandler {
	onOpen?: (socket: WebSocketServerSocket) => MaybePromise<void>;
	onMessage?: (socket: WebSocketServerSocket, data: JsonValue | string | BinaryLike) => MaybePromise<void>;
	onClose?: (socket: WebSocketServerSocket, code?: number, reason?: string) => MaybePromise<void>;
}

export type WebSocketHandlerLike = WebSocketHandler | HandlerFunction;
