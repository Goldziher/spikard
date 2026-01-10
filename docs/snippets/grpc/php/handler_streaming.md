# PHP gRPC Streaming Handlers

Complete PHP implementation examples for client streaming and bidirectional streaming gRPC handlers in Spikard.

## Client Streaming Handler

Client streaming RPC allows a client to send multiple messages and then the server responds with a single message.

### Handler Signature

```php
<?php

declare(strict_types=1);

use Spikard\Grpc\ClientStreamRequest;
use Spikard\Grpc\Response;

/**
 * Client streaming handler signature
 */
public function handleClientStream(ClientStreamRequest $request): Response
{
    // $request->serviceName: string
    // $request->methodName: string
    // $request->metadata: array<string, string>
    // $request->messages: string[]  // Array of serialized messages

    // Process all messages and return single response
}
```

### Example: Batch Message Processing

```php
<?php

declare(strict_types=1);

use Spikard\Grpc\HandlerInterface;
use Spikard\Grpc\ClientStreamRequest;
use Spikard\Grpc\Response;
use Messageservice\Item;
use Messageservice\BatchCreateResponse;

class MessageServiceHandler implements HandlerInterface
{
    public function __construct(
        private MessageRepository $repository,
    ) {}

    /**
     * Handle client streaming: receive multiple messages, send single response
     *
     * Pattern: Collect all input messages, process together, return aggregated response
     */
    public function handleClientStream(ClientStreamRequest $request): Response
    {
        try {
            // Step 1: Validate authorization
            $authToken = $request->getMetadata('authorization');
            if (!$authToken) {
                return Response::error(
                    'Authentication required',
                    ['grpc-status' => 'UNAUTHENTICATED']
                );
            }

            // Step 2: Deserialize all input messages
            $items = [];
            $successCount = 0;
            $totalValue = 0;

            foreach ($request->messages as $index => $messagePayload) {
                try {
                    $item = new Item();
                    $item->mergeFromString($messagePayload);

                    // Validate item
                    if (empty($item->getName()) || $item->getValue() <= 0) {
                        return Response::error(
                            "Invalid item at index {$index}: name and positive value required"
                        );
                    }

                    $items[] = $item;
                } catch (\Exception $e) {
                    return Response::error(
                        "Failed to decode message at index {$index}: {$e->getMessage()}"
                    );
                }
            }

            if (empty($items)) {
                return Response::error('At least one item is required');
            }

            // Step 3: Process all items atomically
            try {
                $this->repository->transaction(function () use ($items, &$successCount, &$totalValue): void {
                    foreach ($items as $item) {
                        $this->repository->create(
                            name: $item->getName(),
                            value: $item->getValue()
                        );
                        $successCount++;
                        $totalValue += $item->getValue();
                    }
                });
            } catch (\Exception $e) {
                return Response::error("Transaction failed: {$e->getMessage()}");
            }

            // Step 4: Build aggregate response
            $batchId = bin2hex(random_bytes(8));
            $response = new BatchCreateResponse();
            $response->setSuccessCount($successCount);
            $response->setTotalValue($totalValue);
            $response->setBatchId($batchId);
            $response->setTimestamp((new \DateTime())->format('c'));

            // Step 5: Serialize and return
            return new Response(
                payload: $response->serializeToString(),
                metadata: [
                    'x-batch-id' => $batchId,
                    'x-count' => (string)$successCount,
                ]
            );

        } catch (\Exception $e) {
            return Response::error("Error: {$e->getMessage()}");
        }
    }

    public function handleRequest(\Spikard\Grpc\Request $request): Response
    {
        // Route unary requests
        return match ($request->methodName) {
            default => Response::error("Unknown method: {$request->methodName}"),
        };
    }
}
```

## Bidirectional Streaming Handler

Bidirectional streaming RPC allows the client to send multiple messages and the server to send multiple messages back.

### Handler Signature

