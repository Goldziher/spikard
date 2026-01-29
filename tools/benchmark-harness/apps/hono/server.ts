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

app.post("/multipart/small", (c) => {
	return c.json({ files_received: 1, total_bytes: 1024 });
});

app.post("/multipart/medium", (c) => {
	return c.json({ files_received: 2, total_bytes: 10240 });
});

app.post("/multipart/large", (c) => {
	return c.json({ files_received: 5, total_bytes: 102400 });
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

app.post("/validated/multipart/small", (c) => {
	const response: MultipartResponse = { files_received: 1, total_bytes: 1024 };
	return c.json(response);
});

app.post("/validated/multipart/medium", (c) => {
	const response: MultipartResponse = { files_received: 2, total_bytes: 10240 };
	return c.json(response);
});

app.post("/validated/multipart/large", (c) => {
	const response: MultipartResponse = { files_received: 5, total_bytes: 102400 };
	return c.json(response);
});

app.post("/validated/urlencoded/simple", async (c) => {
	const body: Record<string, string | File> = await c.req.parseBody();
	return c.json(body);
});

app.post("/validated/urlencoded/complex", async (c) => {
	const body: Record<string, string | File> = await c.req.parseBody();
	return c.json(body);
});

interface SimpleIdResponse {
	id: string;
}

app.get("/validated/path/simple/:id", (c) => {
	const id: string = c.req.param("id");
	const response: SimpleIdResponse = { id };
	return c.json(response);
});

interface MultipleParamsResponse {
	user_id: string;
	post_id: string;
}

app.get("/validated/path/multiple/:user_id/:post_id", (c) => {
	const user_id: string = c.req.param("user_id");
	const post_id: string = c.req.param("post_id");
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

app.get("/validated/path/deep/:org/:team/:project/:resource/:id", (c) => {
	const org: string = c.req.param("org");
	const team: string = c.req.param("team");
	const project: string = c.req.param("project");
	const resource: string = c.req.param("resource");
	const id: string = c.req.param("id");
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

app.get("/validated/query/few", (c) => {
	const query: Record<string, string> = c.req.query();
	return c.json(query);
});

app.get("/validated/query/medium", (c) => {
	const query: Record<string, string> = c.req.query();
	return c.json(query);
});

app.get("/validated/query/many", (c) => {
	const query: Record<string, string> = c.req.query();
	return c.json(query);
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

console.log(`Starting Hono server on http://localhost:${port}`);

serve({
	fetch: app.fetch,
	port,
});
