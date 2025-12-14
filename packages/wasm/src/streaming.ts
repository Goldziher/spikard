import type { HandlerResult, JsonValue } from "./types";

const STREAM_HANDLE_PROP = Symbol.for("spikard.streaming.handle");

type StreamChunk = JsonValue | string | Uint8Array | ArrayBuffer | ArrayBufferView | null | undefined;

type AsyncIteratorLike<T> = AsyncIterator<T> & AsyncIterable<T>;

export interface StreamingResponseInit {
	statusCode?: number;
	headers?: Record<string, string>;
}

export class StreamingResponse {
	public readonly statusCode: number;
	public readonly headers: Record<string, string>;
	public readonly [STREAM_HANDLE_PROP]: AsyncIteratorLike<StreamChunk>;
	public readonly __spikard_streaming__: true;

	constructor(stream: AsyncIterable<StreamChunk> | AsyncIterator<StreamChunk>, init?: StreamingResponseInit) {
		this[STREAM_HANDLE_PROP] = toAsyncIterator(stream);
		this.statusCode = init?.statusCode ?? 200;
		this.headers = init?.headers ?? {};
		this.__spikard_streaming__ = true;
	}

	async collect(): Promise<Uint8Array> {
		const chunks: Uint8Array[] = [];
		for await (const chunk of this[STREAM_HANDLE_PROP]) {
			chunks.push(normalizeChunk(chunk));
		}
		return concatChunks(chunks);
	}
}

export function isStreamingResponse(value: HandlerResult): value is StreamingResponse {
	return Boolean(value) && value instanceof StreamingResponse;
}

function toAsyncIterator(
	source: AsyncIterable<StreamChunk> | AsyncIterator<StreamChunk>,
): AsyncIteratorLike<StreamChunk> {
	if (!source || typeof source !== "object") {
		throw new TypeError("StreamingResponse requires an async iterator or generator");
	}
	if (typeof (source as AsyncIterator<StreamChunk>).next === "function") {
		const iterator = source as AsyncIteratorLike<StreamChunk>;
		if (typeof iterator[Symbol.asyncIterator] === "function") {
			return iterator;
		}
		return wrapIterator(iterator);
	}
	if (typeof (source as AsyncIterable<StreamChunk>)[Symbol.asyncIterator] === "function") {
		return (source as AsyncIterable<StreamChunk>)[Symbol.asyncIterator]() as AsyncIteratorLike<StreamChunk>;
	}
	throw new TypeError("StreamingResponse requires an async iterator or generator");
}

function wrapIterator(iterator: AsyncIterator<StreamChunk>): AsyncIteratorLike<StreamChunk> {
	return {
		next: iterator.next.bind(iterator),
		...(iterator.throw ? { throw: iterator.throw.bind(iterator) } : {}),
		...(iterator.return ? { return: iterator.return.bind(iterator) } : {}),
		[Symbol.asyncIterator]() {
			return this;
		},
	};
}

function normalizeChunk(chunk: unknown): Uint8Array {
	if (typeof chunk === "string") {
		return new TextEncoder().encode(chunk);
	}
	if (chunk instanceof Uint8Array) {
		return chunk;
	}
	if (ArrayBuffer.isView(chunk)) {
		const view = chunk as ArrayBufferView;
		return new Uint8Array(view.buffer.slice(view.byteOffset, view.byteOffset + view.byteLength));
	}
	if (chunk instanceof ArrayBuffer) {
		return new Uint8Array(chunk);
	}
	if (chunk == null) {
		return new Uint8Array();
	}
	return new TextEncoder().encode(typeof chunk === "object" ? JSON.stringify(chunk) : String(chunk));
}

function concatChunks(chunks: Uint8Array[]): Uint8Array {
	if (chunks.length === 1) {
		return chunks[0] ?? new Uint8Array();
	}

	const totalLength = chunks.reduce((sum, chunk) => sum + chunk.length, 0);
	const result = new Uint8Array(totalLength);
	let offset = 0;
	for (const chunk of chunks) {
		result.set(chunk, offset);
		offset += chunk.length;
	}
	return result;
}