```php
<?php

declare(strict_types=1);

use Spikard\Grpc\BidiStreamRequest;
use Spikard\Grpc\BidiStreamResponse;

/**
 * Bidirectional streaming handler signature
 */
public function handleBidiStream(BidiStreamRequest $request): BidiStreamResponse
{
    // $request->serviceName: string
    // $request->methodName: string
    // $request->metadata: array<string, string>
    // $request->messages: string[]  // Array of serialized input messages

    // Process input messages and generate output messages
    // return new BidiStreamResponse(
    //     messages: string[],        // Array of serialized response messages
    //     metadata: array<string, string>
    // );
}
```

### Example: Message Transformation Pipeline

```php
<?php

declare(strict_types=1);

use Spikard\Grpc\HandlerInterface;
use Spikard\Grpc\BidiStreamRequest;
use Spikard\Grpc\BidiStreamResponse;
use Spikard\Grpc\Request;
use Spikard\Grpc\Response;
use Transformservice\Document;
use Transformservice\TransformResult;

class TransformServiceHandler implements HandlerInterface
{
    public function __construct(
        private DocumentTransformer $transformer,
    ) {}

    /**
     * Handle bidirectional streaming: receive multiple messages, send multiple messages
     *
     * Pattern: Collect all input messages, process, generate output messages, return array
     */
    public function handleBidiStream(BidiStreamRequest $request): BidiStreamResponse
    {
        try {
            // Step 1: Deserialize all input messages
            $inputDocuments = [];

            foreach ($request->messages as $index => $messagePayload) {
                try {
                    $document = new Document();
                    $document->mergeFromString($messagePayload);

                    $inputDocuments[] = [
                        'index' => $index,
                        'document' => $document,
                    ];
                } catch (\Exception $e) {
                    // Create error response for this document
                    $errorResult = new TransformResult();
                    $errorResult->setStatus('ERROR');
                    $errorResult->setErrorMessage(
                        "Failed to decode message at index {$index}: {$e->getMessage()}"
                    );

                    // Still return partial response
                    return new BidiStreamResponse(
                        messages: [$errorResult->serializeToString()],
                        metadata: [
                            'x-error-at-index' => (string)$index,
                        ]
                    );
                }
            }

            if (empty($inputDocuments)) {
                return new BidiStreamResponse(
                    messages: [],
                    metadata: ['x-processed-count' => '0']
                );
            }

            // Step 2: Process each document and generate response
            $outputMessages = [];
            $processedCount = 0;
            $errorCount = 0;

            foreach ($inputDocuments as $input) {
                try {
                    $document = $input['document'];

                    // Transform the document
                    $transformedContent = $this->transformer->transform(
                        $document->getContent()
                    );

                    // Build response message
                    $result = new TransformResult();
                    $result->setOriginalId($document->getId());
                    $result->setTransformedContent($transformedContent);
                    $result->setTransformedAt((new \DateTime())->format('c'));
                    $result->setStatus('SUCCESS');

                    // Add metadata about transformation
                    $metadata = new \Google\Protobuf\StringValue();
                    $metadata->setValue(json_encode([
                        'original_size' => strlen($document->getContent()),
                        'transformed_size' => strlen($transformedContent),
                    ], JSON_THROW_ON_ERROR));
                    $result->setMetadata($metadata);

                    // Serialize and collect response
                    $outputMessages[] = $result->serializeToString();
                    $processedCount++;

                } catch (\Exception $e) {
                    // Create error response for this document
                    $errorResult = new TransformResult();
                    $errorResult->setOriginalId($input['document']->getId());
                    $errorResult->setStatus('ERROR');
                    $errorResult->setErrorMessage($e->getMessage());

                    $outputMessages[] = $errorResult->serializeToString();
                    $errorCount++;
                }
            }

            // Step 3: Return all response messages
            return new BidiStreamResponse(
                messages: $outputMessages,
                metadata: [
                    'x-processed-count' => (string)$processedCount,
                    'x-error-count' => (string)$errorCount,
                    'x-timestamp' => (new \DateTime())->format('c'),
                ]
            );

        } catch (\Exception $e) {
            // Return error response
            $errorResult = new TransformResult();
            $errorResult->setStatus('ERROR');
            $errorResult->setErrorMessage("Stream processing failed: {$e->getMessage()}");

            return new BidiStreamResponse(
                messages: [$errorResult->serializeToString()],
                metadata: ['x-error' => 'true']
            );
        }
    }

    public function handleRequest(Request $request): Response
    {
        // Route unary requests
        return Response::error("Unknown method: {$request->methodName}");
    }
}
```

