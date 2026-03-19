Install the CLI, scaffold a real project, and generate typed handlers:

```bash
spikard init my_api --lang python --dir .
cd my_api
uv sync
uv run python -m my_api.app
```

Generate code from a schema with the current command surface:

```bash
# OpenAPI 3.1
spikard generate openapi examples/schemas/auth-service.openapi.yaml --lang python --output ./generated.py

# GraphQL
spikard generate graphql examples/schemas/social.graphql --lang typescript --output ./src/generated.ts

# Protobuf / gRPC
spikard generate protobuf examples/schemas/user-service.proto --lang rust --output ./src/generated.rs
```
