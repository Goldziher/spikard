# Python gRPC Streaming Handlers

Complete Python implementation examples for client streaming and bidirectional streaming gRPC handlers in Spikard.

## Client Streaming Handler

Client streaming RPC allows a client to send multiple messages and then the server responds with a single message.

### Handler Signature

```python
from typing import List
from spikard.grpc import GrpcClientStreamRequest, GrpcResponse

class GrpcClientStreamRequest:
    """Request object for client streaming RPC."""
    service_name: str
    method_name: str
    metadata: dict[str, str]
    messages: List[bytes]  # All client messages collected as list

async def handle_client_stream(
    request: GrpcClientStreamRequest,
) -> GrpcResponse:
    """Process all messages and return single response."""
    pass
```

### Example: Batch Message Processing

```python
from typing import List
from spikard.grpc import GrpcClientStreamRequest, GrpcResponse
import messageservice_pb2  # Generated from proto
from datetime import datetime
import uuid


async def handle_batch_create(
    request: GrpcClientStreamRequest,
) -> GrpcResponse:
    """
    Handle client streaming: receive multiple messages, send single response.

    Pattern: Collect all input messages, process together, return aggregated response.
    """
    messages = request.messages
    metadata = request.metadata

    # Validate authorization
    auth_token = metadata.get('authorization')
    if not auth_token:
        raise PermissionError('Authentication required')

    # Step 1: Deserialize all input messages
    items = []
    for i, msg in enumerate(messages):
        try:
            item = messageservice_pb2.Item()
            item.ParseFromString(msg)
            items.append(item)
        except Exception as err:
            raise ValueError(f'Failed to decode message {i}: {str(err)}')

    # Validate all items before processing
    for item in items:
        if not item.name or not item.value:
            raise ValueError('Each item must have name and value')

    # Step 2: Process all items atomically
    success_count = 0
    total_value = 0

    try:
        # Simulate database transaction
        async with database_transaction() as tx:
            for item in items:
                # Simulate creating item in database
                await tx.items.create(
                    name=item.name,
                    value=item.value,
                )
                success_count += 1
                total_value += item.value
    except Exception as err:
        raise RuntimeError(f'Transaction failed: {str(err)}')

    # Step 3: Build aggregate response
    response = messageservice_pb2.BatchCreateResponse()
    response.success_count = success_count
    response.total_value = total_value
    response.batch_id = str(uuid.uuid4())
    response.timestamp = datetime.utcnow().isoformat()

    # Step 4: Serialize and return
    return GrpcResponse(
        payload=response.SerializeToString(),
        metadata={
            'x-batch-id': response.batch_id,
            'x-count': str(success_count),
        },
    )


async def database_transaction():
    """Context manager for database transactions."""
    # Placeholder for actual database transaction logic
    class Transaction:
        class Items:
            async def create(self, name: str, value: int):
                pass

        def __init__(self):
            self.items = self.Items()

        async def __aenter__(self):
            return self

        async def __aexit__(self, exc_type, exc_val, exc_tb):
            pass

    return Transaction()
```

## Bidirectional Streaming Handler

Bidirectional streaming RPC allows the client to send multiple messages and the server to send multiple messages back.

### Handler Signature

```python
from typing import List
from spikard.grpc import GrpcBidiStreamRequest, GrpcBidiStreamResponse

class GrpcBidiStreamRequest:
    """Request object for bidirectional streaming RPC."""
    service_name: str
    method_name: str
    metadata: dict[str, str]
    messages: List[bytes]  # All input messages collected as list

class GrpcBidiStreamResponse:
    """Response object for bidirectional streaming RPC."""
    messages: List[bytes]  # Array of response messages
    metadata: dict[str, str]

async def handle_bidi_stream(
    request: GrpcBidiStreamRequest,
) -> GrpcBidiStreamResponse:
    """Process input messages and generate output messages."""
    pass
```

### Example: Message Transformation Pipeline

