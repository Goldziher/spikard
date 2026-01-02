/**
 * GraphQL subscription tests
 * @generated
 */

import { TestClient } from "@spikard/wasm";
import { describe, expect, test } from "vitest";
import { createAppGraphqlSubscription } from "../app/main.ts";

describe("GraphQL subscription", () => {
	test("subscription_error", async () => {
		const app = createAppGraphqlSubscription();
		const client = new TestClient(app);

		const response = await client.post(
			"/graphql",
			{
				headers: { "x-spikard-fixture": "subscription_error" },
				json: {
					query: `subscription {\n  invalidSubscription {\n    id\n    data\n  }\n}`,
					variables: null,
					operationName: null,
				},
			},
		);

		expect(response.statusCode).toBe(400);
		const responseBody = response.json();

		const errors = responseBody.errors;
		expect(errors?.length ?? 0).toBe(0);
	});

	test("subscription_unsubscribe", async () => {
		const app = createAppGraphqlSubscription();
		const client = new TestClient(app);

		const response = await client.post(
			"/graphql",
			{
				headers: { "x-spikard-fixture": "subscription_unsubscribe" },
				json: {
					query: `subscription OnTick {\n  ticker {\n    id\n    symbol\n    price\n    timestamp\n  }\n}`,
					variables: null,
					operationName: "OnTick",
				},
			},
		);

		expect(response.statusCode).toBe(200);
		const responseBody = response.json();

		const data = responseBody.data;
		expect(data).toHaveProperty("ticker");
		expect(data.ticker).toHaveProperty("id");
		expect(data.ticker.id).toBe("tick-1");
		expect(data.ticker).toHaveProperty("symbol");
		expect(data.ticker.symbol).toBe("AAPL");
		expect(data.ticker).toHaveProperty("price");
		expect(data.ticker.price).toBe(195.45);
		expect(data.ticker).toHaveProperty("timestamp");
		expect(data.ticker.timestamp).toBe("2025-12-27T15:00:00Z");
		const errors = responseBody.errors;
		expect(errors?.length ?? 0).toBe(0);
	});

	test("subscription_connection_params", async () => {
		const app = createAppGraphqlSubscription();
		const client = new TestClient(app);

		const response = await client.post(
			"/graphql",
			{
				headers: { "x-spikard-fixture": "subscription_connection_params" },
				json: {
					query: `subscription {\n  secureStream {\n    id\n    data\n    timestamp\n  }\n}`,
					variables: null,
					operationName: null,
				},
			},
		);

		expect(response.statusCode).toBe(101);
		const responseBody = response.json();

		const data = responseBody.data;
		expect(data).toHaveProperty("secureStream");
		expect(data.secureStream).toHaveProperty("id");
		expect(data.secureStream.id).toBe("stream-1");
		expect(data.secureStream).toHaveProperty("data");
		expect(data.secureStream.data).toBe("Connection established");
		expect(data.secureStream).toHaveProperty("timestamp");
		expect(data.secureStream.timestamp).toBe("2025-12-27T14:00:00Z");
		const errors = responseBody.errors;
		expect(errors?.length ?? 0).toBe(0);
	});

	test("simple_subscription", async () => {
		const app = createAppGraphqlSubscription();
		const client = new TestClient(app);

		const response = await client.post(
			"/graphql",
			{
				headers: { "x-spikard-fixture": "simple_subscription" },
				json: {
					query: `subscription {\n  messageAdded {\n    id\n    text\n    timestamp\n  }\n}`,
					variables: null,
					operationName: null,
				},
			},
		);

		expect(response.statusCode).toBe(200);
		const responseBody = response.json();

		const data = responseBody.data;
		expect(data).toHaveProperty("messageAdded");
		expect(data.messageAdded).toHaveProperty("id");
		expect(data.messageAdded.id).toBe("msg-1");
		expect(data.messageAdded).toHaveProperty("text");
		expect(data.messageAdded.text).toBe("Hello, WebSocket!");
		expect(data.messageAdded).toHaveProperty("timestamp");
		expect(data.messageAdded.timestamp).toBe("2025-12-27T10:00:00Z");
		const errors = responseBody.errors;
		expect(errors?.length ?? 0).toBe(0);
	});

	test("subscription_multiple_fields", async () => {
		const app = createAppGraphqlSubscription();
		const client = new TestClient(app);

		const response = await client.post(
			"/graphql",
			{
				headers: { "x-spikard-fixture": "subscription_multiple_fields" },
				json: {
					query: `subscription MultiStream {\n  messageAdded {\n    id\n    text\n    author\n  }\n  userOnline {\n    userId\n    username\n    isOnline\n    lastSeen\n  }\n}`,
					variables: null,
					operationName: "MultiStream",
				},
			},
		);

		expect(response.statusCode).toBe(200);
		const responseBody = response.json();

		const data = responseBody.data;
		expect(data).toHaveProperty("messageAdded");
		expect(data.messageAdded).toHaveProperty("id");
		expect(data.messageAdded.id).toBe("msg-101");
		expect(data.messageAdded).toHaveProperty("text");
		expect(data.messageAdded.text).toBe("Hey everyone!");
		expect(data.messageAdded).toHaveProperty("author");
		expect(data.messageAdded.author).toBe("alice");
		expect(data).toHaveProperty("userOnline");
		expect(data.userOnline).toHaveProperty("userId");
		expect(data.userOnline.userId).toBe("user-42");
		expect(data.userOnline).toHaveProperty("username");
		expect(data.userOnline.username).toBe("bob");
		expect(data.userOnline).toHaveProperty("isOnline");
		expect(data.userOnline.isOnline).toBe(true);
		expect(data.userOnline).toHaveProperty("lastSeen");
		expect(data.userOnline.lastSeen).toBe("2025-12-27T13:00:00Z");
		const errors = responseBody.errors;
		expect(errors?.length ?? 0).toBe(0);
	});

	test("subscription_with_variables", async () => {
		const app = createAppGraphqlSubscription();
		const client = new TestClient(app);

		const response = await client.post(
			"/graphql",
			{
				headers: { "x-spikard-fixture": "subscription_with_variables" },
				json: {
					query: `subscription OnUserActivity($userId: ID!) {\n  userActivity(userId: $userId) {\n    id\n    userId\n    action\n    description\n    timestamp\n  }\n}`,
					variables: { userId: "user123" },
					operationName: "OnUserActivity",
				},
			},
		);

		expect(response.statusCode).toBe(200);
		const responseBody = response.json();

		const data = responseBody.data;
		expect(data).toHaveProperty("userActivity");
		expect(data.userActivity).toHaveProperty("id");
		expect(data.userActivity.id).toBe("event-789");
		expect(data.userActivity).toHaveProperty("userId");
		expect(data.userActivity.userId).toBe("user123");
		expect(data.userActivity).toHaveProperty("action");
		expect(data.userActivity.action).toBe("LOGIN");
		expect(data.userActivity).toHaveProperty("description");
		expect(data.userActivity.description).toBe("User logged in from browser");
		expect(data.userActivity).toHaveProperty("timestamp");
		expect(data.userActivity.timestamp).toBe("2025-12-27T12:15:00Z");
		const errors = responseBody.errors;
		expect(errors?.length ?? 0).toBe(0);
	});

	test("subscription_with_auth_middleware", async () => {
		const app = createAppGraphqlSubscription();
		const client = new TestClient(app);

		const response = await client.post(
			"/graphql",
			{
				headers: { "x-spikard-fixture": "subscription_with_auth_middleware" },
				json: {
					query: `subscription {\n  privateNotifications {\n    id\n    userId\n    type\n    message\n    priority\n    createdAt\n  }\n}`,
					variables: null,
					operationName: null,
				},
			},
		);

		expect(response.statusCode).toBe(101);
		const responseBody = response.json();

		const data = responseBody.data;
		expect(data).toHaveProperty("privateNotifications");
		expect(data.privateNotifications).toHaveProperty("id");
		expect(data.privateNotifications.id).toBe("notif-456");
		expect(data.privateNotifications).toHaveProperty("userId");
		expect(data.privateNotifications.userId).toBe("user123");
		expect(data.privateNotifications).toHaveProperty("type");
		expect(data.privateNotifications.type).toBe("ALERT");
		expect(data.privateNotifications).toHaveProperty("message");
		expect(data.privateNotifications.message).toBe("Your subscription is about to expire");
		expect(data.privateNotifications).toHaveProperty("priority");
		expect(data.privateNotifications.priority).toBe("HIGH");
		expect(data.privateNotifications).toHaveProperty("createdAt");
		expect(data.privateNotifications.createdAt).toBe("2025-12-27T16:20:00Z");
		const errors = responseBody.errors;
		expect(errors?.length ?? 0).toBe(0);
	});

	test("subscription_rate_limited", async () => {
		const app = createAppGraphqlSubscription();
		const client = new TestClient(app);

		const response = await client.post(
			"/graphql",
			{
				headers: { "x-spikard-fixture": "subscription_rate_limited" },
				json: {
					query: `subscription OnStockUpdate($symbol: String!) {\n  stockTicker(symbol: $symbol) {\n    id\n    symbol\n    price\n    change\n    changePercent\n    timestamp\n    volume\n  }\n}`,
					variables: { symbol: "AAPL" },
					operationName: "OnStockUpdate",
				},
			},
		);

		expect(response.statusCode).toBe(200);
		const responseBody = response.json();

		const data = responseBody.data;
		expect(data).toHaveProperty("stockTicker");
		expect(data.stockTicker).toHaveProperty("id");
		expect(data.stockTicker.id).toBe("stock-aapl-1");
		expect(data.stockTicker).toHaveProperty("symbol");
		expect(data.stockTicker.symbol).toBe("AAPL");
		expect(data.stockTicker).toHaveProperty("price");
		expect(data.stockTicker.price).toBe(238.45);
		expect(data.stockTicker).toHaveProperty("change");
		expect(data.stockTicker.change).toBe(2.15);
		expect(data.stockTicker).toHaveProperty("changePercent");
		expect(data.stockTicker.changePercent).toBe(0.91);
		expect(data.stockTicker).toHaveProperty("timestamp");
		expect(data.stockTicker.timestamp).toBe("2025-12-27T17:00:00Z");
		expect(data.stockTicker).toHaveProperty("volume");
		expect(data.stockTicker.volume).toBe(52345678);
		const errors = responseBody.errors;
		expect(errors?.length ?? 0).toBe(0);
	});

	test("subscription_authentication", async () => {
		const app = createAppGraphqlSubscription();
		const client = new TestClient(app);

		const response = await client.post(
			"/graphql",
			{
				headers: { "x-spikard-fixture": "subscription_authentication" },
				json: {
					query: `subscription {\n  privateMessages {\n    id\n    from\n    content\n    isPrivate\n  }\n}`,
					variables: null,
					operationName: null,
				},
			},
		);

		expect(response.statusCode).toBe(401);
		const responseBody = response.json();

		const errors = responseBody.errors;
		expect(errors?.length ?? 0).toBe(0);
	});

	test("subscription_with_filtering", async () => {
		const app = createAppGraphqlSubscription();
		const client = new TestClient(app);

		const response = await client.post(
			"/graphql",
			{
				headers: { "x-spikard-fixture": "subscription_with_filtering" },
				json: {
					query: `subscription OnPostUpdated($authorId: ID!, $statuses: [PostStatus!]!, $tagFilter: String, $scoreThreshold: Int) {\n  postUpdated(filter: {\n    authorId: $authorId\n    status: $statuses\n    tags_contains: $tagFilter\n    minScore: $scoreThreshold\n  }) {\n    id\n    title\n    authorId\n    content\n    status\n    tags\n    score\n    updatedAt\n  }\n}`,
					variables: { authorId: "123", statuses: ["PUBLISHED", "DRAFT"], tagFilter: "graphql", scoreThreshold: 50 },
					operationName: "OnPostUpdated",
				},
			},
		);

		expect(response.statusCode).toBe(200);
		const responseBody = response.json();

		const data = responseBody.data;
		expect(data).toHaveProperty("postUpdated");
		expect(data.postUpdated).toHaveProperty("id");
		expect(data.postUpdated.id).toBe("post-789");
		expect(data.postUpdated).toHaveProperty("title");
		expect(data.postUpdated.title).toBe("Advanced GraphQL Patterns");
		expect(data.postUpdated).toHaveProperty("authorId");
		expect(data.postUpdated.authorId).toBe("123");
		expect(data.postUpdated).toHaveProperty("content");
		expect(data.postUpdated.content).toBe("A comprehensive guide to GraphQL subscriptions with advanced filtering techniques...");
		expect(data.postUpdated).toHaveProperty("status");
		expect(data.postUpdated.status).toBe("PUBLISHED");
		expect(data.postUpdated).toHaveProperty("tags");
		expect(data.postUpdated.tags.length).toBe(3);
		expect(data.postUpdated.tags[0]).toBe("graphql");
		expect(data.postUpdated.tags[1]).toBe("subscriptions");
		expect(data.postUpdated.tags[2]).toBe("real-time");
		expect(data.postUpdated).toHaveProperty("score");
		expect(data.postUpdated.score).toBe(95);
		expect(data.postUpdated).toHaveProperty("updatedAt");
		expect(data.postUpdated.updatedAt).toBe("2025-12-27T15:45:00Z");
		const errors = responseBody.errors;
		expect(errors?.length ?? 0).toBe(0);
	});

	test("filtered_subscription", async () => {
		const app = createAppGraphqlSubscription();
		const client = new TestClient(app);

		const response = await client.post(
			"/graphql",
			{
				headers: { "x-spikard-fixture": "filtered_subscription" },
				json: {
					query: `subscription OnOrderUpdated($status: OrderStatus) {\n  orderUpdated(status: $status) {\n    id\n    orderId\n    status\n    amount\n    updatedAt\n  }\n}`,
					variables: { status: "SHIPPED" },
					operationName: "OnOrderUpdated",
				},
			},
		);

		expect(response.statusCode).toBe(200);
		const responseBody = response.json();

		const data = responseBody.data;
		expect(data).toHaveProperty("orderUpdated");
		expect(data.orderUpdated).toHaveProperty("id");
		expect(data.orderUpdated.id).toBe("order-456");
		expect(data.orderUpdated).toHaveProperty("orderId");
		expect(data.orderUpdated.orderId).toBe("ORD-2025-00123");
		expect(data.orderUpdated).toHaveProperty("status");
		expect(data.orderUpdated.status).toBe("SHIPPED");
		expect(data.orderUpdated).toHaveProperty("amount");
		expect(data.orderUpdated.amount).toBe(149.99);
		expect(data.orderUpdated).toHaveProperty("updatedAt");
		expect(data.orderUpdated.updatedAt).toBe("2025-12-27T11:30:00Z");
		const errors = responseBody.errors;
		expect(errors?.length ?? 0).toBe(0);
	});

});
