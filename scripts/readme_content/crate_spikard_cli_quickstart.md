### init

Create a new Spikard project:

```bash
spikard init my-project --lang python --dir .
```

Supported languages: `python`, `typescript`, `rust`, `ruby`, `php`

### generate

Generate code from API specifications:

```bash
# OpenAPI 3.0+
spikard generate openapi ./openapi.json --lang python --output ./generated

# AsyncAPI 2.0+
spikard generate asyncapi ./asyncapi.json --lang python --output ./generated

# GraphQL
spikard generate graphql ./schema.graphql --lang python --output ./generated

# JSON-RPC 2.0
spikard generate jsonrpc ./openrpc.json --lang python --output ./generated

# PHP DTOs
spikard generate php-dto ./openapi.json --output ./src/Generated
```

Supported target languages: `python`, `typescript`, `rust`, `ruby`, `php`

### validate-asyncapi

Validate AsyncAPI specifications:

```bash
spikard validate-asyncapi ./asyncapi.json
```
