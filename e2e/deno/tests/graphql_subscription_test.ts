import { assertEquals, assert } from "jsr:@std/assert@1";
import { TestClient } from "@spikard/wasm";
import { createAppGraphqlSubscription } from "../app/main.ts";

Deno.test("GraphQL subscription: subscription_error", async () => {
	const app = createAppGraphqlSubscription();
	const client = new TestClient(app);

	const response = await client.post("/graphql", {
		headers: { "x-spikard-fixture": "subscription_error" },
		json: {
			query: `subscription {\n  invalidSubscription {\n    id\n    data\n  }\n}`,
			variables: null,
			operationName: null,
		},
	});

	assertEquals(response.statusCode, 400);
	const responseBody = response.json();

	const errors = responseBody.errors;
	assertEquals(errors?.length ?? 0, 0);
});

Deno.test("GraphQL subscription: subscription_unsubscribe", async () => {
	const app = createAppGraphqlSubscription();
	const client = new TestClient(app);

	const response = await client.post("/graphql", {
		headers: { "x-spikard-fixture": "subscription_unsubscribe" },
		json: {
			query: `subscription OnTick {\n  ticker {\n    id\n    symbol\n    price\n    timestamp\n  }\n}`,
			variables: null,
			operationName: "OnTick",
		},
	});

	assertEquals(response.statusCode, 200);
	const responseBody = response.json();

	const data = responseBody.data;
	assert(Object.hasOwn(data, "ticker"));
	assert(Object.hasOwn(data.ticker, "id"));
	assertEquals(data.ticker.id, "tick-1");
	assert(Object.hasOwn(data.ticker, "price"));
	assertEquals(data.ticker.price, 195.45);
	assert(Object.hasOwn(data.ticker, "symbol"));
	assertEquals(data.ticker.symbol, "AAPL");
	assert(Object.hasOwn(data.ticker, "timestamp"));
	assertEquals(data.ticker.timestamp, "2025-12-27T15:00:00Z");
	const errors = responseBody.errors;
	assertEquals(errors?.length ?? 0, 0);
});

Deno.test("GraphQL subscription: subscription_connection_params", async () => {
	const app = createAppGraphqlSubscription();
	const client = new TestClient(app);

	const response = await client.post("/graphql", {
		headers: { "x-spikard-fixture": "subscription_connection_params" },
		json: {
			query: `subscription {\n  secureStream {\n    id\n    data\n    timestamp\n  }\n}`,
			variables: null,
			operationName: null,
		},
	});

	assertEquals(response.statusCode, 101);
	const responseBody = response.json();

	const data = responseBody.data;
	assert(Object.hasOwn(data, "secureStream"));
	assert(Object.hasOwn(data.secureStream, "data"));
	assertEquals(data.secureStream.data, "Connection established");
	assert(Object.hasOwn(data.secureStream, "id"));
	assertEquals(data.secureStream.id, "stream-1");
	assert(Object.hasOwn(data.secureStream, "timestamp"));
	assertEquals(data.secureStream.timestamp, "2025-12-27T14:00:00Z");
	const errors = responseBody.errors;
	assertEquals(errors?.length ?? 0, 0);
});

Deno.test("GraphQL subscription: simple_subscription", async () => {
	const app = createAppGraphqlSubscription();
	const client = new TestClient(app);

	const response = await client.post("/graphql", {
		headers: { "x-spikard-fixture": "simple_subscription" },
		json: {
			query: `subscription {\n  messageAdded {\n    id\n    text\n    timestamp\n  }\n}`,
			variables: null,
			operationName: null,
		},
	});

	assertEquals(response.statusCode, 200);
	const responseBody = response.json();

	const data = responseBody.data;
	assert(Object.hasOwn(data, "messageAdded"));
	assert(Object.hasOwn(data.messageAdded, "id"));
	assertEquals(data.messageAdded.id, "msg-1");
	assert(Object.hasOwn(data.messageAdded, "text"));
	assertEquals(data.messageAdded.text, "Hello, WebSocket!");
	assert(Object.hasOwn(data.messageAdded, "timestamp"));
	assertEquals(data.messageAdded.timestamp, "2025-12-27T10:00:00Z");
	const errors = responseBody.errors;
	assertEquals(errors?.length ?? 0, 0);
});

