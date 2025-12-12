#!/usr/bin/env node
/**
 * Hono RAW comparison server for benchmarking
 *
 * Implements all workload types WITHOUT validation to measure Hono's raw performance.
 * NO zod validation - accepts any JSON body and echoes it back.
 * Runs on Node.js via @hono/node-server.
 */

import { serve } from "@hono/node-server";
import { Hono } from "hono";

const app = new Hono();

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

const port = process.argv[2] ? parseInt(process.argv[2], 10) : process.env.PORT ? parseInt(process.env.PORT, 10) : 8000;