```python
from typing import List
from spikard.grpc import GrpcBidiStreamRequest, GrpcBidiStreamResponse
import transformservice_pb2  # Generated from proto
from datetime import datetime


async def handle_transform_stream(
    request: GrpcBidiStreamRequest,
) -> GrpcBidiStreamResponse:
    """
    Handle bidirectional streaming: receive multiple messages, send multiple messages.

    Pattern: Collect all input messages, process, generate output messages, return array.
    """
    messages = request.messages
    metadata = request.metadata

    # Step 1: Deserialize all input messages
    input_documents = []
    for index, msg in enumerate(messages):
        try:
            document = transformservice_pb2.Document()
            document.ParseFromString(msg)
            input_documents.append({
                'index': index,
                'document': document,
            })
        except Exception as err:
            raise ValueError(
                f'Failed to decode message {index}: {str(err)}'
            )

    # Step 2: Process each document and generate response
    output_messages: List[bytes] = []

    for item in input_documents:
        index = item['index']
        document = item['document']

        try:
            # Transform the document
            transformed = await transform_document(document)

            # Build response message
            result = transformservice_pb2.TransformResult()
            result.original_id = document.id
            result.transformed_content = transformed['content']
            result.transformed_at = datetime.utcnow().isoformat()
            result.status = 'SUCCESS' if transformed['success'] else 'PARTIAL'
            result.metadata['original_size'] = str(len(document.content))
            result.metadata['transformed_size'] = str(len(transformed['content']))

            # Serialize response message
            output_messages.append(result.SerializeToString())
        except Exception as err:
            # Create error response for this document
            error_result = transformservice_pb2.TransformResult()
            error_result.original_id = document.id
            error_result.status = 'ERROR'
            error_result.error_message = str(err)

            output_messages.append(error_result.SerializeToString())

    # Step 3: Return all response messages
    return GrpcBidiStreamResponse(
        messages=output_messages,
        metadata={
            'x-processed-count': str(len(output_messages)),
            'x-timestamp': datetime.utcnow().isoformat(),
        },
    )


async def transform_document(doc) -> dict:
    """Simulate document transformation."""
    # Simulate async transformation work
    return {
        'content': doc.content.upper(),
        'success': True,
    }
```

## Advanced Example: Filtering and Aggregation

### Bidirectional Stream with Filtering

```python
from typing import List
from spikard.grpc import GrpcBidiStreamRequest, GrpcBidiStreamResponse
import recordservice_pb2  # Generated from proto


async def handle_filter_stream(
    request: GrpcBidiStreamRequest,
) -> GrpcBidiStreamResponse:
    """
    Filter records and apply transformations in bidirectional stream.

    Pattern: Filter input based on metadata criteria, transform, return filtered output.
    """
    messages = request.messages
    metadata = request.metadata

    # Parse filter criteria from metadata
    filter_type = metadata.get('x-filter-type', 'all')
    min_value = int(metadata.get('x-min-value', '0'))

    # Step 1: Deserialize and filter
    filtered_items = []

    for msg in messages:
        item = recordservice_pb2.Record()
        item.ParseFromString(msg)

        # Apply filter logic
        if (filter_type == 'all' or
            (filter_type == 'high-value' and item.value >= min_value)):
            filtered_items.append(item)

    # Step 2: Transform filtered items
    output_messages: List[bytes] = []

    for item in filtered_items:
        response = recordservice_pb2.ProcessedRecord()
        response.id = item.id
        response.original_value = item.value
        response.processed_value = item.value * 1.1  # Apply multiplier
        response.filtered = False

        output_messages.append(response.SerializeToString())

    # Step 3: Return response with statistics
    return GrpcBidiStreamResponse(
        messages=output_messages,
        metadata={
            'x-input-count': str(len(messages)),
            'x-output-count': str(len(output_messages)),
            'x-filtered-count': str(len(messages) - len(output_messages)),
        },
    )
```

## Key Patterns

### Message Collection
- All client messages are collected in a single `messages` list
- Messages are provided as `bytes` objects (protobuf serialized)
- No streaming iteration needed - full list is provided
- Order of messages is preserved

### Processing Strategy
1. **Collect**: All input messages received as list
2. **Validate**: Check all messages before processing
3. **Transform**: Process and generate output
4. **Serialize**: Encode response messages as bytes
5. **Return**: List of response message bytes

### Error Handling
- Raise Python exceptions with appropriate types:
  - `ValueError` maps to `INVALID_ARGUMENT` status
  - `PermissionError` maps to `PERMISSION_DENIED` status
  - `NotImplementedError` maps to `UNIMPLEMENTED` status
  - `RuntimeError` or other exceptions map to `INTERNAL` status
- Errors in message deserialization should use `ValueError`
- Processing errors should use `RuntimeError` or domain-specific exceptions
- Per-message errors can be included in response messages (with ERROR status)

### Metadata
- Client streaming: Metadata passed in request, can be included in response
- Bidirectional streaming: Metadata passed in request, can be included in response
- Use metadata for non-payload information (timestamps, counts, filters)
- Access with `metadata.get(key)` and provide defaults

## Limits and Constraints

- **MAX_STREAM_MESSAGES**: 10,000 messages per stream
- **Resource Exhaustion**: Streams exceeding limit raise `MemoryError`
- **Memory**: All messages collected in memory - appropriate for moderate message counts
- **Atomicity**: All messages processed together (transaction-like semantics)

## Testing Client Streaming

