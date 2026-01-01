import { assertEquals, assert } from "jsr:@std/assert@1";
import { TestClient } from "@spikard/wasm";
import { createAppGraphqlQuery } from "../app/main.ts";

Deno.test("GraphQL query: transform_directive", async () => {
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

	assertEquals(response.statusCode, 200);
	const responseBody = response.json();

	const data = responseBody.data;
	assert(Object.hasOwn(data, "message"));
	assertEquals(data.message, "HELLO FROM GRAPHQL");
	assert(Object.hasOwn(data, "title"));
	assertEquals(data.title, "WELCOME TO SPIKARD");
	const errors = responseBody.errors;
	assertEquals(errors?.length ?? 0, 0);
});

Deno.test("GraphQL query: rate_limit_directive", async () => {
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

	assertEquals(response.statusCode, 200);
	const responseBody = response.json();

	const data = responseBody.data;
	assert(Object.hasOwn(data, "expensiveQuery"));
	assertEquals(data.expensiveQuery, "Result from expensive computation");
	const errors = responseBody.errors;
	assertEquals(errors?.length ?? 0, 0);
});

Deno.test("GraphQL query: cache_directive", async () => {
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

	assertEquals(response.statusCode, 200);
	const responseBody = response.json();

	const data = responseBody.data;
	assert(Object.hasOwn(data, "user"));
	assert(Object.hasOwn(data.user, "email"));
	assertEquals(data.user.email, "alice@example.com");
	assert(Object.hasOwn(data.user, "id"));
	assertEquals(data.user.id, "1");
	assert(Object.hasOwn(data.user, "name"));
	assertEquals(data.user.name, "Alice Smith");
	const errors = responseBody.errors;
	assertEquals(errors?.length ?? 0, 0);
});

Deno.test("GraphQL query: custom_auth_directive", async () => {
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

	assertEquals(response.statusCode, 200);
	const responseBody = response.json();

	const data = responseBody.data;
	assert(Object.hasOwn(data, "moderatorData"));
	assertEquals(data.moderatorData, null);
	assert(Object.hasOwn(data, "publicData"));
	assertEquals(data.publicData, "Anyone can see this");
	assert(Object.hasOwn(data, "secretData"));
	assertEquals(data.secretData, null);
	const errors = responseBody.errors;
	assert(errors !== undefined);
	assertEquals(errors?.length, 2);
	assert(errors?.[0]?.message.includes("Unauthorized: User role USER cannot access ADMIN field"));
	assert(errors?.[1]?.message.includes("Unauthorized: User role USER cannot access MODERATOR field"));
});

Deno.test("GraphQL query: deprecated_field", async () => {
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

	assertEquals(response.statusCode, 200);
	const responseBody = response.json();

	const data = responseBody.data;
	assert(Object.hasOwn(data, "newField"));
	assertEquals(data.newField, "modern value");
	assert(Object.hasOwn(data, "oldField"));
	assertEquals(data.oldField, "legacy value");
	const errors = responseBody.errors;
	assertEquals(errors?.length ?? 0, 0);
});

Deno.test("GraphQL query: datetime_scalar", async () => {
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

	assertEquals(response.statusCode, 200);
	const responseBody = response.json();

	const data = responseBody.data;
	assert(Object.hasOwn(data, "events"));
	assertEquals(data.events.length, 2);
	assert(Object.hasOwn(data.events[0], "completedAt"));
	assertEquals(data.events[0].completedAt, "2025-06-15T17:00:00Z");
	assert(Object.hasOwn(data.events[0], "id"));
	assertEquals(data.events[0].id, "event-1");
	assert(Object.hasOwn(data.events[0], "scheduledAt"));
	assertEquals(data.events[0].scheduledAt, "2025-06-15T09:00:00Z");
	assert(Object.hasOwn(data.events[0], "title"));
	assertEquals(data.events[0].title, "Conference");
	assert(Object.hasOwn(data.events[1], "completedAt"));
	assertEquals(data.events[1].completedAt, null);
	assert(Object.hasOwn(data.events[1], "id"));
	assertEquals(data.events[1].id, "event-2");
	assert(Object.hasOwn(data.events[1], "scheduledAt"));
	assertEquals(data.events[1].scheduledAt, "2025-08-20T10:00:00Z");
	assert(Object.hasOwn(data.events[1], "title"));
	assertEquals(data.events[1].title, "Hackathon");
	const errors = responseBody.errors;
	assertEquals(errors?.length ?? 0, 0);
});

Deno.test("GraphQL query: uuid_scalar", async () => {
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

	assertEquals(response.statusCode, 200);
	const responseBody = response.json();

	const data = responseBody.data;
	assert(Object.hasOwn(data, "resource"));
	assert(Object.hasOwn(data.resource, "id"));
	assertEquals(data.resource.id, "550e8400-e29b-41d4-a716-446655440000");
	assert(Object.hasOwn(data.resource, "name"));
	assertEquals(data.resource.name, "Primary Resource");
	assert(Object.hasOwn(data.resource, "ownerId"));
	assertEquals(data.resource.ownerId, "6ba7b811-9dad-11d1-80b4-00c04fd430c8");
	assert(Object.hasOwn(data.resource, "parentId"));
	assertEquals(data.resource.parentId, "6ba7b810-9dad-11d1-80b4-00c04fd430c8");
	assert(Object.hasOwn(data.resource, "relatedIds"));
	assertEquals(data.resource.relatedIds.length, 2);
	assertEquals(data.resource.relatedIds[0], "6ba7b812-9dad-11d1-80b4-00c04fd430c8");
	assertEquals(data.resource.relatedIds[1], "6ba7b814-9dad-11d1-80b4-00c04fd430c8");
	const errors = responseBody.errors;
	assertEquals(errors?.length ?? 0, 0);
});

Deno.test("GraphQL query: json_scalar", async () => {
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

	assertEquals(response.statusCode, 200);
	const responseBody = response.json();

	const data = responseBody.data;
	assert(Object.hasOwn(data, "configuration"));
	assert(Object.hasOwn(data.configuration, "id"));
	assertEquals(data.configuration.id, "config-1");
	assert(Object.hasOwn(data.configuration, "metadata"));
	assert(Object.hasOwn(data.configuration.metadata, "author"));
	assertEquals(data.configuration.metadata.author, "DevOps Team");
	assert(Object.hasOwn(data.configuration.metadata, "environment"));
	assertEquals(data.configuration.metadata.environment, "production");
	assert(Object.hasOwn(data.configuration.metadata, "lastUpdated"));
	assertEquals(data.configuration.metadata.lastUpdated, "2025-12-27T10:00:00Z");
	assert(Object.hasOwn(data.configuration.metadata, "version"));
	assertEquals(data.configuration.metadata.version, "1.0.0");
	assert(Object.hasOwn(data.configuration, "name"));
	assertEquals(data.configuration.name, "Production Config");
	assert(Object.hasOwn(data.configuration, "settings"));
	assert(Object.hasOwn(data.configuration.settings, "endpoints"));
	assertEquals(data.configuration.settings.endpoints.length, 2);
	assertEquals(data.configuration.settings.endpoints[0], "https://api.example.com");
	assertEquals(data.configuration.settings.endpoints[1], "https://api-backup.example.com");
	assert(Object.hasOwn(data.configuration.settings, "features"));
	assert(Object.hasOwn(data.configuration.settings.features, "caching"));
	assertEquals(data.configuration.settings.features.caching, true);
	assert(Object.hasOwn(data.configuration.settings.features, "compression"));
	assertEquals(data.configuration.settings.features.compression, true);
	assert(Object.hasOwn(data.configuration.settings.features, "tracing"));
	assertEquals(data.configuration.settings.features.tracing, false);
	assert(Object.hasOwn(data.configuration.settings, "retries"));
	assertEquals(data.configuration.settings.retries, 3);
	assert(Object.hasOwn(data.configuration.settings, "timeout"));
	assertEquals(data.configuration.settings.timeout, 30000);
	const errors = responseBody.errors;
	assertEquals(errors?.length ?? 0, 0);
});

Deno.test("GraphQL query: complex_query", async () => {
	const app = createAppGraphqlQuery();
	const client = new TestClient(app);

	const response = await client.post("/graphql", {
		headers: { "x-spikard-fixture": "complex_query" },
		json: {
			query: `query ComplexSearch($searchTerm: String!, $userLimit: Int!, $postLimit: Int!) {\n  search(term: $searchTerm) {\n    total\n    users(limit: $userLimit) {\n      id\n      name\n      email\n      profile {\n        bio\n        avatar\n        joinedAt\n      }\n      recentPosts: posts(limit: 3) {\n        id\n        title\n        likes\n      }\n      followerCount: followers(limit: 100) {\n        id\n      }\n    }\n    posts(limit: $postLimit) {\n      id\n      title\n      content\n      likes\n      author {\n        id\n        name\n        profile {\n          avatar\n        }\n      }\n      topComments: comments(limit: 5) {\n        id\n        text\n        likes\n        author {\n          id\n          name\n        }\n      }\n    }\n  }\n}`,
			variables: { postLimit: 10, searchTerm: "graphql", userLimit: 5 },
			operationName: "ComplexSearch",
		},
	});

	assertEquals(response.statusCode, 200);
	const responseBody = response.json();

	const data = responseBody.data;
	assert(Object.hasOwn(data, "search"));
	assert(Object.hasOwn(data.search, "posts"));
	assertEquals(data.search.posts.length, 2);
	assert(Object.hasOwn(data.search.posts[0], "author"));
	assert(Object.hasOwn(data.search.posts[0].author, "id"));
	assertEquals(data.search.posts[0].author.id, "user-1");
	assert(Object.hasOwn(data.search.posts[0].author, "name"));
	assertEquals(data.search.posts[0].author.name, "GraphQL Expert");
	assert(Object.hasOwn(data.search.posts[0].author, "profile"));
	assert(Object.hasOwn(data.search.posts[0].author.profile, "avatar"));
	assertEquals(data.search.posts[0].author.profile.avatar, "https://example.com/avatars/expert.jpg");
	assert(Object.hasOwn(data.search.posts[0], "content"));
	assertEquals(data.search.posts[0].content, "A comprehensive guide to GraphQL best practices and patterns...");
	assert(Object.hasOwn(data.search.posts[0], "id"));
	assertEquals(data.search.posts[0].id, "post-101");
	assert(Object.hasOwn(data.search.posts[0], "likes"));
	assertEquals(data.search.posts[0].likes, 234);
	assert(Object.hasOwn(data.search.posts[0], "title"));
	assertEquals(data.search.posts[0].title, "GraphQL Best Practices");
	assert(Object.hasOwn(data.search.posts[0], "topComments"));
	assertEquals(data.search.posts[0].topComments.length, 2);
	assert(Object.hasOwn(data.search.posts[0].topComments[0], "author"));
	assert(Object.hasOwn(data.search.posts[0].topComments[0].author, "id"));
	assertEquals(data.search.posts[0].topComments[0].author.id, "user-2");
	assert(Object.hasOwn(data.search.posts[0].topComments[0].author, "name"));
	assertEquals(data.search.posts[0].topComments[0].author.name, "API Developer");
	assert(Object.hasOwn(data.search.posts[0].topComments[0], "id"));
	assertEquals(data.search.posts[0].topComments[0].id, "comment-1");
	assert(Object.hasOwn(data.search.posts[0].topComments[0], "likes"));
	assertEquals(data.search.posts[0].topComments[0].likes, 45);
	assert(Object.hasOwn(data.search.posts[0].topComments[0], "text"));
	assertEquals(data.search.posts[0].topComments[0].text, "Great post, very helpful!");
	assert(Object.hasOwn(data.search.posts[0].topComments[1], "author"));
	assert(Object.hasOwn(data.search.posts[0].topComments[1].author, "id"));
	assertEquals(data.search.posts[0].topComments[1].author.id, "user-3");
	assert(Object.hasOwn(data.search.posts[0].topComments[1].author, "name"));
	assertEquals(data.search.posts[0].topComments[1].author.name, "Data Scientist");
	assert(Object.hasOwn(data.search.posts[0].topComments[1], "id"));
	assertEquals(data.search.posts[0].topComments[1].id, "comment-2");
	assert(Object.hasOwn(data.search.posts[0].topComments[1], "likes"));
	assertEquals(data.search.posts[0].topComments[1].likes, 32);
	assert(Object.hasOwn(data.search.posts[0].topComments[1], "text"));
	assertEquals(data.search.posts[0].topComments[1].text, "Could you elaborate on caching?");
	assert(Object.hasOwn(data.search.posts[1], "author"));
	assert(Object.hasOwn(data.search.posts[1].author, "id"));
	assertEquals(data.search.posts[1].author.id, "user-1");
	assert(Object.hasOwn(data.search.posts[1].author, "name"));
	assertEquals(data.search.posts[1].author.name, "GraphQL Expert");
	assert(Object.hasOwn(data.search.posts[1].author, "profile"));
	assert(Object.hasOwn(data.search.posts[1].author.profile, "avatar"));
	assertEquals(data.search.posts[1].author.profile.avatar, "https://example.com/avatars/expert.jpg");
	assert(Object.hasOwn(data.search.posts[1], "content"));
	assertEquals(data.search.posts[1].content, "Exploring common patterns for designing GraphQL schemas...");
	assert(Object.hasOwn(data.search.posts[1], "id"));
	assertEquals(data.search.posts[1].id, "post-102");
	assert(Object.hasOwn(data.search.posts[1], "likes"));
	assertEquals(data.search.posts[1].likes, 189);
	assert(Object.hasOwn(data.search.posts[1], "title"));
	assertEquals(data.search.posts[1].title, "Schema Design Patterns");
	assert(Object.hasOwn(data.search.posts[1], "topComments"));
	assertEquals(data.search.posts[1].topComments.length, 1);
	assert(Object.hasOwn(data.search.posts[1].topComments[0], "author"));
	assert(Object.hasOwn(data.search.posts[1].topComments[0].author, "id"));
	assertEquals(data.search.posts[1].topComments[0].author.id, "user-4");
	assert(Object.hasOwn(data.search.posts[1].topComments[0].author, "name"));
	assertEquals(data.search.posts[1].topComments[0].author.name, "Backend Engineer");
	assert(Object.hasOwn(data.search.posts[1].topComments[0], "id"));
	assertEquals(data.search.posts[1].topComments[0].id, "comment-3");
	assert(Object.hasOwn(data.search.posts[1].topComments[0], "likes"));
	assertEquals(data.search.posts[1].topComments[0].likes, 28);
	assert(Object.hasOwn(data.search.posts[1].topComments[0], "text"));
	assertEquals(data.search.posts[1].topComments[0].text, "Excellent breakdown");
	assert(Object.hasOwn(data.search, "total"));
	assertEquals(data.search.total, 42);
	assert(Object.hasOwn(data.search, "users"));
	assertEquals(data.search.users.length, 2);
	assert(Object.hasOwn(data.search.users[0], "email"));
	assertEquals(data.search.users[0].email, "expert@example.com");
	assert(Object.hasOwn(data.search.users[0], "followerCount"));
	assertEquals(data.search.users[0].followerCount.length, 2);
	assert(Object.hasOwn(data.search.users[0].followerCount[0], "id"));
	assertEquals(data.search.users[0].followerCount[0].id, "user-2");
	assert(Object.hasOwn(data.search.users[0].followerCount[1], "id"));
	assertEquals(data.search.users[0].followerCount[1].id, "user-3");
	assert(Object.hasOwn(data.search.users[0], "id"));
	assertEquals(data.search.users[0].id, "user-1");
	assert(Object.hasOwn(data.search.users[0], "name"));
	assertEquals(data.search.users[0].name, "GraphQL Expert");
	assert(Object.hasOwn(data.search.users[0], "profile"));
	assert(Object.hasOwn(data.search.users[0].profile, "avatar"));
	assertEquals(data.search.users[0].profile.avatar, "https://example.com/avatars/expert.jpg");
	assert(Object.hasOwn(data.search.users[0].profile, "bio"));
	assertEquals(data.search.users[0].profile.bio, "GraphQL enthusiast and API designer");
	assert(Object.hasOwn(data.search.users[0].profile, "joinedAt"));
	assertEquals(data.search.users[0].profile.joinedAt, "2024-01-15T08:30:00Z");
	assert(Object.hasOwn(data.search.users[0], "recentPosts"));
	assertEquals(data.search.users[0].recentPosts.length, 3);
	assert(Object.hasOwn(data.search.users[0].recentPosts[0], "id"));
	assertEquals(data.search.users[0].recentPosts[0].id, "post-101");
	assert(Object.hasOwn(data.search.users[0].recentPosts[0], "likes"));
	assertEquals(data.search.users[0].recentPosts[0].likes, 234);
	assert(Object.hasOwn(data.search.users[0].recentPosts[0], "title"));
	assertEquals(data.search.users[0].recentPosts[0].title, "GraphQL Best Practices");
	assert(Object.hasOwn(data.search.users[0].recentPosts[1], "id"));
	assertEquals(data.search.users[0].recentPosts[1].id, "post-102");
	assert(Object.hasOwn(data.search.users[0].recentPosts[1], "likes"));
	assertEquals(data.search.users[0].recentPosts[1].likes, 189);
	assert(Object.hasOwn(data.search.users[0].recentPosts[1], "title"));
	assertEquals(data.search.users[0].recentPosts[1].title, "Schema Design Patterns");
	assert(Object.hasOwn(data.search.users[0].recentPosts[2], "id"));
	assertEquals(data.search.users[0].recentPosts[2].id, "post-103");
	assert(Object.hasOwn(data.search.users[0].recentPosts[2], "likes"));
	assertEquals(data.search.users[0].recentPosts[2].likes, 156);
	assert(Object.hasOwn(data.search.users[0].recentPosts[2], "title"));
	assertEquals(data.search.users[0].recentPosts[2].title, "Performance Optimization");
	assert(Object.hasOwn(data.search.users[1], "email"));
	assertEquals(data.search.users[1].email, "developer@example.com");
	assert(Object.hasOwn(data.search.users[1], "followerCount"));
	assertEquals(data.search.users[1].followerCount.length, 1);
	assert(Object.hasOwn(data.search.users[1].followerCount[0], "id"));
	assertEquals(data.search.users[1].followerCount[0].id, "user-1");
	assert(Object.hasOwn(data.search.users[1], "id"));
	assertEquals(data.search.users[1].id, "user-2");
	assert(Object.hasOwn(data.search.users[1], "name"));
	assertEquals(data.search.users[1].name, "API Developer");
	assert(Object.hasOwn(data.search.users[1], "profile"));
	assert(Object.hasOwn(data.search.users[1].profile, "avatar"));
	assertEquals(data.search.users[1].profile.avatar, "https://example.com/avatars/developer.jpg");
	assert(Object.hasOwn(data.search.users[1].profile, "bio"));
	assertEquals(data.search.users[1].profile.bio, "Building scalable APIs");
	assert(Object.hasOwn(data.search.users[1].profile, "joinedAt"));
	assertEquals(data.search.users[1].profile.joinedAt, "2024-02-20T10:15:00Z");
	assert(Object.hasOwn(data.search.users[1], "recentPosts"));
	assertEquals(data.search.users[1].recentPosts.length, 1);
	assert(Object.hasOwn(data.search.users[1].recentPosts[0], "id"));
	assertEquals(data.search.users[1].recentPosts[0].id, "post-201");
	assert(Object.hasOwn(data.search.users[1].recentPosts[0], "likes"));
	assertEquals(data.search.users[1].recentPosts[0].likes, 145);
	assert(Object.hasOwn(data.search.users[1].recentPosts[0], "title"));
	assertEquals(data.search.users[1].recentPosts[0].title, "GraphQL vs REST");
	const errors = responseBody.errors;
	assertEquals(errors?.length ?? 0, 0);
});