Deno.test("GraphQL subscription: subscription_multiple_fields", async () => {
	const app = createAppGraphqlSubscription();
	const client = new TestClient(app);

	const response = await client.post("/graphql", {
		headers: { "x-spikard-fixture": "subscription_multiple_fields" },
		json: {
			query: `subscription MultiStream {\n  messageAdded {\n    id\n    text\n    author\n  }\n  userOnline {\n    userId\n    username\n    isOnline\n    lastSeen\n  }\n}`,
			variables: null,
			operationName: "MultiStream",
		},
	});

	assertEquals(response.statusCode, 200);
	const responseBody = response.json();

	const data = responseBody.data;
	assert(Object.hasOwn(data, "messageAdded"));
	assert(Object.hasOwn(data.messageAdded, "author"));
	assertEquals(data.messageAdded.author, "alice");
	assert(Object.hasOwn(data.messageAdded, "id"));
	assertEquals(data.messageAdded.id, "msg-101");
	assert(Object.hasOwn(data.messageAdded, "text"));
	assertEquals(data.messageAdded.text, "Hey everyone!");
	assert(Object.hasOwn(data, "userOnline"));
	assert(Object.hasOwn(data.userOnline, "isOnline"));
	assertEquals(data.userOnline.isOnline, true);
	assert(Object.hasOwn(data.userOnline, "lastSeen"));
	assertEquals(data.userOnline.lastSeen, "2025-12-27T13:00:00Z");
	assert(Object.hasOwn(data.userOnline, "userId"));
	assertEquals(data.userOnline.userId, "user-42");
	assert(Object.hasOwn(data.userOnline, "username"));
	assertEquals(data.userOnline.username, "bob");
	const errors = responseBody.errors;
	assertEquals(errors?.length ?? 0, 0);
});

Deno.test("GraphQL subscription: subscription_with_variables", async () => {
	const app = createAppGraphqlSubscription();
	const client = new TestClient(app);

	const response = await client.post("/graphql", {
		headers: { "x-spikard-fixture": "subscription_with_variables" },
		json: {
			query: `subscription OnUserActivity($userId: ID!) {\n  userActivity(userId: $userId) {\n    id\n    userId\n    action\n    description\n    timestamp\n  }\n}`,
			variables: { userId: "user123" },
			operationName: "OnUserActivity",
		},
	});

	assertEquals(response.statusCode, 200);
	const responseBody = response.json();

	const data = responseBody.data;
	assert(Object.hasOwn(data, "userActivity"));
	assert(Object.hasOwn(data.userActivity, "action"));
	assertEquals(data.userActivity.action, "LOGIN");
	assert(Object.hasOwn(data.userActivity, "description"));
	assertEquals(data.userActivity.description, "User logged in from browser");
	assert(Object.hasOwn(data.userActivity, "id"));
	assertEquals(data.userActivity.id, "event-789");
	assert(Object.hasOwn(data.userActivity, "timestamp"));
	assertEquals(data.userActivity.timestamp, "2025-12-27T12:15:00Z");
	assert(Object.hasOwn(data.userActivity, "userId"));
	assertEquals(data.userActivity.userId, "user123");
	const errors = responseBody.errors;
	assertEquals(errors?.length ?? 0, 0);
});

