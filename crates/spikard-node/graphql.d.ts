/**
 * GraphQL Schema Configuration Bindings for Node.js
 *
 * High-performance TypeScript bindings for configuring GraphQL schemas
 * with support for introspection control, complexity limits, and depth limits.
 *
 * @module spikard/graphql
 */

/**
 * GraphQL Schema Configuration
 *
 * Represents the final configuration for a GraphQL schema after builder setup.
 * This object is passed to the GraphQL execution engine to apply constraints
 * and settings to query execution.
 *
 * @interface SchemaConfig
 */
export interface SchemaConfig {
	/**
	 * Whether to enable GraphQL introspection queries.
	 * Introspection allows clients to query the schema structure.
	 *
	 * @default true
	 */
	introspectionEnabled?: boolean;

	/**
	 * Maximum complexity allowed for queries (0 = unlimited).
	 * Queries exceeding this complexity will be rejected.
	 *
	 * Complexity is calculated based on the query structure and field costs.
	 * Typical production values: 1000-5000
	 *
	 * @default undefined (unlimited)
	 */
	complexityLimit?: number;

	/**
	 * Maximum depth allowed for queries (0 = unlimited).
	 * Queries exceeding this depth will be rejected.
	 *
	 * Depth is the maximum nesting level of selections.
	 * Typical production values: 15-25
	 *
	 * @default undefined (unlimited)
	 */
	depthLimit?: number;
}

/**
 * GraphQL Schema Builder
 *
 * Provides a fluent interface for building GraphQL schema configurations.
 * The builder follows the same pattern as the Rust SchemaBuilder with
 * mutations applied directly to the builder instance.
 *
 * @class GraphQLSchemaBuilder
 * @example
 * const builder = new GraphQLSchemaBuilder();
 * builder.enableIntrospection(true);
 * builder.complexityLimit(5000);
 * builder.depthLimit(50);
 * const config = builder.finish();
 */
export class GraphQLSchemaBuilder {
	/**
	 * Create a new GraphQL schema builder with default settings
	 *
	 * Default configuration:
	 * - Introspection enabled
	 * - No complexity limit
	 * - No depth limit
	 *
	 * @constructor
	 */
	constructor();

	/**
	 * Enable or disable GraphQL introspection
	 *
	 * Introspection is enabled by default. Disabling it prevents clients
	 * from querying the schema structure via introspection queries, which
	 * can be useful for security through obscurity in production.
	 *
	 * @param enabled - Whether to enable introspection
	 * @returns void (mutates this instance)
	 *
	 * @example
	 * builder.enableIntrospection(false);  // Disable introspection
	 */
	enableIntrospection(enabled: boolean): void;

	/**
	 * Set the maximum complexity allowed for queries
	 *
	 * The complexity is calculated based on the query structure and field costs.
	 * Queries exceeding this limit will be rejected with a validation error.
	 * A value of 0 means unlimited.
	 *
	 * Typical values:
	 * - Production: 1000-5000
	 * - Development: 10000+
	 * - Testing: varies based on test scenarios
	 *
	 * @param limit - The maximum complexity allowed (0 = unlimited)
	 * @returns void (mutates this instance)
	 *
	 * @example
	 * builder.complexityLimit(5000);  // Allow up to 5000 complexity
	 */
	complexityLimit(limit: number): void;

	/**
	 * Set the maximum depth allowed for queries
	 *
	 * The depth is the maximum nesting level of selections in a query.
	 * Queries exceeding this limit will be rejected with a validation error.
	 * A value of 0 means unlimited.
	 *
	 * Typical values:
	 * - Production: 15-25
	 * - Development: 50-100
	 * - Testing: varies based on test scenarios
	 *
	 * @param limit - The maximum depth allowed (0 = unlimited)
	 * @returns void (mutates this instance)
	 *
	 * @example
	 * builder.depthLimit(50);  // Allow up to 50 nesting levels
	 */
	depthLimit(limit: number): void;

	/**
	 * Check if introspection is currently enabled
	 *
	 * @returns true if introspection is enabled, false otherwise
	 */
	isIntrospectionEnabled(): boolean;

	/**
	 * Get the current complexity limit if set
	 *
	 * @returns The complexity limit, or null if unlimited
	 */
	getComplexityLimit(): number | null;