Deno.test("GraphQL query: deeply_nested_query", async () => {
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

	assertEquals(response.statusCode, 200);
	const responseBody = response.json();

	const data = responseBody.data;
	assert(Object.hasOwn(data, "user"));
	assert(Object.hasOwn(data.user, "id"));
	assertEquals(data.user.id, "user-deep-001");
	assert(Object.hasOwn(data.user, "name"));
	assertEquals(data.user.name, "Alice Cooper");
	assert(Object.hasOwn(data.user, "profile"));
	assert(Object.hasOwn(data.user.profile, "bio"));
	assertEquals(data.user.profile.bio, "DevOps engineer passionate about scalability");
	assert(Object.hasOwn(data.user.profile, "settings"));
	assert(Object.hasOwn(data.user.profile.settings, "notifications"));
	assert(Object.hasOwn(data.user.profile.settings.notifications, "email"));
	assertEquals(data.user.profile.settings.notifications.email, true);
	assert(Object.hasOwn(data.user.profile.settings.notifications, "push"));
	assertEquals(data.user.profile.settings.notifications.push, false);
	assert(Object.hasOwn(data.user.profile.settings, "preferences"));
	assert(Object.hasOwn(data.user.profile.settings.preferences, "language"));
	assertEquals(data.user.profile.settings.preferences.language, "en-US");
	assert(Object.hasOwn(data.user.profile.settings.preferences, "theme"));
	assertEquals(data.user.profile.settings.preferences.theme, "dark");
	assert(Object.hasOwn(data.user.profile.settings.preferences, "timezone"));
	assert(Object.hasOwn(data.user.profile.settings.preferences.timezone, "name"));
	assertEquals(data.user.profile.settings.preferences.timezone.name, "America/Los_Angeles");
	assert(Object.hasOwn(data.user.profile.settings.preferences.timezone, "offset"));
	assertEquals(data.user.profile.settings.preferences.timezone.offset, -480);
	const errors = responseBody.errors;
	assertEquals(errors?.length ?? 0, 0);
});

Deno.test("GraphQL query: persisted_query_allowlist", async () => {
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

	assertEquals(response.statusCode, 403);
	const responseBody = response.json();

	const errors = responseBody.errors;
	assert(errors !== undefined);
	assertEquals(errors?.length, 1);
	assert(errors?.[0]?.message.includes("Query not in allowlist"));
});

Deno.test("GraphQL query: persisted_query_hash_mismatch", async () => {
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

	assertEquals(response.statusCode, 400);
	const responseBody = response.json();

	const errors = responseBody.errors;
	assert(errors !== undefined);
	assertEquals(errors?.length, 1);
	assert(errors?.[0]?.message.includes("Hash mismatch"));
});

Deno.test("GraphQL query: persisted_query_registration", async () => {
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

	assertEquals(response.statusCode, 200);
	const responseBody = response.json();

	const data = responseBody.data;
	assert(Object.hasOwn(data, "posts"));
	assertEquals(data.posts.length, 2);
	assert(Object.hasOwn(data.posts[0], "author"));
	assert(Object.hasOwn(data.posts[0].author, "id"));
	assertEquals(data.posts[0].author.id, "user-789");
	assert(Object.hasOwn(data.posts[0].author, "name"));
	assertEquals(data.posts[0].author.name, "Bob Johnson");
	assert(Object.hasOwn(data.posts[0], "content"));
	assertEquals(data.posts[0].content, "Understanding GraphQL query optimization...");
	assert(Object.hasOwn(data.posts[0], "id"));
	assertEquals(data.posts[0].id, "post-1");
	assert(Object.hasOwn(data.posts[0], "title"));
	assertEquals(data.posts[0].title, "GraphQL Best Practices");
	assert(Object.hasOwn(data.posts[1], "author"));
	assert(Object.hasOwn(data.posts[1].author, "id"));
	assertEquals(data.posts[1].author.id, "user-789");
	assert(Object.hasOwn(data.posts[1].author, "name"));
	assertEquals(data.posts[1].author.name, "Bob Johnson");
	assert(Object.hasOwn(data.posts[1], "content"));
	assertEquals(data.posts[1].content, "How to implement persisted queries for performance...");
	assert(Object.hasOwn(data.posts[1], "id"));
	assertEquals(data.posts[1].id, "post-2");
	assert(Object.hasOwn(data.posts[1], "title"));
	assertEquals(data.posts[1].title, "Persisted Queries Guide");
	const errors = responseBody.errors;
	assertEquals(errors?.length ?? 0, 0);
});

Deno.test("GraphQL query: persisted_query_hit", async () => {
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

	assertEquals(response.statusCode, 200);
	const responseBody = response.json();

	const data = responseBody.data;
	assert(Object.hasOwn(data, "user"));
	assert(Object.hasOwn(data.user, "email"));
	assertEquals(data.user.email, "alice@example.com");
	assert(Object.hasOwn(data.user, "id"));
	assertEquals(data.user.id, "user-123");
	assert(Object.hasOwn(data.user, "name"));
	assertEquals(data.user.name, "Alice Smith");
	const errors = responseBody.errors;
	assertEquals(errors?.length ?? 0, 0);
});

Deno.test("GraphQL query: persisted_query_miss", async () => {
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

	assertEquals(response.statusCode, 400);
	const responseBody = response.json();

	const errors = responseBody.errors;
	assert(errors !== undefined);
	assertEquals(errors?.length, 1);
	assert(errors?.[0]?.message.includes("PersistedQueryNotFound"));
});

Deno.test("GraphQL query: persisted_query_automatic_persisted", async () => {
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

	assertEquals(response.statusCode, 400);
	const responseBody = response.json();

	const errors = responseBody.errors;
	assert(errors !== undefined);
	assertEquals(errors?.length, 1);
	assert(errors?.[0]?.message.includes("PersistedQueryNotFound"));
});

Deno.test("GraphQL query: with_arguments", async () => {
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

	assertEquals(response.statusCode, 200);
	const responseBody = response.json();

	const data = responseBody.data;
	assert(Object.hasOwn(data, "greet"));
	assertEquals(data.greet, "Hello, Alice!");
	const errors = responseBody.errors;
	assertEquals(errors?.length ?? 0, 0);
});

Deno.test("GraphQL query: nested_objects", async () => {
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

	assertEquals(response.statusCode, 200);
	const responseBody = response.json();

	const data = responseBody.data;
	assert(Object.hasOwn(data, "user"));
	assert(Object.hasOwn(data.user, "email"));
	assertEquals(data.user.email, "alice@example.com");
	assert(Object.hasOwn(data.user, "id"));
	assertEquals(data.user.id, "550e8400-e29b-41d4-a716-446655440000");
	assert(Object.hasOwn(data.user, "name"));
	assertEquals(data.user.name, "Alice Johnson");
	assert(Object.hasOwn(data.user, "profile"));
	assert(Object.hasOwn(data.user.profile, "bio"));
	assertEquals(data.user.profile.bio, "Software engineer and open source enthusiast");
	assert(Object.hasOwn(data.user.profile, "location"));
	assertEquals(data.user.profile.location, "San Francisco, CA");
	const errors = responseBody.errors;
	assertEquals(errors?.length ?? 0, 0);
});

Deno.test("GraphQL query: simple_field", async () => {
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

	assertEquals(response.statusCode, 200);
	const responseBody = response.json();

	const data = responseBody.data;
	assert(Object.hasOwn(data, "hello"));
	assertEquals(data.hello, "Hello, World!");
	const errors = responseBody.errors;
	assertEquals(errors?.length ?? 0, 0);
});

Deno.test("GraphQL query: introspection_disabled", async () => {
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

	assertEquals(response.statusCode, 400);
	const responseBody = response.json();

	const errors = responseBody.errors;
	assert(errors !== undefined);
	assertEquals(errors?.length, 1);
	assert(errors?.[0]?.message.includes("Introspection is disabled"));
});

