# Spikard Troubleshooting Guide

This comprehensive guide covers common issues encountered when working with Spikard across OpenAPI, GraphQL, and Protobuf/gRPC protocols. Solutions are organized by problem category with clear error examples, root causes, and prevention strategies.

## Table of Contents

1. [Schema Validation Errors](#schema-validation-errors)
2. [Code Generation Failures](#code-generation-failures)
3. [Quality Validation Failures](#quality-validation-failures)
4. [Integration Issues](#integration-issues)
5. [Runtime Errors](#runtime-errors)
6. [gRPC-Specific Issues](#grpc-specific-issues)

---

## Schema Validation Errors

### Problem 1: OpenAPI - Missing Required Field in Request Body

**Error Message:**
```json
{
  "type": "https://spikard.dev/errors/validation-error",
  "title": "Request Validation Failed",
  "status": 422,
  "detail": "1 validation error in request",
  "errors": [
    {
      "type": "missing",
      "loc": ["body", "username"],
      "msg": "Field required",
      "input": {}
    }
  ]
}
```

**Root Cause:**
The client is submitting a request missing a field marked as `required` in the OpenAPI schema. Spikard validates all incoming requests against the schema before they reach your handler.

**Solution:**

1. Check your OpenAPI schema definition:
```yaml
components:
  schemas:
    CreateUserRequest:
      type: object
      required:
        - username  # This field is mandatory
        - email
      properties:
        username:
          type: string
        email:
          type: string
```

2. Ensure the client includes all required fields:
```typescript
// TypeScript client
const response = await fetch('/users', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({
    username: 'johndoe',  // Include required field
    email: 'john@example.com'
  })
});
```

**Prevention Tips:**
- Use OpenAPI documentation generators to share API contracts with clients
- Enable request validation in development to catch issues early
- Consider providing default values for optional fields in your schema
- Use schema examples in your OpenAPI spec to document expected inputs

---

### Problem 2: GraphQL - Invalid Query Syntax

**Error Message:**
```json
{
  "errors": [
    {
      "message": "GraphQL parse error: Syntax error near line 3",
      "extensions": {
        "code": "GRAPHQL_PARSE_ERROR",
        "status": 400,
        "type": "https://spikard.dev/errors/graphql-parse-error"
      }
    }
  ]
}
```

**Root Cause:**
The GraphQL query contains syntax errors such as mismatched braces, missing colons, or invalid field names.

**Solution:**

Check for common syntax issues:

```graphql
# INCORRECT - missing closing brace
query GetUser {
  user(id: "123") {
    id
    name
    email

# CORRECT
query GetUser {
  user(id: "123") {
    id
    name
    email
  }
}
```

**Prevention Tips:**
- Use GraphQL IDE tools (GraphiQL, GraphQL Playground) with syntax highlighting
- Enable query validation in your GraphQL client
- Use typed GraphQL clients (like Apollo Client with code generation)
- Add unit tests for your GraphQL queries

---

### Problem 3: Protobuf - Message Type Not Found

**Error Message:**
```
Error: Message type 'example.v1.UserRequest' not found in descriptor pool
```

**Root Cause:**
The protobuf message type referenced in your code doesn't exist in the compiled descriptor set, usually due to incorrect imports or package names.

**Solution:**

1. Verify your `.proto` file package and message definitions:
```protobuf
syntax = "proto3";

package example.v1;  // Package must match

message UserRequest {
  string id = 1;
  string name = 2;
}
```

2. Ensure you're importing the correct generated code:
```python
# Python
from example.v1 import user_pb2

request = user_pb2.UserRequest(id="123", name="John")
```

```typescript
// TypeScript
import { UserRequest } from './gen/example/v1/user';

const request = UserRequest.create({ id: "123", name: "John" });
```

**Prevention Tips:**
- Use consistent package naming across all `.proto` files
- Regenerate protobuf code after schema changes
- Verify import paths match your project structure
- Use `protoc --descriptor_set_out` to debug descriptor issues

---

### Problem 4: String Length Constraint Violation

**Error Message:**
```json
{
  "type": "https://spikard.dev/errors/validation-error",
  "status": 422,
  "errors": [
    {
      "type": "string_too_short",
      "loc": ["body", "password"],
      "msg": "String should have at least 8 characters",
      "ctx": { "min_length": 8 }
    }
  ]
}
```

**Root Cause:**
Input string doesn't meet the minimum or maximum length constraints defined in the schema.

**Solution:**

1. Check schema constraints:
```yaml
# OpenAPI
password:
  type: string
  minLength: 8
  maxLength: 128
```

2. Validate input before submission:
```python
# Python
def validate_password(password: str) -> bool:
    if len(password) < 8:
        raise ValueError("Password must be at least 8 characters")
    if len(password) > 128:
        raise ValueError("Password must not exceed 128 characters")
    return True
```

**Prevention Tips:**
- Display constraint information in UI forms
- Implement client-side validation matching server rules
- Provide clear error messages indicating the constraints
- Use appropriate input types (e.g., password fields with length indicators)

---

### Problem 5: Enum Value Not in Allowed List

**Error Message:**
```json
{
  "type": "enum",
  "loc": ["body", "status"],
  "msg": "Input should be 'active', 'inactive' or 'pending'",
  "ctx": { "expected": "'active', 'inactive' or 'pending'" }
}
```

**Root Cause:**
The provided value doesn't match any of the allowed enum values defined in the schema.

**Solution:**

1. Review allowed enum values:
```yaml
# OpenAPI
status:
  type: string
  enum:
    - active
    - inactive
    - pending
```

2. Use type-safe enums in code:
```typescript
// TypeScript
enum UserStatus {
  Active = 'active',
  Inactive = 'inactive',
  Pending = 'pending'
}

const request = {
  status: UserStatus.Active  // Type-safe
};
```

```python
# Python
from enum import Enum

class UserStatus(str, Enum):
    ACTIVE = "active"
    INACTIVE = "inactive"
    PENDING = "pending"

request = {"status": UserStatus.ACTIVE.value}
```

**Prevention Tips:**
- Generate client code from schemas to get type-safe enums
- Use autocomplete-enabled dropdowns in UI
- Document enum values in API documentation
- Consider using constants instead of magic strings

---

## Code Generation Failures

### Problem 6: TypeScript - Module Resolution Failure

**Error Message:**
```
Error: Cannot find module './gen/models' or its corresponding type declarations.
```

**Root Cause:**
Generated TypeScript files aren't in the expected location or the module resolution configuration is incorrect.

**Solution:**

1. Verify code generation succeeded:
```bash
spikard generate openapi \
  --schema ./api.yaml \
  --language typescript \
  --output ./src/gen
```

2. Check `tsconfig.json` paths configuration:
```json
{
  "compilerOptions": {
    "baseUrl": ".",
    "paths": {
      "@gen/*": ["src/gen/*"]
    }
  }
}
```

3. Verify generated files exist:
```bash
ls -la src/gen/
# Should show index.ts, models.ts, etc.
```

**Prevention Tips:**
- Add generated code directories to `.gitignore`
- Use consistent output paths across the team
- Document code generation steps in README
- Add code generation to CI/CD pipeline

---

### Problem 7: Ruby - Syntax Error in Generated Code

**Error Message:**
```
SyntaxError: unexpected token at 'def handle_request('
```

**Root Cause:**
The code generator produced invalid Ruby syntax, often due to schema edge cases or special characters in field names.

**Solution:**

1. Check for reserved keywords or special characters:
```yaml
# PROBLEMATIC
class:  # 'class' is reserved in Ruby
  type: string

# BETTER
user_class:
  type: string
```

2. Regenerate code with proper escaping:
```bash
spikard generate openapi \
  --schema ./api.yaml \
  --language ruby \
  --output ./lib/gen \
  --validate
```

3. If the issue persists, report the schema pattern to the Spikard team.

**Prevention Tips:**
- Avoid using language-reserved keywords in schema field names
- Use `snake_case` for field names consistently
- Run syntax validation immediately after generation
- Enable strict validation mode during generation

---

### Problem 8: Python - Import Error for Generated Protobuf

**Error Message:**
```
ImportError: cannot import name 'UserServiceStub' from 'gen.user_pb2_grpc'
```

**Root Cause:**
The gRPC Python code wasn't generated or is out of sync with the `.proto` files.

**Solution:**

1. Regenerate gRPC Python code:
```bash
python -m grpc_tools.protoc \
  --proto_path=. \
  --python_out=./gen \
  --grpc_python_out=./gen \
  user.proto
```

2. Verify both `*_pb2.py` and `*_pb2_grpc.py` files exist:
```bash
ls gen/
# Should show: user_pb2.py and user_pb2_grpc.py
```

3. Check import paths:
```python
# Use relative imports if in package
from .gen import user_pb2, user_pb2_grpc

# Or absolute imports
from gen import user_pb2, user_pb2_grpc
```

**Prevention Tips:**
- Add protobuf compilation to build scripts
- Use consistent proto file organization
- Version control `.proto` files, not generated code
- Document gRPC setup in project README

---

## Quality Validation Failures

### Problem 9: Python - mypy Type Error

**Error Message:**
```
error: Incompatible return value type (got "dict[str, Any]", expected "UserResponse")
```

**Root Cause:**
The handler returns a plain dictionary instead of the typed response model, causing mypy strict mode to fail.

**Solution:**

Use proper type annotations and return types:

```python
# INCORRECT
def get_user(user_id: str):
    return {"id": user_id, "name": "John"}

# CORRECT
from msgspec import Struct

class UserResponse(Struct):
    id: str
    name: str

def get_user(user_id: str) -> UserResponse:
    return UserResponse(id=user_id, name="John")
```

**Prevention Tips:**
- Enable mypy in pre-commit hooks
- Use `--strict` mode during development
- Generate type stubs from OpenAPI schemas
- Run type checking in CI/CD

---

### Problem 10: TypeScript - tsc Compilation Error

**Error Message:**
```
error TS2322: Type 'string | undefined' is not assignable to type 'string'.
```

**Root Cause:**
Nullable fields in the schema aren't handled with proper type guards in TypeScript.

**Solution:**

Add proper null checks:

```typescript
// INCORRECT
function processUser(user: User) {
  const name: string = user.name;  // user.name might be undefined
  console.log(name.toUpperCase());
}

// CORRECT
function processUser(user: User) {
  if (user.name) {
    const name: string = user.name;
    console.log(name.toUpperCase());
  } else {
    console.log("Name not provided");
  }
}

// OR use optional chaining
function processUser(user: User) {
  console.log(user.name?.toUpperCase() ?? "Name not provided");
}
```

**Prevention Tips:**
- Enable `strictNullChecks` in `tsconfig.json`
- Use non-nullable types in schemas where appropriate
- Apply consistent null-handling patterns
- Use TypeScript 4.4+ for improved type narrowing

---

### Problem 11: Ruby - Steep Type Mismatch

**Error Message:**
```
Type mismatch: expected `String` but got `String | nil`
```

**Root Cause:**
Optional fields in the schema produce nullable types in Ruby, requiring explicit nil handling.

**Solution:**

Use proper type signatures and nil checks:

```ruby
# Add Steep type signature
# @type var user: User
# @type var name: String

user = get_user(id)

# INCORRECT
name = user.name
puts name.upcase

# CORRECT
if user.name
  name = user.name
  puts name.upcase
else
  puts "Name not available"
end
```

**Prevention Tips:**
- Use RBS type definitions
- Enable Steep in CI pipeline
- Use Sorbet for runtime type checking
- Mark required fields explicitly in schemas

---

### Problem 12: PHP - PHPStan Level Max Error

**Error Message:**
```
Property App\Models\User::$email has no type specified.
```

**Root Cause:**
Generated PHP code lacks proper type hints, failing PHPStan's strict analysis.

**Solution:**

Ensure schema includes type information:

```yaml
# OpenAPI schema
User:
  type: object
  properties:
    id:
      type: string
      format: uuid
    email:
      type: string
      format: email
    age:
      type: integer
      minimum: 0
```

Generated code will include types:

```php
<?php

class User
{
    public function __construct(
        public string $id,
        public string $email,
        public int $age,
    ) {}
}
```

**Prevention Tips:**
- Use PHP 8.1+ for property types
- Specify formats in OpenAPI schema
- Run PHPStan with `--level=max` locally
- Keep generated code in separate namespace

---

### Problem 13: Rust - Clippy Warnings as Errors

**Error Message:**
```
error: this function has too many arguments (8/7)
  --> src/handlers.rs:42:1
```

**Root Cause:**
Generated Rust code violates Clippy's complexity limits, often due to handlers with many parameters.

**Solution:**

Refactor to use request objects:

```rust
// INSTEAD OF
fn handle_user(
    id: String,
    name: String,
    email: String,
    age: i32,
    // ... 8 parameters
) -> Result<Response> {
    // handler
}

// USE
#[derive(Deserialize)]
struct UserRequest {
    id: String,
    name: String,
    email: String,
    age: i32,
}

fn handle_user(req: UserRequest) -> Result<Response> {
    // handler
}
```

**Prevention Tips:**
- Group related parameters into structs
- Use the builder pattern for complex requests
- Run `cargo clippy` before committing
- Configure Clippy to allow specific warnings if needed

---

## Integration Issues

### Problem 14: CORS Preflight Request Failing

**Error Message (Browser Console):**
```
Access to fetch at 'https://api.example.com/users' from origin 'http://localhost:3000'
has been blocked by CORS policy: Response to preflight request doesn't pass access
control check: No 'Access-Control-Allow-Origin' header is present.
```

**Root Cause:**
The server doesn't respond to OPTIONS requests or doesn't include required CORS headers.

**Solution:**

Configure CORS middleware:

```python
# Python
from spikard import App

app = App()
app.config.cors.enabled = True
app.config.cors.allowed_origins = ["http://localhost:3000"]
app.config.cors.allowed_methods = ["GET", "POST", "PUT", "DELETE"]
app.config.cors.allowed_headers = ["Content-Type", "Authorization"]
```

```typescript
// TypeScript
import { App } from 'spikard';

const app = new App({
  cors: {
    enabled: true,
    allowedOrigins: ['http://localhost:3000'],
    allowedMethods: ['GET', 'POST', 'PUT', 'DELETE'],
    allowedHeaders: ['Content-Type', 'Authorization'],
  }
});
```

```ruby
# Ruby
require 'spikard'

app = Spikard::App.new
app.config.cors.enabled = true
app.config.cors.allowed_origins = ['http://localhost:3000']
app.config.cors.allowed_methods = ['GET', 'POST', 'PUT', 'DELETE']
```

**Prevention Tips:**
- Enable CORS in development mode
- Use environment-specific CORS configurations
- Test with actual frontend URLs, not just curl
- Log CORS headers in development

---

### Problem 15: Authentication Token Validation Failure

**Error Message:**
```json
{
  "type": "https://spikard.dev/errors/authentication-error",
  "title": "Unauthorized",
  "status": 401,
  "detail": "Invalid or expired authentication token"
}
```

**Root Cause:**
The JWT token is expired, malformed, or signed with wrong secret.

**Solution:**

1. Verify token structure and expiration:
```python
# Python
import jwt
from datetime import datetime

def verify_token(token: str) -> dict:
    try:
        payload = jwt.decode(
            token,
            key="your-secret-key",
            algorithms=["HS256"]
        )

        # Check expiration
        if payload.get('exp', 0) < datetime.now().timestamp():
            raise ValueError("Token expired")

        return payload
    except jwt.InvalidTokenError as e:
        raise ValueError(f"Invalid token: {e}")
```

2. Include valid token in requests:
```typescript
// TypeScript
const response = await fetch('/api/users', {
  headers: {
    'Authorization': `Bearer ${validToken}`
  }
});
```

**Prevention Tips:**
- Use standard JWT libraries (don't roll your own)
- Set reasonable token expiration times
- Implement token refresh mechanisms
- Log authentication failures for debugging
- Use environment variables for secrets

---

### Problem 16: Request Body Too Large

**Error Message:**
```json
{
  "type": "https://spikard.dev/errors/bad-request",
  "title": "Request Entity Too Large",
  "status": 413,
  "detail": "Request body exceeds maximum size of 1MB"
}
```

**Root Cause:**
The request payload exceeds the configured body size limit.

**Solution:**

1. Increase body size limit if appropriate:
```python
# Python
app.config.max_body_size = 10 * 1024 * 1024  # 10MB
```

```typescript
// TypeScript
const app = new App({
  maxBodySize: 10 * 1024 * 1024  // 10MB
});
```

2. Or use streaming for large uploads:
```python
# Python
from spikard import UploadFile

async def upload_file(file: UploadFile):
    # Stream directly to storage
    await storage.save(file.filename, file.stream)
```

**Prevention Tips:**
- Set appropriate limits for your use case
- Use streaming for large file uploads
- Implement chunked uploads for very large files
- Document size limits in API documentation

---

## Runtime Errors

### Problem 17: Handler Timeout

**Error Message:**
```json
{
  "type": "https://spikard.dev/errors/internal-server-error",
  "title": "Gateway Timeout",
  "status": 504,
  "detail": "Handler execution exceeded timeout of 30s"
}
```

**Root Cause:**
The handler takes longer than the configured timeout to execute, usually due to slow database queries or external API calls.

**Solution:**

1. Optimize slow operations:
```python
# SLOW - sequential database calls
async def get_user_data(user_id: str):
    user = await db.get_user(user_id)
    posts = await db.get_posts(user_id)
    comments = await db.get_comments(user_id)
    return {"user": user, "posts": posts, "comments": comments}

# FAST - parallel database calls
import asyncio

async def get_user_data(user_id: str):
    user, posts, comments = await asyncio.gather(
        db.get_user(user_id),
        db.get_posts(user_id),
        db.get_comments(user_id)
    )
    return {"user": user, "posts": posts, "comments": comments}
```

2. Or increase timeout for specific endpoints:
```python
app.config.request_timeout = 60  # 60 seconds
```

3. Use background tasks for long operations:
```python
from spikard import BackgroundTask

async def process_upload(file_id: str):
    # Long-running processing
    await heavy_processing(file_id)

async def upload_handler(file: UploadFile):
    file_id = await save_file(file)

    # Return immediately, process in background
    return {
        "id": file_id,
        "status": "processing"
    }, BackgroundTask(process_upload, file_id)
```

**Prevention Tips:**
- Profile handler performance
- Add database query indexes
- Use caching for frequently accessed data
- Implement request queuing for expensive operations
- Set realistic timeout values

---

### Problem 18: Dependency Injection Failure

**Error Message:**
```
RuntimeError: No provider registered for type 'Database' in dependency injection container
```

**Root Cause:**
The DI container doesn't have a provider for a required dependency.

**Solution:**

Register providers before starting the app:

```python
# Python
from spikard import App, Provide

class Database:
    def __init__(self, url: str):
        self.url = url

app = App()

# Register provider
@app.provide(Database)
def create_db():
    return Database(url="postgresql://localhost/mydb")

# Use in handler
async def get_user(user_id: str, db: Database = Provide()):
    return await db.query(f"SELECT * FROM users WHERE id = {user_id}")
```

```typescript
// TypeScript
import { App, Provide } from 'spikard';

class Database {
  constructor(private url: string) {}
}

const app = new App();

// Register provider
app.provide(Database, () => {
  return new Database('postgresql://localhost/mydb');
});

// Use in handler
app.get('/users/:id', async (req, { db }: { db: Database }) => {
  return await db.query(`SELECT * FROM users WHERE id = ${req.params.id}`);
});
```

**Prevention Tips:**
- Register all providers at application startup
- Use dependency interfaces for testability
- Validate DI configuration before deployment
- Document required dependencies

---

### Problem 19: Streaming Response Connection Closed

**Error Message (Logs):**
```
Error: Connection closed while streaming response after sending 1024 bytes
```

**Root Cause:**
The client disconnected before the streaming response completed, or the server encountered an error mid-stream.

**Solution:**

Handle connection errors gracefully:

```python
# Python
from spikard import StreamingResponse
import asyncio

async def stream_data():
    try:
        for i in range(100):
            yield f"data: {i}\n\n"
            await asyncio.sleep(0.1)
    except asyncio.CancelledError:
        # Client disconnected
        print("Client disconnected, cleaning up...")
        await cleanup()
        raise
    except Exception as e:
        # Other error during streaming
        print(f"Streaming error: {e}")
        yield f"event: error\ndata: {str(e)}\n\n"

async def sse_handler():
    return StreamingResponse(
        stream_data(),
        media_type="text/event-stream"
    )
```

**Prevention Tips:**
- Implement proper error handling in generators
- Add heartbeat messages to detect dead connections
- Set appropriate timeout for streaming responses
- Test with flaky network conditions

---

### Problem 20: Memory Leak with WebSocket Connections

**Error Message:**
```
MemoryError: Unable to allocate memory for WebSocket buffers.
Currently tracking 10000 active connections.
```

**Root Cause:**
WebSocket connections aren't properly closed, leading to accumulation of stale connections and memory exhaustion.

**Solution:**

Implement connection lifecycle management:

```python
# Python
from spikard import WebSocket
import asyncio

active_connections = set()

async def websocket_handler(ws: WebSocket):
    await ws.accept()
    active_connections.add(ws)

    try:
        while True:
            data = await ws.receive_text()
            await ws.send_text(f"Echo: {data}")
    except Exception as e:
        print(f"WebSocket error: {e}")
    finally:
        # Always cleanup
        active_connections.discard(ws)
        await ws.close()

# Periodic cleanup
async def cleanup_stale_connections():
    while True:
        await asyncio.sleep(60)
        for ws in list(active_connections):
            if ws.client_state.disconnected:
                active_connections.discard(ws)
```

```typescript
// TypeScript
import { WebSocket } from 'spikard';

const activeConnections = new Set<WebSocket>();

async function websocketHandler(ws: WebSocket) {
  activeConnections.add(ws);

  try {
    for await (const message of ws) {
      await ws.send(`Echo: ${message}`);
    }
  } catch (error) {
    console.error('WebSocket error:', error);
  } finally {
    activeConnections.delete(ws);
  }
}
```

**Prevention Tips:**
- Set connection limits
- Implement heartbeat/ping-pong to detect dead connections
- Add connection timeout
- Monitor active connection count
- Close connections on server shutdown

---

## gRPC-Specific Issues

### Problem 21: Protobuf Serialization Error

**Error Message:**
```
grpc._channel._InactiveRpcError: <_InactiveRpcError of RPC that terminated with:
    status = StatusCode.INVALID_ARGUMENT
    details = "Failed to parse request proto: invalid wire type"
```

**Root Cause:**
The client is sending a protobuf message that doesn't match the expected schema, often due to version mismatch between client and server.

**Solution:**

1. Ensure client and server use same `.proto` files:
```bash
# Regenerate both client and server code
protoc --python_out=. --grpc_python_out=. user.proto
```

2. Verify message creation:
```python
# Python
import user_pb2

# INCORRECT - missing required fields
request = user_pb2.GetUserRequest()

# CORRECT
request = user_pb2.GetUserRequest(id="user_123")

# Or use dict syntax
request = user_pb2.GetUserRequest(**{"id": "user_123"})
```

3. Check for breaking changes in `.proto`:
```protobuf
// OLD
message UserRequest {
  string id = 1;
}

// NEW - breaking change (field number changed)
message UserRequest {
  string user_id = 1;  // Changed field name AND number
}

// CORRECT - non-breaking change (field renamed but number kept)
message UserRequest {
  string user_id = 1;  // Renamed but kept field number
}
```

**Prevention Tips:**
- Version your `.proto` files
- Never change field numbers
- Use reserved fields for deleted fields
- Test serialization round-trips
- Keep `.proto` files in a shared repository

---

### Problem 22: gRPC Status Code UNAVAILABLE

**Error Message:**
```
grpc._channel._InactiveRpcError: <_InactiveRpcError of RPC that terminated with:
    status = StatusCode.UNAVAILABLE
    details = "failed to connect to all addresses"
```

**Root Cause:**
The gRPC server is not running, the address is incorrect, or there's a network connectivity issue.

**Solution:**

1. Verify server is running:
```bash
# Check if server is listening
lsof -i :50051

# Or use netstat
netstat -an | grep 50051
```

2. Check connection configuration:
```python
# Python
import grpc
from user_pb2_grpc import UserServiceStub

# INCORRECT - wrong port
channel = grpc.insecure_channel('localhost:50050')

# CORRECT
channel = grpc.insecure_channel('localhost:50051')
stub = UserServiceStub(channel)
```

3. Add retry logic:
```python
# Python
import grpc
from grpc import StatusCode

def call_with_retry(stub, request, max_retries=3):
    for attempt in range(max_retries):
        try:
            return stub.GetUser(request)
        except grpc.RpcError as e:
            if e.code() == StatusCode.UNAVAILABLE:
                if attempt < max_retries - 1:
                    time.sleep(2 ** attempt)  # Exponential backoff
                    continue
            raise
```

**Prevention Tips:**
- Use health check endpoints
- Implement connection pooling
- Add circuit breakers for resilience
- Monitor gRPC server metrics
- Use service discovery in production

---

### Problem 23: gRPC Metadata Not Propagated

**Error Message:**
```ruby
NoMethodError: undefined method 'get' for nil:NilClass
  # When trying to access metadata
```

**Root Cause:**
gRPC metadata (headers) aren't being properly set or accessed on the client or server side.

**Solution:**

1. Set metadata on client:
```python
# Python
import grpc

channel = grpc.insecure_channel('localhost:50051')
stub = UserServiceStub(channel)

# Add metadata
metadata = [
    ('authorization', 'Bearer token123'),
    ('x-request-id', 'req_456')
]

response = stub.GetUser(request, metadata=metadata)
```

```ruby
# Ruby
require 'spikard/grpc'

request = Example::GetUserRequest.new(id: 'user_123')
metadata = {
  'authorization' => 'Bearer token123',
  'x-request-id' => 'req_456'
}

response = stub.get_user(request, metadata: metadata)
```

2. Access metadata on server:
```python
# Python
class UserService:
    def GetUser(self, request, context):
        # Access metadata
        metadata = dict(context.invocation_metadata())
        auth_token = metadata.get('authorization')

        if not auth_token:
            context.abort(grpc.StatusCode.UNAUTHENTICATED,
                         'Missing authorization header')

        return user_pb2.User(id=request.id, name="John")
```

```ruby
# Ruby
class UserServiceHandler < Spikard::Grpc::Handler
  def handle_request(request)
    auth_token = request.metadata['authorization']

    unless auth_token
      return Spikard::Grpc::Response.error(
        'Missing authorization header'
      )
    end

    # Process request
    user = Example::User.new(id: request.payload.id, name: 'John')
    Spikard::Grpc::Response.new(payload: Example::User.encode(user))
  end
end
```

**Prevention Tips:**
- Document required metadata fields
- Validate metadata early in handlers
- Use interceptors for common metadata handling
- Log metadata for debugging

---

### Problem 24: gRPC Streaming Deadlock

**Error Message:**
```
Deadlock detected: client waiting for server response while server waiting for client message
```

**Root Cause:**
Bidirectional streaming has improper flow control, causing both sides to wait for each other.

**Solution:**

Implement proper streaming patterns:

```python
# Python - INCORRECT (deadlock)
def bidi_stream(request_iterator):
    responses = []
    # Trying to read all requests before sending responses
    for request in request_iterator:
        responses.append(process(request))

    # Never reaches here because client is waiting for response
    for response in responses:
        yield response

# CORRECT - interleaved reading and writing
def bidi_stream(request_iterator):
    for request in request_iterator:
        response = process(request)
        yield response  # Send immediately
```

```typescript
// TypeScript - bidirectional streaming
async function* bidiStream(
  requests: AsyncIterable<Request>
): AsyncIterable<Response> {
  for await (const request of requests) {
    const response = await processRequest(request);
    yield response;  // Stream responses as they're ready
  }
}
```

**Prevention Tips:**
- Use unary or server/client streaming when possible
- Implement timeouts for streaming calls
- Test streaming with various message patterns
- Document expected message flow

---

### Problem 25: gRPC Large Message Error

**Error Message:**
```
grpc._channel._InactiveRpcError: <_InactiveRpcError of RPC that terminated with:
    status = StatusCode.RESOURCE_EXHAUSTED
    details = "Received message larger than max (4194304 vs. 4194304)"
```

**Root Cause:**
The gRPC message exceeds the default 4MB size limit.

**Solution:**

1. Increase message size limits:
```python
# Python - Server
server = grpc.server(
    futures.ThreadPoolExecutor(max_workers=10),
    options=[
        ('grpc.max_receive_message_length', 50 * 1024 * 1024),  # 50MB
        ('grpc.max_send_message_length', 50 * 1024 * 1024),
    ]
)
```

```python
# Python - Client
channel = grpc.insecure_channel(
    'localhost:50051',
    options=[
        ('grpc.max_receive_message_length', 50 * 1024 * 1024),
        ('grpc.max_send_message_length', 50 * 1024 * 1024),
    ]
)
```

2. Or use streaming for large data:
```protobuf
// Instead of single large message
service FileService {
  rpc Upload(FileUploadRequest) returns (FileUploadResponse);
}

// Use streaming
service FileService {
  rpc Upload(stream FileChunk) returns (FileUploadResponse);
}

message FileChunk {
  bytes content = 1;
  int64 offset = 2;
}
```

**Prevention Tips:**
- Use streaming for large payloads
- Implement chunking for file transfers
- Set appropriate size limits
- Monitor message sizes in production
- Consider alternative protocols for very large data

---

## General Debugging Tips

### Enable Debug Logging

```python
# Python
import logging
logging.basicConfig(level=logging.DEBUG)

app = App()
app.config.debug = True
```

```typescript
// TypeScript
const app = new App({
  debug: true,
  logger: {
    level: 'debug'
  }
});
```

```ruby
# Ruby
require 'logger'

app = Spikard::App.new
app.config.logger = Logger.new(STDOUT, level: :debug)
```

### Validate Schemas

```bash
# OpenAPI
spikard validate openapi --schema api.yaml

# GraphQL
spikard validate graphql --schema schema.graphql

# Protobuf
protoc --descriptor_set_out=/dev/null user.proto
```

### Test Generated Code

```bash
# Python
mypy --strict gen/

# TypeScript
tsc --noEmit

# Ruby
steep check

# PHP
phpstan analyse --level=max

# Rust
cargo clippy -- -D warnings
```

### Check Version Compatibility

```bash
# Verify Spikard version
spikard --version

# Check for updates
spikard check-updates

# Validate compatibility
spikard validate-config
```

---

## Getting Help

If you encounter an issue not covered in this guide:

1. Check the [Spikard documentation](https://docs.spikard.dev)
2. Search [GitHub issues](https://github.com/spikard/spikard/issues)
3. Review [examples directory](https://github.com/spikard/spikard/tree/main/examples)
4. Ask in the [community Discord](https://discord.gg/spikard)
5. Report bugs with minimal reproduction case

When reporting issues, include:
- Spikard version (`spikard --version`)
- Language and runtime version
- Complete error message
- Minimal schema that reproduces the issue
- Steps to reproduce
