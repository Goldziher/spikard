/**
 * GraphQL Schema Configuration Example
 *
 * This example demonstrates how to use the Spikard GraphQL bindings
 * to configure GraphQL schemas with various settings like introspection
 * control, complexity limits, and depth limits.
 */

import { GraphQL } from 'spikard';

/**
 * Example 1: Creating a basic schema configuration with defaults
 */
function exampleBasicConfig(): void {
  console.log('\n=== Example 1: Basic Schema Configuration ===');

  const config = GraphQL.defaultSchemaConfig();
  console.log('Default config:', {
    introspectionEnabled: config.introspectionEnabled,
    complexityLimit: config.complexityLimit,
    depthLimit: config.depthLimit,
  });
}

/**
 * Example 2: Using the schema builder with fluent API
 */
function exampleSchemaBuilder(): void {
  console.log('\n=== Example 2: Schema Builder (Fluent API) ===');

  const builder = GraphQL.schemaBuilder();
  builder.enableIntrospection(true);
  builder.complexityLimit(5000);
  builder.depthLimit(50);

  console.log('Builder configuration:', {
    introspectionEnabled: builder.isIntrospectionEnabled(),
    complexityLimit: builder.getComplexityLimit(),
    depthLimit: builder.getDepthLimit(),
  });

  // Finish building to get the final config
  const config = builder.finish();
  console.log('Finished config:', config);
}

/**
 * Example 3: Creating specialized schema configurations
 */
function exampleSpecializedConfigs(): void {
  console.log('\n=== Example 3: Specialized Schema Configurations ===');

  // Query-only schema (no mutations or subscriptions)
  const queryOnlyConfig = GraphQL.queryOnlyConfig();
  console.log('Query-only config:', queryOnlyConfig);

  // Query and mutation schema (no subscriptions)
  const queryMutationConfig = GraphQL.queryMutationConfig();
  console.log('Query-mutation config:', queryMutationConfig);

  // Full schema with queries, mutations, and subscriptions
  const fullConfig = GraphQL.fullSchemaConfig();
  console.log('Full schema config:', fullConfig);
}

/**
 * Example 4: Customizing limits for production security
 */
function exampleProductionConfig(): void {
  console.log('\n=== Example 4: Production Security Configuration ===');

  // Production-grade configuration with strict limits
  const productionBuilder = GraphQL.schemaBuilder();
  productionBuilder.enableIntrospection(false); // Disable introspection in production
  productionBuilder.complexityLimit(1000);      // Prevent complex queries
  productionBuilder.depthLimit(20);             // Prevent deeply nested queries

  const config = productionBuilder.finish();
  console.log('Production config:', {
    introspectionEnabled: productionBuilder.isIntrospectionEnabled(),
    complexityLimit: productionBuilder.getComplexityLimit(),
    depthLimit: productionBuilder.getDepthLimit(),
  });
}

/**
 * Example 5: Development configuration with unlimited access
 */
function exampleDevelopmentConfig(): void {
  console.log('\n=== Example 5: Development Configuration ===');

  // Development configuration with looser limits
  const devBuilder = GraphQL.schemaBuilder();
  devBuilder.enableIntrospection(true);  // Enable introspection for development
  devBuilder.complexityLimit(10000);     // Higher limit for exploration
  devBuilder.depthLimit(100);            // Higher depth for complex queries

  const config = devBuilder.finish();
  console.log('Development config:', {
    introspectionEnabled: devBuilder.isIntrospectionEnabled(),
    complexityLimit: devBuilder.getComplexityLimit(),
    depthLimit: devBuilder.getDepthLimit(),
  });
}

/**
 * Example 6: Zero-limit configuration (unlimited)
 */
function exampleUnlimitedConfig(): void {
  console.log('\n=== Example 6: Unlimited Configuration ===');

  // Setting limits to 0 means unlimited
  const unlimitedBuilder = GraphQL.schemaBuilder();
  unlimitedBuilder.complexityLimit(0);  // No complexity limit
  unlimitedBuilder.depthLimit(0);       // No depth limit

  console.log('Unlimited configuration:', {
    complexityLimit: unlimitedBuilder.getComplexityLimit(),
    depthLimit: unlimitedBuilder.getDepthLimit(),
  });
}

/**
 * Example 7: Converting builder to JSON for transmission
 */
function exampleJsonSerialization(): void {
  console.log('\n=== Example 7: JSON Serialization ===');

  const builder = GraphQL.schemaBuilder();
  builder.enableIntrospection(true);
  builder.complexityLimit(5000);
  builder.depthLimit(50);

  // Serialize to JSON for transmission or storage
  const json = builder.to_json();
  console.log('Serialized JSON:', JSON.stringify(json, null, 2));
}

/**
 * Main entry point - runs all examples
 */
async function main(): Promise<void> {
  console.log('Spikard GraphQL Node.js Bindings Examples');
  console.log('==========================================');

  try {
    exampleBasicConfig();
    exampleSchemaBuilder();
    exampleSpecializedConfigs();
    exampleProductionConfig();
    exampleDevelopmentConfig();
    exampleUnlimitedConfig();
    exampleJsonSerialization();

    console.log('\n==========================================');
    console.log('All examples completed successfully!');
  } catch (error) {
    console.error('Error running examples:', error);
    process.exit(1);
  }
}

// Run the main function
main().catch(console.error);
