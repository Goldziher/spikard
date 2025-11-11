# Lifecycle Hooks - Quick Start Guide

## Overview

Spikard's lifecycle hooks system is inspired by Fastify, providing fine-grained control over the request/response lifecycle across all language bindings.

## Hook Points (Execution Order)

```
Request → onRequest → Route Match → preValidation → Validate → preHandler → Handler → onResponse → Response
                ↓                          ↓                         ↓              ↓
             onError ←──────────────────────────────────────────────────────────────┘
```

1. **`onRequest`** - Before routing (auth, logging, request IDs)
2. **`preValidation`** - After routing, before validation (transform data)
3. **`preHandler`** - After validation, before handler (load context)
4. **`onResponse`** - After handler (add headers, logging)
5. **`onError`** - When errors occur (format errors)

## Side-by-Side Comparison

### Basic Hook Registration

<table>
<tr>
<th>Python</th>
<th>TypeScript</th>
<th>Ruby</th>
</tr>
<tr>
<td>

```python
@app.on_request
async def log(request: Request):
    print(f"{request.method} {request.path}")
    return request
```

</td>
<td>

```typescript
app.onRequest(async (request: Request) => {
  console.log(`${request.method} ${request.path}`);
  return request;
});
```

</td>
<td>

```ruby
app.on_request do |request|
  puts "#{request.method} #{request.path}"
  request
end
```

</td>
</tr>
</table>

### Short-Circuit with Response

<table>
<tr>
<th>Python</th>
<th>TypeScript</th>
<th>Ruby</th>
</tr>
<tr>
<td>

```python
@app.pre_handler
async def auth(request: Request):
    if not request.headers.get("Authorization"):
        return Response(
            {"error": "Unauthorized"},
            status_code=401
        )
    return request
```

</td>
<td>

```typescript
app.preHandler(async (request: Request) => {
  if (!request.headers['authorization']) {
    return new Response(
      { error: 'Unauthorized' },
      { status: 401 }
    );
  }
  return request;
});
```

</td>
<td>

```ruby
app.pre_handler do |request|
  unless request.headers['Authorization']
    return Spikard::Response.new(
      { error: 'Unauthorized' },
      status: 401
    )
  end
  request
end
```

</td>
</tr>
</table>

### Passing State to Handlers

<table>
<tr>
<th>Python</th>
<th>TypeScript</th>
<th>Ruby</th>
</tr>
<tr>
<td>

```python
@app.pre_handler
async def load_user(request: Request):
    user = await db.get_user(
        request.headers["Authorization"]
    )
    request.state["user"] = user
    return request

@app.get("/profile")
async def get_profile(request: Request):
    user = request.state["user"]
    return {"name": user.name}
```

</td>
<td>

```typescript
app.preHandler(async (request: Request) => {
  const user = await db.getUser(
    request.headers['authorization']
  );
  request.state.user = user;
  return request;
});

app.get('/profile', async (request: Request) => {
  const user = request.state.user;
  return { name: user.name };
});
```

</td>
<td>

```ruby
app.pre_handler do |request|
  user = DB.get_user(
    request.headers['Authorization']
  )
  request.state[:user] = user
  request
end

app.get '/profile' do |request|
  user = request.state[:user]
  { name: user.name }
end
```

</td>
</tr>
</table>

### Per-Route Hooks

<table>
<tr>
<th>Python</th>
<th>TypeScript</th>
<th>Ruby</th>
</tr>
<tr>
<td>

```python
@app.get("/admin/users", hooks={
    "pre_handler": [
        require_admin,
        rate_limit
    ]
})
async def list_users():
    return {"users": [...]}
```

</td>
<td>

```typescript
app.get('/admin/users', {
  hooks: {
    preHandler: [
      requireAdmin,
      rateLimit
    ]
  },
  handler: async () => {
    return { users: [...] };
  }
});
```

</td>
<td>

```ruby
app.get '/admin/users',
  hooks: {
    pre_handler: [
      method(:require_admin),
      method(:rate_limit)
    ]
  } do
  { users: [...] }
end
```

</td>
</tr>
</table>

### Response Modification

<table>
<tr>
<th>Python</th>
<th>TypeScript</th>
<th>Ruby</th>
</tr>
<tr>
<td>

```python
@app.on_response
async def add_headers(response: Response):
    response.headers["X-Frame-Options"] = "DENY"
    response.headers["X-XSS-Protection"] = "1"
    return response
```

</td>
<td>

```typescript
app.onResponse(async (response: Response) => {
  response.headers['X-Frame-Options'] = 'DENY';
  response.headers['X-XSS-Protection'] = '1';
  return response;
});
```

</td>
<td>

```ruby
app.on_response do |response|
  response.headers['X-Frame-Options'] = 'DENY'
  response.headers['X-XSS-Protection'] = '1'
  response
end
```

</td>
</tr>
</table>

### Error Handling

<table>
<tr>
<th>Python</th>
<th>TypeScript</th>
<th>Ruby</th>
</tr>
<tr>
<td>

