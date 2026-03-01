```typescript
import { GrpcService, Spikard } from 'spikard';

const grpcService = new GrpcService();

grpcService.registerHandler('userservice.UserService', userServiceHandler);

const app = new Spikard();
app.useGrpc(grpcService);
```