```python
import pytest
from typing import List
from spikard.grpc import GrpcClientStreamRequest, GrpcResponse
from batch_handler import handle_batch_create
import messageservice_pb2 as pb


@pytest.mark.asyncio
async def test_batch_create_success():
    """Test processing batch of items."""
    # Create multiple input messages
    items = [
        pb.Item(name='Item 1', value=100),
        pb.Item(name='Item 2', value=200),
        pb.Item(name='Item 3', value=300),
    ]

    messages = [item.SerializeToString() for item in items]

    # Create request
    request = GrpcClientStreamRequest(
        service_name='myservice.MyService',
        method_name='BatchCreate',
        metadata={'authorization': 'Bearer token'},
        messages=messages,
    )

    # Call handler
    response = await handle_batch_create(request)

    # Verify response
    result = pb.BatchCreateResponse()
    result.ParseFromString(response.payload)

    assert result.success_count == 3
    assert result.total_value == 600
    assert 'x-batch-id' in response.metadata
    assert response.metadata['x-count'] == '3'


@pytest.mark.asyncio
async def test_batch_create_missing_authorization():
    """Test batch create without authorization."""
    items = [pb.Item(name='Item 1', value=100)]
    messages = [item.SerializeToString() for item in items]

    request = GrpcClientStreamRequest(
        service_name='myservice.MyService',
        method_name='BatchCreate',
        metadata={},  # No authorization
        messages=messages,
    )

    # Should raise PermissionError
    with pytest.raises(PermissionError, match='Authentication required'):
        await handle_batch_create(request)


@pytest.mark.asyncio
async def test_batch_create_invalid_item():
    """Test batch create with invalid item (missing required fields)."""
    # Item with missing value
    items = [pb.Item(name='Item 1')]  # No value
    messages = [item.SerializeToString() for item in items]

    request = GrpcClientStreamRequest(
        service_name='myservice.MyService',
        method_name='BatchCreate',
        metadata={'authorization': 'Bearer token'},
        messages=messages,
    )

    # Should raise ValueError
    with pytest.raises(ValueError, match='must have name and value'):
        await handle_batch_create(request)


@pytest.mark.asyncio
async def test_batch_create_malformed_message():
    """Test batch create with malformed protobuf."""
    messages = [b'invalid protobuf data']

    request = GrpcClientStreamRequest(
        service_name='myservice.MyService',
        method_name='BatchCreate',
        metadata={'authorization': 'Bearer token'},
        messages=messages,
    )

    # Should raise ValueError for decode error
    with pytest.raises(ValueError, match='Failed to decode message'):
        await handle_batch_create(request)
```

## Testing Bidirectional Streaming

