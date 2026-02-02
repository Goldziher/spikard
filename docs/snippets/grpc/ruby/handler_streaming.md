# Ruby gRPC Streaming Handlers

Complete Ruby implementation examples for client streaming and bidirectional streaming gRPC handlers in Spikard.

## Client Streaming Handler

Client streaming RPC allows a client to send multiple messages and then the server responds with a single message.

### Handler Signature

```ruby
class ClientStreamRequest
  attr_reader :service_name, :method_name, :metadata, :messages

  def initialize(service_name:, method_name:, metadata:, messages:)
    @service_name = service_name
    @method_name = method_name
    @metadata = metadata
    @messages = messages  # All client messages collected as array
  end
end

def handle_client_stream(request)
  # Process all messages and return single response
end
```

### Example: Batch Message Processing

```ruby
require 'spikard/grpc'
require 'messageservice_pb'

class BatchCreateHandler < Spikard::Grpc::Handler
  def initialize(database)
    @database = database
  end

  def handle_request(request)
    case request.method_name
    when 'BatchCreate'
      handle_batch_create(request)
    else
      raise "Unknown method: #{request.method_name}"
    end
  end

  private

  def handle_batch_create(request)
    # Validate authorization
    auth_token = request.metadata['authorization']
    unless auth_token
      raise SecurityError, 'Authentication required'
    end

    # Step 1: Deserialize all input messages
    items = request.messages.map.with_index do |msg, index|
      begin
        Messageservice::Item.decode(msg)
      rescue Google::Protobuf::ParseError => e
        raise ArgumentError, "Failed to decode message #{index}: #{e.message}"
      end
    end

    # Validate all items before processing
    items.each do |item|
      unless item.name && !item.name.empty? && item.value > 0
        raise ArgumentError, 'Each item must have name and positive value'
      end
    end

    # Step 2: Process all items atomically
    success_count = 0
    total_value = 0
    batch_id = SecureRandom.uuid

    begin
      # Begin transaction
      @database.transaction do
        items.each do |item|
          created = @database[:items].insert(
            name: item.name,
            value: item.value,
            batch_id: batch_id
          )
          success_count += 1
          total_value += item.value
        end
      end
    rescue StandardError => e
      raise StandardError, "Transaction failed: #{e.message}"
    end

    # Step 3: Build aggregate response
    response_proto = Messageservice::BatchCreateResponse.new(
      success_count: success_count,
      total_value: total_value,
      batch_id: batch_id,
      timestamp: Time.now.utc.iso8601
    )

    # Step 4: Serialize and return
    response = Spikard::Grpc::Response.new(
      payload: Messageservice::BatchCreateResponse.encode(response_proto)
    )
    response.metadata = {
      'x-batch-id' => batch_id,
      'x-count' => success_count.to_s
    }
    response
  rescue ArgumentError => e
    Spikard::Grpc::Response.error(e.message, 'INVALID_ARGUMENT')
  rescue SecurityError => e
    Spikard::Grpc::Response.error(e.message, 'UNAUTHENTICATED')
  rescue StandardError => e
    Spikard::Grpc::Response.error("Internal error: #{e.message}", 'INTERNAL')
  end
end
```

## Bidirectional Streaming Handler

Bidirectional streaming RPC allows the client to send multiple messages and the server to send multiple messages back.

### Handler Signature

```ruby
class BidiStreamRequest
  attr_reader :service_name, :method_name, :metadata, :messages

  def initialize(service_name:, method_name:, metadata:, messages:)
    @service_name = service_name
    @method_name = method_name
    @metadata = metadata
    @messages = messages  # All input messages collected as array
  end
end

class BidiStreamResponse
  attr_accessor :messages, :metadata

  def initialize(messages:, metadata: {})
    @messages = messages
    @metadata = metadata
  end
end

def handle_bidi_stream(request)
  # Process input messages and generate output messages
  # Return hash with :messages and :metadata keys
end
```

### Example: Message Transformation Pipeline

