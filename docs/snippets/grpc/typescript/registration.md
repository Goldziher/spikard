```typescript
import { GrpcService } from 'spikard';

const grpcService = new GrpcService();

grpcService.registerHandler('userservice.UserService', userServiceHandler);
```
