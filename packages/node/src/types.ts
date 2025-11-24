import type { Request } from "./request";
import type { StreamingResponse } from "./streaming";

export type JsonPrimitive = string | number | boolean | null;

export type JsonValue = JsonPrimitive | JsonValue[] | { [Key in string]: JsonValue };

export type JsonRecord = Record<string, JsonValue>;

export type BinaryLike = ArrayBuffer | ArrayBufferView | Uint8Array;

export type MaybePromise<T> = T | Promise<T>;

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

export type HandlerResult = StructuredHandlerResponse | JsonValue | StreamingResponse | undefined;

export type HandlerFunction<TReturn extends HandlerResult = HandlerResult> = (
	request: Request,
) => MaybePromise<TReturn>;

export type NativeHandlerFunction<TReturn extends HandlerResult = HandlerResult> = (
	requestJson: string,
) => MaybePromise<TReturn | string>;
