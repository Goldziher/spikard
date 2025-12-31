# gRPC Error Handling Guide

This guide covers comprehensive error handling in Spikard's gRPC implementation across all supported languages (Python, TypeScript, Ruby, and PHP). Learn how to throw, catch, and test errors effectively in your gRPC services.

## Table of Contents

- [Overview](#overview)
- [gRPC Status Codes](#grpc-status-codes)
- [Automatic Exception Mapping](#automatic-exception-mapping)
- [Custom Error Handling](#custom-error-handling)
- [Error Metadata and Details](#error-metadata-and-details)
- [Testing Error Cases](#testing-error-cases)
- [Best Practices](#best-practices)

## Overview

Spikard's gRPC runtime provides automatic error handling and status code mapping across the FFI boundary. When your handler throws an exception, it's automatically converted to the appropriate gRPC status code based on the exception type and language conventions.

**Key Features:**
- Automatic exception-to-status-code mapping per language
- Custom error classes for precise control
- Error metadata support
- Consistent behavior across all language bindings
- Detailed error messages preserved across FFI boundary

## gRPC Status Codes

gRPC defines 17 standard status codes. Understanding when to use each code is essential for building robust services:

| Code | Value | Use Case |
|------|-------|----------|
| `OK` | 0 | Success (not an error) |
| `CANCELLED` | 1 | Operation was cancelled (typically by the caller) |
| `UNKNOWN` | 2 | Unknown error (avoid when possible) |
| `INVALID_ARGUMENT` | 3 | Client specified an invalid argument |
| `DEADLINE_EXCEEDED` | 4 | Deadline expired before operation could complete |
| `NOT_FOUND` | 5 | Some requested entity was not found |
| `ALREADY_EXISTS` | 6 | Entity that we attempted to create already exists |
| `PERMISSION_DENIED` | 7 | Caller lacks permission for the operation |
| `RESOURCE_EXHAUSTED` | 8 | Resource has been exhausted (rate limits, quotas) |
| `FAILED_PRECONDITION` | 9 | System not in required state for operation |
| `ABORTED` | 10 | Operation was aborted (concurrency conflict) |
| `OUT_OF_RANGE` | 11 | Operation attempted past the valid range |
| `UNIMPLEMENTED` | 12 | Operation not implemented or not supported |
| `INTERNAL` | 13 | Internal server error |
| `UNAVAILABLE` | 14 | Service is currently unavailable |
| `DATA_LOSS` | 15 | Unrecoverable data loss or corruption |
| `UNAUTHENTICATED` | 16 | Request lacks valid authentication credentials |

## Automatic Exception Mapping

Spikard automatically maps common language exceptions to appropriate gRPC status codes. This allows you to use idiomatic error handling in each language without worrying about gRPC-specific details.

### Python

Python exceptions are mapped to gRPC status codes based on the exception type hierarchy:

```python
from spikard import GrpcRequest, GrpcResponse

class UserService:
    async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
        if request.method_name == "GetUser":
            user_id = int.from_bytes(request.payload[:4], 'little')

            # ValueError → INVALID_ARGUMENT
            if user_id <= 0:
                raise ValueError("User ID must be positive")

            # PermissionError → PERMISSION_DENIED
            if not self.has_permission(user_id):
                raise PermissionError("Access denied for this user")

            # NotImplementedError → UNIMPLEMENTED
            if request.method_name == "AdvancedFeature":
                raise NotImplementedError("Feature not yet available")

            # TimeoutError → DEADLINE_EXCEEDED
            if self.is_slow_request():
                raise TimeoutError("Request processing timed out")

            # FileNotFoundError/KeyError → NOT_FOUND
            user = self.db.get(user_id)
            if user is None:
                raise FileNotFoundError(f"User {user_id} not found")

            # Generic Exception → INTERNAL
            # (Any other exception type defaults to INTERNAL)

            return GrpcResponse(payload=user.serialize())
```

**Python Exception Mapping:**
- `ValueError` → `INVALID_ARGUMENT`
- `PermissionError` → `PERMISSION_DENIED`
- `NotImplementedError` → `UNIMPLEMENTED`
- `TimeoutError` → `DEADLINE_EXCEEDED`
- `FileNotFoundError`, `KeyError` → `NOT_FOUND`
- All other exceptions → `INTERNAL`

### TypeScript

TypeScript uses the `GrpcError` class for explicit status code control. Standard `Error` objects map to `INTERNAL`:

```typescript
import { GrpcHandler, GrpcRequest, GrpcResponse, GrpcError, GrpcStatusCode } from 'spikard';

class UserServiceHandler implements GrpcHandler {
  async handleRequest(request: GrpcRequest): Promise<GrpcResponse> {
    if (request.methodName === 'GetUser') {
      const userId = request.payload.readUInt32LE(0);

      // INVALID_ARGUMENT
      if (userId <= 0) {
        throw new GrpcError(
          GrpcStatusCode.INVALID_ARGUMENT,
          'User ID must be positive'
        );
      }

      // PERMISSION_DENIED
      if (!this.hasPermission(userId)) {
        throw new GrpcError(
          GrpcStatusCode.PERMISSION_DENIED,
          'Access denied for this user'
        );
      }

      // NOT_FOUND
      const user = await this.db.get(userId);
      if (!user) {
        throw new GrpcError(
          GrpcStatusCode.NOT_FOUND,
          `User ${userId} not found`
        );
      }

      // UNIMPLEMENTED
      if (request.methodName === 'AdvancedFeature') {
        throw new GrpcError(
          GrpcStatusCode.UNIMPLEMENTED,
          'Feature not yet available'
        );
      }

      // Regular Error → INTERNAL
      // throw new Error('Something went wrong');

      return {
        payload: Buffer.from(user.serialize())
      };
    }

    throw new GrpcError(
      GrpcStatusCode.UNIMPLEMENTED,
      `Method ${request.methodName} not implemented`
    );
  }
}
```

**TypeScript Error Mapping:**
- `GrpcError` with specific code → Specified status code
- Standard `Error` → `INTERNAL`

### Ruby

Ruby uses a custom exception hierarchy under `Spikard::Grpc::Error`:

```ruby
class UserServiceHandler < Spikard::Grpc::Handler
  def handle_request(request)
    case request.method_name
    when 'GetUser'
      user_id = request.payload.unpack1('L<')

      # InvalidArgumentError → INVALID_ARGUMENT
      if user_id <= 0
        raise Spikard::Grpc::InvalidArgumentError, 'User ID must be positive'
      end

      # PermissionDeniedError → PERMISSION_DENIED
      unless has_permission?(user_id)
        raise Spikard::Grpc::PermissionDeniedError, 'Access denied for this user'
      end

      # NotFoundError → NOT_FOUND
      user = db.get(user_id)
      if user.nil?
        raise Spikard::Grpc::NotFoundError, "User #{user_id} not found"
      end

      # UnimplementedError → UNIMPLEMENTED
      raise Spikard::Grpc::UnimplementedError, 'Feature not yet available'

      # StandardError → INTERNAL (any other error)
      # raise StandardError, 'Something went wrong'

      Spikard::Grpc::Response.new(payload: user.serialize)
    else
      raise Spikard::Grpc::UnimplementedError, "Method #{request.method_name} not implemented"
    end
  end
end
```

**Ruby Exception Classes:**
- `Spikard::Grpc::InvalidArgumentError` → `INVALID_ARGUMENT`
- `Spikard::Grpc::PermissionDeniedError` → `PERMISSION_DENIED`
- `Spikard::Grpc::NotFoundError` → `NOT_FOUND`
- `Spikard::Grpc::AlreadyExistsError` → `ALREADY_EXISTS`
- `Spikard::Grpc::UnimplementedError` → `UNIMPLEMENTED`
- `Spikard::Grpc::UnauthenticatedError` → `UNAUTHENTICATED`
- `Spikard::Grpc::ResourceExhaustedError` → `RESOURCE_EXHAUSTED`
- `Spikard::Grpc::FailedPreconditionError` → `FAILED_PRECONDITION`
- `Spikard::Grpc::AbortedError` → `ABORTED`
- `Spikard::Grpc::OutOfRangeError` → `OUT_OF_RANGE`
- `Spikard::Grpc::DeadlineExceededError` → `DEADLINE_EXCEEDED`
- `Spikard::Grpc::CancelledError` → `CANCELLED`
- `Spikard::Grpc::InternalError` → `INTERNAL`
- `Spikard::Grpc::UnavailableError` → `UNAVAILABLE`
- `Spikard::Grpc::DataLossError` → `DATA_LOSS`
- Any other `StandardError` → `INTERNAL`

### PHP

PHP uses a similar exception hierarchy:

```php
use Spikard\Grpc\Handler;
use Spikard\Grpc\Request;
use Spikard\Grpc\Response;
use Spikard\Grpc\Exceptions\InvalidArgumentException;
use Spikard\Grpc\Exceptions\PermissionDeniedException;
use Spikard\Grpc\Exceptions\NotFoundException;
use Spikard\Grpc\Exceptions\UnimplementedException;

class UserServiceHandler extends Handler
{
    public function handleRequest(Request $request): Response
    {
        if ($request->methodName === 'GetUser') {
            $userId = unpack('V', $request->payload)[1];

            // InvalidArgumentException → INVALID_ARGUMENT
            if ($userId <= 0) {
                throw new InvalidArgumentException('User ID must be positive');
            }

            // PermissionDeniedException → PERMISSION_DENIED
            if (!$this->hasPermission($userId)) {
                throw new PermissionDeniedException('Access denied for this user');
            }

            // NotFoundException → NOT_FOUND
            $user = $this->db->get($userId);
            if ($user === null) {
                throw new NotFoundException("User {$userId} not found");
            }

            // UnimplementedException → UNIMPLEMENTED
            // throw new UnimplementedException('Feature not yet available');

            // Generic Exception → INTERNAL
            // throw new \Exception('Something went wrong');

            return new Response($user->serialize());
        }

        throw new UnimplementedException(
            "Method {$request->methodName} not implemented"
        );
    }
}
```

**PHP Exception Mapping:**
- `Spikard\Grpc\Exceptions\InvalidArgumentException` → `INVALID_ARGUMENT`
- `Spikard\Grpc\Exceptions\PermissionDeniedException` → `PERMISSION_DENIED`
- `Spikard\Grpc\Exceptions\NotFoundException` → `NOT_FOUND`
- `Spikard\Grpc\Exceptions\AlreadyExistsException` → `ALREADY_EXISTS`
- `Spikard\Grpc\Exceptions\UnimplementedException` → `UNIMPLEMENTED`
- Generic `\Exception` → `INTERNAL`

## Custom Error Handling

### Advanced Error Details

All languages support passing detailed error messages that are preserved across the FFI boundary:

**Python:**
```python
# Multi-line error messages
raise ValueError(
    "Validation failed:\n"
    "- Password must be at least 12 characters\n"
    "- Password must contain special characters\n"
    f"Current length: {len(password)}"
)

# Unicode characters in error messages
raise PermissionError("Usuario no autorizado: acceso denegado")
```

**TypeScript:**
```typescript
// Detailed error messages
throw new GrpcError(
  GrpcStatusCode.INVALID_ARGUMENT,
  `Validation failed:
   - Field 'email' must be a valid email address
   - Field 'age' must be between 0 and 120
   Current value: ${email}`
);

// Error with context
throw new GrpcError(
  GrpcStatusCode.RESOURCE_EXHAUSTED,
  `Rate limit exceeded. Maximum 100 requests per minute. ` +
  `Retry after: ${retryAfter}s`
);
```

**Ruby:**
```ruby
# Detailed error with interpolation
raise Spikard::Grpc::InvalidArgumentError,
  "Validation failed:\n" \
  "- Email '#{email}' is invalid\n" \
  "- Age must be between 0 and 120, got #{age}"

# Error with calculation
raise Spikard::Grpc::ResourceExhaustedError,
  "API rate limit exceeded. " \
  "Maximum #{limit} requests per minute. " \
  "Current: #{count}. Retry after #{retry_after}s"
```

**PHP:**
```php
// Multi-line detailed error
throw new InvalidArgumentException(
    "Validation failed:\n" .
    "- Email '{$email}' is invalid\n" .
    "- Age must be between 0 and 120, got {$age}"
);

// Error with context
throw new ResourceExhaustedException(
    "Rate limit exceeded. Maximum {$limit} requests per minute. " .
    "Current: {$count}. Retry after {$retryAfter}s"
);
```

### Conditional Error Handling

Use appropriate status codes based on error conditions:

**Python:**
```python
async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
    if request.method_name == "UpdateUser":
        user_id = int.from_bytes(request.payload[:4], 'little')

        # Check resource exists first
        user = self.db.get(user_id)
        if user is None:
            raise FileNotFoundError(f"User {user_id} not found")

        # Check permissions
        if not self.can_update(user):
            raise PermissionError("Cannot update this user")

        # Check system state
        if self.db.is_read_only():
            raise RuntimeError("Database is in read-only mode")  # → INTERNAL

        # Validate input
        if len(request.payload) < 100:
            raise ValueError("Update payload too small")

        # Update user...
        return GrpcResponse(payload=updated_user.serialize())
```

**TypeScript:**
```typescript
async handleRequest(request: GrpcRequest): Promise<GrpcResponse> {
  if (request.methodName === 'CreateUser') {
    const { email } = this.parseRequest(request.payload);

    // Check for duplicates
    const existing = await this.db.findByEmail(email);
    if (existing) {
      throw new GrpcError(
        GrpcStatusCode.ALREADY_EXISTS,
        `User with email '${email}' already exists`
      );
    }

    // Validate email format
    if (!this.isValidEmail(email)) {
      throw new GrpcError(
        GrpcStatusCode.INVALID_ARGUMENT,
        `Invalid email format: '${email}'`
      );
    }

    // Check rate limits
    if (await this.isRateLimited()) {
      throw new GrpcError(
        GrpcStatusCode.RESOURCE_EXHAUSTED,
        'Too many requests. Please try again later.'
      );
    }

    // Create user...
    return { payload: Buffer.from(newUser.serialize()) };
  }
}
```

## Error Metadata and Details

While the current implementation focuses on error messages, you can include contextual information in the error message itself:

**Python:**
```python
# Include field-specific validation errors
def validate_user_data(data):
    errors = []

    if not data.get('email'):
        errors.append("Field 'email' is required")
    elif '@' not in data['email']:
        errors.append("Field 'email' must be a valid email address")

    if not data.get('age'):
        errors.append("Field 'age' is required")
    elif data['age'] < 0 or data['age'] > 120:
        errors.append(f"Field 'age' must be between 0 and 120, got {data['age']}")

    if errors:
        raise ValueError("Validation failed:\n" + "\n".join(errors))
```

**TypeScript:**
```typescript
// Include operation context in error messages
class OrderService implements GrpcHandler {
  async handleRequest(request: GrpcRequest): Promise<GrpcResponse> {
    if (request.methodName === 'PlaceOrder') {
      const order = this.parseOrder(request.payload);

      // Check inventory
      const available = await this.inventory.check(order.productId);
      if (available < order.quantity) {
        throw new GrpcError(
          GrpcStatusCode.FAILED_PRECONDITION,
          `Insufficient inventory for product ${order.productId}. ` +
          `Requested: ${order.quantity}, Available: ${available}`
        );
      }

      // Process order...
    }
  }
}
```

## Testing Error Cases

### Unit Tests

Test your error handling thoroughly across all status codes:

**Python (pytest):**
```python
import pytest
from spikard import GrpcRequest, GrpcResponse

@pytest.mark.asyncio
async def test_invalid_argument_error():
    """Test that ValueError maps to INVALID_ARGUMENT."""
    handler = UserServiceHandler()
    request = GrpcRequest(
        service_name="test.UserService",
        method_name="GetUser",
        payload=b"\xff\xff\xff\xff",  # Invalid user ID
    )

    with pytest.raises(ValueError, match="User ID must be positive"):
        await handler.handle_request(request)

@pytest.mark.asyncio
async def test_permission_denied_error():
    """Test that PermissionError maps to PERMISSION_DENIED."""
    handler = UserServiceHandler()
    request = GrpcRequest(
        service_name="test.UserService",
        method_name="DeleteUser",
        payload=b"\x01\x00\x00\x00",
    )

    with pytest.raises(PermissionError, match="Access denied"):
        await handler.handle_request(request)

@pytest.mark.asyncio
async def test_not_found_error():
    """Test that FileNotFoundError maps to NOT_FOUND."""
    handler = UserServiceHandler()
    request = GrpcRequest(
        service_name="test.UserService",
        method_name="GetUser",
        payload=b"\x63\x00\x00\x00",  # Non-existent user ID
    )

    with pytest.raises(FileNotFoundError, match="User 99 not found"):
        await handler.handle_request(request)

@pytest.mark.asyncio
async def test_error_message_preservation():
    """Test that error messages with unicode are preserved."""
    handler = UserServiceHandler()
    request = GrpcRequest(
        service_name="test.UserService",
        method_name="GetUser",
        payload=b"",
    )

    with pytest.raises(ValueError, match="Error with unicode: 你好 🚀"):
        await handler.handle_request(request)
```

**TypeScript (Vitest):**
```typescript
import { describe, it, expect } from 'vitest';
import { GrpcError, GrpcStatusCode } from 'spikard';

describe('UserServiceHandler', () => {
  it('should throw INVALID_ARGUMENT for invalid input', async () => {
    const handler = new UserServiceHandler();
    const request: GrpcRequest = {
      serviceName: 'test.UserService',
      methodName: 'GetUser',
      payload: Buffer.from([0xff, 0xff, 0xff, 0xff]),
      metadata: {},
    };

    await expect(handler.handleRequest(request)).rejects.toThrow(GrpcError);
    await expect(handler.handleRequest(request)).rejects.toMatchObject({
      code: GrpcStatusCode.INVALID_ARGUMENT,
      message: expect.stringContaining('User ID must be positive'),
    });
  });

  it('should throw NOT_FOUND for missing resource', async () => {
    const handler = new UserServiceHandler();
    const request: GrpcRequest = {
      serviceName: 'test.UserService',
      methodName: 'GetUser',
      payload: Buffer.from([99, 0, 0, 0]),
      metadata: {},
    };

    await expect(handler.handleRequest(request)).rejects.toThrow(GrpcError);
    await expect(handler.handleRequest(request)).rejects.toMatchObject({
      code: GrpcStatusCode.NOT_FOUND,
      message: expect.stringContaining('User 99 not found'),
    });
  });

  it('should throw PERMISSION_DENIED for unauthorized access', async () => {
    const handler = new UserServiceHandler();
    const request: GrpcRequest = {
      serviceName: 'test.UserService',
      methodName: 'DeleteUser',
      payload: Buffer.from([1, 0, 0, 0]),
      metadata: {},
    };

    await expect(handler.handleRequest(request)).rejects.toThrow(GrpcError);
    await expect(handler.handleRequest(request)).rejects.toMatchObject({
      code: GrpcStatusCode.PERMISSION_DENIED,
      message: expect.stringContaining('Access denied'),
    });
  });
});
```

**Ruby (RSpec):**
```ruby
RSpec.describe UserServiceHandler do
  describe '#handle_request' do
    let(:handler) { described_class.new }

    it 'raises InvalidArgumentError for invalid input' do
      request = double('request',
        method_name: 'GetUser',
        payload: [0xff, 0xff, 0xff, 0xff].pack('L<')
      )

      expect { handler.handle_request(request) }
        .to raise_error(Spikard::Grpc::InvalidArgumentError, /User ID must be positive/)
    end

    it 'raises NotFoundError for missing resource' do
      request = double('request',
        method_name: 'GetUser',
        payload: [99].pack('L<')
      )

      expect { handler.handle_request(request) }
        .to raise_error(Spikard::Grpc::NotFoundError, /User 99 not found/)
    end

    it 'raises PermissionDeniedError for unauthorized access' do
      request = double('request',
        method_name: 'DeleteUser',
        payload: [1].pack('L<')
      )

      expect { handler.handle_request(request) }
        .to raise_error(Spikard::Grpc::PermissionDeniedError, /Access denied/)
    end

    it 'preserves unicode in error messages' do
      request = double('request',
        method_name: 'GetUser',
        payload: ''
      )

      expect { handler.handle_request(request) }
        .to raise_error(Spikard::Grpc::InvalidArgumentError, /你好 🚀/)
    end
  end
end
```

**PHP (PHPUnit):**
```php
use PHPUnit\Framework\TestCase;
use Spikard\Grpc\Exceptions\InvalidArgumentException;
use Spikard\Grpc\Exceptions\NotFoundException;
use Spikard\Grpc\Exceptions\PermissionDeniedException;

class UserServiceHandlerTest extends TestCase
{
    public function testInvalidArgumentError(): void
    {
        $handler = new UserServiceHandler();
        $request = $this->createRequest('GetUser', pack('V', 0xffffffff));

        $this->expectException(InvalidArgumentException::class);
        $this->expectExceptionMessage('User ID must be positive');

        $handler->handleRequest($request);
    }

    public function testNotFoundError(): void
    {
        $handler = new UserServiceHandler();
        $request = $this->createRequest('GetUser', pack('V', 99));

        $this->expectException(NotFoundException::class);
        $this->expectExceptionMessage('User 99 not found');

        $handler->handleRequest($request);
    }

    public function testPermissionDeniedError(): void
    {
        $handler = new UserServiceHandler();
        $request = $this->createRequest('DeleteUser', pack('V', 1));

        $this->expectException(PermissionDeniedException::class);
        $this->expectExceptionMessage('Access denied');

        $handler->handleRequest($request);
    }

    private function createRequest(string $method, string $payload): Request
    {
        $request = $this->createMock(Request::class);
        $request->methodName = $method;
        $request->payload = $payload;
        return $request;
    }
}
```

### Integration Tests

Test error propagation through the entire gRPC stack:

**Python:**
```python
@pytest.mark.asyncio
async def test_grpc_error_propagation():
    """Test that errors propagate correctly through gRPC runtime."""
    from spikard import GrpcService

    service = GrpcService()

    class ErrorHandler:
        async def handle_request(self, request):
            if request.method_name == "ThrowInvalidArgument":
                raise ValueError("Invalid input")
            elif request.method_name == "ThrowNotFound":
                raise FileNotFoundError("Resource not found")
            elif request.method_name == "ThrowPermissionDenied":
                raise PermissionError("Access denied")

    service.register_handler("test.ErrorService", ErrorHandler())

    # Test each error type
    request1 = GrpcRequest(
        service_name="test.ErrorService",
        method_name="ThrowInvalidArgument",
        payload=b"",
    )
    with pytest.raises(ValueError):
        await service.handle_request(request1)

    request2 = GrpcRequest(
        service_name="test.ErrorService",
        method_name="ThrowNotFound",
        payload=b"",
    )
    with pytest.raises(FileNotFoundError):
        await service.handle_request(request2)
```

## Best Practices

### 1. Choose Appropriate Status Codes

Select status codes that accurately describe the error condition:

```python
# ✅ GOOD: Use NOT_FOUND for missing resources
user = db.get(user_id)
if user is None:
    raise FileNotFoundError(f"User {user_id} not found")

# ❌ BAD: Using generic error
user = db.get(user_id)
if user is None:
    raise RuntimeError(f"User {user_id} not found")  # Maps to INTERNAL
```

```typescript
// ✅ GOOD: Use ALREADY_EXISTS for duplicate resources
const existing = await db.findByEmail(email);
if (existing) {
  throw new GrpcError(
    GrpcStatusCode.ALREADY_EXISTS,
    `User with email '${email}' already exists`
  );
}

// ❌ BAD: Using wrong status code
if (existing) {
  throw new GrpcError(
    GrpcStatusCode.INVALID_ARGUMENT,  // Wrong code
    `User with email '${email}' already exists`
  );
}
```

### 2. Provide Detailed Error Messages

Include enough context for clients to understand and fix the issue:

```python
# ✅ GOOD: Detailed, actionable error message
raise ValueError(
    f"Validation failed for field 'age': "
    f"Value must be between 0 and 120, got {age}. "
    f"Please provide a valid age."
)

# ❌ BAD: Vague error message
raise ValueError("Invalid age")
```

### 3. Use Language Idioms

Leverage language-specific error handling patterns:

```python
# ✅ GOOD: Pythonic exception handling
def get_user(user_id: int) -> User:
    if user_id <= 0:
        raise ValueError("User ID must be positive")

    user = db.get(user_id)
    if user is None:
        raise FileNotFoundError(f"User {user_id} not found")

    return user
```

```typescript
// ✅ GOOD: TypeScript with explicit types
function getUser(userId: number): User {
  if (userId <= 0) {
    throw new GrpcError(
      GrpcStatusCode.INVALID_ARGUMENT,
      'User ID must be positive'
    );
  }

  const user = db.get(userId);
  if (!user) {
    throw new GrpcError(
      GrpcStatusCode.NOT_FOUND,
      `User ${userId} not found`
    );
  }

  return user;
}
```

### 4. Validate Input Early

Check preconditions before processing:

```python
async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
    # Validate method name first
    if request.method_name not in ['GetUser', 'CreateUser', 'UpdateUser']:
        raise NotImplementedError(f"Method {request.method_name} not implemented")

    # Validate payload size
    if len(request.payload) == 0:
        raise ValueError("Request payload cannot be empty")

    if len(request.payload) > self.MAX_PAYLOAD_SIZE:
        raise ValueError(f"Payload too large: {len(request.payload)} bytes")

    # Process request...
```

### 5. Test All Error Paths

Ensure every error condition is tested:

```python
@pytest.mark.asyncio
async def test_all_error_conditions():
    """Test all error conditions for GetUser method."""
    handler = UserServiceHandler()

    # Test invalid user ID (negative)
    with pytest.raises(ValueError):
        await handler.handle_request(create_request(-1))

    # Test invalid user ID (zero)
    with pytest.raises(ValueError):
        await handler.handle_request(create_request(0))

    # Test non-existent user
    with pytest.raises(FileNotFoundError):
        await handler.handle_request(create_request(99999))

    # Test permission denied
    with pytest.raises(PermissionError):
        await handler.handle_request(create_request(1, auth=False))

    # Test empty payload
    with pytest.raises(ValueError):
        await handler.handle_request(GrpcRequest(
            service_name="test.Service",
            method_name="GetUser",
            payload=b""
        ))
```

### 6. Handle Edge Cases

Consider unusual but valid scenarios:

```python
async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
    # Handle empty payload gracefully
    if len(request.payload) == 0:
        raise ValueError("Payload cannot be empty")

    # Handle very large payloads
    if len(request.payload) > 5_000_000:  # 5MB
        raise ValueError(
            f"Payload too large: {len(request.payload)} bytes. "
            f"Maximum allowed: 5,000,000 bytes"
        )

    # Handle null bytes in binary data
    # (This is valid for binary protobuf data)
    user_id = int.from_bytes(request.payload[:4], 'little')

    # Handle unicode in error messages
    try:
        result = self.process_user(user_id)
    except Exception as e:
        # Preserve unicode characters
        raise RuntimeError(f"处理失败: {str(e)}")
```

### 7. Document Error Conditions

Document which errors your handlers can throw:

```python
class UserServiceHandler:
    """
    Handler for User service operations.

    Methods:
        GetUser: Retrieve user by ID
            Errors:
                - ValueError (INVALID_ARGUMENT): Invalid user ID
                - FileNotFoundError (NOT_FOUND): User not found
                - PermissionError (PERMISSION_DENIED): Access denied

        CreateUser: Create a new user
            Errors:
                - ValueError (INVALID_ARGUMENT): Invalid user data
                - RuntimeError (ALREADY_EXISTS): User already exists
                - PermissionError (PERMISSION_DENIED): Insufficient permissions
    """

    async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
        # Implementation...
        pass
```

```typescript
/**
 * User service handler
 *
 * Methods:
 *   GetUser: Retrieve user by ID
 *     Errors:
 *       - INVALID_ARGUMENT: Invalid user ID
 *       - NOT_FOUND: User not found
 *       - PERMISSION_DENIED: Access denied
 *
 *   CreateUser: Create a new user
 *     Errors:
 *       - INVALID_ARGUMENT: Invalid user data
 *       - ALREADY_EXISTS: User already exists
 *       - PERMISSION_DENIED: Insufficient permissions
 */
class UserServiceHandler implements GrpcHandler {
  async handleRequest(request: GrpcRequest): Promise<GrpcResponse> {
    // Implementation...
  }
}
```

### 8. Avoid Exposing Internal Details

Don't leak sensitive information in error messages:

```python
# ✅ GOOD: Generic error for authentication failures
if not self.verify_password(user, password):
    raise PermissionError("Invalid credentials")

# ❌ BAD: Reveals which part failed
if not user:
    raise FileNotFoundError("User not found")
if not self.verify_password(user, password):
    raise PermissionError("Invalid password")
```

```typescript
// ✅ GOOD: Don't expose internal paths
throw new GrpcError(
  GrpcStatusCode.INTERNAL,
  'Database error occurred'
);

// ❌ BAD: Exposes internal details
throw new GrpcError(
  GrpcStatusCode.INTERNAL,
  `Database connection failed: host=db.internal.company.com:5432`
);
```

## Summary

Spikard's gRPC error handling provides:

1. **Automatic mapping** - Language exceptions automatically map to gRPC status codes
2. **Custom control** - Use language-specific error classes for precise status codes
3. **Message preservation** - Error messages (including unicode) preserved across FFI
4. **Consistent behavior** - Same error handling patterns across all languages
5. **Easy testing** - Straightforward unit and integration testing

By following these patterns and best practices, you can build robust gRPC services with comprehensive error handling that provides clear, actionable feedback to clients.

## Related Documentation

- [ADR 0011: gRPC FFI Bindings Strategy](/docs/adr/0011-grpc-ffi-bindings-strategy.md)
- [gRPC Status Codes Reference](https://grpc.io/docs/guides/status-codes/)
- [Testing Guide](/docs/guides/testing.md)
