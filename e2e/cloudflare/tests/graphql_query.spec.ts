/**
 * GraphQL query tests
 * @generated
 */

import { TestClient } from "@spikard/wasm";
import { describe, expect, test } from "vitest";
import { createAppGraphqlQuery } from "../app/main.ts";

describe("GraphQL query", () => {
	test("transform_directive", async () => {
		const app = createAppGraphqlQuery();
		const client = new TestClient(app);

		const response = await client.post("/graphql", {
			headers: { "x-spikard-fixture": "transform_directive" },
			json: {
				query: `query {\n  message @uppercase\n  title @uppercase\n}`,
				variables: null,
				operationName: null,
			},
		});

		expect(response.statusCode).toBe(200);
		const responseBody = response.json();

		const data = responseBody.data;
		expect(data).toHaveProperty("message");
		expect(data.message).toBe("HELLO FROM GRAPHQL");
		expect(data).toHaveProperty("title");
		expect(data.title).toBe("WELCOME TO SPIKARD");
		const errors = responseBody.errors;
		expect(errors?.length ?? 0).toBe(0);
	});

	test("rate_limit_directive", async () => {
		const app = createAppGraphqlQuery();
		const client = new TestClient(app);

		const response = await client.post("/graphql", {
			headers: { "x-spikard-fixture": "rate_limit_directive" },
			json: {
				query: `query {\n  expensiveQuery\n}`,
				variables: null,
				operationName: null,
			},
		});

		expect(response.statusCode).toBe(200);
		const responseBody = response.json();

		const data = responseBody.data;
		expect(data).toHaveProperty("expensiveQuery");
		expect(data.expensiveQuery).toBe("Result from expensive computation");
		const errors = responseBody.errors;
		expect(errors?.length ?? 0).toBe(0);
	});

	test("cache_directive", async () => {
		const app = createAppGraphqlQuery();
		const client = new TestClient(app);

		const response = await client.post("/graphql", {
			headers: { "x-spikard-fixture": "cache_directive" },
			json: {
				query: `query GetUser($id: ID!) {\n  user(id: $id) {\n    id\n    name\n    email\n  }\n}`,
				variables: { id: "1" },
				operationName: null,
			},
		});

		expect(response.statusCode).toBe(200);
		const responseBody = response.json();

		const data = responseBody.data;
		expect(data).toHaveProperty("user");
		expect(data.user).toHaveProperty("id");
		expect(data.user.id).toBe("1");
		expect(data.user).toHaveProperty("name");
		expect(data.user.name).toBe("Alice Smith");
		expect(data.user).toHaveProperty("email");
		expect(data.user.email).toBe("alice@example.com");
		const errors = responseBody.errors;
		expect(errors?.length ?? 0).toBe(0);
	});

	test("custom_auth_directive", async () => {
		const app = createAppGraphqlQuery();
		const client = new TestClient(app);

		const response = await client.post("/graphql", {
			headers: { "x-spikard-fixture": "custom_auth_directive" },
			json: {
				query: `query {\n  publicData\n  secretData\n  moderatorData\n}`,
				variables: null,
				operationName: null,
			},
		});

		expect(response.statusCode).toBe(200);
		const responseBody = response.json();

		const data = responseBody.data;
		expect(data).toHaveProperty("publicData");
		expect(data.publicData).toBe("Anyone can see this");
		expect(data).toHaveProperty("secretData");
		expect(data.secretData).toBe(null);
		expect(data).toHaveProperty("moderatorData");
		expect(data.moderatorData).toBe(null);
		const errors = responseBody.errors;
		expect(errors).toBeDefined();
		expect(errors?.length).toBe(2);
		expect(errors?.[0]?.message).toContain("Unauthorized: User role USER cannot access ADMIN field");
		expect(errors?.[1]?.message).toContain("Unauthorized: User role USER cannot access MODERATOR field");
	});

	test("deprecated_field", async () => {
		const app = createAppGraphqlQuery();
		const client = new TestClient(app);

		const response = await client.post("/graphql", {
			headers: { "x-spikard-fixture": "deprecated_field" },
			json: {
				query: `query {\n  oldField\n  newField\n}`,
				variables: null,
				operationName: null,
			},
		});

		expect(response.statusCode).toBe(200);
		const responseBody = response.json();

		const data = responseBody.data;
		expect(data).toHaveProperty("oldField");
		expect(data.oldField).toBe("legacy value");
		expect(data).toHaveProperty("newField");
		expect(data.newField).toBe("modern value");
		const errors = responseBody.errors;
		expect(errors?.length ?? 0).toBe(0);
	});

	test("datetime_scalar", async () => {
		const app = createAppGraphqlQuery();
		const client = new TestClient(app);

		const response = await client.post("/graphql", {
			headers: { "x-spikard-fixture": "datetime_scalar" },
			json: {
				query: `query GetEvents($since: DateTime, $until: DateTime) {\n  events(since: $since, until: $until) {\n    id\n    title\n    scheduledAt\n    completedAt\n  }\n}`,
				variables: { since: "2025-01-01T00:00:00Z", until: "2025-12-31T23:59:59Z" },
				operationName: null,
			},
		});

		expect(response.statusCode).toBe(200);
		const responseBody = response.json();

		const data = responseBody.data;
		expect(data).toHaveProperty("events");
		expect(data.events.length).toBe(2);
		expect(data.events[0]).toHaveProperty("id");
		expect(data.events[0].id).toBe("event-1");
		expect(data.events[0]).toHaveProperty("title");
		expect(data.events[0].title).toBe("Conference");
		expect(data.events[0]).toHaveProperty("scheduledAt");
		expect(data.events[0].scheduledAt).toBe("2025-06-15T09:00:00Z");
		expect(data.events[0]).toHaveProperty("completedAt");
		expect(data.events[0].completedAt).toBe("2025-06-15T17:00:00Z");
		expect(data.events[1]).toHaveProperty("id");
		expect(data.events[1].id).toBe("event-2");
		expect(data.events[1]).toHaveProperty("title");
		expect(data.events[1].title).toBe("Hackathon");
		expect(data.events[1]).toHaveProperty("scheduledAt");
		expect(data.events[1].scheduledAt).toBe("2025-08-20T10:00:00Z");
		expect(data.events[1]).toHaveProperty("completedAt");
		expect(data.events[1].completedAt).toBe(null);
		const errors = responseBody.errors;
		expect(errors?.length ?? 0).toBe(0);
	});

	test("uuid_scalar", async () => {
		const app = createAppGraphqlQuery();
		const client = new TestClient(app);

		const response = await client.post("/graphql", {
			headers: { "x-spikard-fixture": "uuid_scalar" },
			json: {
				query: `query GetResource($id: UUID!) {\n  resource(id: $id) {\n    id\n    parentId\n    name\n    ownerId\n    relatedIds\n  }\n}`,
				variables: { id: "550e8400-e29b-41d4-a716-446655440000" },
				operationName: null,
			},
		});

		expect(response.statusCode).toBe(200);
		const responseBody = response.json();

		const data = responseBody.data;
		expect(data).toHaveProperty("resource");
		expect(data.resource).toHaveProperty("id");
		expect(data.resource.id).toBe("550e8400-e29b-41d4-a716-446655440000");
		expect(data.resource).toHaveProperty("parentId");
		expect(data.resource.parentId).toBe("6ba7b810-9dad-11d1-80b4-00c04fd430c8");
		expect(data.resource).toHaveProperty("name");
		expect(data.resource.name).toBe("Primary Resource");
		expect(data.resource).toHaveProperty("ownerId");
		expect(data.resource.ownerId).toBe("6ba7b811-9dad-11d1-80b4-00c04fd430c8");
		expect(data.resource).toHaveProperty("relatedIds");
		expect(data.resource.relatedIds.length).toBe(2);
		expect(data.resource.relatedIds[0]).toBe("6ba7b812-9dad-11d1-80b4-00c04fd430c8");
		expect(data.resource.relatedIds[1]).toBe("6ba7b814-9dad-11d1-80b4-00c04fd430c8");
		const errors = responseBody.errors;
		expect(errors?.length ?? 0).toBe(0);
	});

	test("json_scalar", async () => {
		const app = createAppGraphqlQuery();
		const client = new TestClient(app);

		const response = await client.post("/graphql", {
			headers: { "x-spikard-fixture": "json_scalar" },
			json: {
				query: `query GetConfig {\n  configuration {\n    id\n    name\n    settings\n    metadata\n  }\n}`,
				variables: null,
				operationName: null,
			},
		});

		expect(response.statusCode).toBe(200);
		const responseBody = response.json();

		const data = responseBody.data;
		expect(data).toHaveProperty("configuration");
		expect(data.configuration).toHaveProperty("id");
		expect(data.configuration.id).toBe("config-1");
		expect(data.configuration).toHaveProperty("name");
		expect(data.configuration.name).toBe("Production Config");
		expect(data.configuration).toHaveProperty("settings");
		expect(data.configuration.settings).toHaveProperty("timeout");
		expect(data.configuration.settings.timeout).toBe(30000);
		expect(data.configuration.settings).toHaveProperty("retries");
		expect(data.configuration.settings.retries).toBe(3);
		expect(data.configuration.settings).toHaveProperty("features");
		expect(data.configuration.settings.features).toHaveProperty("caching");
		expect(data.configuration.settings.features.caching).toBe(true);
		expect(data.configuration.settings.features).toHaveProperty("compression");
		expect(data.configuration.settings.features.compression).toBe(true);
		expect(data.configuration.settings.features).toHaveProperty("tracing");
		expect(data.configuration.settings.features.tracing).toBe(false);
		expect(data.configuration.settings).toHaveProperty("endpoints");
		expect(data.configuration.settings.endpoints.length).toBe(2);
		expect(data.configuration.settings.endpoints[0]).toBe("https://api.example.com");
		expect(data.configuration.settings.endpoints[1]).toBe("https://api-backup.example.com");
		expect(data.configuration).toHaveProperty("metadata");
		expect(data.configuration.metadata).toHaveProperty("version");
		expect(data.configuration.metadata.version).toBe("1.0.0");
		expect(data.configuration.metadata).toHaveProperty("environment");
		expect(data.configuration.metadata.environment).toBe("production");
		expect(data.configuration.metadata).toHaveProperty("lastUpdated");
		expect(data.configuration.metadata.lastUpdated).toBe("2025-12-27T10:00:00Z");
		expect(data.configuration.metadata).toHaveProperty("author");
		expect(data.configuration.metadata.author).toBe("DevOps Team");
		const errors = responseBody.errors;
		expect(errors?.length ?? 0).toBe(0);
	});

	test("complex_query", async () => {
		const app = createAppGraphqlQuery();
		const client = new TestClient(app);

		const response = await client.post("/graphql", {
			headers: { "x-spikard-fixture": "complex_query" },
			json: {
				query: `query ComplexSearch($searchTerm: String!, $userLimit: Int!, $postLimit: Int!) {\n  search(term: $searchTerm) {\n    total\n    users(limit: $userLimit) {\n      id\n      name\n      email\n      profile {\n        bio\n        avatar\n        joinedAt\n      }\n      recentPosts: posts(limit: 3) {\n        id\n        title\n        likes\n      }\n      followerCount: followers(limit: 100) {\n        id\n      }\n    }\n    posts(limit: $postLimit) {\n      id\n      title\n      content\n      likes\n      author {\n        id\n        name\n        profile {\n          avatar\n        }\n      }\n      topComments: comments(limit: 5) {\n        id\n        text\n        likes\n        author {\n          id\n          name\n        }\n      }\n    }\n  }\n}`,
				variables: { searchTerm: "graphql", userLimit: 5, postLimit: 10 },
				operationName: "ComplexSearch",
			},
		});

		expect(response.statusCode).toBe(200);
		const responseBody = response.json();

		const data = responseBody.data;
		expect(data).toHaveProperty("search");
		expect(data.search).toHaveProperty("total");
		expect(data.search.total).toBe(42);
		expect(data.search).toHaveProperty("users");
		expect(data.search.users.length).toBe(2);
		expect(data.search.users[0]).toHaveProperty("id");
		expect(data.search.users[0].id).toBe("user-1");
		expect(data.search.users[0]).toHaveProperty("name");
		expect(data.search.users[0].name).toBe("GraphQL Expert");
		expect(data.search.users[0]).toHaveProperty("email");
		expect(data.search.users[0].email).toBe("expert@example.com");
		expect(data.search.users[0]).toHaveProperty("profile");
		expect(data.search.users[0].profile).toHaveProperty("bio");
		expect(data.search.users[0].profile.bio).toBe("GraphQL enthusiast and API designer");
		expect(data.search.users[0].profile).toHaveProperty("avatar");
		expect(data.search.users[0].profile.avatar).toBe("https://example.com/avatars/expert.jpg");
		expect(data.search.users[0].profile).toHaveProperty("joinedAt");
		expect(data.search.users[0].profile.joinedAt).toBe("2024-01-15T08:30:00Z");
		expect(data.search.users[0]).toHaveProperty("recentPosts");
		expect(data.search.users[0].recentPosts.length).toBe(3);
		expect(data.search.users[0].recentPosts[0]).toHaveProperty("id");
		expect(data.search.users[0].recentPosts[0].id).toBe("post-101");
		expect(data.search.users[0].recentPosts[0]).toHaveProperty("title");
		expect(data.search.users[0].recentPosts[0].title).toBe("GraphQL Best Practices");
		expect(data.search.users[0].recentPosts[0]).toHaveProperty("likes");
		expect(data.search.users[0].recentPosts[0].likes).toBe(234);
		expect(data.search.users[0].recentPosts[1]).toHaveProperty("id");
		expect(data.search.users[0].recentPosts[1].id).toBe("post-102");
		expect(data.search.users[0].recentPosts[1]).toHaveProperty("title");
		expect(data.search.users[0].recentPosts[1].title).toBe("Schema Design Patterns");
		expect(data.search.users[0].recentPosts[1]).toHaveProperty("likes");
		expect(data.search.users[0].recentPosts[1].likes).toBe(189);
		expect(data.search.users[0].recentPosts[2]).toHaveProperty("id");
		expect(data.search.users[0].recentPosts[2].id).toBe("post-103");
		expect(data.search.users[0].recentPosts[2]).toHaveProperty("title");
		expect(data.search.users[0].recentPosts[2].title).toBe("Performance Optimization");
		expect(data.search.users[0].recentPosts[2]).toHaveProperty("likes");
		expect(data.search.users[0].recentPosts[2].likes).toBe(156);
		expect(data.search.users[0]).toHaveProperty("followerCount");
		expect(data.search.users[0].followerCount.length).toBe(2);
		expect(data.search.users[0].followerCount[0]).toHaveProperty("id");
		expect(data.search.users[0].followerCount[0].id).toBe("user-2");
		expect(data.search.users[0].followerCount[1]).toHaveProperty("id");
		expect(data.search.users[0].followerCount[1].id).toBe("user-3");
		expect(data.search.users[1]).toHaveProperty("id");
		expect(data.search.users[1].id).toBe("user-2");
		expect(data.search.users[1]).toHaveProperty("name");
		expect(data.search.users[1].name).toBe("API Developer");
		expect(data.search.users[1]).toHaveProperty("email");
		expect(data.search.users[1].email).toBe("developer@example.com");
		expect(data.search.users[1]).toHaveProperty("profile");
		expect(data.search.users[1].profile).toHaveProperty("bio");
		expect(data.search.users[1].profile.bio).toBe("Building scalable APIs");
		expect(data.search.users[1].profile).toHaveProperty("avatar");
		expect(data.search.users[1].profile.avatar).toBe("https://example.com/avatars/developer.jpg");
		expect(data.search.users[1].profile).toHaveProperty("joinedAt");
		expect(data.search.users[1].profile.joinedAt).toBe("2024-02-20T10:15:00Z");
		expect(data.search.users[1]).toHaveProperty("recentPosts");
		expect(data.search.users[1].recentPosts.length).toBe(1);
		expect(data.search.users[1].recentPosts[0]).toHaveProperty("id");
		expect(data.search.users[1].recentPosts[0].id).toBe("post-201");
		expect(data.search.users[1].recentPosts[0]).toHaveProperty("title");
		expect(data.search.users[1].recentPosts[0].title).toBe("GraphQL vs REST");
		expect(data.search.users[1].recentPosts[0]).toHaveProperty("likes");
		expect(data.search.users[1].recentPosts[0].likes).toBe(145);
		expect(data.search.users[1]).toHaveProperty("followerCount");
		expect(data.search.users[1].followerCount.length).toBe(1);
		expect(data.search.users[1].followerCount[0]).toHaveProperty("id");
		expect(data.search.users[1].followerCount[0].id).toBe("user-1");
		expect(data.search).toHaveProperty("posts");
		expect(data.search.posts.length).toBe(2);
		expect(data.search.posts[0]).toHaveProperty("id");
		expect(data.search.posts[0].id).toBe("post-101");
		expect(data.search.posts[0]).toHaveProperty("title");
		expect(data.search.posts[0].title).toBe("GraphQL Best Practices");
		expect(data.search.posts[0]).toHaveProperty("content");
		expect(data.search.posts[0].content).toBe("A comprehensive guide to GraphQL best practices and patterns...");
		expect(data.search.posts[0]).toHaveProperty("likes");
		expect(data.search.posts[0].likes).toBe(234);
		expect(data.search.posts[0]).toHaveProperty("author");
		expect(data.search.posts[0].author).toHaveProperty("id");
		expect(data.search.posts[0].author.id).toBe("user-1");
		expect(data.search.posts[0].author).toHaveProperty("name");
		expect(data.search.posts[0].author.name).toBe("GraphQL Expert");
		expect(data.search.posts[0].author).toHaveProperty("profile");
		expect(data.search.posts[0].author.profile).toHaveProperty("avatar");
		expect(data.search.posts[0].author.profile.avatar).toBe("https://example.com/avatars/expert.jpg");
		expect(data.search.posts[0]).toHaveProperty("topComments");
		expect(data.search.posts[0].topComments.length).toBe(2);
		expect(data.search.posts[0].topComments[0]).toHaveProperty("id");
		expect(data.search.posts[0].topComments[0].id).toBe("comment-1");
		expect(data.search.posts[0].topComments[0]).toHaveProperty("text");
		expect(data.search.posts[0].topComments[0].text).toBe("Great post, very helpful!");
		expect(data.search.posts[0].topComments[0]).toHaveProperty("likes");
		expect(data.search.posts[0].topComments[0].likes).toBe(45);
		expect(data.search.posts[0].topComments[0]).toHaveProperty("author");
		expect(data.search.posts[0].topComments[0].author).toHaveProperty("id");
		expect(data.search.posts[0].topComments[0].author.id).toBe("user-2");
		expect(data.search.posts[0].topComments[0].author).toHaveProperty("name");
		expect(data.search.posts[0].topComments[0].author.name).toBe("API Developer");
		expect(data.search.posts[0].topComments[1]).toHaveProperty("id");
		expect(data.search.posts[0].topComments[1].id).toBe("comment-2");
		expect(data.search.posts[0].topComments[1]).toHaveProperty("text");
		expect(data.search.posts[0].topComments[1].text).toBe("Could you elaborate on caching?");
		expect(data.search.posts[0].topComments[1]).toHaveProperty("likes");
		expect(data.search.posts[0].topComments[1].likes).toBe(32);
		expect(data.search.posts[0].topComments[1]).toHaveProperty("author");
		expect(data.search.posts[0].topComments[1].author).toHaveProperty("id");
		expect(data.search.posts[0].topComments[1].author.id).toBe("user-3");
		expect(data.search.posts[0].topComments[1].author).toHaveProperty("name");
		expect(data.search.posts[0].topComments[1].author.name).toBe("Data Scientist");
		expect(data.search.posts[1]).toHaveProperty("id");
		expect(data.search.posts[1].id).toBe("post-102");
		expect(data.search.posts[1]).toHaveProperty("title");
		expect(data.search.posts[1].title).toBe("Schema Design Patterns");
		expect(data.search.posts[1]).toHaveProperty("content");
		expect(data.search.posts[1].content).toBe("Exploring common patterns for designing GraphQL schemas...");
		expect(data.search.posts[1]).toHaveProperty("likes");
		expect(data.search.posts[1].likes).toBe(189);
		expect(data.search.posts[1]).toHaveProperty("author");
		expect(data.search.posts[1].author).toHaveProperty("id");
		expect(data.search.posts[1].author.id).toBe("user-1");
		expect(data.search.posts[1].author).toHaveProperty("name");
		expect(data.search.posts[1].author.name).toBe("GraphQL Expert");
		expect(data.search.posts[1].author).toHaveProperty("profile");
		expect(data.search.posts[1].author.profile).toHaveProperty("avatar");
		expect(data.search.posts[1].author.profile.avatar).toBe("https://example.com/avatars/expert.jpg");
		expect(data.search.posts[1]).toHaveProperty("topComments");
		expect(data.search.posts[1].topComments.length).toBe(1);
		expect(data.search.posts[1].topComments[0]).toHaveProperty("id");
		expect(data.search.posts[1].topComments[0].id).toBe("comment-3");
		expect(data.search.posts[1].topComments[0]).toHaveProperty("text");
		expect(data.search.posts[1].topComments[0].text).toBe("Excellent breakdown");
		expect(data.search.posts[1].topComments[0]).toHaveProperty("likes");
		expect(data.search.posts[1].topComments[0].likes).toBe(28);
		expect(data.search.posts[1].topComments[0]).toHaveProperty("author");
		expect(data.search.posts[1].topComments[0].author).toHaveProperty("id");
		expect(data.search.posts[1].topComments[0].author.id).toBe("user-4");
		expect(data.search.posts[1].topComments[0].author).toHaveProperty("name");
		expect(data.search.posts[1].topComments[0].author.name).toBe("Backend Engineer");
		const errors = responseBody.errors;
		expect(errors?.length ?? 0).toBe(0);
	});

	test("deeply_nested_query", async () => {
		const app = createAppGraphqlQuery();
		const client = new TestClient(app);

		const response = await client.post("/graphql", {
			headers: { "x-spikard-fixture": "deeply_nested_query" },
			json: {
				query: `query GetUserDeepNested($userId: String!) {\n  user(id: $userId) {\n    id\n    name\n    profile {\n      bio\n      settings {\n        preferences {\n          theme\n          language\n          timezone {\n            name\n            offset\n          }\n        }\n        notifications {\n          email\n          push\n        }\n      }\n    }\n  }\n}`,
				variables: { userId: "user-deep-001" },
				operationName: null,
			},
		});

		expect(response.statusCode).toBe(200);
		const responseBody = response.json();

		const data = responseBody.data;
		expect(data).toHaveProperty("user");
		expect(data.user).toHaveProperty("id");
		expect(data.user.id).toBe("user-deep-001");
		expect(data.user).toHaveProperty("name");
		expect(data.user.name).toBe("Alice Cooper");
		expect(data.user).toHaveProperty("profile");
		expect(data.user.profile).toHaveProperty("bio");
		expect(data.user.profile.bio).toBe("DevOps engineer passionate about scalability");
		expect(data.user.profile).toHaveProperty("settings");
		expect(data.user.profile.settings).toHaveProperty("preferences");
		expect(data.user.profile.settings.preferences).toHaveProperty("theme");
		expect(data.user.profile.settings.preferences.theme).toBe("dark");
		expect(data.user.profile.settings.preferences).toHaveProperty("language");
		expect(data.user.profile.settings.preferences.language).toBe("en-US");
		expect(data.user.profile.settings.preferences).toHaveProperty("timezone");
		expect(data.user.profile.settings.preferences.timezone).toHaveProperty("name");
		expect(data.user.profile.settings.preferences.timezone.name).toBe("America/Los_Angeles");
		expect(data.user.profile.settings.preferences.timezone).toHaveProperty("offset");
		expect(data.user.profile.settings.preferences.timezone.offset).toBe(-480);
		expect(data.user.profile.settings).toHaveProperty("notifications");
		expect(data.user.profile.settings.notifications).toHaveProperty("email");
		expect(data.user.profile.settings.notifications.email).toBe(true);
		expect(data.user.profile.settings.notifications).toHaveProperty("push");
		expect(data.user.profile.settings.notifications.push).toBe(false);
		const errors = responseBody.errors;
		expect(errors?.length ?? 0).toBe(0);
	});

	test("persisted_query_allowlist", async () => {
		const app = createAppGraphqlQuery();
		const client = new TestClient(app);

		const response = await client.post("/graphql", {
			headers: { "x-spikard-fixture": "persisted_query_allowlist" },
			json: {
				query: ``,
				variables: {},
				operationName: null,
			},
		});

		expect(response.statusCode).toBe(403);
		const responseBody = response.json();

		const errors = responseBody.errors;
		expect(errors).toBeDefined();
		expect(errors?.length).toBe(1);
		expect(errors?.[0]?.message).toContain("Query not in allowlist");
	});

	test("persisted_query_hash_mismatch", async () => {
		const app = createAppGraphqlQuery();
		const client = new TestClient(app);

		const response = await client.post("/graphql", {
			headers: { "x-spikard-fixture": "persisted_query_hash_mismatch" },
			json: {
				query: `query GetUser($id: ID!) {\n  user(id: $id) {\n    id\n    name\n    email\n  }\n}`,
				variables: { id: "user-999" },
				operationName: "GetUser",
			},
		});

		expect(response.statusCode).toBe(400);
		const responseBody = response.json();

		const errors = responseBody.errors;
		expect(errors).toBeDefined();
		expect(errors?.length).toBe(1);
		expect(errors?.[0]?.message).toContain("Hash mismatch");
	});

	test("persisted_query_registration", async () => {
		const app = createAppGraphqlQuery();
		const client = new TestClient(app);

		const response = await client.post("/graphql", {
			headers: { "x-spikard-fixture": "persisted_query_registration" },
			json: {
				query: `query GetUserPosts($userId: ID!) {\n  posts(userId: $userId) {\n    id\n    title\n    content\n    author {\n      id\n      name\n    }\n  }\n}`,
				variables: { userId: "user-789" },
				operationName: "GetUserPosts",
			},
		});

		expect(response.statusCode).toBe(200);
		const responseBody = response.json();

		const data = responseBody.data;
		expect(data).toHaveProperty("posts");
		expect(data.posts.length).toBe(2);
		expect(data.posts[0]).toHaveProperty("id");
		expect(data.posts[0].id).toBe("post-1");
		expect(data.posts[0]).toHaveProperty("title");
		expect(data.posts[0].title).toBe("GraphQL Best Practices");
		expect(data.posts[0]).toHaveProperty("content");
		expect(data.posts[0].content).toBe("Understanding GraphQL query optimization...");
		expect(data.posts[0]).toHaveProperty("author");
		expect(data.posts[0].author).toHaveProperty("id");
		expect(data.posts[0].author.id).toBe("user-789");
		expect(data.posts[0].author).toHaveProperty("name");
		expect(data.posts[0].author.name).toBe("Bob Johnson");
		expect(data.posts[1]).toHaveProperty("id");
		expect(data.posts[1].id).toBe("post-2");
		expect(data.posts[1]).toHaveProperty("title");
		expect(data.posts[1].title).toBe("Persisted Queries Guide");
		expect(data.posts[1]).toHaveProperty("content");
		expect(data.posts[1].content).toBe("How to implement persisted queries for performance...");
		expect(data.posts[1]).toHaveProperty("author");
		expect(data.posts[1].author).toHaveProperty("id");
		expect(data.posts[1].author.id).toBe("user-789");
		expect(data.posts[1].author).toHaveProperty("name");
		expect(data.posts[1].author.name).toBe("Bob Johnson");
		const errors = responseBody.errors;
		expect(errors?.length ?? 0).toBe(0);
	});

	test("persisted_query_hit", async () => {
		const app = createAppGraphqlQuery();
		const client = new TestClient(app);

		const response = await client.post("/graphql", {
			headers: { "x-spikard-fixture": "persisted_query_hit" },
			json: {
				query: ``,
				variables: { id: "user-123" },
				operationName: null,
			},
		});

		expect(response.statusCode).toBe(200);
		const responseBody = response.json();

		const data = responseBody.data;
		expect(data).toHaveProperty("user");
		expect(data.user).toHaveProperty("id");
		expect(data.user.id).toBe("user-123");
		expect(data.user).toHaveProperty("name");
		expect(data.user.name).toBe("Alice Smith");
		expect(data.user).toHaveProperty("email");
		expect(data.user.email).toBe("alice@example.com");
		const errors = responseBody.errors;
		expect(errors?.length ?? 0).toBe(0);
	});

	test("persisted_query_miss", async () => {
		const app = createAppGraphqlQuery();
		const client = new TestClient(app);

		const response = await client.post("/graphql", {
			headers: { "x-spikard-fixture": "persisted_query_miss" },
			json: {
				query: ``,
				variables: { id: "user-456" },
				operationName: null,
			},
		});

		expect(response.statusCode).toBe(400);
		const responseBody = response.json();

		const errors = responseBody.errors;
		expect(errors).toBeDefined();
		expect(errors?.length).toBe(1);
		expect(errors?.[0]?.message).toContain("PersistedQueryNotFound");
	});

	test("persisted_query_automatic_persisted", async () => {
		const app = createAppGraphqlQuery();
		const client = new TestClient(app);

		const response = await client.post("/graphql", {
			headers: { "x-spikard-fixture": "persisted_query_automatic_persisted" },
			json: {
				query: ``,
				variables: { q: "GraphQL" },
				operationName: null,
			},
		});

		expect(response.statusCode).toBe(400);
		const responseBody = response.json();

		const errors = responseBody.errors;
		expect(errors).toBeDefined();
		expect(errors?.length).toBe(1);
		expect(errors?.[0]?.message).toContain("PersistedQueryNotFound");
	});

	test("with_arguments", async () => {
		const app = createAppGraphqlQuery();
		const client = new TestClient(app);

		const response = await client.post("/graphql", {
			headers: { "x-spikard-fixture": "with_arguments" },
			json: {
				query: `query Greet($name: String!) {\n  greet(name: $name)\n}`,
				variables: { name: "Alice" },
				operationName: "Greet",
			},
		});

		expect(response.statusCode).toBe(200);
		const responseBody = response.json();

		const data = responseBody.data;
		expect(data).toHaveProperty("greet");
		expect(data.greet).toBe("Hello, Alice!");
		const errors = responseBody.errors;
		expect(errors?.length ?? 0).toBe(0);
	});

	test("nested_objects", async () => {
		const app = createAppGraphqlQuery();
		const client = new TestClient(app);

		const response = await client.post("/graphql", {
			headers: { "x-spikard-fixture": "nested_objects" },
			json: {
				query: `query GetUser($userId: String!) {\n  user(id: $userId) {\n    id\n    name\n    email\n    profile {\n      bio\n      location\n    }\n  }\n}`,
				variables: { userId: "550e8400-e29b-41d4-a716-446655440000" },
				operationName: null,
			},
		});

		expect(response.statusCode).toBe(200);
		const responseBody = response.json();

		const data = responseBody.data;
		expect(data).toHaveProperty("user");
		expect(data.user).toHaveProperty("id");
		expect(data.user.id).toBe("550e8400-e29b-41d4-a716-446655440000");
		expect(data.user).toHaveProperty("name");
		expect(data.user.name).toBe("Alice Johnson");
		expect(data.user).toHaveProperty("email");
		expect(data.user.email).toBe("alice@example.com");
		expect(data.user).toHaveProperty("profile");
		expect(data.user.profile).toHaveProperty("bio");
		expect(data.user.profile.bio).toBe("Software engineer and open source enthusiast");
		expect(data.user.profile).toHaveProperty("location");
		expect(data.user.profile.location).toBe("San Francisco, CA");
		const errors = responseBody.errors;
		expect(errors?.length ?? 0).toBe(0);
	});

	test("simple_field", async () => {
		const app = createAppGraphqlQuery();
		const client = new TestClient(app);

		const response = await client.post("/graphql", {
			headers: { "x-spikard-fixture": "simple_field" },
			json: {
				query: `query {\n  hello\n}`,
				variables: null,
				operationName: null,
			},
		});

		expect(response.statusCode).toBe(200);
		const responseBody = response.json();

		const data = responseBody.data;
		expect(data).toHaveProperty("hello");
		expect(data.hello).toBe("Hello, World!");
		const errors = responseBody.errors;
		expect(errors?.length ?? 0).toBe(0);
	});

	test("introspection_disabled", async () => {
		const app = createAppGraphqlQuery();
		const client = new TestClient(app);

		const response = await client.post("/graphql", {
			headers: { "x-spikard-fixture": "introspection_disabled" },
			json: {
				query: ``,
				variables: null,
				operationName: null,
			},
		});

		expect(response.statusCode).toBe(400);
		const responseBody = response.json();

		const errors = responseBody.errors;
		expect(errors).toBeDefined();
		expect(errors?.length).toBe(1);
		expect(errors?.[0]?.message).toContain("Introspection is disabled");
	});

	test("full_schema_introspection", async () => {
		const app = createAppGraphqlQuery();
		const client = new TestClient(app);

		const response = await client.post("/graphql", {
			headers: { "x-spikard-fixture": "full_schema_introspection" },
			json: {
				query: ``,
				variables: null,
				operationName: null,
			},
		});

		expect(response.statusCode).toBe(200);
		const responseBody = response.json();

		const data = responseBody.data;
		expect(data).toHaveProperty("__schema");
		expect(data.__schema).toHaveProperty("queryType");
		expect(data.__schema.queryType).toHaveProperty("name");
		expect(data.__schema.queryType.name).toBe("Query");
		expect(data.__schema).toHaveProperty("mutationType");
		expect(data.__schema.mutationType).toHaveProperty("name");
		expect(data.__schema.mutationType.name).toBe("Mutation");
		expect(data.__schema).toHaveProperty("subscriptionType");
		expect(data.__schema.subscriptionType).toBe(null);
		expect(data.__schema).toHaveProperty("types");
		expect(data.__schema.types.length).toBe(7);
		expect(data.__schema.types[0]).toHaveProperty("kind");
		expect(data.__schema.types[0].kind).toBe("SCALAR");
		expect(data.__schema.types[0]).toHaveProperty("name");
		expect(data.__schema.types[0].name).toBe("DateTime");
		expect(data.__schema.types[0]).toHaveProperty("description");
		expect(data.__schema.types[0].description).toBe("ISO 8601 DateTime scalar");
		expect(data.__schema.types[0]).toHaveProperty("fields");
		expect(data.__schema.types[0].fields).toBe(null);
		expect(data.__schema.types[0]).toHaveProperty("inputFields");
		expect(data.__schema.types[0].inputFields).toBe(null);
		expect(data.__schema.types[0]).toHaveProperty("interfaces");
		expect(data.__schema.types[0].interfaces).toBe(null);
		expect(data.__schema.types[0]).toHaveProperty("enumValues");
		expect(data.__schema.types[0].enumValues).toBe(null);
		expect(data.__schema.types[0]).toHaveProperty("possibleTypes");
		expect(data.__schema.types[0].possibleTypes).toBe(null);
		expect(data.__schema.types[1]).toHaveProperty("kind");
		expect(data.__schema.types[1].kind).toBe("SCALAR");
		expect(data.__schema.types[1]).toHaveProperty("name");
		expect(data.__schema.types[1].name).toBe("UUID");
		expect(data.__schema.types[1]).toHaveProperty("description");
		expect(data.__schema.types[1].description).toBe("UUID scalar type");
		expect(data.__schema.types[1]).toHaveProperty("fields");
		expect(data.__schema.types[1].fields).toBe(null);
		expect(data.__schema.types[1]).toHaveProperty("inputFields");
		expect(data.__schema.types[1].inputFields).toBe(null);
		expect(data.__schema.types[1]).toHaveProperty("interfaces");
		expect(data.__schema.types[1].interfaces).toBe(null);
		expect(data.__schema.types[1]).toHaveProperty("enumValues");
		expect(data.__schema.types[1].enumValues).toBe(null);
		expect(data.__schema.types[1]).toHaveProperty("possibleTypes");
		expect(data.__schema.types[1].possibleTypes).toBe(null);
		expect(data.__schema.types[2]).toHaveProperty("kind");
		expect(data.__schema.types[2].kind).toBe("OBJECT");
		expect(data.__schema.types[2]).toHaveProperty("name");
		expect(data.__schema.types[2].name).toBe("Query");
		expect(data.__schema.types[2]).toHaveProperty("description");
		expect(data.__schema.types[2].description).toBe("Root query type");
		expect(data.__schema.types[2]).toHaveProperty("fields");
		expect(data.__schema.types[2].fields.length).toBe(4);
		expect(data.__schema.types[2].fields[0]).toHaveProperty("name");
		expect(data.__schema.types[2].fields[0].name).toBe("hello");
		expect(data.__schema.types[2].fields[0]).toHaveProperty("description");
		expect(data.__schema.types[2].fields[0].description).toBe("Greeting message");
		expect(data.__schema.types[2].fields[0]).toHaveProperty("args");
		expect(data.__schema.types[2].fields[0].args.length).toBe(0);
		expect(data.__schema.types[2].fields[0]).toHaveProperty("type");
		expect(data.__schema.types[2].fields[0].type).toHaveProperty("kind");
		expect(data.__schema.types[2].fields[0].type.kind).toBe("NON_NULL");
		expect(data.__schema.types[2].fields[0].type).toHaveProperty("name");
		expect(data.__schema.types[2].fields[0].type.name).toBe(null);
		expect(data.__schema.types[2].fields[0].type).toHaveProperty("ofType");
		expect(data.__schema.types[2].fields[0].type.ofType).toHaveProperty("kind");
		expect(data.__schema.types[2].fields[0].type.ofType.kind).toBe("SCALAR");
		expect(data.__schema.types[2].fields[0].type.ofType).toHaveProperty("name");
		expect(data.__schema.types[2].fields[0].type.ofType.name).toBe("String");
		expect(data.__schema.types[2].fields[0]).toHaveProperty("isDeprecated");
		expect(data.__schema.types[2].fields[0].isDeprecated).toBe(false);
		expect(data.__schema.types[2].fields[0]).toHaveProperty("deprecationReason");
		expect(data.__schema.types[2].fields[0].deprecationReason).toBe(null);
		expect(data.__schema.types[2].fields[1]).toHaveProperty("name");
		expect(data.__schema.types[2].fields[1].name).toBe("version");
		expect(data.__schema.types[2].fields[1]).toHaveProperty("description");
		expect(data.__schema.types[2].fields[1].description).toBe("API version");
		expect(data.__schema.types[2].fields[1]).toHaveProperty("args");
		expect(data.__schema.types[2].fields[1].args.length).toBe(0);
		expect(data.__schema.types[2].fields[1]).toHaveProperty("type");
		expect(data.__schema.types[2].fields[1].type).toHaveProperty("kind");
		expect(data.__schema.types[2].fields[1].type.kind).toBe("NON_NULL");
		expect(data.__schema.types[2].fields[1].type).toHaveProperty("name");
		expect(data.__schema.types[2].fields[1].type.name).toBe(null);
		expect(data.__schema.types[2].fields[1].type).toHaveProperty("ofType");
		expect(data.__schema.types[2].fields[1].type.ofType).toHaveProperty("kind");
		expect(data.__schema.types[2].fields[1].type.ofType.kind).toBe("SCALAR");
		expect(data.__schema.types[2].fields[1].type.ofType).toHaveProperty("name");
		expect(data.__schema.types[2].fields[1].type.ofType.name).toBe("String");
		expect(data.__schema.types[2].fields[1]).toHaveProperty("isDeprecated");
		expect(data.__schema.types[2].fields[1].isDeprecated).toBe(false);
		expect(data.__schema.types[2].fields[1]).toHaveProperty("deprecationReason");
		expect(data.__schema.types[2].fields[1].deprecationReason).toBe(null);
		expect(data.__schema.types[2].fields[2]).toHaveProperty("name");
		expect(data.__schema.types[2].fields[2].name).toBe("user");
		expect(data.__schema.types[2].fields[2]).toHaveProperty("description");
		expect(data.__schema.types[2].fields[2].description).toBe("Get user by ID");
		expect(data.__schema.types[2].fields[2]).toHaveProperty("args");
		expect(data.__schema.types[2].fields[2].args.length).toBe(1);
		expect(data.__schema.types[2].fields[2].args[0]).toHaveProperty("name");
		expect(data.__schema.types[2].fields[2].args[0].name).toBe("id");
		expect(data.__schema.types[2].fields[2].args[0]).toHaveProperty("description");
		expect(data.__schema.types[2].fields[2].args[0].description).toBe("User ID");
		expect(data.__schema.types[2].fields[2].args[0]).toHaveProperty("type");
		expect(data.__schema.types[2].fields[2].args[0].type).toHaveProperty("kind");
		expect(data.__schema.types[2].fields[2].args[0].type.kind).toBe("NON_NULL");
		expect(data.__schema.types[2].fields[2].args[0].type).toHaveProperty("name");
		expect(data.__schema.types[2].fields[2].args[0].type.name).toBe(null);
		expect(data.__schema.types[2].fields[2].args[0].type).toHaveProperty("ofType");
		expect(data.__schema.types[2].fields[2].args[0].type.ofType).toHaveProperty("kind");
		expect(data.__schema.types[2].fields[2].args[0].type.ofType.kind).toBe("SCALAR");
		expect(data.__schema.types[2].fields[2].args[0].type.ofType).toHaveProperty("name");
		expect(data.__schema.types[2].fields[2].args[0].type.ofType.name).toBe("UUID");
		expect(data.__schema.types[2].fields[2].args[0]).toHaveProperty("defaultValue");
		expect(data.__schema.types[2].fields[2].args[0].defaultValue).toBe(null);
		expect(data.__schema.types[2].fields[2]).toHaveProperty("type");
		expect(data.__schema.types[2].fields[2].type).toHaveProperty("kind");
		expect(data.__schema.types[2].fields[2].type.kind).toBe("OBJECT");
		expect(data.__schema.types[2].fields[2].type).toHaveProperty("name");
		expect(data.__schema.types[2].fields[2].type.name).toBe("User");
		expect(data.__schema.types[2].fields[2]).toHaveProperty("isDeprecated");
		expect(data.__schema.types[2].fields[2].isDeprecated).toBe(false);
		expect(data.__schema.types[2].fields[2]).toHaveProperty("deprecationReason");
		expect(data.__schema.types[2].fields[2].deprecationReason).toBe(null);
		expect(data.__schema.types[2].fields[3]).toHaveProperty("name");
		expect(data.__schema.types[2].fields[3].name).toBe("users");
		expect(data.__schema.types[2].fields[3]).toHaveProperty("description");
		expect(data.__schema.types[2].fields[3].description).toBe("Get all users with pagination");
		expect(data.__schema.types[2].fields[3]).toHaveProperty("args");
		expect(data.__schema.types[2].fields[3].args.length).toBe(2);
		expect(data.__schema.types[2].fields[3].args[0]).toHaveProperty("name");
		expect(data.__schema.types[2].fields[3].args[0].name).toBe("limit");
		expect(data.__schema.types[2].fields[3].args[0]).toHaveProperty("description");
		expect(data.__schema.types[2].fields[3].args[0].description).toBe("Maximum number of results");
		expect(data.__schema.types[2].fields[3].args[0]).toHaveProperty("type");
		expect(data.__schema.types[2].fields[3].args[0].type).toHaveProperty("kind");
		expect(data.__schema.types[2].fields[3].args[0].type.kind).toBe("SCALAR");
		expect(data.__schema.types[2].fields[3].args[0].type).toHaveProperty("name");
		expect(data.__schema.types[2].fields[3].args[0].type.name).toBe("Int");
		expect(data.__schema.types[2].fields[3].args[0]).toHaveProperty("defaultValue");
		expect(data.__schema.types[2].fields[3].args[0].defaultValue).toBe("10");
		expect(data.__schema.types[2].fields[3].args[1]).toHaveProperty("name");
		expect(data.__schema.types[2].fields[3].args[1].name).toBe("offset");
		expect(data.__schema.types[2].fields[3].args[1]).toHaveProperty("description");
		expect(data.__schema.types[2].fields[3].args[1].description).toBe("Number of results to skip");
		expect(data.__schema.types[2].fields[3].args[1]).toHaveProperty("type");
		expect(data.__schema.types[2].fields[3].args[1].type).toHaveProperty("kind");
		expect(data.__schema.types[2].fields[3].args[1].type.kind).toBe("SCALAR");
		expect(data.__schema.types[2].fields[3].args[1].type).toHaveProperty("name");
		expect(data.__schema.types[2].fields[3].args[1].type.name).toBe("Int");
		expect(data.__schema.types[2].fields[3].args[1]).toHaveProperty("defaultValue");
		expect(data.__schema.types[2].fields[3].args[1].defaultValue).toBe("0");
		expect(data.__schema.types[2].fields[3]).toHaveProperty("type");
		expect(data.__schema.types[2].fields[3].type).toHaveProperty("kind");
		expect(data.__schema.types[2].fields[3].type.kind).toBe("NON_NULL");
		expect(data.__schema.types[2].fields[3].type).toHaveProperty("name");
		expect(data.__schema.types[2].fields[3].type.name).toBe(null);
		expect(data.__schema.types[2].fields[3].type).toHaveProperty("ofType");
		expect(data.__schema.types[2].fields[3].type.ofType).toHaveProperty("kind");
		expect(data.__schema.types[2].fields[3].type.ofType.kind).toBe("LIST");
		expect(data.__schema.types[2].fields[3].type.ofType).toHaveProperty("name");
		expect(data.__schema.types[2].fields[3].type.ofType.name).toBe(null);
		expect(data.__schema.types[2].fields[3].type.ofType).toHaveProperty("ofType");
		expect(data.__schema.types[2].fields[3].type.ofType.ofType).toHaveProperty("kind");
		expect(data.__schema.types[2].fields[3].type.ofType.ofType.kind).toBe("NON_NULL");
		expect(data.__schema.types[2].fields[3].type.ofType.ofType).toHaveProperty("name");
		expect(data.__schema.types[2].fields[3].type.ofType.ofType.name).toBe(null);
		expect(data.__schema.types[2].fields[3].type.ofType.ofType).toHaveProperty("ofType");
		expect(data.__schema.types[2].fields[3].type.ofType.ofType.ofType).toHaveProperty("kind");
		expect(data.__schema.types[2].fields[3].type.ofType.ofType.ofType.kind).toBe("OBJECT");
		expect(data.__schema.types[2].fields[3].type.ofType.ofType.ofType).toHaveProperty("name");
		expect(data.__schema.types[2].fields[3].type.ofType.ofType.ofType.name).toBe("User");
		expect(data.__schema.types[2].fields[3]).toHaveProperty("isDeprecated");
		expect(data.__schema.types[2].fields[3].isDeprecated).toBe(false);
		expect(data.__schema.types[2].fields[3]).toHaveProperty("deprecationReason");
		expect(data.__schema.types[2].fields[3].deprecationReason).toBe(null);
		expect(data.__schema.types[2]).toHaveProperty("inputFields");
		expect(data.__schema.types[2].inputFields).toBe(null);
		expect(data.__schema.types[2]).toHaveProperty("interfaces");
		expect(data.__schema.types[2].interfaces.length).toBe(0);
		expect(data.__schema.types[2]).toHaveProperty("enumValues");
		expect(data.__schema.types[2].enumValues).toBe(null);
		expect(data.__schema.types[2]).toHaveProperty("possibleTypes");
		expect(data.__schema.types[2].possibleTypes).toBe(null);
		expect(data.__schema.types[3]).toHaveProperty("kind");
		expect(data.__schema.types[3].kind).toBe("OBJECT");
		expect(data.__schema.types[3]).toHaveProperty("name");
		expect(data.__schema.types[3].name).toBe("Mutation");
		expect(data.__schema.types[3]).toHaveProperty("description");
		expect(data.__schema.types[3].description).toBe("Root mutation type");
		expect(data.__schema.types[3]).toHaveProperty("fields");
		expect(data.__schema.types[3].fields.length).toBe(3);
		expect(data.__schema.types[3].fields[0]).toHaveProperty("name");
		expect(data.__schema.types[3].fields[0].name).toBe("createPost");
		expect(data.__schema.types[3].fields[0]).toHaveProperty("description");
		expect(data.__schema.types[3].fields[0].description).toBe("Create a new post");
		expect(data.__schema.types[3].fields[0]).toHaveProperty("args");
		expect(data.__schema.types[3].fields[0].args.length).toBe(1);
		expect(data.__schema.types[3].fields[0].args[0]).toHaveProperty("name");
		expect(data.__schema.types[3].fields[0].args[0].name).toBe("input");
		expect(data.__schema.types[3].fields[0].args[0]).toHaveProperty("description");
		expect(data.__schema.types[3].fields[0].args[0].description).toBe("Post creation input");
		expect(data.__schema.types[3].fields[0].args[0]).toHaveProperty("type");
		expect(data.__schema.types[3].fields[0].args[0].type).toHaveProperty("kind");
		expect(data.__schema.types[3].fields[0].args[0].type.kind).toBe("NON_NULL");
		expect(data.__schema.types[3].fields[0].args[0].type).toHaveProperty("name");
		expect(data.__schema.types[3].fields[0].args[0].type.name).toBe(null);
		expect(data.__schema.types[3].fields[0].args[0].type).toHaveProperty("ofType");
		expect(data.__schema.types[3].fields[0].args[0].type.ofType).toHaveProperty("kind");
		expect(data.__schema.types[3].fields[0].args[0].type.ofType.kind).toBe("INPUT_OBJECT");
		expect(data.__schema.types[3].fields[0].args[0].type.ofType).toHaveProperty("name");
		expect(data.__schema.types[3].fields[0].args[0].type.ofType.name).toBe("CreatePostInput");
		expect(data.__schema.types[3].fields[0].args[0]).toHaveProperty("defaultValue");
		expect(data.__schema.types[3].fields[0].args[0].defaultValue).toBe(null);
		expect(data.__schema.types[3].fields[0]).toHaveProperty("type");
		expect(data.__schema.types[3].fields[0].type).toHaveProperty("kind");
		expect(data.__schema.types[3].fields[0].type.kind).toBe("NON_NULL");
		expect(data.__schema.types[3].fields[0].type).toHaveProperty("name");
		expect(data.__schema.types[3].fields[0].type.name).toBe(null);
		expect(data.__schema.types[3].fields[0].type).toHaveProperty("ofType");
		expect(data.__schema.types[3].fields[0].type.ofType).toHaveProperty("kind");
		expect(data.__schema.types[3].fields[0].type.ofType.kind).toBe("OBJECT");
		expect(data.__schema.types[3].fields[0].type.ofType).toHaveProperty("name");
		expect(data.__schema.types[3].fields[0].type.ofType.name).toBe("Post");
		expect(data.__schema.types[3].fields[0]).toHaveProperty("isDeprecated");
		expect(data.__schema.types[3].fields[0].isDeprecated).toBe(false);
		expect(data.__schema.types[3].fields[0]).toHaveProperty("deprecationReason");
		expect(data.__schema.types[3].fields[0].deprecationReason).toBe(null);
		expect(data.__schema.types[3].fields[1]).toHaveProperty("name");
		expect(data.__schema.types[3].fields[1].name).toBe("updateUser");
		expect(data.__schema.types[3].fields[1]).toHaveProperty("description");
		expect(data.__schema.types[3].fields[1].description).toBe("Update user information");
		expect(data.__schema.types[3].fields[1]).toHaveProperty("args");
		expect(data.__schema.types[3].fields[1].args.length).toBe(2);
		expect(data.__schema.types[3].fields[1].args[0]).toHaveProperty("name");
		expect(data.__schema.types[3].fields[1].args[0].name).toBe("id");
		expect(data.__schema.types[3].fields[1].args[0]).toHaveProperty("description");
		expect(data.__schema.types[3].fields[1].args[0].description).toBe("User ID");
		expect(data.__schema.types[3].fields[1].args[0]).toHaveProperty("type");
		expect(data.__schema.types[3].fields[1].args[0].type).toHaveProperty("kind");
		expect(data.__schema.types[3].fields[1].args[0].type.kind).toBe("NON_NULL");
		expect(data.__schema.types[3].fields[1].args[0].type).toHaveProperty("name");
		expect(data.__schema.types[3].fields[1].args[0].type.name).toBe(null);
		expect(data.__schema.types[3].fields[1].args[0].type).toHaveProperty("ofType");
		expect(data.__schema.types[3].fields[1].args[0].type.ofType).toHaveProperty("kind");
		expect(data.__schema.types[3].fields[1].args[0].type.ofType.kind).toBe("SCALAR");
		expect(data.__schema.types[3].fields[1].args[0].type.ofType).toHaveProperty("name");
		expect(data.__schema.types[3].fields[1].args[0].type.ofType.name).toBe("UUID");
		expect(data.__schema.types[3].fields[1].args[0]).toHaveProperty("defaultValue");
		expect(data.__schema.types[3].fields[1].args[0].defaultValue).toBe(null);
		expect(data.__schema.types[3].fields[1].args[1]).toHaveProperty("name");
		expect(data.__schema.types[3].fields[1].args[1].name).toBe("name");
		expect(data.__schema.types[3].fields[1].args[1]).toHaveProperty("description");
		expect(data.__schema.types[3].fields[1].args[1].description).toBe("New user name");
		expect(data.__schema.types[3].fields[1].args[1]).toHaveProperty("type");
		expect(data.__schema.types[3].fields[1].args[1].type).toHaveProperty("kind");
		expect(data.__schema.types[3].fields[1].args[1].type.kind).toBe("NON_NULL");
		expect(data.__schema.types[3].fields[1].args[1].type).toHaveProperty("name");
		expect(data.__schema.types[3].fields[1].args[1].type.name).toBe(null);
		expect(data.__schema.types[3].fields[1].args[1].type).toHaveProperty("ofType");
		expect(data.__schema.types[3].fields[1].args[1].type.ofType).toHaveProperty("kind");
		expect(data.__schema.types[3].fields[1].args[1].type.ofType.kind).toBe("SCALAR");
		expect(data.__schema.types[3].fields[1].args[1].type.ofType).toHaveProperty("name");
		expect(data.__schema.types[3].fields[1].args[1].type.ofType.name).toBe("String");
		expect(data.__schema.types[3].fields[1].args[1]).toHaveProperty("defaultValue");
		expect(data.__schema.types[3].fields[1].args[1].defaultValue).toBe(null);
		expect(data.__schema.types[3].fields[1]).toHaveProperty("type");
		expect(data.__schema.types[3].fields[1].type).toHaveProperty("kind");
		expect(data.__schema.types[3].fields[1].type.kind).toBe("NON_NULL");
		expect(data.__schema.types[3].fields[1].type).toHaveProperty("name");
		expect(data.__schema.types[3].fields[1].type.name).toBe(null);
		expect(data.__schema.types[3].fields[1].type).toHaveProperty("ofType");
		expect(data.__schema.types[3].fields[1].type.ofType).toHaveProperty("kind");
		expect(data.__schema.types[3].fields[1].type.ofType.kind).toBe("OBJECT");
		expect(data.__schema.types[3].fields[1].type.ofType).toHaveProperty("name");
		expect(data.__schema.types[3].fields[1].type.ofType.name).toBe("User");
		expect(data.__schema.types[3].fields[1]).toHaveProperty("isDeprecated");
		expect(data.__schema.types[3].fields[1].isDeprecated).toBe(false);
		expect(data.__schema.types[3].fields[1]).toHaveProperty("deprecationReason");
		expect(data.__schema.types[3].fields[1].deprecationReason).toBe(null);
		expect(data.__schema.types[3].fields[2]).toHaveProperty("name");
		expect(data.__schema.types[3].fields[2].name).toBe("deletePost");
		expect(data.__schema.types[3].fields[2]).toHaveProperty("description");
		expect(data.__schema.types[3].fields[2].description).toBe("Delete a post");
		expect(data.__schema.types[3].fields[2]).toHaveProperty("args");
		expect(data.__schema.types[3].fields[2].args.length).toBe(1);
		expect(data.__schema.types[3].fields[2].args[0]).toHaveProperty("name");
		expect(data.__schema.types[3].fields[2].args[0].name).toBe("id");
		expect(data.__schema.types[3].fields[2].args[0]).toHaveProperty("description");
		expect(data.__schema.types[3].fields[2].args[0].description).toBe("Post ID");
		expect(data.__schema.types[3].fields[2].args[0]).toHaveProperty("type");
		expect(data.__schema.types[3].fields[2].args[0].type).toHaveProperty("kind");
		expect(data.__schema.types[3].fields[2].args[0].type.kind).toBe("NON_NULL");
		expect(data.__schema.types[3].fields[2].args[0].type).toHaveProperty("name");
		expect(data.__schema.types[3].fields[2].args[0].type.name).toBe(null);
		expect(data.__schema.types[3].fields[2].args[0].type).toHaveProperty("ofType");
		expect(data.__schema.types[3].fields[2].args[0].type.ofType).toHaveProperty("kind");
		expect(data.__schema.types[3].fields[2].args[0].type.ofType.kind).toBe("SCALAR");
		expect(data.__schema.types[3].fields[2].args[0].type.ofType).toHaveProperty("name");
		expect(data.__schema.types[3].fields[2].args[0].type.ofType.name).toBe("UUID");
		expect(data.__schema.types[3].fields[2].args[0]).toHaveProperty("defaultValue");
		expect(data.__schema.types[3].fields[2].args[0].defaultValue).toBe(null);
		expect(data.__schema.types[3].fields[2]).toHaveProperty("type");
		expect(data.__schema.types[3].fields[2].type).toHaveProperty("kind");
		expect(data.__schema.types[3].fields[2].type.kind).toBe("NON_NULL");
		expect(data.__schema.types[3].fields[2].type).toHaveProperty("name");
		expect(data.__schema.types[3].fields[2].type.name).toBe(null);
		expect(data.__schema.types[3].fields[2].type).toHaveProperty("ofType");
		expect(data.__schema.types[3].fields[2].type.ofType).toHaveProperty("kind");
		expect(data.__schema.types[3].fields[2].type.ofType.kind).toBe("SCALAR");
		expect(data.__schema.types[3].fields[2].type.ofType).toHaveProperty("name");
		expect(data.__schema.types[3].fields[2].type.ofType.name).toBe("Boolean");
		expect(data.__schema.types[3].fields[2]).toHaveProperty("isDeprecated");
		expect(data.__schema.types[3].fields[2].isDeprecated).toBe(false);
		expect(data.__schema.types[3].fields[2]).toHaveProperty("deprecationReason");
		expect(data.__schema.types[3].fields[2].deprecationReason).toBe(null);
		expect(data.__schema.types[3]).toHaveProperty("inputFields");
		expect(data.__schema.types[3].inputFields).toBe(null);
		expect(data.__schema.types[3]).toHaveProperty("interfaces");
		expect(data.__schema.types[3].interfaces.length).toBe(0);
		expect(data.__schema.types[3]).toHaveProperty("enumValues");
		expect(data.__schema.types[3].enumValues).toBe(null);
		expect(data.__schema.types[3]).toHaveProperty("possibleTypes");
		expect(data.__schema.types[3].possibleTypes).toBe(null);
		expect(data.__schema.types[4]).toHaveProperty("kind");
		expect(data.__schema.types[4].kind).toBe("OBJECT");
		expect(data.__schema.types[4]).toHaveProperty("name");
		expect(data.__schema.types[4].name).toBe("User");
		expect(data.__schema.types[4]).toHaveProperty("description");
		expect(data.__schema.types[4].description).toBe("User entity");
		expect(data.__schema.types[4]).toHaveProperty("fields");
		expect(data.__schema.types[4].fields.length).toBe(5);
		expect(data.__schema.types[4].fields[0]).toHaveProperty("name");
		expect(data.__schema.types[4].fields[0].name).toBe("id");
		expect(data.__schema.types[4].fields[0]).toHaveProperty("description");
		expect(data.__schema.types[4].fields[0].description).toBe("Unique identifier");
		expect(data.__schema.types[4].fields[0]).toHaveProperty("args");
		expect(data.__schema.types[4].fields[0].args.length).toBe(0);
		expect(data.__schema.types[4].fields[0]).toHaveProperty("type");
		expect(data.__schema.types[4].fields[0].type).toHaveProperty("kind");
		expect(data.__schema.types[4].fields[0].type.kind).toBe("NON_NULL");
		expect(data.__schema.types[4].fields[0].type).toHaveProperty("name");
		expect(data.__schema.types[4].fields[0].type.name).toBe(null);
		expect(data.__schema.types[4].fields[0].type).toHaveProperty("ofType");
		expect(data.__schema.types[4].fields[0].type.ofType).toHaveProperty("kind");
		expect(data.__schema.types[4].fields[0].type.ofType.kind).toBe("SCALAR");
		expect(data.__schema.types[4].fields[0].type.ofType).toHaveProperty("name");
		expect(data.__schema.types[4].fields[0].type.ofType.name).toBe("UUID");
		expect(data.__schema.types[4].fields[0]).toHaveProperty("isDeprecated");
		expect(data.__schema.types[4].fields[0].isDeprecated).toBe(false);
		expect(data.__schema.types[4].fields[0]).toHaveProperty("deprecationReason");
		expect(data.__schema.types[4].fields[0].deprecationReason).toBe(null);
		expect(data.__schema.types[4].fields[1]).toHaveProperty("name");
		expect(data.__schema.types[4].fields[1].name).toBe("name");
		expect(data.__schema.types[4].fields[1]).toHaveProperty("description");
		expect(data.__schema.types[4].fields[1].description).toBe("User's full name");
		expect(data.__schema.types[4].fields[1]).toHaveProperty("args");
		expect(data.__schema.types[4].fields[1].args.length).toBe(0);
		expect(data.__schema.types[4].fields[1]).toHaveProperty("type");
		expect(data.__schema.types[4].fields[1].type).toHaveProperty("kind");
		expect(data.__schema.types[4].fields[1].type.kind).toBe("NON_NULL");
		expect(data.__schema.types[4].fields[1].type).toHaveProperty("name");
		expect(data.__schema.types[4].fields[1].type.name).toBe(null);
		expect(data.__schema.types[4].fields[1].type).toHaveProperty("ofType");
		expect(data.__schema.types[4].fields[1].type.ofType).toHaveProperty("kind");
		expect(data.__schema.types[4].fields[1].type.ofType.kind).toBe("SCALAR");
		expect(data.__schema.types[4].fields[1].type.ofType).toHaveProperty("name");
		expect(data.__schema.types[4].fields[1].type.ofType.name).toBe("String");
		expect(data.__schema.types[4].fields[1]).toHaveProperty("isDeprecated");
		expect(data.__schema.types[4].fields[1].isDeprecated).toBe(false);
		expect(data.__schema.types[4].fields[1]).toHaveProperty("deprecationReason");
		expect(data.__schema.types[4].fields[1].deprecationReason).toBe(null);
		expect(data.__schema.types[4].fields[2]).toHaveProperty("name");
		expect(data.__schema.types[4].fields[2].name).toBe("email");
		expect(data.__schema.types[4].fields[2]).toHaveProperty("description");
		expect(data.__schema.types[4].fields[2].description).toBe("User's email address");
		expect(data.__schema.types[4].fields[2]).toHaveProperty("args");
		expect(data.__schema.types[4].fields[2].args.length).toBe(0);
		expect(data.__schema.types[4].fields[2]).toHaveProperty("type");
		expect(data.__schema.types[4].fields[2].type).toHaveProperty("kind");
		expect(data.__schema.types[4].fields[2].type.kind).toBe("NON_NULL");
		expect(data.__schema.types[4].fields[2].type).toHaveProperty("name");
		expect(data.__schema.types[4].fields[2].type.name).toBe(null);
		expect(data.__schema.types[4].fields[2].type).toHaveProperty("ofType");
		expect(data.__schema.types[4].fields[2].type.ofType).toHaveProperty("kind");
		expect(data.__schema.types[4].fields[2].type.ofType.kind).toBe("SCALAR");
		expect(data.__schema.types[4].fields[2].type.ofType).toHaveProperty("name");
		expect(data.__schema.types[4].fields[2].type.ofType.name).toBe("String");
		expect(data.__schema.types[4].fields[2]).toHaveProperty("isDeprecated");
		expect(data.__schema.types[4].fields[2].isDeprecated).toBe(false);
		expect(data.__schema.types[4].fields[2]).toHaveProperty("deprecationReason");
		expect(data.__schema.types[4].fields[2].deprecationReason).toBe(null);
		expect(data.__schema.types[4].fields[3]).toHaveProperty("name");
		expect(data.__schema.types[4].fields[3].name).toBe("createdAt");
		expect(data.__schema.types[4].fields[3]).toHaveProperty("description");
		expect(data.__schema.types[4].fields[3].description).toBe("Creation timestamp");
		expect(data.__schema.types[4].fields[3]).toHaveProperty("args");
		expect(data.__schema.types[4].fields[3].args.length).toBe(0);
		expect(data.__schema.types[4].fields[3]).toHaveProperty("type");
		expect(data.__schema.types[4].fields[3].type).toHaveProperty("kind");
		expect(data.__schema.types[4].fields[3].type.kind).toBe("NON_NULL");
		expect(data.__schema.types[4].fields[3].type).toHaveProperty("name");
		expect(data.__schema.types[4].fields[3].type.name).toBe(null);
		expect(data.__schema.types[4].fields[3].type).toHaveProperty("ofType");
		expect(data.__schema.types[4].fields[3].type.ofType).toHaveProperty("kind");
		expect(data.__schema.types[4].fields[3].type.ofType.kind).toBe("SCALAR");
		expect(data.__schema.types[4].fields[3].type.ofType).toHaveProperty("name");
		expect(data.__schema.types[4].fields[3].type.ofType.name).toBe("DateTime");
		expect(data.__schema.types[4].fields[3]).toHaveProperty("isDeprecated");
		expect(data.__schema.types[4].fields[3].isDeprecated).toBe(false);
		expect(data.__schema.types[4].fields[3]).toHaveProperty("deprecationReason");
		expect(data.__schema.types[4].fields[3].deprecationReason).toBe(null);
		expect(data.__schema.types[4].fields[4]).toHaveProperty("name");
		expect(data.__schema.types[4].fields[4].name).toBe("posts");
		expect(data.__schema.types[4].fields[4]).toHaveProperty("description");
		expect(data.__schema.types[4].fields[4].description).toBe("User's posts");
		expect(data.__schema.types[4].fields[4]).toHaveProperty("args");
		expect(data.__schema.types[4].fields[4].args.length).toBe(0);
		expect(data.__schema.types[4].fields[4]).toHaveProperty("type");
		expect(data.__schema.types[4].fields[4].type).toHaveProperty("kind");
		expect(data.__schema.types[4].fields[4].type.kind).toBe("NON_NULL");
		expect(data.__schema.types[4].fields[4].type).toHaveProperty("name");
		expect(data.__schema.types[4].fields[4].type.name).toBe(null);
		expect(data.__schema.types[4].fields[4].type).toHaveProperty("ofType");
		expect(data.__schema.types[4].fields[4].type.ofType).toHaveProperty("kind");
		expect(data.__schema.types[4].fields[4].type.ofType.kind).toBe("LIST");
		expect(data.__schema.types[4].fields[4].type.ofType).toHaveProperty("name");
		expect(data.__schema.types[4].fields[4].type.ofType.name).toBe(null);
		expect(data.__schema.types[4].fields[4].type.ofType).toHaveProperty("ofType");
		expect(data.__schema.types[4].fields[4].type.ofType.ofType).toHaveProperty("kind");
		expect(data.__schema.types[4].fields[4].type.ofType.ofType.kind).toBe("NON_NULL");
		expect(data.__schema.types[4].fields[4].type.ofType.ofType).toHaveProperty("name");
		expect(data.__schema.types[4].fields[4].type.ofType.ofType.name).toBe(null);
		expect(data.__schema.types[4].fields[4].type.ofType.ofType).toHaveProperty("ofType");
		expect(data.__schema.types[4].fields[4].type.ofType.ofType.ofType).toHaveProperty("kind");
		expect(data.__schema.types[4].fields[4].type.ofType.ofType.ofType.kind).toBe("OBJECT");
		expect(data.__schema.types[4].fields[4].type.ofType.ofType.ofType).toHaveProperty("name");
		expect(data.__schema.types[4].fields[4].type.ofType.ofType.ofType.name).toBe("Post");
		expect(data.__schema.types[4].fields[4]).toHaveProperty("isDeprecated");
		expect(data.__schema.types[4].fields[4].isDeprecated).toBe(false);
		expect(data.__schema.types[4].fields[4]).toHaveProperty("deprecationReason");
		expect(data.__schema.types[4].fields[4].deprecationReason).toBe(null);
		expect(data.__schema.types[4]).toHaveProperty("inputFields");
		expect(data.__schema.types[4].inputFields).toBe(null);
		expect(data.__schema.types[4]).toHaveProperty("interfaces");
		expect(data.__schema.types[4].interfaces.length).toBe(0);
		expect(data.__schema.types[4]).toHaveProperty("enumValues");
		expect(data.__schema.types[4].enumValues).toBe(null);
		expect(data.__schema.types[4]).toHaveProperty("possibleTypes");
		expect(data.__schema.types[4].possibleTypes).toBe(null);
		expect(data.__schema.types[5]).toHaveProperty("kind");
		expect(data.__schema.types[5].kind).toBe("OBJECT");
		expect(data.__schema.types[5]).toHaveProperty("name");
		expect(data.__schema.types[5].name).toBe("Post");
		expect(data.__schema.types[5]).toHaveProperty("description");
		expect(data.__schema.types[5].description).toBe("Blog post entity");
		expect(data.__schema.types[5]).toHaveProperty("fields");
		expect(data.__schema.types[5].fields.length).toBe(7);
		expect(data.__schema.types[5].fields[0]).toHaveProperty("name");
		expect(data.__schema.types[5].fields[0].name).toBe("id");
		expect(data.__schema.types[5].fields[0]).toHaveProperty("description");
		expect(data.__schema.types[5].fields[0].description).toBe("Unique identifier");
		expect(data.__schema.types[5].fields[0]).toHaveProperty("args");
		expect(data.__schema.types[5].fields[0].args.length).toBe(0);
		expect(data.__schema.types[5].fields[0]).toHaveProperty("type");
		expect(data.__schema.types[5].fields[0].type).toHaveProperty("kind");
		expect(data.__schema.types[5].fields[0].type.kind).toBe("NON_NULL");
		expect(data.__schema.types[5].fields[0].type).toHaveProperty("name");
		expect(data.__schema.types[5].fields[0].type.name).toBe(null);
		expect(data.__schema.types[5].fields[0].type).toHaveProperty("ofType");
		expect(data.__schema.types[5].fields[0].type.ofType).toHaveProperty("kind");
		expect(data.__schema.types[5].fields[0].type.ofType.kind).toBe("SCALAR");
		expect(data.__schema.types[5].fields[0].type.ofType).toHaveProperty("name");
		expect(data.__schema.types[5].fields[0].type.ofType.name).toBe("UUID");
		expect(data.__schema.types[5].fields[0]).toHaveProperty("isDeprecated");
		expect(data.__schema.types[5].fields[0].isDeprecated).toBe(false);
		expect(data.__schema.types[5].fields[0]).toHaveProperty("deprecationReason");
		expect(data.__schema.types[5].fields[0].deprecationReason).toBe(null);
		expect(data.__schema.types[5].fields[1]).toHaveProperty("name");
		expect(data.__schema.types[5].fields[1].name).toBe("title");
		expect(data.__schema.types[5].fields[1]).toHaveProperty("description");
		expect(data.__schema.types[5].fields[1].description).toBe("Post title");
		expect(data.__schema.types[5].fields[1]).toHaveProperty("args");
		expect(data.__schema.types[5].fields[1].args.length).toBe(0);
		expect(data.__schema.types[5].fields[1]).toHaveProperty("type");
		expect(data.__schema.types[5].fields[1].type).toHaveProperty("kind");
		expect(data.__schema.types[5].fields[1].type.kind).toBe("NON_NULL");
		expect(data.__schema.types[5].fields[1].type).toHaveProperty("name");
		expect(data.__schema.types[5].fields[1].type.name).toBe(null);
		expect(data.__schema.types[5].fields[1].type).toHaveProperty("ofType");
		expect(data.__schema.types[5].fields[1].type.ofType).toHaveProperty("kind");
		expect(data.__schema.types[5].fields[1].type.ofType.kind).toBe("SCALAR");
		expect(data.__schema.types[5].fields[1].type.ofType).toHaveProperty("name");
		expect(data.__schema.types[5].fields[1].type.ofType.name).toBe("String");
		expect(data.__schema.types[5].fields[1]).toHaveProperty("isDeprecated");
		expect(data.__schema.types[5].fields[1].isDeprecated).toBe(false);
		expect(data.__schema.types[5].fields[1]).toHaveProperty("deprecationReason");
		expect(data.__schema.types[5].fields[1].deprecationReason).toBe(null);
		expect(data.__schema.types[5].fields[2]).toHaveProperty("name");
		expect(data.__schema.types[5].fields[2].name).toBe("content");
		expect(data.__schema.types[5].fields[2]).toHaveProperty("description");
		expect(data.__schema.types[5].fields[2].description).toBe("Post content");
		expect(data.__schema.types[5].fields[2]).toHaveProperty("args");
		expect(data.__schema.types[5].fields[2].args.length).toBe(0);
		expect(data.__schema.types[5].fields[2]).toHaveProperty("type");
		expect(data.__schema.types[5].fields[2].type).toHaveProperty("kind");
		expect(data.__schema.types[5].fields[2].type.kind).toBe("NON_NULL");
		expect(data.__schema.types[5].fields[2].type).toHaveProperty("name");
		expect(data.__schema.types[5].fields[2].type.name).toBe(null);
		expect(data.__schema.types[5].fields[2].type).toHaveProperty("ofType");
		expect(data.__schema.types[5].fields[2].type.ofType).toHaveProperty("kind");
		expect(data.__schema.types[5].fields[2].type.ofType.kind).toBe("SCALAR");
		expect(data.__schema.types[5].fields[2].type.ofType).toHaveProperty("name");
		expect(data.__schema.types[5].fields[2].type.ofType.name).toBe("String");
		expect(data.__schema.types[5].fields[2]).toHaveProperty("isDeprecated");
		expect(data.__schema.types[5].fields[2].isDeprecated).toBe(false);
		expect(data.__schema.types[5].fields[2]).toHaveProperty("deprecationReason");
		expect(data.__schema.types[5].fields[2].deprecationReason).toBe(null);
		expect(data.__schema.types[5].fields[3]).toHaveProperty("name");
		expect(data.__schema.types[5].fields[3].name).toBe("authorId");
		expect(data.__schema.types[5].fields[3]).toHaveProperty("description");
		expect(data.__schema.types[5].fields[3].description).toBe("Author's ID");
		expect(data.__schema.types[5].fields[3]).toHaveProperty("args");
		expect(data.__schema.types[5].fields[3].args.length).toBe(0);
		expect(data.__schema.types[5].fields[3]).toHaveProperty("type");
		expect(data.__schema.types[5].fields[3].type).toHaveProperty("kind");
		expect(data.__schema.types[5].fields[3].type.kind).toBe("NON_NULL");
		expect(data.__schema.types[5].fields[3].type).toHaveProperty("name");
		expect(data.__schema.types[5].fields[3].type.name).toBe(null);
		expect(data.__schema.types[5].fields[3].type).toHaveProperty("ofType");
		expect(data.__schema.types[5].fields[3].type.ofType).toHaveProperty("kind");
		expect(data.__schema.types[5].fields[3].type.ofType.kind).toBe("SCALAR");
		expect(data.__schema.types[5].fields[3].type.ofType).toHaveProperty("name");
		expect(data.__schema.types[5].fields[3].type.ofType.name).toBe("UUID");
		expect(data.__schema.types[5].fields[3]).toHaveProperty("isDeprecated");
		expect(data.__schema.types[5].fields[3].isDeprecated).toBe(false);
		expect(data.__schema.types[5].fields[3]).toHaveProperty("deprecationReason");
		expect(data.__schema.types[5].fields[3].deprecationReason).toBe(null);
		expect(data.__schema.types[5].fields[4]).toHaveProperty("name");
		expect(data.__schema.types[5].fields[4].name).toBe("author");
		expect(data.__schema.types[5].fields[4]).toHaveProperty("description");
		expect(data.__schema.types[5].fields[4].description).toBe("Post author");
		expect(data.__schema.types[5].fields[4]).toHaveProperty("args");
		expect(data.__schema.types[5].fields[4].args.length).toBe(0);
		expect(data.__schema.types[5].fields[4]).toHaveProperty("type");
		expect(data.__schema.types[5].fields[4].type).toHaveProperty("kind");
		expect(data.__schema.types[5].fields[4].type.kind).toBe("NON_NULL");
		expect(data.__schema.types[5].fields[4].type).toHaveProperty("name");
		expect(data.__schema.types[5].fields[4].type.name).toBe(null);
		expect(data.__schema.types[5].fields[4].type).toHaveProperty("ofType");
		expect(data.__schema.types[5].fields[4].type.ofType).toHaveProperty("kind");
		expect(data.__schema.types[5].fields[4].type.ofType.kind).toBe("OBJECT");
		expect(data.__schema.types[5].fields[4].type.ofType).toHaveProperty("name");
		expect(data.__schema.types[5].fields[4].type.ofType.name).toBe("User");
		expect(data.__schema.types[5].fields[4]).toHaveProperty("isDeprecated");
		expect(data.__schema.types[5].fields[4].isDeprecated).toBe(false);
		expect(data.__schema.types[5].fields[4]).toHaveProperty("deprecationReason");
		expect(data.__schema.types[5].fields[4].deprecationReason).toBe(null);
		expect(data.__schema.types[5].fields[5]).toHaveProperty("name");
		expect(data.__schema.types[5].fields[5].name).toBe("createdAt");
		expect(data.__schema.types[5].fields[5]).toHaveProperty("description");
		expect(data.__schema.types[5].fields[5].description).toBe("Creation timestamp");
		expect(data.__schema.types[5].fields[5]).toHaveProperty("args");
		expect(data.__schema.types[5].fields[5].args.length).toBe(0);
		expect(data.__schema.types[5].fields[5]).toHaveProperty("type");
		expect(data.__schema.types[5].fields[5].type).toHaveProperty("kind");
		expect(data.__schema.types[5].fields[5].type.kind).toBe("NON_NULL");
		expect(data.__schema.types[5].fields[5].type).toHaveProperty("name");
		expect(data.__schema.types[5].fields[5].type.name).toBe(null);
		expect(data.__schema.types[5].fields[5].type).toHaveProperty("ofType");
		expect(data.__schema.types[5].fields[5].type.ofType).toHaveProperty("kind");
		expect(data.__schema.types[5].fields[5].type.ofType.kind).toBe("SCALAR");
		expect(data.__schema.types[5].fields[5].type.ofType).toHaveProperty("name");
		expect(data.__schema.types[5].fields[5].type.ofType.name).toBe("DateTime");
		expect(data.__schema.types[5].fields[5]).toHaveProperty("isDeprecated");
		expect(data.__schema.types[5].fields[5].isDeprecated).toBe(false);
		expect(data.__schema.types[5].fields[5]).toHaveProperty("deprecationReason");
		expect(data.__schema.types[5].fields[5].deprecationReason).toBe(null);
		expect(data.__schema.types[5].fields[6]).toHaveProperty("name");
		expect(data.__schema.types[5].fields[6].name).toBe("updatedAt");
		expect(data.__schema.types[5].fields[6]).toHaveProperty("description");
		expect(data.__schema.types[5].fields[6].description).toBe("Last update timestamp");
		expect(data.__schema.types[5].fields[6]).toHaveProperty("args");
		expect(data.__schema.types[5].fields[6].args.length).toBe(0);
		expect(data.__schema.types[5].fields[6]).toHaveProperty("type");
		expect(data.__schema.types[5].fields[6].type).toHaveProperty("kind");
		expect(data.__schema.types[5].fields[6].type.kind).toBe("NON_NULL");
		expect(data.__schema.types[5].fields[6].type).toHaveProperty("name");
		expect(data.__schema.types[5].fields[6].type.name).toBe(null);
		expect(data.__schema.types[5].fields[6].type).toHaveProperty("ofType");
		expect(data.__schema.types[5].fields[6].type.ofType).toHaveProperty("kind");
		expect(data.__schema.types[5].fields[6].type.ofType.kind).toBe("SCALAR");
		expect(data.__schema.types[5].fields[6].type.ofType).toHaveProperty("name");
		expect(data.__schema.types[5].fields[6].type.ofType.name).toBe("DateTime");
		expect(data.__schema.types[5].fields[6]).toHaveProperty("isDeprecated");
		expect(data.__schema.types[5].fields[6].isDeprecated).toBe(false);
		expect(data.__schema.types[5].fields[6]).toHaveProperty("deprecationReason");
		expect(data.__schema.types[5].fields[6].deprecationReason).toBe(null);
		expect(data.__schema.types[5]).toHaveProperty("inputFields");
		expect(data.__schema.types[5].inputFields).toBe(null);
		expect(data.__schema.types[5]).toHaveProperty("interfaces");
		expect(data.__schema.types[5].interfaces.length).toBe(0);
		expect(data.__schema.types[5]).toHaveProperty("enumValues");
		expect(data.__schema.types[5].enumValues).toBe(null);
		expect(data.__schema.types[5]).toHaveProperty("possibleTypes");
		expect(data.__schema.types[5].possibleTypes).toBe(null);
		expect(data.__schema.types[6]).toHaveProperty("kind");
		expect(data.__schema.types[6].kind).toBe("INPUT_OBJECT");
		expect(data.__schema.types[6]).toHaveProperty("name");
		expect(data.__schema.types[6].name).toBe("CreatePostInput");
		expect(data.__schema.types[6]).toHaveProperty("description");
		expect(data.__schema.types[6].description).toBe("Input for creating posts");
		expect(data.__schema.types[6]).toHaveProperty("fields");
		expect(data.__schema.types[6].fields).toBe(null);
		expect(data.__schema.types[6]).toHaveProperty("inputFields");
		expect(data.__schema.types[6].inputFields.length).toBe(3);
		expect(data.__schema.types[6].inputFields[0]).toHaveProperty("name");
		expect(data.__schema.types[6].inputFields[0].name).toBe("title");
		expect(data.__schema.types[6].inputFields[0]).toHaveProperty("description");
		expect(data.__schema.types[6].inputFields[0].description).toBe("Post title");
		expect(data.__schema.types[6].inputFields[0]).toHaveProperty("type");
		expect(data.__schema.types[6].inputFields[0].type).toHaveProperty("kind");
		expect(data.__schema.types[6].inputFields[0].type.kind).toBe("NON_NULL");
		expect(data.__schema.types[6].inputFields[0].type).toHaveProperty("name");
		expect(data.__schema.types[6].inputFields[0].type.name).toBe(null);
		expect(data.__schema.types[6].inputFields[0].type).toHaveProperty("ofType");
		expect(data.__schema.types[6].inputFields[0].type.ofType).toHaveProperty("kind");
		expect(data.__schema.types[6].inputFields[0].type.ofType.kind).toBe("SCALAR");
		expect(data.__schema.types[6].inputFields[0].type.ofType).toHaveProperty("name");
		expect(data.__schema.types[6].inputFields[0].type.ofType.name).toBe("String");
		expect(data.__schema.types[6].inputFields[0]).toHaveProperty("defaultValue");
		expect(data.__schema.types[6].inputFields[0].defaultValue).toBe(null);
		expect(data.__schema.types[6].inputFields[1]).toHaveProperty("name");
		expect(data.__schema.types[6].inputFields[1].name).toBe("content");
		expect(data.__schema.types[6].inputFields[1]).toHaveProperty("description");
		expect(data.__schema.types[6].inputFields[1].description).toBe("Post content");
		expect(data.__schema.types[6].inputFields[1]).toHaveProperty("type");
		expect(data.__schema.types[6].inputFields[1].type).toHaveProperty("kind");
		expect(data.__schema.types[6].inputFields[1].type.kind).toBe("NON_NULL");
		expect(data.__schema.types[6].inputFields[1].type).toHaveProperty("name");
		expect(data.__schema.types[6].inputFields[1].type.name).toBe(null);
		expect(data.__schema.types[6].inputFields[1].type).toHaveProperty("ofType");
		expect(data.__schema.types[6].inputFields[1].type.ofType).toHaveProperty("kind");
		expect(data.__schema.types[6].inputFields[1].type.ofType.kind).toBe("SCALAR");
		expect(data.__schema.types[6].inputFields[1].type.ofType).toHaveProperty("name");
		expect(data.__schema.types[6].inputFields[1].type.ofType.name).toBe("String");
		expect(data.__schema.types[6].inputFields[1]).toHaveProperty("defaultValue");
		expect(data.__schema.types[6].inputFields[1].defaultValue).toBe(null);
		expect(data.__schema.types[6].inputFields[2]).toHaveProperty("name");
		expect(data.__schema.types[6].inputFields[2].name).toBe("authorId");
		expect(data.__schema.types[6].inputFields[2]).toHaveProperty("description");
		expect(data.__schema.types[6].inputFields[2].description).toBe("Author ID");
		expect(data.__schema.types[6].inputFields[2]).toHaveProperty("type");
		expect(data.__schema.types[6].inputFields[2].type).toHaveProperty("kind");
		expect(data.__schema.types[6].inputFields[2].type.kind).toBe("NON_NULL");
		expect(data.__schema.types[6].inputFields[2].type).toHaveProperty("name");
		expect(data.__schema.types[6].inputFields[2].type.name).toBe(null);
		expect(data.__schema.types[6].inputFields[2].type).toHaveProperty("ofType");
		expect(data.__schema.types[6].inputFields[2].type.ofType).toHaveProperty("kind");
		expect(data.__schema.types[6].inputFields[2].type.ofType.kind).toBe("SCALAR");
		expect(data.__schema.types[6].inputFields[2].type.ofType).toHaveProperty("name");
		expect(data.__schema.types[6].inputFields[2].type.ofType.name).toBe("UUID");
		expect(data.__schema.types[6].inputFields[2]).toHaveProperty("defaultValue");
		expect(data.__schema.types[6].inputFields[2].defaultValue).toBe(null);
		expect(data.__schema.types[6]).toHaveProperty("interfaces");
		expect(data.__schema.types[6].interfaces).toBe(null);
		expect(data.__schema.types[6]).toHaveProperty("enumValues");
		expect(data.__schema.types[6].enumValues).toBe(null);
		expect(data.__schema.types[6]).toHaveProperty("possibleTypes");
		expect(data.__schema.types[6].possibleTypes).toBe(null);
		expect(data.__schema).toHaveProperty("directives");
		expect(data.__schema.directives.length).toBe(3);
		expect(data.__schema.directives[0]).toHaveProperty("name");
		expect(data.__schema.directives[0].name).toBe("skip");
		expect(data.__schema.directives[0]).toHaveProperty("description");
		expect(data.__schema.directives[0].description).toBe(
			"Directs the executor to skip this field or fragment when the `if` argument is true.",
		);
		expect(data.__schema.directives[0]).toHaveProperty("locations");
		expect(data.__schema.directives[0].locations.length).toBe(3);
		expect(data.__schema.directives[0].locations[0]).toBe("FIELD");
		expect(data.__schema.directives[0].locations[1]).toBe("FRAGMENT_SPREAD");
		expect(data.__schema.directives[0].locations[2]).toBe("INLINE_FRAGMENT");
		expect(data.__schema.directives[0]).toHaveProperty("args");
		expect(data.__schema.directives[0].args.length).toBe(1);
		expect(data.__schema.directives[0].args[0]).toHaveProperty("name");
		expect(data.__schema.directives[0].args[0].name).toBe("if");
		expect(data.__schema.directives[0].args[0]).toHaveProperty("description");
		expect(data.__schema.directives[0].args[0].description).toBe("Skipped when true");
		expect(data.__schema.directives[0].args[0]).toHaveProperty("type");
		expect(data.__schema.directives[0].args[0].type).toHaveProperty("kind");
		expect(data.__schema.directives[0].args[0].type.kind).toBe("NON_NULL");
		expect(data.__schema.directives[0].args[0].type).toHaveProperty("name");
		expect(data.__schema.directives[0].args[0].type.name).toBe(null);
		expect(data.__schema.directives[0].args[0].type).toHaveProperty("ofType");
		expect(data.__schema.directives[0].args[0].type.ofType).toHaveProperty("kind");
		expect(data.__schema.directives[0].args[0].type.ofType.kind).toBe("SCALAR");
		expect(data.__schema.directives[0].args[0].type.ofType).toHaveProperty("name");
		expect(data.__schema.directives[0].args[0].type.ofType.name).toBe("Boolean");
		expect(data.__schema.directives[0].args[0]).toHaveProperty("defaultValue");
		expect(data.__schema.directives[0].args[0].defaultValue).toBe(null);
		expect(data.__schema.directives[1]).toHaveProperty("name");
		expect(data.__schema.directives[1].name).toBe("include");
		expect(data.__schema.directives[1]).toHaveProperty("description");
		expect(data.__schema.directives[1].description).toBe(
			"Directs the executor to include this field or fragment when the `if` argument is true.",
		);
		expect(data.__schema.directives[1]).toHaveProperty("locations");
		expect(data.__schema.directives[1].locations.length).toBe(3);
		expect(data.__schema.directives[1].locations[0]).toBe("FIELD");
		expect(data.__schema.directives[1].locations[1]).toBe("FRAGMENT_SPREAD");
		expect(data.__schema.directives[1].locations[2]).toBe("INLINE_FRAGMENT");
		expect(data.__schema.directives[1]).toHaveProperty("args");
		expect(data.__schema.directives[1].args.length).toBe(1);
		expect(data.__schema.directives[1].args[0]).toHaveProperty("name");
		expect(data.__schema.directives[1].args[0].name).toBe("if");
		expect(data.__schema.directives[1].args[0]).toHaveProperty("description");
		expect(data.__schema.directives[1].args[0].description).toBe("Included when true");
		expect(data.__schema.directives[1].args[0]).toHaveProperty("type");
		expect(data.__schema.directives[1].args[0].type).toHaveProperty("kind");
		expect(data.__schema.directives[1].args[0].type.kind).toBe("NON_NULL");
		expect(data.__schema.directives[1].args[0].type).toHaveProperty("name");
		expect(data.__schema.directives[1].args[0].type.name).toBe(null);
		expect(data.__schema.directives[1].args[0].type).toHaveProperty("ofType");
		expect(data.__schema.directives[1].args[0].type.ofType).toHaveProperty("kind");
		expect(data.__schema.directives[1].args[0].type.ofType.kind).toBe("SCALAR");
		expect(data.__schema.directives[1].args[0].type.ofType).toHaveProperty("name");
		expect(data.__schema.directives[1].args[0].type.ofType.name).toBe("Boolean");
		expect(data.__schema.directives[1].args[0]).toHaveProperty("defaultValue");
		expect(data.__schema.directives[1].args[0].defaultValue).toBe(null);
		expect(data.__schema.directives[2]).toHaveProperty("name");
		expect(data.__schema.directives[2].name).toBe("deprecated");
		expect(data.__schema.directives[2]).toHaveProperty("description");
		expect(data.__schema.directives[2].description).toBe(
			"Marks an element of a GraphQL schema as no longer supported.",
		);
		expect(data.__schema.directives[2]).toHaveProperty("locations");
		expect(data.__schema.directives[2].locations.length).toBe(2);
		expect(data.__schema.directives[2].locations[0]).toBe("FIELD_DEFINITION");
		expect(data.__schema.directives[2].locations[1]).toBe("ENUM_VALUE");
		expect(data.__schema.directives[2]).toHaveProperty("args");
		expect(data.__schema.directives[2].args.length).toBe(1);
		expect(data.__schema.directives[2].args[0]).toHaveProperty("name");
		expect(data.__schema.directives[2].args[0].name).toBe("reason");
		expect(data.__schema.directives[2].args[0]).toHaveProperty("description");
		expect(data.__schema.directives[2].args[0].description).toBe("Explains why this element was deprecated");
		expect(data.__schema.directives[2].args[0]).toHaveProperty("type");
		expect(data.__schema.directives[2].args[0].type).toHaveProperty("kind");
		expect(data.__schema.directives[2].args[0].type.kind).toBe("SCALAR");
		expect(data.__schema.directives[2].args[0].type).toHaveProperty("name");
		expect(data.__schema.directives[2].args[0].type.name).toBe("String");
		expect(data.__schema.directives[2].args[0]).toHaveProperty("defaultValue");
		expect(data.__schema.directives[2].args[0].defaultValue).toBe("No longer supported");
		const errors = responseBody.errors;
		expect(errors?.length ?? 0).toBe(0);
	});

	test("entity_with_key", async () => {
		const app = createAppGraphqlQuery();
		const client = new TestClient(app);

		const response = await client.post("/graphql", {
			headers: { "x-spikard-fixture": "entity_with_key" },
			json: {
				query: `query {\n  _entities(representations: [{__typename: "User", id: "42"}]) {\n    ... on User {\n      id\n      name\n      username\n      profile {\n        bio\n        avatar\n        joinDate\n      }\n    }\n  }\n}`,
				variables: null,
				operationName: null,
			},
		});

		expect(response.statusCode).toBe(200);
		const responseBody = response.json();

		const data = responseBody.data;
		expect(data).toHaveProperty("_entities");
		expect(data._entities.length).toBe(1);
		expect(data._entities[0]).toHaveProperty("id");
		expect(data._entities[0].id).toBe("42");
		expect(data._entities[0]).toHaveProperty("name");
		expect(data._entities[0].name).toBe("Bob Smith");
		expect(data._entities[0]).toHaveProperty("username");
		expect(data._entities[0].username).toBe("bobsmith");
		expect(data._entities[0]).toHaveProperty("profile");
		expect(data._entities[0].profile).toHaveProperty("bio");
		expect(data._entities[0].profile.bio).toBe("Software engineer and open source enthusiast");
		expect(data._entities[0].profile).toHaveProperty("avatar");
		expect(data._entities[0].profile.avatar).toBe("https://example.com/avatars/bob.jpg");
		expect(data._entities[0].profile).toHaveProperty("joinDate");
		expect(data._entities[0].profile.joinDate).toBe("2020-03-15");
		const errors = responseBody.errors;
		expect(errors?.length ?? 0).toBe(0);
	});

	test("requires_directive", async () => {
		const app = createAppGraphqlQuery();
		const client = new TestClient(app);

		const response = await client.post("/graphql", {
			headers: { "x-spikard-fixture": "requires_directive" },
			json: {
				query: `query {\n  _entities(representations: [{__typename: "Shipment", id: "ship-001", weight: 5.5, destination: "NYC"}]) {\n    ... on Shipment {\n      id\n      weight\n      destination\n      shippingEstimate\n    }\n  }\n}`,
				variables: null,
				operationName: null,
			},
		});

		expect(response.statusCode).toBe(200);
		const responseBody = response.json();

		const data = responseBody.data;
		expect(data).toHaveProperty("_entities");
		expect(data._entities.length).toBe(1);
		expect(data._entities[0]).toHaveProperty("id");
		expect(data._entities[0].id).toBe("ship-001");
		expect(data._entities[0]).toHaveProperty("weight");
		expect(data._entities[0].weight).toBe(5.5);
		expect(data._entities[0]).toHaveProperty("destination");
		expect(data._entities[0].destination).toBe("NYC");
		expect(data._entities[0]).toHaveProperty("shippingEstimate");
		expect(data._entities[0].shippingEstimate).toBe(24.75);
		const errors = responseBody.errors;
		expect(errors?.length ?? 0).toBe(0);
	});

	test("cross_subgraph_query", async () => {
		const app = createAppGraphqlQuery();
		const client = new TestClient(app);

		const response = await client.post("/graphql", {
			headers: { "x-spikard-fixture": "cross_subgraph_query" },
			json: {
				query: `query {\n  user(id: "usr-42") {\n    id\n    name\n    email\n    orders {\n      id\n      orderId\n      total\n      status\n      createdAt\n    }\n  }\n}`,
				variables: null,
				operationName: null,
			},
		});

		expect(response.statusCode).toBe(200);
		const responseBody = response.json();

		const data = responseBody.data;
		expect(data).toHaveProperty("user");
		expect(data.user).toHaveProperty("id");
		expect(data.user.id).toBe("usr-42");
		expect(data.user).toHaveProperty("name");
		expect(data.user.name).toBe("Emma Wilson");
		expect(data.user).toHaveProperty("email");
		expect(data.user.email).toBe("emma@example.com");
		expect(data.user).toHaveProperty("orders");
		expect(data.user.orders.length).toBe(2);
		expect(data.user.orders[0]).toHaveProperty("id");
		expect(data.user.orders[0].id).toBe("order-101");
		expect(data.user.orders[0]).toHaveProperty("orderId");
		expect(data.user.orders[0].orderId).toBe("ORD-2024-001");
		expect(data.user.orders[0]).toHaveProperty("total");
		expect(data.user.orders[0].total).toBe(149.99);
		expect(data.user.orders[0]).toHaveProperty("status");
		expect(data.user.orders[0].status).toBe("DELIVERED");
		expect(data.user.orders[0]).toHaveProperty("createdAt");
		expect(data.user.orders[0].createdAt).toBe("2024-01-15T10:30:00Z");
		expect(data.user.orders[1]).toHaveProperty("id");
		expect(data.user.orders[1].id).toBe("order-102");
		expect(data.user.orders[1]).toHaveProperty("orderId");
		expect(data.user.orders[1].orderId).toBe("ORD-2024-002");
		expect(data.user.orders[1]).toHaveProperty("total");
		expect(data.user.orders[1].total).toBe(89.5);
		expect(data.user.orders[1]).toHaveProperty("status");
		expect(data.user.orders[1].status).toBe("PROCESSING");
		expect(data.user.orders[1]).toHaveProperty("createdAt");
		expect(data.user.orders[1].createdAt).toBe("2024-12-20T14:22:00Z");
		const errors = responseBody.errors;
		expect(errors?.length ?? 0).toBe(0);
	});

	test("provides_directive", async () => {
		const app = createAppGraphqlQuery();
		const client = new TestClient(app);

		const response = await client.post("/graphql", {
			headers: { "x-spikard-fixture": "provides_directive" },
			json: {
				query: `query {\n  _entities(representations: [{__typename: "Post", id: "post-123"}]) {\n    ... on Post {\n      id\n      title\n      content\n      reviews {\n        id\n        rating\n        text\n        author {\n          id\n          name\n        }\n      }\n    }\n  }\n}`,
				variables: null,
				operationName: null,
			},
		});

		expect(response.statusCode).toBe(200);
		const responseBody = response.json();

		const data = responseBody.data;
		expect(data).toHaveProperty("_entities");
		expect(data._entities.length).toBe(1);
		expect(data._entities[0]).toHaveProperty("id");
		expect(data._entities[0].id).toBe("post-123");
		expect(data._entities[0]).toHaveProperty("title");
		expect(data._entities[0].title).toBe("Getting Started with GraphQL Federation");
		expect(data._entities[0]).toHaveProperty("content");
		expect(data._entities[0].content).toBe("Learn how to build scalable microservices...");
		expect(data._entities[0]).toHaveProperty("reviews");
		expect(data._entities[0].reviews.length).toBe(2);
		expect(data._entities[0].reviews[0]).toHaveProperty("id");
		expect(data._entities[0].reviews[0].id).toBe("rev-001");
		expect(data._entities[0].reviews[0]).toHaveProperty("rating");
		expect(data._entities[0].reviews[0].rating).toBe(5);
		expect(data._entities[0].reviews[0]).toHaveProperty("text");
		expect(data._entities[0].reviews[0].text).toBe("Excellent post!");
		expect(data._entities[0].reviews[0]).toHaveProperty("author");
		expect(data._entities[0].reviews[0].author).toHaveProperty("id");
		expect(data._entities[0].reviews[0].author.id).toBe("user-1");
		expect(data._entities[0].reviews[0].author).toHaveProperty("name");
		expect(data._entities[0].reviews[0].author.name).toBe("Charlie Brown");
		expect(data._entities[0].reviews[1]).toHaveProperty("id");
		expect(data._entities[0].reviews[1].id).toBe("rev-002");
		expect(data._entities[0].reviews[1]).toHaveProperty("rating");
		expect(data._entities[0].reviews[1].rating).toBe(4);
		expect(data._entities[0].reviews[1]).toHaveProperty("text");
		expect(data._entities[0].reviews[1].text).toBe("Very helpful");
		expect(data._entities[0].reviews[1]).toHaveProperty("author");
		expect(data._entities[0].reviews[1].author).toHaveProperty("id");
		expect(data._entities[0].reviews[1].author.id).toBe("user-2");
		expect(data._entities[0].reviews[1].author).toHaveProperty("name");
		expect(data._entities[0].reviews[1].author.name).toBe("Diana Prince");
		const errors = responseBody.errors;
		expect(errors?.length ?? 0).toBe(0);
	});

	test("external_field", async () => {
		const app = createAppGraphqlQuery();
		const client = new TestClient(app);

		const response = await client.post("/graphql", {
			headers: { "x-spikard-fixture": "external_field" },
			json: {
				query: `query {\n  _entities(representations: [{__typename: "Parcel", id: "parcel-x1", weight: 2.5, dimensions: "10x8x6"}]) {\n    ... on Parcel {\n      id\n      weight\n      dimensions\n      label\n    }\n  }\n}`,
				variables: null,
				operationName: null,
			},
		});

		expect(response.statusCode).toBe(200);
		const responseBody = response.json();

		const data = responseBody.data;
		expect(data).toHaveProperty("_entities");
		expect(data._entities.length).toBe(1);
		expect(data._entities[0]).toHaveProperty("id");
		expect(data._entities[0].id).toBe("parcel-x1");
		expect(data._entities[0]).toHaveProperty("weight");
		expect(data._entities[0].weight).toBe(2.5);
		expect(data._entities[0]).toHaveProperty("dimensions");
		expect(data._entities[0].dimensions).toBe("10x8x6");
		expect(data._entities[0]).toHaveProperty("label");
		expect(data._entities[0].label).toBe("SMALL_PACKAGE_2.5KG");
		const errors = responseBody.errors;
		expect(errors?.length ?? 0).toBe(0);
	});

	test("inaccessible_directive", async () => {
		const app = createAppGraphqlQuery();
		const client = new TestClient(app);

		const response = await client.post("/graphql", {
			headers: { "x-spikard-fixture": "inaccessible_directive" },
			json: {
				query: `query {\n  user(id: "user-42") {\n    id\n    name\n    email\n    internalScore\n    publicStatus\n  }\n}`,
				variables: null,
				operationName: null,
			},
		});

		expect(response.statusCode).toBe(400);
		const responseBody = response.json();

		const errors = responseBody.errors;
		expect(errors).toBeDefined();
		expect(errors?.length).toBe(1);
		expect(errors?.[0]?.message).toContain(
			"Cannot query field 'internalScore' on type 'User'. This field is @inaccessible and not available in the public schema.",
		);
	});

	test("subgraph_introspection", async () => {
		const app = createAppGraphqlQuery();
		const client = new TestClient(app);

		const response = await client.post("/graphql", {
			headers: { "x-spikard-fixture": "subgraph_introspection" },
			json: {
				query: `query {\n  _service {\n    sdl\n  }\n}`,
				variables: null,
				operationName: null,
			},
		});

		expect(response.statusCode).toBe(200);
		const responseBody = response.json();

		const data = responseBody.data;
		expect(data).toHaveProperty("_service");
		expect(data._service).toHaveProperty("sdl");
		expect(data._service.sdl).toBe(
			'type Account @key(fields: "accountId") {\n  accountId: ID!\n  accountName: String!\n  tier: String!\n  createdAt: String!\n}\n\ntype Query {\n  account(accountId: ID!): Account\n}',
		);
		const errors = responseBody.errors;
		expect(errors?.length ?? 0).toBe(0);
	});

	test("shareable_directive", async () => {
		const app = createAppGraphqlQuery();
		const client = new TestClient(app);

		const response = await client.post("/graphql", {
			headers: { "x-spikard-fixture": "shareable_directive" },
			json: {
				query: `query {\n  _entities(representations: [{__typename: "Product", id: "prod-001"}]) {\n    ... on Product {\n      id\n      name\n      description\n      category\n      price\n    }\n  }\n}`,
				variables: null,
				operationName: null,
			},
		});

		expect(response.statusCode).toBe(200);
		const responseBody = response.json();

		const data = responseBody.data;
		expect(data).toHaveProperty("_entities");
		expect(data._entities.length).toBe(1);
		expect(data._entities[0]).toHaveProperty("id");
		expect(data._entities[0].id).toBe("prod-001");
		expect(data._entities[0]).toHaveProperty("name");
		expect(data._entities[0].name).toBe("Wireless Headphones");
		expect(data._entities[0]).toHaveProperty("description");
		expect(data._entities[0].description).toBe("Premium noise-canceling headphones with 30-hour battery life");
		expect(data._entities[0]).toHaveProperty("category");
		expect(data._entities[0].category).toBe("Electronics");
		expect(data._entities[0]).toHaveProperty("price");
		expect(data._entities[0].price).toBe(199.99);
		const errors = responseBody.errors;
		expect(errors?.length ?? 0).toBe(0);
	});

	test("entity_resolution_basic", async () => {
		const app = createAppGraphqlQuery();
		const client = new TestClient(app);

		const response = await client.post("/graphql", {
			headers: { "x-spikard-fixture": "entity_resolution_basic" },
			json: {
				query: `query {\n  _entities(representations: [{__typename: "User", id: "1"}]) {\n    ... on User {\n      id\n      name\n      email\n    }\n  }\n}`,
				variables: null,
				operationName: null,
			},
		});

		expect(response.statusCode).toBe(200);
		const responseBody = response.json();

		const data = responseBody.data;
		expect(data).toHaveProperty("_entities");
		expect(data._entities.length).toBe(1);
		expect(data._entities[0]).toHaveProperty("id");
		expect(data._entities[0].id).toBe("1");
		expect(data._entities[0]).toHaveProperty("name");
		expect(data._entities[0].name).toBe("Alice Johnson");
		expect(data._entities[0]).toHaveProperty("email");
		expect(data._entities[0].email).toBe("alice@example.com");
		const errors = responseBody.errors;
		expect(errors?.length ?? 0).toBe(0);
	});

	test("override_directive", async () => {
		const app = createAppGraphqlQuery();
		const client = new TestClient(app);

		const response = await client.post("/graphql", {
			headers: { "x-spikard-fixture": "override_directive" },
			json: {
				query: `query {\n  user(id: "user-789") {\n    id\n    username\n    email\n    profile {\n      bio\n      joinDate\n      location\n    }\n  }\n}`,
				variables: null,
				operationName: null,
			},
		});

		expect(response.statusCode).toBe(200);
		const responseBody = response.json();

		const data = responseBody.data;
		expect(data).toHaveProperty("user");
		expect(data.user).toHaveProperty("id");
		expect(data.user.id).toBe("user-789");
		expect(data.user).toHaveProperty("username");
		expect(data.user.username).toBe("johndoe");
		expect(data.user).toHaveProperty("email");
		expect(data.user.email).toBe("john.doe@example.com");
		expect(data.user).toHaveProperty("profile");
		expect(data.user.profile).toHaveProperty("bio");
		expect(data.user.profile.bio).toBe("Software developer and tech enthusiast");
		expect(data.user.profile).toHaveProperty("joinDate");
		expect(data.user.profile.joinDate).toBe("2021-06-12");
		expect(data.user.profile).toHaveProperty("location");
		expect(data.user.profile.location).toBe("San Francisco, CA");
		const errors = responseBody.errors;
		expect(errors?.length ?? 0).toBe(0);
	});

	test("federation_type_mismatch", async () => {
		const app = createAppGraphqlQuery();
		const client = new TestClient(app);

		const response = await client.post("/graphql", {
			headers: { "x-spikard-fixture": "federation_type_mismatch" },
			json: {
				query: `query {\n  _entities(representations: [{__typename: "InvalidType", id: "1"}]) {\n    ... on Article {\n      id\n      title\n      content\n      author\n    }\n  }\n}`,
				variables: null,
				operationName: null,
			},
		});

		expect(response.statusCode).toBe(400);
		const responseBody = response.json();

		const errors = responseBody.errors;
		expect(errors).toBeDefined();
		expect(errors?.length).toBe(1);
		expect(errors?.[0]?.message).toContain("Unknown type 'InvalidType' in entity representation");
	});

	test("entity_with_compound_key", async () => {
		const app = createAppGraphqlQuery();
		const client = new TestClient(app);

		const response = await client.post("/graphql", {
			headers: { "x-spikard-fixture": "entity_with_compound_key" },
			json: {
				query: `query {\n  _entities(representations: [{__typename: "Product", sku: "ABC123", category: "electronics"}]) {\n    ... on Product {\n      sku\n      category\n      name\n      description\n      price\n      stock\n    }\n  }\n}`,
				variables: null,
				operationName: null,
			},
		});

		expect(response.statusCode).toBe(200);
		const responseBody = response.json();

		const data = responseBody.data;
		expect(data).toHaveProperty("_entities");
		expect(data._entities.length).toBe(1);
		expect(data._entities[0]).toHaveProperty("sku");
		expect(data._entities[0].sku).toBe("ABC123");
		expect(data._entities[0]).toHaveProperty("category");
		expect(data._entities[0].category).toBe("electronics");
		expect(data._entities[0]).toHaveProperty("name");
		expect(data._entities[0].name).toBe("Wireless Headphones");
		expect(data._entities[0]).toHaveProperty("description");
		expect(data._entities[0].description).toBe("Premium noise-cancelling wireless headphones");
		expect(data._entities[0]).toHaveProperty("price");
		expect(data._entities[0].price).toBe(199.99);
		expect(data._entities[0]).toHaveProperty("stock");
		expect(data._entities[0].stock).toBe(45);
		const errors = responseBody.errors;
		expect(errors?.length ?? 0).toBe(0);
	});

	test("federation_error_missing_entity", async () => {
		const app = createAppGraphqlQuery();
		const client = new TestClient(app);

		const response = await client.post("/graphql", {
			headers: { "x-spikard-fixture": "federation_error_missing_entity" },
			json: {
				query: `query {\n  _entities(representations: [{__typename: "Customer", id: "999999"}]) {\n    ... on Customer {\n      id\n      firstName\n      lastName\n      email\n    }\n  }\n}`,
				variables: null,
				operationName: null,
			},
		});

		expect(response.statusCode).toBe(200);
		const responseBody = response.json();

		const data = responseBody.data;
		expect(data).toHaveProperty("_entities");
		expect(data._entities.length).toBe(1);
		expect(data._entities[0]).toBe(null);
		const errors = responseBody.errors;
		expect(errors?.length ?? 0).toBe(0);
	});

	test("field_error", async () => {
		const app = createAppGraphqlQuery();
		const client = new TestClient(app);

		const response = await client.post("/graphql", {
			headers: { "x-spikard-fixture": "field_error" },
			json: {
				query: `query GetUser($id: ID!) {\n  user(id: $id) {\n    id\n    name\n    invalidField\n  }\n}`,
				variables: { id: "user-123" },
				operationName: null,
			},
		});

		expect(response.statusCode).toBe(400);
		const responseBody = response.json();

		const errors = responseBody.errors;
		expect(errors).toBeDefined();
		expect(errors?.length).toBe(1);
		expect(errors?.[0]?.message).toContain(
			'Cannot query field "invalidField" on type "User". Did you mean "id", "name", or "email"?',
		);
	});

	test("syntax_error", async () => {
		const app = createAppGraphqlQuery();
		const client = new TestClient(app);

		const response = await client.post("/graphql", {
			headers: { "x-spikard-fixture": "syntax_error" },
			json: {
				query: `query {\n  user(id: "123\n}`,
				variables: null,
				operationName: null,
			},
		});

		expect(response.statusCode).toBe(400);
		const responseBody = response.json();

		const errors = responseBody.errors;
		expect(errors).toBeDefined();
		expect(errors?.length).toBe(1);
		expect(errors?.[0]?.message).toContain("Syntax Error in GraphQL query at line 2, column 17: Unterminated string.");
	});

	test("type_error", async () => {
		const app = createAppGraphqlQuery();
		const client = new TestClient(app);

		const response = await client.post("/graphql", {
			headers: { "x-spikard-fixture": "type_error" },
			json: {
				query: `query GetPost($id: ID!) {\n  post(id: $id) {\n    id\n    title\n    content\n  }\n}`,
				variables: { id: true },
				operationName: null,
			},
		});

		expect(response.statusCode).toBe(400);
		const responseBody = response.json();

		const errors = responseBody.errors;
		expect(errors).toBeDefined();
		expect(errors?.length).toBe(1);
		expect(errors?.[0]?.message).toContain('Variable "$id" of type "ID!" was provided invalid value.');
	});

	test("query_batching", async () => {
		const app = createAppGraphqlQuery();
		const client = new TestClient(app);

		const response = await client.post("/graphql", {
			headers: { "x-spikard-fixture": "query_batching" },
			json: {
				query: ``,
				variables: null,
				operationName: null,
			},
		});

		expect(response.statusCode).toBe(200);
		const responseBody = response.json();

		const data = responseBody.data;
		expect(data.length).toBe(3);
		expect(data[0]).toHaveProperty("user");
		expect(data[0].user).toHaveProperty("id");
		expect(data[0].user.id).toBe("user-1");
		expect(data[0].user).toHaveProperty("name");
		expect(data[0].user.name).toBe("Alice Johnson");
		expect(data[0].user).toHaveProperty("email");
		expect(data[0].user.email).toBe("alice@example.com");
		expect(data[1]).toHaveProperty("user");
		expect(data[1].user).toHaveProperty("id");
		expect(data[1].user.id).toBe("user-2");
		expect(data[1].user).toHaveProperty("name");
		expect(data[1].user.name).toBe("Bob Smith");
		expect(data[1].user).toHaveProperty("email");
		expect(data[1].user.email).toBe("bob@example.com");
		expect(data[2]).toHaveProperty("post");
		expect(data[2].post).toHaveProperty("id");
		expect(data[2].post.id).toBe("post-1");
		expect(data[2].post).toHaveProperty("title");
		expect(data[2].post.title).toBe("GraphQL Performance Tips");
		expect(data[2].post).toHaveProperty("author_id");
		expect(data[2].post.author_id).toBe("user-1");
		const errors = responseBody.errors;
		expect(errors?.length ?? 0).toBe(0);
	});

	test("response_streaming", async () => {
		const app = createAppGraphqlQuery();
		const client = new TestClient(app);

		const response = await client.post("/graphql", {
			headers: { "x-spikard-fixture": "response_streaming" },
			json: {
				query: `query GetUserWithDeferred($userId: String!) {\n  user(id: $userId) {\n    id\n    name\n    email\n    ...DeferredPosts @defer(label: "userPosts")\n    ...DeferredFollowers @defer(label: "userFollowers")\n  }\n}\n\nfragment DeferredPosts on User {\n  posts @stream(initialCount: 1, label: "postsStream") {\n    id\n    title\n    published_at\n  }\n}\n\nfragment DeferredFollowers on User {\n  followers @stream(initialCount: 2, label: "followersStream") {\n    id\n    name\n  }\n}`,
				variables: { userId: "user-123" },
				operationName: null,
			},
		});

		expect(response.statusCode).toBe(200);
		const responseBody = response.json();

		const errors = responseBody.errors;
		expect(errors?.length ?? 0).toBe(0);
	});

	test("field_level_permissions", async () => {
		const app = createAppGraphqlQuery();
		const client = new TestClient(app);

		const response = await client.post("/graphql", {
			headers: { "x-spikard-fixture": "field_level_permissions" },
			json: {
				query: `query {\n  user(id: "user123") {\n    id\n    email\n    privateData\n  }\n}`,
				variables: { userId: "user123" },
				operationName: null,
			},
		});

		expect(response.statusCode).toBe(200);
		const responseBody = response.json();

		const errors = responseBody.errors;
		expect(errors).toBeDefined();
		expect(errors?.length).toBe(1);
		expect(errors?.[0]?.message).toContain("Field 'privateData' requires elevated permissions");
	});

	test("role_admin_allowed", async () => {
		const app = createAppGraphqlQuery();
		const client = new TestClient(app);

		const response = await client.post("/graphql", {
			headers: { "x-spikard-fixture": "role_admin_allowed" },
			json: {
				query: `query {\n  adminPanel {\n    stats {\n      totalUsers\n      activeUsers\n      totalRevenue\n    }\n  }\n}`,
				variables: null,
				operationName: null,
			},
		});

		expect(response.statusCode).toBe(200);
		const responseBody = response.json();

		const data = responseBody.data;
		expect(data).toHaveProperty("adminPanel");
		expect(data.adminPanel).toHaveProperty("stats");
		expect(data.adminPanel.stats).toHaveProperty("totalUsers");
		expect(data.adminPanel.stats.totalUsers).toBe(1250);
		expect(data.adminPanel.stats).toHaveProperty("activeUsers");
		expect(data.adminPanel.stats.activeUsers).toBe(856);
		expect(data.adminPanel.stats).toHaveProperty("totalRevenue");
		expect(data.adminPanel.stats.totalRevenue).toBe(125000.5);
		const errors = responseBody.errors;
		expect(errors?.length ?? 0).toBe(0);
	});

	test("resource_owner_allowed", async () => {
		const app = createAppGraphqlQuery();
		const client = new TestClient(app);

		const response = await client.post("/graphql", {
			headers: { "x-spikard-fixture": "resource_owner_allowed" },
			json: {
				query: `query GetUserProfile($userId: String!) {\n  user(id: $userId) {\n    id\n    profile {\n      bio\n      website\n      joinDate\n    }\n  }\n}`,
				variables: { userId: "user123" },
				operationName: null,
			},
		});

		expect(response.statusCode).toBe(200);
		const responseBody = response.json();

		const data = responseBody.data;
		expect(data).toHaveProperty("user");
		expect(data.user).toHaveProperty("id");
		expect(data.user.id).toBe("user123");
		expect(data.user).toHaveProperty("profile");
		expect(data.user.profile).toHaveProperty("bio");
		expect(data.user.profile.bio).toBe("Software engineer from San Francisco");
		expect(data.user.profile).toHaveProperty("website");
		expect(data.user.profile.website).toBe("https://example.com");
		expect(data.user.profile).toHaveProperty("joinDate");
		expect(data.user.profile.joinDate).toBe("2020-01-15");
		const errors = responseBody.errors;
		expect(errors?.length ?? 0).toBe(0);
	});

	test("permission_chain", async () => {
		const app = createAppGraphqlQuery();
		const client = new TestClient(app);

		const response = await client.post("/graphql", {
			headers: { "x-spikard-fixture": "permission_chain" },
			json: {
				query: `query {\n  dashboard {\n    id\n    publicMetrics {\n      pageViews\n      uniqueVisitors\n    }\n    privateMetrics {\n      pageViews\n      uniqueVisitors\n    }\n    adminSettings {\n      apiKey\n      webhookUrl\n    }\n  }\n}`,
				variables: null,
				operationName: null,
			},
		});

		expect(response.statusCode).toBe(200);
		const responseBody = response.json();

		const errors = responseBody.errors;
		expect(errors).toBeDefined();
		expect(errors?.length).toBe(2);
		expect(errors?.[0]?.message).toContain("Insufficient permissions to access privateMetrics");
		expect(errors?.[1]?.message).toContain("Insufficient permissions to access adminSettings");
	});

	test("resource_owner_denied", async () => {
		const app = createAppGraphqlQuery();
		const client = new TestClient(app);

		const response = await client.post("/graphql", {
			headers: { "x-spikard-fixture": "resource_owner_denied" },
			json: {
				query: `query GetUserProfile($userId: String!) {\n  user(id: $userId) {\n    id\n    profile {\n      bio\n      website\n    }\n  }\n}`,
				variables: { userId: "user456" },
				operationName: null,
			},
		});

		expect(response.statusCode).toBe(403);
		const responseBody = response.json();

		const errors = responseBody.errors;
		expect(errors).toBeDefined();
		expect(errors?.length).toBe(1);
		expect(errors?.[0]?.message).toContain("Not authorized to access this resource");
	});

	test("role_user_denied", async () => {
		const app = createAppGraphqlQuery();
		const client = new TestClient(app);

		const response = await client.post("/graphql", {
			headers: { "x-spikard-fixture": "role_user_denied" },
			json: {
				query: `query {\n  adminPanel {\n    stats {\n      totalUsers\n      activeUsers\n    }\n  }\n}`,
				variables: null,
				operationName: null,
			},
		});

		expect(response.statusCode).toBe(403);
		const responseBody = response.json();

		const errors = responseBody.errors;
		expect(errors).toBeDefined();
		expect(errors?.length).toBe(1);
		expect(errors?.[0]?.message).toContain("Insufficient permissions to access adminPanel");
	});

	test("jwt_valid", async () => {
		const app = createAppGraphqlQuery();
		const client = new TestClient(app);

		const response = await client.post("/graphql", {
			headers: { "x-spikard-fixture": "jwt_valid" },
			json: {
				query: `query {\n  currentUser {\n    id\n    email\n    name\n  }\n}`,
				variables: null,
				operationName: null,
			},
		});

		expect(response.statusCode).toBe(200);
		const responseBody = response.json();

		const data = responseBody.data;
		expect(data).toHaveProperty("currentUser");
		expect(data.currentUser).toHaveProperty("id");
		expect(data.currentUser.id).toBe("user123");
		expect(data.currentUser).toHaveProperty("email");
		expect(data.currentUser.email).toBe("john@example.com");
		expect(data.currentUser).toHaveProperty("name");
		expect(data.currentUser.name).toBe("John Doe");
		const errors = responseBody.errors;
		expect(errors?.length ?? 0).toBe(0);
	});

	test("api_key_invalid", async () => {
		const app = createAppGraphqlQuery();
		const client = new TestClient(app);

		const response = await client.post("/graphql", {
			headers: { "x-spikard-fixture": "api_key_invalid" },
			json: {
				query: `query {\n  secureData\n}`,
				variables: null,
				operationName: null,
			},
		});

		expect(response.statusCode).toBe(401);
		const responseBody = response.json();

		const errors = responseBody.errors;
		expect(errors).toBeDefined();
		expect(errors?.length).toBe(1);
		expect(errors?.[0]?.message).toContain("Invalid API key");
	});

	test("jwt_expired", async () => {
		const app = createAppGraphqlQuery();
		const client = new TestClient(app);

		const response = await client.post("/graphql", {
			headers: { "x-spikard-fixture": "jwt_expired" },
			json: {
				query: `query {\n  currentUser {\n    id\n    email\n  }\n}`,
				variables: null,
				operationName: null,
			},
		});

		expect(response.statusCode).toBe(401);
		const responseBody = response.json();

		const errors = responseBody.errors;
		expect(errors).toBeDefined();
		expect(errors?.length).toBe(1);
		expect(errors?.[0]?.message).toContain("Token expired");
	});

	test("jwt_invalid_signature", async () => {
		const app = createAppGraphqlQuery();
		const client = new TestClient(app);

		const response = await client.post("/graphql", {
			headers: { "x-spikard-fixture": "jwt_invalid_signature" },
			json: {
				query: `query {\n  currentUser {\n    id\n    email\n  }\n}`,
				variables: null,
				operationName: null,
			},
		});

		expect(response.statusCode).toBe(401);
		const responseBody = response.json();

		const errors = responseBody.errors;
		expect(errors).toBeDefined();
		expect(errors?.length).toBe(1);
		expect(errors?.[0]?.message).toContain("Invalid token signature");
	});

	test("no_authentication", async () => {
		const app = createAppGraphqlQuery();
		const client = new TestClient(app);

		const response = await client.post("/graphql", {
			headers: { "x-spikard-fixture": "no_authentication" },
			json: {
				query: `query {\n  protectedQuery\n}`,
				variables: null,
				operationName: null,
			},
		});

		expect(response.statusCode).toBe(401);
		const responseBody = response.json();

		const errors = responseBody.errors;
		expect(errors).toBeDefined();
		expect(errors?.length).toBe(1);
		expect(errors?.[0]?.message).toContain("Authentication required");
	});

	test("session_cookie_valid", async () => {
		const app = createAppGraphqlQuery();
		const client = new TestClient(app);

		const response = await client.post("/graphql", {
			headers: { "x-spikard-fixture": "session_cookie_valid" },
			json: {
				query: `query {\n  userProfile {\n    id\n    username\n    email\n  }\n}`,
				variables: null,
				operationName: null,
			},
		});

		expect(response.statusCode).toBe(200);
		const responseBody = response.json();

		const data = responseBody.data;
		expect(data).toHaveProperty("userProfile");
		expect(data.userProfile).toHaveProperty("id");
		expect(data.userProfile.id).toBe("user456");
		expect(data.userProfile).toHaveProperty("username");
		expect(data.userProfile.username).toBe("alice_smith");
		expect(data.userProfile).toHaveProperty("email");
		expect(data.userProfile.email).toBe("alice@example.com");
		const errors = responseBody.errors;
		expect(errors?.length ?? 0).toBe(0);
	});

	test("multiple_auth_methods", async () => {
		const app = createAppGraphqlQuery();
		const client = new TestClient(app);

		const response = await client.post("/graphql", {
			headers: { "x-spikard-fixture": "multiple_auth_methods" },
			json: {
				query: `query {\n  currentUser {\n    id\n    email\n    authMethod\n  }\n}`,
				variables: null,
				operationName: null,
			},
		});

		expect(response.statusCode).toBe(200);
		const responseBody = response.json();

		const data = responseBody.data;
		expect(data).toHaveProperty("currentUser");
		expect(data.currentUser).toHaveProperty("id");
		expect(data.currentUser.id).toBe("user123");
		expect(data.currentUser).toHaveProperty("email");
		expect(data.currentUser.email).toBe("john@example.com");
		expect(data.currentUser).toHaveProperty("authMethod");
		expect(data.currentUser.authMethod).toBe("jwt");
		const errors = responseBody.errors;
		expect(errors?.length ?? 0).toBe(0);
	});

	test("api_key_valid", async () => {
		const app = createAppGraphqlQuery();
		const client = new TestClient(app);

		const response = await client.post("/graphql", {
			headers: { "x-spikard-fixture": "api_key_valid" },
			json: {
				query: `query {\n  secureData\n}`,
				variables: null,
				operationName: null,
			},
		});

		expect(response.statusCode).toBe(200);
		const responseBody = response.json();

		const data = responseBody.data;
		expect(data).toHaveProperty("secureData");
		expect(data.secureData).toBe("Protected data from API key authentication");
		const errors = responseBody.errors;
		expect(errors?.length ?? 0).toBe(0);
	});

	test("invalid_types", async () => {
		const app = createAppGraphqlQuery();
		const client = new TestClient(app);

		const response = await client.post("/graphql", {
			headers: { "x-spikard-fixture": "invalid_types" },
			json: {
				query: `query SearchUsers($limit: Int!, $offset: Int) {\n  searchUsers(limit: $limit, offset: $offset) {\n    id\n    name\n    email\n  }\n}`,
				variables: { limit: "not_an_integer", offset: 10 },
				operationName: null,
			},
		});

		expect(response.statusCode).toBe(400);
		const responseBody = response.json();

		const errors = responseBody.errors;
		expect(errors).toBeDefined();
		expect(errors?.length).toBe(1);
		expect(errors?.[0]?.message).toContain('Variable "$limit" of type "Int!" was provided invalid value.');
	});

	test("dataloader_cache_hit", async () => {
		const app = createAppGraphqlQuery();
		const client = new TestClient(app);

		const response = await client.post("/graphql", {
			headers: { "x-spikard-fixture": "dataloader_cache_hit" },
			json: {
				query: `query {\n  user1: user(id: "1") {\n    id\n    name\n    email\n  }\n  user2: user(id: "1") {\n    id\n    name\n    username\n  }\n  user3: user(id: "2") {\n    id\n    name\n    email\n  }\n}`,
				variables: null,
				operationName: null,
			},
		});

		expect(response.statusCode).toBe(200);
		const responseBody = response.json();

		const data = responseBody.data;
		expect(data).toHaveProperty("user1");
		expect(data.user1).toHaveProperty("id");
		expect(data.user1.id).toBe("1");
		expect(data.user1).toHaveProperty("name");
		expect(data.user1.name).toBe("Alice Smith");
		expect(data.user1).toHaveProperty("email");
		expect(data.user1.email).toBe("alice@example.com");
		expect(data).toHaveProperty("user2");
		expect(data.user2).toHaveProperty("id");
		expect(data.user2.id).toBe("1");
		expect(data.user2).toHaveProperty("name");
		expect(data.user2.name).toBe("Alice Smith");
		expect(data.user2).toHaveProperty("username");
		expect(data.user2.username).toBe("alice_smith");
		expect(data).toHaveProperty("user3");
		expect(data.user3).toHaveProperty("id");
		expect(data.user3.id).toBe("2");
		expect(data.user3).toHaveProperty("name");
		expect(data.user3.name).toBe("Bob Johnson");
		expect(data.user3).toHaveProperty("email");
		expect(data.user3.email).toBe("bob@example.com");
		const errors = responseBody.errors;
		expect(errors?.length ?? 0).toBe(0);
	});

	test("dataloader_with_variables", async () => {
		const app = createAppGraphqlQuery();
		const client = new TestClient(app);

		const response = await client.post("/graphql", {
			headers: { "x-spikard-fixture": "dataloader_with_variables" },
			json: {
				query: `query GetPosts($ids: [ID!]!) {\n  posts(ids: $ids) {\n    id\n    title\n    slug\n    publishedAt\n    tags\n  }\n}`,
				variables: { ids: ["1", "2", "3"] },
				operationName: null,
			},
		});

		expect(response.statusCode).toBe(200);
		const responseBody = response.json();

		const data = responseBody.data;
		expect(data).toHaveProperty("posts");
		expect(data.posts.length).toBe(3);
		expect(data.posts[0]).toHaveProperty("id");
		expect(data.posts[0].id).toBe("1");
		expect(data.posts[0]).toHaveProperty("title");
		expect(data.posts[0].title).toBe("Getting Started with GraphQL");
		expect(data.posts[0]).toHaveProperty("slug");
		expect(data.posts[0].slug).toBe("getting-started-graphql");
		expect(data.posts[0]).toHaveProperty("publishedAt");
		expect(data.posts[0].publishedAt).toBe("2025-01-10T08:00:00Z");
		expect(data.posts[0]).toHaveProperty("tags");
		expect(data.posts[0].tags.length).toBe(2);
		expect(data.posts[0].tags[0]).toBe("graphql");
		expect(data.posts[0].tags[1]).toBe("tutorial");
		expect(data.posts[1]).toHaveProperty("id");
		expect(data.posts[1].id).toBe("2");
		expect(data.posts[1]).toHaveProperty("title");
		expect(data.posts[1].title).toBe("Mastering DataLoader");
		expect(data.posts[1]).toHaveProperty("slug");
		expect(data.posts[1].slug).toBe("mastering-dataloader");
		expect(data.posts[1]).toHaveProperty("publishedAt");
		expect(data.posts[1].publishedAt).toBe("2025-01-15T10:30:00Z");
		expect(data.posts[1]).toHaveProperty("tags");
		expect(data.posts[1].tags.length).toBe(3);
		expect(data.posts[1].tags[0]).toBe("dataloader");
		expect(data.posts[1].tags[1]).toBe("performance");
		expect(data.posts[1].tags[2]).toBe("optimization");
		expect(data.posts[2]).toHaveProperty("id");
		expect(data.posts[2].id).toBe("3");
		expect(data.posts[2]).toHaveProperty("title");
		expect(data.posts[2].title).toBe("GraphQL Best Practices");
		expect(data.posts[2]).toHaveProperty("slug");
		expect(data.posts[2].slug).toBe("graphql-best-practices");
		expect(data.posts[2]).toHaveProperty("publishedAt");
		expect(data.posts[2].publishedAt).toBe("2025-01-20T14:45:00Z");
		expect(data.posts[2]).toHaveProperty("tags");
		expect(data.posts[2].tags.length).toBe(3);
		expect(data.posts[2].tags[0]).toBe("graphql");
		expect(data.posts[2].tags[1]).toBe("best-practices");
		expect(data.posts[2].tags[2]).toBe("patterns");
		const errors = responseBody.errors;
		expect(errors?.length ?? 0).toBe(0);
	});

	test("dataloader_batch_users", async () => {
		const app = createAppGraphqlQuery();
		const client = new TestClient(app);

		const response = await client.post("/graphql", {
			headers: { "x-spikard-fixture": "dataloader_batch_users" },
			json: {
				query: `query GetUsers($ids: [ID!]!) {\n  users(ids: $ids) {\n    id\n    name\n    email\n    age\n  }\n}`,
				variables: { ids: ["1", "2", "3"] },
				operationName: null,
			},
		});

		expect(response.statusCode).toBe(200);
		const responseBody = response.json();

		const data = responseBody.data;
		expect(data).toHaveProperty("users");
		expect(data.users.length).toBe(3);
		expect(data.users[0]).toHaveProperty("id");
		expect(data.users[0].id).toBe("1");
		expect(data.users[0]).toHaveProperty("name");
		expect(data.users[0].name).toBe("Alice Johnson");
		expect(data.users[0]).toHaveProperty("email");
		expect(data.users[0].email).toBe("alice@example.com");
		expect(data.users[0]).toHaveProperty("age");
		expect(data.users[0].age).toBe(28);
		expect(data.users[1]).toHaveProperty("id");
		expect(data.users[1].id).toBe("2");
		expect(data.users[1]).toHaveProperty("name");
		expect(data.users[1].name).toBe("Bob Smith");
		expect(data.users[1]).toHaveProperty("email");
		expect(data.users[1].email).toBe("bob@example.com");
		expect(data.users[1]).toHaveProperty("age");
		expect(data.users[1].age).toBe(34);
		expect(data.users[2]).toHaveProperty("id");
		expect(data.users[2].id).toBe("3");
		expect(data.users[2]).toHaveProperty("name");
		expect(data.users[2].name).toBe("Carol Davis");
		expect(data.users[2]).toHaveProperty("email");
		expect(data.users[2].email).toBe("carol@example.com");
		expect(data.users[2]).toHaveProperty("age");
		expect(data.users[2].age).toBe(26);
		const errors = responseBody.errors;
		expect(errors?.length ?? 0).toBe(0);
	});

	test("dataloader_error_handling", async () => {
		const app = createAppGraphqlQuery();
		const client = new TestClient(app);

		const response = await client.post("/graphql", {
			headers: { "x-spikard-fixture": "dataloader_error_handling" },
			json: {
				query: `query GetUsers($ids: [ID!]!) {\n  users(ids: $ids) {\n    id\n    name\n    email\n  }\n}`,
				variables: { ids: ["1", "999", "2"] },
				operationName: null,
			},
		});

		expect(response.statusCode).toBe(200);
		const responseBody = response.json();

		const data = responseBody.data;
		expect(data).toHaveProperty("users");
		expect(data.users.length).toBe(3);
		expect(data.users[0]).toHaveProperty("id");
		expect(data.users[0].id).toBe("1");
		expect(data.users[0]).toHaveProperty("name");
		expect(data.users[0].name).toBe("Alice Johnson");
		expect(data.users[0]).toHaveProperty("email");
		expect(data.users[0].email).toBe("alice@example.com");
		expect(data.users[1]).toBe(null);
		expect(data.users[2]).toHaveProperty("id");
		expect(data.users[2].id).toBe("2");
		expect(data.users[2]).toHaveProperty("name");
		expect(data.users[2].name).toBe("Bob Smith");
		expect(data.users[2]).toHaveProperty("email");
		expect(data.users[2].email).toBe("bob@example.com");
		const errors = responseBody.errors;
		expect(errors).toBeDefined();
		expect(errors?.length).toBe(1);
		expect(errors?.[0]?.message).toContain("User not found with id '999'");
	});

	test("dataloader_custom_key", async () => {
		const app = createAppGraphqlQuery();
		const client = new TestClient(app);

		const response = await client.post("/graphql", {
			headers: { "x-spikard-fixture": "dataloader_custom_key" },
			json: {
				query: `query GetProduct($slug: String!) {\n  productBySlug(slug: $slug) {\n    id\n    name\n    slug\n    price\n    category\n    description\n  }\n}`,
				variables: { slug: "laptop-pro-2025" },
				operationName: null,
			},
		});

		expect(response.statusCode).toBe(200);
		const responseBody = response.json();

		const data = responseBody.data;
		expect(data).toHaveProperty("productBySlug");
		expect(data.productBySlug).toHaveProperty("id");
		expect(data.productBySlug.id).toBe("prod-1");
		expect(data.productBySlug).toHaveProperty("name");
		expect(data.productBySlug.name).toBe("Professional Laptop");
		expect(data.productBySlug).toHaveProperty("slug");
		expect(data.productBySlug.slug).toBe("laptop-pro-2025");
		expect(data.productBySlug).toHaveProperty("price");
		expect(data.productBySlug.price).toBe(1299.99);
		expect(data.productBySlug).toHaveProperty("category");
		expect(data.productBySlug.category).toBe("electronics");
		expect(data.productBySlug).toHaveProperty("description");
		expect(data.productBySlug.description).toBe("High-performance laptop for professionals");
		const errors = responseBody.errors;
		expect(errors?.length ?? 0).toBe(0);
	});

	test("dataloader_nested_batching", async () => {
		const app = createAppGraphqlQuery();
		const client = new TestClient(app);

		const response = await client.post("/graphql", {
			headers: { "x-spikard-fixture": "dataloader_nested_batching" },
			json: {
				query: `query {\n  posts {\n    id\n    title\n    comments {\n      id\n      text\n      author {\n        id\n        name\n        email\n      }\n    }\n  }\n}`,
				variables: null,
				operationName: null,
			},
		});

		expect(response.statusCode).toBe(200);
		const responseBody = response.json();

		const data = responseBody.data;
		expect(data).toHaveProperty("posts");
		expect(data.posts.length).toBe(2);
		expect(data.posts[0]).toHaveProperty("id");
		expect(data.posts[0].id).toBe("post-1");
		expect(data.posts[0]).toHaveProperty("title");
		expect(data.posts[0].title).toBe("GraphQL Introduction");
		expect(data.posts[0]).toHaveProperty("comments");
		expect(data.posts[0].comments.length).toBe(2);
		expect(data.posts[0].comments[0]).toHaveProperty("id");
		expect(data.posts[0].comments[0].id).toBe("comment-1");
		expect(data.posts[0].comments[0]).toHaveProperty("text");
		expect(data.posts[0].comments[0].text).toBe("Great article!");
		expect(data.posts[0].comments[0]).toHaveProperty("author");
		expect(data.posts[0].comments[0].author).toHaveProperty("id");
		expect(data.posts[0].comments[0].author.id).toBe("user-1");
		expect(data.posts[0].comments[0].author).toHaveProperty("name");
		expect(data.posts[0].comments[0].author.name).toBe("Alice Johnson");
		expect(data.posts[0].comments[0].author).toHaveProperty("email");
		expect(data.posts[0].comments[0].author.email).toBe("alice@example.com");
		expect(data.posts[0].comments[1]).toHaveProperty("id");
		expect(data.posts[0].comments[1].id).toBe("comment-2");
		expect(data.posts[0].comments[1]).toHaveProperty("text");
		expect(data.posts[0].comments[1].text).toBe("Very helpful");
		expect(data.posts[0].comments[1]).toHaveProperty("author");
		expect(data.posts[0].comments[1].author).toHaveProperty("id");
		expect(data.posts[0].comments[1].author.id).toBe("user-2");
		expect(data.posts[0].comments[1].author).toHaveProperty("name");
		expect(data.posts[0].comments[1].author.name).toBe("Bob Smith");
		expect(data.posts[0].comments[1].author).toHaveProperty("email");
		expect(data.posts[0].comments[1].author.email).toBe("bob@example.com");
		expect(data.posts[1]).toHaveProperty("id");
		expect(data.posts[1].id).toBe("post-2");
		expect(data.posts[1]).toHaveProperty("title");
		expect(data.posts[1].title).toBe("Advanced Patterns");
		expect(data.posts[1]).toHaveProperty("comments");
		expect(data.posts[1].comments.length).toBe(1);
		expect(data.posts[1].comments[0]).toHaveProperty("id");
		expect(data.posts[1].comments[0].id).toBe("comment-3");
		expect(data.posts[1].comments[0]).toHaveProperty("text");
		expect(data.posts[1].comments[0].text).toBe("Excellent explanation");
		expect(data.posts[1].comments[0]).toHaveProperty("author");
		expect(data.posts[1].comments[0].author).toHaveProperty("id");
		expect(data.posts[1].comments[0].author.id).toBe("user-1");
		expect(data.posts[1].comments[0].author).toHaveProperty("name");
		expect(data.posts[1].comments[0].author.name).toBe("Alice Johnson");
		expect(data.posts[1].comments[0].author).toHaveProperty("email");
		expect(data.posts[1].comments[0].author.email).toBe("alice@example.com");
		const errors = responseBody.errors;
		expect(errors?.length ?? 0).toBe(0);
	});

	test("dataloader_priming", async () => {
		const app = createAppGraphqlQuery();
		const client = new TestClient(app);

		const response = await client.post("/graphql", {
			headers: { "x-spikard-fixture": "dataloader_priming" },
			json: {
				query: `query {\n  userList {\n    id\n    name\n    email\n    role\n  }\n}`,
				variables: null,
				operationName: null,
			},
		});

		expect(response.statusCode).toBe(200);
		const responseBody = response.json();

		const data = responseBody.data;
		expect(data).toHaveProperty("userList");
		expect(data.userList.length).toBe(3);
		expect(data.userList[0]).toHaveProperty("id");
		expect(data.userList[0].id).toBe("user-1");
		expect(data.userList[0]).toHaveProperty("name");
		expect(data.userList[0].name).toBe("Alice Johnson");
		expect(data.userList[0]).toHaveProperty("email");
		expect(data.userList[0].email).toBe("alice@example.com");
		expect(data.userList[0]).toHaveProperty("role");
		expect(data.userList[0].role).toBe("admin");
		expect(data.userList[1]).toHaveProperty("id");
		expect(data.userList[1].id).toBe("user-2");
		expect(data.userList[1]).toHaveProperty("name");
		expect(data.userList[1].name).toBe("Bob Smith");
		expect(data.userList[1]).toHaveProperty("email");
		expect(data.userList[1].email).toBe("bob@example.com");
		expect(data.userList[1]).toHaveProperty("role");
		expect(data.userList[1].role).toBe("user");
		expect(data.userList[2]).toHaveProperty("id");
		expect(data.userList[2].id).toBe("user-3");
		expect(data.userList[2]).toHaveProperty("name");
		expect(data.userList[2].name).toBe("Carol Davis");
		expect(data.userList[2]).toHaveProperty("email");
		expect(data.userList[2].email).toBe("carol@example.com");
		expect(data.userList[2]).toHaveProperty("role");
		expect(data.userList[2].role).toBe("moderator");
		const errors = responseBody.errors;
		expect(errors?.length ?? 0).toBe(0);
	});

	test("dataloader_n_plus_one_prevention", async () => {
		const app = createAppGraphqlQuery();
		const client = new TestClient(app);

		const response = await client.post("/graphql", {
			headers: { "x-spikard-fixture": "dataloader_n_plus_one_prevention" },
			json: {
				query: `query {\n  posts {\n    id\n    title\n    content\n    author {\n      id\n      name\n      email\n    }\n  }\n}`,
				variables: null,
				operationName: null,
			},
		});

		expect(response.statusCode).toBe(200);
		const responseBody = response.json();

		const data = responseBody.data;
		expect(data).toHaveProperty("posts");
		expect(data.posts.length).toBe(3);
		expect(data.posts[0]).toHaveProperty("id");
		expect(data.posts[0].id).toBe("post-1");
		expect(data.posts[0]).toHaveProperty("title");
		expect(data.posts[0].title).toBe("GraphQL Basics");
		expect(data.posts[0]).toHaveProperty("content");
		expect(data.posts[0].content).toBe("Introduction to GraphQL...");
		expect(data.posts[0]).toHaveProperty("author");
		expect(data.posts[0].author).toHaveProperty("id");
		expect(data.posts[0].author.id).toBe("user-1");
		expect(data.posts[0].author).toHaveProperty("name");
		expect(data.posts[0].author.name).toBe("Alice Johnson");
		expect(data.posts[0].author).toHaveProperty("email");
		expect(data.posts[0].author.email).toBe("alice@example.com");
		expect(data.posts[1]).toHaveProperty("id");
		expect(data.posts[1].id).toBe("post-2");
		expect(data.posts[1]).toHaveProperty("title");
		expect(data.posts[1].title).toBe("DataLoader Patterns");
		expect(data.posts[1]).toHaveProperty("content");
		expect(data.posts[1].content).toBe("Optimizing GraphQL queries...");
		expect(data.posts[1]).toHaveProperty("author");
		expect(data.posts[1].author).toHaveProperty("id");
		expect(data.posts[1].author.id).toBe("user-2");
		expect(data.posts[1].author).toHaveProperty("name");
		expect(data.posts[1].author.name).toBe("Bob Smith");
		expect(data.posts[1].author).toHaveProperty("email");
		expect(data.posts[1].author.email).toBe("bob@example.com");
		expect(data.posts[2]).toHaveProperty("id");
		expect(data.posts[2].id).toBe("post-3");
		expect(data.posts[2]).toHaveProperty("title");
		expect(data.posts[2].title).toBe("Advanced GraphQL");
		expect(data.posts[2]).toHaveProperty("content");
		expect(data.posts[2].content).toBe("Custom directives and more...");
		expect(data.posts[2]).toHaveProperty("author");
		expect(data.posts[2].author).toHaveProperty("id");
		expect(data.posts[2].author.id).toBe("user-1");
		expect(data.posts[2].author).toHaveProperty("name");
		expect(data.posts[2].author.name).toBe("Alice Johnson");
		expect(data.posts[2].author).toHaveProperty("email");
		expect(data.posts[2].author.email).toBe("alice@example.com");
		const errors = responseBody.errors;
		expect(errors?.length ?? 0).toBe(0);
	});
});
