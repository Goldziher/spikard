/**
 * Spikard WASM HTTP server for workload benchmarking.
 *
 * This server implements all workload types to measure WASM binding performance
 * against the pure Rust baseline.
 */

import init, { Spikard } from "./spikard_wasm.js";

await init();

const app = new Spikard();

// ============================================================================
// JSON Body Workloads
// ============================================================================

app.post("/json/small", async (body) => {
	return body;
});

app.post("/json/medium", async (body) => {
	return body;
});

app.post("/json/large", async (body) => {
	return body;
});

app.post("/json/very-large", async (body) => {
	return body;
});

// ============================================================================
// Multipart Form Workloads
// ============================================================================

app.post("/multipart/small", async (body) => {
	return { files_received: 1, total_bytes: 1024 };
});

app.post("/multipart/medium", async (body) => {
	return { files_received: 2, total_bytes: 10240 };
});

app.post("/multipart/large", async (body) => {
	return { files_received: 5, total_bytes: 102400 };
});

// ============================================================================
// URL Encoded Form Workloads
// ============================================================================

app.post("/urlencoded/simple", async (body) => {
	return body;
});

app.post("/urlencoded/complex", async (body) => {
	return body;
});

// ============================================================================
// Path Parameter Workloads
// ============================================================================

app.get("/path/simple/:id", async (params) => {
	return { id: params.id };
});

app.get("/path/multiple/:user_id/:post_id", async (params) => {
	return { user_id: params.user_id, post_id: params.post_id };
});

app.get("/path/deep/:org/:team/:project/:api/:item", async (params) => {
	return {
		org: params.org,
		team: params.team,
		project: params.project,
		api: params.api,
		item: params.item,
	};
});

app.get("/path/int/:id", async (params) => {
	return { id: parseInt(params.id) };
});

app.get("/path/uuid/:id", async (params) => {
	return { id: params.id };
});

app.get("/path/date/:date", async (params) => {
	return { date: params.date };
});

// ============================================================================
// Query Parameter Workloads
// ============================================================================

app.get("/query/few", async (query) => {
	return query;
});

app.get("/query/medium", async (query) => {
	return query;
});

app.get("/query/many", async (query) => {
	return query;
});

// ============================================================================
// Health Check
// ============================================================================

app.get("/health", async () => {
	return { status: "ok" };
});

app.get("/", async () => {
	return { status: "ok" };
});

// Start server
const port = process.argv[2] ? parseInt(process.argv[2]) : 8000;
app.listen(port, "0.0.0.0");
console.error(`[spikard-wasm-workloads] Server listening on port ${port}`);
