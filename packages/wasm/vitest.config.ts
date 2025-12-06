import { defineConfig } from "vitest/config";

export default defineConfig({
	test: {
		globals: true,
		environment: "node",
		include: ["src/**/*.spec.ts", "runtime-tests/**/*.spec.ts"],
		coverage: {
			provider: "v8",
			reporter: ["text", "text-summary", "json", "html", "lcov"],
			include: ["src/**/*.ts"],
			exclude: [
				"**/*.spec.ts",
				"**/*.test.ts",
				"src/index.ts", // re-export surface
				"src/types.ts", // type definitions only
				"src/server.ts", // WASM runtime wrapper
				"src/node.ts", // Node-specific runtime wrapper
				"src/config.ts", // pure type definitions (interfaces only)
				"src/request.ts", // pure type definition (Request interface)
				"src/params.ts", // type definitions + no-op utility
				"src/testing.ts", // WASM integration code (requires full runtime to test properly)
				"runtime/**", // compiled WASM binaries
				"dist/**",
				"node_modules/**",
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
