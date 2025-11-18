#!/usr/bin/env node
const fs = require("node:fs");
const path = require("node:path");

const runtimePath = path.resolve(__dirname, "..", "runtime", "spikard_wasm.js");

if (!fs.existsSync(runtimePath)) {
	process.exit(0);
}

const source = fs.readFileSync(runtimePath, "utf8");
const patched = source.replace(
	/wasm\.__wbindgen_start\(\);/,
	'if (typeof wasm.__wbindgen_start === "function") {\n    wasm.__wbindgen_start();\n}\n',
);

fs.writeFileSync(runtimePath, patched);
