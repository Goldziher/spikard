# TypeScript gRPC Streaming Handlers

Complete TypeScript implementation examples for client streaming and bidirectional streaming gRPC handlers in Spikard.

## Client Streaming Handler

Client streaming RPC allows a client to send multiple messages and then the server responds with a single message.

### Handler Signature

```typescript
interface GrpcClientStreamRequest {
  serviceName: string;
  methodName: string;
  metadata: Record<string, string>;
  messages: Buffer[];  // All client messages collected as array
}

async function handleClientStream(
  request: GrpcClientStreamRequest
): Promise<GrpcResponse> {
  // Process all messages and return single response
}
```

### Example: Batch Message Processing

```typescript
import {
  GrpcError,
  GrpcStatusCode,
  GrpcClientStreamRequest,
  GrpcResponse,
} from 'spikard';
import * as messageservice from './messageservice_pb';

/**
 * Handle client streaming: receive multiple messages, send single response
 * 
 * Pattern: Collect all input messages, process together, return aggregated response
 */
async function handleBatchCreate(
  request: GrpcClientStreamRequest
): Promise<GrpcResponse> {
  const { messages, metadata } = request;

  // Validate authorization
  const authToken = metadata['authorization'];
  if (!authToken) {
    throw new GrpcError(
      GrpcStatusCode.UNAUTHENTICATED,
      'Authentication required'
    );
  }

  // Step 1: Deserialize all input messages
  const items = messages.map((msg) => {
    try {
      return messageservice.Item.decode(msg);
    } catch (err) {
      throw new GrpcError(
        GrpcStatusCode.INVALID_ARGUMENT,
        `Failed to decode message: ${err.message}`
      );
    }
  });

  // Validate all items before processing
  for (const item of items) {
    if (!item.name || !item.value) {
      throw new GrpcError(
        GrpcStatusCode.INVALID_ARGUMENT,
        'Each item must have name and value'
      );
    }
  }

  // Step 2: Process all items atomically
  const database = getDatabase();
  let successCount = 0;
  let totalValue = 0;

  try {
    // Begin transaction
    await database.transaction(async (tx) => {
      for (const item of items) {
        const created = await tx.items.create({
          name: item.name,
          value: item.value,
        });
        successCount++;
        totalValue += item.value;
      }
    });
  } catch (err) {
    throw new GrpcError(
      GrpcStatusCode.INTERNAL,
      `Transaction failed: ${err.message}`
    );
  }

  // Step 3: Build aggregate response
  const response = messageservice.BatchCreateResponse.create({
    successCount,
    totalValue,
    batchId: generateId(),
    timestamp: new Date().toISOString(),
  });

  // Step 4: Serialize and return
  const encoded = messageservice.BatchCreateResponse.encode(response).finish();
  return {
    payload: Buffer.from(encoded),
    metadata: {
      'x-batch-id': response.batchId,
      'x-count': successCount.toString(),
    },
  };
}
```

## Bidirectional Streaming Handler

Bidirectional streaming RPC allows the client to send multiple messages and the server to send multiple messages back.

### Handler Signature

```typescript
interface GrpcBidiStreamRequest {
  serviceName: string;
  methodName: string;
  metadata: Record<string, string>;
  messages: Buffer[];  // All input messages collected as array
}

interface GrpcBidiStreamResponse {
  messages: Buffer[];  // Array of response messages
  metadata?: Record<string, string>;
}

async function handleBidiStream(
  request: GrpcBidiStreamRequest
): Promise<GrpcBidiStreamResponse> {
  // Process input messages and generate output messages
}
```

### Example: Message Transformation Pipeline

```typescript
import {
  GrpcError,
  GrpcStatusCode,
  GrpcBidiStreamRequest,
  GrpcBidiStreamResponse,
} from 'spikard';
import * as transformservice from './transformservice_pb';

/**
 * Handle bidirectional streaming: receive multiple messages, send multiple messages
 * 
 * Pattern: Collect all input messages, process, generate output messages, return array
 */
async function handleTransformStream(
  request: GrpcBidiStreamRequest
): Promise<GrpcBidiStreamResponse> {
  const { messages, metadata } = request;

  // Step 1: Deserialize all input messages
  const inputDocuments = messages.map((msg, index) => {
    try {
      return {
        index,
        document: transformservice.Document.decode(msg),
      };
    } catch (err) {
      throw new GrpcError(
        GrpcStatusCode.INVALID_ARGUMENT,
        `Failed to decode message ${index}: ${err.message}`
      );
    }
  });

  // Step 2: Process each document and generate response
  const outputMessages: Buffer[] = [];

  for (const { index, document } of inputDocuments) {
    try {
      // Transform the document
      const transformed = await transformDocument(document);

      // Build response message
      const result = transformservice.TransformResult.create({
        originalId: document.id,
        transformedContent: transformed.content,
        transformedAt: new Date().toISOString(),
        status: transformed.success ? 'SUCCESS' : 'PARTIAL',
        metadata: {
          originalSize: document.content.length,
          transformedSize: transformed.content.length,
        },
      });

      // Serialize response message
      const encoded = transformservice.TransformResult.encode(result).finish();
      outputMessages.push(Buffer.from(encoded));
    } catch (err) {
      // Create error response for this document
      const errorResult = transformservice.TransformResult.create({
        originalId: document.id,
        status: 'ERROR',
        errorMessage: err.message,
      });

      const encoded = transformservice.TransformResult.encode(errorResult).finish();
      outputMessages.push(Buffer.from(encoded));
    }
  }

  // Step 3: Return all response messages
  return {
    messages: outputMessages,
    metadata: {
      'x-processed-count': outputMessages.length.toString(),
      'x-timestamp': new Date().toISOString(),
    },
  };
}

async function transformDocument(doc: any): Promise<any> {
  // Simulate document transformation
  return {
    content: doc.content.toUpperCase(),
    success: true,
  };
}
```

