import init, { Server } from '@spikard/wasm';

/**
 * WASM test application for Spikard
 *
 * Tests core functionality:
 * - Health check endpoint
 * - Query parameter handling
 * - JSON request/response
 * - Path parameter extraction
 */

export async function createApp() {
  // Initialize WASM module
  await init();

  const server = new Server({
    host: '127.0.0.1',
    port: 0, // Random port for testing
  });

  // Health check
  server.get('/health', async (_req) => {
    return {
      status: 200,
      headers: { 'content-type': 'application/json' },
      body: JSON.stringify({ status: 'ok' }),
    };
  });

  // Query parameters
  server.get('/query', async (req) => {
    const params = req.queryParams || {};
    return {
      status: 200,
      headers: { 'content-type': 'application/json' },
      body: JSON.stringify({
        name: params.name || null,
        age: params.age ? parseInt(params.age) : null,
      }),
    };
  });

  // JSON echo
  server.post('/echo', async (req) => {
    const body = req.body ? JSON.parse(req.body) : {};
    return {
      status: 200,
      headers: { 'content-type': 'application/json' },
      body: JSON.stringify({
        received: body,
        method: req.method,
      }),
    };
  });

  // Path parameters
  server.get('/users/:id', async (req) => {
    const userId = req.pathParams?.id;
    return {
      status: 200,
      headers: { 'content-type': 'application/json' },
      body: JSON.stringify({
        userId,
        type: typeof userId,
      }),
    };
  });

  return server;
}
