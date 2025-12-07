/**
 * Validation Example
 *
 * Demonstrates JSON request body validation, query parameter handling,
 * and structured error responses.
 */

import { get, post, type Request, Spikard } from "@spikard/node";

const app = new Spikard({
	port: 8000,
});

// Simple in-memory user store
const users: Record<number, { id: number; name: string; email: string }> = {
	1: { id: 1, name: "Alice", email: "alice@example.com" },
	2: { id: 2, name: "Bob", email: "bob@example.com" },
};

/**
 * GET endpoint returning a list of users
 */
const listUsers = get("/users")(async function listUsers(req: Request) {
	// Support optional filtering by name via query params
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
const getUser = get("/users/:id")(async function getUser(req: Request) {
	const userId = parseInt(req.params?.id as string, 10);

	if (!Number.isInteger(userId)) {
		return {
			status: 400,
			body: {
				error: "Invalid user ID",
				code: "invalid_id",
				details: { received: req.params?.id },
			},
		};
	}

	const user = users[userId];
	if (!user) {
		return {
			status: 404,
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
const createUser = post("/users")(async function createUser(req: Request) {
	const body = req.body as {
		name?: string;
		email?: string;
	} | null;

	// Validate required fields
	if (!body?.name || typeof body.name !== "string") {
		return {
			status: 400,
			body: {
				error: "Missing or invalid required field: name",
				code: "validation_error",
				details: { field: "name", received_type: typeof body?.name },
			},
		};
	}

	if (!body.email || typeof body.email !== "string") {
		return {
			status: 400,
			body: {
				error: "Missing or invalid required field: email",
				code: "validation_error",
				details: { field: "email", received_type: typeof body?.email },
			},
		};
	}

	// Validate email format (simple check)
	if (!body.email.includes("@")) {
		return {
			status: 400,
			body: {
				error: "Invalid email format",
				code: "validation_error",
				details: { field: "email", received: body.email },
			},
		};
	}

	// Create user
	const newId = Math.max(...Object.keys(users).map(Number)) + 1;
	const newUser = {
		id: newId,
		name: body.name,
		email: body.email,
	};
	users[newId] = newUser;

	return {
		status: 201,
		body: newUser,
	};
});

// Register handlers
app.registerHandler(listUsers);
app.registerHandler(getUser);
app.registerHandler(createUser);

console.log("Starting Validation Example on http://127.0.0.1:8000");
console.log("Try:");
console.log("  curl http://127.0.0.1:8000/users");
console.log("  curl 'http://127.0.0.1:8000/users?name=Alice'");
console.log("  curl http://127.0.0.1:8000/users/1");
console.log(
	'  curl -X POST http://127.0.0.1:8000/users -H \'Content-Type: application/json\' -d \'{"name":"Charlie","email":"charlie@example.com"}\'',
);
console.log("");

app.listen().catch((error) => {
	console.error("Server error:", error);
	process.exit(1);
});
