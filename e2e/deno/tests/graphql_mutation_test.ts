import { assertEquals, assert } from "jsr:@std/assert@1";
import { TestClient } from "@spikard/wasm";
import { createAppGraphqlMutation } from "../app/main.ts";

Deno.test("GraphQL mutation: validation_directive", async () => {
	const app = createAppGraphqlMutation();
	const client = new TestClient(app);

	const response = await client.post("/graphql", {
		headers: { "x-spikard-fixture": "validation_directive" },
		json: {
			query: `mutation CreateUser($input: CreateUserInput!) {\n  createUser(input: $input) {\n    id\n    name\n    bio\n  }\n}`,
			variables: { input: { bio: null, name: "a" } },
			operationName: null,
		},
	});

	assertEquals(response.statusCode, 422);
	const responseBody = response.json();

	const errors = responseBody.errors;
	assert(errors !== undefined);
	assertEquals(errors?.length, 1);
	assert(
		errors?.[0]?.message.includes(
			"Validation error on input field 'name': String length must be between 3 and 50 characters (provided: 1)",
		),
	);
});

Deno.test("GraphQL mutation: custom_scalar_invalid", async () => {
	const app = createAppGraphqlMutation();
	const client = new TestClient(app);

	const response = await client.post("/graphql", {
		headers: { "x-spikard-fixture": "custom_scalar_invalid" },
		json: {
			query: `mutation CreateContact($input: CreateContactInput!) {\n  createContact(input: $input) {\n    id\n    name\n    email\n    website\n    phone\n    createdAt\n  }\n}`,
			variables: { input: { email: "not-an-email", name: "Invalid Contact", phone: "123", website: "not a url" } },
			operationName: "CreateContact",
		},
	});

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

	const response = await client.post("/graphql", {
		headers: { "x-spikard-fixture": "custom_scalar_validation" },
		json: {
			query: `mutation CreateContact($input: CreateContactInput!) {\n  createContact(input: $input) {\n    id\n    name\n    email\n    website\n    phone\n    createdAt\n  }\n}`,
			variables: {
				input: {
					email: "alice.johnson@example.com",
					name: "Alice Johnson",
					phone: "+1-555-123-4567",
					website: "https://example.com",
				},
			},
			operationName: "CreateContact",
		},
	});

	assertEquals(response.statusCode, 200);
	const responseBody = response.json();

	const data = responseBody.data;
	assert(Object.hasOwn(data, "createContact"));
	assert(Object.hasOwn(data.createContact, "createdAt"));
	assertEquals(data.createContact.createdAt, "2025-12-27T14:30:00Z");
	assert(Object.hasOwn(data.createContact, "email"));
	assertEquals(data.createContact.email, "alice.johnson@example.com");
	assert(Object.hasOwn(data.createContact, "id"));
	assertEquals(data.createContact.id, "contact-001");
	assert(Object.hasOwn(data.createContact, "name"));
	assertEquals(data.createContact.name, "Alice Johnson");
	assert(Object.hasOwn(data.createContact, "phone"));
	assertEquals(data.createContact.phone, "+1-555-123-4567");
	assert(Object.hasOwn(data.createContact, "website"));
	assertEquals(data.createContact.website, "https://example.com");
	const errors = responseBody.errors;
	assertEquals(errors?.length ?? 0, 0);
});

