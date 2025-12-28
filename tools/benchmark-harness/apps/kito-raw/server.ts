#!/usr/bin/env tsx
/**
 * Kito RAW comparison server for benchmarking
 *
 * Implements all workload types WITHOUT validation to measure Kito's raw performance.
 * NO schema validation - accepts any JSON body and echoes it back.
 * Runs on Node.js, Bun, or Deno via kitojs.
 */

import { server } from "kitojs";

const app = server();

app.post("/json/small", ({ req, res }) => {
	res.json(req.body);
});

app.post("/json/medium", ({ req, res }) => {
	res.json(req.body);
});

app.post("/json/large", ({ req, res }) => {
	res.json(req.body);
});

app.post("/json/very-large", ({ req, res }) => {
	res.json(req.body);
});

app.post("/multipart/small", ({ res }) => {
	res.json({ files_received: 1, total_bytes: 1024 });
});

app.post("/multipart/medium", ({ res }) => {
	res.json({ files_received: 2, total_bytes: 10240 });
});

app.post("/multipart/large", ({ res }) => {
	res.json({ files_received: 5, total_bytes: 102400 });
});

function parseUrlencoded(body: string): Record<string, string | string[]> {
	if (!body) {
		return {};
	}

	const params = new URLSearchParams(body);
	const result: Record<string, string | string[]> = {};

	for (const [key, value] of params) {
		const existing = result[key];
		if (existing === undefined) {
			result[key] = value;
		} else if (Array.isArray(existing)) {
			existing.push(value);
		} else {
			result[key] = [existing, value];
		}
	}

	return result;
}

app.post("/urlencoded/simple", ({ req, res }) => {
	res.json(parseUrlencoded(req.text()));
});

app.post("/urlencoded/complex", ({ req, res }) => {
	res.json(parseUrlencoded(req.text()));
});

app.get("/path/simple/:id", ({ req, res }) => {
	res.json({ id: req.params.id });
});

app.get("/path/multiple/:user_id/:post_id", ({ req, res }) => {
	res.json({ user_id: req.params.user_id, post_id: req.params.post_id });
});

app.get("/path/deep/:org/:team/:project/:resource/:id", ({ req, res }) => {
	res.json({
		org: req.params.org,
		team: req.params.team,
		project: req.params.project,
		resource: req.params.resource,
		id: req.params.id,
	});
});

app.get("/path/int/:id", ({ req, res }) => {
	const id = Number.parseInt(req.params.id as string, 10);
	res.json({ id });
});

app.get("/path/uuid/:uuid", ({ req, res }) => {
	res.json({ uuid: req.params.uuid });
});

app.get("/path/date/:date", ({ req, res }) => {
	res.json({ date: req.params.date });
});

app.get("/query/few", ({ req, res }) => {
	res.json(req.query || {});
});

app.get("/query/medium", ({ req, res }) => {
	res.json(req.query || {});
});

app.get("/query/many", ({ req, res }) => {
	res.json(req.query || {});
});

app.get("/health", ({ res }) => {
	res.json({ status: "ok" });
});

app.get("/", ({ res }) => {
	res.json({ status: "ok" });
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

app.listen(port, () => {
	console.error(`[kito-raw] Starting server on port ${port}`);
});
