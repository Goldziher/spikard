/**
 * Validation Example
 *
 * Demonstrates JSON request body validation, query parameter handling,
 * and structured error responses.
 */

import { get, post, type Request, Spikard } from "@spikard/node";

const app = new Spikard();

const users: Record<number, { id: number; name: string; email: string }> = {
	1: { id: 1, name: "Alice", email: "alice@example.com" },
	2: { id: 2, name: "Bob", email: "bob@example.com" },
};

/**
 * GET endpoint returning a list of users
 */
get("/users")(async function listUsers(req: Request) {
	const nameFilter = req.query?.name as string | undefined;

	let userList = Object.values(users);
	if (nameFilter) {
		userList = userList.filter((u) => u.name.toLowerCase().includes(nameFilter.toLowerCase()));
	}

	return {
		users: userList,
		count: userList.length,
	};
});

/**
 * GET endpoint returning a single user by ID
 */
get("/users/:id")(async function getUser(req: Request) {
	const userId = parseInt(req.params?.id as string, 10);

	if (!Number.isInteger(userId)) {
		return {
			statusCode: 400,
			body: {
				error: "Invalid user ID",
				code: "invalid_id",
				details: { received: req.params?.id ?? null },
			},
		};
	}

	const user = users[userId];
	if (!user) {
		return {
			statusCode: 404,
			body: {
				error: "User not found",
				code: "not_found",
				details: { user_id: userId },
			},
		};
	}

	return user;
});

/**
 * POST endpoint to create a new user
 */
post("/users")(async function createUser(req: Request) {
	const body = req.body as {
		name?: string;
		email?: string;
	} | null;

	if (!body?.name || typeof body.name !== "string") {
		return {
			statusCode: 400,
			body: {
				error: "Missing or invalid required field: name",
				code: "validation_error",
				details: { field: "name", received_type: typeof body?.name },
			},
		};
	}

	if (!body.email || typeof body.email !== "string") {
		return {
			statusCode: 400,
			body: {
				error: "Missing or invalid required field: email",
				code: "validation_error",
				details: { field: "email", received_type: typeof body?.email },
			},
		};
	}

	if (!body.email.includes("@")) {
		return {
			statusCode: 400,
			body: {
				error: "Invalid email format",
				code: "validation_error",
				details: { field: "email", received: body.email },
			},
		};
	}

	const newId = Math.max(...Object.keys(users).map(Number)) + 1;
	const newUser = {
		id: newId,
		name: body.name,
		email: body.email,
	};
	users[newId] = newUser;

	return {
		statusCode: 201,
		body: newUser,
	};
});

console.log("Starting Validation Example on http://127.0.0.1:8000");
console.log("Try:");
console.log("  curl http://127.0.0.1:8000/users");
console.log("  curl 'http://127.0.0.1:8000/users?name=Alice'");
console.log("  curl http://127.0.0.1:8000/users/1");
console.log(
	'  curl -X POST http://127.0.0.1:8000/users -H \'Content-Type: application/json\' -d \'{"name":"Charlie","email":"charlie@example.com"}\'',
);
console.log("");

app.run({ port: 8000, host: "0.0.0.0" });
