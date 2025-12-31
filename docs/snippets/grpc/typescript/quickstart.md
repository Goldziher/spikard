```typescript
// user_handler.ts
import { GrpcRequest, GrpcResponse } from '@spikard/node';
import { userservice } from './user_service';

export class UserServiceHandler {
  async handleRequest(request: GrpcRequest): Promise<GrpcResponse> {
    if (request.methodName === 'GetUser') {
      // Deserialize
      const req = userservice.v1.GetUserRequest.decode(request.payload);

      // Process
      const user = userservice.v1.User.create({
        id: req.userId,
        name: 'Alice',
        email: 'alice@example.com'
      });

      // Serialize and return
      const payload = userservice.v1.User.encode(user).finish();
      return new GrpcResponse({ payload });
    }
  }
}
```
