import type { HandlerResult, JsonValue } from "./types";

export interface StreamingResponseInit {
	statusCode?: number;
	headers?: Record<string, string>;
}

const STREAM_HANDLE_PROP = "__spikard_stream_handle" as const;

type StreamChunk = JsonValue | string | Buffer | Uint8Array | ArrayBuffer | ArrayBufferView | null | undefined;

type ChunkIterator = AsyncIterator<StreamChunk>;

type StreamingHandleFactory = (iterator: ChunkIterator, init: StreamingResponseInit) => number;

interface NativeStreamingBinding {
	createStreamingHandle(iterator: ChunkIterator, init: StreamingResponseInit): number;
}

let nativeBinding: NativeStreamingBinding;

const loadBinding = (): NativeStreamingBinding => {
	try {
		return require("../spikard-node.darwin-arm64.node") as NativeStreamingBinding;
	} catch {
		try {
			return require("../spikard-node.node") as NativeStreamingBinding;
		} catch {
			console.warn("[spikard-node] Native binding not found. Please run: pnpm build:native");
			return {
				createStreamingHandle: () => {
					throw new Error("Native binding not built. Run: pnpm build:native");
				},
			};
		}
	}
};

nativeBinding = loadBinding();

const createHandle: StreamingHandleFactory = (iterator, init) => nativeBinding.createStreamingHandle(iterator, init);

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
