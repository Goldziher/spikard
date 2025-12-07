#!/usr/bin/env node
/**
 * Fastify RAW comparison server for benchmarking
 *
 * NO VALIDATION - accepts any JSON body and echoes it back.
 * This measures Fastify's raw performance without validation overhead.
 */

const fastify = require('fastify')({ logger: false });

// Register form body parser for URL-encoded forms
fastify.register(require('@fastify/formbody'));

// ============================================================================
// JSON Body Workloads - NO VALIDATION
// ============================================================================

fastify.post('/json/small', async (request, reply) => {
  return request.body;
});

fastify.post('/json/medium', async (request, reply) => {
  return request.body;
});

fastify.post('/json/large', async (request, reply) => {
  return request.body;
});

fastify.post('/json/very-large', async (request, reply) => {
  return request.body;
});

// ============================================================================
// Multipart Form Workloads - NO VALIDATION
// ============================================================================

fastify.post('/multipart/small', async (request, reply) => {
  return { files_received: 1, total_bytes: 1024 };
});

fastify.post('/multipart/medium', async (request, reply) => {
  return { files_received: 2, total_bytes: 10240 };
});

fastify.post('/multipart/large', async (request, reply) => {
  return { files_received: 5, total_bytes: 102400 };
});

// ============================================================================
// URL Encoded Form Workloads - NO VALIDATION
// ============================================================================

fastify.post('/urlencoded/simple', async (request, reply) => {
  return request.body || {};
});

fastify.post('/urlencoded/complex', async (request, reply) => {
  return request.body || {};
});

// ============================================================================
// Path Parameter Workloads - NO VALIDATION
// ============================================================================

fastify.get('/path/simple/:id', async (request, reply) => {
  const { id } = request.params;
  return { id };
});

fastify.get('/path/multiple/:user_id/:post_id', async (request, reply) => {
  const { user_id, post_id } = request.params;
  return { user_id, post_id };
});

fastify.get('/path/deep/:org/:team/:project/:resource/:id', async (request, reply) => {
  const { org, team, project, resource, id } = request.params;
  return { org, team, project, resource, id };
});

fastify.get('/path/int/:id', async (request, reply) => {
  const { id } = request.params;
  return { id: parseInt(id, 10) };
});

fastify.get('/path/uuid/:uuid', async (request, reply) => {
  const { uuid } = request.params;
  return { uuid };
});

fastify.get('/path/date/:date', async (request, reply) => {
  const { date } = request.params;
  return { date };
});

// ============================================================================
// Query Parameter Workloads - NO VALIDATION
// ============================================================================

fastify.get('/query/few', async (request, reply) => {
  return request.query || {};
});

fastify.get('/query/medium', async (request, reply) => {
  return request.query || {};
});

fastify.get('/query/many', async (request, reply) => {
  return request.query || {};
});

// ============================================================================
// Health Check
// ============================================================================

fastify.get('/health', async (request, reply) => {
  return { status: 'ok' };
});

fastify.get('/', async (request, reply) => {
  return { status: 'ok' };
});

// ============================================================================
// Server Startup
// ============================================================================

const port = process.argv[2]
  ? parseInt(process.argv[2], 10)
  : process.env.PORT
  ? parseInt(process.env.PORT, 10)
  : 8000;