```python
@app.on_error
async def handle_error(response: Response):
    if response.status_code >= 500:
        await sentry.capture(response)

    response.content = {
        "error": "Server error",
        "request_id": response.state["request_id"]
    }
    return response
```

</td>
<td>

```typescript
app.onError(async (response: Response) => {
  if (response.status >= 500) {
    await sentry.capture(response);
  }

  response.body = {
    error: 'Server error',
    requestId: response.state.requestId
  };
  return response;
});
```

</td>
<td>

```ruby
app.on_error do |response|
  if response.status >= 500
    Sentry.capture(response)
  end

  response.body = {
    error: 'Server error',
    request_id: response.state[:request_id]
  }
  response
end
```

</td>
</tr>
</table>

## Common Patterns

### 1. Authentication Middleware

```python
# Python
@app.pre_handler
async def authenticate(request: Request) -> Request | Response:
    # Skip public routes
    if request.path.startswith("/public"):
        return request

    token = request.headers.get("Authorization", "").replace("Bearer ", "")
    user = await validate_jwt(token)

    if not user:
        return Response({"error": "Invalid token"}, status_code=401)

    request.state["user"] = user
    return request
```

```typescript
// TypeScript
app.preHandler(async (request: Request): Promise<Request | Response> => {
  // Skip public routes
  if (request.path.startsWith('/public')) {
    return request;
  }

  const token = (request.headers['authorization'] || '').replace('Bearer ', '');
  const user = await validateJWT(token);

  if (!user) {
    return new Response({ error: 'Invalid token' }, { status: 401 });
  }

  request.state.user = user;
  return request;
});
```

```ruby
# Ruby
app.pre_handler do |request|
  # Skip public routes
  return request if request.path.start_with?('/public')

  token = request.headers['Authorization'].to_s.delete_prefix('Bearer ')
  user = validate_jwt(token)

  unless user
    return Spikard::Response.new({ error: 'Invalid token' }, status: 401)
  end

  request.state[:user] = user
  request
end
```

### 2. Request Logging

```python
# Python
@app.on_request
async def log_request(request: Request) -> Request:
    logger.info(f"{request.method} {request.path}", extra={
        "user_agent": request.headers.get("User-Agent"),
        "ip": request.headers.get("X-Forwarded-For")
    })
    request.state["start_time"] = time.time()
    return request

@app.on_response
async def log_response(response: Response) -> Response:
    duration = time.time() - response.state["start_time"]
    logger.info(f"Response {response.status_code} in {duration*1000:.2f}ms")
    return response
```

```typescript
// TypeScript
app.onRequest(async (request: Request): Promise<Request> => {
  logger.info(`${request.method} ${request.path}`, {
    userAgent: request.headers['user-agent'],
    ip: request.headers['x-forwarded-for']
  });
  request.state.startTime = Date.now();
  return request;
});

app.onResponse(async (response: Response): Promise<Response> => {
  const duration = Date.now() - response.state.startTime;
  logger.info(`Response ${response.status} in ${duration}ms`);
  return response;
});
```

```ruby
# Ruby
app.on_request do |request|
  logger.info("#{request.method} #{request.path}",
    user_agent: request.headers['User-Agent'],
    ip: request.headers['X-Forwarded-For']
  )
  request.state[:start_time] = Time.now
  request
end

app.on_response do |response|
  duration = ((Time.now - response.state[:start_time]) * 1000).round(2)
  logger.info("Response #{response.status} in #{duration}ms")
  response
end
```

### 3. CORS Headers

```python
# Python
@app.on_response
async def add_cors_headers(response: Response) -> Response:
    origin = response.state.get("cors_origin", "*")
    response.headers["Access-Control-Allow-Origin"] = origin
    response.headers["Access-Control-Allow-Methods"] = "GET, POST, PUT, DELETE"
    response.headers["Access-Control-Allow-Headers"] = "Content-Type, Authorization"
    return response
```

```typescript
// TypeScript
app.onResponse(async (response: Response): Promise<Response> => {
  const origin = response.state.corsOrigin || '*';
  response.headers['Access-Control-Allow-Origin'] = origin;
  response.headers['Access-Control-Allow-Methods'] = 'GET, POST, PUT, DELETE';
  response.headers['Access-Control-Allow-Headers'] = 'Content-Type, Authorization';
  return response;
});
```

```ruby
# Ruby
app.on_response do |response|
  origin = response.state[:cors_origin] || '*'
  response.headers['Access-Control-Allow-Origin'] = origin
  response.headers['Access-Control-Allow-Methods'] = 'GET, POST, PUT, DELETE'
  response.headers['Access-Control-Allow-Headers'] = 'Content-Type, Authorization'
  response
end
```

### 4. Rate Limiting

```python
# Python
from datetime import datetime, timedelta

rate_limit_store = {}

@app.pre_handler
async def rate_limit(request: Request) -> Request | Response:
    user_id = request.state.get("user", {}).get("id", "anonymous")
    key = f"rate_limit:{user_id}"

    now = datetime.now()
    requests = rate_limit_store.get(key, [])

    # Remove requests older than 1 minute
    requests = [ts for ts in requests if ts > now - timedelta(minutes=1)]

    if len(requests) >= 100:  # 100 requests per minute
        return Response(
            {"error": "Rate limit exceeded"},
            status_code=429,
            headers={"Retry-After": "60"}
        )

    requests.append(now)
    rate_limit_store[key] = requests
    return request
```