## Advanced Example: Filtering and Aggregation

### Bidirectional Stream with Filtering

```php
<?php

declare(strict_types=1);

use Spikard\Grpc\BidiStreamRequest;
use Spikard\Grpc\BidiStreamResponse;
use Recordservice\Record;
use Recordservice\ProcessedRecord;

class RecordServiceHandler
{
    public function handleBidiStream(BidiStreamRequest $request): BidiStreamResponse
    {
        try {
            // Parse filter criteria from metadata
            $filterType = $request->getMetadata('x-filter-type') ?? 'all';
            $minValue = (int)($request->getMetadata('x-min-value') ?? '0');
            $maxValue = (int)($request->getMetadata('x-max-value') ?? PHP_INT_MAX);

            // Step 1: Deserialize and filter
            $filteredRecords = [];
            $inputCount = count($request->messages);

            foreach ($request->messages as $messagePayload) {
                $record = new Record();
                $record->mergeFromString($messagePayload);

                // Apply filter logic
                $value = $record->getValue();

                if ($this->matchesFilter($record, $filterType, $minValue, $maxValue)) {
                    $filteredRecords[] = $record;
                }
            }

            // Step 2: Transform filtered records
            $outputMessages = [];

            foreach ($filteredRecords as $record) {
                $processed = new ProcessedRecord();
                $processed->setId($record->getId());
                $processed->setOriginalValue($record->getValue());

                // Apply business logic: apply multiplier
                $multiplier = $this->calculateMultiplier($record);
                $processed->setProcessedValue($record->getValue() * $multiplier);
                $processed->setFiltered(false);
                $processed->setProcessedAt((new \DateTime())->format('c'));

                $outputMessages[] = $processed->serializeToString();
            }

            // Step 3: Return response with statistics
            return new BidiStreamResponse(
                messages: $outputMessages,
                metadata: [
                    'x-input-count' => (string)$inputCount,
                    'x-output-count' => (string)count($outputMessages),
                    'x-filtered-count' => (string)($inputCount - count($outputMessages)),
                    'x-filter-type' => $filterType,
                ]
            );

        } catch (\Exception $e) {
            $errorResult = new ProcessedRecord();
            $errorResult->setId('error');

            return new BidiStreamResponse(
                messages: [$errorResult->serializeToString()],
                metadata: ['x-error' => $e->getMessage()]
            );
        }
    }

    private function matchesFilter(
        Record $record,
        string $filterType,
        int $minValue,
        int $maxValue
    ): bool {
        return match ($filterType) {
            'all' => true,
            'high-value' => $record->getValue() >= $minValue && $record->getValue() <= $maxValue,
            'range' => $record->getValue() >= $minValue && $record->getValue() <= $maxValue,
            'low-value' => $record->getValue() < $minValue,
            default => true,
        };
    }

    private function calculateMultiplier(Record $record): float
    {
        return match (true) {
            $record->getValue() > 1000 => 1.05,
            $record->getValue() > 500 => 1.03,
            default => 1.01,
        };
    }
}
```

## Key Patterns

### Message Collection
- All client messages are collected in the `messages` array property
- Messages are already serialized as binary strings (use `mergeFromString()` to deserialize)
- No streaming iteration needed - full array is provided

### Processing Strategy
1. **Collect**: All input messages received as array
2. **Validate**: Check all messages before processing
3. **Transform**: Process and generate output
4. **Serialize**: Encode response messages with `serializeToString()`
5. **Return**: Array of response messages in response object

### Error Handling
- Return `Response::error()` for fatal errors that abort the stream
- Per-message errors can be included in response messages (with ERROR status)
- Deserialization errors should return error response immediately
- Processing errors can return partial results (successful messages + error messages)

