import { createRequest, type NativeRequestData, type Request } from "./request";
import { isStreamingResponse } from "./streaming";
import type { HandlerFunction, HandlerResult, MaybePromise, NativeHandlerFunction } from "./types";

const NATIVE_HANDLER_FLAG = Symbol("spikard.nativeHandler");

type SerializableResult = HandlerResult | string | undefined;

const serializeHandlerResult = (result: SerializableResult): string | HandlerResult => {
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

export function wrapHandler(handler: HandlerFunction): NativeHandlerFunction {
	const nativeHandler: NativeHandlerFunction = async (requestJson: string) => {
		const data = JSON.parse(requestJson) as NativeRequestData;
		const request = createRequest(data);
		const result = await handler(request);
		return serializeHandlerResult(result);
	};

	return markNative(nativeHandler);
}

export function wrapBodyHandler<TBody = unknown>(
	handler: (body: TBody, request: Request) => MaybePromise<HandlerResult>,
): NativeHandlerFunction {
	const nativeHandler: NativeHandlerFunction = async (requestJson: string) => {
		const data = JSON.parse(requestJson) as NativeRequestData;
		const request = createRequest(data);
		const body = request.json<TBody>();
		const result = await handler(body, request);
		return serializeHandlerResult(result);
	};

	return markNative(nativeHandler);
}
