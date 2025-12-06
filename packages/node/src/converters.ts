/**
 * Type conversion utilities for handler parameters
 *
 * This module handles converting validated JSON data from Rust into TypeScript types,
 * particularly for UploadFile instances.
 */

import type { JsonValue } from "./types";
import { UploadFile } from "./upload";

/**
 * File metadata structure from Rust multipart parsing
 */
export interface FileMetadata {
	filename: string;
	content: string; // base64 encoded or raw string
	size?: number | undefined;
	content_type?: string | undefined;
	content_encoding?: "base64" | "text" | undefined;
}

/**
 * Check if a value looks like file metadata from Rust
 */
function isFileMetadata(value: unknown): value is FileMetadata {
	return typeof value === "object" && value !== null && "filename" in value && "content" in value;
}

/**
 * Convert file metadata JSON to UploadFile instance
 *
 * @param fileData - File metadata from Rust (filename, content, size, content_type)
 * @returns UploadFile instance
 */
export function convertFileMetadataToUploadFile(fileData: FileMetadata): UploadFile {
	const { filename, content, size, content_type, content_encoding } = fileData;

	// Convert content string to Buffer
	let buffer: Buffer;
	if (content_encoding === "base64") {
		// Explicitly marked as base64 encoded binary content
		buffer = Buffer.from(content, "base64");
	} else {
		// Treat as plain text by default (including test payloads)
		buffer = Buffer.from(content, "utf-8");
	}

	return new UploadFile(filename, buffer, content_type ?? null, size ?? null);
}

/**
 * Process handler parameters, converting file metadata to UploadFile instances
 *
 * This function recursively processes the body parameter, looking for file metadata
 * structures and converting them to UploadFile instances.
 *
 * @param value - The value to process (can be object, array, or primitive)
 * @returns Processed value with UploadFile instances
 */
export function processUploadFileFields(value: JsonValue | undefined): unknown {
	// Handle null/undefined
	if (value === null || value === undefined) {
		return value;
	}

	// Handle primitives (string, number, boolean)
	if (typeof value !== "object") {
		return value;
	}

	// Handle arrays - recursively process each element
	if (Array.isArray(value)) {
		return value.map((item) => {
			// Check if this array item is file metadata
			if (isFileMetadata(item)) {
				return convertFileMetadataToUploadFile(item);
			}
			// Recursively process nested arrays/objects
			return processUploadFileFields(item as JsonValue);
		});
	}

	// Handle objects - check if it's file metadata first
	if (isFileMetadata(value)) {
		return convertFileMetadataToUploadFile(value);
	}

	// Otherwise, recursively process object properties
	const result: Record<string, unknown> = {};
	for (const [key, val] of Object.entries(value)) {
		result[key] = processUploadFileFields(val as JsonValue);
	}

	return result;
}

/**
 * Convert __spikard_multipart__ test payload to handler body
 * This merges files and fields into a single object with UploadFile instances
 */
function convertMultipartTestPayload(payload: {
	fields?: Record<string, unknown>;
	files?: Array<{ name: string; filename?: string; content: string; contentType?: string }>;
}): unknown {
	const result: Record<string, unknown> = {};

	// Add fields
	if (payload.fields) {
		Object.assign(result, payload.fields);
	}

	// Add files, grouping by field name
	if (payload.files && payload.files.length > 0) {
		const filesByName: Record<string, unknown[]> = {};

		for (const file of payload.files) {
			const fileMetadata: FileMetadata = {
				filename: file.filename || file.name,
				content: file.content,
				content_type: file.contentType,
			};
			const uploadFile = convertFileMetadataToUploadFile(fileMetadata);

			if (!filesByName[file.name]) {
				filesByName[file.name] = [];
			}
			const files = filesByName[file.name];
			if (files) {
				files.push(uploadFile);
			}
		}

		// Flatten single files, keep arrays for multiple files with same name
		for (const [name, files] of Object.entries(filesByName)) {
			result[name] = files.length === 1 ? files[0] : files;
		}
	}

	return result;
}

/**
 * Process handler body parameter, handling UploadFile conversion
 *
 * This is the main entry point for converting Rust-provided request data
 * into TypeScript types. It handles:
 * - Single UploadFile
 * - Arrays of UploadFile
 * - Objects with UploadFile fields
 * - Nested structures
 * - TestClient multipart payloads
 *
 * @param body - The body parameter from Rust (already JSON-parsed)
 * @returns Processed body with UploadFile instances
 */
export function convertHandlerBody(body: JsonValue | undefined): unknown {
	// Handle TestClient multipart payload
	if (
		typeof body === "object" &&
		body !== null &&
		"__spikard_multipart__" in body &&
		typeof (body as Record<string, unknown>).__spikard_multipart__ === "object"
	) {
		const multipart = (body as Record<string, unknown>).__spikard_multipart__ as {
			fields?: Record<string, unknown>;
			files?: Array<{ name: string; filename?: string; content: string; contentType?: string }>;
		};
		return convertMultipartTestPayload(multipart);
	}

	return processUploadFileFields(body);
}
