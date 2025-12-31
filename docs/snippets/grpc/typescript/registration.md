```typescript
import { createApp } from 'spikard';

const app = createApp();

// Register gRPC handler
app.registerGrpcHandler('userservice.UserService', userServiceHandler);

// Server ready
await app.listen(50051);
```