### Metadata
- Client streaming: Metadata passed in request, can be included in response
- Bidirectional streaming: Metadata passed in request, can be included in response
- Use metadata for non-payload information (timestamps, counts, filters)

## Limits and Constraints

- **MAX_STREAM_MESSAGES**: 10,000 messages per stream
- **Resource Exhaustion**: Streams exceeding limit return error response
- **Memory**: All messages collected in memory - appropriate for moderate message counts
- **Atomicity**: All messages processed together (transaction-like semantics for client streaming)

## Testing Client Streaming

```php
<?php

declare(strict_types=1);

namespace Tests;

use PHPUnit\Framework\TestCase;
use PHPUnit\Framework\MockObject\MockObject;
use Spikard\Grpc\ClientStreamRequest;
use Messageservice\Item;
use Messageservice\BatchCreateResponse;

class ClientStreamHandlerTest extends TestCase
{
    private MessageServiceHandler $handler;
    private MockObject $repository;

    protected function setUp(): void
    {
        $this->repository = $this->createMock(MessageRepository::class);
        $this->handler = new MessageServiceHandler($this->repository);
    }

    public function testClientStreamBatchCreate(): void
    {
        // Setup mock
        $this->repository
            ->expects($this->once())
            ->method('transaction')
            ->willReturnCallback(function (callable $callback): void {
                $callback();
            });

        $this->repository
            ->expects($this->exactly(3))
            ->method('create')
            ->willReturn(true);

        // Create multiple input messages
        $items = [
            $this->createItem('Item 1', 100),
            $this->createItem('Item 2', 200),
            $this->createItem('Item 3', 300),
        ];

        $messages = array_map(
            fn(Item $item) => $item->serializeToString(),
            $items
        );

        // Create request
        $request = new ClientStreamRequest(
            serviceName: 'messageservice.MessageService',
            methodName: 'BatchCreate',
            messages: $messages,
            metadata: ['authorization' => 'Bearer token']
        );

        // Call handler
        $response = $this->handler->handleClientStream($request);

        // Verify response
        $this->assertFalse($response->isError());

        $result = new BatchCreateResponse();
        $result->mergeFromString($response->payload);

        $this->assertEquals(3, $result->getSuccessCount());
        $this->assertEquals(600, $result->getTotalValue());
        $this->assertNotEmpty($result->getBatchId());
        $this->assertEquals('3', $response->getMetadata('x-count'));
    }

    public function testClientStreamRequiresAuthentication(): void
    {
        // Create request without auth
        $item = $this->createItem('Item 1', 100);

        $request = new ClientStreamRequest(
            serviceName: 'messageservice.MessageService',
            methodName: 'BatchCreate',
            messages: [$item->serializeToString()],
            metadata: []  // No authorization
        );

        // Call handler
        $response = $this->handler->handleClientStream($request);

        // Should return auth error
        $this->assertTrue($response->isError());
        $this->assertStringContainsString('Authentication required', $response->errorMessage);
        $this->assertEquals('UNAUTHENTICATED', $response->getMetadata('grpc-status'));
    }

    public function testClientStreamValidatesItems(): void
    {
        // Create item with invalid value
        $item = $this->createItem('Item 1', -100);

        $request = new ClientStreamRequest(
            serviceName: 'messageservice.MessageService',
            methodName: 'BatchCreate',
            messages: [$item->serializeToString()],
            metadata: ['authorization' => 'Bearer token']
        );

        // Call handler
        $response = $this->handler->handleClientStream($request);

        // Should return validation error
        $this->assertTrue($response->isError());
        $this->assertStringContainsString('positive value required', $response->errorMessage);
    }

    public function testClientStreamHandlesDeserializationError(): void
    {
        $request = new ClientStreamRequest(
            serviceName: 'messageservice.MessageService',
            methodName: 'BatchCreate',
            messages: ['invalid protobuf data'],
            metadata: ['authorization' => 'Bearer token']
        );

        // Call handler
        $response = $this->handler->handleClientStream($request);

        // Should return deserialization error
        $this->assertTrue($response->isError());
        $this->assertStringContainsString('Failed to decode', $response->errorMessage);
    }

    private function createItem(string $name, int $value): Item
    {
        $item = new Item();
        $item->setName($name);
        $item->setValue($value);
        return $item;
    }
}
```

