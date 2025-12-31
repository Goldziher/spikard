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
