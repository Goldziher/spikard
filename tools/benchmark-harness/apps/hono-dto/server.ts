#!/usr/bin/env node
/**
 * Hono comparison server for benchmarking
 *
 * Implements all workload types to match spikard-node server exactly.
 * Uses zod for validation via @hono/zod-validator.
 * Runs on Node.js via @hono/node-server (Hono is designed for edge but supports Node.js).
 */

import { serve } from "@hono/node-server";
import { zValidator } from "@hono/zod-validator";
import { Hono } from "hono";
import { z } from "zod";

const app = new Hono();

/**
 * Small JSON payload schema (~100 bytes)
 */
const SmallPayloadSchema = z.object({
	name: z.string(),
	description: z.string(),
	price: z.number(),
	tax: z.number(),
});

type SmallPayload = z.infer<typeof SmallPayloadSchema>;

/**
 * Address nested schema
 */
const ImageSchema = z.object({
	url: z.string(),
	name: z.string(),
});

/**
 * Medium JSON payload schema (~1KB)
 */
const MediumPayloadSchema = z.object({
	name: z.string(),
	price: z.number(),
	image: ImageSchema,
});

type MediumPayload = z.infer<typeof MediumPayloadSchema>;

/**
 * Item nested schema
 */
const CountrySchema = z.object({
	name: z.string(),
	code: z.string(),
});

/**
 * Large JSON payload schema (~10KB)
 */
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

/**
 * Very large JSON payload schema (~100KB)
 */
const VeryLargePayloadSchema = z.object({
	name: z.string(),
	tags: z.array(z.string()),
	images: z.array(ImageSchema),
});

type VeryLargePayload = z.infer<typeof VeryLargePayloadSchema>;

app.post("/json/small", zValidator("json", SmallPayloadSchema), (c) => {
	const validated: SmallPayload = c.req.valid("json");
	return c.json(validated);
});

app.post("/json/medium", zValidator("json", MediumPayloadSchema), (c) => {
	const validated: MediumPayload = c.req.valid("json");
	return c.json(validated);
});

app.post("/json/large", zValidator("json", LargePayloadSchema), (c) => {
	const validated: LargePayload = c.req.valid("json");
	return c.json(validated);
});

app.post("/json/very-large", zValidator("json", VeryLargePayloadSchema), (c) => {
	const validated: VeryLargePayload = c.req.valid("json");
	return c.json(validated);
});

interface MultipartResponse {
	files_received: number;
	total_bytes: number;
}

app.post("/multipart/small", (c) => {
	const response: MultipartResponse = { files_received: 1, total_bytes: 1024 };
	return c.json(response);
});

app.post("/multipart/medium", (c) => {
	const response: MultipartResponse = { files_received: 2, total_bytes: 10240 };
	return c.json(response);
});

app.post("/multipart/large", (c) => {
	const response: MultipartResponse = { files_received: 5, total_bytes: 102400 };
	return c.json(response);
});

app.post("/urlencoded/simple", async (c) => {
	const body: Record<string, string | File> = await c.req.parseBody();
	return c.json(body);
});

app.post("/urlencoded/complex", async (c) => {
	const body: Record<string, string | File> = await c.req.parseBody();
	return c.json(body);
});

interface SimpleIdResponse {
	id: string;
}

app.get("/path/simple/:id", (c) => {
	const id: string = c.req.param("id");
	const response: SimpleIdResponse = { id };
	return c.json(response);
});

interface MultipleParamsResponse {
	user_id: string;
	post_id: string;
}

app.get("/path/multiple/:user_id/:post_id", (c) => {
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

app.get("/path/deep/:org/:team/:project/:resource/:id", (c) => {
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

app.get("/path/int/:id", (c) => {
	const id: number = Number.parseInt(c.req.param("id"), 10);
	const response: IntIdResponse = { id };
	return c.json(response);
});

interface UuidResponse {
	uuid: string;
}

app.get("/path/uuid/:uuid", (c) => {
	const uuid: string = c.req.param("uuid");
	const response: UuidResponse = { uuid };
	return c.json(response);
});

interface DateResponse {
	date: string;
}

app.get("/path/date/:date", (c) => {
	const date: string = c.req.param("date");
	const response: DateResponse = { date };
	return c.json(response);
});

app.get("/query/few", (c) => {
	const query: Record<string, string> = c.req.query();
	return c.json(query);
});

app.get("/query/medium", (c) => {
	const query: Record<string, string> = c.req.query();
	return c.json(query);
});

app.get("/query/many", (c) => {
	const query: Record<string, string> = c.req.query();
	return c.json(query);
});

interface HealthResponse {
	status: string;
}

app.get("/health", (c) => {
	const response: HealthResponse = { status: "ok" };
	return c.json(response);
});

app.get("/", (c) => {
	const response: HealthResponse = { status: "ok" };
	return c.json(response);
});

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