Deno.test("GraphQL subscription: subscription_with_auth_middleware", async () => {
	const app = createAppGraphqlSubscription();
	const client = new TestClient(app);

	const response = await client.post("/graphql", {
		headers: { "x-spikard-fixture": "subscription_with_auth_middleware" },
		json: {
			query: `subscription {\n  privateNotifications {\n    id\n    userId\n    type\n    message\n    priority\n    createdAt\n  }\n}`,
			variables: null,
			operationName: null,
		},
	});

	assertEquals(response.statusCode, 101);
	const responseBody = response.json();

	const data = responseBody.data;
	assert(Object.hasOwn(data, "privateNotifications"));
	assert(Object.hasOwn(data.privateNotifications, "createdAt"));
	assertEquals(data.privateNotifications.createdAt, "2025-12-27T16:20:00Z");
	assert(Object.hasOwn(data.privateNotifications, "id"));
	assertEquals(data.privateNotifications.id, "notif-456");
	assert(Object.hasOwn(data.privateNotifications, "message"));
	assertEquals(data.privateNotifications.message, "Your subscription is about to expire");
	assert(Object.hasOwn(data.privateNotifications, "priority"));
	assertEquals(data.privateNotifications.priority, "HIGH");
	assert(Object.hasOwn(data.privateNotifications, "type"));
	assertEquals(data.privateNotifications.type, "ALERT");
	assert(Object.hasOwn(data.privateNotifications, "userId"));
	assertEquals(data.privateNotifications.userId, "user123");
	const errors = responseBody.errors;
	assertEquals(errors?.length ?? 0, 0);
});

Deno.test("GraphQL subscription: subscription_rate_limited", async () => {
	const app = createAppGraphqlSubscription();
	const client = new TestClient(app);

	const response = await client.post("/graphql", {
		headers: { "x-spikard-fixture": "subscription_rate_limited" },
		json: {
			query: `subscription OnStockUpdate($symbol: String!) {\n  stockTicker(symbol: $symbol) {\n    id\n    symbol\n    price\n    change\n    changePercent\n    timestamp\n    volume\n  }\n}`,
			variables: { symbol: "AAPL" },
			operationName: "OnStockUpdate",
		},
	});

	assertEquals(response.statusCode, 200);
	const responseBody = response.json();

	const data = responseBody.data;
	assert(Object.hasOwn(data, "stockTicker"));
	assert(Object.hasOwn(data.stockTicker, "change"));
	assertEquals(data.stockTicker.change, 2.15);
	assert(Object.hasOwn(data.stockTicker, "changePercent"));
	assertEquals(data.stockTicker.changePercent, 0.91);
	assert(Object.hasOwn(data.stockTicker, "id"));
	assertEquals(data.stockTicker.id, "stock-aapl-1");
	assert(Object.hasOwn(data.stockTicker, "price"));
	assertEquals(data.stockTicker.price, 238.45);
	assert(Object.hasOwn(data.stockTicker, "symbol"));
	assertEquals(data.stockTicker.symbol, "AAPL");
	assert(Object.hasOwn(data.stockTicker, "timestamp"));
	assertEquals(data.stockTicker.timestamp, "2025-12-27T17:00:00Z");
	assert(Object.hasOwn(data.stockTicker, "volume"));
	assertEquals(data.stockTicker.volume, 52345678);
	const errors = responseBody.errors;
	assertEquals(errors?.length ?? 0, 0);
});

Deno.test("GraphQL subscription: subscription_authentication", async () => {
	const app = createAppGraphqlSubscription();
	const client = new TestClient(app);

	const response = await client.post("/graphql", {
		headers: { "x-spikard-fixture": "subscription_authentication" },
		json: {
			query: `subscription {\n  privateMessages {\n    id\n    from\n    content\n    isPrivate\n  }\n}`,
			variables: null,
			operationName: null,
		},
	});

	assertEquals(response.statusCode, 401);
	const responseBody = response.json();

	const errors = responseBody.errors;
	assertEquals(errors?.length ?? 0, 0);
});

