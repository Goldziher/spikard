import { createRequest, type NativeRequestData, type Request } from "./request";
import { isStreamingResponse } from "./streaming";
import type { HandlerFunction, HandlerResult, MaybePromise, NativeHandlerFunction } from "./types";

const NATIVE_HANDLER_FLAG = Symbol("spikard.nativeHandler");

type SerializableResult = HandlerResult | string | undefined;

/**
 * Convert handler result to the format expected by the Rust runtime.
 * When `objectMode` is true (input was an object), return objects directly.
 * When `objectMode` is false (input was JSON string), serialize to JSON strings.
 *
 * Optimized for the common case: objectMode + result with status field.
 * Hot path checks status first before invoking isStreamingResponse.
 */
const formatHandlerResult = (result: SerializableResult, objectMode: boolean): HandlerResult | string => {
	// Fast path: objectMode with status field (most common for NAPI handlers)
	if (objectMode) {
		if (result !== undefined && result !== null && typeof result === "object") {
			// Check for status field first (hot path for HandlerOutput format)
			if ("status" in result) {
				return result as HandlerResult;
			}
			// Only check streaming if result is an object without status
			if (isStreamingResponse(result)) {
				return result;
			}
			// Wrap plain objects as body with default status
			return { status: 200, body: result as unknown as null };
		}
		// Handle undefined/null in objectMode
		return result === undefined ? { status: 200, body: null } : { status: 200, body: result as unknown as null };
	}

	// Legacy string mode: check streaming first, then serialize
	if (isStreamingResponse(result)) {
		return result;
	}
	if (result === undefined) {
		return "null";
	}
	if (typeof result === "string") {
		return result;
	}
	return JSON.stringify(result);
};

export const isNativeHandler = (handler: unknown): handler is NativeHandlerFunction =>
	Boolean((handler as { [NATIVE_HANDLER_FLAG]?: boolean })?.[NATIVE_HANDLER_FLAG]);

function markNative(handler: NativeHandlerFunction): NativeHandlerFunction {
	(handler as { [NATIVE_HANDLER_FLAG]?: boolean })[NATIVE_HANDLER_FLAG] = true;
	return handler;
}

function markRawBody(handler: NativeHandlerFunction, prefersRaw: boolean): NativeHandlerFunction {
	(handler as { __spikard_raw_body?: boolean }).__spikard_raw_body = prefersRaw;
	return handler;
}

/**
 * Unwrap array-wrapped input from ThreadsafeFunction callback and create request.
 * Shared between sync and async wrappers.
 */
function prepareRequest(requestInput: unknown): { request: Request; objectMode: boolean } {
	let actualInput = requestInput;
	if (Array.isArray(requestInput) && requestInput.length === 1) {
		actualInput = requestInput[0];
	}

	const objectMode = typeof actualInput === "object" && actualInput !== null;
	const data: NativeRequestData = objectMode
		? (actualInput as NativeRequestData)
		: (JSON.parse(actualInput as string) as NativeRequestData);
	const request = createRequest(data);

	return { request, objectMode };
}

export function wrapHandler(handler: HandlerFunction): NativeHandlerFunction {
	// Detect if the original handler is async.
	// Sync handlers use a regular function wrapper, avoiding Promise creation overhead.
	// This allows Rust to use the sync ThreadsafeFunction path (1 await vs 2).
	const isHandlerAsync = handler.constructor.name === "AsyncFunction";

	if (isHandlerAsync) {
		const nativeHandler: NativeHandlerFunction = async (requestInput: unknown) => {
			const { request, objectMode } = prepareRequest(requestInput);
			const result = await handler(request);
			return formatHandlerResult(result, objectMode);
		};
		return markNative(markRawBody(nativeHandler, true));
	}

	// Sync path: regular function avoids Promise overhead on Rust side
	const nativeHandler: NativeHandlerFunction = (requestInput: unknown) => {
		const { request, objectMode } = prepareRequest(requestInput);
		const result = handler(request);
		return formatHandlerResult(result as SerializableResult, objectMode);
	};
	return markNative(markRawBody(nativeHandler, true));
}

export function wrapBodyHandler<TBody = unknown>(
	handler: (body: TBody, request: Request) => MaybePromise<HandlerResult>,
): NativeHandlerFunction {
	const isHandlerAsync = handler.constructor.name === "AsyncFunction";

	if (isHandlerAsync) {
		const nativeHandler: NativeHandlerFunction = async (requestInput: unknown) => {
			const { request, objectMode } = prepareRequest(requestInput);
			const body = request.json<TBody>();
			const result = await handler(body, request);
			return formatHandlerResult(result, objectMode);
		};
		return markNative(markRawBody(nativeHandler, true));
	}

	const nativeHandler: NativeHandlerFunction = (requestInput: unknown) => {
		const { request, objectMode } = prepareRequest(requestInput);
		const body = request.json<TBody>();
		const result = handler(body, request);
		return formatHandlerResult(result as SerializableResult, objectMode);
	};
	return markNative(markRawBody(nativeHandler, true));
}
