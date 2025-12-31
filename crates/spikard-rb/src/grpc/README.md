# Ruby gRPC FFI Bindings

This module provides Ruby bindings for Spikard's gRPC runtime using Magnus FFI.

## Overview

The gRPC binding allows Ruby code to implement gRPC service handlers that work with Spikard's Rust-based gRPC runtime. Handlers receive and return binary protobuf messages that can be serialized/deserialized using the `google-protobuf` gem.

## Architecture

### Rust Side (`handler.rs`)

- **`RubyGrpcRequest`**: Wraps incoming gRPC requests, exposes service/method names, payload, and metadata to Ruby
- **`RubyGrpcResponse`**: Wraps outgoing gRPC responses, accepts binary payload and metadata from Ruby
- **`RubyGrpcHandler`**: Implements `GrpcHandler` trait, bridges between Ruby handler code and Rust gRPC runtime

### Ruby Side (`lib/spikard/grpc.rb`)

- **`Spikard::Grpc::Request`**: Ruby class representing incoming gRPC requests
- **`Spikard::Grpc::Response`**: Ruby class for building gRPC responses
- **`Spikard::Grpc::Handler`**: Base class for implementing gRPC service handlers
- **`Spikard::Grpc::Service`**: Registry for managing handler instances

## Usage Example

```ruby
require 'spikard/grpc'
require 'user_pb'  # Generated protobuf file

class UserServiceHandler < Spikard::Grpc::Handler
  def handle_request(request)
    case request.method_name
    when 'GetUser'
      # Deserialize the protobuf request
      req = Example::GetUserRequest.decode(request.payload)

      # Process the request
      user = Example::User.new(id: req.id, name: 'John Doe', email: 'john@example.com')

      # Create and return gRPC response
      response = Spikard::Grpc::Response.new(payload: Example::User.encode(user))
      response.metadata = { 'x-user-id' => req.id.to_s }
      response

    when 'CreateUser'
      req = Example::CreateUserRequest.decode(request.payload)
      user = Example::User.new(id: 123, name: req.name, email: req.email)

      Spikard::Grpc::Response.new(payload: Example::User.encode(user))

    else
      raise "Unknown method: #{request.method_name}"
    end
  end
end

# Register the handler
service = Spikard::Grpc::Service.new
handler = UserServiceHandler.new
service.register_handler('myapp.UserService', handler)
```

## Key Features

- **Binary Protobuf Support**: Payloads are binary strings compatible with `google-protobuf` gem
- **Metadata Support**: Request and response metadata (similar to HTTP headers)
- **Error Handling**: Ruby exceptions are converted to gRPC Status errors
- **Thread Safety**: Uses RefCell for interior mutability, safe for Ruby's GVL

## Implementation Details

### Request Flow

1. Rust gRPC runtime receives request
2. `RubyGrpcHandler::call()` invoked with `GrpcRequestData`
3. Request converted to `RubyGrpcRequest` and wrapped for Ruby
4. Ruby handler's `handle_request` method called (with GVL)
5. Ruby returns `Spikard::Grpc::Response` instance
6. Response converted back to `GrpcResponseData`
7. Rust runtime sends response

### Memory Management

- `Opaque<Value>` keeps Ruby handler alive for GC
- `RefCell` allows interior mutability for Response payload/metadata
- Mark hooks ensure Ruby values aren't garbage collected

### Type Conversions

- **Service/Method Names**: Rust String ↔ Ruby String
- **Payload**: Rust `Bytes` ↔ Ruby binary String
- **Metadata**: Rust `MetadataMap` ↔ Ruby Hash<String, String>

## Testing

### Rust Tests
```bash
cargo test --package spikard-rb --lib grpc
```

### Ruby Tests
```bash
cd packages/ruby
bundle exec rspec spec/grpc_spec.rb
```

## Dependencies

### Rust
- `tonic`: gRPC framework and types
- `bytes`: Efficient byte buffer
- `magnus`: Ruby FFI

### Ruby
- `google-protobuf`: Protobuf serialization (required by applications)

## Future Enhancements

- [ ] Streaming RPC support (client/server/bidirectional)
- [ ] Custom interceptors/middleware
- [ ] Automatic protobuf code generation integration
- [ ] Performance metrics and tracing
