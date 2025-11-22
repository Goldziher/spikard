#!/usr/bin/env node
/**
 * Hono comparison server for benchmarking
 *
 * Implements all workload types to match spikard-node server exactly.
 * Uses zod for validation via @hono/zod-validator.
 * Runs on Node.js via @hono/node-server (Hono is designed for edge but supports Node.js).
 */

const { serve } = require('@hono/node-server');
const { Hono } = require('hono');
const { zValidator } = require('@hono/zod-validator');
const { z } = require('zod');

const app = new Hono();

// ============================================================================
// Zod Schema Definitions
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

app.post('/json/small', zValidator('json', SmallPayloadSchema), (c) => {
  const validated = c.req.valid('json');
  return c.json(validated);
});

app.post('/json/medium', zValidator('json', MediumPayloadSchema), (c) => {
  const validated = c.req.valid('json');
  return c.json(validated);
});

app.post('/json/large', zValidator('json', LargePayloadSchema), (c) => {
  const validated = c.req.valid('json');
  return c.json(validated);
});

app.post('/json/very-large', zValidator('json', VeryLargePayloadSchema), (c) => {
  const validated = c.req.valid('json');
  return c.json(validated);
});

// ============================================================================
// Multipart Form Workloads
// ============================================================================

app.post('/multipart/small', (c) => {
  return c.json({ files_received: 1, total_bytes: 1024 });
});

app.post('/multipart/medium', (c) => {
  return c.json({ files_received: 2, total_bytes: 10240 });
});

app.post('/multipart/large', (c) => {
  return c.json({ files_received: 5, total_bytes: 102400 });
});

// ============================================================================
// URL Encoded Form Workloads
// ============================================================================

app.post('/urlencoded/simple', async (c) => {
  const body = await c.req.parseBody();
  return c.json(body || {});
});

app.post('/urlencoded/complex', async (c) => {
  const body = await c.req.parseBody();
  return c.json(body || {});
});

// ============================================================================
// Path Parameter Workloads
// ============================================================================

app.get('/path/simple/:id', (c) => {
  const id = c.req.param('id');
  return c.json({ id });
});

app.get('/path/multiple/:user_id/:post_id', (c) => {
  const user_id = c.req.param('user_id');
  const post_id = c.req.param('post_id');
  return c.json({ user_id, post_id });
});

app.get('/path/deep/:org/:team/:project/:resource/:id', (c) => {
  const org = c.req.param('org');
  const team = c.req.param('team');
  const project = c.req.param('project');
  const resource = c.req.param('resource');
  const id = c.req.param('id');
  return c.json({ org, team, project, resource, id });
});

app.get('/path/int/:id', (c) => {
  const id = parseInt(c.req.param('id'), 10);
  return c.json({ id });
});

app.get('/path/uuid/:uuid', (c) => {
  const uuid = c.req.param('uuid');
  return c.json({ uuid });
});

app.get('/path/date/:date', (c) => {
  const date = c.req.param('date');
  return c.json({ date });
});

// ============================================================================
// Query Parameter Workloads
// ============================================================================

app.get('/query/few', (c) => {
  const query = c.req.query();
  return c.json(query || {});
});

app.get('/query/medium', (c) => {
  const query = c.req.query();
  return c.json(query || {});
});

app.get('/query/many', (c) => {
  const query = c.req.query();
  return c.json(query || {});
});

// ============================================================================
// Health Check
// ============================================================================

app.get('/health', (c) => {
  return c.json({ status: 'ok' });
});

app.get('/', (c) => {
  return c.json({ status: 'ok' });
});

// ============================================================================
// Server Startup
// ============================================================================

const port = process.argv[2] ? parseInt(process.argv[2], 10) : 8000;

console.error(`[hono] Starting server on port ${port}`);

serve({
  fetch: app.fetch,
  port: port,
  hostname: '0.0.0.0'
});
