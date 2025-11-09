/**
 * Simple Spikard Node example server
 */

// For now, use a simple compatible API since the native module isn't built yet
const routes = [];
const handlers = {};

function get(path) {
  return function(handler) {
    const metadata = {
      method: 'GET',
      path,
      handler_name: handler.name,
      is_async: true,
    };
    routes.push(metadata);
    handlers[handler.name] = handler;
    return handler;
  };
}

function post(path) {
  return function(handler) {
    const metadata = {
      method: 'POST',
      path,
      handler_name: handler.name,
      is_async: true,
    };
    routes.push(metadata);
    handlers[handler.name] = handler;
    return handler;
  };
}

// Handler functions that receive JSON string and return Promise<string>
// This matches the ThreadsafeFunction signature expected by Rust
async function root(requestJson) {
  const request = JSON.parse(requestJson);
  const response = { message: 'Hello from Spikard Node!', request };
  return JSON.stringify(response);
}

async function health(requestJson) {
  const request = JSON.parse(requestJson);
  const response = { status: 'healthy', request };
  return JSON.stringify(response);
}

async function getUserById(requestJson) {
  const request = JSON.parse(requestJson);
  const userId = request.path_params.id || 'unknown';
  const response = { user_id: userId, name: 'Test User', request };
  return JSON.stringify(response);
}

async function echo(requestJson) {
  const request = JSON.parse(requestJson);
  const response = { echoed: true, body: request.body, request };
  return JSON.stringify(response);
}

// Define routes
get('/'  )(root);
get('/health')(health);
get('/users/{id}')(getUserById);
post('/echo')(echo);

// Create app object
const app = {
  routes,
  handlers,
};

// Try to load and run the server
try {
  // Load the native module directly
  const native = require('../../packages/node/spikard-node.darwin-arm64.node');
  console.log('[spikard-node] Native module loaded');
  console.log('[spikard-node] Starting server...');
  native.runServer(app, '0.0.0.0', 8000);
} catch (e) {
  console.error('[spikard-node] Failed to start server:', e.message);
  console.error(e.stack);
  console.log('\nTo build the native module, run:');
  console.log('  cd packages/node && pnpm build:native\n');
  process.exit(1);
}
