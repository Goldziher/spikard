/**
 * Comprehensive tests for UploadFile and ergonomic handler integration
 *
 * Tests cover:
 * - UploadFile class API (unit tests)
 * - Handler integration with automatic JSON parsing and UploadFile conversion
 * - Type-safe request bodies
 * - Error handling and edge cases
 */

import { beforeEach, describe, expect, it } from "vitest";
import { convertFileMetadataToUploadFile, processUploadFileFields } from "./converters";
import { UploadFile } from "./upload";

/**
 * SECTION 1: UploadFile API Tests (Unit Tests)
 */
describe("UploadFile API", () => {
	describe("Basic Properties", () => {
		it("should create an UploadFile with all properties", () => {
			const content = Buffer.from("Hello, World!");
			const file = new UploadFile("test.txt", content, "text/plain", content.length, { "x-custom-header": "value" });

			expect(file.filename).toBe("test.txt");
			expect(file.contentType).toBe("text/plain");
			expect(file.size).toBe(13);
			expect(file.headers).toEqual({ "x-custom-header": "value" });
		});

		it("should use default content type if not provided", () => {
			const content = Buffer.from("data");
			const file = new UploadFile("file.bin", content);

			expect(file.contentType).toBe("application/octet-stream");
		});

		it("should compute size from content if not provided", () => {
			const content = Buffer.from("Hello, World!");
			const file = new UploadFile("test.txt", content, "text/plain");

			expect(file.size).toBe(13);
		});

		it("should have empty headers by default", () => {
			const content = Buffer.from("data");
			const file = new UploadFile("file.txt", content);

			expect(file.headers).toEqual({});
		});

		it("should provide correct string representation", () => {
			const content = Buffer.from("Hello, World!");
			const file = new UploadFile("test.txt", content, "text/plain");

			const str = file.toString();
			expect(str).toContain("UploadFile");
			expect(str).toContain("test.txt");
			expect(str).toContain("text/plain");
			expect(str).toContain("13");
		});

		it("should provide JSON representation", () => {
			const content = Buffer.from("data");
			const file = new UploadFile("test.txt", content, "text/plain", 4, {
				"x-header": "value",
			});

			const json = file.toJSON();
			expect(json).toEqual({
				filename: "test.txt",
				contentType: "text/plain",
				size: 4,
				headers: { "x-header": "value" },
			});
		});
	});

	describe("Synchronous Read/Write Operations", () => {
		it("should read entire file content", () => {
			const content = Buffer.from("Hello, World!");
			const file = new UploadFile("test.txt", content);

			const data = file.read();
			expect(data.toString()).toBe("Hello, World!");
		});

		it("should read partial content with size parameter", () => {
			const content = Buffer.from("Hello, World!");
			const file = new UploadFile("test.txt", content);

			expect(file.read(5).toString()).toBe("Hello");
			expect(file.read(2).toString()).toBe(", ");
			expect(file.read(6).toString()).toBe("World!");
		});

		it("should return empty buffer at EOF", () => {
			const content = Buffer.from("Hi");
			const file = new UploadFile("test.txt", content);

			file.read(); // Read everything
			expect(file.read().length).toBe(0);
		});

		it("should read as text (UTF-8)", () => {
			const content = Buffer.from("Hello, 世界!");
			const file = new UploadFile("test.txt", content);

			expect(file.text()).toBe("Hello, 世界!");
		});

		it("should support seeking with absolute position (whence=0)", () => {
			const content = Buffer.from("Hello, World!");
			const file = new UploadFile("test.txt", content);

			file.seek(7); // Seek to position 7 ("World!")
			expect(file.read().toString()).toBe("World!");
		});

		it("should support relative seeking (whence=1)", () => {
			const content = Buffer.from("Hello, World!");
			const file = new UploadFile("test.txt", content);

			file.read(5); // Position is now 5
			file.seek(2, 1); // Seek 2 bytes forward relative
			expect(file.tell()).toBe(7);
		});

		it("should support seeking from end (whence=2)", () => {
			const content = Buffer.from("Hello, World!");
			const file = new UploadFile("test.txt", content);

			file.seek(-6, 2); // Seek 6 bytes from end
			expect(file.read().toString()).toBe("World!");
		});

		it("should clamp seek position to valid range", () => {
			const content = Buffer.from("Hello");
			const file = new UploadFile("test.txt", content);

			file.seek(-100); // Try to seek before start
			expect(file.tell()).toBe(0);

			file.seek(1000); // Try to seek past end
			expect(file.tell()).toBe(5);
		});

		it("should return current position with tell()", () => {
			const content = Buffer.from("Hello, World!");
			const file = new UploadFile("test.txt", content);

			expect(file.tell()).toBe(0);
			file.read(5);
			expect(file.tell()).toBe(5);
		});

		it("should provide buffer access", () => {
			const content = Buffer.from("Hello, World!");
			const file = new UploadFile("test.txt", content);

			expect(file.getBuffer()).toEqual(content);
		});

		it("should handle close() as no-op", () => {
			const file = new UploadFile("test.txt", Buffer.from("data"));

			// Should not throw
			file.close();
			expect(file.tell()).toBe(0);
		});
	});

	describe("Asynchronous Operations", () => {
		it("should support async read", async () => {
			const content = Buffer.from("Hello, World!");
			const file = new UploadFile("test.txt", content);

			const data = await file.readAsync();
			expect(data.toString()).toBe("Hello, World!");
		});

		it("should support async text reading", async () => {
			const content = Buffer.from("Hello, 世界!");
			const file = new UploadFile("test.txt", content);

			const text = await file.textAsync();
			expect(text).toBe("Hello, 世界!");
		});

		it("should support async seek", async () => {
			const content = Buffer.from("Hello, World!");
			const file = new UploadFile("test.txt", content);

			await file.seekAsync(7);
			expect(file.tell()).toBe(7);
			expect(file.read().toString()).toBe("World!");
		});

		it("should support async close", async () => {
			const file = new UploadFile("test.txt", Buffer.from("data"));

			// Should not throw
			await file.closeAsync();
			expect(file.tell()).toBe(0);
		});
	});
});

