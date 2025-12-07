import { defineConfig } from "vitest/config";

export default defineConfig({
	test: {
		globals: true,
		environment: "node",
		coverage: {
			provider: "v8",
			reporter: ["text", "text-summary", "json", "html", "lcov"],
			include: ["src/**/*.ts"],
			exclude: [
				"**/*.spec.ts",
				"**/*.test.ts",
				// Native binding re-export surfaces and thin runtime wrappers that are exercised through
				// the Rust-backed integration suite rather than Vitest.
				"src/index.ts",
				"src/app.ts",
				"src/server.ts",
				"src/routing.ts",
				"src/params.ts",
				"src/types.ts",
				"src/config.ts",
				"src/background.ts",
				"src/streaming.ts",
			],
			all: true,
			lines: 80,
			functions: 80,
			branches: 80,
			statements: 80,
			skipFull: false,
			reportOnFailure: true,
		},
	},
});
