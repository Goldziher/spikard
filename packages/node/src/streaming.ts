import { createStreamingHandle } from "../index";
import type { HandlerResult, JsonValue } from "./types";

const STREAM_HANDLE_PROP = "__spikard_stream_handle" as const;

type StreamChunk = JsonValue | string | Buffer | Uint8Array | ArrayBuffer | ArrayBufferView | null | undefined;

type ChunkIterator = AsyncIterator<StreamChunk>;

type StreamingHandleFactory = (iterator: ChunkIterator, init: StreamingResponseInit) => number;

const createHandle: StreamingHandleFactory = createStreamingHandle as StreamingHandleFactory;

export interface StreamingResponseInit {
	statusCode?: number;
	headers?: Record<string, string>;
}

export class StreamingResponse {
	public readonly [STREAM_HANDLE_PROP]: number;

	constructor(stream: AsyncIterable<StreamChunk> | AsyncIterator<StreamChunk>, init?: StreamingResponseInit) {
		const iterator = toAsyncIterator(stream);
		this[STREAM_HANDLE_PROP] = createHandle(iterator, init ?? {});
	}
}

export function isStreamingResponse(value: HandlerResult): value is StreamingResponse {
	return Boolean(value) && value instanceof StreamingResponse;
}

function toAsyncIterator(source: AsyncIterable<StreamChunk> | AsyncIterator<StreamChunk>): ChunkIterator {
	if (source && typeof (source as ChunkIterator).next === "function") {
		return source as ChunkIterator;
	}
	if (source && typeof (source as AsyncIterable<StreamChunk>)[Symbol.asyncIterator] === "function") {
		return (source as AsyncIterable<StreamChunk>)[Symbol.asyncIterator]();
	}
	throw new TypeError("StreamingResponse requires an async iterator or generator");
}
