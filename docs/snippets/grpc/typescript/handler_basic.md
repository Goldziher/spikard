# TypeScript gRPC Handler

Complete TypeScript handler implementation for UserService with GetUser and CreateUser methods.

```typescript
import {
  GrpcHandler,
  GrpcRequest,
  GrpcResponse,
  GrpcError,
  GrpcStatusCode,
  createServiceHandler,
  createUnaryHandler,
} from 'spikard';
import * as userservice from './userservice_pb';  // Generated protobufjs types

class UserServiceHandler implements GrpcHandler {
  constructor(private userRepository: UserRepository) {}

  async handleRequest(request: GrpcRequest): Promise<GrpcResponse> {
    /**
     * Handle incoming gRPC requests.
     * Routes to appropriate method based on request.methodName.
     */
    switch (request.methodName) {
      case 'GetUser':
        return this.getUser(request);
      case 'CreateUser':
        return this.createUser(request);
      default:
        throw new GrpcError(
          GrpcStatusCode.UNIMPLEMENTED,
          `Method ${request.methodName} not implemented`
        );
    }
  }

  private async getUser(request: GrpcRequest): Promise<GrpcResponse> {
    // 1. Deserialize request
    const req = userservice.GetUserRequest.decode(request.payload);

    // 2. Validate input
    if (req.id <= 0) {
      throw new GrpcError(
        GrpcStatusCode.INVALID_ARGUMENT,
        'User ID must be positive'
      );
    }

    // 3. Business logic
    const user = await this.userRepository.findById(req.id);
    if (!user) {
      throw new GrpcError(
        GrpcStatusCode.NOT_FOUND,
        `User ${req.id} not found`
      );
    }

    // 4. Build response
    const responseUser = userservice.User.create({
      id: user.id,
      name: user.name,
      email: user.email,
      createdAt: user.createdAt.toISOString(),
    });

    // 5. Serialize and return
    const encoded = userservice.User.encode(responseUser).finish();
    return {
      payload: Buffer.from(encoded),
      metadata: { 'x-user-found': 'true' },
    };
  }

  private async createUser(request: GrpcRequest): Promise<GrpcResponse> {
    // 1. Deserialize request
    const req = userservice.CreateUserRequest.decode(request.payload);

    // 2. Validate input
    if (!req.name || !req.email) {
      throw new GrpcError(
        GrpcStatusCode.INVALID_ARGUMENT,
        'Name and email are required'
      );
    }

    // 3. Check authorization from metadata
    const authToken = request.metadata['authorization'];
    if (!authToken) {
      throw new GrpcError(
        GrpcStatusCode.UNAUTHENTICATED,
        'Authentication required'
      );
    }

    // 4. Business logic
    const user = await this.userRepository.create({
      name: req.name,
      email: req.email,
    });

    // 5. Build response
    const responseUser = userservice.User.create({
      id: user.id,
      name: user.name,
      email: user.email,
      createdAt: new Date().toISOString(),
    });

    // 6. Serialize with metadata
    const encoded = userservice.User.encode(responseUser).finish();
    return {
      payload: Buffer.from(encoded),
      metadata: {
        'x-user-id': user.id.toString(),
        'x-created': 'true',
      },
    };
  }
}
```

## Key Patterns

- **Protobufjs**: Uses `.decode()` and `.encode().finish()` for serialization
- **Buffer**: gRPC payloads are Node.js `Buffer` objects
- **GrpcError**: Throw with explicit status codes for proper error responses
- **Helper functions**: `createUnaryHandler` and `createServiceHandler` reduce boilerplate
- **Type safety**: Full TypeScript type inference for protobuf messages

## Registration

```typescript
import { createApp } from 'spikard';

const app = createApp();

// Register gRPC handler
app.registerGrpcHandler('userservice.UserService', userServiceHandler);

// Server ready
await app.listen(50051);
```
