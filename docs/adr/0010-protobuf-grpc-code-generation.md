# ADR 0010: Protobuf/gRPC Code Generation Architecture
**Status**: Accepted
**Date**: 2025-12-31

## Context

Spikard needs to generate type-safe, idiomatic code from `.proto` files for gRPC service implementations across five languages (Python, TypeScript, Ruby, PHP, Rust). Unlike REST/GraphQL which have JSON-based schemas, protobuf uses a binary protocol with strictly-typed messages requiring:

1. **Binary serialization support** - Integration with protobuf runtimes (google-protobuf, prost, etc.)
2. **Four streaming modes** - Unary, server streaming, client streaming, bidirectional streaming
3. **Metadata handling** - gRPC-specific headers and trailers distinct from HTTP headers
4. **17 standard status codes** - OK, CANCELLED, INVALID_ARGUMENT, etc.
5. **Cross-language type mapping** - Proto3 scalar types → language native types with proper nullability

The generator must produce code that passes strict quality tools (mypy --strict, tsc, steep, phpstan level max, clippy) while integrating with Spikard's existing gRPC runtime.

## Decision

### Architecture

**CLI Entry Point**: `spikard generate protobuf --input schema.proto --output ./generated/`

**Generator Organization**: `crates/spikard-cli/src/codegen/protobuf/`
```
protobuf/
├── spec_parser.rs           # Proto3 schema parser (uses prost-reflect)
├── type_mapper.rs           # Proto3 → language type mapping
├── generators/
│   ├── base.rs             # Shared generator logic
│   ├── python.rs           # Python: dataclasses with google-protobuf integration
│   ├── typescript.rs       # TypeScript: interfaces + protobufjs integration
│   ├── ruby.rs             # Ruby: classes with google-protobuf gem
│   ├── php.rs              # PHP: classes with google/protobuf package
│   └── rust.rs             # Rust: structs with prost derives
└── tests/
    └── quality_tests.rs    # Fixture-driven quality validation
```

### Core Design Principles

1. **Runtime Integration First**
   - Generated code uses existing `spikard-http` gRPC runtime
   - No standalone gRPC server generation - integrates with Spikard HTTP server
   - Handlers implement language-specific gRPC handler traits

2. **Proto3 Focus**
   - Only proto3 syntax supported (proto2 deprecated)
   - All fields are optional by default (proto3 semantics)
   - Proper handling of `optional`, `repeated`, and `map` fields

3. **Type Safety**
   - Strict null safety: proto3 optional → language nullable types
   - Enum type safety with generated union types
   - Message nesting preserved with proper scoping

4. **Binary Protocol**
   - Messages generated as serializable types
   - Integration with language-specific protobuf libraries
   - Zero-copy where possible (Rust Bytes, Python memoryview)

### Type Mapping Strategy

**Proto3 → Language Type Matrix**:

| Proto3 Type | Python | TypeScript | Ruby | PHP | Rust |
|-------------|--------|------------|------|-----|------|
| `double` | `float` | `number` | `Float` | `float` | `f64` |
| `float` | `float` | `number` | `Float` | `float` | `f32` |
| `int32` | `int` | `number` | `Integer` | `int` | `i32` |
| `int64` | `int` | `number\|bigint` | `Integer` | `int` | `i64` |
| `uint32` | `int` | `number` | `Integer` | `int` | `u32` |
| `uint64` | `int` | `number\|bigint` | `Integer` | `int` | `u64` |
| `bool` | `bool` | `boolean` | `Boolean` | `bool` | `bool` |
| `string` | `str` | `string` | `String` | `string` | `String` |
| `bytes` | `bytes` | `Uint8Array` | `String` | `string` | `Bytes` |
| `message` | dataclass | interface | class | class | struct |
| `enum` | `Literal[...]` | union type | module | class | enum |
| `repeated T` | `list[T]` | `T[]` | `Array<T>` | `array<T>` | `Vec<T>` |
| `map<K,V>` | `dict[K,V]` | `Map<K,V>` | `Hash{K=>V}` | `array<K,V>` | `HashMap<K,V>` |
| `optional T` | `T\|None` | `T\|undefined` | `T\|nil` | `?T` | `Option<T>` |