Deno.test("GraphQL mutation: create_resource", async () => {
	const app = createAppGraphqlMutation();
	const client = new TestClient(app);

	const response = await client.post("/graphql", {
		headers: { "x-spikard-fixture": "create_resource" },
		json: {
			query: `mutation CreateUser($input: CreateUserInput!) {\n  createUser(input: $input) {\n    id\n    name\n    email\n    role\n    createdAt\n  }\n}`,
			variables: { input: { email: "john@example.com", name: "John Doe", role: "admin" } },
			operationName: "CreateUser",
		},
	});

	assertEquals(response.statusCode, 200);
	const responseBody = response.json();

	const data = responseBody.data;
	assert(Object.hasOwn(data, "createUser"));
	assert(Object.hasOwn(data.createUser, "createdAt"));
	assertEquals(data.createUser.createdAt, "2025-12-27T10:30:00Z");
	assert(Object.hasOwn(data.createUser, "email"));
	assertEquals(data.createUser.email, "john@example.com");
	assert(Object.hasOwn(data.createUser, "id"));
	assertEquals(data.createUser.id, "user-123");
	assert(Object.hasOwn(data.createUser, "name"));
	assertEquals(data.createUser.name, "John Doe");
	assert(Object.hasOwn(data.createUser, "role"));
	assertEquals(data.createUser.role, "admin");
	const errors = responseBody.errors;
	assertEquals(errors?.length ?? 0, 0);
});

Deno.test("GraphQL mutation: delete_resource", async () => {
	const app = createAppGraphqlMutation();
	const client = new TestClient(app);

	const response = await client.post("/graphql", {
		headers: { "x-spikard-fixture": "delete_resource" },
		json: {
			query: `mutation DeleteUser($id: ID!) {\n  deleteUser(id: $id) {\n    success\n    message\n    deletedId\n  }\n}`,
			variables: { id: "user-123" },
			operationName: "DeleteUser",
		},
	});

	assertEquals(response.statusCode, 200);
	const responseBody = response.json();

	const data = responseBody.data;
	assert(Object.hasOwn(data, "deleteUser"));
	assert(Object.hasOwn(data.deleteUser, "deletedId"));
	assertEquals(data.deleteUser.deletedId, "user-123");
	assert(Object.hasOwn(data.deleteUser, "message"));
	assertEquals(data.deleteUser.message, "User successfully deleted");
	assert(Object.hasOwn(data.deleteUser, "success"));
	assertEquals(data.deleteUser.success, true);
	const errors = responseBody.errors;
	assertEquals(errors?.length ?? 0, 0);
});

Deno.test("GraphQL mutation: update_resource", async () => {
	const app = createAppGraphqlMutation();
	const client = new TestClient(app);

	const response = await client.post("/graphql", {
		headers: { "x-spikard-fixture": "update_resource" },
		json: {
			query: `mutation UpdateUser($id: ID!, $input: UpdateUserInput!) {\n  updateUser(id: $id, input: $input) {\n    id\n    name\n    email\n    role\n    updatedAt\n  }\n}`,
			variables: { id: "user-123", input: { email: "john.doe@example.com", role: "editor" } },
			operationName: "UpdateUser",
		},
	});

	assertEquals(response.statusCode, 200);
	const responseBody = response.json();

	const data = responseBody.data;
	assert(Object.hasOwn(data, "updateUser"));
	assert(Object.hasOwn(data.updateUser, "email"));
	assertEquals(data.updateUser.email, "john.doe@example.com");
	assert(Object.hasOwn(data.updateUser, "id"));
	assertEquals(data.updateUser.id, "user-123");
	assert(Object.hasOwn(data.updateUser, "name"));
	assertEquals(data.updateUser.name, "John Doe");
	assert(Object.hasOwn(data.updateUser, "role"));
	assertEquals(data.updateUser.role, "editor");
	assert(Object.hasOwn(data.updateUser, "updatedAt"));
	assertEquals(data.updateUser.updatedAt, "2025-12-27T11:45:00Z");
	const errors = responseBody.errors;
	assertEquals(errors?.length ?? 0, 0);
});