Deno.test("GraphQL query: full_schema_introspection", async () => {
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

	assertEquals(response.statusCode, 200);
	const responseBody = response.json();

	const data = responseBody.data;
	assert(Object.hasOwn(data, "__schema"));
	assert(Object.hasOwn(data.__schema, "directives"));
	assertEquals(data.__schema.directives.length, 3);
	assert(Object.hasOwn(data.__schema.directives[0], "args"));
	assertEquals(data.__schema.directives[0].args.length, 1);
	assert(Object.hasOwn(data.__schema.directives[0].args[0], "defaultValue"));
	assertEquals(data.__schema.directives[0].args[0].defaultValue, null);
	assert(Object.hasOwn(data.__schema.directives[0].args[0], "description"));
	assertEquals(data.__schema.directives[0].args[0].description, "Skipped when true");
	assert(Object.hasOwn(data.__schema.directives[0].args[0], "name"));
	assertEquals(data.__schema.directives[0].args[0].name, "if");
	assert(Object.hasOwn(data.__schema.directives[0].args[0], "type"));
	assert(Object.hasOwn(data.__schema.directives[0].args[0].type, "kind"));
	assertEquals(data.__schema.directives[0].args[0].type.kind, "NON_NULL");
	assert(Object.hasOwn(data.__schema.directives[0].args[0].type, "name"));
	assertEquals(data.__schema.directives[0].args[0].type.name, null);
	assert(Object.hasOwn(data.__schema.directives[0].args[0].type, "ofType"));
	assert(Object.hasOwn(data.__schema.directives[0].args[0].type.ofType, "kind"));
	assertEquals(data.__schema.directives[0].args[0].type.ofType.kind, "SCALAR");
	assert(Object.hasOwn(data.__schema.directives[0].args[0].type.ofType, "name"));
	assertEquals(data.__schema.directives[0].args[0].type.ofType.name, "Boolean");
	assert(Object.hasOwn(data.__schema.directives[0], "description"));
	assertEquals(
		data.__schema.directives[0].description,
		"Directs the executor to skip this field or fragment when the `if` argument is true.",
	);
	assert(Object.hasOwn(data.__schema.directives[0], "locations"));
	assertEquals(data.__schema.directives[0].locations.length, 3);
	assertEquals(data.__schema.directives[0].locations[0], "FIELD");
	assertEquals(data.__schema.directives[0].locations[1], "FRAGMENT_SPREAD");
	assertEquals(data.__schema.directives[0].locations[2], "INLINE_FRAGMENT");
	assert(Object.hasOwn(data.__schema.directives[0], "name"));
	assertEquals(data.__schema.directives[0].name, "skip");
	assert(Object.hasOwn(data.__schema.directives[1], "args"));
	assertEquals(data.__schema.directives[1].args.length, 1);
	assert(Object.hasOwn(data.__schema.directives[1].args[0], "defaultValue"));
	assertEquals(data.__schema.directives[1].args[0].defaultValue, null);
	assert(Object.hasOwn(data.__schema.directives[1].args[0], "description"));
	assertEquals(data.__schema.directives[1].args[0].description, "Included when true");
	assert(Object.hasOwn(data.__schema.directives[1].args[0], "name"));
	assertEquals(data.__schema.directives[1].args[0].name, "if");
	assert(Object.hasOwn(data.__schema.directives[1].args[0], "type"));
	assert(Object.hasOwn(data.__schema.directives[1].args[0].type, "kind"));
	assertEquals(data.__schema.directives[1].args[0].type.kind, "NON_NULL");
	assert(Object.hasOwn(data.__schema.directives[1].args[0].type, "name"));
	assertEquals(data.__schema.directives[1].args[0].type.name, null);
	assert(Object.hasOwn(data.__schema.directives[1].args[0].type, "ofType"));
	assert(Object.hasOwn(data.__schema.directives[1].args[0].type.ofType, "kind"));
	assertEquals(data.__schema.directives[1].args[0].type.ofType.kind, "SCALAR");
	assert(Object.hasOwn(data.__schema.directives[1].args[0].type.ofType, "name"));
	assertEquals(data.__schema.directives[1].args[0].type.ofType.name, "Boolean");
	assert(Object.hasOwn(data.__schema.directives[1], "description"));
	assertEquals(
		data.__schema.directives[1].description,
		"Directs the executor to include this field or fragment when the `if` argument is true.",
	);
	assert(Object.hasOwn(data.__schema.directives[1], "locations"));
	assertEquals(data.__schema.directives[1].locations.length, 3);
	assertEquals(data.__schema.directives[1].locations[0], "FIELD");
	assertEquals(data.__schema.directives[1].locations[1], "FRAGMENT_SPREAD");
	assertEquals(data.__schema.directives[1].locations[2], "INLINE_FRAGMENT");
	assert(Object.hasOwn(data.__schema.directives[1], "name"));
	assertEquals(data.__schema.directives[1].name, "include");
	assert(Object.hasOwn(data.__schema.directives[2], "args"));
	assertEquals(data.__schema.directives[2].args.length, 1);
	assert(Object.hasOwn(data.__schema.directives[2].args[0], "defaultValue"));
	assertEquals(data.__schema.directives[2].args[0].defaultValue, "No longer supported");
	assert(Object.hasOwn(data.__schema.directives[2].args[0], "description"));
	assertEquals(data.__schema.directives[2].args[0].description, "Explains why this element was deprecated");
	assert(Object.hasOwn(data.__schema.directives[2].args[0], "name"));
	assertEquals(data.__schema.directives[2].args[0].name, "reason");
	assert(Object.hasOwn(data.__schema.directives[2].args[0], "type"));
	assert(Object.hasOwn(data.__schema.directives[2].args[0].type, "kind"));
	assertEquals(data.__schema.directives[2].args[0].type.kind, "SCALAR");
	assert(Object.hasOwn(data.__schema.directives[2].args[0].type, "name"));
	assertEquals(data.__schema.directives[2].args[0].type.name, "String");
	assert(Object.hasOwn(data.__schema.directives[2], "description"));
	assertEquals(data.__schema.directives[2].description, "Marks an element of a GraphQL schema as no longer supported.");
	assert(Object.hasOwn(data.__schema.directives[2], "locations"));
	assertEquals(data.__schema.directives[2].locations.length, 2);
	assertEquals(data.__schema.directives[2].locations[0], "FIELD_DEFINITION");
	assertEquals(data.__schema.directives[2].locations[1], "ENUM_VALUE");
	assert(Object.hasOwn(data.__schema.directives[2], "name"));
	assertEquals(data.__schema.directives[2].name, "deprecated");
	assert(Object.hasOwn(data.__schema, "mutationType"));
	assert(Object.hasOwn(data.__schema.mutationType, "name"));
	assertEquals(data.__schema.mutationType.name, "Mutation");
	assert(Object.hasOwn(data.__schema, "queryType"));
	assert(Object.hasOwn(data.__schema.queryType, "name"));
	assertEquals(data.__schema.queryType.name, "Query");
	assert(Object.hasOwn(data.__schema, "subscriptionType"));
	assertEquals(data.__schema.subscriptionType, null);
	assert(Object.hasOwn(data.__schema, "types"));
	assertEquals(data.__schema.types.length, 7);
	assert(Object.hasOwn(data.__schema.types[0], "description"));
	assertEquals(data.__schema.types[0].description, "ISO 8601 DateTime scalar");
	assert(Object.hasOwn(data.__schema.types[0], "enumValues"));
	assertEquals(data.__schema.types[0].enumValues, null);
	assert(Object.hasOwn(data.__schema.types[0], "fields"));
	assertEquals(data.__schema.types[0].fields, null);
	assert(Object.hasOwn(data.__schema.types[0], "inputFields"));
	assertEquals(data.__schema.types[0].inputFields, null);
	assert(Object.hasOwn(data.__schema.types[0], "interfaces"));
	assertEquals(data.__schema.types[0].interfaces, null);
	assert(Object.hasOwn(data.__schema.types[0], "kind"));
	assertEquals(data.__schema.types[0].kind, "SCALAR");
	assert(Object.hasOwn(data.__schema.types[0], "name"));
	assertEquals(data.__schema.types[0].name, "DateTime");
	assert(Object.hasOwn(data.__schema.types[0], "possibleTypes"));
	assertEquals(data.__schema.types[0].possibleTypes, null);
	assert(Object.hasOwn(data.__schema.types[1], "description"));
	assertEquals(data.__schema.types[1].description, "UUID scalar type");
	assert(Object.hasOwn(data.__schema.types[1], "enumValues"));
	assertEquals(data.__schema.types[1].enumValues, null);
	assert(Object.hasOwn(data.__schema.types[1], "fields"));
	assertEquals(data.__schema.types[1].fields, null);
	assert(Object.hasOwn(data.__schema.types[1], "inputFields"));
	assertEquals(data.__schema.types[1].inputFields, null);
	assert(Object.hasOwn(data.__schema.types[1], "interfaces"));
	assertEquals(data.__schema.types[1].interfaces, null);
	assert(Object.hasOwn(data.__schema.types[1], "kind"));
	assertEquals(data.__schema.types[1].kind, "SCALAR");
	assert(Object.hasOwn(data.__schema.types[1], "name"));
	assertEquals(data.__schema.types[1].name, "UUID");
	assert(Object.hasOwn(data.__schema.types[1], "possibleTypes"));
	assertEquals(data.__schema.types[1].possibleTypes, null);
	assert(Object.hasOwn(data.__schema.types[2], "description"));
	assertEquals(data.__schema.types[2].description, "Root query type");
	assert(Object.hasOwn(data.__schema.types[2], "enumValues"));
	assertEquals(data.__schema.types[2].enumValues, null);
	assert(Object.hasOwn(data.__schema.types[2], "fields"));
	assertEquals(data.__schema.types[2].fields.length, 4);
	assert(Object.hasOwn(data.__schema.types[2].fields[0], "args"));
	assertEquals(data.__schema.types[2].fields[0].args.length, 0);
	assert(Object.hasOwn(data.__schema.types[2].fields[0], "deprecationReason"));
	assertEquals(data.__schema.types[2].fields[0].deprecationReason, null);
	assert(Object.hasOwn(data.__schema.types[2].fields[0], "description"));
	assertEquals(data.__schema.types[2].fields[0].description, "Greeting message");
	assert(Object.hasOwn(data.__schema.types[2].fields[0], "isDeprecated"));
	assertEquals(data.__schema.types[2].fields[0].isDeprecated, false);
	assert(Object.hasOwn(data.__schema.types[2].fields[0], "name"));
	assertEquals(data.__schema.types[2].fields[0].name, "hello");
	assert(Object.hasOwn(data.__schema.types[2].fields[0], "type"));
	assert(Object.hasOwn(data.__schema.types[2].fields[0].type, "kind"));
	assertEquals(data.__schema.types[2].fields[0].type.kind, "NON_NULL");
	assert(Object.hasOwn(data.__schema.types[2].fields[0].type, "name"));
	assertEquals(data.__schema.types[2].fields[0].type.name, null);
	assert(Object.hasOwn(data.__schema.types[2].fields[0].type, "ofType"));
	assert(Object.hasOwn(data.__schema.types[2].fields[0].type.ofType, "kind"));
	assertEquals(data.__schema.types[2].fields[0].type.ofType.kind, "SCALAR");
	assert(Object.hasOwn(data.__schema.types[2].fields[0].type.ofType, "name"));
	assertEquals(data.__schema.types[2].fields[0].type.ofType.name, "String");
	assert(Object.hasOwn(data.__schema.types[2].fields[1], "args"));
	assertEquals(data.__schema.types[2].fields[1].args.length, 0);
	assert(Object.hasOwn(data.__schema.types[2].fields[1], "deprecationReason"));
	assertEquals(data.__schema.types[2].fields[1].deprecationReason, null);
	assert(Object.hasOwn(data.__schema.types[2].fields[1], "description"));
	assertEquals(data.__schema.types[2].fields[1].description, "API version");
	assert(Object.hasOwn(data.__schema.types[2].fields[1], "isDeprecated"));
	assertEquals(data.__schema.types[2].fields[1].isDeprecated, false);
	assert(Object.hasOwn(data.__schema.types[2].fields[1], "name"));
	assertEquals(data.__schema.types[2].fields[1].name, "version");
	assert(Object.hasOwn(data.__schema.types[2].fields[1], "type"));
	assert(Object.hasOwn(data.__schema.types[2].fields[1].type, "kind"));
	assertEquals(data.__schema.types[2].fields[1].type.kind, "NON_NULL");
	assert(Object.hasOwn(data.__schema.types[2].fields[1].type, "name"));
	assertEquals(data.__schema.types[2].fields[1].type.name, null);
	assert(Object.hasOwn(data.__schema.types[2].fields[1].type, "ofType"));
	assert(Object.hasOwn(data.__schema.types[2].fields[1].type.ofType, "kind"));
	assertEquals(data.__schema.types[2].fields[1].type.ofType.kind, "SCALAR");
	assert(Object.hasOwn(data.__schema.types[2].fields[1].type.ofType, "name"));
	assertEquals(data.__schema.types[2].fields[1].type.ofType.name, "String");
	assert(Object.hasOwn(data.__schema.types[2].fields[2], "args"));
	assertEquals(data.__schema.types[2].fields[2].args.length, 1);
	assert(Object.hasOwn(data.__schema.types[2].fields[2].args[0], "defaultValue"));
	assertEquals(data.__schema.types[2].fields[2].args[0].defaultValue, null);
	assert(Object.hasOwn(data.__schema.types[2].fields[2].args[0], "description"));
	assertEquals(data.__schema.types[2].fields[2].args[0].description, "User ID");
	assert(Object.hasOwn(data.__schema.types[2].fields[2].args[0], "name"));
	assertEquals(data.__schema.types[2].fields[2].args[0].name, "id");
	assert(Object.hasOwn(data.__schema.types[2].fields[2].args[0], "type"));
	assert(Object.hasOwn(data.__schema.types[2].fields[2].args[0].type, "kind"));
	assertEquals(data.__schema.types[2].fields[2].args[0].type.kind, "NON_NULL");
	assert(Object.hasOwn(data.__schema.types[2].fields[2].args[0].type, "name"));
	assertEquals(data.__schema.types[2].fields[2].args[0].type.name, null);
	assert(Object.hasOwn(data.__schema.types[2].fields[2].args[0].type, "ofType"));
	assert(Object.hasOwn(data.__schema.types[2].fields[2].args[0].type.ofType, "kind"));
	assertEquals(data.__schema.types[2].fields[2].args[0].type.ofType.kind, "SCALAR");
	assert(Object.hasOwn(data.__schema.types[2].fields[2].args[0].type.ofType, "name"));
	assertEquals(data.__schema.types[2].fields[2].args[0].type.ofType.name, "UUID");
	assert(Object.hasOwn(data.__schema.types[2].fields[2], "deprecationReason"));
	assertEquals(data.__schema.types[2].fields[2].deprecationReason, null);
	assert(Object.hasOwn(data.__schema.types[2].fields[2], "description"));
	assertEquals(data.__schema.types[2].fields[2].description, "Get user by ID");
	assert(Object.hasOwn(data.__schema.types[2].fields[2], "isDeprecated"));
	assertEquals(data.__schema.types[2].fields[2].isDeprecated, false);
	assert(Object.hasOwn(data.__schema.types[2].fields[2], "name"));
	assertEquals(data.__schema.types[2].fields[2].name, "user");
	assert(Object.hasOwn(data.__schema.types[2].fields[2], "type"));
	assert(Object.hasOwn(data.__schema.types[2].fields[2].type, "kind"));
	assertEquals(data.__schema.types[2].fields[2].type.kind, "OBJECT");
	assert(Object.hasOwn(data.__schema.types[2].fields[2].type, "name"));
	assertEquals(data.__schema.types[2].fields[2].type.name, "User");
	assert(Object.hasOwn(data.__schema.types[2].fields[3], "args"));
	assertEquals(data.__schema.types[2].fields[3].args.length, 2);
	assert(Object.hasOwn(data.__schema.types[2].fields[3].args[0], "defaultValue"));
	assertEquals(data.__schema.types[2].fields[3].args[0].defaultValue, "10");
	assert(Object.hasOwn(data.__schema.types[2].fields[3].args[0], "description"));
	assertEquals(data.__schema.types[2].fields[3].args[0].description, "Maximum number of results");
	assert(Object.hasOwn(data.__schema.types[2].fields[3].args[0], "name"));
	assertEquals(data.__schema.types[2].fields[3].args[0].name, "limit");
	assert(Object.hasOwn(data.__schema.types[2].fields[3].args[0], "type"));
	assert(Object.hasOwn(data.__schema.types[2].fields[3].args[0].type, "kind"));
	assertEquals(data.__schema.types[2].fields[3].args[0].type.kind, "SCALAR");
	assert(Object.hasOwn(data.__schema.types[2].fields[3].args[0].type, "name"));
	assertEquals(data.__schema.types[2].fields[3].args[0].type.name, "Int");
	assert(Object.hasOwn(data.__schema.types[2].fields[3].args[1], "defaultValue"));
	assertEquals(data.__schema.types[2].fields[3].args[1].defaultValue, "0");
	assert(Object.hasOwn(data.__schema.types[2].fields[3].args[1], "description"));
	assertEquals(data.__schema.types[2].fields[3].args[1].description, "Number of results to skip");
	assert(Object.hasOwn(data.__schema.types[2].fields[3].args[1], "name"));
	assertEquals(data.__schema.types[2].fields[3].args[1].name, "offset");
	assert(Object.hasOwn(data.__schema.types[2].fields[3].args[1], "type"));
	assert(Object.hasOwn(data.__schema.types[2].fields[3].args[1].type, "kind"));
	assertEquals(data.__schema.types[2].fields[3].args[1].type.kind, "SCALAR");
	assert(Object.hasOwn(data.__schema.types[2].fields[3].args[1].type, "name"));
	assertEquals(data.__schema.types[2].fields[3].args[1].type.name, "Int");
	assert(Object.hasOwn(data.__schema.types[2].fields[3], "deprecationReason"));
	assertEquals(data.__schema.types[2].fields[3].deprecationReason, null);
	assert(Object.hasOwn(data.__schema.types[2].fields[3], "description"));
	assertEquals(data.__schema.types[2].fields[3].description, "Get all users with pagination");
	assert(Object.hasOwn(data.__schema.types[2].fields[3], "isDeprecated"));
	assertEquals(data.__schema.types[2].fields[3].isDeprecated, false);
	assert(Object.hasOwn(data.__schema.types[2].fields[3], "name"));
	assertEquals(data.__schema.types[2].fields[3].name, "users");
	assert(Object.hasOwn(data.__schema.types[2].fields[3], "type"));
	assert(Object.hasOwn(data.__schema.types[2].fields[3].type, "kind"));
	assertEquals(data.__schema.types[2].fields[3].type.kind, "NON_NULL");
	assert(Object.hasOwn(data.__schema.types[2].fields[3].type, "name"));
	assertEquals(data.__schema.types[2].fields[3].type.name, null);
	assert(Object.hasOwn(data.__schema.types[2].fields[3].type, "ofType"));
	assert(Object.hasOwn(data.__schema.types[2].fields[3].type.ofType, "kind"));
	assertEquals(data.__schema.types[2].fields[3].type.ofType.kind, "LIST");
	assert(Object.hasOwn(data.__schema.types[2].fields[3].type.ofType, "name"));
	assertEquals(data.__schema.types[2].fields[3].type.ofType.name, null);
	assert(Object.hasOwn(data.__schema.types[2].fields[3].type.ofType, "ofType"));
	assert(Object.hasOwn(data.__schema.types[2].fields[3].type.ofType.ofType, "kind"));
	assertEquals(data.__schema.types[2].fields[3].type.ofType.ofType.kind, "NON_NULL");
	assert(Object.hasOwn(data.__schema.types[2].fields[3].type.ofType.ofType, "name"));
	assertEquals(data.__schema.types[2].fields[3].type.ofType.ofType.name, null);
	assert(Object.hasOwn(data.__schema.types[2].fields[3].type.ofType.ofType, "ofType"));
	assert(Object.hasOwn(data.__schema.types[2].fields[3].type.ofType.ofType.ofType, "kind"));
	assertEquals(data.__schema.types[2].fields[3].type.ofType.ofType.ofType.kind, "OBJECT");
	assert(Object.hasOwn(data.__schema.types[2].fields[3].type.ofType.ofType.ofType, "name"));
	assertEquals(data.__schema.types[2].fields[3].type.ofType.ofType.ofType.name, "User");
	assert(Object.hasOwn(data.__schema.types[2], "inputFields"));
	assertEquals(data.__schema.types[2].inputFields, null);
	assert(Object.hasOwn(data.__schema.types[2], "interfaces"));
	assertEquals(data.__schema.types[2].interfaces.length, 0);
	assert(Object.hasOwn(data.__schema.types[2], "kind"));
	assertEquals(data.__schema.types[2].kind, "OBJECT");
	assert(Object.hasOwn(data.__schema.types[2], "name"));
	assertEquals(data.__schema.types[2].name, "Query");
	assert(Object.hasOwn(data.__schema.types[2], "possibleTypes"));
	assertEquals(data.__schema.types[2].possibleTypes, null);
	assert(Object.hasOwn(data.__schema.types[3], "description"));
	assertEquals(data.__schema.types[3].description, "Root mutation type");
	assert(Object.hasOwn(data.__schema.types[3], "enumValues"));
	assertEquals(data.__schema.types[3].enumValues, null);
	assert(Object.hasOwn(data.__schema.types[3], "fields"));
	assertEquals(data.__schema.types[3].fields.length, 3);
	assert(Object.hasOwn(data.__schema.types[3].fields[0], "args"));
	assertEquals(data.__schema.types[3].fields[0].args.length, 1);
	assert(Object.hasOwn(data.__schema.types[3].fields[0].args[0], "defaultValue"));
	assertEquals(data.__schema.types[3].fields[0].args[0].defaultValue, null);
	assert(Object.hasOwn(data.__schema.types[3].fields[0].args[0], "description"));
	assertEquals(data.__schema.types[3].fields[0].args[0].description, "Post creation input");
	assert(Object.hasOwn(data.__schema.types[3].fields[0].args[0], "name"));
	assertEquals(data.__schema.types[3].fields[0].args[0].name, "input");
	assert(Object.hasOwn(data.__schema.types[3].fields[0].args[0], "type"));
	assert(Object.hasOwn(data.__schema.types[3].fields[0].args[0].type, "kind"));
	assertEquals(data.__schema.types[3].fields[0].args[0].type.kind, "NON_NULL");
	assert(Object.hasOwn(data.__schema.types[3].fields[0].args[0].type, "name"));
	assertEquals(data.__schema.types[3].fields[0].args[0].type.name, null);
	assert(Object.hasOwn(data.__schema.types[3].fields[0].args[0].type, "ofType"));
	assert(Object.hasOwn(data.__schema.types[3].fields[0].args[0].type.ofType, "kind"));
	assertEquals(data.__schema.types[3].fields[0].args[0].type.ofType.kind, "INPUT_OBJECT");
	assert(Object.hasOwn(data.__schema.types[3].fields[0].args[0].type.ofType, "name"));
	assertEquals(data.__schema.types[3].fields[0].args[0].type.ofType.name, "CreatePostInput");
	assert(Object.hasOwn(data.__schema.types[3].fields[0], "deprecationReason"));
	assertEquals(data.__schema.types[3].fields[0].deprecationReason, null);
	assert(Object.hasOwn(data.__schema.types[3].fields[0], "description"));
	assertEquals(data.__schema.types[3].fields[0].description, "Create a new post");
	assert(Object.hasOwn(data.__schema.types[3].fields[0], "isDeprecated"));
	assertEquals(data.__schema.types[3].fields[0].isDeprecated, false);
	assert(Object.hasOwn(data.__schema.types[3].fields[0], "name"));
	assertEquals(data.__schema.types[3].fields[0].name, "createPost");
	assert(Object.hasOwn(data.__schema.types[3].fields[0], "type"));
	assert(Object.hasOwn(data.__schema.types[3].fields[0].type, "kind"));
	assertEquals(data.__schema.types[3].fields[0].type.kind, "NON_NULL");
	assert(Object.hasOwn(data.__schema.types[3].fields[0].type, "name"));
	assertEquals(data.__schema.types[3].fields[0].type.name, null);
	assert(Object.hasOwn(data.__schema.types[3].fields[0].type, "ofType"));
	assert(Object.hasOwn(data.__schema.types[3].fields[0].type.ofType, "kind"));
	assertEquals(data.__schema.types[3].fields[0].type.ofType.kind, "OBJECT");
	assert(Object.hasOwn(data.__schema.types[3].fields[0].type.ofType, "name"));
	assertEquals(data.__schema.types[3].fields[0].type.ofType.name, "Post");
	assert(Object.hasOwn(data.__schema.types[3].fields[1], "args"));
	assertEquals(data.__schema.types[3].fields[1].args.length, 2);
	assert(Object.hasOwn(data.__schema.types[3].fields[1].args[0], "defaultValue"));
	assertEquals(data.__schema.types[3].fields[1].args[0].defaultValue, null);
	assert(Object.hasOwn(data.__schema.types[3].fields[1].args[0], "description"));
	assertEquals(data.__schema.types[3].fields[1].args[0].description, "User ID");
	assert(Object.hasOwn(data.__schema.types[3].fields[1].args[0], "name"));
	assertEquals(data.__schema.types[3].fields[1].args[0].name, "id");
	assert(Object.hasOwn(data.__schema.types[3].fields[1].args[0], "type"));
	assert(Object.hasOwn(data.__schema.types[3].fields[1].args[0].type, "kind"));
	assertEquals(data.__schema.types[3].fields[1].args[0].type.kind, "NON_NULL");
	assert(Object.hasOwn(data.__schema.types[3].fields[1].args[0].type, "name"));
	assertEquals(data.__schema.types[3].fields[1].args[0].type.name, null);
	assert(Object.hasOwn(data.__schema.types[3].fields[1].args[0].type, "ofType"));
	assert(Object.hasOwn(data.__schema.types[3].fields[1].args[0].type.ofType, "kind"));
	assertEquals(data.__schema.types[3].fields[1].args[0].type.ofType.kind, "SCALAR");
	assert(Object.hasOwn(data.__schema.types[3].fields[1].args[0].type.ofType, "name"));
	assertEquals(data.__schema.types[3].fields[1].args[0].type.ofType.name, "UUID");
	assert(Object.hasOwn(data.__schema.types[3].fields[1].args[1], "defaultValue"));
	assertEquals(data.__schema.types[3].fields[1].args[1].defaultValue, null);
	assert(Object.hasOwn(data.__schema.types[3].fields[1].args[1], "description"));
	assertEquals(data.__schema.types[3].fields[1].args[1].description, "New user name");
	assert(Object.hasOwn(data.__schema.types[3].fields[1].args[1], "name"));
	assertEquals(data.__schema.types[3].fields[1].args[1].name, "name");
	assert(Object.hasOwn(data.__schema.types[3].fields[1].args[1], "type"));
	assert(Object.hasOwn(data.__schema.types[3].fields[1].args[1].type, "kind"));
	assertEquals(data.__schema.types[3].fields[1].args[1].type.kind, "NON_NULL");
	assert(Object.hasOwn(data.__schema.types[3].fields[1].args[1].type, "name"));
	assertEquals(data.__schema.types[3].fields[1].args[1].type.name, null);
	assert(Object.hasOwn(data.__schema.types[3].fields[1].args[1].type, "ofType"));
	assert(Object.hasOwn(data.__schema.types[3].fields[1].args[1].type.ofType, "kind"));
	assertEquals(data.__schema.types[3].fields[1].args[1].type.ofType.kind, "SCALAR");
	assert(Object.hasOwn(data.__schema.types[3].fields[1].args[1].type.ofType, "name"));
	assertEquals(data.__schema.types[3].fields[1].args[1].type.ofType.name, "String");
	assert(Object.hasOwn(data.__schema.types[3].fields[1], "deprecationReason"));
	assertEquals(data.__schema.types[3].fields[1].deprecationReason, null);
	assert(Object.hasOwn(data.__schema.types[3].fields[1], "description"));
	assertEquals(data.__schema.types[3].fields[1].description, "Update user information");
	assert(Object.hasOwn(data.__schema.types[3].fields[1], "isDeprecated"));
	assertEquals(data.__schema.types[3].fields[1].isDeprecated, false);
	assert(Object.hasOwn(data.__schema.types[3].fields[1], "name"));
	assertEquals(data.__schema.types[3].fields[1].name, "updateUser");
	assert(Object.hasOwn(data.__schema.types[3].fields[1], "type"));
	assert(Object.hasOwn(data.__schema.types[3].fields[1].type, "kind"));
	assertEquals(data.__schema.types[3].fields[1].type.kind, "NON_NULL");
	assert(Object.hasOwn(data.__schema.types[3].fields[1].type, "name"));
	assertEquals(data.__schema.types[3].fields[1].type.name, null);
	assert(Object.hasOwn(data.__schema.types[3].fields[1].type, "ofType"));
	assert(Object.hasOwn(data.__schema.types[3].fields[1].type.ofType, "kind"));
	assertEquals(data.__schema.types[3].fields[1].type.ofType.kind, "OBJECT");
	assert(Object.hasOwn(data.__schema.types[3].fields[1].type.ofType, "name"));
	assertEquals(data.__schema.types[3].fields[1].type.ofType.name, "User");
	assert(Object.hasOwn(data.__schema.types[3].fields[2], "args"));
	assertEquals(data.__schema.types[3].fields[2].args.length, 1);
	assert(Object.hasOwn(data.__schema.types[3].fields[2].args[0], "defaultValue"));
	assertEquals(data.__schema.types[3].fields[2].args[0].defaultValue, null);
	assert(Object.hasOwn(data.__schema.types[3].fields[2].args[0], "description"));
	assertEquals(data.__schema.types[3].fields[2].args[0].description, "Post ID");
	assert(Object.hasOwn(data.__schema.types[3].fields[2].args[0], "name"));
	assertEquals(data.__schema.types[3].fields[2].args[0].name, "id");
	assert(Object.hasOwn(data.__schema.types[3].fields[2].args[0], "type"));
	assert(Object.hasOwn(data.__schema.types[3].fields[2].args[0].type, "kind"));
	assertEquals(data.__schema.types[3].fields[2].args[0].type.kind, "NON_NULL");
	assert(Object.hasOwn(data.__schema.types[3].fields[2].args[0].type, "name"));
	assertEquals(data.__schema.types[3].fields[2].args[0].type.name, null);
	assert(Object.hasOwn(data.__schema.types[3].fields[2].args[0].type, "ofType"));
	assert(Object.hasOwn(data.__schema.types[3].fields[2].args[0].type.ofType, "kind"));
	assertEquals(data.__schema.types[3].fields[2].args[0].type.ofType.kind, "SCALAR");
	assert(Object.hasOwn(data.__schema.types[3].fields[2].args[0].type.ofType, "name"));
	assertEquals(data.__schema.types[3].fields[2].args[0].type.ofType.name, "UUID");
	assert(Object.hasOwn(data.__schema.types[3].fields[2], "deprecationReason"));
	assertEquals(data.__schema.types[3].fields[2].deprecationReason, null);
	assert(Object.hasOwn(data.__schema.types[3].fields[2], "description"));
	assertEquals(data.__schema.types[3].fields[2].description, "Delete a post");
	assert(Object.hasOwn(data.__schema.types[3].fields[2], "isDeprecated"));
	assertEquals(data.__schema.types[3].fields[2].isDeprecated, false);
	assert(Object.hasOwn(data.__schema.types[3].fields[2], "name"));
	assertEquals(data.__schema.types[3].fields[2].name, "deletePost");
	assert(Object.hasOwn(data.__schema.types[3].fields[2], "type"));
	assert(Object.hasOwn(data.__schema.types[3].fields[2].type, "kind"));
	assertEquals(data.__schema.types[3].fields[2].type.kind, "NON_NULL");
	assert(Object.hasOwn(data.__schema.types[3].fields[2].type, "name"));
	assertEquals(data.__schema.types[3].fields[2].type.name, null);
	assert(Object.hasOwn(data.__schema.types[3].fields[2].type, "ofType"));
	assert(Object.hasOwn(data.__schema.types[3].fields[2].type.ofType, "kind"));
	assertEquals(data.__schema.types[3].fields[2].type.ofType.kind, "SCALAR");
	assert(Object.hasOwn(data.__schema.types[3].fields[2].type.ofType, "name"));
	assertEquals(data.__schema.types[3].fields[2].type.ofType.name, "Boolean");
	assert(Object.hasOwn(data.__schema.types[3], "inputFields"));
	assertEquals(data.__schema.types[3].inputFields, null);
	assert(Object.hasOwn(data.__schema.types[3], "interfaces"));
	assertEquals(data.__schema.types[3].interfaces.length, 0);
	assert(Object.hasOwn(data.__schema.types[3], "kind"));
	assertEquals(data.__schema.types[3].kind, "OBJECT");
	assert(Object.hasOwn(data.__schema.types[3], "name"));
	assertEquals(data.__schema.types[3].name, "Mutation");
	assert(Object.hasOwn(data.__schema.types[3], "possibleTypes"));
	assertEquals(data.__schema.types[3].possibleTypes, null);
	assert(Object.hasOwn(data.__schema.types[4], "description"));
	assertEquals(data.__schema.types[4].description, "User entity");
	assert(Object.hasOwn(data.__schema.types[4], "enumValues"));
	assertEquals(data.__schema.types[4].enumValues, null);
	assert(Object.hasOwn(data.__schema.types[4], "fields"));
	assertEquals(data.__schema.types[4].fields.length, 5);
	assert(Object.hasOwn(data.__schema.types[4].fields[0], "args"));
	assertEquals(data.__schema.types[4].fields[0].args.length, 0);
	assert(Object.hasOwn(data.__schema.types[4].fields[0], "deprecationReason"));
	assertEquals(data.__schema.types[4].fields[0].deprecationReason, null);
	assert(Object.hasOwn(data.__schema.types[4].fields[0], "description"));
	assertEquals(data.__schema.types[4].fields[0].description, "Unique identifier");
	assert(Object.hasOwn(data.__schema.types[4].fields[0], "isDeprecated"));
	assertEquals(data.__schema.types[4].fields[0].isDeprecated, false);
	assert(Object.hasOwn(data.__schema.types[4].fields[0], "name"));
	assertEquals(data.__schema.types[4].fields[0].name, "id");
	assert(Object.hasOwn(data.__schema.types[4].fields[0], "type"));
	assert(Object.hasOwn(data.__schema.types[4].fields[0].type, "kind"));
	assertEquals(data.__schema.types[4].fields[0].type.kind, "NON_NULL");
	assert(Object.hasOwn(data.__schema.types[4].fields[0].type, "name"));
	assertEquals(data.__schema.types[4].fields[0].type.name, null);
	assert(Object.hasOwn(data.__schema.types[4].fields[0].type, "ofType"));
	assert(Object.hasOwn(data.__schema.types[4].fields[0].type.ofType, "kind"));
	assertEquals(data.__schema.types[4].fields[0].type.ofType.kind, "SCALAR");
	assert(Object.hasOwn(data.__schema.types[4].fields[0].type.ofType, "name"));
	assertEquals(data.__schema.types[4].fields[0].type.ofType.name, "UUID");
	assert(Object.hasOwn(data.__schema.types[4].fields[1], "args"));
	assertEquals(data.__schema.types[4].fields[1].args.length, 0);
	assert(Object.hasOwn(data.__schema.types[4].fields[1], "deprecationReason"));
	assertEquals(data.__schema.types[4].fields[1].deprecationReason, null);
	assert(Object.hasOwn(data.__schema.types[4].fields[1], "description"));
	assertEquals(data.__schema.types[4].fields[1].description, "User's full name");
	assert(Object.hasOwn(data.__schema.types[4].fields[1], "isDeprecated"));
	assertEquals(data.__schema.types[4].fields[1].isDeprecated, false);
	assert(Object.hasOwn(data.__schema.types[4].fields[1], "name"));
	assertEquals(data.__schema.types[4].fields[1].name, "name");
	assert(Object.hasOwn(data.__schema.types[4].fields[1], "type"));
	assert(Object.hasOwn(data.__schema.types[4].fields[1].type, "kind"));
	assertEquals(data.__schema.types[4].fields[1].type.kind, "NON_NULL");
	assert(Object.hasOwn(data.__schema.types[4].fields[1].type, "name"));
	assertEquals(data.__schema.types[4].fields[1].type.name, null);
	assert(Object.hasOwn(data.__schema.types[4].fields[1].type, "ofType"));
	assert(Object.hasOwn(data.__schema.types[4].fields[1].type.ofType, "kind"));
	assertEquals(data.__schema.types[4].fields[1].type.ofType.kind, "SCALAR");
	assert(Object.hasOwn(data.__schema.types[4].fields[1].type.ofType, "name"));
	assertEquals(data.__schema.types[4].fields[1].type.ofType.name, "String");
	assert(Object.hasOwn(data.__schema.types[4].fields[2], "args"));
	assertEquals(data.__schema.types[4].fields[2].args.length, 0);
	assert(Object.hasOwn(data.__schema.types[4].fields[2], "deprecationReason"));
	assertEquals(data.__schema.types[4].fields[2].deprecationReason, null);
	assert(Object.hasOwn(data.__schema.types[4].fields[2], "description"));
	assertEquals(data.__schema.types[4].fields[2].description, "User's email address");
	assert(Object.hasOwn(data.__schema.types[4].fields[2], "isDeprecated"));
	assertEquals(data.__schema.types[4].fields[2].isDeprecated, false);
	assert(Object.hasOwn(data.__schema.types[4].fields[2], "name"));
	assertEquals(data.__schema.types[4].fields[2].name, "email");
	assert(Object.hasOwn(data.__schema.types[4].fields[2], "type"));
	assert(Object.hasOwn(data.__schema.types[4].fields[2].type, "kind"));
	assertEquals(data.__schema.types[4].fields[2].type.kind, "NON_NULL");
	assert(Object.hasOwn(data.__schema.types[4].fields[2].type, "name"));
	assertEquals(data.__schema.types[4].fields[2].type.name, null);
	assert(Object.hasOwn(data.__schema.types[4].fields[2].type, "ofType"));
	assert(Object.hasOwn(data.__schema.types[4].fields[2].type.ofType, "kind"));
	assertEquals(data.__schema.types[4].fields[2].type.ofType.kind, "SCALAR");
	assert(Object.hasOwn(data.__schema.types[4].fields[2].type.ofType, "name"));
	assertEquals(data.__schema.types[4].fields[2].type.ofType.name, "String");
	assert(Object.hasOwn(data.__schema.types[4].fields[3], "args"));
	assertEquals(data.__schema.types[4].fields[3].args.length, 0);
	assert(Object.hasOwn(data.__schema.types[4].fields[3], "deprecationReason"));
	assertEquals(data.__schema.types[4].fields[3].deprecationReason, null);
	assert(Object.hasOwn(data.__schema.types[4].fields[3], "description"));
	assertEquals(data.__schema.types[4].fields[3].description, "Creation timestamp");
	assert(Object.hasOwn(data.__schema.types[4].fields[3], "isDeprecated"));
	assertEquals(data.__schema.types[4].fields[3].isDeprecated, false);
	assert(Object.hasOwn(data.__schema.types[4].fields[3], "name"));
	assertEquals(data.__schema.types[4].fields[3].name, "createdAt");
	assert(Object.hasOwn(data.__schema.types[4].fields[3], "type"));
	assert(Object.hasOwn(data.__schema.types[4].fields[3].type, "kind"));
	assertEquals(data.__schema.types[4].fields[3].type.kind, "NON_NULL");
	assert(Object.hasOwn(data.__schema.types[4].fields[3].type, "name"));
	assertEquals(data.__schema.types[4].fields[3].type.name, null);
	assert(Object.hasOwn(data.__schema.types[4].fields[3].type, "ofType"));
	assert(Object.hasOwn(data.__schema.types[4].fields[3].type.ofType, "kind"));
	assertEquals(data.__schema.types[4].fields[3].type.ofType.kind, "SCALAR");
	assert(Object.hasOwn(data.__schema.types[4].fields[3].type.ofType, "name"));
	assertEquals(data.__schema.types[4].fields[3].type.ofType.name, "DateTime");
	assert(Object.hasOwn(data.__schema.types[4].fields[4], "args"));
	assertEquals(data.__schema.types[4].fields[4].args.length, 0);
	assert(Object.hasOwn(data.__schema.types[4].fields[4], "deprecationReason"));
	assertEquals(data.__schema.types[4].fields[4].deprecationReason, null);
	assert(Object.hasOwn(data.__schema.types[4].fields[4], "description"));
	assertEquals(data.__schema.types[4].fields[4].description, "User's posts");
	assert(Object.hasOwn(data.__schema.types[4].fields[4], "isDeprecated"));
	assertEquals(data.__schema.types[4].fields[4].isDeprecated, false);
	assert(Object.hasOwn(data.__schema.types[4].fields[4], "name"));
	assertEquals(data.__schema.types[4].fields[4].name, "posts");
	assert(Object.hasOwn(data.__schema.types[4].fields[4], "type"));
	assert(Object.hasOwn(data.__schema.types[4].fields[4].type, "kind"));
	assertEquals(data.__schema.types[4].fields[4].type.kind, "NON_NULL");
	assert(Object.hasOwn(data.__schema.types[4].fields[4].type, "name"));
	assertEquals(data.__schema.types[4].fields[4].type.name, null);
	assert(Object.hasOwn(data.__schema.types[4].fields[4].type, "ofType"));
	assert(Object.hasOwn(data.__schema.types[4].fields[4].type.ofType, "kind"));
	assertEquals(data.__schema.types[4].fields[4].type.ofType.kind, "LIST");
	assert(Object.hasOwn(data.__schema.types[4].fields[4].type.ofType, "name"));
	assertEquals(data.__schema.types[4].fields[4].type.ofType.name, null);
	assert(Object.hasOwn(data.__schema.types[4].fields[4].type.ofType, "ofType"));
	assert(Object.hasOwn(data.__schema.types[4].fields[4].type.ofType.ofType, "kind"));
	assertEquals(data.__schema.types[4].fields[4].type.ofType.ofType.kind, "NON_NULL");
	assert(Object.hasOwn(data.__schema.types[4].fields[4].type.ofType.ofType, "name"));
	assertEquals(data.__schema.types[4].fields[4].type.ofType.ofType.name, null);
	assert(Object.hasOwn(data.__schema.types[4].fields[4].type.ofType.ofType, "ofType"));
	assert(Object.hasOwn(data.__schema.types[4].fields[4].type.ofType.ofType.ofType, "kind"));
	assertEquals(data.__schema.types[4].fields[4].type.ofType.ofType.ofType.kind, "OBJECT");
	assert(Object.hasOwn(data.__schema.types[4].fields[4].type.ofType.ofType.ofType, "name"));
	assertEquals(data.__schema.types[4].fields[4].type.ofType.ofType.ofType.name, "Post");
	assert(Object.hasOwn(data.__schema.types[4], "inputFields"));
	assertEquals(data.__schema.types[4].inputFields, null);
	assert(Object.hasOwn(data.__schema.types[4], "interfaces"));
	assertEquals(data.__schema.types[4].interfaces.length, 0);
	assert(Object.hasOwn(data.__schema.types[4], "kind"));
	assertEquals(data.__schema.types[4].kind, "OBJECT");
	assert(Object.hasOwn(data.__schema.types[4], "name"));
	assertEquals(data.__schema.types[4].name, "User");
	assert(Object.hasOwn(data.__schema.types[4], "possibleTypes"));
	assertEquals(data.__schema.types[4].possibleTypes, null);
	assert(Object.hasOwn(data.__schema.types[5], "description"));
	assertEquals(data.__schema.types[5].description, "Blog post entity");
	assert(Object.hasOwn(data.__schema.types[5], "enumValues"));
	assertEquals(data.__schema.types[5].enumValues, null);
	assert(Object.hasOwn(data.__schema.types[5], "fields"));
	assertEquals(data.__schema.types[5].fields.length, 7);
	assert(Object.hasOwn(data.__schema.types[5].fields[0], "args"));
	assertEquals(data.__schema.types[5].fields[0].args.length, 0);
	assert(Object.hasOwn(data.__schema.types[5].fields[0], "deprecationReason"));
	assertEquals(data.__schema.types[5].fields[0].deprecationReason, null);
	assert(Object.hasOwn(data.__schema.types[5].fields[0], "description"));
	assertEquals(data.__schema.types[5].fields[0].description, "Unique identifier");
	assert(Object.hasOwn(data.__schema.types[5].fields[0], "isDeprecated"));
	assertEquals(data.__schema.types[5].fields[0].isDeprecated, false);
	assert(Object.hasOwn(data.__schema.types[5].fields[0], "name"));
	assertEquals(data.__schema.types[5].fields[0].name, "id");
	assert(Object.hasOwn(data.__schema.types[5].fields[0], "type"));
	assert(Object.hasOwn(data.__schema.types[5].fields[0].type, "kind"));
	assertEquals(data.__schema.types[5].fields[0].type.kind, "NON_NULL");
	assert(Object.hasOwn(data.__schema.types[5].fields[0].type, "name"));
	assertEquals(data.__schema.types[5].fields[0].type.name, null);
	assert(Object.hasOwn(data.__schema.types[5].fields[0].type, "ofType"));
	assert(Object.hasOwn(data.__schema.types[5].fields[0].type.ofType, "kind"));
	assertEquals(data.__schema.types[5].fields[0].type.ofType.kind, "SCALAR");
	assert(Object.hasOwn(data.__schema.types[5].fields[0].type.ofType, "name"));
	assertEquals(data.__schema.types[5].fields[0].type.ofType.name, "UUID");
	assert(Object.hasOwn(data.__schema.types[5].fields[1], "args"));
	assertEquals(data.__schema.types[5].fields[1].args.length, 0);
	assert(Object.hasOwn(data.__schema.types[5].fields[1], "deprecationReason"));
	assertEquals(data.__schema.types[5].fields[1].deprecationReason, null);
	assert(Object.hasOwn(data.__schema.types[5].fields[1], "description"));
	assertEquals(data.__schema.types[5].fields[1].description, "Post title");
	assert(Object.hasOwn(data.__schema.types[5].fields[1], "isDeprecated"));
	assertEquals(data.__schema.types[5].fields[1].isDeprecated, false);
	assert(Object.hasOwn(data.__schema.types[5].fields[1], "name"));
	assertEquals(data.__schema.types[5].fields[1].name, "title");
	assert(Object.hasOwn(data.__schema.types[5].fields[1], "type"));
	assert(Object.hasOwn(data.__schema.types[5].fields[1].type, "kind"));
	assertEquals(data.__schema.types[5].fields[1].type.kind, "NON_NULL");
	assert(Object.hasOwn(data.__schema.types[5].fields[1].type, "name"));
	assertEquals(data.__schema.types[5].fields[1].type.name, null);
	assert(Object.hasOwn(data.__schema.types[5].fields[1].type, "ofType"));
	assert(Object.hasOwn(data.__schema.types[5].fields[1].type.ofType, "kind"));
	assertEquals(data.__schema.types[5].fields[1].type.ofType.kind, "SCALAR");
	assert(Object.hasOwn(data.__schema.types[5].fields[1].type.ofType, "name"));
	assertEquals(data.__schema.types[5].fields[1].type.ofType.name, "String");
	assert(Object.hasOwn(data.__schema.types[5].fields[2], "args"));
	assertEquals(data.__schema.types[5].fields[2].args.length, 0);
	assert(Object.hasOwn(data.__schema.types[5].fields[2], "deprecationReason"));
	assertEquals(data.__schema.types[5].fields[2].deprecationReason, null);
	assert(Object.hasOwn(data.__schema.types[5].fields[2], "description"));
	assertEquals(data.__schema.types[5].fields[2].description, "Post content");
	assert(Object.hasOwn(data.__schema.types[5].fields[2], "isDeprecated"));
	assertEquals(data.__schema.types[5].fields[2].isDeprecated, false);
	assert(Object.hasOwn(data.__schema.types[5].fields[2], "name"));
	assertEquals(data.__schema.types[5].fields[2].name, "content");
	assert(Object.hasOwn(data.__schema.types[5].fields[2], "type"));
	assert(Object.hasOwn(data.__schema.types[5].fields[2].type, "kind"));
	assertEquals(data.__schema.types[5].fields[2].type.kind, "NON_NULL");
	assert(Object.hasOwn(data.__schema.types[5].fields[2].type, "name"));
	assertEquals(data.__schema.types[5].fields[2].type.name, null);
	assert(Object.hasOwn(data.__schema.types[5].fields[2].type, "ofType"));
	assert(Object.hasOwn(data.__schema.types[5].fields[2].type.ofType, "kind"));
	assertEquals(data.__schema.types[5].fields[2].type.ofType.kind, "SCALAR");
	assert(Object.hasOwn(data.__schema.types[5].fields[2].type.ofType, "name"));
	assertEquals(data.__schema.types[5].fields[2].type.ofType.name, "String");
	assert(Object.hasOwn(data.__schema.types[5].fields[3], "args"));
	assertEquals(data.__schema.types[5].fields[3].args.length, 0);
	assert(Object.hasOwn(data.__schema.types[5].fields[3], "deprecationReason"));
	assertEquals(data.__schema.types[5].fields[3].deprecationReason, null);
	assert(Object.hasOwn(data.__schema.types[5].fields[3], "description"));
	assertEquals(data.__schema.types[5].fields[3].description, "Author's ID");
	assert(Object.hasOwn(data.__schema.types[5].fields[3], "isDeprecated"));
	assertEquals(data.__schema.types[5].fields[3].isDeprecated, false);
	assert(Object.hasOwn(data.__schema.types[5].fields[3], "name"));
	assertEquals(data.__schema.types[5].fields[3].name, "authorId");
	assert(Object.hasOwn(data.__schema.types[5].fields[3], "type"));
	assert(Object.hasOwn(data.__schema.types[5].fields[3].type, "kind"));
	assertEquals(data.__schema.types[5].fields[3].type.kind, "NON_NULL");
	assert(Object.hasOwn(data.__schema.types[5].fields[3].type, "name"));
	assertEquals(data.__schema.types[5].fields[3].type.name, null);
	assert(Object.hasOwn(data.__schema.types[5].fields[3].type, "ofType"));
	assert(Object.hasOwn(data.__schema.types[5].fields[3].type.ofType, "kind"));
	assertEquals(data.__schema.types[5].fields[3].type.ofType.kind, "SCALAR");
	assert(Object.hasOwn(data.__schema.types[5].fields[3].type.ofType, "name"));
	assertEquals(data.__schema.types[5].fields[3].type.ofType.name, "UUID");
	assert(Object.hasOwn(data.__schema.types[5].fields[4], "args"));
	assertEquals(data.__schema.types[5].fields[4].args.length, 0);
	assert(Object.hasOwn(data.__schema.types[5].fields[4], "deprecationReason"));
	assertEquals(data.__schema.types[5].fields[4].deprecationReason, null);
	assert(Object.hasOwn(data.__schema.types[5].fields[4], "description"));
	assertEquals(data.__schema.types[5].fields[4].description, "Post author");
	assert(Object.hasOwn(data.__schema.types[5].fields[4], "isDeprecated"));
	assertEquals(data.__schema.types[5].fields[4].isDeprecated, false);
	assert(Object.hasOwn(data.__schema.types[5].fields[4], "name"));
	assertEquals(data.__schema.types[5].fields[4].name, "author");
	assert(Object.hasOwn(data.__schema.types[5].fields[4], "type"));
	assert(Object.hasOwn(data.__schema.types[5].fields[4].type, "kind"));
	assertEquals(data.__schema.types[5].fields[4].type.kind, "NON_NULL");
	assert(Object.hasOwn(data.__schema.types[5].fields[4].type, "name"));
	assertEquals(data.__schema.types[5].fields[4].type.name, null);
	assert(Object.hasOwn(data.__schema.types[5].fields[4].type, "ofType"));
	assert(Object.hasOwn(data.__schema.types[5].fields[4].type.ofType, "kind"));
	assertEquals(data.__schema.types[5].fields[4].type.ofType.kind, "OBJECT");
	assert(Object.hasOwn(data.__schema.types[5].fields[4].type.ofType, "name"));
	assertEquals(data.__schema.types[5].fields[4].type.ofType.name, "User");
	assert(Object.hasOwn(data.__schema.types[5].fields[5], "args"));
	assertEquals(data.__schema.types[5].fields[5].args.length, 0);
	assert(Object.hasOwn(data.__schema.types[5].fields[5], "deprecationReason"));
	assertEquals(data.__schema.types[5].fields[5].deprecationReason, null);
	assert(Object.hasOwn(data.__schema.types[5].fields[5], "description"));
	assertEquals(data.__schema.types[5].fields[5].description, "Creation timestamp");
	assert(Object.hasOwn(data.__schema.types[5].fields[5], "isDeprecated"));
	assertEquals(data.__schema.types[5].fields[5].isDeprecated, false);
	assert(Object.hasOwn(data.__schema.types[5].fields[5], "name"));
	assertEquals(data.__schema.types[5].fields[5].name, "createdAt");
	assert(Object.hasOwn(data.__schema.types[5].fields[5], "type"));
	assert(Object.hasOwn(data.__schema.types[5].fields[5].type, "kind"));
	assertEquals(data.__schema.types[5].fields[5].type.kind, "NON_NULL");
	assert(Object.hasOwn(data.__schema.types[5].fields[5].type, "name"));
	assertEquals(data.__schema.types[5].fields[5].type.name, null);
	assert(Object.hasOwn(data.__schema.types[5].fields[5].type, "ofType"));
	assert(Object.hasOwn(data.__schema.types[5].fields[5].type.ofType, "kind"));
	assertEquals(data.__schema.types[5].fields[5].type.ofType.kind, "SCALAR");
	assert(Object.hasOwn(data.__schema.types[5].fields[5].type.ofType, "name"));
	assertEquals(data.__schema.types[5].fields[5].type.ofType.name, "DateTime");
	assert(Object.hasOwn(data.__schema.types[5].fields[6], "args"));
	assertEquals(data.__schema.types[5].fields[6].args.length, 0);
	assert(Object.hasOwn(data.__schema.types[5].fields[6], "deprecationReason"));
	assertEquals(data.__schema.types[5].fields[6].deprecationReason, null);
	assert(Object.hasOwn(data.__schema.types[5].fields[6], "description"));
	assertEquals(data.__schema.types[5].fields[6].description, "Last update timestamp");
	assert(Object.hasOwn(data.__schema.types[5].fields[6], "isDeprecated"));
	assertEquals(data.__schema.types[5].fields[6].isDeprecated, false);
	assert(Object.hasOwn(data.__schema.types[5].fields[6], "name"));
	assertEquals(data.__schema.types[5].fields[6].name, "updatedAt");
	assert(Object.hasOwn(data.__schema.types[5].fields[6], "type"));
	assert(Object.hasOwn(data.__schema.types[5].fields[6].type, "kind"));
	assertEquals(data.__schema.types[5].fields[6].type.kind, "NON_NULL");
	assert(Object.hasOwn(data.__schema.types[5].fields[6].type, "name"));
	assertEquals(data.__schema.types[5].fields[6].type.name, null);
	assert(Object.hasOwn(data.__schema.types[5].fields[6].type, "ofType"));
	assert(Object.hasOwn(data.__schema.types[5].fields[6].type.ofType, "kind"));
	assertEquals(data.__schema.types[5].fields[6].type.ofType.kind, "SCALAR");
	assert(Object.hasOwn(data.__schema.types[5].fields[6].type.ofType, "name"));
	assertEquals(data.__schema.types[5].fields[6].type.ofType.name, "DateTime");
	assert(Object.hasOwn(data.__schema.types[5], "inputFields"));
	assertEquals(data.__schema.types[5].inputFields, null);
	assert(Object.hasOwn(data.__schema.types[5], "interfaces"));
	assertEquals(data.__schema.types[5].interfaces.length, 0);
	assert(Object.hasOwn(data.__schema.types[5], "kind"));
	assertEquals(data.__schema.types[5].kind, "OBJECT");
	assert(Object.hasOwn(data.__schema.types[5], "name"));
	assertEquals(data.__schema.types[5].name, "Post");
	assert(Object.hasOwn(data.__schema.types[5], "possibleTypes"));
	assertEquals(data.__schema.types[5].possibleTypes, null);
	assert(Object.hasOwn(data.__schema.types[6], "description"));
	assertEquals(data.__schema.types[6].description, "Input for creating posts");
	assert(Object.hasOwn(data.__schema.types[6], "enumValues"));
	assertEquals(data.__schema.types[6].enumValues, null);
	assert(Object.hasOwn(data.__schema.types[6], "fields"));
	assertEquals(data.__schema.types[6].fields, null);
	assert(Object.hasOwn(data.__schema.types[6], "inputFields"));
	assertEquals(data.__schema.types[6].inputFields.length, 3);
	assert(Object.hasOwn(data.__schema.types[6].inputFields[0], "defaultValue"));
	assertEquals(data.__schema.types[6].inputFields[0].defaultValue, null);
	assert(Object.hasOwn(data.__schema.types[6].inputFields[0], "description"));
	assertEquals(data.__schema.types[6].inputFields[0].description, "Post title");
	assert(Object.hasOwn(data.__schema.types[6].inputFields[0], "name"));
	assertEquals(data.__schema.types[6].inputFields[0].name, "title");
	assert(Object.hasOwn(data.__schema.types[6].inputFields[0], "type"));
	assert(Object.hasOwn(data.__schema.types[6].inputFields[0].type, "kind"));
	assertEquals(data.__schema.types[6].inputFields[0].type.kind, "NON_NULL");
	assert(Object.hasOwn(data.__schema.types[6].inputFields[0].type, "name"));
	assertEquals(data.__schema.types[6].inputFields[0].type.name, null);
	assert(Object.hasOwn(data.__schema.types[6].inputFields[0].type, "ofType"));
	assert(Object.hasOwn(data.__schema.types[6].inputFields[0].type.ofType, "kind"));
	assertEquals(data.__schema.types[6].inputFields[0].type.ofType.kind, "SCALAR");
	assert(Object.hasOwn(data.__schema.types[6].inputFields[0].type.ofType, "name"));
	assertEquals(data.__schema.types[6].inputFields[0].type.ofType.name, "String");
	assert(Object.hasOwn(data.__schema.types[6].inputFields[1], "defaultValue"));
	assertEquals(data.__schema.types[6].inputFields[1].defaultValue, null);
	assert(Object.hasOwn(data.__schema.types[6].inputFields[1], "description"));
	assertEquals(data.__schema.types[6].inputFields[1].description, "Post content");
	assert(Object.hasOwn(data.__schema.types[6].inputFields[1], "name"));
	assertEquals(data.__schema.types[6].inputFields[1].name, "content");
	assert(Object.hasOwn(data.__schema.types[6].inputFields[1], "type"));
	assert(Object.hasOwn(data.__schema.types[6].inputFields[1].type, "kind"));
	assertEquals(data.__schema.types[6].inputFields[1].type.kind, "NON_NULL");
	assert(Object.hasOwn(data.__schema.types[6].inputFields[1].type, "name"));
	assertEquals(data.__schema.types[6].inputFields[1].type.name, null);
	assert(Object.hasOwn(data.__schema.types[6].inputFields[1].type, "ofType"));
	assert(Object.hasOwn(data.__schema.types[6].inputFields[1].type.ofType, "kind"));
	assertEquals(data.__schema.types[6].inputFields[1].type.ofType.kind, "SCALAR");
	assert(Object.hasOwn(data.__schema.types[6].inputFields[1].type.ofType, "name"));
	assertEquals(data.__schema.types[6].inputFields[1].type.ofType.name, "String");
	assert(Object.hasOwn(data.__schema.types[6].inputFields[2], "defaultValue"));
	assertEquals(data.__schema.types[6].inputFields[2].defaultValue, null);
	assert(Object.hasOwn(data.__schema.types[6].inputFields[2], "description"));
	assertEquals(data.__schema.types[6].inputFields[2].description, "Author ID");
	assert(Object.hasOwn(data.__schema.types[6].inputFields[2], "name"));
	assertEquals(data.__schema.types[6].inputFields[2].name, "authorId");
	assert(Object.hasOwn(data.__schema.types[6].inputFields[2], "type"));
	assert(Object.hasOwn(data.__schema.types[6].inputFields[2].type, "kind"));
	assertEquals(data.__schema.types[6].inputFields[2].type.kind, "NON_NULL");
	assert(Object.hasOwn(data.__schema.types[6].inputFields[2].type, "name"));
	assertEquals(data.__schema.types[6].inputFields[2].type.name, null);
	assert(Object.hasOwn(data.__schema.types[6].inputFields[2].type, "ofType"));
	assert(Object.hasOwn(data.__schema.types[6].inputFields[2].type.ofType, "kind"));
	assertEquals(data.__schema.types[6].inputFields[2].type.ofType.kind, "SCALAR");
	assert(Object.hasOwn(data.__schema.types[6].inputFields[2].type.ofType, "name"));
	assertEquals(data.__schema.types[6].inputFields[2].type.ofType.name, "UUID");
	assert(Object.hasOwn(data.__schema.types[6], "interfaces"));
	assertEquals(data.__schema.types[6].interfaces, null);
	assert(Object.hasOwn(data.__schema.types[6], "kind"));
	assertEquals(data.__schema.types[6].kind, "INPUT_OBJECT");
	assert(Object.hasOwn(data.__schema.types[6], "name"));
	assertEquals(data.__schema.types[6].name, "CreatePostInput");
	assert(Object.hasOwn(data.__schema.types[6], "possibleTypes"));
	assertEquals(data.__schema.types[6].possibleTypes, null);
	const errors = responseBody.errors;
	assertEquals(errors?.length ?? 0, 0);
});

