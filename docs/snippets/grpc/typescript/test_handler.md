# TypeScript gRPC Handler Tests

Comprehensive test examples for gRPC handlers using Vitest.

```typescript
// user_handler.test.ts
import { describe, it, expect } from 'vitest';
import { GrpcRequest } from '@spikard/node';
import { UserServiceHandler } from './user_handler';
import { userservice } from './user_service';

describe('UserServiceHandler', () => {
  it('should get an existing user', async () => {
    const handler = new UserServiceHandler();

    // Create request
    const req = userservice.v1.GetUserRequest.create({ userId: 1 });
    const payload = userservice.v1.GetUserRequest.encode(req).finish();
    const grpcRequest = new GrpcRequest({
      serviceName: 'userservice.v1.UserService',
      methodName: 'GetUser',
      payload,
    });

    // Call handler
    const response = await handler.handleRequest(grpcRequest);

    // Deserialize response
    const userResponse = userservice.v1.UserResponse.decode(response.payload);

    // Assertions
    expect(userResponse.success).toBe(true);
    expect(userResponse.user?.id).toBe(1);
    expect(userResponse.user?.name).toBe('Alice');
  });

  it('should return error for non-existent user', async () => {
    const handler = new UserServiceHandler();

    const req = userservice.v1.GetUserRequest.create({ userId: 999 });
    const payload = userservice.v1.GetUserRequest.encode(req).finish();
    const grpcRequest = new GrpcRequest({
      serviceName: 'userservice.v1.UserService',
      methodName: 'GetUser',
      payload,
    });

    const response = await handler.handleRequest(grpcRequest);
    const userResponse = userservice.v1.UserResponse.decode(response.payload);

    expect(userResponse.success).toBe(false);
    expect(userResponse.errorMessage).toContain('not found');
  });

  it('should create a new user', async () => {
    const handler = new UserServiceHandler();

    const req = userservice.v1.CreateUserRequest.create({
      name: 'Charlie',
      email: 'charlie@example.com',
      tags: ['developer'],
    });
    const payload = userservice.v1.CreateUserRequest.encode(req).finish();
    const grpcRequest = new GrpcRequest({
      serviceName: 'userservice.v1.UserService',
      methodName: 'CreateUser',
      payload,
    });

    const response = await handler.handleRequest(grpcRequest);
    const userResponse = userservice.v1.UserResponse.decode(response.payload);

    expect(userResponse.success).toBe(true);
    expect(userResponse.user?.name).toBe('Charlie');
    expect(userResponse.user?.id).toBe(3);
  });

  it('should validate required fields on create', async () => {
    const handler = new UserServiceHandler();

    const req = userservice.v1.CreateUserRequest.create({
      name: 'Test User',
      email: '', // Missing email
    });
    const payload = userservice.v1.CreateUserRequest.encode(req).finish();
    const grpcRequest = new GrpcRequest({
      serviceName: 'userservice.v1.UserService',
      methodName: 'CreateUser',
      payload,
    });

    const response = await handler.handleRequest(grpcRequest);
    const userResponse = userservice.v1.UserResponse.decode(response.payload);

    expect(userResponse.success).toBe(false);
    expect(userResponse.errorMessage).toContain('required');
  });

  it('should throw for unknown methods', async () => {
    const handler = new UserServiceHandler();

    const grpcRequest = new GrpcRequest({
      serviceName: 'userservice.v1.UserService',
      methodName: 'DeleteUser', // Not implemented
      payload: new Uint8Array(),
    });

    await expect(handler.handleRequest(grpcRequest)).rejects.toThrow(
      'Unknown method'
    );
  });
});
```

## Test Patterns

### Using Test Helpers

```typescript
// test-helpers.ts
import { GrpcRequest } from '@spikard/node';
import { userservice } from './user_service';

export function createGetUserRequest(userId: number): GrpcRequest {
  const req = userservice.v1.GetUserRequest.create({ userId });
  const payload = userservice.v1.GetUserRequest.encode(req).finish();
  return new GrpcRequest({
    serviceName: 'userservice.v1.UserService',
    methodName: 'GetUser',
    payload,
  });
}

export function createCreateUserRequest(
  name: string,
  email: string,
  tags?: string[]
): GrpcRequest {
  const req = userservice.v1.CreateUserRequest.create({
    name,
    email,
    tags: tags || [],
  });
  const payload = userservice.v1.CreateUserRequest.encode(req).finish();
  return new GrpcRequest({
    serviceName: 'userservice.v1.UserService',
    methodName: 'CreateUser',
    payload,
  });
}
```

### Testing with Metadata

```typescript
it('should handle authorization header', async () => {
  const handler = new UserServiceHandler();

  const req = userservice.v1.CreateUserRequest.create({
    name: 'Test',
    email: 'test@example.com',
  });
  const payload = userservice.v1.CreateUserRequest.encode(req).finish();
  const grpcRequest = new GrpcRequest({
    serviceName: 'userservice.v1.UserService',
    methodName: 'CreateUser',
    payload,
    metadata: {
      authorization: 'Bearer valid-token',
    },
  });

  const response = await handler.handleRequest(grpcRequest);
  const userResponse = userservice.v1.UserResponse.decode(response.payload);

  expect(userResponse.success).toBe(true);
});
```

## Running Tests

```bash
# Run all tests
npx vitest

# Run with coverage
npx vitest --coverage

# Run specific file
npx vitest user_handler.test.ts
```