```typescript
// TypeScript
const rateLimitStore = new Map<string, number[]>();

app.preHandler(async (request: Request): Promise<Request | Response> => {
  const userId = request.state.user?.id || 'anonymous';
  const key = `rate_limit:${userId}`;

  const now = Date.now();
  let requests = rateLimitStore.get(key) || [];

  // Remove requests older than 1 minute
  requests = requests.filter(ts => ts > now - 60000);

  if (requests.length >= 100) {  // 100 requests per minute
    return new Response(
      { error: 'Rate limit exceeded' },
      { status: 429, headers: { 'Retry-After': '60' } }
    );
  }

  requests.push(now);
  rateLimitStore.set(key, requests);
  return request;
});
```

```ruby
# Ruby
RATE_LIMIT_STORE = {}

app.pre_handler do |request|
  user_id = request.state[:user]&.id || 'anonymous'
  key = "rate_limit:#{user_id}"

  now = Time.now
  requests = RATE_LIMIT_STORE[key] || []

  # Remove requests older than 1 minute
  requests = requests.select { |ts| ts > now - 60 }

  if requests.length >= 100  # 100 requests per minute
    return Spikard::Response.new(
      { error: 'Rate limit exceeded' },
      status: 429,
      headers: { 'Retry-After' => '60' }
    )
  end

  requests << now
  RATE_LIMIT_STORE[key] = requests
  request
end
```

## Type Safety

### Python (mypy/pyright)

```python
from spikard import Request, Response
from typing import Union

@app.on_request
async def my_hook(request: Request) -> Union[Request, Response]:
    reveal_type(request)  # Revealed type is 'Request'
    reveal_type(request.method)  # Revealed type is 'str'
    reveal_type(request.state)  # Revealed type is 'dict[str, Any]'

    return request  # ✅ Type checks
    return Response({}, status_code=200)  # ✅ Type checks
    return "invalid"  # ❌ Type error

@app.on_response
async def response_hook(response: Response) -> Response:
    reveal_type(response)  # Revealed type is 'Response'
    return response  # ✅ Type checks
    return request  # ❌ Type error
```

### TypeScript

```typescript
import { Request, Response } from '@spikard/node';

app.onRequest(async (request: Request): Promise<Request | Response> => {
  type R = typeof request;  // Request
  type M = typeof request.method;  // string
  type S = typeof request.state;  // Record<string, any>

  return request;  // ✅ Type checks
  return new Response({}, { status: 200 });  // ✅ Type checks
  return "invalid";  // ❌ Type error
});

app.onResponse(async (response: Response): Promise<Response> => {
  type R = typeof response;  // Response
  return response;  // ✅ Type checks
  return request;  // ❌ Type error
});
```

### Ruby (Sorbet/RBS)

```ruby
# typed: strict
extend T::Sig

sig { params(request: Spikard::Request).returns(T.any(Spikard::Request, Spikard::Response)) }
def my_hook(request)
  T.reveal_type(request)  # Spikard::Request
  T.reveal_type(request.method)  # String
  T.reveal_type(request.state)  # T::Hash[Symbol, T.untyped]

  request  # ✅ Type checks
  Spikard::Response.new({}, status: 200)  # ✅ Type checks
  "invalid"  # ❌ Type error
end

app.on_request(&method(:my_hook))

sig { params(response: Spikard::Response).returns(Spikard::Response) }
def response_hook(response)
  T.reveal_type(response)  # Spikard::Response
  response  # ✅ Type checks
  request  # ❌ Type error
end

app.on_response(&method(:response_hook))
```

## Performance

### Zero-Cost When Not Used

When no hooks are registered, the overhead is ~0.5ns (single null pointer check):

```rust
// Rust implementation
#[inline(always)]
pub async fn execute_on_request(&self, req: Request) -> Result<HookResult<Request>, String> {
    // Fast path: no hooks registered
    if self.on_request.is_empty() {
        return Ok(HookResult::Continue(req));
    }
    // ... execute hooks
}
```

### Hook Execution Cost

- **Sync hooks**: ~100ns per hook (function call + data passing)
- **Async hooks**: ~1-5μs per hook (tokio task spawn + event loop)
- **Multiple hooks**: Execute sequentially, sum of individual costs

### Optimization Tips

1. **Use sync hooks when possible** - Avoid async if you don't need I/O
2. **Keep hooks lightweight** - Move heavy work to handlers
3. **Minimize state passing** - Only add essential data to `request.state`
4. **Use per-route hooks sparingly** - Global hooks are more efficient

## Next Steps

1. Read the [full API design document](./lifecycle-hooks-api-design.md)
2. Check [implementation status](./lifecycle-hooks-implementation.md)
3. See [examples directory](../../examples/) for complete applications
4. Run the test suite to see hooks in action