	/**
	 * Get the current depth limit if set
	 *
	 * @returns The depth limit, or null if unlimited
	 */
	getDepthLimit(): number | null;

	/**
	 * Build and return the schema configuration
	 *
	 * This method finalizes the configuration and returns a SchemaConfig object
	 * that can be serialized and passed to the GraphQL execution engine.
	 *
	 * @returns A SchemaConfig instance with the configured settings
	 *
	 * @example
	 * const builder = new GraphQLSchemaBuilder();
	 * builder.complexityLimit(5000);
	 * const config = builder.finish();
	 * // config is now {
	 * //   introspectionEnabled: true,
	 * //   complexityLimit: 5000,
	 * //   depthLimit: undefined
	 * // }
	 */
	finish(): SchemaConfig;

	/**
	 * Convert the schema configuration to a JSON object
	 *
	 * This method serializes the current builder state to a JSON Value
	 * for transmission to the Rust execution engine or storage.
	 * Zero-copy serialization is used where possible.
	 *
	 * @returns A JSON representation of the schema configuration
	 *
	 * @example
	 * const builder = new GraphQLSchemaBuilder();
	 * builder.complexityLimit(5000);
	 * const json = builder.to_json();
	 * console.log(JSON.stringify(json, null, 2));
	 */
	to_json(): Record<string, any>;
}

/**
 * GraphQL Utilities and Factory Functions
 *
 * Provides factory methods for creating schema builders and configurations
 * with common patterns. All factory methods follow the same configuration
 * semantics as the SchemaBuilder.
 *
 * @class GraphQL
 *
 * @example
 * const builder = GraphQL.schemaBuilder();
 * const config = GraphQL.defaultSchemaConfig();
 * const queryConfig = GraphQL.queryOnlyConfig();
 */
export class GraphQL {
	/**
	 * Create a new GraphQL schema builder
	 *
	 * Returns a builder instance that can be configured with various settings.
	 * The builder uses fluent API with mutation methods (returns void).
	 *
	 * @static
	 * @returns A new GraphQLSchemaBuilder instance with default settings
	 *
	 * @example
	 * const builder = GraphQL.schemaBuilder()
	 *   .enableIntrospection(true)
	 *   .complexityLimit(5000)
	 *   .depthLimit(50);
	 */
	static schemaBuilder(): GraphQLSchemaBuilder;

	/**
	 * Create a default schema configuration
	 *
	 * Returns a configuration with default settings:
	 * - Introspection enabled
	 * - No complexity limit
	 * - No depth limit
	 *
	 * @static
	 * @returns A SchemaConfig with default settings
	 *
	 * @example
	 * const config = GraphQL.defaultSchemaConfig();
	 * console.log(config.introspectionEnabled);  // true
	 */
	static defaultSchemaConfig(): SchemaConfig;

	/**
	 * Create a schema configuration for query-only schemas
	 *
	 * Returns a configuration suitable for schemas without mutations or
	 * subscriptions. Useful for read-only APIs.
	 *
	 * @static
	 * @returns A SchemaConfig optimized for query-only schemas
	 *
	 * @example
	 * const config = GraphQL.queryOnlyConfig();
	 * // Use this for read-only GraphQL endpoints
	 */
	static queryOnlyConfig(): SchemaConfig;

	/**
	 * Create a schema configuration for query and mutation schemas
	 *
	 * Returns a configuration suitable for schemas with queries and mutations
	 * but no subscriptions. This is the most common pattern for REST-like
	 * GraphQL APIs.
	 *
	 * @static
	 * @returns A SchemaConfig optimized for query and mutation schemas
	 *
	 * @example
	 * const config = GraphQL.queryMutationConfig();
	 * // Use this for typical CRUD GraphQL endpoints
	 */
	static queryMutationConfig(): SchemaConfig;

	/**
	 * Create a full schema configuration
	 *
	 * Returns a configuration suitable for schemas with queries, mutations,
	 * and subscriptions. Use this for real-time GraphQL APIs.
	 *
	 * @static
	 * @returns A SchemaConfig optimized for full-featured schemas
	 *
	 * @example
	 * const config = GraphQL.fullSchemaConfig();
	 * // Use this for real-time GraphQL APIs with subscriptions
	 */
	static fullSchemaConfig(): SchemaConfig;
}