## Advanced Example: Filtering and Aggregation

### Bidirectional Stream with Filtering

```typescript
async function handleFilterStream(
  request: GrpcBidiStreamRequest
): Promise<GrpcBidiStreamResponse> {
  const { messages, metadata } = request;

  // Parse filter criteria from metadata
  const filterType = metadata['x-filter-type'] || 'all';
  const minValue = parseInt(metadata['x-min-value'] || '0', 10);

  // Step 1: Deserialize and filter
  const filteredItems: any[] = [];

  for (const msg of messages) {
    const item = recordservice.Record.decode(msg);

    // Apply filter logic
    if (filterType === 'all' || (filterType === 'high-value' && item.value >= minValue)) {
      filteredItems.push(item);
    }
  }

  // Step 2: Transform filtered items
  const outputMessages = filteredItems.map((item) => {
    const response = recordservice.ProcessedRecord.create({
      id: item.id,
      originalValue: item.value,
      processedValue: item.value * 1.1,  // Apply multiplier
      filtered: false,
    });

    return Buffer.from(recordservice.ProcessedRecord.encode(response).finish());
  });

  // Step 3: Return response with statistics
  return {
    messages: outputMessages,
    metadata: {
      'x-input-count': messages.length.toString(),
      'x-output-count': outputMessages.length.toString(),
      'x-filtered-count': (messages.length - outputMessages.length).toString(),
    },
  };
}
```

## Key Patterns

### Message Collection
- All client messages are collected in a single `messages` array
- Messages are already deserialized as `Buffer` objects
- No streaming iteration needed - full array is provided

### Processing Strategy
1. **Collect**: All input messages received as array
2. **Validate**: Check all messages before processing
3. **Transform**: Process and generate output
4. **Serialize**: Encode response messages as Buffers
5. **Return**: Array of response messages

### Error Handling
- Throw `GrpcError` with appropriate status codes
- Errors in message deserialization should use `INVALID_ARGUMENT`
- Processing errors should use `INTERNAL` or domain-specific codes
- Per-message errors can be included in response messages (with ERROR status)

### Metadata
- Client streaming: Metadata passed in request, can be included in response
- Bidirectional streaming: Metadata passed in request, can be included in response
- Use metadata for non-payload information (timestamps, counts, filters)

## Limits and Constraints

- **MAX_STREAM_MESSAGES**: 10,000 messages per stream
- **Resource Exhaustion**: Streams exceeding limit return `RESOURCE_EXHAUSTED` error
- **Memory**: All messages collected in memory - appropriate for moderate message counts
- **Atomicity**: All messages processed together (transaction-like semantics)

## Testing Client Streaming

```typescript
import { createTestClient } from 'spikard/testing';

describe('Client Streaming Handler', () => {
  it('should process batch of items', async () => {
    const client = createTestClient(userServiceHandler);

    // Create multiple input messages
    const items = [
      messageservice.Item.create({ name: 'Item 1', value: 100 }),
      messageservice.Item.create({ name: 'Item 2', value: 200 }),
      messageservice.Item.create({ name: 'Item 3', value: 300 }),
    ];

    const messages = items.map((item) =>
      Buffer.from(messageservice.Item.encode(item).finish())
    );

    // Call client streaming handler
    const response = await client.callClientStream(
      'myservice.MyService',
      'BatchCreate',
      {
        messages,
        metadata: { authorization: 'Bearer token' },
      }
    );

    // Verify response
    const result = messageservice.BatchCreateResponse.decode(response.payload);
    expect(result.successCount).toBe(3);
    expect(result.totalValue).toBe(600);
  });
});
```

## Testing Bidirectional Streaming

```typescript
describe('Bidirectional Streaming Handler', () => {
  it('should transform and return multiple messages', async () => {
    const client = createTestClient(transformServiceHandler);

    // Create input messages
    const documents = [
      transformservice.Document.create({ id: 1, content: 'hello' }),
      transformservice.Document.create({ id: 2, content: 'world' }),
    ];

    const messages = documents.map((doc) =>
      Buffer.from(transformservice.Document.encode(doc).finish())
    );

    // Call bidirectional streaming handler
    const response = await client.callBidiStream(
      'myservice.MyService',
      'TransformStream',
      {
        messages,
        metadata: { authorization: 'Bearer token' },
      }
    );

    // Verify multiple response messages
    expect(response.messages).toHaveLength(2);

    response.messages.forEach((msg) => {
      const result = transformservice.TransformResult.decode(msg);
      expect(result.status).toBe('SUCCESS');
      expect(result.transformedContent).toBeDefined();
    });
  });
});
```

## Comparison with Other Patterns

| Aspect | Client Streaming | Bidirectional | Unary |
|--------|------------------|---------------|-------|
| Input | Multiple messages | Multiple messages | Single message |
| Output | Single response | Multiple messages | Single response |
| Use case | Batch operations | Stream processing | Simple requests |
| Message order | Important | Important | N/A |
| Atomicity | Full batch atomic | Per-message or batch | Single atomic |

See the [gRPC documentation](../../../grpc-typescript-example.md) for more examples.
