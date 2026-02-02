### Class-Based Handler

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

### Helper Function Pattern (Alternative)

For simpler handlers, use the factory helper pattern:

```typescript
// Create handler using helper function
const userServiceHandler = createServiceHandler({
  GetUser: createUnaryHandler<GetUserRequest, User>(
    'GetUser',
    async (req, metadata) => {
      // Validate
      if (req.id <= 0) {
        throw new GrpcError(GrpcStatusCode.INVALID_ARGUMENT, 'Invalid ID');
      }

      // Business logic
      const user = await userRepository.findById(req.id);
      if (!user) {
        throw new GrpcError(GrpcStatusCode.NOT_FOUND, 'User not found');
      }

      // Return with metadata
      return {
        response: user,
        metadata: { 'x-user-found': 'true' },
      };
    },
    userservice.GetUserRequest,
    userservice.User
  ),

  CreateUser: createUnaryHandler<CreateUserRequest, User>(
    'CreateUser',
    async (req, metadata) => {
      // Validate
      if (!req.name || !req.email) {
        throw new GrpcError(GrpcStatusCode.INVALID_ARGUMENT, 'Missing fields');
      }

      // Check auth
      if (!metadata['authorization']) {
        throw new GrpcError(GrpcStatusCode.UNAUTHENTICATED, 'Auth required');
      }

      // Business logic
      const user = await userRepository.create(req);

      return {
        response: user,
        metadata: { 'x-created': 'true' },
      };
    },
    userservice.CreateUserRequest,
    userservice.User
  ),
});
```

**When to use each:**
- **Class-based**: Complex services with shared state, dependency injection, multiple methods
- **Helper functions**: Simple services, functional style, less boilerplate