```ruby
require 'spikard/grpc'
require 'transformservice_pb'

class TransformStreamHandler < Spikard::Grpc::Handler
  def initialize(transformer_service)
    @transformer_service = transformer_service
  end

  def handle_request(request)
    case request.method_name
    when 'TransformStream'
      handle_transform_stream(request)
    else
      raise "Unknown method: #{request.method_name}"
    end
  end

  private

  def handle_transform_stream(request)
    # Step 1: Deserialize all input messages
    input_documents = request.messages.map.with_index do |msg, index|
      begin
        {
          index: index,
          document: Transformservice::Document.decode(msg)
        }
      rescue Google::Protobuf::ParseError => e
        raise ArgumentError, "Failed to decode message #{index}: #{e.message}"
      end
    end

    # Step 2: Process each document and generate response
    output_messages = []

    input_documents.each do |input|
      index = input[:index]
      document = input[:document]

      begin
        # Transform the document
        transformed = transform_document(document)

        # Build response message
        result = Transformservice::TransformResult.new(
          original_id: document.id,
          transformed_content: transformed[:content],
          transformed_at: Time.now.utc.iso8601,
          status: transformed[:success] ? 'SUCCESS' : 'PARTIAL',
          metadata: {
            'original_size' => document.content.length.to_s,
            'transformed_size' => transformed[:content].length.to_s
          }
        )

        # Serialize response message
        encoded = Transformservice::TransformResult.encode(result)
        output_messages << encoded
      rescue StandardError => e
        # Create error response for this document
        error_result = Transformservice::TransformResult.new(
          original_id: document.id,
          status: 'ERROR',
          error_message: e.message
        )

        encoded = Transformservice::TransformResult.encode(error_result)
        output_messages << encoded
      end
    end

    # Step 3: Return all response messages
    {
      messages: output_messages,
      metadata: {
        'x-processed-count' => output_messages.length.to_s,
        'x-timestamp' => Time.now.utc.iso8601
      }
    }
  rescue ArgumentError => e
    Spikard::Grpc::Response.error(e.message, 'INVALID_ARGUMENT')
  rescue StandardError => e
    Spikard::Grpc::Response.error("Internal error: #{e.message}", 'INTERNAL')
  end

  def transform_document(doc)
    # Simulate document transformation
    {
      content: doc.content.upcase,
      success: true
    }
  end
end
```

## Advanced Example: Filtering and Aggregation

### Bidirectional Stream with Filtering

```ruby
class FilterStreamHandler < Spikard::Grpc::Handler
  def initialize(record_service)
    @record_service = record_service
  end

  def handle_request(request)
    case request.method_name
    when 'FilterStream'
      handle_filter_stream(request)
    else
      raise "Unknown method: #{request.method_name}"
    end
  end

  private

  def handle_filter_stream(request)
    # Parse filter criteria from metadata
    filter_type = request.metadata['x-filter-type'] || 'all'
    min_value = (request.metadata['x-min-value'] || '0').to_i

    # Step 1: Deserialize and filter
    filtered_items = request.messages.filter_map do |msg|
      item = Recordservice::Record.decode(msg)

      # Apply filter logic
      if filter_type == 'all' || (filter_type == 'high-value' && item.value >= min_value)
        item
      end
    end

    # Step 2: Transform filtered items
    output_messages = filtered_items.map do |item|
      response = Recordservice::ProcessedRecord.new(
        id: item.id,
        original_value: item.value,
        processed_value: item.value * 1.1,  # Apply multiplier
        filtered: false
      )

      Recordservice::ProcessedRecord.encode(response)
    end

    # Step 3: Return response with statistics
    {
      messages: output_messages,
      metadata: {
        'x-input-count' => request.messages.length.to_s,
        'x-output-count' => output_messages.length.to_s,
        'x-filtered-count' => (request.messages.length - output_messages.length).to_s
      }
    }
  rescue ArgumentError => e
    Spikard::Grpc::Response.error(e.message, 'INVALID_ARGUMENT')
  rescue StandardError => e
    Spikard::Grpc::Response.error("Internal error: #{e.message}", 'INTERNAL')
  end
end
```

## Key Patterns

### Message Collection
- All client messages are collected in a single `messages` array via `request.messages`
- Messages are binary strings (serialized protobuf)
- No streaming iteration needed - full array is provided
- Use `.each.with_index` or `.map.with_index` to process with index

### Processing Strategy
1. **Collect**: All input messages received as array via `request.messages`
2. **Validate**: Check all messages before processing with `.each` or `.filter_map`
3. **Transform**: Process and generate output using blocks
4. **Serialize**: Encode response messages with protobuf `.encode()` method
5. **Return**: Hash with `:messages` and `:metadata` keys for bidi; `Response` object for client stream