Deno.test("GraphQL query: entity_with_key", async () => {
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

	assertEquals(response.statusCode, 200);
	const responseBody = response.json();

	const data = responseBody.data;
	assert(Object.hasOwn(data, "_entities"));
	assertEquals(data._entities.length, 1);
	assert(Object.hasOwn(data._entities[0], "id"));
	assertEquals(data._entities[0].id, "42");
	assert(Object.hasOwn(data._entities[0], "name"));
	assertEquals(data._entities[0].name, "Bob Smith");
	assert(Object.hasOwn(data._entities[0], "profile"));
	assert(Object.hasOwn(data._entities[0].profile, "avatar"));
	assertEquals(data._entities[0].profile.avatar, "https://example.com/avatars/bob.jpg");
	assert(Object.hasOwn(data._entities[0].profile, "bio"));
	assertEquals(data._entities[0].profile.bio, "Software engineer and open source enthusiast");
	assert(Object.hasOwn(data._entities[0].profile, "joinDate"));
	assertEquals(data._entities[0].profile.joinDate, "2020-03-15");
	assert(Object.hasOwn(data._entities[0], "username"));
	assertEquals(data._entities[0].username, "bobsmith");
	const errors = responseBody.errors;
	assertEquals(errors?.length ?? 0, 0);
});

