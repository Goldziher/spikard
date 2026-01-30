#!/usr/bin/env tsx
/**
 * Kito benchmark server - unified raw + validation endpoints
 *
 * Raw endpoints: no validation (original paths)
 * Validated endpoints: Kito schema validation (under /validated/* prefix)
 */

import { schema, server, t } from "kitojs";

const app = server();

// ============================================================================
// RAW ENDPOINTS (No validation)
// ============================================================================

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

app.post("/multipart/small", async ({ req, res }) => {
	const formData = await req.formData();
	let files_received = 0;
	let total_bytes = 0;

	for (const [key, value] of formData.entries()) {
		if (key.startsWith("file") && value instanceof File) {
			files_received++;
			total_bytes += value.size;
		}
	}

	res.json({ files_received, total_bytes });
});

app.post("/multipart/medium", async ({ req, res }) => {
	const formData = await req.formData();
	let files_received = 0;
	let total_bytes = 0;

	for (const [key, value] of formData.entries()) {
		if (key.startsWith("file") && value instanceof File) {
			files_received++;
			total_bytes += value.size;
		}
	}

	res.json({ files_received, total_bytes });
});

app.post("/multipart/large", async ({ req, res }) => {
	const formData = await req.formData();
	let files_received = 0;
	let total_bytes = 0;

	for (const [key, value] of formData.entries()) {
		if (key.startsWith("file") && value instanceof File) {
			files_received++;
			total_bytes += value.size;
		}
	}

	res.json({ files_received, total_bytes });
});

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

// ============================================================================
// VALIDATED ENDPOINTS (With Kito schema validation)
// ============================================================================

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
		date: t.str().regex(/^\d{4}-\d{2}-\d{2}$/),
	}),
});

const stringParamSimpleSchema = schema({
	params: t.object({
		id: t.str().min(1).max(255).regex(/^[a-zA-Z0-9_-]+$/),
	}),
});

const stringParamMultipleSchema = schema({
	params: t.object({
		user_id: t.str().min(1).max(255).regex(/^[a-zA-Z0-9_-]+$/),
		post_id: t.str().min(1).max(255).regex(/^[a-zA-Z0-9_-]+$/),
	}),
});

const stringParamDeepSchema = schema({
	params: t.object({
		org: t.str().min(1).max(255).regex(/^[a-zA-Z0-9_-]+$/),
		team: t.str().min(1).max(255).regex(/^[a-zA-Z0-9_-]+$/),
		project: t.str().min(1).max(255).regex(/^[a-zA-Z0-9_-]+$/),
		resource: t.str().min(1).max(255).regex(/^[a-zA-Z0-9_-]+$/),
		id: t.str().min(1).max(255).regex(/^[a-zA-Z0-9_-]+$/),
	}),
});

const urlencodedSimpleSchema = schema({
	body: t.object({
		name: t.str(),
		email: t.str(),
		age: t.str(),
		subscribe: t.str(),
	}),
});

const urlencodedComplexSchema = schema({
	body: t.object({
		username: t.str(),
		password: t.str(),
		email: t.str(),
		first_name: t.str(),
		last_name: t.str(),
		age: t.str(),
		country: t.str(),
		state: t.str(),
		city: t.str(),
		zip: t.str(),
		phone: t.str(),
		company: t.str(),
		job_title: t.str(),
		subscribe: t.str(),
		newsletter: t.str(),
		terms_accepted: t.str(),
		privacy_accepted: t.str(),
		marketing_consent: t.str(),
		two_factor_enabled: t.str(),
	}),
});

const queryFewSchema = schema({
	query: t.object({
		q: t.str(),
		page: t.num().int().optional(),
		limit: t.num().int().optional(),
	}),
});

const queryMediumSchema = schema({
	query: t.object({
		search: t.str(),
		category: t.str().optional(),
		sort: t.str().optional(),
		order: t.str().optional(),
		page: t.num().int().optional(),
		limit: t.num().int().optional(),
		filter: t.str().optional(),
	}),
});