Deno.test("GraphQL mutation: validation_error", async () => {
	const app = createAppGraphqlMutation();
	const client = new TestClient(app);

	const response = await client.post("/graphql", {
		headers: { "x-spikard-fixture": "validation_error" },
		json: {
			query: `mutation CreatePost($input: CreatePostInput!) {\n  createPost(input: $input) {\n    id\n    title\n    content\n    tags\n    createdAt\n  }\n}`,
			variables: { input: { content: "This is a post", title: "My Post" } },
			operationName: null,
		},
	});

	assertEquals(response.statusCode, 400);
	const responseBody = response.json();

	const errors = responseBody.errors;
	assert(errors !== undefined);
	assertEquals(errors?.length, 1);
	assert(errors?.[0]?.message.includes('Field "CreatePostInput.tags" of required type "[String!]!" was not provided.'));
});

Deno.test("GraphQL mutation: mutation_permission_check", async () => {
	const app = createAppGraphqlMutation();
	const client = new TestClient(app);

	const response = await client.post("/graphql", {
		headers: { "x-spikard-fixture": "mutation_permission_check" },
		json: {
			query: `mutation DeleteUser($userId: String!) {\n  deleteUser(id: $userId) {\n    success\n    message\n  }\n}`,
			variables: { userId: "user123" },
			operationName: null,
		},
	});

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

	const response = await client.post("/graphql", {
		headers: { "x-spikard-fixture": "dynamic_authorization" },
		json: {
			query: `mutation ApprovePost($postId: String!) {\n  approvePost(id: $postId) {\n    success\n    postId\n    status\n  }\n}`,
			variables: { postId: "post123" },
			operationName: null,
		},
	});

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

	const response = await client.post("/graphql", {
		headers: { "x-spikard-fixture": "required_fields" },
		json: {
			query: `mutation Register($input: UserRegistrationInput!) {\n  registerUser(input: $input) {\n    success\n    userId\n    message\n  }\n}`,
			variables: { input: { email: "john@example.com", username: "johndoe" } },
			operationName: null,
		},
	});

	assertEquals(response.statusCode, 400);
	const responseBody = response.json();

	const errors = responseBody.errors;
	assert(errors !== undefined);
	assertEquals(errors?.length, 1);
	assert(
		errors?.[0]?.message.includes(
			'Field "UserRegistrationInput.password" of required type "String!" was not provided.',
		),
	);
});

Deno.test("GraphQL mutation: file_upload_validation_type", async () => {
	const app = createAppGraphqlMutation();
	const client = new TestClient(app);

	const response = await client.post("/graphql", {
		headers: { "x-spikard-fixture": "file_upload_validation_type" },
		json: {
			query: `mutation UploadImage($file: Upload!) {\n  uploadImage(file: $file) {\n    id\n    filename\n    mimetype\n    width\n    height\n  }\n}`,
			variables: { file: null },
			operationName: "UploadImage",
		},
	});

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

	const response = await client.post("/graphql", {
		headers: { "x-spikard-fixture": "multiple_files_upload" },
		json: {
			query: `mutation MultipleUpload($files: [Upload!]!) {\n  multipleUpload(files: $files) {\n    id\n    filename\n    mimetype\n    size\n  }\n}`,
			variables: { files: [null, null, null] },
			operationName: "MultipleUpload",
		},
	});

	assertEquals(response.statusCode, 200);
	const responseBody = response.json();

	const data = responseBody.data;
	assert(Object.hasOwn(data, "multipleUpload"));
	assertEquals(data.multipleUpload.length, 3);
	assert(Object.hasOwn(data.multipleUpload[0], "filename"));
	assertEquals(data.multipleUpload[0].filename, "document.pdf");
	assert(Object.hasOwn(data.multipleUpload[0], "id"));
	assertEquals(data.multipleUpload[0].id, "file-002");
	assert(Object.hasOwn(data.multipleUpload[0], "mimetype"));
	assertEquals(data.multipleUpload[0].mimetype, "application/pdf");
	assert(Object.hasOwn(data.multipleUpload[0], "size"));
	assertEquals(data.multipleUpload[0].size, 32);
	assert(Object.hasOwn(data.multipleUpload[1], "filename"));
	assertEquals(data.multipleUpload[1].filename, "image.png");
	assert(Object.hasOwn(data.multipleUpload[1], "id"));
	assertEquals(data.multipleUpload[1].id, "file-003");
	assert(Object.hasOwn(data.multipleUpload[1], "mimetype"));
	assertEquals(data.multipleUpload[1].mimetype, "image/png");
	assert(Object.hasOwn(data.multipleUpload[1], "size"));
	assertEquals(data.multipleUpload[1].size, 24);
	assert(Object.hasOwn(data.multipleUpload[2], "filename"));
	assertEquals(data.multipleUpload[2].filename, "data.csv");
	assert(Object.hasOwn(data.multipleUpload[2], "id"));
	assertEquals(data.multipleUpload[2].id, "file-004");
	assert(Object.hasOwn(data.multipleUpload[2], "mimetype"));
	assertEquals(data.multipleUpload[2].mimetype, "text/csv");
	assert(Object.hasOwn(data.multipleUpload[2], "size"));
	assertEquals(data.multipleUpload[2].size, 68);
	const errors = responseBody.errors;
	assertEquals(errors?.length ?? 0, 0);
});

