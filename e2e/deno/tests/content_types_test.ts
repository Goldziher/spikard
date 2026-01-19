/**
 * E2E tests for content_types
 * @generated
 */

import { TestClient } from "@spikard/wasm";
import { assert, assertEquals } from "jsr:@std/assert@1";
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

	Deno.test("content_types: 415 Unsupported Media Type", async () => {
		const app = createAppContentTypes415UnsupportedMediaType();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/xml",
		};
		const json = "<?xml version=\"1.0\"?><item><name>Item</name></item>";
		const response = await client.post("/items/", { headers, json });

		assertEquals(response.statusCode, 415);
	});

	Deno.test("content_types: XML response - application xml", async () => {
		const app = createAppContentTypesXmlResponseApplicationXml();
		const client = new TestClient(app);

		const response = await client.get("/xml");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assertEquals(responseData, "<?xml version=\"1.0\"?><item><name>Item</name><price>42.0</price></item>");
		const responseHeaders = response.headers();
		assertEquals(responseHeaders["content-type"], "application/xml");
	});

	Deno.test("content_types: 14_content_type_case_insensitive", async () => {
		const app = createAppContentTypes14ContentTypeCaseInsensitive();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "APPLICATION/JSON",
		};
		const json = { name: "test" };
		const response = await client.post("/data", { headers, json });

		assertEquals(response.statusCode, 201);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "name"));
		assertEquals(responseData.name, "test");
	});

	Deno.test("content_types: JSON with UTF-8 charset", async () => {
		const app = createAppContentTypesJsonWithUtf8Charset();
		const client = new TestClient(app);

		const response = await client.get("/items/unicode");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "name"));
		assertEquals(responseData.name, "Café");
		assert(Object.hasOwn(responseData, "emoji"));
		assertEquals(responseData.emoji, "☕");
		const responseHeaders = response.headers();
		assertEquals(responseHeaders["content-type"], "application/json; charset=utf-8");
	});

	Deno.test("content_types: 16_text_plain_not_accepted", async () => {
		const app = createAppContentTypes16TextPlainNotAccepted();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "text/plain",
		};
		const json = "{\"data\": \"value\"}";
		const response = await client.post("/data", { headers, json });

		assertEquals(response.statusCode, 415);
	});

	Deno.test("content_types: PDF response - application pdf", async () => {
		const app = createAppContentTypesPdfResponseApplicationPdf();
		const client = new TestClient(app);

		const response = await client.get("/download/document.pdf");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assertEquals(responseData, "pdf_binary_data");
		const responseHeaders = response.headers();
		assertEquals(responseHeaders["content-disposition"], "attachment; filename=document.pdf");
		assertEquals(responseHeaders["content-type"], "application/pdf");
	});

	Deno.test({
		name: "content_types: 20_content_length_mismatch",
		ignore: true,
		fn: async () => {
		// Not supported by the in-memory HTTP client
		},
	});

	Deno.test("content_types: 17_vendor_json_accepted", async () => {
		const app = createAppContentTypes17VendorJsonAccepted();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/vnd.api+json",
		};
		const json = { data: "value" };
		const response = await client.post("/api/v1/resource", { headers, json });

		assertEquals(response.statusCode, 201);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "data"));
		assertEquals(responseData.data, "value");
	});

	Deno.test("content_types: 13_json_with_charset_utf16", async () => {
		const app = createAppContentTypes13JsonWithCharsetUtf16();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json; charset=utf-16",
		};
		const json = { value: "test" };
		const response = await client.post("/data", { headers, json });

		assertEquals(response.statusCode, 415);
	});

	Deno.test("content_types: JSON response - application json", async () => {
		const app = createAppContentTypesJsonResponseApplicationJson();
		const client = new TestClient(app);

		const response = await client.get("/items/json");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "name"));
		assertEquals(responseData.name, "Item");
		assert(Object.hasOwn(responseData, "price"));
		assertEquals(responseData.price, 42.0);
		const responseHeaders = response.headers();
		assertEquals(responseHeaders["content-type"], "application/json");
	});

	Deno.test("content_types: 15_multipart_boundary_required", async () => {
		const app = createAppContentTypes15MultipartBoundaryRequired();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "multipart/form-data",
		};
		const response = await client.post("/upload", { headers });

		assertEquals(response.statusCode, 400);
	});

	Deno.test("content_types: Content negotiation - Accept header", async () => {
		const app = createAppContentTypesContentNegotiationAcceptHeader();
		const client = new TestClient(app);

		const headers = {
			Accept: "application/json",
		};
		const response = await client.get("/accept-test/1", headers);

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "id"));
		assertEquals(responseData.id, 1);
		assert(Object.hasOwn(responseData, "name"));
		assertEquals(responseData.name, "Item");
		const responseHeaders = response.headers();
		assertEquals(responseHeaders["content-type"], "application/json");
	});

	Deno.test("content_types: HTML response - text html", async () => {
		const app = createAppContentTypesHtmlResponseTextHtml();
		const client = new TestClient(app);

		const response = await client.get("/html");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assertEquals(responseData, "<html><body><h1>Hello</h1></body></html>");
		const responseHeaders = response.headers();
		assertEquals(responseHeaders["content-type"], "text/html; charset=utf-8");
	});

	Deno.test("content_types: JPEG image response - image jpeg", async () => {
		const app = createAppContentTypesJpegImageResponseImageJpeg();
		const client = new TestClient(app);

		const response = await client.get("/images/photo.jpg");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assertEquals(responseData, "jpeg_binary_data");
		const responseHeaders = response.headers();
		assertEquals(responseHeaders["content-type"], "image/jpeg");
	});

	Deno.test("content_types: 19_missing_content_type_default_json", async () => {
		const app = createAppContentTypes19MissingContentTypeDefaultJson();
		const client = new TestClient(app);

		const json = { name: "test" };
		const response = await client.post("/data", { json });

		assertEquals(response.statusCode, 201);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "name"));
		assertEquals(responseData.name, "test");
	});

	Deno.test("content_types: PNG image response - image png", async () => {
		const app = createAppContentTypesPngImageResponseImagePng();
		const client = new TestClient(app);

		const response = await client.get("/images/logo.png");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assertEquals(responseData, "png_binary_data");
		const responseHeaders = response.headers();
		assertEquals(responseHeaders["content-type"], "image/png");
	});

	Deno.test("content_types: Plain text response - text plain", async () => {
		const app = createAppContentTypesPlainTextResponseTextPlain();
		const client = new TestClient(app);

		const response = await client.get("/text");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assertEquals(responseData, "Hello, World!");
		const responseHeaders = response.headers();
		assertEquals(responseHeaders["content-type"], "text/plain; charset=utf-8");
	});

	Deno.test("content_types: 18_content_type_with_multiple_params", async () => {
		const app = createAppContentTypes18ContentTypeWithMultipleParams();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json; charset=utf-8; boundary=something",
		};
		const json = { value: "test" };
		const response = await client.post("/data", { headers, json });

		assertEquals(response.statusCode, 201);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "value"));
		assertEquals(responseData.value, "test");
	});

	Deno.test("content_types: CSV response - text csv", async () => {
		const app = createAppContentTypesCsvResponseTextCsv();
		const client = new TestClient(app);

		const response = await client.get("/export/data.csv");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assertEquals(responseData, "id,name,price\n1,Item A,10.0\n2,Item B,20.0");
		const responseHeaders = response.headers();
		assertEquals(responseHeaders["content-disposition"], "attachment; filename=data.csv");
		assertEquals(responseHeaders["content-type"], "text/csv; charset=utf-8");
	});

	Deno.test("content_types: Binary response - application octet-stream", async () => {
		const app = createAppContentTypesBinaryResponseApplicationOctetStream();
		const client = new TestClient(app);

		const response = await client.get("/download/file.bin");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assertEquals(responseData, "binary_data_placeholder");
		const responseHeaders = response.headers();
		assertEquals(responseHeaders["content-type"], "application/octet-stream");
		assertEquals(responseHeaders["content-disposition"], "attachment; filename=file.bin");
	});