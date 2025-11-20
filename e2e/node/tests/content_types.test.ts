/**
 * E2E tests for content_types
 * @generated
 */

import { TestClient } from "@spikard/node";
import { describe, expect, test } from "vitest";
import {
	createAppContentTypes13JsonWithCharsetUtf16,
	createAppContentTypes14ContentTypeCaseInsensitive,
	createAppContentTypes15MultipartBoundaryRequired,
	createAppContentTypes16TextPlainNotAccepted,
	createAppContentTypes17VendorJsonAccepted,
	createAppContentTypes18ContentTypeWithMultipleParams,
	createAppContentTypes19MissingContentTypeDefaultJson,
	createAppContentTypes415UnsupportedMediaType,
	createAppContentTypesBinaryResponseApplicationOctetStream,
	createAppContentTypesContentNegotiationAcceptHeader,
	createAppContentTypesCsvResponseTextCsv,
	createAppContentTypesHtmlResponseTextHtml,
	createAppContentTypesJpegImageResponseImageJpeg,
	createAppContentTypesJsonResponseApplicationJson,
	createAppContentTypesJsonWithUtf8Charset,
	createAppContentTypesPdfResponseApplicationPdf,
	createAppContentTypesPlainTextResponseTextPlain,
	createAppContentTypesPngImageResponseImagePng,
	createAppContentTypesXmlResponseApplicationXml,
} from "../app/main.ts";