Deno.test("GraphQL query: requires_directive", async () => {
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

	assertEquals(response.statusCode, 200);
	const responseBody = response.json();

	const data = responseBody.data;
	assert(Object.hasOwn(data, "_entities"));
	assertEquals(data._entities.length, 1);
	assert(Object.hasOwn(data._entities[0], "destination"));
	assertEquals(data._entities[0].destination, "NYC");
	assert(Object.hasOwn(data._entities[0], "id"));
	assertEquals(data._entities[0].id, "ship-001");
	assert(Object.hasOwn(data._entities[0], "shippingEstimate"));
	assertEquals(data._entities[0].shippingEstimate, 24.75);
	assert(Object.hasOwn(data._entities[0], "weight"));
	assertEquals(data._entities[0].weight, 5.5);
	const errors = responseBody.errors;
	assertEquals(errors?.length ?? 0, 0);
});

Deno.test("GraphQL query: cross_subgraph_query", async () => {
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

	assertEquals(response.statusCode, 200);
	const responseBody = response.json();

	const data = responseBody.data;
	assert(Object.hasOwn(data, "user"));
	assert(Object.hasOwn(data.user, "email"));
	assertEquals(data.user.email, "emma@example.com");
	assert(Object.hasOwn(data.user, "id"));
	assertEquals(data.user.id, "usr-42");
	assert(Object.hasOwn(data.user, "name"));
	assertEquals(data.user.name, "Emma Wilson");
	assert(Object.hasOwn(data.user, "orders"));
	assertEquals(data.user.orders.length, 2);
	assert(Object.hasOwn(data.user.orders[0], "createdAt"));
	assertEquals(data.user.orders[0].createdAt, "2024-01-15T10:30:00Z");
	assert(Object.hasOwn(data.user.orders[0], "id"));
	assertEquals(data.user.orders[0].id, "order-101");
	assert(Object.hasOwn(data.user.orders[0], "orderId"));
	assertEquals(data.user.orders[0].orderId, "ORD-2024-001");
	assert(Object.hasOwn(data.user.orders[0], "status"));
	assertEquals(data.user.orders[0].status, "DELIVERED");
	assert(Object.hasOwn(data.user.orders[0], "total"));
	assertEquals(data.user.orders[0].total, 149.99);
	assert(Object.hasOwn(data.user.orders[1], "createdAt"));
	assertEquals(data.user.orders[1].createdAt, "2024-12-20T14:22:00Z");
	assert(Object.hasOwn(data.user.orders[1], "id"));
	assertEquals(data.user.orders[1].id, "order-102");
	assert(Object.hasOwn(data.user.orders[1], "orderId"));
	assertEquals(data.user.orders[1].orderId, "ORD-2024-002");
	assert(Object.hasOwn(data.user.orders[1], "status"));
	assertEquals(data.user.orders[1].status, "PROCESSING");
	assert(Object.hasOwn(data.user.orders[1], "total"));
	assertEquals(data.user.orders[1].total, 89.5);
	const errors = responseBody.errors;
	assertEquals(errors?.length ?? 0, 0);
});

