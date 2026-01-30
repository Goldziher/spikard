#!/usr/bin/env node
/**
 * Hono benchmark server - unified raw + validation endpoints
 *
 * Raw endpoints: no validation (original paths)
 * Validated endpoints: Zod validation (under /validated/* prefix)
 */

import { serve } from "@hono/node-server";
import { zValidator } from "@hono/zod-validator";
import { Hono } from "hono";
import { z } from "zod";

const app = new Hono();

// ============================================================================
// RAW ENDPOINTS (No validation)
// ============================================================================

app.post("/json/small", async (c) => {
	const body = await c.req.json();
	return c.json(body);
});

app.post("/json/medium", async (c) => {
	const body = await c.req.json();
	return c.json(body);
});

app.post("/json/large", async (c) => {
	const body = await c.req.json();
	return c.json(body);
});

app.post("/json/very-large", async (c) => {
	const body = await c.req.json();
	return c.json(body);
});

app.post("/multipart/small", async (c) => {
	const formData = await c.req.formData();
	let files_received = 0;
	let total_bytes = 0;

	for (const [key, value] of formData.entries()) {
		if (key.startsWith("file") && value instanceof File) {
			files_received++;
			total_bytes += value.size;
		}
	}

	return c.json({ files_received, total_bytes });
});

app.post("/multipart/medium", async (c) => {
	const formData = await c.req.formData();
	let files_received = 0;
	let total_bytes = 0;

	for (const [key, value] of formData.entries()) {
		if (key.startsWith("file") && value instanceof File) {
			files_received++;
			total_bytes += value.size;
		}
	}

	return c.json({ files_received, total_bytes });
});

app.post("/multipart/large", async (c) => {
	const formData = await c.req.formData();
	let files_received = 0;
	let total_bytes = 0;

	for (const [key, value] of formData.entries()) {
		if (key.startsWith("file") && value instanceof File) {
			files_received++;
			total_bytes += value.size;
		}
	}

	return c.json({ files_received, total_bytes });
});

app.post("/urlencoded/simple", async (c) => {
	const body = await c.req.parseBody();
	return c.json(body || {});
});

app.post("/urlencoded/complex", async (c) => {
	const body = await c.req.parseBody();
	return c.json(body || {});
});

app.get("/path/simple/:id", (c) => {
	const id = c.req.param("id");
	return c.json({ id });
});

app.get("/path/multiple/:user_id/:post_id", (c) => {
	const user_id = c.req.param("user_id");
	const post_id = c.req.param("post_id");
	return c.json({ user_id, post_id });
});

app.get("/path/deep/:org/:team/:project/:resource/:id", (c) => {
	const org = c.req.param("org");
	const team = c.req.param("team");
	const project = c.req.param("project");
	const resource = c.req.param("resource");
	const id = c.req.param("id");
	return c.json({ org, team, project, resource, id });
});

app.get("/path/int/:id", (c) => {
	const id = parseInt(c.req.param("id"), 10);
	return c.json({ id });
});

app.get("/path/uuid/:uuid", (c) => {
	const uuid = c.req.param("uuid");
	return c.json({ uuid });
});

app.get("/path/date/:date", (c) => {
	const date = c.req.param("date");
	return c.json({ date });
});

app.get("/query/few", (c) => {
	const query = c.req.query();
	return c.json(query || {});
});

app.get("/query/medium", (c) => {
	const query = c.req.query();
	return c.json(query || {});
});

app.get("/query/many", (c) => {
	const query = c.req.query();
	return c.json(query || {});
});

app.get("/health", (c) => {
	return c.json({ status: "ok" });
});

app.get("/", (c) => {
	return c.json({ status: "ok" });
});

// ============================================================================
// VALIDATED ENDPOINTS (With Zod validation)
// ============================================================================

const SmallPayloadSchema = z.object({
	name: z.string(),
	description: z.string(),
	price: z.number(),
	tax: z.number(),
});

type SmallPayload = z.infer<typeof SmallPayloadSchema>;

const ImageSchema = z.object({
	url: z.string(),
	name: z.string(),
});

const MediumPayloadSchema = z.object({
	name: z.string(),
	price: z.number(),
	image: ImageSchema,
});

type MediumPayload = z.infer<typeof MediumPayloadSchema>;

const CountrySchema = z.object({
	name: z.string(),
	code: z.string(),
});

const AddressSchema = z.object({
	street: z.string(),
	city: z.string(),
	country: CountrySchema,
});

const SellerSchema = z.object({
	name: z.string(),
	address: AddressSchema,
});