```python
import pytest
from typing import List
from spikard.grpc import GrpcBidiStreamRequest, GrpcBidiStreamResponse
from transform_handler import handle_transform_stream
import transformservice_pb2 as pb


@pytest.mark.asyncio
async def test_transform_stream_success():
    """Test transforming multiple documents."""
    # Create input messages
    documents = [
        pb.Document(id=1, content='hello world'),
        pb.Document(id=2, content='goodbye world'),
    ]

    messages = [doc.SerializeToString() for doc in documents]

    # Create request
    request = GrpcBidiStreamRequest(
        service_name='myservice.MyService',
        method_name='TransformStream',
        metadata={},
        messages=messages,
    )

    # Call handler
    response = await handle_transform_stream(request)

    # Verify multiple response messages
    assert len(response.messages) == 2
    assert 'x-processed-count' in response.metadata
    assert response.metadata['x-processed-count'] == '2'

    # Verify each response
    for msg in response.messages:
        result = pb.TransformResult()
        result.ParseFromString(msg)
        assert result.status == 'SUCCESS'
        assert result.transformed_content  # Should have transformed content
        assert 'HELLO' in result.transformed_content or 'GOODBYE' in result.transformed_content


@pytest.mark.asyncio
async def test_transform_stream_partial_failure():
    """Test transform stream with some valid and some invalid documents."""
    documents = [
        pb.Document(id=1, content='valid content'),
        pb.Document(id=2, content='another valid'),
    ]

    messages = [doc.SerializeToString() for doc in documents]

    request = GrpcBidiStreamRequest(
        service_name='myservice.MyService',
        method_name='TransformStream',
        metadata={},
        messages=messages,
    )

    response = await handle_transform_stream(request)

    # All documents should get responses (some may be errors)
    assert len(response.messages) == 2

    # Check that we get both SUCCESS and possibly ERROR responses
    statuses = []
    for msg in response.messages:
        result = pb.TransformResult()
        result.ParseFromString(msg)
        statuses.append(result.status)

    assert 'SUCCESS' in statuses or 'ERROR' in statuses


@pytest.mark.asyncio
async def test_filter_stream_high_value():
    """Test filtering records by minimum value."""
    from filter_handler import handle_filter_stream

    records = [
        pb.Record(id=1, value=50),
        pb.Record(id=2, value=150),
        pb.Record(id=3, value=250),
    ]

    messages = [rec.SerializeToString() for rec in records]

    request = GrpcBidiStreamRequest(
        service_name='myservice.MyService',
        method_name='FilterStream',
        metadata={
            'x-filter-type': 'high-value',
            'x-min-value': '100',
        },
        messages=messages,
    )

    response = await handle_filter_stream(request)

    # Only records with value >= 100 should be returned
    assert len(response.messages) == 2
    assert response.metadata['x-output-count'] == '2'
    assert response.metadata['x-filtered-count'] == '1'

    # Verify output values
    for msg in response.messages:
        result = pb.ProcessedRecord()
        result.ParseFromString(msg)
        assert result.original_value >= 100


@pytest.mark.asyncio
async def test_filter_stream_all_records():
    """Test returning all records without filtering."""
    from filter_handler import handle_filter_stream

    records = [
        pb.Record(id=1, value=50),
        pb.Record(id=2, value=75),
        pb.Record(id=3, value=150),
    ]

    messages = [rec.SerializeToString() for rec in records]

    request = GrpcBidiStreamRequest(
        service_name='myservice.MyService',
        method_name='FilterStream',
        metadata={'x-filter-type': 'all'},
        messages=messages,
    )

    response = await handle_filter_stream(request)

    # All records should be returned
    assert len(response.messages) == 3
    assert response.metadata['x-output-count'] == '3'
    assert response.metadata['x-filtered-count'] == '0'


@pytest.mark.asyncio
async def test_filter_stream_malformed_input():
    """Test filter stream with malformed protobuf."""
    from filter_handler import handle_filter_stream

    messages = [b'invalid protobuf']

    request = GrpcBidiStreamRequest(
        service_name='myservice.MyService',
        method_name='FilterStream',
        metadata={'x-filter-type': 'all'},
        messages=messages,
    )

    # Should raise ValueError
    with pytest.raises(ValueError, match='Failed to decode'):
        await handle_filter_stream(request)
```

## Running Tests

```bash
# Run all streaming tests
pytest test_streaming_handlers.py -v

# Run specific test
pytest test_streaming_handlers.py::test_batch_create_success -v

# Run with async support and verbose output
pytest test_streaming_handlers.py -v -k "batch" --asyncio-mode=auto
```

## Comparison with Other Patterns

| Aspect | Client Streaming | Bidirectional | Unary |
|--------|------------------|---------------|-------|
| Input | Multiple messages | Multiple messages | Single message |
| Output | Single response | Multiple messages | Single response |
| Use case | Batch operations | Stream processing | Simple requests |
| Message order | Important | Important | N/A |
| Atomicity | Full batch atomic | Per-message or batch | Single atomic |

## Common Pitfalls

### 1. Forgetting to Deserialize Messages
```python
# WRONG: Using raw bytes
for msg in messages:
    print(msg)  # prints raw bytes

# CORRECT: Deserialize protobuf
for msg in messages:
    item = messageservice_pb2.Item()
    item.ParseFromString(msg)
    print(item.name)
```

### 2. Not Handling All Message Errors
```python
# WRONG: Failing entire stream on first error
for msg in messages:
    item = messageservice_pb2.Item()
    item.ParseFromString(msg)  # May raise exception

# CORRECT: Handle per-message errors
for msg in messages:
    try:
        item = messageservice_pb2.Item()
        item.ParseFromString(msg)
    except Exception as e:
        # Generate error response for this message
        error_result = messageservice_pb2.ItemResult()
        error_result.status = 'ERROR'
        error_result.error_message = str(e)
        output_messages.append(error_result.SerializeToString())
```

### 3. Forgetting to Serialize Response Messages
```python
# WRONG: Returning protobuf objects instead of bytes
response = pb.Item(name='test')
return GrpcBidiStreamResponse(messages=[response])  # Wrong!

# CORRECT: Serialize to bytes
response = pb.Item(name='test')
return GrpcBidiStreamResponse(
    messages=[response.SerializeToString()]  # Correct
)
```

### 4. Not Using Async Functions
```python
# WRONG: Blocking operation in async handler
def handle_batch_create(request: GrpcClientStreamRequest) -> GrpcResponse:
    time.sleep(1)  # Blocks entire server!
    return response

# CORRECT: Use async/await
async def handle_batch_create(request: GrpcClientStreamRequest) -> GrpcResponse:
    await asyncio.sleep(1)  # Non-blocking
    return response
```

See the gRPC documentation for more examples.
