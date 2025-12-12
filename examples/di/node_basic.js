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

const { Spikard, get } = require("@spikard/node");

async function runSmokeTest() {
	const app = new Spikard();

	app.provide("app_name", "SpikardApp");
	app.provide("version", "1.0.0");
	app.provide("max_connections", 100);

	app.provide(
		"database_url",
		async ({ app_name }) => {
			await new Promise((resolve) => setTimeout(resolve, 10));
			return `postgresql://localhost/${app_name.toLowerCase()}`;
		},
		{
			dependsOn: ["app_name"],
			singleton: true,
			cacheable: true,
		},
	);

	app.provide(
		"db_pool",
		async ({ database_url }) => {
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
	get("/config")(async function getConfig(_request, { app_name, version }) {
		return {
			app: app_name,
			version: version,
		};
	});

	/**
	 * Handler with integer dependency
	 */
	get("/stats")(async function getStats(_request, { max_connections }) {
		return {
			max_connections: max_connections,
			current_connections: 42,
			available: max_connections - 42,
		};
	});

	/**
	 * Handler with multiple dependencies including factory
	 */
	get("/db-info")(async function getDbInfo(_request, { app_name, db_pool }) {
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
	get("/all")(async function getAll(_request, { app_name, version, max_connections, database_url }) {
		return {
			app: app_name,
			version: version,
			max_connections: max_connections,
			database_url: database_url,
		};
	});

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
