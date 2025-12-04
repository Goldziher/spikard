import { defineConfig } from "vitest/config";

export default defineConfig({
	test: {
		globals: true,
		environment: "node",
		coverage: {
			provider: "v8",
			reporter: ["text", "text-summary", "json", "html", "lcov"],
			include: ["src/**/*.ts"],
			exclude: ["**/*.spec.ts", "**/*.test.ts"],
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
