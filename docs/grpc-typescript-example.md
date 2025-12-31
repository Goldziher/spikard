# gRPC TypeScript Example for Spikard

This guide demonstrates how to implement gRPC service handlers in TypeScript using Spikard's gRPC FFI bindings.

## Overview

Spikard provides a TypeScript FFI layer for gRPC that allows you to:
- Implement gRPC service handlers in TypeScript/Node.js
- Use protobufjs for message serialization/deserialization
- Integrate seamlessly with Spikard's Rust-based gRPC runtime
- Handle both unary and streaming RPCs (streaming support coming soon)

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      TypeScript Handler                      │
│  - Implements GrpcHandler interface                         │
│  - Uses protobufjs for serialization                        │
└──────────────────────┬──────────────────────────────────────┘
                       │ napi-rs ThreadsafeFunction
┌──────────────────────▼──────────────────────────────────────┐
│                  NodeGrpcHandler (Rust)                      │
│  - Implements spikard_http::grpc::GrpcHandler trait         │
│  - Bridges between Rust and TypeScript                      │
└──────────────────────┬──────────────────────────────────────┘
                       │
┌──────────────────────▼──────────────────────────────────────┐
│              Spikard gRPC Runtime (Rust)                     │
│  - Tonic-based gRPC server                                  │
│  - HTTP/2 multiplexing with HTTP/1.1                        │
└─────────────────────────────────────────────────────────────┘
```

## Step 1: Define Your Protobuf Schema

Create a `.proto` file with your service definition:

```protobuf
// user.proto
syntax = "proto3";

package myapp.users.v1;

message User {
  int32 id = 1;
  string name = 2;
  string email = 3;
  optional string phone = 4;
}

message GetUserRequest {
  int32 id = 1;
}

message CreateUserRequest {
  string name = 1;
  string email = 2;
  optional string phone = 3;
}

message ListUsersRequest {
  int32 page = 1;
  int32 page_size = 2;
}

message ListUsersResponse {
  repeated User users = 1;
  int32 total_count = 2;
}

service UserService {
  rpc GetUser(GetUserRequest) returns (User);
  rpc CreateUser(CreateUserRequest) returns (User);
  rpc ListUsers(ListUsersRequest) returns (ListUsersResponse);
}
```

## Step 2: Generate TypeScript Types

Use `protobufjs` to generate TypeScript types from your `.proto` file:

```bash
# Install protobufjs and ts-proto (optional, for better TypeScript types)
npm install protobufjs
npm install -D @types/protobufjs

# Generate types using protobufjs CLI
npx pbjs -t static-module -w commonjs -o user_pb.js user.proto
npx pbts -o user_pb.d.ts user_pb.js
```

## Step 3: Implement Your gRPC Handler

Create a TypeScript handler that implements the `GrpcHandler` interface:

```typescript
// user-service.ts
import {
  GrpcHandler,
  GrpcRequest,
  GrpcResponse,
  GrpcError,
  GrpcStatusCode,
  createServiceHandler,
  createUnaryHandler,
} from 'spikard';
import * as userProto from './user_pb';

// Simple in-memory database for demonstration
const users = new Map<number, userProto.myapp.users.v1.User>();
let nextId = 1;

// Initialize with some sample data
users.set(nextId++, {
  id: 1,
  name: 'Alice Johnson',
  email: 'alice@example.com',
  phone: '+1-555-0100',
});
users.set(nextId++, {
  id: 2,
  name: 'Bob Smith',
  email: 'bob@example.com',
});

// Implement GetUser method
const getUser = createUnaryHandler(
  'GetUser',
  async (req: userProto.myapp.users.v1.GetUserRequest, metadata: Record<string, string>) => {
    console.log('GetUser called with id:', req.id);
    console.log('Metadata:', metadata);

    const user = users.get(req.id);
    if (!user) {
      throw new GrpcError(GrpcStatusCode.NOT_FOUND, `User with id ${req.id} not found`);
    }

    return user;
  },
  userProto.myapp.users.v1.GetUserRequest,
  userProto.myapp.users.v1.User,
);

