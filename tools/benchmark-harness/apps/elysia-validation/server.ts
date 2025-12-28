#!/usr/bin/env bun
/**
 * Elysia benchmark server for workload comparison.
 *
 * Elysia is a high-performance TypeScript framework built for Bun runtime.
 * It features built-in schema validation via TypeBox and excellent type inference.
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

app.post("/json/small", ({ body }) => body, {
	body: SmallPayloadSchema,
});

app.post("/json/medium", ({ body }) => body, {
	body: MediumPayloadSchema,
});

app.post("/json/large", ({ body }) => body, {
	body: LargePayloadSchema,
});

app.post("/json/very-large", ({ body }) => body, {
	body: VeryLargePayloadSchema,
});

app.post("/multipart/small", () => ({
	files_received: 1,
	total_bytes: 1024,
}));

app.post("/multipart/medium", () => ({
	files_received: 2,
	total_bytes: 10240,
}));

app.post("/multipart/large", () => ({
	files_received: 5,
	total_bytes: 102400,
}));

app.post("/urlencoded/simple", ({ body }) => body ?? {});

app.post("/urlencoded/complex", ({ body }) => body ?? {});

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

app.get(
	"/path/int/:id",
	({ params: { id } }) => ({
		id,
	}),
	{
		params: t.Object({
			id: t.Integer(),
		}),
	},
);

app.get("/path/uuid/:uuid", ({ params: { uuid } }) => ({ uuid }), {
	params: t.Object({
		uuid: t.String({ format: "uuid" }),
	}),
});

app.get("/path/date/:date", ({ params: { date } }) => ({ date }), {
	params: t.Object({
		date: t.String({ format: "date" }),
	}),
});

app.get("/query/few", ({ query }) => query ?? {});

app.get("/query/medium", ({ query }) => query ?? {});

app.get("/query/many", ({ query }) => query ?? {});

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