const LargePayloadSchema = z.object({
	name: z.string(),
	price: z.number(),
	seller: SellerSchema,
});

type LargePayload = z.infer<typeof LargePayloadSchema>;

const VeryLargePayloadSchema = z.object({
	name: z.string(),
	tags: z.array(z.string()),
	images: z.array(ImageSchema),
});

type VeryLargePayload = z.infer<typeof VeryLargePayloadSchema>;

const IntParamSchema = z.object({
	id: z.coerce.number().int(),
});

const UuidParamSchema = z.object({
	uuid: z.string().uuid(),
});

const DateParamSchema = z.object({
	date: z.string().date(),
});

const StringParamSimpleSchema = z.object({
	id: z
		.string()
		.min(1)
		.max(255)
		.regex(/^[a-zA-Z0-9_-]+$/),
});

const StringParamMultipleSchema = z.object({
	user_id: z
		.string()
		.min(1)
		.max(255)
		.regex(/^[a-zA-Z0-9_-]+$/),
	post_id: z
		.string()
		.min(1)
		.max(255)
		.regex(/^[a-zA-Z0-9_-]+$/),
});

const StringParamDeepSchema = z.object({
	org: z
		.string()
		.min(1)
		.max(255)
		.regex(/^[a-zA-Z0-9_-]+$/),
	team: z
		.string()
		.min(1)
		.max(255)
		.regex(/^[a-zA-Z0-9_-]+$/),
	project: z
		.string()
		.min(1)
		.max(255)
		.regex(/^[a-zA-Z0-9_-]+$/),
	resource: z
		.string()
		.min(1)
		.max(255)
		.regex(/^[a-zA-Z0-9_-]+$/),
	id: z
		.string()
		.min(1)
		.max(255)
		.regex(/^[a-zA-Z0-9_-]+$/),
});

const UrlencodedSimpleSchema = z.object({
	name: z.string(),
	email: z.string(),
	age: z.union([z.string(), z.coerce.number().int()]),
	subscribe: z.union([z.string(), z.coerce.boolean()]),
});

const UrlencodedComplexSchema = z.object({
	username: z.string(),
	password: z.string(),
	email: z.string(),
	first_name: z.string(),
	last_name: z.string(),
	age: z.union([z.string(), z.coerce.number().int()]),
	country: z.string(),
	state: z.string(),
	city: z.string(),
	zip: z.string(),
	phone: z.string(),
	company: z.string(),
	job_title: z.string(),
	subscribe: z.union([z.string(), z.coerce.boolean()]),
	newsletter: z.union([z.string(), z.coerce.boolean()]),
	terms_accepted: z.union([z.string(), z.coerce.boolean()]),
	privacy_accepted: z.union([z.string(), z.coerce.boolean()]),
	marketing_consent: z.union([z.string(), z.coerce.boolean()]),
	two_factor_enabled: z.union([z.string(), z.coerce.boolean()]),
});

const QueryFewSchema = z.object({
	q: z.string(),
	page: z.coerce.number().int().optional(),
	limit: z.coerce.number().int().optional(),
});

const QueryMediumSchema = z.object({
	search: z.string(),
	category: z.string().optional(),
	sort: z.string().optional(),
	order: z.string().optional(),
	page: z.coerce.number().int().optional(),
	limit: z.coerce.number().int().optional(),
	filter: z.string().optional(),
});

const QueryManySchema = z.object({
	q: z.string(),
	category: z.string().optional(),
	subcategory: z.string().optional(),
	brand: z.string().optional(),
	min_price: z.coerce.number().optional(),
	max_price: z.coerce.number().optional(),
	color: z.string().optional(),
	size: z.string().optional(),
	material: z.string().optional(),
	rating: z.coerce.number().int().optional(),
	sort: z.string().optional(),
	order: z.string().optional(),
	page: z.coerce.number().int().optional(),
	limit: z.coerce.number().int().optional(),
	in_stock: z.coerce.boolean().optional(),
	on_sale: z.coerce.boolean().optional(),
});

app.post("/validated/json/small", zValidator("json", SmallPayloadSchema), (c) => {
	const validated: SmallPayload = c.req.valid("json");
	return c.json(validated);
});

app.post("/validated/json/medium", zValidator("json", MediumPayloadSchema), (c) => {
	const validated: MediumPayload = c.req.valid("json");
	return c.json(validated);
});

app.post("/validated/json/large", zValidator("json", LargePayloadSchema), (c) => {
	const validated: LargePayload = c.req.valid("json");
	return c.json(validated);
});

