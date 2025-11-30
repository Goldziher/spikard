/**
 * Basic dependency injection example with value and factory dependencies.
 *
 * This example demonstrates:
 * - Registering static value dependencies
 * - Creating factory dependencies with async resolution
 * - Auto-injection by parameter name (via destructuring)
 * - Method chaining with provide()
 *
 * Run with: node --loader ts-node/esm node_basic.ts
 */

import { get, Spikard } from "spikard";

const app = new Spikard();

// Register value dependencies
// Values are automatically cached and shared across all requests
app.provide("app_name", "SpikardApp");
app.provide("version", "1.0.0");
app.provide("max_connections", 100);

// Register a factory dependency that depends on other dependencies
// Factories can be async and access other resolved dependencies
app.provide(
	"database_url",
	async ({ app_name }: { app_name: string }) => {
		// Simulate async configuration loading
		await new Promise((resolve) => setTimeout(resolve, 10));
		return `postgresql://localhost/${app_name.toLowerCase()}`;
	},
	{
		dependsOn: ["app_name"],
		singleton: true, // Resolved once globally
		cacheable: true, // Cache the result
	},
);

// Factory that creates a mock database connection
app.provide(
	"db_pool",
	async ({ database_url }: { database_url: string }) => {
		// Simulate database connection
		console.log(`Connecting to database: ${database_url}`);
		return {
			connected: true,
			url: database_url,
			maxConnections: 20,
		};
	},
	{
		dependsOn: ["database_url"],
		singleton: true,
	},
);

/**
 * Handler with value dependency injection
 *
 * Dependencies are passed as the second parameter and can be destructured
 */
get("/config")(async function getConfig(request, { app_name, version }: { app_name: string; version: string }) {
	return {
		app: app_name,
		version: version,
	};
});

/**
 * Handler with integer dependency
 */
get("/stats")(async function getStats(request, { max_connections }: { max_connections: number }) {
	return {
		max_connections: max_connections,
		current_connections: 42,
		available: max_connections - 42,
	};
});

/**
 * Handler with multiple dependencies including factory
 */
get("/db-info")(async function getDbInfo(
	request,
	{ app_name, db_pool }: { app_name: string; db_pool: { connected: boolean; url: string; maxConnections: number } },
) {
	return {
		app: app_name,
		database: {
			connected: db_pool.connected,
			url: db_pool.url,
			maxConnections: db_pool.maxConnections,
		},
	};
});

/**
 * Handler with all dependencies
 */
get("/all")(async function getAll(
	request,
	{
		app_name,
		version,
		max_connections,
		database_url,
	}: { app_name: string; version: string; max_connections: number; database_url: string },
) {
	return {
		app: app_name,
		version: version,
		max_connections: max_connections,
		database_url: database_url,
	};
});

// Start the server (only if run directly)
if (require.main === module) {
	console.log("Starting Spikard server with dependency injection...");
	app.run({ port: 8000 });
	console.log("Server running on http://localhost:8000");
	console.log("\nAvailable endpoints:");
	console.log("  GET /config     - App name and version");
	console.log("  GET /stats      - Connection statistics");
	console.log("  GET /db-info    - Database connection info");
	console.log("  GET /all        - All configuration values");
}

export default app;