// Implement CreateUser method
const createUser = createUnaryHandler(
  'CreateUser',
  async (req: userProto.myapp.users.v1.CreateUserRequest) => {
    console.log('CreateUser called with:', req);

    // Validate input
    if (!req.name || req.name.trim().length === 0) {
      throw new GrpcError(GrpcStatusCode.INVALID_ARGUMENT, 'Name is required');
    }
    if (!req.email || !req.email.includes('@')) {
      throw new GrpcError(GrpcStatusCode.INVALID_ARGUMENT, 'Valid email is required');
    }

    const id = nextId++;
    const user: userProto.myapp.users.v1.User = {
      id,
      name: req.name,
      email: req.email,
      phone: req.phone,
    };

    users.set(id, user);
    return user;
  },
  userProto.myapp.users.v1.CreateUserRequest,
  userProto.myapp.users.v1.User,
);

// Implement ListUsers method
const listUsers = createUnaryHandler(
  'ListUsers',
  async (req: userProto.myapp.users.v1.ListUsersRequest) => {
    console.log('ListUsers called with page:', req.page, 'page_size:', req.page_size);

    const page = req.page || 1;
    const pageSize = req.page_size || 10;

    const allUsers = Array.from(users.values());
    const start = (page - 1) * pageSize;
    const end = start + pageSize;
    const pageUsers = allUsers.slice(start, end);

    return {
      users: pageUsers,
      totalCount: allUsers.length,
    };
  },
  userProto.myapp.users.v1.ListUsersRequest,
  userProto.myapp.users.v1.ListUsersResponse,
);

// Combine all methods into a single service handler
export const userServiceHandler = createServiceHandler({
  GetUser: getUser,
  CreateUser: createUser,
  ListUsers: listUsers,
});
```

## Step 4: Register the Handler with Spikard

Integrate your gRPC handler with your Spikard application:

```typescript
// app.ts
import { Spikard, ServerConfig } from 'spikard';
import { userServiceHandler } from './user-service';

const app = new Spikard();

// Register the gRPC service handler
// Note: This assumes the Spikard API has been extended to support gRPC registration
// The exact API will depend on the implementation in the Spikard TypeScript package
app.registerGrpcService('myapp.users.v1.UserService', userServiceHandler);

// Configure server
const config: ServerConfig = {
  host: '0.0.0.0',
  port: 50051, // Standard gRPC port
  enableHttpTrace: true,
};

// Start server
app.run(config);

console.log('gRPC server running on port 50051');
```

## Step 5: Test Your Service

Create a gRPC client to test your service:

```typescript
// client.ts
import * as grpc from '@grpc/grpc-js';
import * as protoLoader from '@grpc/proto-loader';

const PROTO_PATH = './user.proto';

// Load the protobuf
const packageDefinition = protoLoader.loadSync(PROTO_PATH, {
  keepCase: true,
  longs: String,
  enums: String,
  defaults: true,
  oneofs: true,
});

const userProto = grpc.loadPackageDefinition(packageDefinition).myapp.users.v1 as any;

// Create client
const client = new userProto.UserService(
  'localhost:50051',
  grpc.credentials.createInsecure()
);

// Test GetUser
client.GetUser({ id: 1 }, (error: any, response: any) => {
  if (error) {
    console.error('Error:', error);
  } else {
    console.log('User:', response);
  }
});

// Test CreateUser
client.CreateUser(
  {
    name: 'Charlie Brown',
    email: 'charlie@example.com',
    phone: '+1-555-0102',
  },
  (error: any, response: any) => {
    if (error) {
      console.error('Error:', error);
    } else {
      console.log('Created user:', response);
    }
  }
);

