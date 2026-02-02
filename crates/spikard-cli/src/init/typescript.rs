//! TypeScript Project Scaffolder
//!
//! Generates a minimal TypeScript project structure with Spikard integration.
//! Follows modern TypeScript/Node.js conventions with strict typing and ESM module support.

use super::scaffolder::{ProjectScaffolder, ScaffoldedFile};
use anyhow::Result;
use std::path::Path;
use std::path::PathBuf;

/// TypeScript/Node.js project scaffolder
pub struct TypeScriptScaffolder;

impl ProjectScaffolder for TypeScriptScaffolder {
    #[allow(clippy::vec_init_then_push)]
    fn scaffold(&self, _project_dir: &Path, project_name: &str) -> Result<Vec<ScaffoldedFile>> {
        let kebab_name = project_name.replace('_', "-").to_lowercase();

        let mut files = vec![];

        // package.json
        files.push(ScaffoldedFile::new(
            PathBuf::from("package.json"),
            self.generate_package_json(&kebab_name),
        ));

        // pnpm-lock.yaml (empty placeholder)
        files.push(ScaffoldedFile::new(PathBuf::from("pnpm-lock.yaml"), String::new()));

        // tsconfig.json
        files.push(ScaffoldedFile::new(
            PathBuf::from("tsconfig.json"),
            self.generate_tsconfig(),
        ));

        // vitest.config.ts
        files.push(ScaffoldedFile::new(
            PathBuf::from("vitest.config.ts"),
            self.generate_vitest_config(),
        ));

        // .gitignore
        files.push(ScaffoldedFile::new(
            PathBuf::from(".gitignore"),
            self.generate_gitignore(),
        ));

        // README.md
        files.push(ScaffoldedFile::new(
            PathBuf::from("README.md"),
            self.generate_readme(&kebab_name),
        ));

        // src/app.ts
        files.push(ScaffoldedFile::new(PathBuf::from("src/app.ts"), self.generate_app_ts()));

        // tests/app.spec.ts
        files.push(ScaffoldedFile::new(
            PathBuf::from("tests/app.spec.ts"),
            self.generate_app_spec_ts(),
        ));

        Ok(files)
    }

    fn next_steps(&self, project_name: &str) -> Vec<String> {
        let kebab_name = project_name.replace('_', "-").to_lowercase();
        vec![
            format!("cd {}", kebab_name),
            "pnpm install".to_string(),
            "pnpm dev".to_string(),
        ]
    }
}

