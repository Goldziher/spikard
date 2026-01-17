# Protobuf/gRPC Test Fixtures

This directory contains the first batch of test fixtures for the comprehensive protobuf/gRPC testing infrastructure (Phase 2 of the plan).

## Fixture Overview

### Batch 1: Core Functionality (10 fixtures)

| # | Fixture | Description | RPC Type | Status Code |
|---|---------|-------------|----------|------------|
| 01 | `01_simple_unary.json` | Basic unary RPC with scalar types | Unary | OK |
| 02 | `02_nested_messages.json` | Messages with nested types | Unary | OK |
| 03 | `03_repeated_fields.json` | Arrays/repeated fields | Unary | OK |
| 04 | `04_optional_fields.json` | Optional fields with presence | Unary | OK |
| 05 | `05_enum_types.json` | Enum definitions and values | Unary | OK |
| 06 | `06_server_streaming.json` | Server streaming RPC | Server Streaming | OK |
| 07 | `07_client_streaming.json` | Client streaming RPC | Client Streaming | OK |
| 08 | `08_bidirectional_streaming.json` | Bidirectional streaming RPC | Bidirectional | OK |
| 09 | `09_error_handling.json` | gRPC status codes and errors | Unary | INVALID_ARGUMENT |
| 10 | `10_metadata.json` | gRPC metadata (headers) | Unary | OK |

## Streaming Mode Coverage

- **Unary RPC**: 7 fixtures (01-05, 09-10)
  - Simple scalar types (01)
  - Nested messages (02)
  - Repeated fields (03)
  - Optional fields (04)
  - Enum types (05)
  - Error handling (09)
  - Metadata headers (10)

- **Server Streaming**: 1 fixture (06)
  - Paginated results pattern
  - Multiple response messages

- **Client Streaming**: 1 fixture (07)
  - File upload pattern
  - Aggregation of multiple requests

- **Bidirectional Streaming**: 1 fixture (08)
  - Chat/real-time collaboration pattern
  - Concurrent send/receive

## Fixture Structure

Each fixture follows the template from Phase 2.2:

```json
{
  "name": "Fixture display name",
  "description": "Detailed description of what this fixture tests",
  "protobuf": {
    "package": "example.v1",
    "messages": [
      {
        "name": "MessageName",
        "fields": [
          {
            "name": "field_name",
            "type": "string|int32|int64|bool|double|bytes|CustomType",
            "number": 1,
            "label": "required|optional|repeated"
          }
        ]
      }
    ],
    "enums": [
      {
        "name": "EnumName",
        "values": [
          {"name": "ENUM_VALUE", "number": 0}
        ]
      }
    ],
    "services": [
      {
        "name": "ServiceName",
        "methods": [
          {
            "name": "MethodName",
            "input_type": "RequestMessage",
            "output_type": "ResponseMessage",
            "client_streaming": false,
            "server_streaming": false
          }
        ]
      }
    ]
  },
  "handler": {
    "service": "package.ServiceName",
    "method": "MethodName"
  },
  "request": {
    "metadata": {
      "authorization": "Bearer token",
      "content-type": "application/grpc"
    },
    "message": { /* protobuf message */ }
  },
  "expected_response": {
    "status_code": "OK",
    "message": { /* protobuf message */ }
  }
}
```

## Fixture Categories

### 1. Message Type Coverage
- **Scalar Types**: int32, string, double, bool, bytes, int64
- **Custom Types**: Nested messages, enums
- **Field Labels**: required, optional, repeated

### 2. Service Patterns
- **Unary**: Simple request-response
- **Server Streaming**: Single request, multiple responses
- **Client Streaming**: Multiple requests, single response
- **Bidirectional**: Multiple requests and responses

### 3. Error Handling
- Status codes: OK, INVALID_ARGUMENT, NOT_FOUND, INTERNAL, etc.
- Error messages and codes
- Validation error patterns

### 4. Metadata
- Authorization headers
- Custom headers (trace IDs, etc.)
- Request/response metadata handling

## Usage

These fixtures are designed to be consumed by:

1. **Test Generator** (`tools/test-generator`)
   - Generates .proto files from fixture definitions
   - Creates language-specific test code
   - Validates generated code quality

2. **Code Generators** (`spikard generate protobuf`)
   - Generates protobuf types from messages
   - Generates service stubs

3. **Integration Tests**
   - Real handler implementations
   - End-to-end testing of gRPC runtime

## Implementation Status

- [x] Phase 1: Fixture schema (extended in 00-FIXTURE-SCHEMA.json)
- [x] Phase 2.1: Directory structure created (`testing_data/protobuf/`)
- [x] Phase 2.2: First 10 fixtures created (this batch)
- [ ] Phase 2.3: Remaining 40+ fixtures (future)
- [ ] Phase 3: Test generator updates
- [ ] Phase 4: Rust integration tests
- [ ] Phase 5: Language package tests
- [ ] Phase 6-8: Quality gates, CI/CD, documentation

## Next Steps

1. Create Phase 2.3 fixtures:
   - Basic Messages (11-19): More complex message patterns
   - Unary RPCs (20-29): All scalar types, custom messages
   - Server Streaming (30-39): Pagination, real-time updates
   - Client Streaming (40-49): Batch uploads, aggregation
   - Bidirectional Streaming (50-59): Advanced patterns
   - Error Handling (60-69): All gRPC status codes
   - Metadata (70-79): Complex header scenarios
   - Large Payloads (80-89): 1MB, 10MB messages
   - Edge Cases (90-99): Unicode, binary data
   - Security (100-109): JWT, mTLS

2. Update test generator to consume these fixtures

3. Generate language-specific tests from fixtures

4. Run quality gates on generated code

## References

- Plan: `.claude/plans/synthetic-cooking-kettle.md`
- Schema: `testing_data/00-FIXTURE-SCHEMA.json`
- Test Generator: `tools/test-generator/`
