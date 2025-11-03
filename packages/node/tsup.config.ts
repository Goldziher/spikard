import { defineConfig } from "tsup";

export default defineConfig({
	entry: ["src/index.ts"],
	format: ["cjs", "esm"],
	dts: false, // Use napi-generated types instead
	splitting: false,
	sourcemap: true,
	clean: true,
	minify: false,
	// Mark the parent directory (napi-generated index.js) as external
	external: [/^\.\./],
	noExternal: [],
});
