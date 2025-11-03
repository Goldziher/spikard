/**
 * E2E tests for multipart
 * @generated
 */

import { describe, test, expect } from "vitest";
import { TestClient } from "@spikard/node";
import { createAppMultipartMultipleValuesForSameFieldName, createAppMultipart19FileMimeSpoofingPngAsJpeg, createAppMultipart20FileMimeSpoofingJpegAsPng, createAppMultipart21FilePdfMagicNumberSuccess, createAppMultipartContentTypeValidationInvalidType, createAppMultipartPdfFileUpload, createAppMultipartFileListUploadArrayOfFiles, createAppMultipartOptionalFileUploadProvided, createAppMultipartFileSizeValidationTooLarge, createAppMultipartMixedFilesAndFormData, createAppMultipartSimpleFileUpload, createAppMultipartEmptyFileUpload, createAppMultipartOptionalFileUploadMissing, createAppMultipartFileUploadWithoutFilename, createAppMultipart18FileMagicNumberJpegSuccess, createAppMultipart22FileEmptyBuffer, createAppMultipart17FileMagicNumberPngSuccess, createAppMultipartFormDataWithoutFiles, createAppMultipartMultipleFileUploads, createAppMultipartFileUploadWithCustomHeaders, createAppMultipartRequiredFileUploadMissing, createAppMultipartImageFileUpload } from "../app/main.js";

