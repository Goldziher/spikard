#!/usr/bin/env node
/**
 * Express comparison server for benchmarking (TypeScript)
 *
 * Implements all 18 workload endpoints with Zod validation.
 * Uses Express middleware pattern for request validation.
 */

import express, { type Request, type Response, type NextFunction } from 'express';
import { z, type ZodSchema } from 'zod';

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
  data: z.array(z.record(z.unknown())),
  metadata: z.record(z.unknown()),
});

// ============================================================================
// Type Definitions
// ============================================================================

type SmallPayload = z.infer<typeof SmallPayloadSchema>;
type MediumPayload = z.infer<typeof MediumPayloadSchema>;
type LargePayload = z.infer<typeof LargePayloadSchema>;
type VeryLargePayload = z.infer<typeof VeryLargePayloadSchema>;

// ============================================================================
// Helper: Zod validation middleware
// ============================================================================

function validateBody<T>(schema: ZodSchema<T>) {
  return (req: Request, res: Response, next: NextFunction): void => {
    try {
      req.body = schema.parse(req.body);
      next();
    } catch (error) {
      if (error instanceof z.ZodError) {
        res.status(400).json({
          error: 'Validation failed',
          details: error.errors,
        });
      } else {
        res.status(400).json({
          error: 'Validation failed',
          details: String(error),
        });
      }
    }
  };
}

// ============================================================================
// JSON Body Workloads
// ============================================================================

app.post('/json/small', validateBody(SmallPayloadSchema), (req: Request, res: Response): void => {
  res.json(req.body as SmallPayload);
});

app.post('/json/medium', validateBody(MediumPayloadSchema), (req: Request, res: Response): void => {
  res.json(req.body as MediumPayload);
});

app.post('/json/large', validateBody(LargePayloadSchema), (req: Request, res: Response): void => {
  res.json(req.body as LargePayload);
});

app.post('/json/very-large', validateBody(VeryLargePayloadSchema), (req: Request, res: Response): void => {
  res.json(req.body as VeryLargePayload);
});

// ============================================================================
// Multipart Form Workloads
// ============================================================================

app.post('/multipart/small', (_req: Request, res: Response): void => {
  res.json({ files_received: 1, total_bytes: 1024 });
});

app.post('/multipart/medium', (_req: Request, res: Response): void => {
  res.json({ files_received: 2, total_bytes: 10240 });
});

app.post('/multipart/large', (_req: Request, res: Response): void => {
  res.json({ files_received: 5, total_bytes: 102400 });
});

// ============================================================================
// URL Encoded Form Workloads
// ============================================================================

app.post('/urlencoded/simple', (req: Request, res: Response): void => {
  res.json(req.body ?? {});
});

app.post('/urlencoded/complex', (req: Request, res: Response): void => {
  res.json(req.body ?? {});
});

// ============================================================================
// Path Parameter Workloads
// ============================================================================

app.get('/path/simple/:id', (req: Request, res: Response): void => {
  res.json({ id: req.params.id });
});

app.get('/path/multiple/:user_id/:post_id', (req: Request, res: Response): void => {
  res.json({
    user_id: req.params.user_id,
    post_id: req.params.post_id,
  });
});

app.get('/path/deep/:org/:team/:project/:resource/:id', (req: Request, res: Response): void => {
  res.json({
    org: req.params.org,
    team: req.params.team,
    project: req.params.project,
    resource: req.params.resource,
    id: req.params.id,
  });
});

app.get('/path/int/:id', (req: Request, res: Response): void => {
  res.json({ id: Number.parseInt(req.params.id ?? '0', 10) });
});

app.get('/path/uuid/:uuid', (req: Request, res: Response): void => {
  res.json({ uuid: req.params.uuid });
});

app.get('/path/date/:date', (req: Request, res: Response): void => {
  res.json({ date: req.params.date });
});

// ============================================================================
// Query Parameter Workloads
// ============================================================================

app.get('/query/few', (req: Request, res: Response): void => {
  res.json(req.query ?? {});
});

app.get('/query/medium', (req: Request, res: Response): void => {
  res.json(req.query ?? {});
});

app.get('/query/many', (req: Request, res: Response): void => {
  res.json(req.query ?? {});
});

// ============================================================================
// Health Check
// ============================================================================

app.get('/health', (_req: Request, res: Response): void => {
  res.json({ status: 'ok' });
});

app.get('/', (_req: Request, res: Response): void => {
  res.json({ status: 'ok' });
});

// ============================================================================
// Server Startup
// ============================================================================

const port = process.argv[2] ? Number.parseInt(process.argv[2], 10) : 8000;

app.listen(port, '0.0.0.0', (): void => {
  console.error(`[express] Starting server on port ${port}`);
});