Deno.test("GraphQL mutation: file_upload_multipart_spec", async () => {
	const app = createAppGraphqlMutation();
	const client = new TestClient(app);

	const response = await client.post("/graphql", {
		headers: { "x-spikard-fixture": "file_upload_multipart_spec" },
		json: {
			query: `mutation UploadDocument($title: String!, $files: [Upload!]!) {\n  uploadDocument(title: $title, files: $files) {\n    id\n    title\n    files {\n      id\n      filename\n      mimetype\n      size\n    }\n    uploadedAt\n  }\n}`,
			variables: { files: [null, null], title: "Important Documents" },
			operationName: "UploadDocument",
		},
	});

	assertEquals(response.statusCode, 200);
	const responseBody = response.json();

	const data = responseBody.data;
	assert(Object.hasOwn(data, "uploadDocument"));
	assert(Object.hasOwn(data.uploadDocument, "files"));
	assertEquals(data.uploadDocument.files.length, 2);
	assert(Object.hasOwn(data.uploadDocument.files[0], "filename"));
	assertEquals(data.uploadDocument.files[0].filename, "contract.pdf");
	assert(Object.hasOwn(data.uploadDocument.files[0], "id"));
	assertEquals(data.uploadDocument.files[0].id, "file-006");
	assert(Object.hasOwn(data.uploadDocument.files[0], "mimetype"));
	assertEquals(data.uploadDocument.files[0].mimetype, "application/pdf");
	assert(Object.hasOwn(data.uploadDocument.files[0], "size"));
	assertEquals(data.uploadDocument.files[0].size, 88);
	assert(Object.hasOwn(data.uploadDocument.files[1], "filename"));
	assertEquals(data.uploadDocument.files[1].filename, "summary.txt");
	assert(Object.hasOwn(data.uploadDocument.files[1], "id"));
	assertEquals(data.uploadDocument.files[1].id, "file-007");
	assert(Object.hasOwn(data.uploadDocument.files[1], "mimetype"));
	assertEquals(data.uploadDocument.files[1].mimetype, "text/plain");
	assert(Object.hasOwn(data.uploadDocument.files[1], "size"));
	assertEquals(data.uploadDocument.files[1].size, 65);
	assert(Object.hasOwn(data.uploadDocument, "id"));
	assertEquals(data.uploadDocument.id, "doc-001");
	assert(Object.hasOwn(data.uploadDocument, "title"));
	assertEquals(data.uploadDocument.title, "Important Documents");
	assert(Object.hasOwn(data.uploadDocument, "uploadedAt"));
	assertEquals(data.uploadDocument.uploadedAt, "2025-12-27T14:30:00Z");
	const errors = responseBody.errors;
	assertEquals(errors?.length ?? 0, 0);
});

