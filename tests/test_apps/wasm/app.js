import { Spikard, get, post, TestClient } from '@spikard/wasm';

/**
 * WASM test application for Spikard
 *
 * Tests core functionality:
 * - Health check endpoint
 * - Query parameter handling
 * - JSON request/response
 * - Path parameter extraction
 *
 * Uses TestClient for in-memory testing without actual HTTP server.
 */

export function createApp() {
  const app = new Spikard();

  // Health check endpoint
  get('/health', async (_req) => {
    return { status: 'ok' };
  });

  // Query parameters endpoint
  get('/query', async (req) => {
    return {
      name: req.query?.name ?? null,
      age: req.query?.age ? parseInt(String(req.query.age)) : null,
    };
  });

  // JSON echo endpoint
  post('/echo', async (req) => {
    const body = req.json();
    return {
      received: body,
      method: req.method,
    };
  });

  // Path parameters endpoint
  get('/users/{id}', async (req) => {
    const userId = req.pathParams?.id;
    return {
      userId,
      type: typeof userId,
    };
  });

  // Return app and test client
  return {
    app,
    async start() {
      // TestClient doesn't require start
    },
    async stop() {
      // TestClient doesn't require stop
    },
    address() {
      return { host: '127.0.0.1', port: 8000 };
    },
    async getTestClient() {
      return new TestClient(app);
    },
  };
}