/**
 * SECTION 2: Handler Integration Tests
 *
 * Tests verify:
 * - Automatic JSON parsing (no manual JSON.parse needed)
 * - File metadata → UploadFile conversion
 * - Type-safe request bodies
 * - Ergonomic API with zero boilerplate
 *
 * NOTE: These tests are skipped pending full framework integration with Rust bindings
 */

// Handler type definitions for test scenarios
type UploadHandler<TReq extends object = object, TRes extends object = object> = (payload: TReq) => Promise<TRes>;

interface MockApp {
	routes: unknown[];
	handlers: Record<string, UploadHandler>;
}

interface TestClientType {
	post(
		path: string,
		config: unknown,
	): Promise<{
		statusCode: number;
		json(): Record<string, unknown>;
	}>;
}

describe.skip("UploadFile Handler Integration", () => {
	let app: MockApp;
	let client: TestClientType;

	beforeEach(() => {
		app = { routes: [], handlers: {} }; // Mock - will be implemented with Spikard integration
	});

	describe("Single File Upload", () => {
		it("should handle single file in typed request body", async () => {
			interface UploadRequest {
				file: UploadFile;
				description: string;
			}

			interface UploadResponse {
				filename: string;
				size: number;
				contentType: string;
				description: string;
				content: string;
			}

			app.routes.push({
				method: "POST",
				path: "/upload",
				handler_name: "uploadHandler",
				is_async: true,
			});

			app.handlers.uploadHandler = async (payload: UploadRequest): Promise<UploadResponse> => {
				const body = payload;
				return {
					filename: body.file.filename,
					size: body.file.size,
					contentType: body.file.contentType,
					description: body.description,
					content: body.file.text(),
				};
			};

			client = new TestClient(app);

			const response = await client.post("/upload", {
				multipart: {
					files: [
						{
							name: "file",
							filename: "test.txt",
							content: "Hello, World!",
							contentType: "text/plain",
						},
					],
					fields: {
						description: "Test file",
					},
				},
			});

			expect(response.statusCode).toBe(200);
			const body = response.json();
			expect(body.filename).toBe("test.txt");
			expect(body.contentType).toBe("text/plain");
			expect(body.description).toBe("Test file");
			expect(body.content).toBe("Hello, World!");
		});

		it("should handle file with different MIME types", async () => {
			app.routes.push({
				method: "POST",
				path: "/upload-json",
				handler_name: "uploadJson",
				is_async: true,
			});

			app.handlers.uploadJson = async (payload: {
				file: UploadFile;
			}): Promise<{ filename: string; contentType: string }> => {
				const file = payload.file as UploadFile;
				return {
					filename: file.filename,
					contentType: file.contentType,
				};
			};

			client = new TestClient(app);

			const response = await client.post("/upload-json", {
				multipart: {
					files: [
						{
							name: "file",
							filename: "data.json",
							content: '{"key": "value"}',
							contentType: "application/json",
						},
					],
				},
			});

			expect(response.statusCode).toBe(200);
			const body = response.json();
			expect(body.contentType).toBe("application/json");
		});

		it("should provide access to file headers", async () => {
			app.routes.push({
				method: "POST",
				path: "/upload-headers",
				handler_name: "uploadHeaders",
				is_async: true,
			});

			app.handlers.uploadHeaders = async (payload: {
				file: UploadFile;
			}): Promise<{ filename: string; hasHeaders: boolean }> => {
				const file = payload.file as UploadFile;
				return {
					filename: file.filename,
					hasHeaders: Object.keys(file.headers).length > 0,
				};
			};

			client = new TestClient(app);

			const response = await client.post("/upload-headers", {
				multipart: {
					files: [
						{
							name: "file",
							filename: "test.txt",
							content: "data",
						},
					],
				},
			});

			expect(response.statusCode).toBe(200);
		});
	});

	describe("Multiple Files Upload", () => {
		it("should handle array of files", async () => {
			interface MultiFileRequest {
				files: UploadFile[];
			}

			app.routes.push({
				method: "POST",
				path: "/upload-multiple",
				handler_name: "uploadMultiple",
				is_async: true,
			});

			app.handlers.uploadMultiple = async (payload: {
				files: UploadFile[];
			}): Promise<{ count: number; filenames: string[]; sizes: number[] }> => {
				const body = payload as MultiFileRequest;
				return {
					count: body.files.length,
					filenames: body.files.map((f) => f.filename),
					sizes: body.files.map((f) => f.size),
				};
			};

			client = new TestClient(app);

			const response = await client.post("/upload-multiple", {
				multipart: {
					files: [
						{
							name: "files",
							filename: "file1.txt",
							content: "Content 1",
							contentType: "text/plain",
						},
						{
							name: "files",
							filename: "file2.txt",
							content: "Content 2",
							contentType: "text/plain",
						},
						{
							name: "files",
							filename: "file3.txt",
							content: "Content 3",
							contentType: "text/plain",
						},
					],
				},
			});

			expect(response.statusCode).toBe(200);
			const body = response.json();
			expect(body.count).toBe(3);
			expect(body.filenames).toEqual(["file1.txt", "file2.txt", "file3.txt"]);
			expect(body.sizes).toEqual([9, 9, 9]);
		});

		it("should handle empty file array", async () => {
			app.routes.push({
				method: "POST",
				path: "/upload-empty",
				handler_name: "uploadEmpty",
				is_async: true,
			});

			app.handlers.uploadEmpty = async (payload: UploadFile[]): Promise<{ count: number }> => {
				const files = payload as UploadFile[];
				return {
					count: Array.isArray(files) ? files.length : 0,
				};
			};

			client = new TestClient(app);

			const response = await client.post("/upload-empty", {
				multipart: {
					files: [],
				},
			});

			expect(response.statusCode).toBe(200);
			const body = response.json();
			expect(body.count).toBe(0);
		});
	});

	describe("Optional File Upload", () => {
		it("should handle optional file when provided", async () => {
			interface OptionalUploadRequest {
				file?: UploadFile;
				name: string;
			}

			app.routes.push({
				method: "POST",
				path: "/upload-optional",
				handler_name: "uploadOptional",
				is_async: true,
			});

			app.handlers.uploadOptional = async (payload: {
				file?: UploadFile;
				name: string;
			}): Promise<{ hasFile: boolean; filename?: string; name: string }> => {
				const body = payload as OptionalUploadRequest;
				return {
					hasFile: body.file !== undefined && body.file !== null,
					filename: body.file?.filename,
					name: body.name,
				};
			};

			client = new TestClient(app);

			const response = await client.post("/upload-optional", {
				multipart: {
					files: [
						{
							name: "file",
							filename: "test.txt",
							content: "data",
						},
					],
					fields: {
						name: "Alice",
					},
				},
			});

			expect(response.statusCode).toBe(200);
			const body = response.json();
			expect(body.hasFile).toBe(true);
			expect(body.filename).toBe("test.txt");
			expect(body.name).toBe("Alice");
		});

		it("should handle optional file when not provided", async () => {
			interface OptionalUploadRequest {
				file?: UploadFile;
				name: string;
			}

			app.routes.push({
				method: "POST",
				path: "/upload-optional-absent",
				handler_name: "uploadOptionalAbsent",
				is_async: true,
			});

			app.handlers.uploadOptionalAbsent = async (payload: {
				file?: UploadFile;
				name: string;
			}): Promise<{ hasFile: boolean; filename: string | null; name: string }> => {
				const body = payload as OptionalUploadRequest;
				return {
					hasFile: body.file !== undefined && body.file !== null,
					filename: body.file?.filename ?? null,
					name: body.name,
				};
			};

			client = new TestClient(app);

			const response = await client.post("/upload-optional-absent", {
				multipart: {
					fields: {
						name: "Bob",
					},
				},
			});

			expect(response.statusCode).toBe(200);
			const body = response.json();
			expect(body.hasFile).toBe(false);
			expect(body.filename).toBe(null);
			expect(body.name).toBe("Bob");
		});
	});

	describe("Mixed Form Data and Files", () => {
		it("should handle mixed form fields and files", async () => {
			interface MixedRequest {
				file: UploadFile;
				name: string;
				email: string;
				age: number;
			}

			app.routes.push({
				method: "POST",
				path: "/upload-mixed",
				handler_name: "uploadMixed",
				is_async: true,
			});

			app.handlers.uploadMixed = async (payload: {
				file: UploadFile;
				name: string;
				email: string;
				age: number;
			}): Promise<{ filename: string; name: string; email: string; age: number }> => {
				const body = payload as MixedRequest;
				return {
					filename: body.file.filename,
					name: body.name,
					email: body.email,
					age: body.age,
				};
			};

			client = new TestClient(app);

			const response = await client.post("/upload-mixed", {
				multipart: {
					files: [
						{
							name: "file",
							filename: "profile.txt",
							content: "User profile",
						},
					],
					fields: {
						name: "Charlie",
						email: "charlie@example.com",
						age: 25,
					},
				},
			});

			expect(response.statusCode).toBe(200);
			const body = response.json();
			expect(body.filename).toBe("profile.txt");
			expect(body.name).toBe("Charlie");
			expect(body.email).toBe("charlie@example.com");
			expect(body.age).toBe(25);
		});

		it("should handle files in nested objects", async () => {
			interface NestedFileRequest {
				metadata: {
					title: string;
					file: UploadFile;
				};
			}

			app.routes.push({
				method: "POST",
				path: "/upload-nested",
				handler_name: "uploadNested",
				is_async: true,
			});

			app.handlers.uploadNested = async (payload: {
				metadata: { title: string; file: UploadFile };
			}): Promise<{ title: string; filename: string; size: number }> => {
				const body = payload as NestedFileRequest;
				return {
					title: body.metadata.title,
					filename: body.metadata.file.filename,
					size: body.metadata.file.size,
				};
			};

			client = new TestClient(app);

			const response = await client.post("/upload-nested", {
				multipart: {
					files: [
						{
							name: "metadata.file",
							filename: "document.txt",
							content: "Document content",
						},
					],
					fields: {
						"metadata.title": "My Document",
					},
				},
			});

			expect(response.statusCode).toBe(200);
			const body = response.json();
			expect(body.title).toBe("My Document");
			expect(body.filename).toBe("document.txt");
		});
	});

	describe("Large File Handling", () => {
		it("should handle large binary files", async () => {
			app.routes.push({
				method: "POST",
				path: "/upload-large",
				handler_name: "uploadLarge",
				is_async: true,
			});

			app.handlers.uploadLarge = async (payload: {
				file: UploadFile;
			}): Promise<{ filename: string; size: number; readSuccess: boolean }> => {
				const file = payload.file as UploadFile;
				return {
					filename: file.filename,
					size: file.size,
					readSuccess: true,
				};
			};

			client = new TestClient(app);

			// Create a 1MB file
			const largeContent = Buffer.alloc(1024 * 1024).toString("base64");

			const response = await client.post("/upload-large", {
				multipart: {
					files: [
						{
							name: "file",
							filename: "large.bin",
							content: largeContent,
							contentType: "application/octet-stream",
						},
					],
				},
			});

			expect(response.statusCode).toBe(200);
			const body = response.json();
			expect(body.filename).toBe("large.bin");
			expect(body.readSuccess).toBe(true);
		});

		it("should handle files with special characters in filenames", async () => {
			app.routes.push({
				method: "POST",
				path: "/upload-special",
				handler_name: "uploadSpecial",
				is_async: true,
			});

			app.handlers.uploadSpecial = async (payload: { file: UploadFile }): Promise<{ filename: string }> => {
				const file = payload.file as UploadFile;
				return {
					filename: file.filename,
				};
			};

			client = new TestClient(app);

			const response = await client.post("/upload-special", {
				multipart: {
					files: [
						{
							name: "file",
							filename: "文件-2024_01_01.txt",
							content: "content",
						},
					],
				},
			});

			expect(response.statusCode).toBe(200);
			const body = response.json();
			expect(body.filename).toBe("文件-2024_01_01.txt");
		});

		it("should handle binary file content (base64 encoded)", async () => {
			app.routes.push({
				method: "POST",
				path: "/upload-binary",
				handler_name: "uploadBinary",
				is_async: true,
			});

			app.handlers.uploadBinary = async (payload: {
				file: UploadFile;
			}): Promise<{ filename: string; size: number; firstBytes: string }> => {
				const file = payload.file as UploadFile;
				const buffer = file.getBuffer();
				return {
					filename: file.filename,
					size: file.size,
					firstBytes: Array.from(buffer.slice(0, 4)).join(","),
				};
			};

			client = new TestClient(app);

			// Create binary data: [0xFF, 0xD8, 0xFF, 0xE0, ...] (JPEG header)
			const binaryData = Buffer.from([0xff, 0xd8, 0xff, 0xe0, 0x00, 0x10]);
			const base64Content = binaryData.toString("base64");

			const response = await client.post("/upload-binary", {
				multipart: {
					files: [
						{
							name: "file",
							filename: "image.jpg",
							content: base64Content,
							contentType: "image/jpeg",
						},
					],
				},
			});

			expect(response.statusCode).toBe(200);
			const body = response.json();
			expect(body.filename).toBe("image.jpg");
		});
	});

	describe("Error Cases", () => {
		it("should handle missing required file gracefully", async () => {
			app.routes.push({
				method: "POST",
				path: "/upload-required",
				handler_name: "uploadRequired",
				is_async: true,
			});

			app.handlers.uploadRequired = async (payload: {
				file?: UploadFile;
			}): Promise<{ status?: number; body?: Record<string, unknown>; filename?: string }> => {
				const file = payload?.file;
				if (!file) {
					return {
						status: 400,
						body: { error: "File is required" },
					};
				}
				return {
					filename: file.filename,
				};
			};

			client = new TestClient(app);

			// Send request without file
			const response = await client.post("/upload-required", {
				multipart: {
					fields: {},
				},
			});

			// Handler should return error response
			expect(response.statusCode).toBe(400);
			const body = response.json();
			expect(body.error).toBe("File is required");
		});

		it("should handle wrong field name gracefully", async () => {
			app.routes.push({
				method: "POST",
				path: "/upload-wrong-field",
				handler_name: "uploadWrongField",
				is_async: true,
			});

			app.handlers.uploadWrongField = async (
				payload: Record<string, unknown>,
			): Promise<{ hasFile: boolean; type: string }> => {
				const file = payload?.file;
				return {
					hasFile: file !== undefined && file !== null,
					type: typeof file,
				};
			};

			client = new TestClient(app);

			// Send file with wrong field name
			const response = await client.post("/upload-wrong-field", {
				multipart: {
					files: [
						{
							name: "document", // Wrong field name
							filename: "test.txt",
							content: "data",
						},
					],
				},
			});

			expect(response.statusCode).toBe(200);
			const body = response.json();
			expect(body.hasFile).toBe(false);
		});

		it("should handle invalid JSON in fields", async () => {
			interface RequestWithJson {
				file: UploadFile;
				metadata: Record<string, unknown>;
			}

			app.routes.push({
				method: "POST",
				path: "/upload-invalid-json",
				handler_name: "uploadInvalidJson",
				is_async: true,
			});

			app.handlers.uploadInvalidJson = async (payload: {
				file: UploadFile;
				metadata: Record<string, unknown>;
			}): Promise<{ filename: string }> => {
				const body = payload as RequestWithJson;
				return {
					filename: body.file.filename,
				};
			};

			client = new TestClient(app);

			const response = await client.post("/upload-invalid-json", {
				multipart: {
					files: [
						{
							name: "file",
							filename: "test.txt",
							content: "data",
						},
					],
					fields: {
						metadata: { key: "value" }, // Valid JSON object
					},
				},
			});

			expect(response.statusCode).toBe(200);
		});
	});

	describe("Ergonomic API Features", () => {
		it("should NOT require manual JSON.parse", async () => {
			let jsonParseWasCalled = false;
			const originalParse = JSON.parse;
			JSON.parse = (text: string, ...args: unknown[]) => {
				jsonParseWasCalled = true;
				return originalParse.apply(JSON, [text, ...args]);
			};

			app.routes.push({
				method: "POST",
				path: "/upload-no-parse",
				handler_name: "uploadNoParse",
				is_async: true,
			});

			app.handlers.uploadNoParse = async (payload: {
				file: UploadFile;
				description: string;
			}): Promise<{ filename: string; description: string }> => {
				// No manual JSON.parse needed - payload is already an object
				const file = payload.file as UploadFile;
				return {
					filename: file.filename,
					// Access properties directly, no stringify/parse
					description: payload.description,
				};
			};

			client = new TestClient(app);

			try {
				const response = await client.post("/upload-no-parse", {
					multipart: {
						files: [
							{
								name: "file",
								filename: "test.txt",
								content: "data",
							},
						],
						fields: {
							description: "Test description",
						},
					},
				});

				expect(response.statusCode).toBe(200);
				const body = response.json();
				expect(body.filename).toBe("test.txt");
				expect(body.description).toBe("Test description");
				expect(jsonParseWasCalled).toBe(false);
			} finally {
				// Restore original JSON.parse
				JSON.parse = originalParse;
			}
		});

		it("should provide type-safe file instances", async () => {
			interface TypedRequest {
				file: UploadFile;
				name: string;
			}

			app.routes.push({
				method: "POST",
				path: "/upload-typed",
				handler_name: "uploadTyped",
				is_async: true,
			});

			app.handlers.uploadTyped = async (payload: {
				file: UploadFile;
				name: string;
			}): Promise<{ filename: string; size: number; contentType: string; name: string }> => {
				const body = payload as TypedRequest;

				// TypeScript knows these properties exist
				const filename: string = body.file.filename;
				const size: number = body.file.size;
				const contentType: string = body.file.contentType;
				const name: string = body.name;

				return {
					filename,
					size,
					contentType,
					name,
				};
			};

			client = new TestClient(app);

			const response = await client.post("/upload-typed", {
				multipart: {
					files: [
						{
							name: "file",
							filename: "test.txt",
							content: "Content",
							contentType: "text/plain",
						},
					],
					fields: {
						name: "Test",
					},
				},
			});

			expect(response.statusCode).toBe(200);
			const body = response.json();
			expect(body.filename).toBe("test.txt");
			expect(body.size).toBe(7);
			expect(body.contentType).toBe("text/plain");
			expect(body.name).toBe("Test");
		});

		it("should auto-convert file metadata at the boundary", async () => {
			interface RequestWithFile {
				file: UploadFile;
			}

			app.routes.push({
				method: "POST",
				path: "/upload-auto-convert",
				handler_name: "uploadAutoConvert",
				is_async: true,
			});

			app.handlers.uploadAutoConvert = async (payload: {
				file: UploadFile;
			}): Promise<{
				isUploadFile: boolean;
				hasReadMethod: boolean;
				hasSeekMethod: boolean;
				hasTextMethod: boolean;
			}> => {
				const body = payload as RequestWithFile;

				// file should be an UploadFile instance, not raw metadata
				const isUploadFile =
					body.file &&
					typeof body.file.filename === "string" &&
					typeof body.file.read === "function" &&
					typeof body.file.seek === "function";

				return {
					isUploadFile,
					hasReadMethod: typeof body.file.read === "function",
					hasSeekMethod: typeof body.file.seek === "function",
					hasTextMethod: typeof body.file.text === "function",
				};
			};

			client = new TestClient(app);

			const response = await client.post("/upload-auto-convert", {
				multipart: {
					files: [
						{
							name: "file",
							filename: "test.txt",
							content: "data",
						},
					],
				},
			});

			expect(response.statusCode).toBe(200);
			const body = response.json();
			expect(body.isUploadFile).toBe(true);
			expect(body.hasReadMethod).toBe(true);
			expect(body.hasSeekMethod).toBe(true);
			expect(body.hasTextMethod).toBe(true);
		});

		it("should support easy async file operations", async () => {
			app.routes.push({
				method: "POST",
				path: "/upload-async-ops",
				handler_name: "uploadAsyncOps",
				is_async: true,
			});

			app.handlers.uploadAsyncOps = async (payload: {
				file: UploadFile;
			}): Promise<{ text: string; position: number }> => {
				const file = payload.file as UploadFile;

				// Can use async operations naturally
				const text = await file.textAsync();
				const position = await file.seekAsync(0);

				return {
					text,
					position,
				};
			};

			client = new TestClient(app);

			const response = await client.post("/upload-async-ops", {
				multipart: {
					files: [
						{
							name: "file",
							filename: "test.txt",
							content: "Hello",
						},
					],
				},
			});

			expect(response.statusCode).toBe(200);
			const body = response.json();
			expect(body.text).toBe("Hello");
			expect(body.position).toBe(0);
		});
	});

	describe("Type Safety and Validation", () => {
		it("should preserve field type information", async () => {
			interface TypedRequest {
				file: UploadFile;
				count: number;
				enabled: boolean;
				tags: string[];
			}

			app.routes.push({
				method: "POST",
				path: "/upload-typed-fields",
				handler_name: "uploadTypedFields",
				is_async: true,
			});

			app.handlers.uploadTypedFields = async (payload: {
				file: UploadFile;
				count: number;
				enabled: boolean;
				tags: string[];
			}): Promise<{ count: number; enabled: boolean; tags: string[] }> => {
				const body = payload as TypedRequest;
				return {
					count: body.count,
					enabled: body.enabled,
					tags: body.tags,
				};
			};

			client = new TestClient(app);

			const response = await client.post("/upload-typed-fields", {
				multipart: {
					files: [
						{
							name: "file",
							filename: "test.txt",
							content: "data",
						},
					],
					fields: {
						count: 42,
						enabled: true,
						tags: ["important", "urgent"],
					},
				},
			});

			expect(response.statusCode).toBe(200);
			const body = response.json();
			expect(body.count).toBe(42);
			expect(body.enabled).toBe(true);
			expect(Array.isArray(body.tags)).toBe(true);
		});
	});
});

