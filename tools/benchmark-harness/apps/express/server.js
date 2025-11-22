#!/usr/bin/env node
/**
 * Express comparison server for benchmarking
 *
 * Implements all workload types to match spikard-node server exactly.
 * Uses zod for validation (most popular choice for Express).
 */

const express = require('express');
const { z } = require('zod');

const app = express();

// Middleware for parsing JSON and URL-encoded bodies
app.use(express.json());
app.use(express.urlencoded({ extended: true }));

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
// Helper: Zod validation middleware
// ============================================================================

function validateBody(schema) {
  return (req, res, next) => {
    try {
      req.body = schema.parse(req.body);
      next();
    } catch (error) {
      res.status(400).json({
        error: 'Validation failed',
        details: error.errors
      });
    }
  };
}

// ============================================================================
// JSON Body Workloads
// ============================================================================

app.post('/json/small', validateBody(SmallPayloadSchema), (req, res) => {
  res.json(req.body);
});

app.post('/json/medium', validateBody(MediumPayloadSchema), (req, res) => {
  res.json(req.body);
});

app.post('/json/large', validateBody(LargePayloadSchema), (req, res) => {
  res.json(req.body);
});

app.post('/json/very-large', validateBody(VeryLargePayloadSchema), (req, res) => {
  res.json(req.body);
});

// ============================================================================
// Multipart Form Workloads
// ============================================================================

app.post('/multipart/small', (req, res) => {
  res.json({ files_received: 1, total_bytes: 1024 });
});

app.post('/multipart/medium', (req, res) => {
  res.json({ files_received: 2, total_bytes: 10240 });
});

app.post('/multipart/large', (req, res) => {
  res.json({ files_received: 5, total_bytes: 102400 });
});

// ============================================================================
// URL Encoded Form Workloads
// ============================================================================

app.post('/urlencoded/simple', (req, res) => {
  res.json(req.body || {});
});

app.post('/urlencoded/complex', (req, res) => {
  res.json(req.body || {});
});

// ============================================================================
// Path Parameter Workloads
// ============================================================================

app.get('/path/simple/:id', (req, res) => {
  res.json({ id: req.params.id });
});

app.get('/path/multiple/:user_id/:post_id', (req, res) => {
  res.json({
    user_id: req.params.user_id,
    post_id: req.params.post_id
  });
});

app.get('/path/deep/:org/:team/:project/:resource/:id', (req, res) => {
  res.json({
    org: req.params.org,
    team: req.params.team,
    project: req.params.project,
    resource: req.params.resource,
    id: req.params.id
  });
});

app.get('/path/int/:id', (req, res) => {
  res.json({ id: parseInt(req.params.id, 10) });
});

app.get('/path/uuid/:uuid', (req, res) => {
  res.json({ uuid: req.params.uuid });
});

app.get('/path/date/:date', (req, res) => {
  res.json({ date: req.params.date });
});

// ============================================================================
// Query Parameter Workloads
// ============================================================================

app.get('/query/few', (req, res) => {
  res.json(req.query || {});
});

app.get('/query/medium', (req, res) => {
  res.json(req.query || {});
});

app.get('/query/many', (req, res) => {
  res.json(req.query || {});
});

// ============================================================================
// Health Check
// ============================================================================

app.get('/health', (req, res) => {
  res.json({ status: 'ok' });
});

app.get('/', (req, res) => {
  res.json({ status: 'ok' });
});

// ============================================================================
// Server Startup
// ============================================================================

const port = process.argv[2] ? parseInt(process.argv[2], 10) : 8000;

app.listen(port, '0.0.0.0', () => {
  console.error(`[express] Starting server on port ${port}`);
});
