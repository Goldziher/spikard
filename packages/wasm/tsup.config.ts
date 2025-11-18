import { defineConfig } from "tsup";

export default defineConfig({
	entry: ["src/index.ts", "src/node.ts"],
	format: ["esm", "cjs"],
	dts: true,
	sourcemap: true,
	minify: false,
	treeshake: true,
	clean: true,
	target: "esnext",
	publicDir: "runtime",
});