Deno.test("GraphQL query: provides_directive", async () => {
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

	assertEquals(response.statusCode, 200);
	const responseBody = response.json();

	const data = responseBody.data;
	assert(Object.hasOwn(data, "_entities"));
	assertEquals(data._entities.length, 1);
	assert(Object.hasOwn(data._entities[0], "content"));
	assertEquals(data._entities[0].content, "Learn how to build scalable microservices...");
	assert(Object.hasOwn(data._entities[0], "id"));
	assertEquals(data._entities[0].id, "post-123");
	assert(Object.hasOwn(data._entities[0], "reviews"));
	assertEquals(data._entities[0].reviews.length, 2);
	assert(Object.hasOwn(data._entities[0].reviews[0], "author"));
	assert(Object.hasOwn(data._entities[0].reviews[0].author, "id"));
	assertEquals(data._entities[0].reviews[0].author.id, "user-1");
	assert(Object.hasOwn(data._entities[0].reviews[0].author, "name"));
	assertEquals(data._entities[0].reviews[0].author.name, "Charlie Brown");
	assert(Object.hasOwn(data._entities[0].reviews[0], "id"));
	assertEquals(data._entities[0].reviews[0].id, "rev-001");
	assert(Object.hasOwn(data._entities[0].reviews[0], "rating"));
	assertEquals(data._entities[0].reviews[0].rating, 5);
	assert(Object.hasOwn(data._entities[0].reviews[0], "text"));
	assertEquals(data._entities[0].reviews[0].text, "Excellent post!");
	assert(Object.hasOwn(data._entities[0].reviews[1], "author"));
	assert(Object.hasOwn(data._entities[0].reviews[1].author, "id"));
	assertEquals(data._entities[0].reviews[1].author.id, "user-2");
	assert(Object.hasOwn(data._entities[0].reviews[1].author, "name"));
	assertEquals(data._entities[0].reviews[1].author.name, "Diana Prince");
	assert(Object.hasOwn(data._entities[0].reviews[1], "id"));
	assertEquals(data._entities[0].reviews[1].id, "rev-002");
	assert(Object.hasOwn(data._entities[0].reviews[1], "rating"));
	assertEquals(data._entities[0].reviews[1].rating, 4);
	assert(Object.hasOwn(data._entities[0].reviews[1], "text"));
	assertEquals(data._entities[0].reviews[1].text, "Very helpful");
	assert(Object.hasOwn(data._entities[0], "title"));
	assertEquals(data._entities[0].title, "Getting Started with GraphQL Federation");
	const errors = responseBody.errors;
	assertEquals(errors?.length ?? 0, 0);
});

Deno.test("GraphQL query: external_field", async () => {
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

	assertEquals(response.statusCode, 200);
	const responseBody = response.json();

	const data = responseBody.data;
	assert(Object.hasOwn(data, "_entities"));
	assertEquals(data._entities.length, 1);
	assert(Object.hasOwn(data._entities[0], "dimensions"));
	assertEquals(data._entities[0].dimensions, "10x8x6");
	assert(Object.hasOwn(data._entities[0], "id"));
	assertEquals(data._entities[0].id, "parcel-x1");
	assert(Object.hasOwn(data._entities[0], "label"));
	assertEquals(data._entities[0].label, "SMALL_PACKAGE_2.5KG");
	assert(Object.hasOwn(data._entities[0], "weight"));
	assertEquals(data._entities[0].weight, 2.5);
	const errors = responseBody.errors;
	assertEquals(errors?.length ?? 0, 0);
});

