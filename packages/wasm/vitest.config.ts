import { defineConfig } from "vitest/config";

export default defineConfig({
	test: {
		globals: true,
		environment: "node",
		include: ["src/**/*.spec.ts", "runtime-tests/**/*.spec.ts"],
	},
	coverage: {
		provider: "v8",
		reporter: ["text", "html", "lcov"],
		exclude: ["node_modules/**", "dist/**", "**/*.spec.ts", "**/*.test.ts"],
		thresholds: {
			lines: 80,
			functions: 80,
			branches: 80,
			statements: 80,
			autoUpdate: false,
			perFile: false,
		},
	},
});