Deno.test("GraphQL subscription: subscription_with_filtering", async () => {
	const app = createAppGraphqlSubscription();
	const client = new TestClient(app);

	const response = await client.post("/graphql", {
		headers: { "x-spikard-fixture": "subscription_with_filtering" },
		json: {
			query: `subscription OnPostUpdated($authorId: ID!, $statuses: [PostStatus!]!, $tagFilter: String, $scoreThreshold: Int) {\n  postUpdated(filter: {\n    authorId: $authorId\n    status: $statuses\n    tags_contains: $tagFilter\n    minScore: $scoreThreshold\n  }) {\n    id\n    title\n    authorId\n    content\n    status\n    tags\n    score\n    updatedAt\n  }\n}`,
			variables: { authorId: "123", scoreThreshold: 50, statuses: ["PUBLISHED", "DRAFT"], tagFilter: "graphql" },
			operationName: "OnPostUpdated",
		},
	});

	assertEquals(response.statusCode, 200);
	const responseBody = response.json();

	const data = responseBody.data;
	assert(Object.hasOwn(data, "postUpdated"));
	assert(Object.hasOwn(data.postUpdated, "authorId"));
	assertEquals(data.postUpdated.authorId, "123");
	assert(Object.hasOwn(data.postUpdated, "content"));
	assertEquals(
		data.postUpdated.content,
		"A comprehensive guide to GraphQL subscriptions with advanced filtering techniques...",
	);
	assert(Object.hasOwn(data.postUpdated, "id"));
	assertEquals(data.postUpdated.id, "post-789");
	assert(Object.hasOwn(data.postUpdated, "score"));
	assertEquals(data.postUpdated.score, 95);
	assert(Object.hasOwn(data.postUpdated, "status"));
	assertEquals(data.postUpdated.status, "PUBLISHED");
	assert(Object.hasOwn(data.postUpdated, "tags"));
	assertEquals(data.postUpdated.tags.length, 3);
	assertEquals(data.postUpdated.tags[0], "graphql");
	assertEquals(data.postUpdated.tags[1], "subscriptions");
	assertEquals(data.postUpdated.tags[2], "real-time");
	assert(Object.hasOwn(data.postUpdated, "title"));
	assertEquals(data.postUpdated.title, "Advanced GraphQL Patterns");
	assert(Object.hasOwn(data.postUpdated, "updatedAt"));
	assertEquals(data.postUpdated.updatedAt, "2025-12-27T15:45:00Z");
	const errors = responseBody.errors;
	assertEquals(errors?.length ?? 0, 0);
});

Deno.test("GraphQL subscription: filtered_subscription", async () => {
	const app = createAppGraphqlSubscription();
	const client = new TestClient(app);

	const response = await client.post("/graphql", {
		headers: { "x-spikard-fixture": "filtered_subscription" },
		json: {
			query: `subscription OnOrderUpdated($status: OrderStatus) {\n  orderUpdated(status: $status) {\n    id\n    orderId\n    status\n    amount\n    updatedAt\n  }\n}`,
			variables: { status: "SHIPPED" },
			operationName: "OnOrderUpdated",
		},
	});

	assertEquals(response.statusCode, 200);
	const responseBody = response.json();

	const data = responseBody.data;
	assert(Object.hasOwn(data, "orderUpdated"));
	assert(Object.hasOwn(data.orderUpdated, "amount"));
	assertEquals(data.orderUpdated.amount, 149.99);
	assert(Object.hasOwn(data.orderUpdated, "id"));
	assertEquals(data.orderUpdated.id, "order-456");
	assert(Object.hasOwn(data.orderUpdated, "orderId"));
	assertEquals(data.orderUpdated.orderId, "ORD-2025-00123");
	assert(Object.hasOwn(data.orderUpdated, "status"));
	assertEquals(data.orderUpdated.status, "SHIPPED");
	assert(Object.hasOwn(data.orderUpdated, "updatedAt"));
	assertEquals(data.orderUpdated.updatedAt, "2025-12-27T11:30:00Z");
	const errors = responseBody.errors;
	assertEquals(errors?.length ?? 0, 0);
});