### Generated Code Structure

**Python Example** (`user_service_pb.py`):
```python
"""Generated from user.proto - DO NOT EDIT"""
from dataclasses import dataclass
from typing import Optional
from google.protobuf import message

@dataclass
class GetUserRequest:
    user_id: int

@dataclass
class User:
    id: int
    name: str
    email: Optional[str] = None

    @classmethod
    def from_proto(cls, msg: message.Message) -> "User":
        """Deserialize from protobuf message"""

    def to_proto(self) -> message.Message:
        """Serialize to protobuf message"""
```

**TypeScript Example** (`user_service_pb.ts`):
```typescript
// Generated from user.proto - DO NOT EDIT
import { Message } from 'protobufjs';

export interface GetUserRequest {
  userId: number;
}

export interface User {
  id: number;
  name: string;
  email?: string;
}

export function deserializeUser(msg: Message): User { /*...*/ }
export function serializeUser(user: User): Message { /*...*/ }
```

### Quality Validation

All generated code must pass:
- **Python**: `mypy --strict`, `ruff check`
- **TypeScript**: `tsc --noEmit`, `biome check`
- **Ruby**: `steep check`, `rubocop`
- **PHP**: `phpstan --level=max`, `php-cs-fixer`
- **Rust**: `cargo check`, `cargo clippy`

Quality tests run in `crates/spikard-cli/tests/protobuf_quality.rs` using fixture-driven approach.

### Service Handler Generation

Handlers integrate with existing gRPC runtime traits:

**Python**:
```python
from spikard.grpc import GrpcHandler, GrpcRequest, GrpcResponse

class UserServiceHandler(GrpcHandler):
    def handle_request(self, request: GrpcRequest) -> GrpcResponse:
        if request.method_name == "GetUser":
            req = GetUserRequest.from_proto(request.payload)
            user = User(id=req.user_id, name="Alice")
            return GrpcResponse(payload=user.to_proto())
```

**Rust**:
```rust
use spikard_http::grpc::{GrpcHandler, GrpcRequest, GrpcResponse};

struct UserServiceHandler;

impl GrpcHandler for UserServiceHandler {
    fn service_name(&self) -> &'static str { "user.UserService" }

    async fn call(&self, request: GrpcRequest) -> Result<GrpcResponse> {
        match request.method_name.as_str() {
            "GetUser" => {
                let req: GetUserRequest = prost::Message::decode(&request.payload)?;
                let user = User { id: req.user_id, name: "Alice".into(), ..Default::default() };
                Ok(GrpcResponse::new(user.encode_to_vec()))
            }
            _ => Err(tonic::Status::unimplemented("Method not found"))
        }
    }
}
```

## Consequences

**Benefits**:
- Type-safe protobuf message handling across all languages
- Generated code passes strictest quality tools
- Integration with existing Spikard gRPC runtime
- Consistent API patterns across languages
- Reuses shared codegen utilities (case conversion, escaping, formatters)

**Trade-offs**:
- Requires language-specific protobuf runtime dependencies
- Binary protocol adds complexity vs JSON
- Limited to proto3 (no proto2 support)
- Must maintain compatibility with multiple protobuf library versions

**Performance**:
- Binary serialization is ~3-5x smaller than JSON
- Faster parsing than JSON (no string→number conversion)
- Zero-copy optimizations in Rust with `prost` + `Bytes`

**Maintenance**:
- Update type mappings when protobuf spec evolves
- Keep quality validators in sync with protobuf library updates
- Test fixtures must cover all proto3 features (nested messages, oneof, maps, etc.)

## References

- Spec parser: `crates/spikard-cli/src/codegen/protobuf/spec_parser.rs`
- Generators: `crates/spikard-cli/src/codegen/protobuf/generators/`
- Type mapper: `crates/spikard-cli/src/codegen/protobuf/type_mapper.rs`
- Quality tests: `crates/spikard-cli/tests/protobuf_quality.rs`
- Runtime integration: `crates/spikard-http/src/grpc/`
- Proto3 spec: https://protobuf.dev/programming-guides/proto3/