Deno.test("GraphQL query: inaccessible_directive", async () => {
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

	assertEquals(response.statusCode, 400);
	const responseBody = response.json();

	const errors = responseBody.errors;
	assert(errors !== undefined);
	assertEquals(errors?.length, 1);
	assert(
		errors?.[0]?.message.includes(
			"Cannot query field 'internalScore' on type 'User'. This field is @inaccessible and not available in the public schema.",
		),
	);
});

Deno.test("GraphQL query: subgraph_introspection", async () => {
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

	assertEquals(response.statusCode, 200);
	const responseBody = response.json();

	const data = responseBody.data;
	assert(Object.hasOwn(data, "_service"));
	assert(Object.hasOwn(data._service, "sdl"));
	assertEquals(
		data._service.sdl,
		'type Account @key(fields: "accountId") {\n  accountId: ID!\n  accountName: String!\n  tier: String!\n  createdAt: String!\n}\n\ntype Query {\n  account(accountId: ID!): Account\n}',
	);
	const errors = responseBody.errors;
	assertEquals(errors?.length ?? 0, 0);
});

Deno.test("GraphQL query: shareable_directive", async () => {
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

	assertEquals(response.statusCode, 200);
	const responseBody = response.json();

	const data = responseBody.data;
	assert(Object.hasOwn(data, "_entities"));
	assertEquals(data._entities.length, 1);
	assert(Object.hasOwn(data._entities[0], "category"));
	assertEquals(data._entities[0].category, "Electronics");
	assert(Object.hasOwn(data._entities[0], "description"));
	assertEquals(data._entities[0].description, "Premium noise-canceling headphones with 30-hour battery life");
	assert(Object.hasOwn(data._entities[0], "id"));
	assertEquals(data._entities[0].id, "prod-001");
	assert(Object.hasOwn(data._entities[0], "name"));
	assertEquals(data._entities[0].name, "Wireless Headphones");
	assert(Object.hasOwn(data._entities[0], "price"));
	assertEquals(data._entities[0].price, 199.99);
	const errors = responseBody.errors;
	assertEquals(errors?.length ?? 0, 0);
});

Deno.test("GraphQL query: entity_resolution_basic", async () => {
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

	assertEquals(response.statusCode, 200);
	const responseBody = response.json();

	const data = responseBody.data;
	assert(Object.hasOwn(data, "_entities"));
	assertEquals(data._entities.length, 1);
	assert(Object.hasOwn(data._entities[0], "email"));
	assertEquals(data._entities[0].email, "alice@example.com");
	assert(Object.hasOwn(data._entities[0], "id"));
	assertEquals(data._entities[0].id, "1");
	assert(Object.hasOwn(data._entities[0], "name"));
	assertEquals(data._entities[0].name, "Alice Johnson");
	const errors = responseBody.errors;
	assertEquals(errors?.length ?? 0, 0);
});

Deno.test("GraphQL query: override_directive", async () => {
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

	assertEquals(response.statusCode, 200);
	const responseBody = response.json();

	const data = responseBody.data;
	assert(Object.hasOwn(data, "user"));
	assert(Object.hasOwn(data.user, "email"));
	assertEquals(data.user.email, "john.doe@example.com");
	assert(Object.hasOwn(data.user, "id"));
	assertEquals(data.user.id, "user-789");
	assert(Object.hasOwn(data.user, "profile"));
	assert(Object.hasOwn(data.user.profile, "bio"));
	assertEquals(data.user.profile.bio, "Software developer and tech enthusiast");
	assert(Object.hasOwn(data.user.profile, "joinDate"));
	assertEquals(data.user.profile.joinDate, "2021-06-12");
	assert(Object.hasOwn(data.user.profile, "location"));
	assertEquals(data.user.profile.location, "San Francisco, CA");
	assert(Object.hasOwn(data.user, "username"));
	assertEquals(data.user.username, "johndoe");
	const errors = responseBody.errors;
	assertEquals(errors?.length ?? 0, 0);
});

Deno.test("GraphQL query: federation_type_mismatch", async () => {
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

	assertEquals(response.statusCode, 400);
	const responseBody = response.json();

	const errors = responseBody.errors;
	assert(errors !== undefined);
	assertEquals(errors?.length, 1);
	assert(errors?.[0]?.message.includes("Unknown type 'InvalidType' in entity representation"));
});

Deno.test("GraphQL query: entity_with_compound_key", async () => {
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

	assertEquals(response.statusCode, 200);
	const responseBody = response.json();

	const data = responseBody.data;
	assert(Object.hasOwn(data, "_entities"));
	assertEquals(data._entities.length, 1);
	assert(Object.hasOwn(data._entities[0], "category"));
	assertEquals(data._entities[0].category, "electronics");
	assert(Object.hasOwn(data._entities[0], "description"));
	assertEquals(data._entities[0].description, "Premium noise-cancelling wireless headphones");
	assert(Object.hasOwn(data._entities[0], "name"));
	assertEquals(data._entities[0].name, "Wireless Headphones");
	assert(Object.hasOwn(data._entities[0], "price"));
	assertEquals(data._entities[0].price, 199.99);
	assert(Object.hasOwn(data._entities[0], "sku"));
	assertEquals(data._entities[0].sku, "ABC123");
	assert(Object.hasOwn(data._entities[0], "stock"));
	assertEquals(data._entities[0].stock, 45);
	const errors = responseBody.errors;
	assertEquals(errors?.length ?? 0, 0);
});

Deno.test("GraphQL query: federation_error_missing_entity", async () => {
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

	assertEquals(response.statusCode, 200);
	const responseBody = response.json();

	const data = responseBody.data;
	assert(Object.hasOwn(data, "_entities"));
	assertEquals(data._entities.length, 1);
	assertEquals(data._entities[0], null);
	const errors = responseBody.errors;
	assertEquals(errors?.length ?? 0, 0);
});

Deno.test("GraphQL query: field_error", async () => {
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

	assertEquals(response.statusCode, 400);
	const responseBody = response.json();

	const errors = responseBody.errors;
	assert(errors !== undefined);
	assertEquals(errors?.length, 1);
	assert(
		errors?.[0]?.message.includes(
			'Cannot query field "invalidField" on type "User". Did you mean "id", "name", or "email"?',
		),
	);
});

Deno.test("GraphQL query: syntax_error", async () => {
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

	assertEquals(response.statusCode, 400);
	const responseBody = response.json();

	const errors = responseBody.errors;
	assert(errors !== undefined);
	assertEquals(errors?.length, 1);
	assert(errors?.[0]?.message.includes("Syntax Error in GraphQL query at line 2, column 17: Unterminated string."));
});

Deno.test("GraphQL query: type_error", async () => {
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

	assertEquals(response.statusCode, 400);
	const responseBody = response.json();

	const errors = responseBody.errors;
	assert(errors !== undefined);
	assertEquals(errors?.length, 1);
	assert(errors?.[0]?.message.includes('Variable "$id" of type "ID!" was provided invalid value.'));
});

Deno.test("GraphQL query: query_batching", async () => {
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

	assertEquals(response.statusCode, 200);
	const responseBody = response.json();

	const data = responseBody.data;
	assertEquals(data.length, 3);
	assert(Object.hasOwn(data[0], "user"));
	assert(Object.hasOwn(data[0].user, "email"));
	assertEquals(data[0].user.email, "alice@example.com");
	assert(Object.hasOwn(data[0].user, "id"));
	assertEquals(data[0].user.id, "user-1");
	assert(Object.hasOwn(data[0].user, "name"));
	assertEquals(data[0].user.name, "Alice Johnson");
	assert(Object.hasOwn(data[1], "user"));
	assert(Object.hasOwn(data[1].user, "email"));
	assertEquals(data[1].user.email, "bob@example.com");
	assert(Object.hasOwn(data[1].user, "id"));
	assertEquals(data[1].user.id, "user-2");
	assert(Object.hasOwn(data[1].user, "name"));
	assertEquals(data[1].user.name, "Bob Smith");
	assert(Object.hasOwn(data[2], "post"));
	assert(Object.hasOwn(data[2].post, "author_id"));
	assertEquals(data[2].post.author_id, "user-1");
	assert(Object.hasOwn(data[2].post, "id"));
	assertEquals(data[2].post.id, "post-1");
	assert(Object.hasOwn(data[2].post, "title"));
	assertEquals(data[2].post.title, "GraphQL Performance Tips");
	const errors = responseBody.errors;
	assertEquals(errors?.length ?? 0, 0);
});

Deno.test("GraphQL query: response_streaming", async () => {
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

	assertEquals(response.statusCode, 200);
	const responseBody = response.json();

	const errors = responseBody.errors;
	assertEquals(errors?.length ?? 0, 0);
});

Deno.test("GraphQL query: field_level_permissions", async () => {
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

	assertEquals(response.statusCode, 200);
	const responseBody = response.json();

	const errors = responseBody.errors;
	assert(errors !== undefined);
	assertEquals(errors?.length, 1);
	assert(errors?.[0]?.message.includes("Field 'privateData' requires elevated permissions"));
});

Deno.test("GraphQL query: role_admin_allowed", async () => {
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

	assertEquals(response.statusCode, 200);
	const responseBody = response.json();

	const data = responseBody.data;
	assert(Object.hasOwn(data, "adminPanel"));
	assert(Object.hasOwn(data.adminPanel, "stats"));
	assert(Object.hasOwn(data.adminPanel.stats, "activeUsers"));
	assertEquals(data.adminPanel.stats.activeUsers, 856);
	assert(Object.hasOwn(data.adminPanel.stats, "totalRevenue"));
	assertEquals(data.adminPanel.stats.totalRevenue, 125000.5);
	assert(Object.hasOwn(data.adminPanel.stats, "totalUsers"));
	assertEquals(data.adminPanel.stats.totalUsers, 1250);
	const errors = responseBody.errors;
	assertEquals(errors?.length ?? 0, 0);
});

Deno.test("GraphQL query: resource_owner_allowed", async () => {
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

	assertEquals(response.statusCode, 200);
	const responseBody = response.json();

	const data = responseBody.data;
	assert(Object.hasOwn(data, "user"));
	assert(Object.hasOwn(data.user, "id"));
	assertEquals(data.user.id, "user123");
	assert(Object.hasOwn(data.user, "profile"));
	assert(Object.hasOwn(data.user.profile, "bio"));
	assertEquals(data.user.profile.bio, "Software engineer from San Francisco");
	assert(Object.hasOwn(data.user.profile, "joinDate"));
	assertEquals(data.user.profile.joinDate, "2020-01-15");
	assert(Object.hasOwn(data.user.profile, "website"));
	assertEquals(data.user.profile.website, "https://example.com");
	const errors = responseBody.errors;
	assertEquals(errors?.length ?? 0, 0);
});

Deno.test("GraphQL query: permission_chain", async () => {
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

	assertEquals(response.statusCode, 200);
	const responseBody = response.json();

	const errors = responseBody.errors;
	assert(errors !== undefined);
	assertEquals(errors?.length, 2);
	assert(errors?.[0]?.message.includes("Insufficient permissions to access privateMetrics"));
	assert(errors?.[1]?.message.includes("Insufficient permissions to access adminSettings"));
});

Deno.test("GraphQL query: resource_owner_denied", async () => {
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

	assertEquals(response.statusCode, 403);
	const responseBody = response.json();

	const errors = responseBody.errors;
	assert(errors !== undefined);
	assertEquals(errors?.length, 1);
	assert(errors?.[0]?.message.includes("Not authorized to access this resource"));
});

Deno.test("GraphQL query: role_user_denied", async () => {
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

	assertEquals(response.statusCode, 403);
	const responseBody = response.json();

	const errors = responseBody.errors;
	assert(errors !== undefined);
	assertEquals(errors?.length, 1);
	assert(errors?.[0]?.message.includes("Insufficient permissions to access adminPanel"));
});

Deno.test("GraphQL query: jwt_valid", async () => {
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

	assertEquals(response.statusCode, 200);
	const responseBody = response.json();

	const data = responseBody.data;
	assert(Object.hasOwn(data, "currentUser"));
	assert(Object.hasOwn(data.currentUser, "email"));
	assertEquals(data.currentUser.email, "john@example.com");
	assert(Object.hasOwn(data.currentUser, "id"));
	assertEquals(data.currentUser.id, "user123");
	assert(Object.hasOwn(data.currentUser, "name"));
	assertEquals(data.currentUser.name, "John Doe");
	const errors = responseBody.errors;
	assertEquals(errors?.length ?? 0, 0);
});

Deno.test("GraphQL query: api_key_invalid", async () => {
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

	assertEquals(response.statusCode, 401);
	const responseBody = response.json();

	const errors = responseBody.errors;
	assert(errors !== undefined);
	assertEquals(errors?.length, 1);
	assert(errors?.[0]?.message.includes("Invalid API key"));
});

Deno.test("GraphQL query: jwt_expired", async () => {
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

	assertEquals(response.statusCode, 401);
	const responseBody = response.json();

	const errors = responseBody.errors;
	assert(errors !== undefined);
	assertEquals(errors?.length, 1);
	assert(errors?.[0]?.message.includes("Token expired"));
});

Deno.test("GraphQL query: jwt_invalid_signature", async () => {
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

	assertEquals(response.statusCode, 401);
	const responseBody = response.json();

	const errors = responseBody.errors;
	assert(errors !== undefined);
	assertEquals(errors?.length, 1);
	assert(errors?.[0]?.message.includes("Invalid token signature"));
});

Deno.test("GraphQL query: no_authentication", async () => {
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

	assertEquals(response.statusCode, 401);
	const responseBody = response.json();

	const errors = responseBody.errors;
	assert(errors !== undefined);
	assertEquals(errors?.length, 1);
	assert(errors?.[0]?.message.includes("Authentication required"));
});

Deno.test("GraphQL query: session_cookie_valid", async () => {
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

	assertEquals(response.statusCode, 200);
	const responseBody = response.json();

	const data = responseBody.data;
	assert(Object.hasOwn(data, "userProfile"));
	assert(Object.hasOwn(data.userProfile, "email"));
	assertEquals(data.userProfile.email, "alice@example.com");
	assert(Object.hasOwn(data.userProfile, "id"));
	assertEquals(data.userProfile.id, "user456");
	assert(Object.hasOwn(data.userProfile, "username"));
	assertEquals(data.userProfile.username, "alice_smith");
	const errors = responseBody.errors;
	assertEquals(errors?.length ?? 0, 0);
});

Deno.test("GraphQL query: multiple_auth_methods", async () => {
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

	assertEquals(response.statusCode, 200);
	const responseBody = response.json();

	const data = responseBody.data;
	assert(Object.hasOwn(data, "currentUser"));
	assert(Object.hasOwn(data.currentUser, "authMethod"));
	assertEquals(data.currentUser.authMethod, "jwt");
	assert(Object.hasOwn(data.currentUser, "email"));
	assertEquals(data.currentUser.email, "john@example.com");
	assert(Object.hasOwn(data.currentUser, "id"));
	assertEquals(data.currentUser.id, "user123");
	const errors = responseBody.errors;
	assertEquals(errors?.length ?? 0, 0);
});

Deno.test("GraphQL query: api_key_valid", async () => {
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

	assertEquals(response.statusCode, 200);
	const responseBody = response.json();

	const data = responseBody.data;
	assert(Object.hasOwn(data, "secureData"));
	assertEquals(data.secureData, "Protected data from API key authentication");
	const errors = responseBody.errors;
	assertEquals(errors?.length ?? 0, 0);
});

Deno.test("GraphQL query: invalid_types", async () => {
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

	assertEquals(response.statusCode, 400);
	const responseBody = response.json();

	const errors = responseBody.errors;
	assert(errors !== undefined);
	assertEquals(errors?.length, 1);
	assert(errors?.[0]?.message.includes('Variable "$limit" of type "Int!" was provided invalid value.'));
});

