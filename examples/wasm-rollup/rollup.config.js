import resolve from "@rollup/plugin-node-resolve";
import typescript from "@rollup/plugin-typescript";
import terser from "@rollup/plugin-terser";

const isProduction = process.env.NODE_ENV === "production";

/**
 * @type {import('rollup').RollupOptions[]}
 */
export default [
	{
		input: "src/index.ts",
		output: {
			file: "dist/index.mjs",
			format: "es",
			sourcemap: !isProduction,
		},
		plugins: [
			typescript({
				tsconfig: false,
				compilerOptions: {
					target: "ES2022",
					module: "ESNext",
					lib: ["ESNext", "DOM", "DOM.Iterable"],
					strict: true,
					skipLibCheck: true,
					noImplicitAny: true,
					noUncheckedIndexedAccess: true,
					moduleResolution: "bundler",
				},
				exclude: ["node_modules"],
			}),
			resolve({
				browser: true,
				preferBuiltins: false,
			}),
			isProduction && terser(),
		],
		external: [],
	},

	{
		input: "src/index.ts",
		output: {
			file: "dist/index.cjs",
			format: "cjs",
			sourcemap: !isProduction,
			exports: "named",
		},
		plugins: [
			typescript({
				tsconfig: false,
				compilerOptions: {
					target: "ES2022",
					module: "ESNext",
					lib: ["ESNext"],
					strict: true,
					skipLibCheck: true,
					moduleResolution: "node",
				},
				exclude: ["node_modules"],
			}),
			resolve({
				browser: false,
				preferBuiltins: true,
			}),
			isProduction && terser(),
		],
		external: ["@spikard/wasm"],
	},
];