impl TypeScriptScaffolder {
    fn generate_package_json(&self, kebab_name: &str) -> String {
        format!(
            r#"{{
	"name": "{kebab_name}",
	"version": "0.0.1",
	"type": "module",
	"description": "Spikard TypeScript application",
	"main": "dist/app.js",
	"scripts": {{
		"dev": "tsx src/app.ts",
		"start": "node dist/app.js",
		"build": "tsc",
		"test": "vitest",
		"test:run": "vitest run",
		"lint": "biome lint src tests",
		"format": "biome format src tests"
	}},
	"dependencies": {{
		"@spikard/node": "^0.6.0"
	}},
	"devDependencies": {{
		"@biomejs/biome": "^1.9.4",
		"@types/node": "^20.0.0",
		"tsx": "^4.21.0",
		"typescript": "^5.9.3",
		"vitest": "^1.0.0"
	}},
	"engines": {{
		"node": ">=18"
	}}
}}
"#
        )
    }

    fn generate_tsconfig(&self) -> String {
        r#"{
	"compilerOptions": {
		"allowJs": true,
		"allowSyntheticDefaultImports": true,
		"alwaysStrict": true,
		"baseUrl": ".",
		"declaration": true,
		"esModuleInterop": true,
		"exactOptionalPropertyTypes": true,
		"forceConsistentCasingInFileNames": true,
		"incremental": true,
		"isolatedModules": true,
		"lib": ["ES2022"],
		"module": "ESNext",
		"moduleResolution": "bundler",
		"noEmit": false,
		"noImplicitAny": true,
		"noUncheckedIndexedAccess": true,
		"noUnusedLocals": true,
		"noUnusedParameters": true,
		"outDir": "dist",
		"removeComments": true,
		"resolveJsonModule": true,
		"skipLibCheck": true,
		"strict": true,
		"strictBindCallApply": true,
		"strictFunctionTypes": true,
		"strictNullChecks": true,
		"strictPropertyInitialization": true,
		"target": "ES2022"
	},
	"include": ["src/**/*.ts"],
	"exclude": ["node_modules", "dist", "tests"]
}
"#
        .to_string()
    }

    fn generate_vitest_config(&self) -> String {
        r"import { defineConfig } from 'vitest/config';

export default defineConfig({
	test: {
		environment: 'node',
		globals: true,
		coverage: {
			provider: 'v8',
			reporter: ['text', 'json', 'html'],
			exclude: [
				'node_modules/',
				'dist/',
			],
		},
	},
});
"
        .to_string()
    }

    fn generate_gitignore(&self) -> String {
        r"# Dependencies
node_modules/
pnpm-lock.yaml
package-lock.json
yarn.lock

# Build output
dist/
*.tsbuildinfo
*.js
*.mjs
*.cjs

# Testing
coverage/
.vitest/

# IDE
.vscode/
.idea/
*.swp
*.swo
*~

# Environment
.env
.env.local
.env.*.local

# OS
.DS_Store
Thumbs.db
"
        .to_string()
    }

    fn generate_readme(&self, kebab_name: &str) -> String {
        format!(
            r"# {kebab_name}

A Spikard TypeScript application.

## Requirements

- Node.js 18+
- pnpm 10+

## Installation

```bash
pnpm install
```

## Development

Start the development server with hot reload:

```bash
pnpm dev
```

The server will start on `http://127.0.0.1:8000`.

## Building

```bash
pnpm build
```

## Testing

Run tests:

```bash
pnpm test
```

Run tests once:

```bash
pnpm test:run
```

## Linting & Formatting

Lint the code:

```bash
pnpm lint
```

Format the code:

```bash
pnpm format
```

## Next Steps

1. Install dependencies: `pnpm install`
2. Start development: `pnpm dev`
3. Make requests to `http://localhost:8000` to verify
4. Write your handlers in `src/app.ts`
5. Add tests in `tests/`

## Documentation

- [Spikard Documentation](https://github.com/Goldziher/spikard)
- [TypeScript Handbook](https://www.typescriptlang.org/docs/)
- [Node.js API](https://nodejs.org/api/)
"
        )
    }

    fn generate_app_ts(&self) -> String {
        r"/**
 * Basic Spikard TypeScript Application
 *
 * This example demonstrates a simple HTTP server with health check
 * and echo endpoints using the Spikard Node.js bindings.
 */

import { Spikard, get, post, type HandlerFunction } from '@spikard/node';

const app = new Spikard();

/**
 * Root endpoint - returns welcome message
 */
const handleRoot: HandlerFunction = async () => {
	return {
		message: 'Hello from Spikard TypeScript!',
		timestamp: new Date().toISOString(),
	};
};
get('/')(handleRoot);

/**
 * Health check endpoint
 */
const handleHealth: HandlerFunction = async () => {
	return {
		status: 'healthy',
		uptime: process.uptime(),
		timestamp: new Date().toISOString(),
	};
};
get('/health')(handleHealth);

/**
 * Echo endpoint - returns request body
 */
const handleEcho: HandlerFunction = async (req) => {
	try {
		const body = req.body ? req.json() : null;
		return {
			echoed: true,
			body,
			receivedAt: new Date().toISOString(),
		};
	} catch {
		return {
			error: 'Invalid JSON in request body',
			code: 'invalid_body',
		};
	}
};
post('/echo')(handleEcho);

console.log('Starting Spikard TypeScript server on http://0.0.0.0:8000');
console.log('Press Ctrl+C to stop\n');

app.run({ port: 8000, host: '0.0.0.0' });
"
        .to_string()
    }

    fn generate_app_spec_ts(&self) -> String {
        r"import { describe, it, expect } from 'vitest';

describe('Spikard App', () => {
	it('should be importable', () => {
		// Basic test to verify the app structure is valid
		expect(true).toBe(true);
	});

	it('should define handler functions', () => {
		// Test that handlers are properly defined
		// Integration tests would test the actual HTTP behavior
		expect(true).toBe(true);
	});

	describe('Health endpoint', () => {
		it('should return health status', () => {
			// Integration test for /health would go here
			expect(true).toBe(true);
		});
	});

	describe('Echo endpoint', () => {
		it('should echo request body', () => {
			// Integration test for /echo would go here
			expect(true).toBe(true);
		});
	});
});
"
        .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_typescript_scaffold_creates_files() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let scaffolder = TypeScriptScaffolder;
        let files = scaffolder.scaffold(temp_dir.path(), "test_app")?;

        assert!(!files.is_empty(), "Should create multiple files");

        // Check expected files exist in the vec
        let file_paths: Vec<_> = files.iter().map(|f| f.path.to_string_lossy().to_string()).collect();

        assert!(file_paths.iter().any(|p| p == "package.json"));
        assert!(file_paths.iter().any(|p| p == "tsconfig.json"));
        assert!(file_paths.iter().any(|p| p == "vitest.config.ts"));
        assert!(file_paths.iter().any(|p| p == ".gitignore"));
        assert!(file_paths.iter().any(|p| p == "README.md"));
        assert!(file_paths.iter().any(|p| p == "src/app.ts"));
        assert!(file_paths.iter().any(|p| p == "tests/app.spec.ts"));

        Ok(())
    }

    #[test]
    fn test_typescript_scaffold_package_json_valid() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let scaffolder = TypeScriptScaffolder;
        let files = scaffolder.scaffold(temp_dir.path(), "my-app")?;

        let pkg_json = files
            .iter()
            .find(|f| f.path.file_name().unwrap() == "package.json")
            .unwrap();

        assert!(pkg_json.content.contains("\"type\": \"module\""));
        assert!(pkg_json.content.contains("@spikard/node"));
        assert!(pkg_json.content.contains("vitest"));
        assert!(pkg_json.content.contains("typescript"));

        Ok(())
    }

    #[test]
    fn test_typescript_scaffold_tsconfig_has_strict_mode() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let scaffolder = TypeScriptScaffolder;
        let files = scaffolder.scaffold(temp_dir.path(), "test-app")?;

        let tsconfig = files
            .iter()
            .find(|f| f.path.file_name().unwrap() == "tsconfig.json")
            .unwrap();

        assert!(tsconfig.content.contains("\"strict\": true"));
        assert!(tsconfig.content.contains("\"noImplicitAny\": true"));
        assert!(tsconfig.content.contains("\"strictNullChecks\": true"));
        assert!(tsconfig.content.contains("\"target\": \"ES2022\""));

        Ok(())
    }

    #[test]
    fn test_typescript_next_steps() {
        let scaffolder = TypeScriptScaffolder;
        let steps = scaffolder.next_steps("my_app");

        assert!(!steps.is_empty());
        assert!(steps[0].contains("my-app"));
        assert!(steps.iter().any(|s| s.contains("pnpm install")));
        assert!(steps.iter().any(|s| s.contains("pnpm dev")));
    }

    #[test]
    #[allow(clippy::cmp_owned)]
    fn test_typescript_app_ts_has_handlers() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let scaffolder = TypeScriptScaffolder;
        let files = scaffolder.scaffold(temp_dir.path(), "test")?;

        let app_ts = files.iter().find(|f| f.path == PathBuf::from("src/app.ts")).unwrap();

        assert!(app_ts.content.contains("Spikard"));
        assert!(app_ts.content.contains("get('/')"));
        assert!(app_ts.content.contains("get('/health')"));
        assert!(app_ts.content.contains("post('/echo')"));

        Ok(())
    }
}