### Error Handling
- Use Ruby exceptions (`ArgumentError`, `StandardError`, custom errors) to signal failures
- Catch exceptions in `handle_request` and convert to `Spikard::Grpc::Response.error(message, status_code)`
- Errors in message deserialization should use `INVALID_ARGUMENT` status
- Processing errors should use `INTERNAL` or domain-specific codes
- Per-message errors can be included in response messages (with ERROR status)

### Metadata
- Client streaming: Metadata passed in request, can be included in response
- Bidirectional streaming: Metadata passed in request, can be included in response
- Use metadata for non-payload information (timestamps, counts, filters)
- Access request metadata via `request.metadata[key]` returning `String | nil`

## Limits and Constraints

- **MAX_STREAM_MESSAGES**: 10,000 messages per stream
- **Resource Exhaustion**: Streams exceeding limit return `RESOURCE_EXHAUSTED` error
- **Memory**: All messages collected in memory - appropriate for moderate message counts
- **Atomicity**: All messages processed together (transaction-like semantics)

## Testing Client Streaming

```ruby
require 'rspec'
require 'spikard/grpc'
require 'messageservice_pb'
require_relative '../lib/batch_create_handler'

RSpec.describe BatchCreateHandler do
  let(:database) { instance_double('Database') }
  let(:handler) { described_class.new(database) }

  describe '#handle_request' do
    context 'BatchCreate' do
      it 'should process batch of items' do
        # Create multiple input messages
        items = [
          Messageservice::Item.new(name: 'Item 1', value: 100),
          Messageservice::Item.new(name: 'Item 2', value: 200),
          Messageservice::Item.new(name: 'Item 3', value: 300)
        ]

        messages = items.map { |item| Messageservice::Item.encode(item) }

        # Setup database mock
        allow(database).to receive(:transaction).and_yield(database)
        allow(database).to receive(:[]).and_return(
          instance_double('Table', insert: true)
        )

        # Create request
        request = Spikard::Grpc::Request.new(
          service_name: 'messageservice.MessageService',
          method_name: 'BatchCreate',
          metadata: { 'authorization' => 'Bearer token' },
          payload: ''  # Not used in client streaming
        )
        request.instance_variable_set(:@messages, messages)

        # Call handler
        response = handler.handle_request(request)

        # Verify response
        result = Messageservice::BatchCreateResponse.decode(response.payload)
        expect(result.success_count).to eq(3)
        expect(result.total_value).to eq(600)
        expect(response.metadata['x-count']).to eq('3')
      end

      it 'returns error when authorization is missing' do
        items = [
          Messageservice::Item.new(name: 'Item 1', value: 100)
        ]
        messages = items.map { |item| Messageservice::Item.encode(item) }

        request = Spikard::Grpc::Request.new(
          service_name: 'messageservice.MessageService',
          method_name: 'BatchCreate',
          metadata: {},  # Missing authorization
          payload: ''
        )
        request.instance_variable_set(:@messages, messages)

        expect { handler.handle_request(request) }.to raise_error(
          SecurityError, /Authentication required/
        )
      end

      it 'validates all items have values' do
        items = [
          Messageservice::Item.new(name: 'Item 1', value: 100),
          Messageservice::Item.new(name: 'Item 2', value: 0)  # Invalid
        ]
        messages = items.map { |item| Messageservice::Item.encode(item) }

        request = Spikard::Grpc::Request.new(
          service_name: 'messageservice.MessageService',
          method_name: 'BatchCreate',
          metadata: { 'authorization' => 'Bearer token' },
          payload: ''
        )
        request.instance_variable_set(:@messages, messages)

        expect { handler.handle_request(request) }.to raise_error(
          ArgumentError, /positive value/
        )
      end
    end
  end
end
```

## Testing Bidirectional Streaming