Deno.test("GraphQL mutation: file_upload_validation_size", async () => {
	const app = createAppGraphqlMutation();
	const client = new TestClient(app);

	const response = await client.post("/graphql", {
		headers: { "x-spikard-fixture": "file_upload_validation_size" },
		json: {
			query: `mutation Upload($file: Upload!) {\n  singleUpload(file: $file) {\n    id\n    filename\n    mimetype\n    size\n  }\n}`,
			variables: { file: null },
			operationName: "Upload",
		},
	});

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

	const response = await client.post("/graphql", {
		headers: { "x-spikard-fixture": "single_file_upload" },
		json: {
			query: `mutation Upload($file: Upload!) {\n  singleUpload(file: $file) {\n    id\n    filename\n    mimetype\n    size\n  }\n}`,
			variables: { file: null },
			operationName: "Upload",
		},
	});

	assertEquals(response.statusCode, 200);
	const responseBody = response.json();

	const data = responseBody.data;
	assert(Object.hasOwn(data, "singleUpload"));
	assert(Object.hasOwn(data.singleUpload, "filename"));
	assertEquals(data.singleUpload.filename, "test.txt");
	assert(Object.hasOwn(data.singleUpload, "id"));
	assertEquals(data.singleUpload.id, "file-001");
	assert(Object.hasOwn(data.singleUpload, "mimetype"));
	assertEquals(data.singleUpload.mimetype, "text/plain");
	assert(Object.hasOwn(data.singleUpload, "size"));
	assertEquals(data.singleUpload.size, 39);
	const errors = responseBody.errors;
	assertEquals(errors?.length ?? 0, 0);
});

Deno.test("GraphQL mutation: file_upload_with_variables", async () => {
	const app = createAppGraphqlMutation();
	const client = new TestClient(app);

	const response = await client.post("/graphql", {
		headers: { "x-spikard-fixture": "file_upload_with_variables" },
		json: {
			query: `mutation UploadProfile($userId: ID!, $file: Upload!) {\n  uploadProfilePicture(userId: $userId, file: $file) {\n    id\n    name\n    email\n    profilePicture {\n      id\n      filename\n      mimetype\n      size\n    }\n  }\n}`,
			variables: { file: null, userId: "user-123" },
			operationName: "UploadProfile",
		},
	});

	assertEquals(response.statusCode, 200);
	const responseBody = response.json();

	const data = responseBody.data;
	assert(Object.hasOwn(data, "uploadProfilePicture"));
	assert(Object.hasOwn(data.uploadProfilePicture, "email"));
	assertEquals(data.uploadProfilePicture.email, "john@example.com");
	assert(Object.hasOwn(data.uploadProfilePicture, "id"));
	assertEquals(data.uploadProfilePicture.id, "user-123");
	assert(Object.hasOwn(data.uploadProfilePicture, "name"));
	assertEquals(data.uploadProfilePicture.name, "John Doe");
	assert(Object.hasOwn(data.uploadProfilePicture, "profilePicture"));
	assert(Object.hasOwn(data.uploadProfilePicture.profilePicture, "filename"));
	assertEquals(data.uploadProfilePicture.profilePicture.filename, "profile.jpg");
	assert(Object.hasOwn(data.uploadProfilePicture.profilePicture, "id"));
	assertEquals(data.uploadProfilePicture.profilePicture.id, "file-005");
	assert(Object.hasOwn(data.uploadProfilePicture.profilePicture, "mimetype"));
	assertEquals(data.uploadProfilePicture.profilePicture.mimetype, "image/jpeg");
	assert(Object.hasOwn(data.uploadProfilePicture.profilePicture, "size"));
	assertEquals(data.uploadProfilePicture.profilePicture.size, 24568);
	const errors = responseBody.errors;
	assertEquals(errors?.length ?? 0, 0);
});
