/**
 * Comprehensive tests for converters module
 *
 * Tests cover:
 * - File metadata conversion to UploadFile
 * - Upload file field processing (primitives, objects, arrays)
 * - Multipart test payload conversion
 * - Handler body conversion with special payloads
 * - Edge cases and nested structures
 */

import { describe, expect, it } from "vitest";
import { convertFileMetadataToUploadFile, convertHandlerBody, processUploadFileFields } from "./converters";
import { UploadFile } from "./upload";

describe("converters", () => {
	describe("convertFileMetadataToUploadFile", () => {
		it("should convert basic file metadata", () => {
			const metadata = {
				filename: "test.txt",
				content: "Hello, World!",
				content_type: "text/plain",
				size: 13,
			};

			const file = convertFileMetadataToUploadFile(metadata as unknown);

			expect(file).toBeInstanceOf(UploadFile);
			expect(file.filename).toBe("test.txt");
			expect(file.contentType).toBe("text/plain");
			expect(file.size).toBe(13);
			expect(file.text()).toBe("Hello, World!");
		});

		it("should handle missing optional properties", () => {
			const metadata = {
				filename: "file.bin",
				content: "Binary data",
			};

			const file = convertFileMetadataToUploadFile(metadata as unknown);

			expect(file.filename).toBe("file.bin");
			expect(file.contentType).toBe("application/octet-stream");
			expect(file.size).toBe(11);
		});

		it("should handle base64 encoded content", () => {
			const binaryData = Buffer.from([0xff, 0xd8, 0xff, 0xe0]);
			const base64Content = binaryData.toString("base64");

			const metadata = {
				filename: "image.jpg",
				content: base64Content,
				content_encoding: "base64",
				content_type: "image/jpeg",
				size: 4,
			};

			const file = convertFileMetadataToUploadFile(metadata as unknown);

			expect(file.getBuffer()).toEqual(binaryData);
			expect(file.filename).toBe("image.jpg");
		});

		it("should treat text content as UTF-8 by default", () => {
			const metadata = {
				filename: "hello.txt",
				content: "Hello, 世界!",
				content_type: "text/plain; charset=utf-8",
			};

			const file = convertFileMetadataToUploadFile(metadata as unknown);

			expect(file.text()).toBe("Hello, 世界!");
		});

		it("should handle content_type in metadata", () => {
			const metadata = {
				filename: "data.json",
				content: '{"key":"value"}',
				content_type: "application/json",
			};

			const file = convertFileMetadataToUploadFile(metadata as unknown);

			expect(file.contentType).toBe("application/json");
		});

		it("should handle all content encoding types", () => {
			const metadata = {
				filename: "file.bin",
				content: Buffer.from("data").toString("base64"),
				content_encoding: "base64" as const,
			};

			const file = convertFileMetadataToUploadFile(metadata as unknown);

			expect(file.getBuffer().toString()).toBe("data");
		});

		it("should use provided size if available", () => {
			const metadata = {
				filename: "large.bin",
				content: "Large file content",
				size: 999,
			};

			const file = convertFileMetadataToUploadFile(metadata as unknown);

			expect(file.size).toBe(999);
		});
	});

	describe("processUploadFileFields", () => {
		it("should pass through null values", () => {
			expect(processUploadFileFields(null as unknown)).toBeNull();
		});

		it("should pass through undefined values", () => {
			expect(processUploadFileFields(undefined as unknown)).toBeUndefined();
		});

		it("should pass through string primitives", () => {
			expect(processUploadFileFields("hello" as unknown)).toBe("hello");
		});

		it("should pass through number primitives", () => {
			expect(processUploadFileFields(42 as unknown)).toBe(42);
		});

		it("should pass through boolean primitives", () => {
			expect(processUploadFileFields(true as unknown)).toBe(true);
			expect(processUploadFileFields(false as unknown)).toBe(false);
		});

		it("should convert file metadata in objects", () => {
			const body = {
				file: {
					filename: "test.txt",
					content: "Test content",
					content_type: "text/plain",
				},
				name: "John",
			};

			const result = processUploadFileFields(body as unknown) as Record<string, unknown>;

			expect(result.file).toBeInstanceOf(UploadFile);
			expect(result.name).toBe("John");
			expect((result.file as UploadFile).filename).toBe("test.txt");
		});

		it("should process arrays recursively", () => {
			const body = [
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
			];

			const result = processUploadFileFields(body as unknown) as unknown[];

			expect(Array.isArray(result)).toBe(true);
			expect(result[0]).toBeInstanceOf(UploadFile);
			expect(result[1]).toBeInstanceOf(UploadFile);
		});

		it("should process nested objects with file fields", () => {
			const body = {
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
			expect(metadata.attachment).toBeInstanceOf(UploadFile);
		});

		it("should handle mixed arrays with files and primitives", () => {
			const body = [
				{
					filename: "test.txt",
					content: "Test",
					content_type: "text/plain",
				},
				"string value",
				42,
				true,
			];

			const result = processUploadFileFields(body as unknown) as unknown[];

			expect(result[0]).toBeInstanceOf(UploadFile);
			expect(result[1]).toBe("string value");
			expect(result[2]).toBe(42);
			expect(result[3]).toBe(true);
		});

		it("should handle empty objects", () => {
			const body = {};
			const result = processUploadFileFields(body as unknown) as Record<string, unknown>;

			expect(Object.keys(result).length).toBe(0);
		});

		it("should handle empty arrays", () => {
			const body = [];
			const result = processUploadFileFields(body as unknown) as unknown[];

			expect(result.length).toBe(0);
		});

		it("should process deeply nested structures", () => {
			const body = {
				level1: {
					level2: {
						level3: {
							file: {
								filename: "deep.txt",
								content: "Deep content",
								content_type: "text/plain",
							},
						},
					},
				},
			};

			const result = processUploadFileFields(body as unknown) as Record<string, unknown>;
			const level1 = result.level1 as Record<string, unknown>;
			const level2 = level1.level2 as Record<string, unknown>;
			const level3 = level2.level3 as Record<string, unknown>;

			expect(level3.file).toBeInstanceOf(UploadFile);
		});

		it("should handle arrays of objects with files", () => {
			const body = [
				{
					name: "file1",
					file: {
						filename: "file1.txt",
						content: "Content 1",
						content_type: "text/plain",
					},
				},
				{
					name: "file2",
					file: {
						filename: "file2.txt",
						content: "Content 2",
						content_type: "text/plain",
					},
				},
			];

			const result = processUploadFileFields(body as unknown) as Array<Record<string, unknown>>;

			expect(result[0].name).toBe("file1");
			expect(result[0].file).toBeInstanceOf(UploadFile);
			expect(result[1].name).toBe("file2");
			expect(result[1].file).toBeInstanceOf(UploadFile);
		});

		it("should preserve non-file object properties", () => {
			const body = {
				file: {
					filename: "test.txt",
					content: "Test",
					content_type: "text/plain",
				},
				metadata: {
					created: "2024-01-01",
					tags: ["important", "urgent"],
				},
				count: 5,
				enabled: true,
			};

			const result = processUploadFileFields(body as unknown) as Record<string, unknown>;

			expect(result.file).toBeInstanceOf(UploadFile);
			expect((result.metadata as Record<string, unknown>).created).toBe("2024-01-01");
			expect((result.metadata as Record<string, unknown>).tags).toEqual(["important", "urgent"]);
			expect(result.count).toBe(5);
			expect(result.enabled).toBe(true);
		});

		it("should handle file arrays within objects", () => {
			const body = {
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
				description: "Multiple files",
			};

			const result = processUploadFileFields(body as unknown) as Record<string, unknown>;

			expect(Array.isArray(result.files)).toBe(true);
			const files = result.files as unknown[];
			expect(files[0]).toBeInstanceOf(UploadFile);
			expect(files[1]).toBeInstanceOf(UploadFile);
			expect(result.description).toBe("Multiple files");
		});
	});

	describe("convertHandlerBody", () => {
		it("should handle multipart test payload", () => {
			const body = {
				__spikard_multipart__: {
					fields: {
						name: "John",
						email: "john@example.com",
					},
					files: [
						{
							name: "document",
							filename: "resume.pdf",
							content: "PDF content",
							contentType: "application/pdf",
						},
					],
				},
			};

			const result = convertHandlerBody(body as unknown) as Record<string, unknown>;

			expect(result.name).toBe("John");
			expect(result.email).toBe("john@example.com");
			expect(result.document).toBeInstanceOf(UploadFile);
			expect((result.document as UploadFile).filename).toBe("resume.pdf");
		});

		it("should handle multipart with multiple files same name", () => {
			const body = {
				__spikard_multipart__: {
					fields: {},
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
					],
				},
			};

			const result = convertHandlerBody(body as unknown) as Record<string, unknown>;

			expect(Array.isArray(result.files)).toBe(true);
			const files = result.files as unknown[];
			expect(files.length).toBe(2);
			expect(files[0]).toBeInstanceOf(UploadFile);
			expect(files[1]).toBeInstanceOf(UploadFile);
		});

		it("should handle multipart with no files", () => {
			const body = {
				__spikard_multipart__: {
					fields: {
						username: "alice",
						password: "secret",
					},
					files: [],
				},
			};

			const result = convertHandlerBody(body as unknown) as Record<string, unknown>;

			expect(result.username).toBe("alice");
			expect(result.password).toBe("secret");
			expect(Object.keys(result).length).toBe(2);
		});

		it("should handle multipart with no fields", () => {
			const body = {
				__spikard_multipart__: {
					fields: undefined,
					files: [
						{
							name: "file",
							filename: "data.txt",
							content: "Data",
							contentType: "text/plain",
						},
					],
				},
			};

			const result = convertHandlerBody(body as unknown) as Record<string, unknown>;

			expect(result.file).toBeInstanceOf(UploadFile);
		});

		it("should handle regular JSON body with file metadata", () => {
			const body = {
				file: {
					filename: "upload.txt",
					content: "Upload content",
					content_type: "text/plain",
				},
				name: "test",
			};

			const result = convertHandlerBody(body as unknown) as Record<string, unknown>;

			expect(result.file).toBeInstanceOf(UploadFile);
			expect(result.name).toBe("test");
		});

		it("should handle null body", () => {
			const result = convertHandlerBody(null as unknown);

			expect(result).toBeNull();
		});

		it("should handle undefined body", () => {
			const result = convertHandlerBody(undefined as unknown);

			expect(result).toBeUndefined();
		});

		it("should handle primitive body values", () => {
			expect(convertHandlerBody("string" as unknown)).toBe("string");
			expect(convertHandlerBody(42 as unknown)).toBe(42);
			expect(convertHandlerBody(true as unknown)).toBe(true);
		});

		it("should handle multipart with fields only", () => {
			const body = {
				__spikard_multipart__: {
					fields: {
						title: "Document",
						description: "A test document",
					},
				},
			};

			const result = convertHandlerBody(body as unknown) as Record<string, unknown>;

			expect(result.title).toBe("Document");
			expect(result.description).toBe("A test document");
		});

		it("should handle multipart with empty fields object", () => {
			const body = {
				__spikard_multipart__: {
					fields: {},
					files: [
						{
							name: "file",
							filename: "test.txt",
							content: "Content",
						},
					],
				},
			};

			const result = convertHandlerBody(body as unknown) as Record<string, unknown>;

			expect(result.file).toBeInstanceOf(UploadFile);
		});

		it("should distinguish multipart from regular JSON", () => {
			const multipart = {
				__spikard_multipart__: {
					fields: { name: "test" },
					files: [],
				},
			};

			const regular = {
				__spikard_multipart__: undefined,
				name: "test",
			};

			const resultMultipart = convertHandlerBody(multipart as unknown) as Record<string, unknown>;
			const resultRegular = convertHandlerBody(regular as unknown) as Record<string, unknown>;

			expect(resultMultipart).toEqual({ name: "test" });
			expect(resultRegular.__spikard_multipart__).toBeUndefined();
		});

		it("should handle multipart with defaulted properties", () => {
			const body = {
				__spikard_multipart__: {
					fields: {
						name: "John",
					},
					files: [
						{
							name: "avatar",
							content: "image data",
							// No filename provided
							// No contentType provided
						},
					],
				},
			};

			const result = convertHandlerBody(body as unknown) as Record<string, unknown>;

			expect(result.name).toBe("John");
			expect(result.avatar).toBeInstanceOf(UploadFile);
			const file = result.avatar as UploadFile;
			expect(file.filename).toBe("avatar"); // Falls back to field name
		});

		it("should process nested structures within multipart fields", () => {
			const body = {
				__spikard_multipart__: {
					fields: {
						user: {
							name: "Alice",
							email: "alice@example.com",
						},
						preferences: {
							theme: "dark",
							language: "en",
						},
					},
					files: [],
				},
			};

			const result = convertHandlerBody(body as unknown) as Record<string, unknown>;

			const user = result.user as Record<string, unknown>;
			expect(user.name).toBe("Alice");
			expect(user.email).toBe("alice@example.com");

			const prefs = result.preferences as Record<string, unknown>;
			expect(prefs.theme).toBe("dark");
		});

		it("should handle mixed multipart with files and regular fields", () => {
			const body = {
				__spikard_multipart__: {
					fields: {
						title: "My Document",
						author: "John Doe",
						tags: ["important", "archived"],
						metadata: {
							created: "2024-01-01",
							version: 1,
						},
					},
					files: [
						{
							name: "document",
							filename: "report.pdf",
							content: "PDF data",
							contentType: "application/pdf",
						},
						{
							name: "attachments",
							filename: "notes.txt",
							content: "Notes",
							contentType: "text/plain",
						},
					],
				},
			};

			const result = convertHandlerBody(body as unknown) as Record<string, unknown>;

			expect(result.title).toBe("My Document");
			expect(result.author).toBe("John Doe");
			expect(Array.isArray(result.tags)).toBe(true);
			expect(result.document).toBeInstanceOf(UploadFile);
			expect(result.attachments).toBeInstanceOf(UploadFile);
		});
	});

	describe("Edge Cases and Integration", () => {
		it("should handle ISO-8859-1 encoded content as text", () => {
			const content = Buffer.from("Café").toString("utf-8");
			const metadata = {
				filename: "menu.txt",
				content,
				content_type: "text/plain; charset=iso-8859-1",
			};

			const file = convertFileMetadataToUploadFile(metadata as unknown);

			expect(file.text()).toContain("Caf");
		});

		it("should handle large file content", () => {
			const largeContent = "x".repeat(1024 * 1024); // 1MB of x's
			const metadata = {
				filename: "large.txt",
				content: largeContent,
				content_type: "text/plain",
				size: largeContent.length,
			};

			const file = convertFileMetadataToUploadFile(metadata as unknown);

			expect(file.size).toBe(1024 * 1024);
			expect(file.text().length).toBe(1024 * 1024);
		});

		it("should handle special characters in filenames", () => {
			const metadata = {
				filename: "文件-2024_01_01 (1).txt",
				content: "Special filename test",
				content_type: "text/plain",
			};

			const file = convertFileMetadataToUploadFile(metadata as unknown);

			expect(file.filename).toBe("文件-2024_01_01 (1).txt");
		});

		it("should handle files with null size field", () => {
			const metadata = {
				filename: "test.txt",
				content: "Test content",
				content_type: "text/plain",
				size: null,
			};

			const file = convertFileMetadataToUploadFile(metadata as unknown);

			// Should compute size from content
			expect(file.size).toBe(12);
		});
	});
});