describe("content_types", () => {
	test("415 Unsupported Media Type", async () => {
		const app = createAppContentTypes415UnsupportedMediaType();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/xml",
		};
		const json = '<?xml version="1.0"?><item><name>Item</name></item>';
		const response = await client.post("/items/", { headers, json });

		expect(response.statusCode).toBe(415);
	});

	test("XML response - application xml", async () => {
		const app = createAppContentTypesXmlResponseApplicationXml();
		const client = new TestClient(app);

		const response = await client.get("/xml");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toBe('<?xml version="1.0"?><item><name>Item</name><price>42.0</price></item>');
		const responseHeaders = response.headers();
		expect(responseHeaders["content-type"]).toBe("application/xml");
	});

	test("14_content_type_case_insensitive", async () => {
		const app = createAppContentTypes14ContentTypeCaseInsensitive();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "APPLICATION/JSON",
		};
		const json = { name: "test" };
		const response = await client.post("/data", { headers, json });

		expect(response.statusCode).toBe(201);
		const responseData = response.json();
		expect(responseData).toHaveProperty("name");
		expect(responseData.name).toBe("test");
	});

	test("JSON with UTF-8 charset", async () => {
		const app = createAppContentTypesJsonWithUtf8Charset();
		const client = new TestClient(app);

		const response = await client.get("/items/unicode");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("emoji");
		expect(responseData.emoji).toBe("☕");
		expect(responseData).toHaveProperty("name");
		expect(responseData.name).toBe("Café");
		const responseHeaders = response.headers();
		expect(responseHeaders["content-type"]).toBe("application/json; charset=utf-8");
	});

	test("16_text_plain_not_accepted", async () => {
		const app = createAppContentTypes16TextPlainNotAccepted();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "text/plain",
		};
		const json = '{"data": "value"}';
		const response = await client.post("/data", { headers, json });

		expect(response.statusCode).toBe(415);
	});

	test("PDF response - application pdf", async () => {
		const app = createAppContentTypesPdfResponseApplicationPdf();
		const client = new TestClient(app);

		const response = await client.get("/download/document.pdf");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toBe("pdf_binary_data");
		const responseHeaders = response.headers();
		expect(responseHeaders["content-disposition"]).toBe("attachment; filename=document.pdf");
		expect(responseHeaders["content-type"]).toBe("application/pdf");
	});

	test.skip("20_content_length_mismatch", async () => {
		// Not supported by the in-memory HTTP client
	});

	test("17_vendor_json_accepted", async () => {
		const app = createAppContentTypes17VendorJsonAccepted();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/vnd.api+json",
		};
		const json = { data: "value" };
		const response = await client.post("/api/v1/resource", { headers, json });

		expect(response.statusCode).toBe(201);
		const responseData = response.json();
		expect(responseData).toHaveProperty("data");
		expect(responseData.data).toBe("value");
	});

	test("13_json_with_charset_utf16", async () => {
		const app = createAppContentTypes13JsonWithCharsetUtf16();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json; charset=utf-16",
		};
		const json = { value: "test" };
		const response = await client.post("/data", { headers, json });

		expect(response.statusCode).toBe(415);
	});

	test("JSON response - application json", async () => {
		const app = createAppContentTypesJsonResponseApplicationJson();
		const client = new TestClient(app);

		const response = await client.get("/items/json");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("name");
		expect(responseData.name).toBe("Item");
		expect(responseData).toHaveProperty("price");
		expect(responseData.price).toBe(42.0);
		const responseHeaders = response.headers();
		expect(responseHeaders["content-type"]).toBe("application/json");
	});

	test("15_multipart_boundary_required", async () => {
		const app = createAppContentTypes15MultipartBoundaryRequired();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "multipart/form-data",
		};
		const response = await client.post("/upload", { headers });

		expect(response.statusCode).toBe(400);
	});

	test("Content negotiation - Accept header", async () => {
		const app = createAppContentTypesContentNegotiationAcceptHeader();
		const client = new TestClient(app);

		const headers = {
			Accept: "application/json",
		};
		const response = await client.get("/accept-test/1", headers);

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("id");
		expect(responseData.id).toBe(1);
		expect(responseData).toHaveProperty("name");
		expect(responseData.name).toBe("Item");
		const responseHeaders = response.headers();
		expect(responseHeaders["content-type"]).toBe("application/json");
	});

	test("HTML response - text html", async () => {
		const app = createAppContentTypesHtmlResponseTextHtml();
		const client = new TestClient(app);

		const response = await client.get("/html");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toBe("<html><body><h1>Hello</h1></body></html>");
		const responseHeaders = response.headers();
		expect(responseHeaders["content-type"]).toBe("text/html; charset=utf-8");
	});

	test("JPEG image response - image jpeg", async () => {
		const app = createAppContentTypesJpegImageResponseImageJpeg();
		const client = new TestClient(app);

		const response = await client.get("/images/photo.jpg");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toBe("jpeg_binary_data");
		const responseHeaders = response.headers();
		expect(responseHeaders["content-type"]).toBe("image/jpeg");
	});

	test("19_missing_content_type_default_json", async () => {
		const app = createAppContentTypes19MissingContentTypeDefaultJson();
		const client = new TestClient(app);

		const json = { name: "test" };
		const response = await client.post("/data", { json });

		expect(response.statusCode).toBe(201);
		const responseData = response.json();
		expect(responseData).toHaveProperty("name");
		expect(responseData.name).toBe("test");
	});

	test("PNG image response - image png", async () => {
		const app = createAppContentTypesPngImageResponseImagePng();
		const client = new TestClient(app);

		const response = await client.get("/images/logo.png");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toBe("png_binary_data");
		const responseHeaders = response.headers();
		expect(responseHeaders["content-type"]).toBe("image/png");
	});

	test("Plain text response - text plain", async () => {
		const app = createAppContentTypesPlainTextResponseTextPlain();
		const client = new TestClient(app);

		const response = await client.get("/text");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toBe("Hello, World!");
		const responseHeaders = response.headers();
		expect(responseHeaders["content-type"]).toBe("text/plain; charset=utf-8");
	});

	test("18_content_type_with_multiple_params", async () => {
		const app = createAppContentTypes18ContentTypeWithMultipleParams();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json; charset=utf-8; boundary=something",
		};
		const json = { value: "test" };
		const response = await client.post("/data", { headers, json });

		expect(response.statusCode).toBe(201);
		const responseData = response.json();
		expect(responseData).toHaveProperty("value");
		expect(responseData.value).toBe("test");
	});

	test("CSV response - text csv", async () => {
		const app = createAppContentTypesCsvResponseTextCsv();
		const client = new TestClient(app);

		const response = await client.get("/export/data.csv");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toBe("id,name,price\n1,Item A,10.0\n2,Item B,20.0");
		const responseHeaders = response.headers();
		expect(responseHeaders["content-type"]).toBe("text/csv; charset=utf-8");
		expect(responseHeaders["content-disposition"]).toBe("attachment; filename=data.csv");
	});

	test("Binary response - application octet-stream", async () => {
		const app = createAppContentTypesBinaryResponseApplicationOctetStream();
		const client = new TestClient(app);

		const response = await client.get("/download/file.bin");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toBe("binary_data_placeholder");
		const responseHeaders = response.headers();
		expect(responseHeaders["content-disposition"]).toBe("attachment; filename=file.bin");
		expect(responseHeaders["content-type"]).toBe("application/octet-stream");
	});
});