describe("multipart", () => {
	test("Multiple values for same field name", async () => {
		const app = createAppMultipartMultipleValuesForSameFieldName();
		const client = new TestClient(app);

		const response = await client.post("/", {});

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("files");
		expect(responseData["files"].length).toBe(2);
		expect(responseData["files"][0]).toHaveProperty("content");
		expect(responseData["files"][0]["content"]).toBe("first file");
		expect(responseData["files"][0]).toHaveProperty("content_type");
		expect(responseData["files"][0]["content_type"]).toBe("text/plain");
		expect(responseData["files"][0]).toHaveProperty("filename");
		expect(responseData["files"][0]["filename"]).toBe("file1.txt");
		expect(responseData["files"][0]).toHaveProperty("size");
		expect(responseData["files"][0]["size"]).toBe(10);
		expect(responseData["files"][1]).toHaveProperty("content");
		expect(responseData["files"][1]["content"]).toBe("second file");
		expect(responseData["files"][1]).toHaveProperty("content_type");
		expect(responseData["files"][1]["content_type"]).toBe("text/plain");
		expect(responseData["files"][1]).toHaveProperty("filename");
		expect(responseData["files"][1]["filename"]).toBe("file2.txt");
		expect(responseData["files"][1]).toHaveProperty("size");
		expect(responseData["files"][1]["size"]).toBe(11);
		expect(responseData).toHaveProperty("tags");
		expect(responseData["tags"].length).toBe(3);
		expect(responseData["tags"][0]).toBe("python");
		expect(responseData["tags"][1]).toBe("rust");
		expect(responseData["tags"][2]).toBe("web");
	});

	test("19_file_mime_spoofing_png_as_jpeg", async () => {
		const app = createAppMultipart19FileMimeSpoofingPngAsJpeg();
		const client = new TestClient(app);

		const response = await client.post("/upload", {});

		expect(response.statusCode).toBe(422);
		const responseData = response.json();
		// Validation should be done by framework, not handler
		expect(responseData).toHaveProperty("errors");
	});

	test("20_file_mime_spoofing_jpeg_as_png", async () => {
		const app = createAppMultipart20FileMimeSpoofingJpegAsPng();
		const client = new TestClient(app);

		const response = await client.post("/upload", {});

		expect(response.statusCode).toBe(422);
		const responseData = response.json();
		// Validation should be done by framework, not handler
		expect(responseData).toHaveProperty("errors");
	});

	test("21_file_pdf_magic_number_success", async () => {
		const app = createAppMultipart21FilePdfMagicNumberSuccess();
		const client = new TestClient(app);

		const response = await client.post("/upload", {});

		expect(response.statusCode).toBe(201);
	});

	test("Content-Type validation - invalid type", async () => {
		const app = createAppMultipartContentTypeValidationInvalidType();
		const client = new TestClient(app);

		const response = await client.post("/files/images-only", {});

		expect(response.statusCode).toBe(422);
		const responseData = response.json();
		// Validation should be done by framework, not handler
		expect(responseData).toHaveProperty("errors");
	});

	test("PDF file upload", async () => {
		const app = createAppMultipartPdfFileUpload();
		const client = new TestClient(app);

		const response = await client.post("/files/document", {});

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("content_type");
		expect(responseData["content_type"]).toBe("application/pdf");
		expect(responseData).toHaveProperty("filename");
		expect(responseData["filename"]).toBe("report.pdf");
		expect(responseData).toHaveProperty("size");
		expect(responseData["size"]).toBe(16);
	});

	test("File list upload array of files", async () => {
		const app = createAppMultipartFileListUploadArrayOfFiles();
		const client = new TestClient(app);

		const response = await client.post("/files/list", {});

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("filenames");
		expect(responseData["filenames"].length).toBe(2);
		expect(responseData["filenames"][0]).toBe("file1.txt");
		expect(responseData["filenames"][1]).toBe("file2.txt");
		expect(responseData).toHaveProperty("total_size");
		expect(responseData["total_size"]).toBe(35);
	});

	test("Optional file upload - provided", async () => {
		const app = createAppMultipartOptionalFileUploadProvided();
		const client = new TestClient(app);

		const response = await client.post("/files/optional", {});

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("content_type");
		expect(responseData["content_type"]).toBe("text/plain");
		expect(responseData).toHaveProperty("filename");
		expect(responseData["filename"]).toBe("optional.txt");
		expect(responseData).toHaveProperty("size");
		expect(responseData["size"]).toBe(21);
	});

	test("File size validation - too large", async () => {
		const app = createAppMultipartFileSizeValidationTooLarge();
		const client = new TestClient(app);

		const response = await client.post("/files/validated", {});

		expect(response.statusCode).toBe(413);
		const responseData = response.json();
		expect(responseData).toHaveProperty("detail");
		expect(responseData["detail"]).toBe("File too large. Maximum size is 1MB");
	});

	test("Mixed files and form data", async () => {
		const app = createAppMultipartMixedFilesAndFormData();
		const client = new TestClient(app);

		const response = await client.post("/", {});

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("active");
		expect(responseData["active"]).toBe("true");
		expect(responseData).toHaveProperty("age");
		expect(responseData["age"]).toBe("25");
		expect(responseData).toHaveProperty("file");
		expect(responseData["file"]).toHaveProperty("content");
		expect(responseData["file"]["content"]).toBe("file data here");
		expect(responseData["file"]).toHaveProperty("content_type");
		expect(responseData["file"]["content_type"]).toBe("text/plain");
		expect(responseData["file"]).toHaveProperty("filename");
		expect(responseData["file"]["filename"]).toBe("upload.txt");
		expect(responseData["file"]).toHaveProperty("size");
		expect(responseData["file"]["size"]).toBe(14);
		expect(responseData).toHaveProperty("username");
		expect(responseData["username"]).toBe("testuser");
	});

	test("Simple file upload", async () => {
		const app = createAppMultipartSimpleFileUpload();
		const client = new TestClient(app);

		const response = await client.post("/", {});

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("test");
		expect(responseData["test"]).toHaveProperty("content");
		expect(responseData["test"]["content"]).toBe("<file content>");
		expect(responseData["test"]).toHaveProperty("content_type");
		expect(responseData["test"]["content_type"]).toBe("text/plain");
		expect(responseData["test"]).toHaveProperty("filename");
		expect(responseData["test"]["filename"]).toBe("test.txt");
		expect(responseData["test"]).toHaveProperty("size");
		expect(responseData["test"]["size"]).toBe(14);
	});

	test("Empty file upload", async () => {
		const app = createAppMultipartEmptyFileUpload();
		const client = new TestClient(app);

		const response = await client.post("/files/upload", {});

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("filename");
		expect(responseData["filename"]).toBe("empty.txt");
		expect(responseData).toHaveProperty("size");
		expect(responseData["size"]).toBe(0);
	});

	test("Optional file upload - missing", async () => {
		const app = createAppMultipartOptionalFileUploadMissing();
		const client = new TestClient(app);

		const json = {
		};
		const response = await client.post("/files/optional", {json});

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("file");
		expect(responseData["file"]).toBe(null);
	});

	test("File upload without filename", async () => {
		const app = createAppMultipartFileUploadWithoutFilename();
		const client = new TestClient(app);

		const response = await client.post("/", {});

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("test1");
		expect(responseData["test1"]).toBe("<file1 content>");
	});

	test("18_file_magic_number_jpeg_success", async () => {
		const app = createAppMultipart18FileMagicNumberJpegSuccess();
		const client = new TestClient(app);

		const response = await client.post("/upload", {});

		expect(response.statusCode).toBe(201);
	});

	test("22_file_empty_buffer", async () => {
		const app = createAppMultipart22FileEmptyBuffer();
		const client = new TestClient(app);

		const response = await client.post("/upload", {});

		expect(response.statusCode).toBe(422);
		const responseData = response.json();
		// Validation should be done by framework, not handler
		expect(responseData).toHaveProperty("errors");
	});

	test("17_file_magic_number_png_success", async () => {
		const app = createAppMultipart17FileMagicNumberPngSuccess();
		const client = new TestClient(app);

		const response = await client.post("/upload", {});

		expect(response.statusCode).toBe(201);
	});

	test("Form data without files", async () => {
		const app = createAppMultipartFormDataWithoutFiles();
		const client = new TestClient(app);

		const response = await client.post("/", {});

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("some");
		expect(responseData["some"]).toBe("data");
	});

	test("Multiple file uploads", async () => {
		const app = createAppMultipartMultipleFileUploads();
		const client = new TestClient(app);

		const response = await client.post("/", {});

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("test1");
		expect(responseData["test1"]).toHaveProperty("content");
		expect(responseData["test1"]["content"]).toBe("<file1 content>");
		expect(responseData["test1"]).toHaveProperty("content_type");
		expect(responseData["test1"]["content_type"]).toBe("text/plain");
		expect(responseData["test1"]).toHaveProperty("filename");
		expect(responseData["test1"]["filename"]).toBe("test1.txt");
		expect(responseData["test1"]).toHaveProperty("size");
		expect(responseData["test1"]["size"]).toBe(15);
		expect(responseData).toHaveProperty("test2");
		expect(responseData["test2"]).toHaveProperty("content");
		expect(responseData["test2"]["content"]).toBe("<file2 content>");
		expect(responseData["test2"]).toHaveProperty("content_type");
		expect(responseData["test2"]["content_type"]).toBe("text/plain");
		expect(responseData["test2"]).toHaveProperty("filename");
		expect(responseData["test2"]["filename"]).toBe("test2.txt");
		expect(responseData["test2"]).toHaveProperty("size");
		expect(responseData["test2"]["size"]).toBe(15);
	});

	test("File upload with custom headers", async () => {
		const app = createAppMultipartFileUploadWithCustomHeaders();
		const client = new TestClient(app);

		const response = await client.post("/", {});

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("test2");
		expect(responseData["test2"]).toHaveProperty("content");
		expect(responseData["test2"]["content"]).toBe("<file2 content>");
		expect(responseData["test2"]).toHaveProperty("content_type");
		expect(responseData["test2"]["content_type"]).toBe("text/plain");
		expect(responseData["test2"]).toHaveProperty("filename");
		expect(responseData["test2"]["filename"]).toBe("test2.txt");
		expect(responseData["test2"]).toHaveProperty("headers");
		expect(responseData["test2"]["headers"].length).toBe(3);
		expect(responseData["test2"]["headers"][0].length).toBe(2);
		expect(responseData["test2"]["headers"][0][0]).toBe("content-disposition");
		expect(responseData["test2"]["headers"][0][1]).toBe("form-data; name=\"test2\"; filename=\"test2.txt\"");
		expect(responseData["test2"]["headers"][1].length).toBe(2);
		expect(responseData["test2"]["headers"][1][0]).toBe("content-type");
		expect(responseData["test2"]["headers"][1][1]).toBe("text/plain");
		expect(responseData["test2"]["headers"][2].length).toBe(2);
		expect(responseData["test2"]["headers"][2][0]).toBe("x-custom");
		expect(responseData["test2"]["headers"][2][1]).toBe("f2");
		expect(responseData["test2"]).toHaveProperty("size");
		expect(responseData["test2"]["size"]).toBe(15);
	});

	test("Required file upload - missing", async () => {
		const app = createAppMultipartRequiredFileUploadMissing();
		const client = new TestClient(app);

		const json = {
		};
		const response = await client.post("/files/required", {json});

		expect(response.statusCode).toBe(422);
		const responseData = response.json();
		// Validation should be done by framework, not handler
		expect(responseData).toHaveProperty("errors");
	});

	test("Image file upload", async () => {
		const app = createAppMultipartImageFileUpload();
		const client = new TestClient(app);

		const response = await client.post("/files/image", {});

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("content_type");
		expect(responseData["content_type"]).toBe("image/jpeg");
		expect(responseData).toHaveProperty("filename");
		expect(responseData["filename"]).toBe("photo.jpg");
		expect(responseData).toHaveProperty("size");
		expect(responseData["size"]).toBe(22);
	});

});
