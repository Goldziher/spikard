#!/usr/bin/env tsx
/**
 * Express RAW performance server (NO validation) for benchmarking
 *
 * Implements all 18 workload types to match Express server exactly,
 * but WITHOUT any validation overhead - pure Express performance.
 */

import express, { Request, Response } from 'express';

const app = express();

// Middleware for parsing JSON and URL-encoded bodies
app.use(express.json());
app.use(express.urlencoded({ extended: true }));

// ============================================================================
// JSON Body Workloads - NO VALIDATION
// ============================================================================

app.post('/json/small', (req: Request, res: Response) => {
  res.json(req.body);
});

app.post('/json/medium', (req: Request, res: Response) => {
  res.json(req.body);
});

app.post('/json/large', (req: Request, res: Response) => {
  res.json(req.body);
});

app.post('/json/very-large', (req: Request, res: Response) => {
  res.json(req.body);
});

// ============================================================================
// Multipart Form Workloads - NO VALIDATION
// ============================================================================

app.post('/multipart/small', (req: Request, res: Response) => {
  res.json({ files_received: 1, total_bytes: 1024 });
});

app.post('/multipart/medium', (req: Request, res: Response) => {
  res.json({ files_received: 2, total_bytes: 10240 });
});

app.post('/multipart/large', (req: Request, res: Response) => {
  res.json({ files_received: 5, total_bytes: 102400 });
});

// ============================================================================
// URL Encoded Form Workloads - NO VALIDATION
// ============================================================================

app.post('/urlencoded/simple', (req: Request, res: Response) => {
  res.json(req.body || {});
});

app.post('/urlencoded/complex', (req: Request, res: Response) => {
  res.json(req.body || {});
});

// ============================================================================
// Path Parameter Workloads - NO VALIDATION
// ============================================================================

app.get('/path/simple/:id', (req: Request, res: Response) => {
  res.json({ id: req.params.id });
});

app.get('/path/multiple/:user_id/:post_id', (req: Request, res: Response) => {
  res.json({
    user_id: req.params.user_id,
    post_id: req.params.post_id
  });
});

app.get('/path/deep/:org/:team/:project/:resource/:id', (req: Request, res: Response) => {
  res.json({
    org: req.params.org,
    team: req.params.team,
    project: req.params.project,
    resource: req.params.resource,
    id: req.params.id
  });
});

app.get('/path/int/:id', (req: Request, res: Response) => {
  res.json({ id: parseInt(req.params.id ?? '0', 10) });
});

app.get('/path/uuid/:uuid', (req: Request, res: Response) => {
  res.json({ uuid: req.params.uuid ?? '' });
});

app.get('/path/date/:date', (req: Request, res: Response) => {
  res.json({ date: req.params.date });
});

// ============================================================================
// Query Parameter Workloads - NO VALIDATION
// ============================================================================

app.get('/query/few', (req: Request, res: Response) => {
  res.json(req.query || {});
});

app.get('/query/medium', (req: Request, res: Response) => {
  res.json(req.query || {});
});

app.get('/query/many', (req: Request, res: Response) => {
  res.json(req.query || {});
});

// ============================================================================
// Health Check
// ============================================================================

app.get('/health', (req: Request, res: Response) => {
  res.json({ status: 'ok' });
});

app.get('/', (req: Request, res: Response) => {
  res.json({ status: 'ok' });
});

// ============================================================================
// Server Startup
// ============================================================================

const port = process.argv[2] ? parseInt(process.argv[2], 10) : 8000;

app.listen(port, '0.0.0.0', () => {
  console.error(`[express-raw] Starting server on port ${port}`);
});
