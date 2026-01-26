import { createRequest, type NativeRequestData, type Request } from "./request";
import { isStreamingResponse } from "./streaming";
import type { HandlerFunction, HandlerResult, MaybePromise, NativeHandlerFunction } from "./types";

const NATIVE_HANDLER_FLAG = Symbol("spikard.nativeHandler");

type SerializableResult = HandlerResult | string | undefined;

/**
 * Convert handler result to the format expected by the Rust runtime.
 * When `objectMode` is true (input was an object), return objects directly.
 * When `objectMode` is false (input was JSON string), serialize to JSON strings.
 */
const formatHandlerResult = (result: SerializableResult, objectMode: boolean): HandlerResult | string => {
	if (isStreamingResponse(result)) {
		return result;
	}

	// In object mode (new napi path), return objects directly for HandlerOutput conversion
	if (objectMode) {
		if (result === undefined) {
			return { status: 200, body: null };
		}
		// If result already has status field, it's in HandlerOutput format - return as-is
		if (typeof result === "object" && result !== null && "status" in result) {
			return result as HandlerResult;
		}
		// Otherwise wrap the result as body with default status
		// Cast to unknown first to satisfy TypeScript's strict type checking
		return { status: 200, body: result as unknown as null };
	}

	// In string mode (legacy JSON path), serialize everything
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

export function wrapHandler(handler: HandlerFunction): NativeHandlerFunction {
	const nativeHandler: NativeHandlerFunction = async (requestInput: unknown) => {
		// Handle the case where ThreadsafeFunction callback wraps the value in an array
		// e.g., Rust `.build_callback(|ctx| Ok(vec![ctx.value]))` produces [jsonString]
		let actualInput = requestInput;
		if (Array.isArray(requestInput) && requestInput.length === 1) {
			actualInput = requestInput[0];
		}

		// Detect if Rust is passing objects directly (new napi path) or JSON strings (legacy path)
		const objectMode = typeof actualInput === "object" && actualInput !== null;
		const data: NativeRequestData = objectMode
			? (actualInput as NativeRequestData)
			: (JSON.parse(actualInput as string) as NativeRequestData);
		const request = createRequest(data);
		const result = await handler(request);
		return formatHandlerResult(result, objectMode);
	};

	return markNative(markRawBody(nativeHandler, true));
}

export function wrapBodyHandler<TBody = unknown>(
	handler: (body: TBody, request: Request) => MaybePromise<HandlerResult>,
): NativeHandlerFunction {
	const nativeHandler: NativeHandlerFunction = async (requestInput: unknown) => {
		// Handle the case where ThreadsafeFunction callback wraps the value in an array
		let actualInput = requestInput;
		if (Array.isArray(requestInput) && requestInput.length === 1) {
			actualInput = requestInput[0];
		}

		// Detect if Rust is passing objects directly (new napi path) or JSON strings (legacy path)
		const objectMode = typeof actualInput === "object" && actualInput !== null;
		const data: NativeRequestData = objectMode
			? (actualInput as NativeRequestData)
			: (JSON.parse(actualInput as string) as NativeRequestData);
		const request = createRequest(data);
		const body = request.json<TBody>();
		const result = await handler(body, request);
		return formatHandlerResult(result, objectMode);
	};

	return markNative(markRawBody(nativeHandler, true));
}