```ruby
require 'rspec'
require 'spikard/grpc'
require 'transformservice_pb'
require_relative '../lib/transform_stream_handler'

RSpec.describe TransformStreamHandler do
  let(:transformer_service) { instance_double('TransformerService') }
  let(:handler) { described_class.new(transformer_service) }

  describe '#handle_request' do
    context 'TransformStream' do
      it 'should transform and return multiple messages' do
        # Create input messages
        documents = [
          Transformservice::Document.new(id: '1', content: 'hello'),
          Transformservice::Document.new(id: '2', content: 'world')
        ]

        messages = documents.map { |doc| Transformservice::Document.encode(doc) }

        # Create request
        request = Spikard::Grpc::Request.new(
          service_name: 'transformservice.TransformService',
          method_name: 'TransformStream',
          metadata: { 'authorization' => 'Bearer token' },
          payload: ''
        )
        request.instance_variable_set(:@messages, messages)

        # Call handler
        response = handler.handle_request(request)

        # Verify response
        expect(response).to include(:messages, :metadata)
        expect(response[:messages]).to have_length(2)

        response[:messages].each do |msg|
          result = Transformservice::TransformResult.decode(msg)
          expect(result.status).to eq('SUCCESS')
          expect(result.transformed_content).not_to be_empty
        end

        expect(response[:metadata]['x-processed-count']).to eq('2')
      end

      it 'handles per-message errors gracefully' do
        documents = [
          Transformservice::Document.new(id: '1', content: 'valid'),
          Transformservice::Document.new(id: '2', content: nil)  # Will fail
        ]

        messages = documents.map { |doc| Transformservice::Document.encode(doc) }

        request = Spikard::Grpc::Request.new(
          service_name: 'transformservice.TransformService',
          method_name: 'TransformStream',
          metadata: { 'authorization' => 'Bearer token' },
          payload: ''
        )
        request.instance_variable_set(:@messages, messages)

        # Call handler - should return response with error for second message
        response = handler.handle_request(request)

        expect(response[:messages]).to have_length(2)

        # First message should be SUCCESS
        result1 = Transformservice::TransformResult.decode(response[:messages][0])
        expect(result1.status).to eq('SUCCESS')

        # Second message should be ERROR
        result2 = Transformservice::TransformResult.decode(response[:messages][1])
        expect(result2.status).to eq('ERROR')
        expect(result2.error_message).not_to be_empty
      end

      it 'filters items based on metadata' do
        records = [
          Recordservice::Record.new(id: '1', value: 50),
          Recordservice::Record.new(id: '2', value: 100),
          Recordservice::Record.new(id: '3', value: 150)
        ]

        messages = records.map { |rec| Recordservice::Record.encode(rec) }

        request = Spikard::Grpc::Request.new(
          service_name: 'recordservice.RecordService',
          method_name: 'FilterStream',
          metadata: {
            'x-filter-type' => 'high-value',
            'x-min-value' => '100'
          },
          payload: ''
        )
        request.instance_variable_set(:@messages, messages)

        # Call handler
        response = handler.handle_request(request)

        # Only 2 items should pass filter (value >= 100)
        expect(response[:messages]).to have_length(2)
        expect(response[:metadata]['x-output-count']).to eq('2')
        expect(response[:metadata]['x-filtered-count']).to eq('1')
      end
    end
  end
end
```

## Comparison with Other Patterns

| Aspect | Client Streaming | Bidirectional | Unary |
|--------|------------------|---------------|-------|
| Input | Multiple messages | Multiple messages | Single message |
| Output | Single response | Multiple messages | Single response |
| Use case | Batch operations | Stream processing | Simple requests |
| Message order | Important | Important | N/A |
| Atomicity | Full batch atomic | Per-message or batch | Single atomic |

## Ruby-Specific Patterns

### Using Enumerables Effectively

```ruby
# Filter and transform in one pass
output_messages = request.messages.filter_map do |msg|
  item = decode_message(msg)
  next unless item.valid?  # Filters out invalid items
  encode_response(item)    # Transforms to response
end

# Using each_with_index for detailed indexing
request.messages.each_with_index do |msg, idx|
  process_with_position(msg, idx)
end

# Collect with error handling
results = request.messages.map do |msg|
  decode_and_process(msg)
rescue StandardError => e
  handle_error(e)
end
```

### Exception Handling in Streaming

```ruby
# Catch and convert exceptions to gRPC responses
def handle_request(request)
  case request.method_name
  when 'BatchCreate'
    handle_batch_create(request)
  else
    raise "Unknown method"
  end
rescue ArgumentError => e
  Spikard::Grpc::Response.error(e.message, 'INVALID_ARGUMENT')
rescue SecurityError => e
  Spikard::Grpc::Response.error(e.message, 'UNAUTHENTICATED')
rescue StandardError => e
  Spikard::Grpc::Response.error("Internal error: #{e.message}", 'INTERNAL')
end
```

### Transaction Patterns with Blocks

```ruby
begin
  @database.transaction do
    items.each do |item|
      @database[:items].insert(item.to_h)
    end
  end
rescue StandardError => e
  raise StandardError, "Transaction failed: #{e.message}"
end
```

See the gRPC documentation for more examples.
