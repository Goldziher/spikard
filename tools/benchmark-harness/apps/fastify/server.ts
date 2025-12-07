#!/usr/bin/env node
/**
 * Fastify benchmark server for workload comparison.
 *
 * Fastify is a high-performance Node.js web framework with built-in JSON Schema validation.
 */

import Fastify from "fastify";

const fastify = Fastify({ logger: false });

// ============================================================================
// JSON Schema Definitions
// ============================================================================

const smallPayloadSchema = {
  type: "object",
  required: ["name", "description", "price"],
  properties: {
    name: { type: "string" },
    description: { type: "string" },
    price: { type: "number" },
    tax: { type: "number", nullable: true },
  },
} as const;

const mediumPayloadSchema = {
  type: "object",
  required: ["name", "email", "age", "address", "tags"],
  properties: {
    name: { type: "string" },
    email: { type: "string" },
    age: { type: "integer" },
    address: {
      type: "object",
      required: ["street", "city", "state", "zip_code"],
      properties: {
        street: { type: "string" },
        city: { type: "string" },
        state: { type: "string" },
        zip_code: { type: "string" },
      },
    },
    tags: {
      type: "array",
      items: { type: "string" },
    },
  },
} as const;

const largePayloadSchema = {
  type: "object",
  required: ["user_id", "name", "email", "items", "metadata"],
  properties: {
    user_id: { type: "string" },
    name: { type: "string" },
    email: { type: "string" },
    items: {
      type: "array",
      items: {
        type: "object",
        required: ["id", "name", "price", "quantity"],
        properties: {
          id: { type: "string" },
          name: { type: "string" },
          price: { type: "number" },
          quantity: { type: "integer" },
        },
      },
    },
    metadata: {
      type: "object",
      additionalProperties: true,
    },
  },
} as const;

const veryLargePayloadSchema = {
  type: "object",
  required: ["batch_id", "records", "summary"],
  properties: {
    batch_id: { type: "string" },
    records: {
      type: "array",
      items: {
        type: "object",
        additionalProperties: true,
      },
    },
    summary: {
      type: "object",
      additionalProperties: true,
    },
  },
} as const;

const urlencodedSimpleSchema = {
  type: "object",
  properties: {
    username: { type: "string" },
    password: { type: "string" },
  },
} as const;

const urlencodedComplexSchema = {
  type: "object",
  additionalProperties: true,
} as const;

// ============================================================================
// JSON Body Workloads
// ============================================================================

fastify.post("/json/small", {
  schema: {
    body: smallPayloadSchema,
  },
  handler: async (request, reply) => {
    return request.body;
  },
});

fastify.post("/json/medium", {
  schema: {
    body: mediumPayloadSchema,
  },
  handler: async (request, reply) => {
    return request.body;
  },
});

fastify.post("/json/large", {
  schema: {
    body: largePayloadSchema,
  },
  handler: async (request, reply) => {
    return request.body;
  },
});

fastify.post("/json/very-large", {
  schema: {
    body: veryLargePayloadSchema,
  },
  handler: async (request, reply) => {
    return request.body;
  },
});

// ============================================================================
// Multipart Form Workloads
// ============================================================================

fastify.post("/multipart/small", async (request, reply) => {
  // Mock response for multipart form (~1KB)
  return { files_received: 1, total_bytes: 1024 };
});

fastify.post("/multipart/medium", async (request, reply) => {
  // Mock response for multipart form (~10KB)
  return { files_received: 2, total_bytes: 10240 };
});

fastify.post("/multipart/large", async (request, reply) => {
  // Mock response for multipart form (~100KB)
  return { files_received: 5, total_bytes: 102400 };
});

// ============================================================================
// URL Encoded Form Workloads
// ============================================================================

fastify.post("/urlencoded/simple", {
  schema: {
    body: urlencodedSimpleSchema,
  },
  handler: async (request, reply) => {
    return request.body;
  },
});

fastify.post("/urlencoded/complex", {
  schema: {
    body: urlencodedComplexSchema,
  },
  handler: async (request, reply) => {
    return request.body;
  },
});

// ============================================================================
// Path Parameter Workloads
// ============================================================================

fastify.get("/path/simple/:id", async (request, reply) => {
  const { id } = request.params as { id: string };
  return { id };
});

fastify.get("/path/multiple/:user_id/:post_id", async (request, reply) => {
  const { user_id, post_id } = request.params as {
    user_id: string;
    post_id: string;
  };
  return { user_id, post_id };
});

fastify.get(
  "/path/deep/:org/:team/:project/:resource/:id",
  async (request, reply) => {
    const { org, team, project, resource, id } = request.params as {
      org: string;
      team: string;
      project: string;
      resource: string;
      id: string;
    };
    return { org, team, project, resource, id };
  },
);

fastify.get("/path/int/:id", async (request, reply) => {
  const { id } = request.params as { id: string };
  return { id: parseInt(id, 10) };
});

fastify.get("/path/uuid/:uuid", async (request, reply) => {
  const { uuid } = request.params as { uuid: string };
  return { uuid };
});

fastify.get("/path/date/:date", async (request, reply) => {
  const { date } = request.params as { date: string };
  return { date };
});

// ============================================================================
// Query Parameter Workloads
// ============================================================================

fastify.get("/query/few", async (request, reply) => {
  // Few query parameters (1-2)
  return request.query;
});

fastify.get("/query/medium", async (request, reply) => {
  // Medium query parameters (3-5)
  return request.query;
});

fastify.get("/query/many", async (request, reply) => {
  // Many query parameters (6-10)
  return request.query;
});

// ============================================================================
// Health Check
// ============================================================================

fastify.get("/health", async (request, reply) => {
  return { status: "ok" };
});

fastify.get("/", async (request, reply) => {
  return { status: "ok" };
});

// ============================================================================
// Server Startup
// ============================================================================

const port = process.argv[2]
  ? parseInt(process.argv[2], 10)
  : process.env.PORT
  ? parseInt(process.env.PORT, 10)
  : 8000;

const start = async () => {
  try {
    await fastify.listen({ port, host: "0.0.0.0" });
    console.error(`[fastify] Starting server on port ${port}`);
  } catch (err) {
    console.error(err);
    process.exit(1);
  }
};

start();