## Testing Bidirectional Streaming

```php
<?php

declare(strict_types=1);

namespace Tests;

use PHPUnit\Framework\TestCase;
use PHPUnit\Framework\MockObject\MockObject;
use Spikard\Grpc\BidiStreamRequest;
use Transformservice\Document;
use Transformservice\TransformResult;

class BidiStreamHandlerTest extends TestCase
{
    private TransformServiceHandler $handler;
    private MockObject $transformer;

    protected function setUp(): void
    {
        $this->transformer = $this->createMock(DocumentTransformer::class);
        $this->handler = new TransformServiceHandler($this->transformer);
    }

    public function testBidiStreamTransformMultipleDocuments(): void
    {
        // Setup mock transformer
        $this->transformer
            ->method('transform')
            ->willReturnCallback(fn(string $content) => strtoupper($content));

        // Create input documents
        $documents = [
            $this->createDocument(1, 'hello world'),
            $this->createDocument(2, 'foo bar'),
            $this->createDocument(3, 'test content'),
        ];

        $messages = array_map(
            fn(Document $doc) => $doc->serializeToString(),
            $documents
        );

        // Create request
        $request = new BidiStreamRequest(
            serviceName: 'transformservice.TransformService',
            methodName: 'TransformStream',
            messages: $messages,
            metadata: ['authorization' => 'Bearer token']
        );

        // Call handler
        $response = $this->handler->handleBidiStream($request);

        // Verify response contains multiple messages
        $this->assertCount(3, $response->messages);

        // Verify each response message
        foreach ($response->messages as $index => $messagePayload) {
            $result = new TransformResult();
            $result->mergeFromString($messagePayload);

            $this->assertEquals('SUCCESS', $result->getStatus());
            $this->assertNotEmpty($result->getTransformedContent());
            $this->assertEquals((string)($index + 1), $result->getOriginalId());
        }

        // Verify metadata
        $this->assertEquals('3', $response->getMetadata('x-processed-count'));
        $this->assertEquals('0', $response->getMetadata('x-error-count'));
    }

    public function testBidiStreamPartialErrors(): void
    {
        // Setup transformer to fail on specific content
        $this->transformer
            ->method('transform')
            ->willReturnCallback(function (string $content) {
                if ($content === 'error') {
                    throw new \Exception('Transform failed');
                }
                return strtoupper($content);
            });

        // Create documents, one will fail
        $documents = [
            $this->createDocument(1, 'success'),
            $this->createDocument(2, 'error'),
            $this->createDocument(3, 'success'),
        ];

        $messages = array_map(
            fn(Document $doc) => $doc->serializeToString(),
            $documents
        );

        $request = new BidiStreamRequest(
            serviceName: 'transformservice.TransformService',
            methodName: 'TransformStream',
            messages: $messages,
            metadata: []
        );

        // Call handler
        $response = $this->handler->handleBidiStream($request);

        // Should have response for all documents
        $this->assertCount(3, $response->messages);

        // Check results
        $results = [];
        foreach ($response->messages as $messagePayload) {
            $result = new TransformResult();
            $result->mergeFromString($messagePayload);
            $results[] = $result->getStatus();
        }

        $this->assertContains('SUCCESS', $results);
        $this->assertContains('ERROR', $results);

        // Verify error count in metadata
        $this->assertEquals('1', $response->getMetadata('x-error-count'));
    }

    public function testBidiStreamEmptyInput(): void
    {
        $request = new BidiStreamRequest(
            serviceName: 'transformservice.TransformService',
            methodName: 'TransformStream',
            messages: [],
            metadata: []
        );

        // Call handler
        $response = $this->handler->handleBidiStream($request);

        // Should return empty messages
        $this->assertEmpty($response->messages);
        $this->assertEquals('0', $response->getMetadata('x-processed-count'));
    }

    private function createDocument(int $id, string $content): Document
    {
        $document = new Document();
        $document->setId((string)$id);
        $document->setContent($content);
        return $document;
    }
}
```

