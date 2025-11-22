/**
 * File upload handling for multipart/form-data requests
 *
 * This module provides the UploadFile class for handling file uploads,
 * designed to be compatible with FastAPI and Litestar patterns while
 * optimized for Spikard's Rust-backed request processing.
 */

/**
 * Represents an uploaded file from multipart/form-data requests
 *
 * This class provides both sync and async interfaces for file operations,
 * with automatic buffer management for efficient memory usage.
 *
 * @example
 * ```typescript
 * import { Spikard, type UploadFile } from '@spikard/node';
 *
 * const app = new Spikard();
 *
 * interface UploadRequest {
 *   file: UploadFile;
 *   description: string;
 * }
 *
 * app.post('/upload', async ({ body }: { body: UploadRequest }) => {
 *   const content = body.file.read();
 *   return {
 *     filename: body.file.filename,
 *     size: body.file.size,
 *     contentType: body.file.contentType,
 *     description: body.description,
 *   };
 * });
 * ```
 */
export class UploadFile {
	/** Original filename from the client */
	readonly filename: string;

	/** MIME type of the uploaded file */
	readonly contentType: string;

	/** Size of the file in bytes */
	readonly size: number;

	/** Additional headers associated with this file field */
	readonly headers: Record<string, string>;

	/** Internal buffer storing file contents */
	private readonly _content: Buffer;

	/** Current read position in the buffer */
	private _position: number = 0;

	/**
	 * Create a new UploadFile instance
	 *
	 * @param filename - Original filename from the client
	 * @param content - File contents as Buffer
	 * @param contentType - MIME type (defaults to "application/octet-stream")
	 * @param size - File size in bytes (computed from content if not provided)
	 * @param headers - Additional headers from the multipart field
	 */
	constructor(
		filename: string,
		content: Buffer,
		contentType: string | null = null,
		size: number | null = null,
		headers: Record<string, string> | null = null,
	) {
		this.filename = filename;
		this.contentType = contentType ?? "application/octet-stream";
		this.size = size ?? content.length;
		this.headers = headers ?? {};
		this._content = content;
	}

	/**
	 * Read file contents synchronously
	 *
	 * @param size - Number of bytes to read (-1 for all remaining)
	 * @returns File contents as Buffer
	 */
	read(size: number = -1): Buffer {
		if (size === -1) {
			const result = this._content.subarray(this._position);
			this._position = this._content.length;
			return result;
		}

		const end = Math.min(this._position + size, this._content.length);
		const result = this._content.subarray(this._position, end);
		this._position = end;
		return result;
	}

	/**
	 * Read file contents asynchronously
	 *
	 * Since the file is already in memory from Rust parsing, this is a simple wrapper.
	 *
	 * @param size - Number of bytes to read (-1 for all remaining)
	 * @returns File contents as Buffer
	 */
	async readAsync(size: number = -1): Promise<Buffer> {
		return this.read(size);
	}

	/**
	 * Read entire file as UTF-8 text
	 *
	 * @returns File contents as string
	 */
	text(): string {
		return this._content.toString("utf-8");
	}

	/**
	 * Read entire file as UTF-8 text asynchronously
	 *
	 * @returns File contents as string
	 */
	async textAsync(): Promise<string> {
		return this.text();
	}

	/**
	 * Seek to a position in the file
	 *
	 * @param offset - Position to seek to
	 * @param whence - How to interpret offset (0=absolute, 1=relative, 2=from end)
	 * @returns New absolute position
	 */
	seek(offset: number, whence: number = 0): number {
		switch (whence) {
			case 0: // Absolute
				this._position = Math.max(0, Math.min(offset, this._content.length));
				break;
			case 1: // Relative
				this._position = Math.max(0, Math.min(this._position + offset, this._content.length));
				break;
			case 2: // From end
				this._position = Math.max(0, Math.min(this._content.length + offset, this._content.length));
				break;
			default:
				throw new Error(`Invalid whence value: ${whence}`);
		}
		return this._position;
	}

	/**
	 * Seek to a position in the file asynchronously
	 *
	 * @param offset - Position to seek to
	 * @param whence - How to interpret offset (0=absolute, 1=relative, 2=from end)
	 * @returns New absolute position
	 */
	async seekAsync(offset: number, whence: number = 0): Promise<number> {
		return this.seek(offset, whence);
	}

	/**
	 * Get current position in the file
	 *
	 * @returns Current byte position
	 */
	tell(): number {
		return this._position;
	}

	/**
	 * Get the underlying Buffer
	 *
	 * @returns Complete file contents as Buffer
	 */
	getBuffer(): Buffer {
		return this._content;
	}

	/**
	 * Close the file (no-op for in-memory files, provided for API compatibility)
	 */
	close(): void {
		// No-op for in-memory buffers, but provided for API compatibility
	}

	/**
	 * Close the file asynchronously (no-op, provided for API compatibility)
	 */
	async closeAsync(): Promise<void> {
		// No-op for in-memory buffers
	}

	/**
	 * String representation of the upload file
	 */
	toString(): string {
		return `UploadFile(filename=${JSON.stringify(this.filename)}, contentType=${JSON.stringify(this.contentType)}, size=${this.size})`;
	}

	/**
	 * JSON representation for debugging
	 */
	toJSON(): Record<string, unknown> {
		return {
			filename: this.filename,
			contentType: this.contentType,
			size: this.size,
			headers: this.headers,
		};
	}
}
