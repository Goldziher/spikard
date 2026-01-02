/**
 * GraphQL mutation tests
 * @generated
 */

import { TestClient } from "@spikard/wasm";
import { describe, expect, test } from "vitest";
import { createAppGraphqlMutation } from "../app/main.ts";

describe("GraphQL mutation", () => {
	test("validation_directive", async () => {
		const app = createAppGraphqlMutation();
		const client = new TestClient(app);

		const response = await client.post(
			"/graphql",
			{
				headers: { "x-spikard-fixture": "validation_directive" },
				json: {
					query: `mutation CreateUser($input: CreateUserInput!) {\n  createUser(input: $input) {\n    id\n    name\n    bio\n  }\n}`,
					variables: { input: { name: "a", bio: null } },
					operationName: null,
				},
			},
		);

		expect(response.statusCode).toBe(422);
		const responseBody = response.json();

		const errors = responseBody.errors;
		expect(errors).toBeDefined();
		expect(errors?.length).toBe(1);
		expect(errors?.[0]?.message).toContain("Validation error on input field 'name': String length must be between 3 and 50 characters (provided: 1)");
	});

	test("custom_scalar_invalid", async () => {
		const app = createAppGraphqlMutation();
		const client = new TestClient(app);

		const response = await client.post(
			"/graphql",
			{
				headers: { "x-spikard-fixture": "custom_scalar_invalid" },
				json: {
					query: `mutation CreateContact($input: CreateContactInput!) {\n  createContact(input: $input) {\n    id\n    name\n    email\n    website\n    phone\n    createdAt\n  }\n}`,
					variables: { input: { name: "Invalid Contact", email: "not-an-email", website: "not a url", phone: "123" } },
					operationName: "CreateContact",
				},
			},
		);

		expect(response.statusCode).toBe(422);
		const responseBody = response.json();

		const errors = responseBody.errors;
		expect(errors).toBeDefined();
		expect(errors?.length).toBe(3);
		expect(errors?.[0]?.message).toContain("Email must be a valid email address");
		expect(errors?.[1]?.message).toContain("URL must start with http:// or https://");
		expect(errors?.[2]?.message).toContain("PhoneNumber must be a valid E.164 format");
	});

	test("custom_scalar_validation", async () => {
		const app = createAppGraphqlMutation();
		const client = new TestClient(app);

		const response = await client.post(
			"/graphql",
			{
				headers: { "x-spikard-fixture": "custom_scalar_validation" },
				json: {
					query: `mutation CreateContact($input: CreateContactInput!) {\n  createContact(input: $input) {\n    id\n    name\n    email\n    website\n    phone\n    createdAt\n  }\n}`,
					variables: { input: { name: "Alice Johnson", email: "alice.johnson@example.com", website: "https://example.com", phone: "+1-555-123-4567" } },
					operationName: "CreateContact",
				},
			},
		);

		expect(response.statusCode).toBe(200);
		const responseBody = response.json();

		const data = responseBody.data;
		expect(data).toHaveProperty("createContact");
		expect(data.createContact).toHaveProperty("id");
		expect(data.createContact.id).toBe("contact-001");
		expect(data.createContact).toHaveProperty("name");
		expect(data.createContact.name).toBe("Alice Johnson");
		expect(data.createContact).toHaveProperty("email");
		expect(data.createContact.email).toBe("alice.johnson@example.com");
		expect(data.createContact).toHaveProperty("website");
		expect(data.createContact.website).toBe("https://example.com");
		expect(data.createContact).toHaveProperty("phone");
		expect(data.createContact.phone).toBe("+1-555-123-4567");
		expect(data.createContact).toHaveProperty("createdAt");
		expect(data.createContact.createdAt).toBe("2025-12-27T14:30:00Z");
		const errors = responseBody.errors;
		expect(errors?.length ?? 0).toBe(0);
	});

	test("create_resource", async () => {
		const app = createAppGraphqlMutation();
		const client = new TestClient(app);

		const response = await client.post(
			"/graphql",
			{
				headers: { "x-spikard-fixture": "create_resource" },
				json: {
					query: `mutation CreateUser($input: CreateUserInput!) {\n  createUser(input: $input) {\n    id\n    name\n    email\n    role\n    createdAt\n  }\n}`,
					variables: { input: { name: "John Doe", email: "john@example.com", role: "admin" } },
					operationName: "CreateUser",
				},
			},
		);

		expect(response.statusCode).toBe(200);
		const responseBody = response.json();

		const data = responseBody.data;
		expect(data).toHaveProperty("createUser");
		expect(data.createUser).toHaveProperty("id");
		expect(data.createUser.id).toBe("user-123");
		expect(data.createUser).toHaveProperty("name");
		expect(data.createUser.name).toBe("John Doe");
		expect(data.createUser).toHaveProperty("email");
		expect(data.createUser.email).toBe("john@example.com");
		expect(data.createUser).toHaveProperty("role");
		expect(data.createUser.role).toBe("admin");
		expect(data.createUser).toHaveProperty("createdAt");
		expect(data.createUser.createdAt).toBe("2025-12-27T10:30:00Z");
		const errors = responseBody.errors;
		expect(errors?.length ?? 0).toBe(0);
	});

	test("delete_resource", async () => {
		const app = createAppGraphqlMutation();
		const client = new TestClient(app);

		const response = await client.post(
			"/graphql",
			{
				headers: { "x-spikard-fixture": "delete_resource" },
				json: {
					query: `mutation DeleteUser($id: ID!) {\n  deleteUser(id: $id) {\n    success\n    message\n    deletedId\n  }\n}`,
					variables: { id: "user-123" },
					operationName: "DeleteUser",
				},
			},
		);

		expect(response.statusCode).toBe(200);
		const responseBody = response.json();

		const data = responseBody.data;
		expect(data).toHaveProperty("deleteUser");
		expect(data.deleteUser).toHaveProperty("success");
		expect(data.deleteUser.success).toBe(true);
		expect(data.deleteUser).toHaveProperty("message");
		expect(data.deleteUser.message).toBe("User successfully deleted");
		expect(data.deleteUser).toHaveProperty("deletedId");
		expect(data.deleteUser.deletedId).toBe("user-123");
		const errors = responseBody.errors;
		expect(errors?.length ?? 0).toBe(0);
	});

	test("update_resource", async () => {
		const app = createAppGraphqlMutation();
		const client = new TestClient(app);

		const response = await client.post(
			"/graphql",
			{
				headers: { "x-spikard-fixture": "update_resource" },
				json: {
					query: `mutation UpdateUser($id: ID!, $input: UpdateUserInput!) {\n  updateUser(id: $id, input: $input) {\n    id\n    name\n    email\n    role\n    updatedAt\n  }\n}`,
					variables: { id: "user-123", input: { email: "john.doe@example.com", role: "editor" } },
					operationName: "UpdateUser",
				},
			},
		);

		expect(response.statusCode).toBe(200);
		const responseBody = response.json();

		const data = responseBody.data;
		expect(data).toHaveProperty("updateUser");
		expect(data.updateUser).toHaveProperty("id");
		expect(data.updateUser.id).toBe("user-123");
		expect(data.updateUser).toHaveProperty("name");
		expect(data.updateUser.name).toBe("John Doe");
		expect(data.updateUser).toHaveProperty("email");
		expect(data.updateUser.email).toBe("john.doe@example.com");
		expect(data.updateUser).toHaveProperty("role");
		expect(data.updateUser.role).toBe("editor");
		expect(data.updateUser).toHaveProperty("updatedAt");
		expect(data.updateUser.updatedAt).toBe("2025-12-27T11:45:00Z");
		const errors = responseBody.errors;
		expect(errors?.length ?? 0).toBe(0);
	});

	test("validation_error", async () => {
		const app = createAppGraphqlMutation();
		const client = new TestClient(app);

		const response = await client.post(
			"/graphql",
			{
				headers: { "x-spikard-fixture": "validation_error" },
				json: {
					query: `mutation CreatePost($input: CreatePostInput!) {\n  createPost(input: $input) {\n    id\n    title\n    content\n    tags\n    createdAt\n  }\n}`,
					variables: { input: { title: "My Post", content: "This is a post" } },
					operationName: null,
				},
			},
		);

		expect(response.statusCode).toBe(400);
		const responseBody = response.json();

		const errors = responseBody.errors;
		expect(errors).toBeDefined();
		expect(errors?.length).toBe(1);
		expect(errors?.[0]?.message).toContain("Field \"CreatePostInput.tags\" of required type \"[String!]!\" was not provided.");
	});

	test("mutation_permission_check", async () => {
		const app = createAppGraphqlMutation();
		const client = new TestClient(app);

		const response = await client.post(
			"/graphql",
			{
				headers: { "x-spikard-fixture": "mutation_permission_check" },
				json: {
					query: `mutation DeleteUser($userId: String!) {\n  deleteUser(id: $userId) {\n    success\n    message\n  }\n}`,
					variables: { userId: "user123" },
					operationName: null,
				},
			},
		);

		expect(response.statusCode).toBe(403);
		const responseBody = response.json();

		const errors = responseBody.errors;
		expect(errors).toBeDefined();
		expect(errors?.length).toBe(1);
		expect(errors?.[0]?.message).toContain("Missing required permission: DELETE");
	});

	test("dynamic_authorization", async () => {
		const app = createAppGraphqlMutation();
		const client = new TestClient(app);

		const response = await client.post(
			"/graphql",
			{
				headers: { "x-spikard-fixture": "dynamic_authorization" },
				json: {
					query: `mutation ApprovePost($postId: String!) {\n  approvePost(id: $postId) {\n    success\n    postId\n    status\n  }\n}`,
					variables: { postId: "post123" },
					operationName: null,
				},
			},
		);

		expect(response.statusCode).toBe(403);
		const responseBody = response.json();

		const errors = responseBody.errors;
		expect(errors).toBeDefined();
		expect(errors?.length).toBe(1);
		expect(errors?.[0]?.message).toContain("Only post author or admin can approve posts");
	});

	test("required_fields", async () => {
		const app = createAppGraphqlMutation();
		const client = new TestClient(app);

		const response = await client.post(
			"/graphql",
			{
				headers: { "x-spikard-fixture": "required_fields" },
				json: {
					query: `mutation Register($input: UserRegistrationInput!) {\n  registerUser(input: $input) {\n    success\n    userId\n    message\n  }\n}`,
					variables: { input: { username: "johndoe", email: "john@example.com" } },
					operationName: null,
				},
			},
		);

		expect(response.statusCode).toBe(400);
		const responseBody = response.json();

		const errors = responseBody.errors;
		expect(errors).toBeDefined();
		expect(errors?.length).toBe(1);
		expect(errors?.[0]?.message).toContain("Field \"UserRegistrationInput.password\" of required type \"String!\" was not provided.");
	});

	test("file_upload_validation_type", async () => {
		const app = createAppGraphqlMutation();
		const client = new TestClient(app);

		const response = await client.post(
			"/graphql",
			{
				headers: { "x-spikard-fixture": "file_upload_validation_type" },
				json: {
					query: `mutation UploadImage($file: Upload!) {\n  uploadImage(file: $file) {\n    id\n    filename\n    mimetype\n    width\n    height\n  }\n}`,
					variables: { file: null },
					operationName: "UploadImage",
				},
			},
		);

		expect(response.statusCode).toBe(400);
		const responseBody = response.json();

		const errors = responseBody.errors;
		expect(errors).toBeDefined();
		expect(errors?.length).toBe(1);
		expect(errors?.[0]?.message).toContain("Invalid file type");
	});

	test("multiple_files_upload", async () => {
		const app = createAppGraphqlMutation();
		const client = new TestClient(app);

		const response = await client.post(
			"/graphql",
			{
				headers: { "x-spikard-fixture": "multiple_files_upload" },
				json: {
					query: `mutation MultipleUpload($files: [Upload!]!) {\n  multipleUpload(files: $files) {\n    id\n    filename\n    mimetype\n    size\n  }\n}`,
					variables: { files: [null, null, null] },
					operationName: "MultipleUpload",
				},
			},
		);

		expect(response.statusCode).toBe(200);
		const responseBody = response.json();

		const data = responseBody.data;
		expect(data).toHaveProperty("multipleUpload");
		expect(data.multipleUpload.length).toBe(3);
		expect(data.multipleUpload[0]).toHaveProperty("id");
		expect(data.multipleUpload[0].id).toBe("file-002");
		expect(data.multipleUpload[0]).toHaveProperty("filename");
		expect(data.multipleUpload[0].filename).toBe("document.pdf");
		expect(data.multipleUpload[0]).toHaveProperty("mimetype");
		expect(data.multipleUpload[0].mimetype).toBe("application/pdf");
		expect(data.multipleUpload[0]).toHaveProperty("size");
		expect(data.multipleUpload[0].size).toBe(32);
		expect(data.multipleUpload[1]).toHaveProperty("id");
		expect(data.multipleUpload[1].id).toBe("file-003");
		expect(data.multipleUpload[1]).toHaveProperty("filename");
		expect(data.multipleUpload[1].filename).toBe("image.png");
		expect(data.multipleUpload[1]).toHaveProperty("mimetype");
		expect(data.multipleUpload[1].mimetype).toBe("image/png");
		expect(data.multipleUpload[1]).toHaveProperty("size");
		expect(data.multipleUpload[1].size).toBe(24);
		expect(data.multipleUpload[2]).toHaveProperty("id");
		expect(data.multipleUpload[2].id).toBe("file-004");
		expect(data.multipleUpload[2]).toHaveProperty("filename");
		expect(data.multipleUpload[2].filename).toBe("data.csv");
		expect(data.multipleUpload[2]).toHaveProperty("mimetype");
		expect(data.multipleUpload[2].mimetype).toBe("text/csv");
		expect(data.multipleUpload[2]).toHaveProperty("size");
		expect(data.multipleUpload[2].size).toBe(68);
		const errors = responseBody.errors;
		expect(errors?.length ?? 0).toBe(0);
	});

	test("file_upload_multipart_spec", async () => {
		const app = createAppGraphqlMutation();
		const client = new TestClient(app);

		const response = await client.post(
			"/graphql",
			{
				headers: { "x-spikard-fixture": "file_upload_multipart_spec" },
				json: {
					query: `mutation UploadDocument($title: String!, $files: [Upload!]!) {\n  uploadDocument(title: $title, files: $files) {\n    id\n    title\n    files {\n      id\n      filename\n      mimetype\n      size\n    }\n    uploadedAt\n  }\n}`,
					variables: { title: "Important Documents", files: [null, null] },
					operationName: "UploadDocument",
				},
			},
		);

		expect(response.statusCode).toBe(200);
		const responseBody = response.json();

		const data = responseBody.data;
		expect(data).toHaveProperty("uploadDocument");
		expect(data.uploadDocument).toHaveProperty("id");
		expect(data.uploadDocument.id).toBe("doc-001");
		expect(data.uploadDocument).toHaveProperty("title");
		expect(data.uploadDocument.title).toBe("Important Documents");
		expect(data.uploadDocument).toHaveProperty("files");
		expect(data.uploadDocument.files.length).toBe(2);
		expect(data.uploadDocument.files[0]).toHaveProperty("id");
		expect(data.uploadDocument.files[0].id).toBe("file-006");
		expect(data.uploadDocument.files[0]).toHaveProperty("filename");
		expect(data.uploadDocument.files[0].filename).toBe("contract.pdf");
		expect(data.uploadDocument.files[0]).toHaveProperty("mimetype");
		expect(data.uploadDocument.files[0].mimetype).toBe("application/pdf");
		expect(data.uploadDocument.files[0]).toHaveProperty("size");
		expect(data.uploadDocument.files[0].size).toBe(88);
		expect(data.uploadDocument.files[1]).toHaveProperty("id");
		expect(data.uploadDocument.files[1].id).toBe("file-007");
		expect(data.uploadDocument.files[1]).toHaveProperty("filename");
		expect(data.uploadDocument.files[1].filename).toBe("summary.txt");
		expect(data.uploadDocument.files[1]).toHaveProperty("mimetype");
		expect(data.uploadDocument.files[1].mimetype).toBe("text/plain");
		expect(data.uploadDocument.files[1]).toHaveProperty("size");
		expect(data.uploadDocument.files[1].size).toBe(65);
		expect(data.uploadDocument).toHaveProperty("uploadedAt");
		expect(data.uploadDocument.uploadedAt).toBe("2025-12-27T14:30:00Z");
		const errors = responseBody.errors;
		expect(errors?.length ?? 0).toBe(0);
	});

	test("file_upload_validation_size", async () => {
		const app = createAppGraphqlMutation();
		const client = new TestClient(app);

		const response = await client.post(
			"/graphql",
			{
				headers: { "x-spikard-fixture": "file_upload_validation_size" },
				json: {
					query: `mutation Upload($file: Upload!) {\n  singleUpload(file: $file) {\n    id\n    filename\n    mimetype\n    size\n  }\n}`,
					variables: { file: null },
					operationName: "Upload",
				},
			},
		);

		expect(response.statusCode).toBe(400);
		const responseBody = response.json();

		const errors = responseBody.errors;
		expect(errors).toBeDefined();
		expect(errors?.length).toBe(1);
		expect(errors?.[0]?.message).toContain("File too large");
	});

	test("single_file_upload", async () => {
		const app = createAppGraphqlMutation();
		const client = new TestClient(app);

		const response = await client.post(
			"/graphql",
			{
				headers: { "x-spikard-fixture": "single_file_upload" },
				json: {
					query: `mutation Upload($file: Upload!) {\n  singleUpload(file: $file) {\n    id\n    filename\n    mimetype\n    size\n  }\n}`,
					variables: { file: null },
					operationName: "Upload",
				},
			},
		);

		expect(response.statusCode).toBe(200);
		const responseBody = response.json();

		const data = responseBody.data;
		expect(data).toHaveProperty("singleUpload");
		expect(data.singleUpload).toHaveProperty("id");
		expect(data.singleUpload.id).toBe("file-001");
		expect(data.singleUpload).toHaveProperty("filename");
		expect(data.singleUpload.filename).toBe("test.txt");
		expect(data.singleUpload).toHaveProperty("mimetype");
		expect(data.singleUpload.mimetype).toBe("text/plain");
		expect(data.singleUpload).toHaveProperty("size");
		expect(data.singleUpload.size).toBe(39);
		const errors = responseBody.errors;
		expect(errors?.length ?? 0).toBe(0);
	});

	test("file_upload_with_variables", async () => {
		const app = createAppGraphqlMutation();
		const client = new TestClient(app);

		const response = await client.post(
			"/graphql",
			{
				headers: { "x-spikard-fixture": "file_upload_with_variables" },
				json: {
					query: `mutation UploadProfile($userId: ID!, $file: Upload!) {\n  uploadProfilePicture(userId: $userId, file: $file) {\n    id\n    name\n    email\n    profilePicture {\n      id\n      filename\n      mimetype\n      size\n    }\n  }\n}`,
					variables: { userId: "user-123", file: null },
					operationName: "UploadProfile",
				},
			},
		);

		expect(response.statusCode).toBe(200);
		const responseBody = response.json();

		const data = responseBody.data;
		expect(data).toHaveProperty("uploadProfilePicture");
		expect(data.uploadProfilePicture).toHaveProperty("id");
		expect(data.uploadProfilePicture.id).toBe("user-123");
		expect(data.uploadProfilePicture).toHaveProperty("name");
		expect(data.uploadProfilePicture.name).toBe("John Doe");
		expect(data.uploadProfilePicture).toHaveProperty("email");
		expect(data.uploadProfilePicture.email).toBe("john@example.com");
		expect(data.uploadProfilePicture).toHaveProperty("profilePicture");
		expect(data.uploadProfilePicture.profilePicture).toHaveProperty("id");
		expect(data.uploadProfilePicture.profilePicture.id).toBe("file-005");
		expect(data.uploadProfilePicture.profilePicture).toHaveProperty("filename");
		expect(data.uploadProfilePicture.profilePicture.filename).toBe("profile.jpg");
		expect(data.uploadProfilePicture.profilePicture).toHaveProperty("mimetype");
		expect(data.uploadProfilePicture.profilePicture.mimetype).toBe("image/jpeg");
		expect(data.uploadProfilePicture.profilePicture).toHaveProperty("size");
		expect(data.uploadProfilePicture.profilePicture.size).toBe(24568);
		const errors = responseBody.errors;
		expect(errors?.length ?? 0).toBe(0);
	});

});
