#!/usr/bin/env node
/**
 * MoroJS benchmark server for workload comparison.
 *
 * MoroJS is a TypeScript-first framework built on uWebSockets.js for maximum performance.
 * Uses Zod for validation matching the Fastify reference implementation.
 */

import { createApp } from "@morojs/moro";
import { z } from "zod";

const app = createApp();

/**
 * Small JSON payload schema (~100 bytes)
 */
const SmallPayloadSchema = z.object({
	name: z.string(),
	description: z.string(),
	price: z.number(),
	tax: z.number().optional().nullable(),
});

/**
 * Address nested schema
 */
const AddressSchema = z.object({
	street: z.string(),
	city: z.string(),
	state: z.string(),
	zip_code: z.string(),
});

/**
 * Medium JSON payload schema (~1KB)
 */
const MediumPayloadSchema = z.object({
	name: z.string(),
	email: z.string(),
	age: z.number().int(),
	address: AddressSchema,
	tags: z.array(z.string()),
});

/**
 * Item nested schema
 */
const ItemSchema = z.object({
	id: z.string(),
	name: z.string(),
	price: z.number(),
	quantity: z.number().int(),
});

/**
 * Large JSON payload schema (~10KB)
 */
const LargePayloadSchema = z.object({
	user_id: z.string(),
	name: z.string(),
	email: z.string(),
	items: z.array(ItemSchema),
	metadata: z.record(z.string(), z.any()),
});

/**
 * Very large JSON payload schema (~100KB)
 */
const VeryLargePayloadSchema = z.object({
	batch_id: z.string(),
	records: z.array(z.record(z.string(), z.any())),
	summary: z.record(z.string(), z.any()),
});

app.post("/json/small", {
	body: SmallPayloadSchema,
	handler: ({ body }) => {
		return body;
	},
});

app.post("/json/medium", {
	body: MediumPayloadSchema,
	handler: ({ body }) => {
		return body;
	},
});

app.post("/json/large", {
	body: LargePayloadSchema,
	handler: ({ body }) => {
		return body;
	},
});

app.post("/json/very-large", {
	body: VeryLargePayloadSchema,
	handler: ({ body }) => {
		return body;
	},
});

app.post("/multipart/small", {
	handler: () => {
		return { files_received: 1, total_bytes: 1024 };
	},
});

app.post("/multipart/medium", {
	handler: () => {
		return { files_received: 2, total_bytes: 10240 };
	},
});

app.post("/multipart/large", {
	handler: () => {
		return { files_received: 5, total_bytes: 102400 };
	},
});

app.post("/urlencoded/simple", {
	handler: ({ req }) => {
		return req.body;
	},
});

app.post("/urlencoded/complex", {
	handler: ({ req }) => {
		return req.body;
	},
});

app.get("/path/simple/:id", {
	handler: ({ req }) => {
		const { id } = req.params;
		return { id };
	},
});

app.get("/path/multiple/:user_id/:post_id", {
	handler: ({ req }) => {
		const { user_id, post_id } = req.params;
		return { user_id, post_id };
	},
});

app.get("/path/deep/:org/:team/:project/:resource/:id", {
	handler: ({ req }) => {
		const { org, team, project, resource, id } = req.params;
		return { org, team, project, resource, id };
	},
});

app.get("/path/int/:id", {
	handler: ({ req }) => {
		const { id } = req.params;
		return { id: parseInt(id, 10) };
	},
});

app.get("/path/uuid/:uuid", {
	handler: ({ req }) => {
		const { uuid } = req.params;
		return { uuid };
	},
});

app.get("/path/date/:date", {
	handler: ({ req }) => {
		const { date } = req.params;
		return { date };
	},
});

app.get("/query/few", {
	handler: ({ req }) => {
		return req.query;
	},
});

app.get("/query/medium", {
	handler: ({ req }) => {
		return req.query;
	},
});

app.get("/query/many", {
	handler: ({ req }) => {
		return req.query;
	},
});

app.get("/health", {
	handler: () => {
		return { status: "ok" };
	},
});

app.get("/", {
	handler: () => {
		return { status: "ok" };
	},
});

const port = process.argv[2] ? parseInt(process.argv[2], 10) : process.env.PORT ? parseInt(process.env.PORT, 10) : 8000;

app.listen(port);
