#!/usr/bin/env node
/**
 * Spikard Node.js HTTP server for workload benchmarking.
 *
 * This server implements all workload types to measure Node.js binding performance.
 */

// Import Zod for validation
const { z } = require('zod');

// Route registration arrays
const routes = [];
const handlers = {};

// Helper functions to register routes
function registerRoute(method, path, handler) {
  const metadata = {
    method: method.toUpperCase(),
    path,
    handler_name: handler.name,
    is_async: true,
  };
  routes.push(metadata);
  handlers[handler.name] = handler;
  return handler;
}

function get(path) {
  return (handler) => registerRoute('GET', path, handler);
}

function post(path) {
  return (handler) => registerRoute('POST', path, handler);
}

// ============================================================================
// Zod Schemas for Validation
// ============================================================================

const SmallPayloadSchema = z.object({
  name: z.string(),
  description: z.string(),
  price: z.number(),
  tax: z.number().optional(),
});

const AddressSchema = z.object({
  street: z.string(),
  city: z.string(),
  state: z.string(),
  zip_code: z.string(),
});

const MediumPayloadSchema = z.object({
  user_id: z.number(),
  username: z.string(),
  email: z.string(),
  is_active: z.boolean(),
  address: AddressSchema,
  tags: z.array(z.string()),
});

const ItemSchema = z.object({
  id: z.number(),
  name: z.string(),
  price: z.number(),
  in_stock: z.boolean(),
});

const LargePayloadSchema = z.object({
  order_id: z.string(),
  customer_name: z.string(),
  items: z.array(ItemSchema),
  total: z.number(),
  notes: z.string(),
});

const VeryLargePayloadSchema = z.object({
  data: z.array(z.record(z.any())),
  metadata: z.record(z.any()),
});

// ============================================================================
// JSON Body Workloads
// ============================================================================

async function post_json_small(requestJson) {
  const request = JSON.parse(requestJson);
  const validated = SmallPayloadSchema.parse(request.body);
  return JSON.stringify(validated);
}

async function post_json_medium(requestJson) {
  const request = JSON.parse(requestJson);
  const validated = MediumPayloadSchema.parse(request.body);
  return JSON.stringify(validated);
}

async function post_json_large(requestJson) {
  const request = JSON.parse(requestJson);
  const validated = LargePayloadSchema.parse(request.body);
  return JSON.stringify(validated);
}

async function post_json_very_large(requestJson) {
  const request = JSON.parse(requestJson);
  const validated = VeryLargePayloadSchema.parse(request.body);
  return JSON.stringify(validated);
}

// ============================================================================
// Multipart Form Workloads
// ============================================================================

async function post_multipart_small(requestJson) {
  return JSON.stringify({ files_received: 1, total_bytes: 1024 });
}

async function post_multipart_medium(requestJson) {
  return JSON.stringify({ files_received: 2, total_bytes: 10240 });
}

async function post_multipart_large(requestJson) {
  return JSON.stringify({ files_received: 5, total_bytes: 102400 });
}

// ============================================================================
// URL Encoded Form Workloads
// ============================================================================

async function post_urlencoded_simple(requestJson) {
  const request = JSON.parse(requestJson);
  return JSON.stringify(request.body || {});
}

async function post_urlencoded_complex(requestJson) {
  const request = JSON.parse(requestJson);
  return JSON.stringify(request.body || {});
}

// ============================================================================
// Path Parameter Workloads
// ============================================================================

async function get_path_simple(requestJson) {
  const request = JSON.parse(requestJson);
  return JSON.stringify({ id: request.path_params.id });
}

async function get_path_multiple(requestJson) {
  const request = JSON.parse(requestJson);
  return JSON.stringify({
    user_id: request.path_params.user_id,
    post_id: request.path_params.post_id
  });
}

async function get_path_deep(requestJson) {
  const request = JSON.parse(requestJson);
  return JSON.stringify({
    org: request.path_params.org,
    team: request.path_params.team,
    project: request.path_params.project,
    resource: request.path_params.resource,
    id: request.path_params.id
  });
}

async function get_path_int(requestJson) {
  const request = JSON.parse(requestJson);
  return JSON.stringify({ id: parseInt(request.path_params.id) });
}

async function get_path_uuid(requestJson) {
  const request = JSON.parse(requestJson);
  return JSON.stringify({ uuid: request.path_params.uuid });
}

async function get_path_date(requestJson) {
  const request = JSON.parse(requestJson);
  return JSON.stringify({ date: request.path_params.date });
}

// ============================================================================
// Query Parameter Workloads
// ============================================================================

async function get_query_few(requestJson) {
  const request = JSON.parse(requestJson);
  return JSON.stringify(request.query_params || {});
}

async function get_query_medium(requestJson) {
  const request = JSON.parse(requestJson);
  return JSON.stringify(request.query_params || {});
}

async function get_query_many(requestJson) {
  const request = JSON.parse(requestJson);
  return JSON.stringify(request.query_params || {});
}

// ============================================================================
// Health Check
// ============================================================================

async function get_health(requestJson) {
  return JSON.stringify({ status: 'ok' });
}

async function get_root(requestJson) {
  return JSON.stringify({ status: 'ok' });
}

// Register all routes
post('/json/small')(post_json_small);
post('/json/medium')(post_json_medium);
post('/json/large')(post_json_large);
post('/json/very-large')(post_json_very_large);

post('/multipart/small')(post_multipart_small);
post('/multipart/medium')(post_multipart_medium);
post('/multipart/large')(post_multipart_large);

post('/urlencoded/simple')(post_urlencoded_simple);
post('/urlencoded/complex')(post_urlencoded_complex);

get('/path/simple/{id}')(get_path_simple);
get('/path/multiple/{user_id}/{post_id}')(get_path_multiple);
get('/path/deep/{org}/{team}/{project}/{resource}/{id}')(get_path_deep);
get('/path/int/{id}')(get_path_int);
get('/path/uuid/{uuid}')(get_path_uuid);
get('/path/date/{date}')(get_path_date);

get('/query/few')(get_query_few);
get('/query/medium')(get_query_medium);
get('/query/many')(get_query_many);

get('/health')(get_health);
get('/')(get_root);

// Create app object
const app = {
  routes,
  handlers,
};

// Load and run the server
const path = require('path');
const nativePath = path.join(__dirname, '../../../../packages/node/spikard-node.darwin-arm64.node');
const native = require(nativePath);
const port = process.argv[2] ? parseInt(process.argv[2]) : 8000;

console.error(`[spikard-node] Starting server on port ${port}`);
native.runServer(app, { host: '0.0.0.0', port: port });
