import { init, TestClient } from '@spikard/wasm';

/**
 * WASM test application for Spikard
 *
 * Tests core functionality:
 * - Health check endpoint
 * - Query parameter handling
 * - JSON request/response
 * - Path parameter extraction
 *
 * Note: The WASM TestClient is an in-memory test client for unit testing.
 * It doesn't support HTTP server features (start/stop), but provides
 * request routing for testing handler functions.
 */

const routes = {};
let testClient = null;

// Register route handlers
function registerRoute(method, path, handler) {
  const key = `${method} ${path}`;
  routes[key] = handler;
}

// Register health check
registerRoute('GET', '/health', async (_req) => {
  return {
    status: 200,
    headers: { 'content-type': 'application/json' },
    body: JSON.stringify({ status: 'ok' }),
  };
});

// Register query parameters route
registerRoute('GET', '/query', async (req) => {
  const params = req.queryParams || {};
  return {
    status: 200,
    headers: { 'content-type': 'application/json' },
    body: JSON.stringify({
      name: params.name ? (Array.isArray(params.name) ? params.name[0] : params.name) : null,
      age: params.age ? parseInt(Array.isArray(params.age) ? params.age[0] : params.age) : null,
    }),
  };
});

// Register JSON echo route
registerRoute('POST', '/echo', async (req) => {
  const body = req.body ? (typeof req.body === 'string' ? JSON.parse(req.body) : req.body) : {};
  return {
    status: 200,
    headers: { 'content-type': 'application/json' },
    body: JSON.stringify({
      received: body,
      method: req.method,
    }),
  };
});

// Register path parameters route
registerRoute('GET', '/users/:id', async (req) => {
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

export async function createApp() {
  // Initialize WASM module
  await init();

  // Create a test client wrapper
  return {
    routes,
    start: async () => {
      // WASM TestClient doesn't support actual server start
      // In real use, you'd use the Spikard API instead
    },
    stop: async () => {
      // WASM TestClient doesn't support server stop
    },
    address: () => {
      return { host: '127.0.0.1', port: 8000 };
    },
  };
}
