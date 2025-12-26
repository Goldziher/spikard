#!/usr/bin/env node
/**
 * MoroJS benchmark server for workload comparison (raw mode - no validation).
 *
 * MoroJS is a TypeScript-first framework built on uWebSockets.js for maximum performance.
 * This raw mode skips validation for pure performance testing.
 */

import { createApp } from "@morojs/moro";

const app = createApp({
	autoDiscover: false,
	logging: {
		level: "error",
		outputs: {
			console: false,
		},
	},
	server: {
		requestLogging: {
			enabled: false,
		},
	},
});

// JSON body routes - no validation, direct echo
app.post("/json/small").handler((req, res) => res.json(req.body));
app.post("/json/medium").handler((req, res) => res.json(req.body));
app.post("/json/large").handler((req, res) => res.json(req.body));
app.post("/json/very-large").handler((req, res) => res.json(req.body));

// Multipart form routes (mock responses)
app.post("/multipart/small").handler((_req, res) => res.json({ files_received: 1, total_bytes: 1024 }));
app.post("/multipart/medium").handler((_req, res) => res.json({ files_received: 2, total_bytes: 10240 }));
app.post("/multipart/large").handler((_req, res) => res.json({ files_received: 5, total_bytes: 102400 }));

// URL-encoded routes
app.post("/urlencoded/simple").handler((req, res) => res.json(req.body));
app.post("/urlencoded/complex").handler((req, res) => res.json(req.body));

// Path parameter routes
app.get("/path/simple/:id").handler((req, res) => res.json({ id: req.params.id }));

app.get("/path/multiple/:user_id/:post_id").handler((req, res) =>
	res.json({ user_id: req.params.user_id, post_id: req.params.post_id }),
);

app.get("/path/deep/:org/:team/:project/:resource/:id").handler((req, res) =>
	res.json({
		org: req.params.org,
		team: req.params.team,
		project: req.params.project,
		resource: req.params.resource,
		id: req.params.id,
	}),
);

app.get("/path/int/:id").handler((req, res) => res.json({ id: Number.parseInt(req.params.id, 10) }));
app.get("/path/uuid/:uuid").handler((req, res) => res.json({ uuid: req.params.uuid }));
app.get("/path/date/:date").handler((req, res) => res.json({ date: req.params.date }));

// Query parameter routes
app.get("/query/few").handler((req, res) => res.json(req.query));
app.get("/query/medium").handler((req, res) => res.json(req.query));
app.get("/query/many").handler((req, res) => res.json(req.query));

// Health check routes
app.get("/health").handler((_req, res) => res.json({ status: "ok" }));
app.get("/").handler((_req, res) => res.json({ status: "ok" }));

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

app.listen(port);
