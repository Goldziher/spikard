import fs from "node:fs";
import path from "node:path";
import ts from "typescript";
import { describe, expect, test } from "vitest";

const SNIPPET_PATH = path.resolve(__dirname, "../../../docs/snippets/typescript/dependency_injection.md");

const extractSnippet = (): string => {
	const content = fs.readFileSync(SNIPPET_PATH, "utf8");
	const match = content.match(/```typescript\s*([\s\S]*?)```/);
	if (!match || !match[1]) {
		throw new Error("DI snippet not found in docs/snippets/typescript/dependency_injection.md");
	}
	return match[1].trim();
};

describe("docs DI snippet (TypeScript)", () => {
	test("parses and transpiles without errors", () => {
		const source = extractSnippet();
		const result = ts.transpileModule(source, {
			compilerOptions: {
				module: ts.ModuleKind.CommonJS,
				target: ts.ScriptTarget.ES2020,
				esModuleInterop: true,
				strict: true,
			},
			reportDiagnostics: true,
		});
		expect(result.diagnostics?.length ?? 0).toBe(0);
	});
});
