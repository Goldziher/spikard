#!/usr/bin/env bun
/**
 * Elysia benchmark server - validation only (no raw mode exists)
 *
 * Elysia is a high-performance TypeScript framework built for Bun runtime.
 * It features built-in schema validation via TypeBox.
 *
 * Note: This requires Bun runtime (not Node.js) - install from https://bun.sh
 */

import { Elysia, t } from "elysia";

const app = new Elysia();

const SmallPayloadSchema = t.Object({
	name: t.String(),
	description: t.String(),
	price: t.Number(),
	tax: t.Number(),
});

const ImageSchema = t.Object({
	url: t.String(),
	name: t.String(),
});

const MediumPayloadSchema = t.Object({
	name: t.String(),
	price: t.Number(),
	image: ImageSchema,
});

const CountrySchema = t.Object({
	name: t.String(),
	code: t.String(),
});

const AddressSchema = t.Object({
	street: t.String(),
	city: t.String(),
	country: CountrySchema,
});

const SellerSchema = t.Object({
	name: t.String(),
	address: AddressSchema,
});

const LargePayloadSchema = t.Object({
	name: t.String(),
	price: t.Number(),
	seller: SellerSchema,
});

const VeryLargePayloadSchema = t.Object({
	name: t.String(),
	tags: t.Array(t.String()),
	images: t.Array(ImageSchema),
});

const QueryFewSchema = t.Object({
	q: t.String(),
	page: t.Optional(t.Numeric()),
	limit: t.Optional(t.Numeric()),
});

const QueryMediumSchema = t.Object({
	search: t.String(),
	category: t.Optional(t.String()),
	sort: t.Optional(t.String()),
	order: t.Optional(t.String()),
	page: t.Optional(t.Numeric()),
	limit: t.Optional(t.Numeric()),
	filter: t.Optional(t.String()),
});

const QueryManySchema = t.Object({
	q: t.String(),
	category: t.Optional(t.String()),
	subcategory: t.Optional(t.String()),
	brand: t.Optional(t.String()),
	min_price: t.Optional(t.Numeric()),
	max_price: t.Optional(t.Numeric()),
	color: t.Optional(t.String()),
	size: t.Optional(t.String()),
	material: t.Optional(t.String()),
	rating: t.Optional(t.Numeric()),
	sort: t.Optional(t.String()),
	order: t.Optional(t.String()),
	page: t.Optional(t.Numeric()),
	limit: t.Optional(t.Numeric()),
	in_stock: t.Optional(t.BooleanString()),
	on_sale: t.Optional(t.BooleanString()),
});

app.post("/json/small", ({ body }) => body);

app.post("/json/medium", ({ body }) => body);

app.post("/json/large", ({ body }) => body);

app.post("/json/very-large", ({ body }) => body);

app.get("/path/simple/:id", ({ params: { id } }) => ({ id }));

app.get("/path/multiple/:user_id/:post_id", ({ params: { user_id, post_id } }) => ({
	user_id,
	post_id,
}));

app.get("/path/deep/:org/:team/:project/:resource/:id", ({ params: { org, team, project, resource, id } }) => ({
	org,
	team,
	project,
	resource,
	id,
}));

app.get("/path/int/:id", ({ params: { id } }) => ({
	id: Number.parseInt(id, 10),
}));

app.get("/path/uuid/:uuid", ({ params: { uuid } }) => ({ uuid }));

app.get("/path/date/:date", ({ params: { date } }) => ({ date }));

app.get("/query/few", ({ query }) => query ?? {});

app.get("/query/medium", ({ query }) => query ?? {});

app.get("/query/many", ({ query }) => query ?? {});

// Validated variants (mirror of above with /validated/ prefix)
app.post("/validated/json/small", ({ body }) => body, {
	body: SmallPayloadSchema,
});

app.post("/validated/json/medium", ({ body }) => body, {
	body: MediumPayloadSchema,
});

app.post("/validated/json/large", ({ body }) => body, {
	body: LargePayloadSchema,
});

app.post("/validated/json/very-large", ({ body }) => body, {
	body: VeryLargePayloadSchema,
});

app.get("/validated/path/simple/:id", ({ params: { id } }) => ({ id }), {
	params: t.Object({
		id: t.String({ minLength: 1, maxLength: 255, pattern: "^[a-zA-Z0-9_-]+$" }),
	}),
});

app.get(
	"/validated/path/multiple/:user_id/:post_id",
	({ params: { user_id, post_id } }) => ({
		user_id,
		post_id,
	}),
	{
		params: t.Object({
			user_id: t.String({ minLength: 1, maxLength: 255, pattern: "^[a-zA-Z0-9_-]+$" }),
			post_id: t.String({ minLength: 1, maxLength: 255, pattern: "^[a-zA-Z0-9_-]+$" }),
		}),
	},
);

app.get(
	"/validated/path/deep/:org/:team/:project/:resource/:id",
	({ params: { org, team, project, resource, id } }) => ({
		org,
		team,
		project,
		resource,
		id,
	}),
	{
		params: t.Object({
			org: t.String({ minLength: 1, maxLength: 255, pattern: "^[a-zA-Z0-9_-]+$" }),
			team: t.String({ minLength: 1, maxLength: 255, pattern: "^[a-zA-Z0-9_-]+$" }),
			project: t.String({ minLength: 1, maxLength: 255, pattern: "^[a-zA-Z0-9_-]+$" }),
			resource: t.String({ minLength: 1, maxLength: 255, pattern: "^[a-zA-Z0-9_-]+$" }),
			id: t.String({ minLength: 1, maxLength: 255, pattern: "^[a-zA-Z0-9_-]+$" }),
		}),
	},
);

app.get(
	"/validated/path/int/:id",
	({ params: { id } }) => ({
		id,
	}),
	{
		params: t.Object({
			id: t.Integer(),
		}),
	},
);

app.get("/validated/path/uuid/:uuid", ({ params: { uuid } }) => ({ uuid }), {
	params: t.Object({
		uuid: t.String({ format: "uuid" }),
	}),
});

app.get("/validated/path/date/:date", ({ params: { date } }) => ({ date }), {
	params: t.Object({
		date: t.String({ format: "date" }),
	}),
});

app.get("/validated/query/few", ({ query }) => query ?? {}, {
	query: QueryFewSchema,
});

app.get("/validated/query/medium", ({ query }) => query ?? {}, {
	query: QueryMediumSchema,
});

app.get("/validated/query/many", ({ query }) => query ?? {}, {
	query: QueryManySchema,
});

app.get("/health", () => ({ status: "ok" }));

app.get("/", () => ({ status: "ok" }));

function resolvePort(defaultPort = 8000): number {
	for (const arg of Bun.argv.slice(2)) {
		const parsed = Number.parseInt(arg, 10);
		if (Number.isFinite(parsed) && parsed >= 0 && parsed < 65536) {
			return parsed;
		}
	}

	const envPort = process.env.PORT ? Number.parseInt(process.env.PORT, 10) : Number.NaN;
	if (Number.isFinite(envPort) && envPort >= 0 && envPort < 65536) {
		return envPort;
	}

	return defaultPort;
}

const port = resolvePort();

app.listen(port, () => {
	console.error(`[elysia] Starting server on port ${port}`);
});