Deno.test("GraphQL query: dataloader_cache_hit", async () => {
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

	assertEquals(response.statusCode, 200);
	const responseBody = response.json();

	const data = responseBody.data;
	assert(Object.hasOwn(data, "user1"));
	assert(Object.hasOwn(data.user1, "email"));
	assertEquals(data.user1.email, "alice@example.com");
	assert(Object.hasOwn(data.user1, "id"));
	assertEquals(data.user1.id, "1");
	assert(Object.hasOwn(data.user1, "name"));
	assertEquals(data.user1.name, "Alice Smith");
	assert(Object.hasOwn(data, "user2"));
	assert(Object.hasOwn(data.user2, "id"));
	assertEquals(data.user2.id, "1");
	assert(Object.hasOwn(data.user2, "name"));
	assertEquals(data.user2.name, "Alice Smith");
	assert(Object.hasOwn(data.user2, "username"));
	assertEquals(data.user2.username, "alice_smith");
	assert(Object.hasOwn(data, "user3"));
	assert(Object.hasOwn(data.user3, "email"));
	assertEquals(data.user3.email, "bob@example.com");
	assert(Object.hasOwn(data.user3, "id"));
	assertEquals(data.user3.id, "2");
	assert(Object.hasOwn(data.user3, "name"));
	assertEquals(data.user3.name, "Bob Johnson");
	const errors = responseBody.errors;
	assertEquals(errors?.length ?? 0, 0);
});

Deno.test("GraphQL query: dataloader_with_variables", async () => {
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

	assertEquals(response.statusCode, 200);
	const responseBody = response.json();

	const data = responseBody.data;
	assert(Object.hasOwn(data, "posts"));
	assertEquals(data.posts.length, 3);
	assert(Object.hasOwn(data.posts[0], "id"));
	assertEquals(data.posts[0].id, "1");
	assert(Object.hasOwn(data.posts[0], "publishedAt"));
	assertEquals(data.posts[0].publishedAt, "2025-01-10T08:00:00Z");
	assert(Object.hasOwn(data.posts[0], "slug"));
	assertEquals(data.posts[0].slug, "getting-started-graphql");
	assert(Object.hasOwn(data.posts[0], "tags"));
	assertEquals(data.posts[0].tags.length, 2);
	assertEquals(data.posts[0].tags[0], "graphql");
	assertEquals(data.posts[0].tags[1], "tutorial");
	assert(Object.hasOwn(data.posts[0], "title"));
	assertEquals(data.posts[0].title, "Getting Started with GraphQL");
	assert(Object.hasOwn(data.posts[1], "id"));
	assertEquals(data.posts[1].id, "2");
	assert(Object.hasOwn(data.posts[1], "publishedAt"));
	assertEquals(data.posts[1].publishedAt, "2025-01-15T10:30:00Z");
	assert(Object.hasOwn(data.posts[1], "slug"));
	assertEquals(data.posts[1].slug, "mastering-dataloader");
	assert(Object.hasOwn(data.posts[1], "tags"));
	assertEquals(data.posts[1].tags.length, 3);
	assertEquals(data.posts[1].tags[0], "dataloader");
	assertEquals(data.posts[1].tags[1], "performance");
	assertEquals(data.posts[1].tags[2], "optimization");
	assert(Object.hasOwn(data.posts[1], "title"));
	assertEquals(data.posts[1].title, "Mastering DataLoader");
	assert(Object.hasOwn(data.posts[2], "id"));
	assertEquals(data.posts[2].id, "3");
	assert(Object.hasOwn(data.posts[2], "publishedAt"));
	assertEquals(data.posts[2].publishedAt, "2025-01-20T14:45:00Z");
	assert(Object.hasOwn(data.posts[2], "slug"));
	assertEquals(data.posts[2].slug, "graphql-best-practices");
	assert(Object.hasOwn(data.posts[2], "tags"));
	assertEquals(data.posts[2].tags.length, 3);
	assertEquals(data.posts[2].tags[0], "graphql");
	assertEquals(data.posts[2].tags[1], "best-practices");
	assertEquals(data.posts[2].tags[2], "patterns");
	assert(Object.hasOwn(data.posts[2], "title"));
	assertEquals(data.posts[2].title, "GraphQL Best Practices");
	const errors = responseBody.errors;
	assertEquals(errors?.length ?? 0, 0);
});

Deno.test("GraphQL query: dataloader_batch_users", async () => {
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

	assertEquals(response.statusCode, 200);
	const responseBody = response.json();

	const data = responseBody.data;
	assert(Object.hasOwn(data, "users"));
	assertEquals(data.users.length, 3);
	assert(Object.hasOwn(data.users[0], "age"));
	assertEquals(data.users[0].age, 28);
	assert(Object.hasOwn(data.users[0], "email"));
	assertEquals(data.users[0].email, "alice@example.com");
	assert(Object.hasOwn(data.users[0], "id"));
	assertEquals(data.users[0].id, "1");
	assert(Object.hasOwn(data.users[0], "name"));
	assertEquals(data.users[0].name, "Alice Johnson");
	assert(Object.hasOwn(data.users[1], "age"));
	assertEquals(data.users[1].age, 34);
	assert(Object.hasOwn(data.users[1], "email"));
	assertEquals(data.users[1].email, "bob@example.com");
	assert(Object.hasOwn(data.users[1], "id"));
	assertEquals(data.users[1].id, "2");
	assert(Object.hasOwn(data.users[1], "name"));
	assertEquals(data.users[1].name, "Bob Smith");
	assert(Object.hasOwn(data.users[2], "age"));
	assertEquals(data.users[2].age, 26);
	assert(Object.hasOwn(data.users[2], "email"));
	assertEquals(data.users[2].email, "carol@example.com");
	assert(Object.hasOwn(data.users[2], "id"));
	assertEquals(data.users[2].id, "3");
	assert(Object.hasOwn(data.users[2], "name"));
	assertEquals(data.users[2].name, "Carol Davis");
	const errors = responseBody.errors;
	assertEquals(errors?.length ?? 0, 0);
});

Deno.test("GraphQL query: dataloader_error_handling", async () => {
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

	assertEquals(response.statusCode, 200);
	const responseBody = response.json();

	const data = responseBody.data;
	assert(Object.hasOwn(data, "users"));
	assertEquals(data.users.length, 3);
	assert(Object.hasOwn(data.users[0], "email"));
	assertEquals(data.users[0].email, "alice@example.com");
	assert(Object.hasOwn(data.users[0], "id"));
	assertEquals(data.users[0].id, "1");
	assert(Object.hasOwn(data.users[0], "name"));
	assertEquals(data.users[0].name, "Alice Johnson");
	assertEquals(data.users[1], null);
	assert(Object.hasOwn(data.users[2], "email"));
	assertEquals(data.users[2].email, "bob@example.com");
	assert(Object.hasOwn(data.users[2], "id"));
	assertEquals(data.users[2].id, "2");
	assert(Object.hasOwn(data.users[2], "name"));
	assertEquals(data.users[2].name, "Bob Smith");
	const errors = responseBody.errors;
	assert(errors !== undefined);
	assertEquals(errors?.length, 1);
	assert(errors?.[0]?.message.includes("User not found with id '999'"));
});

Deno.test("GraphQL query: dataloader_custom_key", async () => {
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

	assertEquals(response.statusCode, 200);
	const responseBody = response.json();

	const data = responseBody.data;
	assert(Object.hasOwn(data, "productBySlug"));
	assert(Object.hasOwn(data.productBySlug, "category"));
	assertEquals(data.productBySlug.category, "electronics");
	assert(Object.hasOwn(data.productBySlug, "description"));
	assertEquals(data.productBySlug.description, "High-performance laptop for professionals");
	assert(Object.hasOwn(data.productBySlug, "id"));
	assertEquals(data.productBySlug.id, "prod-1");
	assert(Object.hasOwn(data.productBySlug, "name"));
	assertEquals(data.productBySlug.name, "Professional Laptop");
	assert(Object.hasOwn(data.productBySlug, "price"));
	assertEquals(data.productBySlug.price, 1299.99);
	assert(Object.hasOwn(data.productBySlug, "slug"));
	assertEquals(data.productBySlug.slug, "laptop-pro-2025");
	const errors = responseBody.errors;
	assertEquals(errors?.length ?? 0, 0);
});

Deno.test("GraphQL query: dataloader_nested_batching", async () => {
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

	assertEquals(response.statusCode, 200);
	const responseBody = response.json();

	const data = responseBody.data;
	assert(Object.hasOwn(data, "posts"));
	assertEquals(data.posts.length, 2);
	assert(Object.hasOwn(data.posts[0], "comments"));
	assertEquals(data.posts[0].comments.length, 2);
	assert(Object.hasOwn(data.posts[0].comments[0], "author"));
	assert(Object.hasOwn(data.posts[0].comments[0].author, "email"));
	assertEquals(data.posts[0].comments[0].author.email, "alice@example.com");
	assert(Object.hasOwn(data.posts[0].comments[0].author, "id"));
	assertEquals(data.posts[0].comments[0].author.id, "user-1");
	assert(Object.hasOwn(data.posts[0].comments[0].author, "name"));
	assertEquals(data.posts[0].comments[0].author.name, "Alice Johnson");
	assert(Object.hasOwn(data.posts[0].comments[0], "id"));
	assertEquals(data.posts[0].comments[0].id, "comment-1");
	assert(Object.hasOwn(data.posts[0].comments[0], "text"));
	assertEquals(data.posts[0].comments[0].text, "Great article!");
	assert(Object.hasOwn(data.posts[0].comments[1], "author"));
	assert(Object.hasOwn(data.posts[0].comments[1].author, "email"));
	assertEquals(data.posts[0].comments[1].author.email, "bob@example.com");
	assert(Object.hasOwn(data.posts[0].comments[1].author, "id"));
	assertEquals(data.posts[0].comments[1].author.id, "user-2");
	assert(Object.hasOwn(data.posts[0].comments[1].author, "name"));
	assertEquals(data.posts[0].comments[1].author.name, "Bob Smith");
	assert(Object.hasOwn(data.posts[0].comments[1], "id"));
	assertEquals(data.posts[0].comments[1].id, "comment-2");
	assert(Object.hasOwn(data.posts[0].comments[1], "text"));
	assertEquals(data.posts[0].comments[1].text, "Very helpful");
	assert(Object.hasOwn(data.posts[0], "id"));
	assertEquals(data.posts[0].id, "post-1");
	assert(Object.hasOwn(data.posts[0], "title"));
	assertEquals(data.posts[0].title, "GraphQL Introduction");
	assert(Object.hasOwn(data.posts[1], "comments"));
	assertEquals(data.posts[1].comments.length, 1);
	assert(Object.hasOwn(data.posts[1].comments[0], "author"));
	assert(Object.hasOwn(data.posts[1].comments[0].author, "email"));
	assertEquals(data.posts[1].comments[0].author.email, "alice@example.com");
	assert(Object.hasOwn(data.posts[1].comments[0].author, "id"));
	assertEquals(data.posts[1].comments[0].author.id, "user-1");
	assert(Object.hasOwn(data.posts[1].comments[0].author, "name"));
	assertEquals(data.posts[1].comments[0].author.name, "Alice Johnson");
	assert(Object.hasOwn(data.posts[1].comments[0], "id"));
	assertEquals(data.posts[1].comments[0].id, "comment-3");
	assert(Object.hasOwn(data.posts[1].comments[0], "text"));
	assertEquals(data.posts[1].comments[0].text, "Excellent explanation");
	assert(Object.hasOwn(data.posts[1], "id"));
	assertEquals(data.posts[1].id, "post-2");
	assert(Object.hasOwn(data.posts[1], "title"));
	assertEquals(data.posts[1].title, "Advanced Patterns");
	const errors = responseBody.errors;
	assertEquals(errors?.length ?? 0, 0);
});

Deno.test("GraphQL query: dataloader_priming", async () => {
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

	assertEquals(response.statusCode, 200);
	const responseBody = response.json();

	const data = responseBody.data;
	assert(Object.hasOwn(data, "userList"));
	assertEquals(data.userList.length, 3);
	assert(Object.hasOwn(data.userList[0], "email"));
	assertEquals(data.userList[0].email, "alice@example.com");
	assert(Object.hasOwn(data.userList[0], "id"));
	assertEquals(data.userList[0].id, "user-1");
	assert(Object.hasOwn(data.userList[0], "name"));
	assertEquals(data.userList[0].name, "Alice Johnson");
	assert(Object.hasOwn(data.userList[0], "role"));
	assertEquals(data.userList[0].role, "admin");
	assert(Object.hasOwn(data.userList[1], "email"));
	assertEquals(data.userList[1].email, "bob@example.com");
	assert(Object.hasOwn(data.userList[1], "id"));
	assertEquals(data.userList[1].id, "user-2");
	assert(Object.hasOwn(data.userList[1], "name"));
	assertEquals(data.userList[1].name, "Bob Smith");
	assert(Object.hasOwn(data.userList[1], "role"));
	assertEquals(data.userList[1].role, "user");
	assert(Object.hasOwn(data.userList[2], "email"));
	assertEquals(data.userList[2].email, "carol@example.com");
	assert(Object.hasOwn(data.userList[2], "id"));
	assertEquals(data.userList[2].id, "user-3");
	assert(Object.hasOwn(data.userList[2], "name"));
	assertEquals(data.userList[2].name, "Carol Davis");
	assert(Object.hasOwn(data.userList[2], "role"));
	assertEquals(data.userList[2].role, "moderator");
	const errors = responseBody.errors;
	assertEquals(errors?.length ?? 0, 0);
});

Deno.test("GraphQL query: dataloader_n_plus_one_prevention", async () => {
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

	assertEquals(response.statusCode, 200);
	const responseBody = response.json();

	const data = responseBody.data;
	assert(Object.hasOwn(data, "posts"));
	assertEquals(data.posts.length, 3);
	assert(Object.hasOwn(data.posts[0], "author"));
	assert(Object.hasOwn(data.posts[0].author, "email"));
	assertEquals(data.posts[0].author.email, "alice@example.com");
	assert(Object.hasOwn(data.posts[0].author, "id"));
	assertEquals(data.posts[0].author.id, "user-1");
	assert(Object.hasOwn(data.posts[0].author, "name"));
	assertEquals(data.posts[0].author.name, "Alice Johnson");
	assert(Object.hasOwn(data.posts[0], "content"));
	assertEquals(data.posts[0].content, "Introduction to GraphQL...");
	assert(Object.hasOwn(data.posts[0], "id"));
	assertEquals(data.posts[0].id, "post-1");
	assert(Object.hasOwn(data.posts[0], "title"));
	assertEquals(data.posts[0].title, "GraphQL Basics");
	assert(Object.hasOwn(data.posts[1], "author"));
	assert(Object.hasOwn(data.posts[1].author, "email"));
	assertEquals(data.posts[1].author.email, "bob@example.com");
	assert(Object.hasOwn(data.posts[1].author, "id"));
	assertEquals(data.posts[1].author.id, "user-2");
	assert(Object.hasOwn(data.posts[1].author, "name"));
	assertEquals(data.posts[1].author.name, "Bob Smith");
	assert(Object.hasOwn(data.posts[1], "content"));
	assertEquals(data.posts[1].content, "Optimizing GraphQL queries...");
	assert(Object.hasOwn(data.posts[1], "id"));
	assertEquals(data.posts[1].id, "post-2");
	assert(Object.hasOwn(data.posts[1], "title"));
	assertEquals(data.posts[1].title, "DataLoader Patterns");
	assert(Object.hasOwn(data.posts[2], "author"));
	assert(Object.hasOwn(data.posts[2].author, "email"));
	assertEquals(data.posts[2].author.email, "alice@example.com");
	assert(Object.hasOwn(data.posts[2].author, "id"));
	assertEquals(data.posts[2].author.id, "user-1");
	assert(Object.hasOwn(data.posts[2].author, "name"));
	assertEquals(data.posts[2].author.name, "Alice Johnson");
	assert(Object.hasOwn(data.posts[2], "content"));
	assertEquals(data.posts[2].content, "Custom directives and more...");
	assert(Object.hasOwn(data.posts[2], "id"));
	assertEquals(data.posts[2].id, "post-3");
	assert(Object.hasOwn(data.posts[2], "title"));
	assertEquals(data.posts[2].title, "Advanced GraphQL");
	const errors = responseBody.errors;
	assertEquals(errors?.length ?? 0, 0);
});