const queryManySchema = schema({
	query: t.object({
		q: t.str(),
		category: t.str().optional(),
		subcategory: t.str().optional(),
		brand: t.str().optional(),
		min_price: t.num().optional(),
		max_price: t.num().optional(),
		color: t.str().optional(),
		size: t.str().optional(),
		material: t.str().optional(),
		rating: t.num().int().optional(),
		sort: t.str().optional(),
		order: t.str().optional(),
		page: t.num().int().optional(),
		limit: t.num().int().optional(),
		in_stock: t.bool().optional(),
		on_sale: t.bool().optional(),
	}),
});

app.post(
	"/validated/json/small",
	({ req, res }) => {
		res.json(req.body);
	},
	smallPayloadSchema,
);

app.post(
	"/validated/json/medium",
	({ req, res }) => {
		res.json(req.body);
	},
	mediumPayloadSchema,
);

app.post(
	"/validated/json/large",
	({ req, res }) => {
		res.json(req.body);
	},
	largePayloadSchema,
);

app.post(
	"/validated/json/very-large",
	({ req, res }) => {
		res.json(req.body);
	},
	veryLargePayloadSchema,
);

app.post("/validated/multipart/small", async ({ req, res }) => {
	const formData = await req.formData();
	let files_received = 0;
	let total_bytes = 0;

	for (const [key, value] of formData.entries()) {
		if (key.startsWith("file") && value instanceof File) {
			files_received++;
			total_bytes += value.size;
		}
	}

	if (files_received === 0) {
		res.status(400).json({ error: "No files received" });
		return;
	}

	res.json({ files_received, total_bytes });
});

app.post("/validated/multipart/medium", async ({ req, res }) => {
	const formData = await req.formData();
	let files_received = 0;
	let total_bytes = 0;

	for (const [key, value] of formData.entries()) {
		if (key.startsWith("file") && value instanceof File) {
			files_received++;
			total_bytes += value.size;
		}
	}

	if (files_received === 0) {
		res.status(400).json({ error: "No files received" });
		return;
	}

	res.json({ files_received, total_bytes });
});

app.post("/validated/multipart/large", async ({ req, res }) => {
	const formData = await req.formData();
	let files_received = 0;
	let total_bytes = 0;

	for (const [key, value] of formData.entries()) {
		if (key.startsWith("file") && value instanceof File) {
			files_received++;
			total_bytes += value.size;
		}
	}

	if (files_received === 0) {
		res.status(400).json({ error: "No files received" });
		return;
	}

	res.json({ files_received, total_bytes });
});

app.post(
	"/validated/urlencoded/simple",
	({ req, res }) => {
		res.json(req.body);
	},
	urlencodedSimpleSchema,
);

app.post(
	"/validated/urlencoded/complex",
	({ req, res }) => {
		res.json(req.body);
	},
	urlencodedComplexSchema,
);

app.get(
	"/validated/path/simple/:id",
	({ req, res }) => {
		res.json({ id: req.params.id });
	},
	stringParamSimpleSchema,
);

app.get(
	"/validated/path/multiple/:user_id/:post_id",
	({ req, res }) => {
		res.json({
			user_id: req.params.user_id,
			post_id: req.params.post_id,
		});
	},
	stringParamMultipleSchema,
);

app.get(
	"/validated/path/deep/:org/:team/:project/:resource/:id",
	({ req, res }) => {
		res.json({
			org: req.params.org,
			team: req.params.team,
			project: req.params.project,
			resource: req.params.resource,
			id: req.params.id,
		});
	},
	stringParamDeepSchema,
);

app.get(
	"/validated/path/int/:id",
	({ req, res }) => {
		res.json({ id: req.params.id });
	},
	intParamSchema,
);

app.get(
	"/validated/path/uuid/:uuid",
	({ req, res }) => {
		res.json({ uuid: req.params.uuid });
	},
	uuidParamSchema,
);

app.get(
	"/validated/path/date/:date",
	({ req, res }) => {
		res.json({ date: req.params.date });
	},
	dateParamSchema,
);

app.get(
	"/validated/query/few",
	({ req, res }) => {
		res.json(req.query || {});
	},
	queryFewSchema,
);

app.get(
	"/validated/query/medium",
	({ req, res }) => {
		res.json(req.query || {});
	},
	queryMediumSchema,
);

app.get(
	"/validated/query/many",
	({ req, res }) => {
		res.json(req.query || {});
	},
	queryManySchema,
);

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

app.listen(port, () => {
	console.error(`[kito] Starting server on port ${port}`);
});
