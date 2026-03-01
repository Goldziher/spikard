```typescript
import { GrpcService, Spikard } from 'spikard';

const grpcService = new GrpcService();

grpcService.registerUnary('userservice.UserService', 'GetUser', userServiceHandler);

const app = new Spikard();
app.useGrpc(grpcService);
```