## Testing Filtering and Aggregation

```php
<?php

declare(strict_types=1);

namespace Tests;

use PHPUnit\Framework\TestCase;
use Spikard\Grpc\BidiStreamRequest;
use Recordservice\Record;
use Recordservice\ProcessedRecord;

class FilterStreamHandlerTest extends TestCase
{
    private RecordServiceHandler $handler;

    protected function setUp(): void
    {
        $this->handler = new RecordServiceHandler();
    }

    /**
     * @dataProvider filterTypeProvider
     */
    public function testBidiStreamWithFilters(
        string $filterType,
        int $minValue,
        int $expectedOutputCount
    ): void {
        // Create records with various values
        $records = [
            $this->createRecord('1', 100),
            $this->createRecord('2', 500),
            $this->createRecord('3', 1500),
            $this->createRecord('4', 2500),
        ];

        $messages = array_map(
            fn(Record $record) => $record->serializeToString(),
            $records
        );

        $request = new BidiStreamRequest(
            serviceName: 'recordservice.RecordService',
            methodName: 'FilterStream',
            messages: $messages,
            metadata: [
                'x-filter-type' => $filterType,
                'x-min-value' => (string)$minValue,
            ]
        );

        // Call handler
        $response = $this->handler->handleBidiStream($request);

        // Verify filtered count
        $this->assertCount($expectedOutputCount, $response->messages);
        $this->assertEquals(
            (string)$expectedOutputCount,
            $response->getMetadata('x-output-count')
        );
    }

    public static function filterTypeProvider(): array
    {
        return [
            'all records' => ['all', 0, 4],
            'high value (>=500)' => ['high-value', 500, 3],
            'range (>=1000, <=2000)' => ['range', 1000, 1],
            'low value (<500)' => ['low-value', 500, 1],
        ];
    }

    public function testBidiStreamAppliesMultiplier(): void
    {
        $record = $this->createRecord('1', 1500);

        $request = new BidiStreamRequest(
            serviceName: 'recordservice.RecordService',
            methodName: 'FilterStream',
            messages: [$record->serializeToString()],
            metadata: ['x-filter-type' => 'all']
        );

        $response = $this->handler->handleBidiStream($request);

        // Parse response
        $result = new ProcessedRecord();
        $result->mergeFromString($response->messages[0]);

        // Verify multiplier applied (1500 >= 1000, so multiplier = 1.05)
        $expectedValue = 1500 * 1.05;
        $this->assertEquals($expectedValue, $result->getProcessedValue());
    }

    private function createRecord(string $id, int $value): Record
    {
        $record = new Record();
        $record->setId($id);
        $record->setValue($value);
        return $record;
    }
}
```

## Comparison with Other Patterns

| Aspect | Client Streaming | Bidirectional | Unary |
|--------|------------------|---------------|-------|
| Input | Multiple messages | Multiple messages | Single message |
| Output | Single response | Multiple messages | Single response |
| Use case | Batch operations | Stream processing | Simple requests |
| Message order | Important | Important | N/A |
| Atomicity | Full batch atomic | Per-message or batch | Single atomic |
| PHP Pattern | `handleClientStream()` | `handleBidiStream()` | `handleRequest()` |

## Key PHP-Specific Patterns

### Using `serializeToString()` and `mergeFromString()`
- Always use `mergeFromString()` to deserialize protobuf messages
- Always use `serializeToString()` to serialize protobuf responses
- Never use `parse()` for deserialization in handlers

### Error Handling
- Prefer returning error responses over throwing exceptions
- Use try-catch to handle deserialization and business logic errors
- Include context in error messages (index, original value, etc.)

### Metadata Access
- Use `getMetadata(string $key)` to safely access metadata with null coalescing
- Return metadata in response for logging and debugging
- Use standard header names (e.g., 'x-batch-id', 'x-count')

### Type Hints
- Use strict type hints on all function parameters
- Return types must be `Response` or `BidiStreamResponse`
- Use `declare(strict_types=1)` in all files

See the [gRPC documentation](../../../grpc-php-example.md) for more examples.
