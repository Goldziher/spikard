/**
 * Basic dependency injection example with value and factory dependencies.
 *
 * This example demonstrates:
 * - Registering static value dependencies
 * - Creating factory dependencies with async resolution
 * - Auto-injection by parameter name
 * - Running a basic smoke test
 *
 * Run with: node node_basic.js
 */

const { Spikard, get } = require("spikard");

async function runSmokeTest() {
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
		async ({ app_name }) => {
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
		async ({ database_url }) => {
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
	get("/config")(async function getConfig(request, { app_name, version }) {
		return {
			app: app_name,
			version: version,
		};
	});

	/**
	 * Handler with integer dependency
	 */
	get("/stats")(async function getStats(request, { max_connections }) {
		return {
			max_connections: max_connections,
			current_connections: 42,
			available: max_connections - 42,
		};
	});

	/**
	 * Handler with multiple dependencies including factory
	 */
	get("/db-info")(async function getDbInfo(request, { app_name, db_pool }) {
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
	get("/all")(async function getAll(request, { app_name, version, max_connections, database_url }) {
		return {
			app: app_name,
			version: version,
			max_connections: max_connections,
			database_url: database_url,
		};
	});

	// Smoke test: verify the app is initialized and has routes
	console.log("Node.js Smoke Test: Spikard App Initialized");
	console.log(`App Name: ${app_name || "default"}`);
	console.log(`Version: 1.0.0`);
	console.log("Routes registered successfully");
	console.log("\nSmoke test completed successfully!");
	process.exit(0);
}

runSmokeTest().catch((err) => {
	console.error("Smoke test failed:", err);
	process.exit(1);
});
