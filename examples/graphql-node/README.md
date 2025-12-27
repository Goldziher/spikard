# Spikard GraphQL Node.js/TypeScript Example

This example demonstrates how to use the Spikard GraphQL bindings with Node.js/TypeScript to configure and manage GraphQL schemas with full type safety.

## Overview

The Spikard GraphQL bindings provide a high-performance, type-safe API for configuring GraphQL schemas in TypeScript. The bindings expose:

- **GraphQLSchemaBuilder**: A fluent builder for creating custom schema configurations
- **GraphQL**: Factory methods for common schema configuration patterns
- **SchemaConfig**: The resulting configuration object

## Features

- **Introspection Control**: Enable/disable GraphQL introspection queries
- **Complexity Limits**: Set maximum query complexity to prevent expensive queries
- **Depth Limits**: Set maximum nesting depth to prevent deeply nested queries
- **Type-Safe Configuration**: Full TypeScript type support with IntelliSense
- **Zero-Copy Serialization**: Efficient JSON serialization for schema configuration

## API Examples

### Basic Usage

```typescript
import { GraphQL } from 'spikard';

// Create a new schema builder
const builder = GraphQL.schemaBuilder();

// Configure it
builder.enableIntrospection(true);
builder.complexityLimit(5000);
builder.depthLimit(50);

// Get the final configuration
const config = builder.finish();
```

### Factory Methods

```typescript
// Query-only schema (no mutations or subscriptions)
const queryConfig = GraphQL.queryOnlyConfig();

// Query and mutation schema (no subscriptions)
const queryMutationConfig = GraphQL.queryMutationConfig();

// Full-featured schema (queries, mutations, and subscriptions)
const fullConfig = GraphQL.fullSchemaConfig();

// Default configuration
const defaultConfig = GraphQL.defaultSchemaConfig();
```

### Security Configuration

```typescript
// Production-grade configuration
const prodBuilder = GraphQL.schemaBuilder();
prodBuilder.enableIntrospection(false);  // Disable introspection
prodBuilder.complexityLimit(1000);       // Strict complexity limit
prodBuilder.depthLimit(20);              // Strict depth limit
const prodConfig = prodBuilder.finish();
```

### Development Configuration

```typescript
// Permissive development configuration
const devBuilder = GraphQL.schemaBuilder();
devBuilder.enableIntrospection(true);   // Enable introspection
devBuilder.complexityLimit(10000);      // Higher limit
devBuilder.depthLimit(100);             // Higher depth
const devConfig = devBuilder.finish();
```

## Configuration Options

### `enableIntrospection(enabled: boolean)`

Controls whether GraphQL introspection is enabled. Introspection allows clients to query the schema structure.

- **Default**: `true`
- **Production Recommendation**: `false` (for security through obscurity)
- **Development**: `true` (for tooling and exploration)

### `complexityLimit(limit: number)`

Sets the maximum complexity allowed for queries. The complexity is calculated based on query structure and field costs.

- **Default**: None (unlimited)
- **Value**: 0 means unlimited, any positive number is the limit
- **Typical Production**: 1000-5000
- **Development**: 10000+

### `depthLimit(limit: number)`

Sets the maximum nesting depth allowed for queries. Prevents deeply nested selections.

- **Default**: None (unlimited)
- **Value**: 0 means unlimited, any positive number is the limit
- **Typical Production**: 15-25
- **Development**: 50-100

## Type Safety

The TypeScript bindings provide full type safety:

```typescript
interface SchemaConfig {
  introspectionEnabled?: boolean;
  complexityLimit?: number;
  depthLimit?: number;
}
```

All builder methods are properly typed with JSDoc documentation for IDE autocomplete.

## Performance Considerations

- The builder uses **zero-copy** design where possible
- Configuration serialization to JSON is optimized
- All limits are compile-time validated where applicable
- The builder state is managed efficiently in memory

## Running the Examples

```bash
# Install dependencies
npm install

# Compile TypeScript
npm run build

# Run the examples
npm start

# Watch mode (development)
npm run dev
```

## Integration with Spikard HTTP Server

The GraphQL configuration can be integrated with the Spikard HTTP server:

```typescript
import { runServer } from 'spikard';

const graphqlConfig = GraphQL.schemaBuilder()
  .enableIntrospection(true)
  .complexityLimit(5000)
  .depthLimit(50)
  .finish();

// Pass to your GraphQL handler
const app = new Spikard();
// ... register GraphQL handlers ...
app.run({ port: 8000 });
```

## Best Practices

1. **Production**: Disable introspection and use strict limits
2. **Development**: Enable introspection and use generous limits
3. **Testing**: Use custom limits for specific test scenarios
4. **Monitoring**: Track complexity and depth metrics in production
5. **Documentation**: Document custom limits in your API documentation

## Troubleshooting

### Configuration Not Applied

Ensure you call `finish()` to get the final configuration:

```typescript
const builder = GraphQL.schemaBuilder();
builder.complexityLimit(5000);
// const config = builder.finish();  // Must call this!
```

### Type Errors

Make sure you're using the correct types:

```typescript
// Correct
builder.complexityLimit(5000);  // number

// Incorrect
builder.complexityLimit('5000');  // string - will cause type error
```

## Further Reading

- [Spikard GraphQL Documentation](../graphql/README.md)
- [GraphQL Best Practices](https://graphql.org/learn/best-practices/)
- [Query Complexity Analysis](https://graphql.org/en/learn/security/)

## Contributing

Issues and pull requests are welcome. Please ensure all examples compile and run correctly before submitting.