// Test ListUsers
client.ListUsers({ page: 1, page_size: 10 }, (error: any, response: any) => {
  if (error) {
    console.error('Error:', error);
  } else {
    console.log('Users:', response.users);
    console.log('Total count:', response.totalCount);
  }
});
```

## Advanced: Custom Error Handling

You can throw gRPC errors with specific status codes:

```typescript
import { GrpcError, GrpcStatusCode } from 'spikard';

const deleteUser = createUnaryHandler(
  'DeleteUser',
  async (req: DeleteUserRequest, metadata: Record<string, string>) => {
    // Check authentication
    const authToken = metadata['authorization'];
    if (!authToken || !authToken.startsWith('Bearer ')) {
      throw new GrpcError(
        GrpcStatusCode.UNAUTHENTICATED,
        'Missing or invalid authentication token'
      );
    }

    // Check authorization
    const hasPermission = await checkPermission(authToken, 'delete_user');
    if (!hasPermission) {
      throw new GrpcError(
        GrpcStatusCode.PERMISSION_DENIED,
        'Insufficient permissions to delete users'
      );
    }

    // Check if user exists
    if (!users.has(req.id)) {
      throw new GrpcError(
        GrpcStatusCode.NOT_FOUND,
        `User with id ${req.id} not found`
      );
    }

    // Delete user
    users.delete(req.id);
    return {}; // Empty response
  },
  DeleteUserRequest,
  DeleteUserResponse,
);
```

## Advanced: Response Metadata

You can include metadata in your gRPC responses:

```typescript
const getUserWithMetadata = createUnaryHandler(
  'GetUser',
  async (req: GetUserRequest): Promise<GrpcResponse> => {
    const user = users.get(req.id);
    if (!user) {
      throw new GrpcError(GrpcStatusCode.NOT_FOUND, 'User not found');
    }

    // Serialize the user
    const payload = Buffer.from(User.encode(user).finish());

    // Return response with custom metadata
    return {
      payload,
      metadata: {
        'x-user-id': user.id.toString(),
        'x-cache-status': 'miss',
        'x-server-region': 'us-west-2',
      },
    };
  },
  GetUserRequest,
  User,
);
```

## Error Handling Reference

| gRPC Status Code | When to Use |
|-----------------|-------------|
| `OK` | Success (automatic) |
| `CANCELLED` | Client cancelled the request |
| `UNKNOWN` | Unknown error |
| `INVALID_ARGUMENT` | Invalid input parameters |
| `DEADLINE_EXCEEDED` | Operation timeout |
| `NOT_FOUND` | Resource not found |
| `ALREADY_EXISTS` | Resource already exists |
| `PERMISSION_DENIED` | Insufficient permissions |
| `RESOURCE_EXHAUSTED` | Rate limit exceeded |
| `FAILED_PRECONDITION` | System not in required state |
| `ABORTED` | Operation aborted |
| `OUT_OF_RANGE` | Out of valid range |
| `UNIMPLEMENTED` | Method not implemented |
| `INTERNAL` | Internal server error |
| `UNAVAILABLE` | Service unavailable |
| `DATA_LOSS` | Data corruption |
| `UNAUTHENTICATED` | Missing authentication |

## Performance Tips

1. **Reuse protobuf instances**: Create protobuf type instances once and reuse them
2. **Buffer pooling**: Consider using Buffer pools for large messages
3. **Async processing**: Use async/await for all I/O operations
4. **Error handling**: Use specific gRPC error codes instead of generic errors
5. **Metadata**: Keep metadata small to reduce overhead

## Next Steps

- Implement streaming RPCs (server streaming, client streaming, bidirectional)
- Add authentication middleware for gRPC
- Implement request/response logging
- Add metrics and monitoring
- Deploy with load balancing

## Resources

- [gRPC Documentation](https://grpc.io/docs/)
- [protobufjs Documentation](https://protobufjs.github.io/protobuf.js/)
- [Spikard Documentation](https://docs.spikard.dev)
- [gRPC Status Codes](https://grpc.io/docs/guides/status-codes/)
