import { createRequire } from "node:module";
import type { HandlerResult, JsonValue } from "./types";

export interface StreamingResponseInit {
	statusCode?: number;
	headers?: Record<string, string>;
}

const STREAM_HANDLE_PROP = "__spikard_stream_handle" as const;

type StreamChunk = JsonValue | string | Buffer | Uint8Array | ArrayBuffer | ArrayBufferView | null | undefined;

type ChunkIterator = AsyncIterator<StreamChunk> & AsyncIterable<StreamChunk>;

type StreamingHandle =
	| { kind: "native"; handle: number; init: StreamingResponseInit }
	| { kind: "js"; iterator: ChunkIterator; init: StreamingResponseInit };

type StreamingHandleFactory = (iterator: ChunkIterator, init: StreamingResponseInit) => StreamingHandle;

interface NativeStreamingBinding {
	createStreamingHandle(iterator: ChunkIterator, init: StreamingResponseInit): number;
}

let nativeBinding: NativeStreamingBinding | null = null;

const loadBinding = (): NativeStreamingBinding | null => {
	try {
		// createRequire allows us to require CommonJS modules from ESM context
		// This is necessary to load the NAPI binding which is a .node file loaded via CommonJS
		const require = createRequire(import.meta.url);
		return require("../index.js") as NativeStreamingBinding;
	} catch {
		console.warn("[spikard-node] Native binding not found. Please run: pnpm build:native");
		return null;
	}
};

nativeBinding = loadBinding();

const createHandle: StreamingHandleFactory = (iterator, init) => {
	if (nativeBinding && typeof nativeBinding.createStreamingHandle === "function") {
		return { kind: "native", handle: nativeBinding.createStreamingHandle(iterator, init), init };
	}
	return { kind: "js", iterator, init };
};

export class StreamingResponse {
	public readonly [STREAM_HANDLE_PROP]: StreamingHandle;

	constructor(stream: AsyncIterable<StreamChunk> | AsyncIterator<StreamChunk>, init?: StreamingResponseInit) {
		const iterator = toAsyncIterator(stream);
		this[STREAM_HANDLE_PROP] = createHandle(iterator, init ?? {});
	}
}

export function isStreamingResponse(value: HandlerResult): value is StreamingResponse {
	return Boolean(value) && value instanceof StreamingResponse;
}

export const getStreamingHandle = (response: StreamingResponse): StreamingHandle => response[STREAM_HANDLE_PROP];

function toAsyncIterator(source: AsyncIterable<StreamChunk> | AsyncIterator<StreamChunk>): ChunkIterator {
	if (source && typeof (source as AsyncIterator<StreamChunk>).next === "function") {
		const iterator = source as AsyncIterator<StreamChunk> & Partial<AsyncIterable<StreamChunk>>;
		if (typeof iterator[Symbol.asyncIterator] === "function") {
			return iterator as ChunkIterator;
		}
		return {
			next: (...args) => iterator.next(...args),
			[Symbol.asyncIterator]() {
				return this;
			},
		} as ChunkIterator;
	}
	if (source && typeof (source as AsyncIterable<StreamChunk>)[Symbol.asyncIterator] === "function") {
		return (source as AsyncIterable<StreamChunk>)[Symbol.asyncIterator]() as ChunkIterator;
	}
	throw new TypeError("StreamingResponse requires an async iterator or generator");
}
