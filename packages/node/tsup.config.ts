import { defineConfig } from "tsup";

export default defineConfig({
	entry: ["src/index.ts"],
	format: ["cjs", "esm"],
	dts: false, // Use napi-generated types instead
	splitting: false,
	sourcemap: true,
	clean: true,
	minify: false,
	shims: true, // Enable __dirname and __filename in ESM
	// Mark the parent directory (napi-generated index.js) and ws as external
	external: [/^\.\./, "ws"],
	noExternal: [],
});
