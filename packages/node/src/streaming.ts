import { createStreamingHandle } from "../index.js";

const STREAM_HANDLE_PROP = "__spikard_stream_handle" as const;

export interface StreamingResponseInit {
	statusCode?: number;
	headers?: Record<string, string>;
}

export class StreamingResponse {
	public readonly [STREAM_HANDLE_PROP]: number;

	constructor(stream: AsyncIterable<unknown> | AsyncGenerator<unknown>, init?: StreamingResponseInit) {
		const iterator = toAsyncIterator(stream);
		this[STREAM_HANDLE_PROP] = createStreamingHandle(iterator, init ?? {});
	}
}

function toAsyncIterator(source: any): AsyncIterator<unknown> {
	if (source && typeof source.next === "function") {
		return source as AsyncIterator<unknown>;
	}
	if (source && typeof source[Symbol.asyncIterator] === "function") {
		return source[Symbol.asyncIterator]();
	}
	throw new TypeError("StreamingResponse requires an async iterator or generator");
}
