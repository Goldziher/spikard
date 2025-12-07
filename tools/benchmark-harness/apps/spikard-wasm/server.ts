#!/usr/bin/env -S deno run --allow-net --allow-read
/**
 * Spikard WASM HTTP server for workload benchmarking using Deno.
 *
 * This server implements all workload types to measure WASM binding performance
 * against the pure Rust baseline.
 */

import init, { TestClient } from "./pkg/spikard_wasm.js";

await init();

// Route definitions and handlers for the benchmark app
const routes = [];
const handlers: Record<string, Function> = {};

function registerRoute(method: string, path: string, handler: Function) {
  routes.push({
    method: method.toUpperCase(),
    path,
    handler_name: handler.name,
    is_async: true,
  });
  handlers[handler.name] = handler;
}

function get(path: string) {
  return (handler: Function) => registerRoute('GET', path, handler);
}

function post(path: string) {
  return (handler: Function) => registerRoute('POST', path, handler);
}

// ============================================================================
// JSON Body Workloads
// ============================================================================

post("/json/small")(async function jsonSmall(_body: unknown) {
  return _body;
});

post("/json/medium")(async function jsonMedium(_body: unknown) {
  return _body;
});

post("/json/large")(async function jsonLarge(_body: unknown) {
  return _body;
});

post("/json/very-large")(async function jsonVeryLarge(_body: unknown) {
  return _body;
});

// ============================================================================
// Multipart Form Workloads
// ============================================================================

post("/multipart/small")(async function multipartSmall(_body: unknown) {
  return { files_received: 1, total_bytes: 1024 };
});

post("/multipart/medium")(async function multipartMedium(_body: unknown) {
  return { files_received: 2, total_bytes: 10240 };
});

post("/multipart/large")(async function multipartLarge(_body: unknown) {
  return { files_received: 5, total_bytes: 102400 };
});

// ============================================================================
// URL Encoded Form Workloads
// ============================================================================

post("/urlencoded/simple")(async function urlencodedSimple(_body: unknown) {
  return _body;
});

post("/urlencoded/complex")(async function urlencodedComplex(_body: unknown) {
  return _body;
});

// ============================================================================
// Path Parameter Workloads
// ============================================================================

get("/path/simple/:id")(async function pathSimple(params: any) {
  return { id: params.id };
});

get("/path/multiple/:user_id/:post_id")(async function pathMultiple(params: any) {
  return { user_id: params.user_id, post_id: params.post_id };
});

get("/path/deep/:org/:team/:project/:api/:item")(async function pathDeep(params: any) {
  return {
    org: params.org,
    team: params.team,
    project: params.project,
    api: params.api,
    item: params.item,
  };
});

get("/path/int/:id")(async function pathInt(params: any) {
  return { id: parseInt(params.id, 10) };
});

get("/path/uuid/:id")(async function pathUuid(params: any) {
  return { id: params.id };
});

get("/path/date/:date")(async function pathDate(params: any) {
  return { date: params.date };
});

// ============================================================================
// Query Parameter Workloads
// ============================================================================

get("/query/few")(async function queryFew(query: unknown) {
  return query;
});

get("/query/medium")(async function queryMedium(query: unknown) {
  return query;
});

get("/query/many")(async function queryMany(query: unknown) {
  return query;
});

// ============================================================================
// Health Check
// ============================================================================

get("/health")(async function health() {
  return { status: "ok" };
});

get("/")(async function root() {
  return { status: "ok" };
});

// Create the TestClient
const client = new TestClient(
  JSON.stringify(routes),
  handlers,
  undefined,
  undefined
);

// Start HTTP server
const port = Deno.args[0] ? parseInt(Deno.args[0], 10) : 8000;

console.log(`Starting Spikard WASM server on port ${port}`);

Deno.serve({ port }, async (req) => {
  try {
    const url = new URL(req.url);
    const method = req.method;
    const path = url.pathname;
    const query = Object.fromEntries(url.searchParams);

    let body = null;
    if (method === "POST" && req.body) {
      const contentType = req.headers.get("content-type") || "";
      if (contentType.includes("application/json")) {
        body = await req.json();
      } else {
        body = await req.text();
      }
    }

    const requestPayload = {
      method,
      path,
      query,
      headers: Object.fromEntries(req.headers),
      body,
    };

    const response = await client.handle_request(JSON.stringify(requestPayload));
    const responseData = JSON.parse(response);

    return new Response(
      JSON.stringify(responseData.body),
      {
        status: responseData.status || 200,
        headers: {
          "content-type": "application/json",
          ...(responseData.headers || {}),
        },
      }
    );
  } catch (error) {
    return new Response(
      JSON.stringify({ error: String(error) }),
      {
        status: 500,
        headers: { "content-type": "application/json" },
      }
    );
  }
});
