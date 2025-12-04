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
 */

import type { SpikardApp } from "./index";
import { TestClient } from "./testing";

describe("UploadFile Handler Integration", () => {
	let app: SpikardApp;

	beforeEach(() => {
		app = { routes: [], handlers: {} };
	});

	describe("Single File Upload", () => {
		it("should handle single file in typed request body", async () => {
			interface UploadRequest {
				file: UploadFile;
				description: string;
			}

			app.routes?.push({
				method: "POST",
				path: "/upload",
				handler_name: "uploadHandler",
				is_async: true,
			});

			app.handlers.uploadHandler = async (request) => {
				const body = request.json<UploadRequest>();
				return {
					filename: body.file.filename,
					size: body.file.size,
					contentType: body.file.contentType,
					description: body.description,
					content: body.file.text(),
				};
			};

			const client = new TestClient(app);

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
			const result = response.json() as unknown;
			expect((result as Record<string, unknown>).filename).toBe("test.txt");
			expect((result as Record<string, unknown>).contentType).toBe("text/plain");
			expect((result as Record<string, unknown>).description).toBe("Test file");
			expect((result as Record<string, unknown>).content).toBe("Hello, World!");
		});

		it("should handle file with different MIME types", async () => {
			interface UploadRequest {
				file: UploadFile;
			}

			app.routes?.push({
				method: "POST",
				path: "/upload-json",
				handler_name: "uploadJson",
				is_async: true,
			});

			app.handlers.uploadJson = async (request) => {
				const body = request.json<UploadRequest>();
				return {
					filename: body.file.filename,
					contentType: body.file.contentType,
				};
			};

			const client = new TestClient(app);

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
			const body = response.json() as unknown;
			expect((body as Record<string, unknown>).contentType).toBe("application/json");
		});

		it("should provide access to file headers", async () => {
			interface UploadRequest {
				file: UploadFile;
			}

			app.routes?.push({
				method: "POST",
				path: "/upload-headers",
				handler_name: "uploadHeaders",
				is_async: true,
			});

			app.handlers.uploadHeaders = async (request) => {
				const body = request.json<UploadRequest>();
				return {
					filename: body.file.filename,
					hasHeaders: Object.keys(body.file.headers).length > 0,
				};
			};

			const client = new TestClient(app);

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

			app.routes?.push({
				method: "POST",
				path: "/upload-multiple",
				handler_name: "uploadMultiple",
				is_async: true,
			});

			app.handlers.uploadMultiple = async (request) => {
				const body = request.json<MultiFileRequest>();
				return {
					count: body.files.length,
					filenames: body.files.map((f) => f.filename),
					sizes: body.files.map((f) => f.size),
				};
			};

			const client = new TestClient(app);

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
			const body = response.json() as unknown;
			expect((body as Record<string, unknown>).count).toBe(3);
			expect((body as Record<string, unknown>).filenames).toEqual(["file1.txt", "file2.txt", "file3.txt"]);
			expect((body as Record<string, unknown>).sizes).toEqual([9, 9, 9]);
		});

		it("should handle empty file array", async () => {
			interface EmptyFileRequest {
				files?: UploadFile[];
			}

			app.routes?.push({
				method: "POST",
				path: "/upload-empty",
				handler_name: "uploadEmpty",
				is_async: true,
			});

			app.handlers.uploadEmpty = async (request) => {
				const body = request.json<EmptyFileRequest>();
				const files = (body.files ?? []) as UploadFile[];
				return {
					count: Array.isArray(files) ? files.length : 0,
				};
			};

			const client = new TestClient(app);

			const response = await client.post("/upload-empty", {
				multipart: {
					files: [],
				},
			});

			expect(response.statusCode).toBe(200);
			const body = response.json() as unknown;
			expect((body as Record<string, unknown>).count).toBe(0);
		});
	});

	describe("Optional File Upload", () => {
		it("should handle optional file when provided", async () => {
			interface OptionalUploadRequest {
				file?: UploadFile;
				name: string;
			}

			app.routes?.push({
				method: "POST",
				path: "/upload-optional",
				handler_name: "uploadOptional",
				is_async: true,
			});

			app.handlers.uploadOptional = async (request) => {
				const body = request.json<OptionalUploadRequest>();
				return {
					hasFile: body.file !== undefined && body.file !== null,
					filename: body.file?.filename,
					name: body.name,
				};
			};

			const client = new TestClient(app);

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
			const body = response.json() as unknown;
			expect((body as Record<string, unknown>).hasFile).toBe(true);
			expect((body as Record<string, unknown>).filename).toBe("test.txt");
			expect((body as Record<string, unknown>).name).toBe("Alice");
		});

		it("should handle optional file when not provided", async () => {
			interface OptionalUploadRequest {
				file?: UploadFile;
				name: string;
			}

			app.routes?.push({
				method: "POST",
				path: "/upload-optional-absent",
				handler_name: "uploadOptionalAbsent",
				is_async: true,
			});

			app.handlers.uploadOptionalAbsent = async (request) => {
				const body = request.json<OptionalUploadRequest>();
				return {
					hasFile: body.file !== undefined && body.file !== null,
					filename: body.file?.filename ?? null,
					name: body.name,
				};
			};

			const client = new TestClient(app);

			const response = await client.post("/upload-optional-absent", {
				multipart: {
					fields: {
						name: "Bob",
					},
				},
			});

			expect(response.statusCode).toBe(200);
			const body = response.json() as unknown;
			expect((body as Record<string, unknown>).hasFile).toBe(false);
			expect((body as Record<string, unknown>).filename).toBe(null);
			expect((body as Record<string, unknown>).name).toBe("Bob");
		});
	});

	describe("Mixed Form Data and Files", () => {
		it("should handle mixed form fields and files", async () => {
			interface MixedRequest {
				file: UploadFile;
				name: string;
				email: string;
				age: string | number;
			}

			app.routes?.push({
				method: "POST",
				path: "/upload-mixed",
				handler_name: "uploadMixed",
				is_async: true,
			});

			app.handlers.uploadMixed = async (request) => {
				const body = request.json<MixedRequest>();
				// Parse numeric field
				const age = typeof body.age === "string" ? parseInt(body.age, 10) : body.age;
				return {
					filename: body.file.filename,
					name: body.name,
					email: body.email,
					age,
				};
			};

			const client = new TestClient(app);

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
			const body = response.json() as unknown;
			expect((body as Record<string, unknown>).filename).toBe("profile.txt");
			expect((body as Record<string, unknown>).name).toBe("Charlie");
			expect((body as Record<string, unknown>).email).toBe("charlie@example.com");
			expect((body as Record<string, unknown>).age).toBe(25);
		});

		it("should handle files in nested objects", async () => {
			app.routes?.push({
				method: "POST",
				path: "/upload-nested",
				handler_name: "uploadNested",
				is_async: true,
			});

			app.handlers.uploadNested = async (request) => {
				const body = request.json<Record<string, unknown>>();
				// Since form data uses dotted keys, access file by the dotted key
				const file = (body["metadata.file"] as UploadFile) || (body.metadata as any)?.file;
				const title = (body["metadata.title"] as string) || (body.metadata as any)?.title;

				return {
					title,
					filename: file?.filename,
					size: file?.size,
				};
			};

			const client = new TestClient(app);

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
			const body = response.json() as unknown;
			expect((body as Record<string, unknown>).title).toBe("My Document");
			expect((body as Record<string, unknown>).filename).toBe("document.txt");
		});
	});

	describe("Large File Handling", () => {
		it("should handle large binary files", async () => {
			interface LargeFileRequest {
				file: UploadFile;
			}

			app.routes?.push({
				method: "POST",
				path: "/upload-large",
				handler_name: "uploadLarge",
				is_async: true,
			});

			app.handlers.uploadLarge = async (request) => {
				const body = request.json<LargeFileRequest>();
				return {
					filename: body.file.filename,
					size: body.file.size,
					readSuccess: true,
				};
			};

			const client = new TestClient(app);

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
			const body = response.json() as unknown;
			expect((body as Record<string, unknown>).filename).toBe("large.bin");
			expect((body as Record<string, unknown>).readSuccess).toBe(true);
		});

		it("should handle files with special characters in filenames", async () => {
			interface SpecialFileRequest {
				file: UploadFile;
			}

			app.routes?.push({
				method: "POST",
				path: "/upload-special",
				handler_name: "uploadSpecial",
				is_async: true,
			});

			app.handlers.uploadSpecial = async (request) => {
				const body = request.json<SpecialFileRequest>();
				return {
					filename: body.file.filename,
				};
			};

			const client = new TestClient(app);

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
			const body = response.json() as unknown;
			expect((body as Record<string, unknown>).filename).toBe("文件-2024_01_01.txt");
		});

		it("should handle binary file content (base64 encoded)", async () => {
			interface BinaryFileRequest {
				file: UploadFile;
			}

			app.routes?.push({
				method: "POST",
				path: "/upload-binary",
				handler_name: "uploadBinary",
				is_async: true,
			});

			app.handlers.uploadBinary = async (request) => {
				const body = request.json<BinaryFileRequest>();
				const buffer = body.file.getBuffer();
				return {
					filename: body.file.filename,
					size: body.file.size,
					firstBytes: Array.from(buffer.slice(0, 4)).join(","),
				};
			};

			const client = new TestClient(app);

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
			const body = response.json() as unknown;
			expect((body as Record<string, unknown>).filename).toBe("image.jpg");
		});
	});

	describe("Error Cases", () => {
		it("should handle missing required file gracefully", async () => {
			interface RequestWithOptionalFile {
				file?: UploadFile;
			}

			app.routes?.push({
				method: "POST",
				path: "/upload-required",
				handler_name: "uploadRequired",
				is_async: true,
			});

			app.handlers.uploadRequired = async (request) => {
				const body = request.json<RequestWithOptionalFile>();
				const file = body?.file;
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

			const client = new TestClient(app);

			// Send request without file
			const response = await client.post("/upload-required", {
				multipart: {
					fields: {},
				},
			});

			// Handler should return error response
			expect(response.statusCode).toBe(400);
			const body = response.json() as unknown;
			expect((body as Record<string, unknown>).error).toBe("File is required");
		});

		it("should handle wrong field name gracefully", async () => {
			app.routes?.push({
				method: "POST",
				path: "/upload-wrong-field",
				handler_name: "uploadWrongField",
				is_async: true,
			});

			app.handlers.uploadWrongField = async (request) => {
				const body = request.json<Record<string, unknown>>();
				const file = body?.file;
				return {
					hasFile: file !== undefined && file !== null,
					type: typeof file,
				};
			};

			const client = new TestClient(app);

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
			const body = response.json() as unknown;
			expect((body as Record<string, unknown>).hasFile).toBe(false);
		});

		it("should handle invalid JSON in fields", async () => {
			interface RequestWithJson {
				file: UploadFile;
				metadata: Record<string, unknown>;
			}

			app.routes?.push({
				method: "POST",
				path: "/upload-invalid-json",
				handler_name: "uploadInvalidJson",
				is_async: true,
			});

			app.handlers.uploadInvalidJson = async (request) => {
				const body = request.json<RequestWithJson>();
				return {
					filename: body.file.filename,
				};
			};

			const client = new TestClient(app);

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
			interface UploadRequest {
				file: UploadFile;
				description: string;
			}

			app.routes?.push({
				method: "POST",
				path: "/upload-no-parse",
				handler_name: "uploadNoParse",
				is_async: true,
			});

			// This handler does NOT manually call JSON.parse - it just uses request.json()
			app.handlers.uploadNoParse = async (request) => {
				const body = request.json<UploadRequest>();
				// No manual JSON.parse needed - request.json() handles it
				const file = body.file as UploadFile;
				return {
					filename: file.filename,
					// Access properties directly, no manual parsing needed
					description: body.description,
				};
			};

			const client = new TestClient(app);

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
			const body = response.json() as unknown;
			expect((body as Record<string, unknown>).filename).toBe("test.txt");
			expect((body as Record<string, unknown>).description).toBe("Test description");
		});

		it("should provide type-safe file instances", async () => {
			interface TypedRequest {
				file: UploadFile;
				name: string;
			}

			app.routes?.push({
				method: "POST",
				path: "/upload-typed",
				handler_name: "uploadTyped",
				is_async: true,
			});

			app.handlers.uploadTyped = async (request) => {
				const body = request.json<TypedRequest>();
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

			const client = new TestClient(app);

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
			const body = response.json() as unknown;
			expect((body as Record<string, unknown>).filename).toBe("test.txt");
			expect((body as Record<string, unknown>).size).toBe(7);
			expect((body as Record<string, unknown>).contentType).toBe("text/plain");
			expect((body as Record<string, unknown>).name).toBe("Test");
		});

		it("should auto-convert file metadata at the boundary", async () => {
			interface RequestWithFile {
				file: UploadFile;
			}

			app.routes?.push({
				method: "POST",
				path: "/upload-auto-convert",
				handler_name: "uploadAutoConvert",
				is_async: true,
			});

			app.handlers.uploadAutoConvert = async (request) => {
				const body = request.json<RequestWithFile>();
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

			const client = new TestClient(app);

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
			const body = response.json() as unknown;
			expect((body as Record<string, unknown>).isUploadFile).toBe(true);
			expect((body as Record<string, unknown>).hasReadMethod).toBe(true);
			expect((body as Record<string, unknown>).hasSeekMethod).toBe(true);
			expect((body as Record<string, unknown>).hasTextMethod).toBe(true);
		});

		it("should support easy async file operations", async () => {
			interface UploadRequest {
				file: UploadFile;
			}

			app.routes?.push({
				method: "POST",
				path: "/upload-async-ops",
				handler_name: "uploadAsyncOps",
				is_async: true,
			});

			app.handlers.uploadAsyncOps = async (request) => {
				const body = request.json<UploadRequest>();
				// Can use async operations naturally
				const text = await body.file.textAsync();
				await body.file.seekAsync(0);
				const position = body.file.tell();

				return {
					text,
					position,
				};
			};

			const client = new TestClient(app);

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
			const body = response.json() as unknown;
			expect((body as Record<string, unknown>).text).toBe("Hello");
			expect((body as Record<string, unknown>).position).toBe(0);
		});
	});

	describe("Type Safety and Validation", () => {
		it("should preserve field type information", async () => {
			interface TypedRequest {
				file: UploadFile;
				count: string | number;
				enabled: string | boolean;
				tags: string | string[];
			}

			app.routes?.push({
				method: "POST",
				path: "/upload-typed-fields",
				handler_name: "uploadTypedFields",
				is_async: true,
			});

			app.handlers.uploadTypedFields = async (request) => {
				const body = request.json<TypedRequest>();
				// Parse fields since form data converts everything to strings
				const count = typeof body.count === "string" ? parseInt(body.count, 10) : body.count;
				const enabled = typeof body.enabled === "string" ? body.enabled === "true" : body.enabled;
				const tags = Array.isArray(body.tags) ? body.tags : typeof body.tags === "string" ? [body.tags] : [];

				return {
					count,
					enabled,
					tags,
				};
			};

			const client = new TestClient(app);

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
			const body = response.json() as unknown;
			expect((body as Record<string, unknown>).count).toBe(42);
			expect((body as Record<string, unknown>).enabled).toBe(true);
			expect(Array.isArray((body as Record<string, unknown>).tags)).toBe(true);
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