app.post("/validated/json/very-large", zValidator("json", VeryLargePayloadSchema), (c) => {
	const validated: VeryLargePayload = c.req.valid("json");
	return c.json(validated);
});

interface MultipartResponse {
	files_received: number;
	total_bytes: number;
}

app.post("/validated/multipart/small", async (c) => {
	const formData = await c.req.formData();
	let files_received = 0;
	let total_bytes = 0;

	for (const [key, value] of formData.entries()) {
		if (key.startsWith("file") && value instanceof File) {
			files_received++;
			total_bytes += value.size;
		}
	}

	if (files_received === 0) {
		return c.json({ error: "No files received" }, 400);
	}

	const response: MultipartResponse = { files_received, total_bytes };
	return c.json(response);
});

app.post("/validated/multipart/medium", async (c) => {
	const formData = await c.req.formData();
	let files_received = 0;
	let total_bytes = 0;

	for (const [key, value] of formData.entries()) {
		if (key.startsWith("file") && value instanceof File) {
			files_received++;
			total_bytes += value.size;
		}
	}

	if (files_received === 0) {
		return c.json({ error: "No files received" }, 400);
	}

	const response: MultipartResponse = { files_received, total_bytes };
	return c.json(response);
});

app.post("/validated/multipart/large", async (c) => {
	const formData = await c.req.formData();
	let files_received = 0;
	let total_bytes = 0;

	for (const [key, value] of formData.entries()) {
		if (key.startsWith("file") && value instanceof File) {
			files_received++;
			total_bytes += value.size;
		}
	}

	if (files_received === 0) {
		return c.json({ error: "No files received" }, 400);
	}

	const response: MultipartResponse = { files_received, total_bytes };
	return c.json(response);
});

app.post("/validated/urlencoded/simple", zValidator("form", UrlencodedSimpleSchema), (c) => {
	const validated = c.req.valid("form");
	return c.json(validated);
});

app.post("/validated/urlencoded/complex", zValidator("form", UrlencodedComplexSchema), (c) => {
	const validated = c.req.valid("form");
	return c.json(validated);
});

interface SimpleIdResponse {
	id: string;
}

app.get("/validated/path/simple/:id", zValidator("param", StringParamSimpleSchema), (c) => {
	const { id } = c.req.valid("param");
	const response: SimpleIdResponse = { id };
	return c.json(response);
});

interface MultipleParamsResponse {
	user_id: string;
	post_id: string;
}

app.get("/validated/path/multiple/:user_id/:post_id", zValidator("param", StringParamMultipleSchema), (c) => {
	const { user_id, post_id } = c.req.valid("param");
	const response: MultipleParamsResponse = { user_id, post_id };
	return c.json(response);
});

interface DeepPathResponse {
	org: string;
	team: string;
	project: string;
	resource: string;
	id: string;
}

app.get("/validated/path/deep/:org/:team/:project/:resource/:id", zValidator("param", StringParamDeepSchema), (c) => {
	const { org, team, project, resource, id } = c.req.valid("param");
	const response: DeepPathResponse = { org, team, project, resource, id };
	return c.json(response);
});

interface IntIdResponse {
	id: number;
}

app.get("/validated/path/int/:id", zValidator("param", IntParamSchema), (c) => {
	const { id } = c.req.valid("param");
	const response: IntIdResponse = { id };
	return c.json(response);
});

interface UuidResponse {
	uuid: string;
}

app.get("/validated/path/uuid/:uuid", zValidator("param", UuidParamSchema), (c) => {
	const { uuid } = c.req.valid("param");
	const response: UuidResponse = { uuid };
	return c.json(response);
});

interface DateResponse {
	date: string;
}

app.get("/validated/path/date/:date", zValidator("param", DateParamSchema), (c) => {
	const { date } = c.req.valid("param");
	const response: DateResponse = { date };
	return c.json(response);
});

app.get("/validated/query/few", zValidator("query", QueryFewSchema), (c) => {
	const validated = c.req.valid("query");
	return c.json(validated);
});

app.get("/validated/query/medium", zValidator("query", QueryMediumSchema), (c) => {
	const validated = c.req.valid("query");
	return c.json(validated);
});

app.get("/validated/query/many", zValidator("query", QueryManySchema), (c) => {
	const validated = c.req.valid("query");
	return c.json(validated);
});

// ============================================================================
// Server Startup
// ============================================================================

function resolvePort(defaultPort = 8000): number {
	for (const arg of process.argv.slice(2)) {
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

console.error(`Starting Hono server on http://localhost:${port}`);

serve({
	fetch: app.fetch,
	port,
});
