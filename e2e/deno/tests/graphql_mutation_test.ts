import { assertEquals, assert } from "jsr:@std/assert@1";
import { TestClient } from "@spikard/wasm";
import { assertEquals } from "jsr:@std/assert@1";
import { createAppGraphqlMutation } from "../app/main.ts";

	Deno.test("GraphQL mutation: validation_directive", async () => {
		const app = createAppGraphqlMutation();
		const client = new TestClient(app);

		const response = await client.post(
			"/graphql",
			{
				json: {
					query: `mutation CreateUser($input: CreateUserInput!) {\n  createUser(input: $input) {\n    id\n    name\n    bio\n  }\n}`,
					variables: { input: { bio: null, name: "a" } },
					operationName: null,
				},
			},
		);

		assertEquals(response.statusCode, 422);
		const responseBody = response.json();

		const errors = responseBody.errors;
		assert(errors !== undefined);
		assertEquals(errors?.length, 1);
		assert(errors?.[0]?.message.includes("Validation error on input field 'name': String length must be between 3 and 50 characters (provided: 1)"));
	});

	Deno.test("GraphQL mutation: custom_scalar_invalid", async () => {
		const app = createAppGraphqlMutation();
		const client = new TestClient(app);

		const response = await client.post(
			"/graphql",
			{
				json: {
					query: `mutation CreateContact($input: CreateContactInput!) {\n  createContact(input: $input) {\n    id\n    name\n    email\n    website\n    phone\n    createdAt\n  }\n}`,
					variables: { input: { email: "not-an-email", name: "Invalid Contact", phone: "123", website: "not a url" } },
					operationName: "CreateContact",
				},
			},
		);

		assertEquals(response.statusCode, 422);
		const responseBody = response.json();

		const errors = responseBody.errors;
		assert(errors !== undefined);
		assertEquals(errors?.length, 3);
		assert(errors?.[0]?.message.includes("Email must be a valid email address"));
		assert(errors?.[1]?.message.includes("URL must start with http:// or https://"));
		assert(errors?.[2]?.message.includes("PhoneNumber must be a valid E.164 format"));
	});

	Deno.test("GraphQL mutation: custom_scalar_validation", async () => {
		const app = createAppGraphqlMutation();
		const client = new TestClient(app);

		const response = await client.post(
			"/graphql",
			{
				json: {
					query: `mutation CreateContact($input: CreateContactInput!) {\n  createContact(input: $input) {\n    id\n    name\n    email\n    website\n    phone\n    createdAt\n  }\n}`,
					variables: { input: { email: "alice.johnson@example.com", name: "Alice Johnson", phone: "+1-555-123-4567", website: "https://example.com" } },
					operationName: "CreateContact",
				},
			},
		);

		assertEquals(response.statusCode, 200);
		const responseBody = response.json();

		const data = responseBody.data;
		assert(data.hasOwnProperty("createContact"));
		assert(data.createContact.hasOwnProperty("createdAt"));
		assertEquals(data.createContact.createdAt, "2025-12-27T14:30:00Z");
		assert(data.createContact.hasOwnProperty("email"));
		assertEquals(data.createContact.email, "alice.johnson@example.com");
		assert(data.createContact.hasOwnProperty("id"));
		assertEquals(data.createContact.id, "contact-001");
		assert(data.createContact.hasOwnProperty("name"));
		assertEquals(data.createContact.name, "Alice Johnson");
		assert(data.createContact.hasOwnProperty("phone"));
		assertEquals(data.createContact.phone, "+1-555-123-4567");
		assert(data.createContact.hasOwnProperty("website"));
		assertEquals(data.createContact.website, "https://example.com");
		const errors = responseBody.errors;
		assertEquals(errors?.length ?? 0, 0);
	});

	Deno.test("GraphQL mutation: create_resource", async () => {
		const app = createAppGraphqlMutation();
		const client = new TestClient(app);

		const response = await client.post(
			"/graphql",
			{
				json: {
					query: `mutation CreateUser($input: CreateUserInput!) {\n  createUser(input: $input) {\n    id\n    name\n    email\n    role\n    createdAt\n  }\n}`,
					variables: { input: { email: "john@example.com", name: "John Doe", role: "admin" } },
					operationName: "CreateUser",
				},
			},
		);

		assertEquals(response.statusCode, 200);
		const responseBody = response.json();

		const data = responseBody.data;
		assert(data.hasOwnProperty("createUser"));
		assert(data.createUser.hasOwnProperty("createdAt"));
		assertEquals(data.createUser.createdAt, "2025-12-27T10:30:00Z");
		assert(data.createUser.hasOwnProperty("email"));
		assertEquals(data.createUser.email, "john@example.com");
		assert(data.createUser.hasOwnProperty("id"));
		assertEquals(data.createUser.id, "user-123");
		assert(data.createUser.hasOwnProperty("name"));
		assertEquals(data.createUser.name, "John Doe");
		assert(data.createUser.hasOwnProperty("role"));
		assertEquals(data.createUser.role, "admin");
		const errors = responseBody.errors;
		assertEquals(errors?.length ?? 0, 0);
	});

	Deno.test("GraphQL mutation: delete_resource", async () => {
		const app = createAppGraphqlMutation();
		const client = new TestClient(app);

		const response = await client.post(
			"/graphql",
			{
				json: {
					query: `mutation DeleteUser($id: ID!) {\n  deleteUser(id: $id) {\n    success\n    message\n    deletedId\n  }\n}`,
					variables: { id: "user-123" },
					operationName: "DeleteUser",
				},
			},
		);

		assertEquals(response.statusCode, 200);
		const responseBody = response.json();

		const data = responseBody.data;
		assert(data.hasOwnProperty("deleteUser"));
		assert(data.deleteUser.hasOwnProperty("deletedId"));
		assertEquals(data.deleteUser.deletedId, "user-123");
		assert(data.deleteUser.hasOwnProperty("message"));
		assertEquals(data.deleteUser.message, "User successfully deleted");
		assert(data.deleteUser.hasOwnProperty("success"));
		assertEquals(data.deleteUser.success, true);
		const errors = responseBody.errors;
		assertEquals(errors?.length ?? 0, 0);
	});

	Deno.test("GraphQL mutation: update_resource", async () => {
		const app = createAppGraphqlMutation();
		const client = new TestClient(app);

		const response = await client.post(
			"/graphql",
			{
				json: {
					query: `mutation UpdateUser($id: ID!, $input: UpdateUserInput!) {\n  updateUser(id: $id, input: $input) {\n    id\n    name\n    email\n    role\n    updatedAt\n  }\n}`,
					variables: { id: "user-123", input: { email: "john.doe@example.com", role: "editor" } },
					operationName: "UpdateUser",
				},
			},
		);

		assertEquals(response.statusCode, 200);
		const responseBody = response.json();

		const data = responseBody.data;
		assert(data.hasOwnProperty("updateUser"));
		assert(data.updateUser.hasOwnProperty("email"));
		assertEquals(data.updateUser.email, "john.doe@example.com");
		assert(data.updateUser.hasOwnProperty("id"));
		assertEquals(data.updateUser.id, "user-123");
		assert(data.updateUser.hasOwnProperty("name"));
		assertEquals(data.updateUser.name, "John Doe");
		assert(data.updateUser.hasOwnProperty("role"));
		assertEquals(data.updateUser.role, "editor");
		assert(data.updateUser.hasOwnProperty("updatedAt"));
		assertEquals(data.updateUser.updatedAt, "2025-12-27T11:45:00Z");
		const errors = responseBody.errors;
		assertEquals(errors?.length ?? 0, 0);
	});

	Deno.test("GraphQL mutation: validation_error", async () => {
		const app = createAppGraphqlMutation();
		const client = new TestClient(app);

		const response = await client.post(
			"/graphql",
			{
				json: {
					query: `mutation CreatePost($input: CreatePostInput!) {\n  createPost(input: $input) {\n    id\n    title\n    content\n    tags\n    createdAt\n  }\n}`,
					variables: { input: { content: "This is a post", title: "My Post" } },
					operationName: null,
				},
			},
		);

		assertEquals(response.statusCode, 400);
		const responseBody = response.json();

		const errors = responseBody.errors;
		assert(errors !== undefined);
		assertEquals(errors?.length, 1);
		assert(errors?.[0]?.message.includes("Field \""CreatePostInput.tags\" of required type \["[String!]!\" was not provided."))	});

	Deno.test("GraphQL mutation: mutation_permission_check", async () => {
		const app = createAppGraphqlMutation();
		const client = new TestClient(app);

		const response = await client.post(
			"/graphql",
			{
				json: {
					query: `mutation DeleteUser($userId: String!) {\n  deleteUser(id: $userId) {\n    success\n    message\n  }\n}`,
					variables: { userId: "user123" },
					operationName: null,
				},
			},
		);

		assertEquals(response.statusCode, 403);
		const responseBody = response.json();

		const errors = responseBody.errors;
		assert(errors !== undefined);
		assertEquals(errors?.length, 1);
		assert(errors?.[0]?.message.includes("Missing required permission: DELETE"));
	});

	Deno.test("GraphQL mutation: dynamic_authorization", async () => {
		const app = createAppGraphqlMutation();
		const client = new TestClient(app);

		const response = await client.post(
			"/graphql",
			{
				json: {
					query: `mutation ApprovePost($postId: String!) {\n  approvePost(id: $postId) {\n    success\n    postId\n    status\n  }\n}`,
					variables: { postId: "post123" },
					operationName: null,
				},
			},
		);

		assertEquals(response.statusCode, 403);
		const responseBody = response.json();

		const errors = responseBody.errors;
		assert(errors !== undefined);
		assertEquals(errors?.length, 1);
		assert(errors?.[0]?.message.includes("Only post author or admin can approve posts"));
	});

	Deno.test("GraphQL mutation: required_fields", async () => {
		const app = createAppGraphqlMutation();
		const client = new TestClient(app);

		const response = await client.post(
			"/graphql",
			{
				json: {
					query: `mutation Register($input: UserRegistrationInput!) {\n  registerUser(input: $input) {\n    success\n    userId\n    message\n  }\n}`,
					variables: { input: { email: "john@example.com", username: "johndoe" } },
					operationName: null,
				},
			},
		);

		assertEquals(response.statusCode, 400);
		const responseBody = response.json();

		const errors = responseBody.errors;
		assert(errors !== undefined);
		assertEquals(errors?.length, 1);
		assert(errors?.[0]?.message.includes("Field \""UserRegistrationInput.password\" of required type \S"String!\" was not provided."))	});

	Deno.test("GraphQL mutation: file_upload_validation_type", async () => {
		const app = createAppGraphqlMutation();
		const client = new TestClient(app);

		const response = await client.post(
			"/graphql",
			{
				json: {
					query: `mutation UploadImage($file: Upload!) {\n  uploadImage(file: $file) {\n    id\n    filename\n    mimetype\n    width\n    height\n  }\n}`,
					variables: { file: null },
					operationName: "UploadImage",
				},
			},
		);

		assertEquals(response.statusCode, 400);
		const responseBody = response.json();

		const errors = responseBody.errors;
		assert(errors !== undefined);
		assertEquals(errors?.length, 1);
		assert(errors?.[0]?.message.includes("Invalid file type"));
	});

	Deno.test("GraphQL mutation: multiple_files_upload", async () => {
		const app = createAppGraphqlMutation();
		const client = new TestClient(app);

		const response = await client.post(
			"/graphql",
			{
				json: {
					query: `mutation MultipleUpload($files: [Upload!]!) {\n  multipleUpload(files: $files) {\n    id\n    filename\n    mimetype\n    size\n  }\n}`,
					variables: { files: [null, null, null] },
					operationName: "MultipleUpload",
				},
			},
		);

		assertEquals(response.statusCode, 200);
		const responseBody = response.json();

		const data = responseBody.data;
		assert(data.hasOwnProperty("multipleUpload"));
		assertEquals(data.multipleUpload.length, 3);
		assert(data.multipleUpload[0].hasOwnProperty("filename"));
		assertEquals(data.multipleUpload[0].filename, "document.pdf");
		assert(data.multipleUpload[0].hasOwnProperty("id"));
		assertEquals(data.multipleUpload[0].id, "file-002");
		assert(data.multipleUpload[0].hasOwnProperty("mimetype"));
		assertEquals(data.multipleUpload[0].mimetype, "application/pdf");
		assert(data.multipleUpload[0].hasOwnProperty("size"));
		assertEquals(data.multipleUpload[0].size, 32);
		assert(data.multipleUpload[1].hasOwnProperty("filename"));
		assertEquals(data.multipleUpload[1].filename, "image.png");
		assert(data.multipleUpload[1].hasOwnProperty("id"));
		assertEquals(data.multipleUpload[1].id, "file-003");
		assert(data.multipleUpload[1].hasOwnProperty("mimetype"));
		assertEquals(data.multipleUpload[1].mimetype, "image/png");
		assert(data.multipleUpload[1].hasOwnProperty("size"));
		assertEquals(data.multipleUpload[1].size, 24);
		assert(data.multipleUpload[2].hasOwnProperty("filename"));
		assertEquals(data.multipleUpload[2].filename, "data.csv");
		assert(data.multipleUpload[2].hasOwnProperty("id"));
		assertEquals(data.multipleUpload[2].id, "file-004");
		assert(data.multipleUpload[2].hasOwnProperty("mimetype"));
		assertEquals(data.multipleUpload[2].mimetype, "text/csv");
		assert(data.multipleUpload[2].hasOwnProperty("size"));
		assertEquals(data.multipleUpload[2].size, 68);
		const errors = responseBody.errors;
		assertEquals(errors?.length ?? 0, 0);
	});

	Deno.test("GraphQL mutation: file_upload_multipart_spec", async () => {
		const app = createAppGraphqlMutation();
		const client = new TestClient(app);

		const response = await client.post(
			"/graphql",
			{
				json: {
					query: `mutation UploadDocument($title: String!, $files: [Upload!]!) {\n  uploadDocument(title: $title, files: $files) {\n    id\n    title\n    files {\n      id\n      filename\n      mimetype\n      size\n    }\n    uploadedAt\n  }\n}`,
					variables: { files: [null, null], title: "Important Documents" },
					operationName: "UploadDocument",
				},
			},
		);

		assertEquals(response.statusCode, 200);
		const responseBody = response.json();

		const data = responseBody.data;
		assert(data.hasOwnProperty("uploadDocument"));
		assert(data.uploadDocument.hasOwnProperty("files"));
		assertEquals(data.uploadDocument.files.length, 2);
		assert(data.uploadDocument.files[0].hasOwnProperty("filename"));
		assertEquals(data.uploadDocument.files[0].filename, "contract.pdf");
		assert(data.uploadDocument.files[0].hasOwnProperty("id"));
		assertEquals(data.uploadDocument.files[0].id, "file-006");
		assert(data.uploadDocument.files[0].hasOwnProperty("mimetype"));
		assertEquals(data.uploadDocument.files[0].mimetype, "application/pdf");
		assert(data.uploadDocument.files[0].hasOwnProperty("size"));
		assertEquals(data.uploadDocument.files[0].size, 88);
		assert(data.uploadDocument.files[1].hasOwnProperty("filename"));
		assertEquals(data.uploadDocument.files[1].filename, "summary.txt");
		assert(data.uploadDocument.files[1].hasOwnProperty("id"));
		assertEquals(data.uploadDocument.files[1].id, "file-007");
		assert(data.uploadDocument.files[1].hasOwnProperty("mimetype"));
		assertEquals(data.uploadDocument.files[1].mimetype, "text/plain");
		assert(data.uploadDocument.files[1].hasOwnProperty("size"));
		assertEquals(data.uploadDocument.files[1].size, 65);
		assert(data.uploadDocument.hasOwnProperty("id"));
		assertEquals(data.uploadDocument.id, "doc-001");
		assert(data.uploadDocument.hasOwnProperty("title"));
		assertEquals(data.uploadDocument.title, "Important Documents");
		assert(data.uploadDocument.hasOwnProperty("uploadedAt"));
		assertEquals(data.uploadDocument.uploadedAt, "2025-12-27T14:30:00Z");
		const errors = responseBody.errors;
		assertEquals(errors?.length ?? 0, 0);
	});

	Deno.test("GraphQL mutation: file_upload_validation_size", async () => {
		const app = createAppGraphqlMutation();
		const client = new TestClient(app);

		const response = await client.post(
			"/graphql",
			{
				json: {
					query: `mutation Upload($file: Upload!) {\n  singleUpload(file: $file) {\n    id\n    filename\n    mimetype\n    size\n  }\n}`,
					variables: { file: null },
					operationName: "Upload",
				},
			},
		);

		assertEquals(response.statusCode, 400);
		const responseBody = response.json();

		const errors = responseBody.errors;
		assert(errors !== undefined);
		assertEquals(errors?.length, 1);
		assert(errors?.[0]?.message.includes("File too large"));
	});

	Deno.test("GraphQL mutation: single_file_upload", async () => {
		const app = createAppGraphqlMutation();
		const client = new TestClient(app);

		const response = await client.post(
			"/graphql",
			{
				json: {
					query: `mutation Upload($file: Upload!) {\n  singleUpload(file: $file) {\n    id\n    filename\n    mimetype\n    size\n  }\n}`,
					variables: { file: null },
					operationName: "Upload",
				},
			},
		);

		assertEquals(response.statusCode, 200);
		const responseBody = response.json();

		const data = responseBody.data;
		assert(data.hasOwnProperty("singleUpload"));
		assert(data.singleUpload.hasOwnProperty("filename"));
		assertEquals(data.singleUpload.filename, "test.txt");
		assert(data.singleUpload.hasOwnProperty("id"));
		assertEquals(data.singleUpload.id, "file-001");
		assert(data.singleUpload.hasOwnProperty("mimetype"));
		assertEquals(data.singleUpload.mimetype, "text/plain");
		assert(data.singleUpload.hasOwnProperty("size"));
		assertEquals(data.singleUpload.size, 39);
		const errors = responseBody.errors;
		assertEquals(errors?.length ?? 0, 0);
	});

	Deno.test("GraphQL mutation: file_upload_with_variables", async () => {
		const app = createAppGraphqlMutation();
		const client = new TestClient(app);

		const response = await client.post(
			"/graphql",
			{
				json: {
					query: `mutation UploadProfile($userId: ID!, $file: Upload!) {\n  uploadProfilePicture(userId: $userId, file: $file) {\n    id\n    name\n    email\n    profilePicture {\n      id\n      filename\n      mimetype\n      size\n    }\n  }\n}`,
					variables: { file: null, userId: "user-123" },
					operationName: "UploadProfile",
				},
			},
		);

		assertEquals(response.statusCode, 200);
		const responseBody = response.json();

		const data = responseBody.data;
		assert(data.hasOwnProperty("uploadProfilePicture"));
		assert(data.uploadProfilePicture.hasOwnProperty("email"));
		assertEquals(data.uploadProfilePicture.email, "john@example.com");
		assert(data.uploadProfilePicture.hasOwnProperty("id"));
		assertEquals(data.uploadProfilePicture.id, "user-123");
		assert(data.uploadProfilePicture.hasOwnProperty("name"));
		assertEquals(data.uploadProfilePicture.name, "John Doe");
		assert(data.uploadProfilePicture.hasOwnProperty("profilePicture"));
		assert(data.uploadProfilePicture.profilePicture.hasOwnProperty("filename"));
		assertEquals(data.uploadProfilePicture.profilePicture.filename, "profile.jpg");
		assert(data.uploadProfilePicture.profilePicture.hasOwnProperty("id"));
		assertEquals(data.uploadProfilePicture.profilePicture.id, "file-005");
		assert(data.uploadProfilePicture.profilePicture.hasOwnProperty("mimetype"));
		assertEquals(data.uploadProfilePicture.profilePicture.mimetype, "image/jpeg");
		assert(data.uploadProfilePicture.profilePicture.hasOwnProperty("size"));
		assertEquals(data.uploadProfilePicture.profilePicture.size, 24568);
		const errors = responseBody.errors;
		assertEquals(errors?.length ?? 0, 0);
	});