#!/usr/bin/env bun
/**
 * Elysia benchmark server for workload comparison.
 *
 * Elysia is a high-performance TypeScript framework built for Bun runtime.
 * It features built-in schema validation via TypeBox and excellent type inference.
 *
 * Note: This requires Bun runtime (not Node.js) - install from https://bun.sh
 */

import { Elysia, t } from "elysia";

const app = new Elysia();

// ============================================================================
// Schema Definitions (using Elysia's TypeBox-based validation)
// ============================================================================

const SmallPayloadSchema = t.Object({
  name: t.String(),
  description: t.String(),
  price: t.Number(),
  tax: t.Optional(t.Nullable(t.Number())),
});

const AddressSchema = t.Object({
  street: t.String(),
  city: t.String(),
  state: t.String(),
  zip_code: t.String(),
});

const MediumPayloadSchema = t.Object({
  name: t.String(),
  email: t.String(),
  age: t.Integer(),
  address: AddressSchema,
  tags: t.Array(t.String()),
});

const ItemSchema = t.Object({
  id: t.String(),
  name: t.String(),
  price: t.Number(),
  quantity: t.Integer(),
});

const LargePayloadSchema = t.Object({
  user_id: t.String(),
  name: t.String(),
  email: t.String(),
  items: t.Array(ItemSchema),
  metadata: t.Record(t.String(), t.Any()),
});

const VeryLargePayloadSchema = t.Object({
  batch_id: t.String(),
  records: t.Array(t.Record(t.String(), t.Any())),
  summary: t.Record(t.String(), t.Any()),
});

// ============================================================================
// JSON Body Workloads
// ============================================================================

app.post("/json/small", ({ body }) => body, {
  body: SmallPayloadSchema,
});

app.post("/json/medium", ({ body }) => body, {
  body: MediumPayloadSchema,
});

app.post("/json/large", ({ body }) => body, {
  body: LargePayloadSchema,
});

app.post("/json/very-large", ({ body }) => body, {
  body: VeryLargePayloadSchema,
});

// ============================================================================
// Multipart Form Workloads
// ============================================================================

app.post("/multipart/small", () => ({
  files_received: 1,
  total_bytes: 1024,
}));

app.post("/multipart/medium", () => ({
  files_received: 2,
  total_bytes: 10240,
}));

app.post("/multipart/large", () => ({
  files_received: 5,
  total_bytes: 102400,
}));

// ============================================================================
// URL Encoded Form Workloads
// ============================================================================

app.post("/urlencoded/simple", ({ body }) => body ?? {});

app.post("/urlencoded/complex", ({ body }) => body ?? {});

// ============================================================================
// Path Parameter Workloads
// ============================================================================

app.get("/path/simple/:id", ({ params: { id } }) => ({ id }));

app.get("/path/multiple/:user_id/:post_id", ({ params: { user_id, post_id } }) => ({
  user_id,
  post_id,
}));

app.get(
  "/path/deep/:org/:team/:project/:resource/:id",
  ({ params: { org, team, project, resource, id } }) => ({
    org,
    team,
    project,
    resource,
    id,
  }),
);

app.get("/path/int/:id", ({ params: { id } }) => ({
  id: parseInt(id, 10),
}));

app.get("/path/uuid/:uuid", ({ params: { uuid } }) => ({ uuid }));

app.get("/path/date/:date", ({ params: { date } }) => ({ date }));

// ============================================================================
// Query Parameter Workloads
// ============================================================================

app.get("/query/few", ({ query }) => query ?? {});

app.get("/query/medium", ({ query }) => query ?? {});

app.get("/query/many", ({ query }) => query ?? {});

// ============================================================================
// Health Check
// ============================================================================

app.get("/health", () => ({ status: "ok" }));

app.get("/", () => ({ status: "ok" }));

// ============================================================================
// Server Startup
// ============================================================================

const port = Bun.argv[2] ? parseInt(Bun.argv[2], 10) : 8000;

app.listen(port, () => {
  console.error(`[elysia] Starting server on port ${port}`);
});
