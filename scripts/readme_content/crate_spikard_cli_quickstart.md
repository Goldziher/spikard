### init

Create a new Spikard project:

```bash
spikard init my-project --lang python --dir .
```

Supported languages: `python`, `typescript`, `rust`, `ruby`, `php`, `elixir`

### generate

Generate code from API specifications:

```bash
# OpenAPI 3.1
spikard generate openapi ./openapi.yaml --lang python --output ./generated.py

# AsyncAPI 3.0
spikard generate asyncapi ./asyncapi.yaml --lang elixir --output ./lib/generated.ex

# GraphQL SDL / introspection
spikard generate graphql ./schema.graphql --lang typescript --output ./src/generated.ts

# OpenRPC / JSON-RPC
spikard generate jsonrpc ./openrpc.json --lang ruby --output ./generated.rb

# Protobuf / gRPC
spikard generate protobuf ./service.proto --lang rust --output ./src/generated.rs

# PHP DTOs
spikard generate php-dto --output ./src/Generated
```

Supported target languages: `python`, `typescript`, `rust`, `ruby`, `php`, `elixir`

### validate-asyncapi

Validate AsyncAPI specifications:

```bash
spikard validate-asyncapi ./asyncapi.json
```
