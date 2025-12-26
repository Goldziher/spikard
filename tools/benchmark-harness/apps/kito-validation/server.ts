#!/usr/bin/env tsx
/**
 * Kito DTO comparison server for benchmarking
 *
 * Implements all workload types WITH validation to measure Kito's DTO performance.
 * Uses Kito's built-in schema validation.
 */

import { schema, server, t } from "kitojs";

const app = server();

const smallPayloadSchema = schema({
	body: t.object({
		name: t.str(),
		description: t.str(),
		price: t.num(),
		tax: t.num(),
	}),
});

const mediumPayloadSchema = schema({
	body: t.object({
		name: t.str(),
		price: t.num(),
		image: t.object({
			url: t.str(),
			name: t.str(),
		}),
	}),
});

const largePayloadSchema = schema({
	body: t.object({
		name: t.str(),
		price: t.num(),
		seller: t.object({
			name: t.str(),
			address: t.object({
				street: t.str(),
				city: t.str(),
				country: t.object({
					name: t.str(),
					code: t.str(),
				}),
			}),
		}),
	}),
});

const veryLargePayloadSchema = schema({
	body: t.object({
		name: t.str(),
		tags: t.array(t.str()),
		images: t.array(
			t.object({
				url: t.str(),
				name: t.str(),
			}),
		),
	}),
});

const intParamSchema = schema({
	params: t.object({
		id: t.num().int(),
	}),
});

const uuidParamSchema = schema({
	params: t.object({
		uuid: t.str().uuid(),
	}),
});

const dateParamSchema = schema({
	params: t.object({
		date: t.str().date(),
	}),
});

app.post(
	"/json/small",
	({ req, res }) => {
		res.json(req.body);
	},
	smallPayloadSchema,
);

app.post(
	"/json/medium",
	({ req, res }) => {
		res.json(req.body);
	},
	mediumPayloadSchema,
);

app.post(
	"/json/large",
	({ req, res }) => {
		res.json(req.body);
	},
	largePayloadSchema,
);

app.post(
	"/json/very-large",
	({ req, res }) => {
		res.json(req.body);
	},
	veryLargePayloadSchema,
);

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
	res.json({
		user_id: req.params.user_id,
		post_id: req.params.post_id,
	});
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

app.get(
	"/path/int/:id",
	({ req, res }) => {
		res.json({ id: req.params.id });
	},
	intParamSchema,
);

app.get(
	"/path/uuid/:uuid",
	({ req, res }) => {
		res.json({ uuid: req.params.uuid });
	},
	uuidParamSchema,
);

app.get(
	"/path/date/:date",
	({ req, res }) => {
		res.json({ date: req.params.date });
	},
	dateParamSchema,
);

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

	const envPort = process.env.PORT
		? Number.parseInt(process.env.PORT, 10)
		: Number.NaN;
	if (Number.isFinite(envPort) && envPort >= 0 && envPort < 65536) {
		return envPort;
	}

	return defaultPort;
}

const port = resolvePort();

app.listen(port, () => {
	console.error(`[kito-validation] Starting server on port ${port}`);
});
