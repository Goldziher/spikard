/**
 * E2E tests for multipart
 * @generated
 */

import { TestClient } from "@spikard/wasm";
import { assert, assertEquals } from "jsr:@std/assert@1";
import {
	createAppMultipart17FileMagicNumberPngSuccess,
	createAppMultipart18FileMagicNumberJpegSuccess,
	createAppMultipart19FileMimeSpoofingPngAsJpeg,
	createAppMultipart20FileMimeSpoofingJpegAsPng,
	createAppMultipart21FilePdfMagicNumberSuccess,
	createAppMultipart22FileEmptyBuffer,
	createAppMultipartContentTypeValidationInvalidType,
	createAppMultipartEmptyFileUpload,
	createAppMultipartFileListUploadArrayOfFiles,
	createAppMultipartFileSizeValidationTooLarge,
	createAppMultipartFileUploadWithCustomHeaders,
	createAppMultipartFileUploadWithoutFilename,
	createAppMultipartFormDataWithoutFiles,
	createAppMultipartImageFileUpload,
	createAppMultipartMixedFilesAndFormData,
	createAppMultipartMultipleFileUploads,
	createAppMultipartMultipleValuesForSameFieldName,
	createAppMultipartOptionalFileUploadMissing,
	createAppMultipartOptionalFileUploadProvided,
	createAppMultipartPdfFileUpload,
	createAppMultipartRequiredFileUploadMissing,
	createAppMultipartSimpleFileUpload,
} from "../app/main.ts";

	Deno.test("multipart: Multiple values for same field name", async () => {
		const app = createAppMultipartMultipleValuesForSameFieldName();
		const client = new TestClient(app);

		const multipart = { fields: { tags: ["python", "rust", "web"] }, files: [{ name: "files", filename: "file1.txt", content: "first file", contentType: "text/plain" }, { name: "files", filename: "file2.txt", content: "second file", contentType: "text/plain" }] };
		const response = await client.post("/", { multipart });

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "files"));
		assertEquals(responseData.files.length, 2);
		assert(Object.hasOwn(responseData.files[0], "content"));
		assertEquals(responseData.files[0].content, "first file");
		assert(Object.hasOwn(responseData.files[0], "content_type"));
		assertEquals(responseData.files[0].content_type, "text/plain");
		assert(Object.hasOwn(responseData.files[0], "filename"));
		assertEquals(responseData.files[0].filename, "file1.txt");
		assert(Object.hasOwn(responseData.files[0], "size"));
		assertEquals(responseData.files[0].size, 10);
		assert(Object.hasOwn(responseData.files[1], "content"));
		assertEquals(responseData.files[1].content, "second file");
		assert(Object.hasOwn(responseData.files[1], "content_type"));
		assertEquals(responseData.files[1].content_type, "text/plain");
		assert(Object.hasOwn(responseData.files[1], "filename"));
		assertEquals(responseData.files[1].filename, "file2.txt");
		assert(Object.hasOwn(responseData.files[1], "size"));
		assertEquals(responseData.files[1].size, 11);
		assert(Object.hasOwn(responseData, "tags"));
		assertEquals(responseData.tags.length, 3);
		assertEquals(responseData.tags[0], "python");
		assertEquals(responseData.tags[1], "rust");
		assertEquals(responseData.tags[2], "web");
	});

	Deno.test("multipart: 19_file_mime_spoofing_png_as_jpeg", async () => {
		const app = createAppMultipart19FileMimeSpoofingPngAsJpeg();
		const client = new TestClient(app);

		const multipart = { files: [{ name: "image", filename: "fake.jpg", contentType: "image/jpeg", magic_bytes: "89504e470d0a1a0a" }] };
		const response = await client.post("/upload", { multipart });

		assertEquals(response.statusCode, 422);
	});

	Deno.test("multipart: 20_file_mime_spoofing_jpeg_as_png", async () => {
		const app = createAppMultipart20FileMimeSpoofingJpegAsPng();
		const client = new TestClient(app);

		const multipart = { files: [{ name: "image", filename: "fake.png", contentType: "image/png", magic_bytes: "ffd8ffe0" }] };
		const response = await client.post("/upload", { multipart });

		assertEquals(response.statusCode, 422);
	});

	Deno.test("multipart: 21_file_pdf_magic_number_success", async () => {
		const app = createAppMultipart21FilePdfMagicNumberSuccess();
		const client = new TestClient(app);

		const multipart = { files: [{ name: "document", filename: "test.pdf", contentType: "application/pdf", magic_bytes: "25504446" }] };
		const response = await client.post("/upload", { multipart });

		assertEquals(response.statusCode, 201);
	});

	Deno.test("multipart: Content-Type validation - invalid type", async () => {
		const app = createAppMultipartContentTypeValidationInvalidType();
		const client = new TestClient(app);

		const multipart = { files: [{ name: "file", filename: "script.sh", content: "#!/bin/bash\necho hello", contentType: "application/x-sh" }] };
		const response = await client.post("/files/images-only", { multipart });

		assertEquals(response.statusCode, 422);
	});

	Deno.test("multipart: PDF file upload", async () => {
		const app = createAppMultipartPdfFileUpload();
		const client = new TestClient(app);

		const multipart = { files: [{ name: "document", filename: "report.pdf", content: "fake_pdf_content", contentType: "application/pdf" }] };
		const response = await client.post("/files/document", { multipart });

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "content_type"));
		assertEquals(responseData.content_type, "application/pdf");
		assert(Object.hasOwn(responseData, "filename"));
		assertEquals(responseData.filename, "report.pdf");
		assert(Object.hasOwn(responseData, "size"));
		assertEquals(responseData.size, 16);
	});

	Deno.test("multipart: File list upload array of files", async () => {
		const app = createAppMultipartFileListUploadArrayOfFiles();
		const client = new TestClient(app);

		const multipart = { files: [{ name: "files", filename: "file1.txt", content: "content of file 1", contentType: "text/plain" }, { name: "files", filename: "file2.txt", content: "content of file 2", contentType: "text/plain" }] };
		const response = await client.post("/files/list", { multipart });

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "filenames"));
		assertEquals(responseData.filenames.length, 2);
		assertEquals(responseData.filenames[0], "file1.txt");
		assertEquals(responseData.filenames[1], "file2.txt");
		assert(Object.hasOwn(responseData, "total_size"));
		assertEquals(responseData.total_size, 35);
	});

	Deno.test("multipart: Optional file upload - provided", async () => {
		const app = createAppMultipartOptionalFileUploadProvided();
		const client = new TestClient(app);

		const multipart = { files: [{ name: "file", filename: "optional.txt", content: "optional file content", contentType: "text/plain" }] };
		const response = await client.post("/files/optional", { multipart });

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "content_type"));
		assertEquals(responseData.content_type, "text/plain");
		assert(Object.hasOwn(responseData, "filename"));
		assertEquals(responseData.filename, "optional.txt");
		assert(Object.hasOwn(responseData, "size"));
		assertEquals(responseData.size, 21);
	});

	Deno.test("multipart: File size validation - too large", async () => {
		const app = createAppMultipartFileSizeValidationTooLarge();
		const client = new TestClient(app);

		const multipart = { files: [{ name: "file", filename: "large.txt", content: "x", contentType: "text/plain" }] };
		const response = await client.post("/files/validated", { multipart });

		assertEquals(response.statusCode, 413);
	});

	Deno.test("multipart: Mixed files and form data", async () => {
		const app = createAppMultipartMixedFilesAndFormData();
		const client = new TestClient(app);

		const multipart = { fields: { active: "true", age: "25", username: "testuser" }, files: [{ name: "file", filename: "upload.txt", content: "file data here", contentType: "text/plain" }] };
		const response = await client.post("/", { multipart });

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "active"));
		assertEquals(responseData.active, "true");
		assert(Object.hasOwn(responseData, "age"));
		assertEquals(responseData.age, "25");
		assert(Object.hasOwn(responseData, "file"));
		assert(Object.hasOwn(responseData.file, "content"));
		assertEquals(responseData.file.content, "file data here");
		assert(Object.hasOwn(responseData.file, "content_type"));
		assertEquals(responseData.file.content_type, "text/plain");
		assert(Object.hasOwn(responseData.file, "filename"));
		assertEquals(responseData.file.filename, "upload.txt");
		assert(Object.hasOwn(responseData.file, "size"));
		assertEquals(responseData.file.size, 14);
		assert(Object.hasOwn(responseData, "username"));
		assertEquals(responseData.username, "testuser");
	});

	Deno.test("multipart: Simple file upload", async () => {
		const app = createAppMultipartSimpleFileUpload();
		const client = new TestClient(app);

		const multipart = { files: [{ name: "test", filename: "test.txt", content: "<file content>", contentType: "text/plain" }] };
		const response = await client.post("/", { multipart });

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "test"));
		assert(Object.hasOwn(responseData.test, "content"));
		assertEquals(responseData.test.content, "<file content>");
		assert(Object.hasOwn(responseData.test, "content_type"));
		assertEquals(responseData.test.content_type, "text/plain");
		assert(Object.hasOwn(responseData.test, "filename"));
		assertEquals(responseData.test.filename, "test.txt");
		assert(Object.hasOwn(responseData.test, "size"));
		assertEquals(responseData.test.size, 14);
	});

	Deno.test("multipart: Empty file upload", async () => {
		const app = createAppMultipartEmptyFileUpload();
		const client = new TestClient(app);

		const multipart = { files: [{ name: "file", filename: "empty.txt", content: "", contentType: "text/plain" }] };
		const response = await client.post("/files/upload", { multipart });

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "filename"));
		assertEquals(responseData.filename, "empty.txt");
		assert(Object.hasOwn(responseData, "size"));
		assertEquals(responseData.size, 0);
	});

	Deno.test("multipart: Optional file upload - missing", async () => {
		const app = createAppMultipartOptionalFileUploadMissing();
		const client = new TestClient(app);

		const multipart = { files: [] };
		const response = await client.post("/files/optional", { multipart });

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "file"));
		assertEquals(responseData.file, null);
	});

	Deno.test("multipart: File upload without filename", async () => {
		const app = createAppMultipartFileUploadWithoutFilename();
		const client = new TestClient(app);

		const multipart = { files: [{ name: "test1", content: "<file1 content>" }] };
		const response = await client.post("/", { multipart });

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "test1"));
		assertEquals(responseData.test1, "<file1 content>");
	});

	Deno.test("multipart: 18_file_magic_number_jpeg_success", async () => {
		const app = createAppMultipart18FileMagicNumberJpegSuccess();
		const client = new TestClient(app);

		const multipart = { files: [{ name: "image", filename: "test.jpg", contentType: "image/jpeg", magic_bytes: "ffd8ffe0" }] };
		const response = await client.post("/upload", { multipart });

		assertEquals(response.statusCode, 201);
	});

	Deno.test("multipart: 22_file_empty_buffer", async () => {
		const app = createAppMultipart22FileEmptyBuffer();
		const client = new TestClient(app);

		const multipart = { files: [{ name: "file", filename: "empty.txt", contentType: "text/plain", magic_bytes: "" }] };
		const response = await client.post("/upload", { multipart });

		assertEquals(response.statusCode, 422);
	});

	Deno.test("multipart: 17_file_magic_number_png_success", async () => {
		const app = createAppMultipart17FileMagicNumberPngSuccess();
		const client = new TestClient(app);

		const multipart = { files: [{ name: "image", filename: "test.png", contentType: "image/png", magic_bytes: "89504e470d0a1a0a" }] };
		const response = await client.post("/upload", { multipart });

		assertEquals(response.statusCode, 201);
	});

	Deno.test("multipart: Form data without files", async () => {
		const app = createAppMultipartFormDataWithoutFiles();
		const client = new TestClient(app);

		const multipart = { fields: { some: "data" }, files: [] };
		const response = await client.post("/", { multipart });

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "some"));
		assertEquals(responseData.some, "data");
	});

	Deno.test("multipart: Multiple file uploads", async () => {
		const app = createAppMultipartMultipleFileUploads();
		const client = new TestClient(app);

		const multipart = { files: [{ name: "test1", filename: "test1.txt", content: "<file1 content>", contentType: "text/plain" }, { name: "test2", filename: "test2.txt", content: "<file2 content>", contentType: "text/plain" }] };
		const response = await client.post("/", { multipart });

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "test1"));
		assert(Object.hasOwn(responseData.test1, "content"));
		assertEquals(responseData.test1.content, "<file1 content>");
		assert(Object.hasOwn(responseData.test1, "content_type"));
		assertEquals(responseData.test1.content_type, "text/plain");
		assert(Object.hasOwn(responseData.test1, "filename"));
		assertEquals(responseData.test1.filename, "test1.txt");
		assert(Object.hasOwn(responseData.test1, "size"));
		assertEquals(responseData.test1.size, 15);
		assert(Object.hasOwn(responseData, "test2"));
		assert(Object.hasOwn(responseData.test2, "content"));
		assertEquals(responseData.test2.content, "<file2 content>");
		assert(Object.hasOwn(responseData.test2, "content_type"));
		assertEquals(responseData.test2.content_type, "text/plain");
		assert(Object.hasOwn(responseData.test2, "filename"));
		assertEquals(responseData.test2.filename, "test2.txt");
		assert(Object.hasOwn(responseData.test2, "size"));
		assertEquals(responseData.test2.size, 15);
	});

	Deno.test("multipart: File upload with custom headers", async () => {
		const app = createAppMultipartFileUploadWithCustomHeaders();
		const client = new TestClient(app);

		const multipart = { files: [{ name: "test2", filename: "test2.txt", content: "<file2 content>", contentType: "text/plain" }] };
		const response = await client.post("/", { multipart });

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "test2"));
		assert(Object.hasOwn(responseData.test2, "content"));
		assertEquals(responseData.test2.content, "<file2 content>");
		assert(Object.hasOwn(responseData.test2, "content_type"));
		assertEquals(responseData.test2.content_type, "text/plain");
		assert(Object.hasOwn(responseData.test2, "filename"));
		assertEquals(responseData.test2.filename, "test2.txt");
		assert(Object.hasOwn(responseData.test2, "headers"));
		assertEquals(responseData.test2.headers.length, 3);
		assertEquals(responseData.test2.headers[0].length, 2);
		assertEquals(responseData.test2.headers[0][0], "content-disposition");
		assertEquals(responseData.test2.headers[0][1], "form-data; name=\"test2\"; filename=\"test2.txt\"");
		assertEquals(responseData.test2.headers[1].length, 2);
		assertEquals(responseData.test2.headers[1][0], "content-type");
		assertEquals(responseData.test2.headers[1][1], "text/plain");
		assertEquals(responseData.test2.headers[2].length, 2);
		assertEquals(responseData.test2.headers[2][0], "x-custom");
		assertEquals(responseData.test2.headers[2][1], "f2");
		assert(Object.hasOwn(responseData.test2, "size"));
		assertEquals(responseData.test2.size, 15);
	});

	Deno.test("multipart: Required file upload - missing", async () => {
		const app = createAppMultipartRequiredFileUploadMissing();
		const client = new TestClient(app);

		const multipart = { files: [] };
		const response = await client.post("/files/required", { multipart });

		assertEquals(response.statusCode, 422);
	});

	Deno.test("multipart: Image file upload", async () => {
		const app = createAppMultipartImageFileUpload();
		const client = new TestClient(app);

		const multipart = { files: [{ name: "image", filename: "photo.jpg", content: "fake_jpeg_content_here", contentType: "image/jpeg" }] };
		const response = await client.post("/files/image", { multipart });

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "content_type"));
		assertEquals(responseData.content_type, "image/jpeg");
		assert(Object.hasOwn(responseData, "filename"));
		assertEquals(responseData.filename, "photo.jpg");
		assert(Object.hasOwn(responseData, "size"));
		assertEquals(responseData.size, 22);
	});