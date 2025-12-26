import { Spikard, type Request } from '@spikard/node';

/**
 * Node.js test application for published @spikard/node package.
 *
 * Tests the PUBLISHED package from npm (0.6.0).
 */

export function createApp() {
  const app = new Spikard();

  // Health check endpoint
  app.addRoute(
    {
      method: 'GET',
      path: '/health',
      handler_name: 'health',
      is_async: true,
    },
    async (_req: Request) => {
      return { status: 'ok' };
    },
  );

  // Query parameters endpoint
  app.addRoute(
    {
      method: 'GET',
      path: '/query',
      handler_name: 'query',
      is_async: true,
    },
    async (req: Request) => {
      const name = req.query['name'] ?? '';
      const ageStr = req.query['age'] ?? '0';
      const age = parseInt(ageStr as string, 10);
      return { name, age };
    },
  );

  // JSON echo endpoint
  app.addRoute(
    {
      method: 'POST',
      path: '/echo',
      handler_name: 'echo',
      is_async: true,
    },
    async (req: Request) => {
      const body = req.json();
      return {
        received: body,
        method: 'POST',
      };
    },
  );

  // Path parameters endpoint
  app.addRoute(
    {
      method: 'GET',
      path: '/users/:id',
      handler_name: 'user',
      is_async: true,
    },
    async (req: Request) => {
      const userId = req.params['id'] ?? '';
      return {
        userId,
        type: 'string',
      };
    },
  );

  return app;
}

// Run if executed directly
if (require.main === module) {
  const port = process.argv[2] ? parseInt(process.argv[2], 10) : 8000;
  const app = createApp();
  console.log(`Starting Spikard Node.js test server on http://127.0.0.1:${port}`);
  app.run({ host: '127.0.0.1', port });
}
