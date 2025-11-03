# @spikard/node

High-performance HTTP framework for Node.js/TypeScript. Type-safe routing, validation, and testing powered by Rust core.

## Installation

```bash
npm install @spikard/node
# or
pnpm add @spikard/node
# or
yarn add @spikard/node
```

## Features

- 🚀 **High Performance** - Rust-powered HTTP server with native bindings
- 🔒 **Type-Safe** - Full TypeScript support with type inference
- ✅ **Built-in Validation** - JSON Schema validation at the framework level
- 🧪 **Easy Testing** - TestClient for testing without starting a server
- 🎯 **FastAPI-style** - Path parameter type hints (`/users/{id:uuid}`)

## Quick Start

```typescript
import { TestClient } from '@spikard/node';

// Define your application
const app = {
  routes: [
    {
      method: 'GET',
      path: '/users/{id:uuid}',
      handler_name: 'getUser',
      is_async: true
    }
  ],
  handlers: {
    getUser: async (req) => {
      return { id: req.params.id, name: 'Alice' };
    }
  }
};

// Test it
const client = new TestClient(app);
const response = await client.get('/users/550e8400-e29b-41d4-a716-446655440000');
console.log(response.json()); // { id: '550e8400-e29b-41d4-a716-446655440000', name: 'Alice' }
```

## Testing

```typescript
import { describe, it, expect } from 'vitest';
import { TestClient } from '@spikard/node';

describe('User API', () => {
  const client = new TestClient(app);

  it('should get user by ID', async () => {
    const response = await client.get('/users/123');
    expect(response.statusCode).toBe(200);
    expect(response.json()).toEqual({ id: '123', name: 'Alice' });
  });

  it('should create user', async () => {
    const response = await client.post('/users', {
      json: { name: 'Bob', email: 'bob@example.com' }
    });
    expect(response.statusCode).toBe(201);
  });
});
```

## License

MIT