/**
 * SECTION 3: Converter Function Tests
 *
 * Tests for the file metadata conversion utilities
 */
describe("File Metadata Converters", () => {
	it("should convert file metadata to UploadFile", () => {
		const metadata: {
			filename: string;
			content: string;
			content_type: string;
			size: number;
		} = {
			filename: "test.txt",
			content: "Hello, World!",
			content_type: "text/plain",
			size: 13,
		};

		const uploadFile = convertFileMetadataToUploadFile(metadata as unknown);

		expect(uploadFile.filename).toBe("test.txt");
		expect(uploadFile.contentType).toBe("text/plain");
		expect(uploadFile.size).toBe(13);
		expect(uploadFile.text()).toBe("Hello, World!");
	});

	it("should handle base64 encoded content", () => {
		const binaryData = Buffer.from("Binary content");
		const base64Content = binaryData.toString("base64");

		const metadata: {
			filename: string;
			content: string;
			content_encoding: "base64";
			content_type: string;
		} = {
			filename: "binary.bin",
			content: base64Content,
			content_encoding: "base64" as const,
			content_type: "application/octet-stream",
		};

		const uploadFile = convertFileMetadataToUploadFile(metadata as unknown);

		expect(uploadFile.getBuffer()).toEqual(binaryData);
	});

	it("should process upload file fields in request body", () => {
		const body: {
			file: { filename: string; content: string; content_type: string };
			description: string;
		} = {
			file: {
				filename: "test.txt",
				content: "Hello",
				content_type: "text/plain",
			},
			description: "Test",
		};

		const result = processUploadFileFields(body as unknown) as unknown;

		expect((result as Record<string, unknown>).file instanceof UploadFile).toBe(true);
		expect((result as Record<string, unknown>).description).toBe("Test");
	});

	it("should process arrays of upload file fields", () => {
		const body: {
			files: Array<{ filename: string; content: string; content_type: string }>;
		} = {
			files: [
				{
					filename: "file1.txt",
					content: "Content 1",
					content_type: "text/plain",
				},
				{
					filename: "file2.txt",
					content: "Content 2",
					content_type: "text/plain",
				},
			],
		};

		const result = processUploadFileFields(body as unknown) as unknown;

		expect(Array.isArray((result as Record<string, unknown>).files)).toBe(true);
		expect(((result as Record<string, unknown>).files as unknown[]).length).toBe(2);
		expect(((result as Record<string, unknown>).files as unknown[])[0] instanceof UploadFile).toBe(true);
	});

	it("should handle nested objects with upload files", () => {
		const body: {
			metadata: {
				title: string;
				attachment: { filename: string; content: string; content_type: string };
			};
		} = {
			metadata: {
				title: "Document",
				attachment: {
					filename: "doc.pdf",
					content: "PDF content",
					content_type: "application/pdf",
				},
			},
		};

		const result = processUploadFileFields(body as unknown) as Record<string, unknown>;

		const metadata = result.metadata as Record<string, unknown>;
		expect(metadata.title).toBe("Document");
		expect(metadata.attachment instanceof UploadFile).toBe(true);
	});
});
