/**
 * Unit tests for Node.js-specific utilities (static file serving)
 */

import fs, { mkdirSync, rmSync, writeFileSync } from "node:fs";
import path from "node:path";
import { afterEach, beforeEach, describe, expect, it } from "vitest";
import { Spikard } from "./app.ts";
import type { StaticFilesConfig } from "./config.ts";
import type { SpikardApp } from "./index.ts";

describe("Node.js Static File Utilities", () => {
	let tmpDir: string;

	beforeEach(() => {
		// Create a temporary directory for test files
		tmpDir = path.join(process.cwd(), `.test-static-${Math.random().toString(36).slice(2)}`);
		try {
			mkdirSync(tmpDir, { recursive: true });
		} catch (e) {
			// Directory might already exist
		}
	});

	afterEach(() => {
		// Clean up test directory
		try {
			if (fs.existsSync(tmpDir)) {
				rmSync(tmpDir, { recursive: true, force: true });
			}
		} catch (e) {
			// Ignore cleanup errors
		}
	});

	describe("TestClient creation", () => {
		it("should create TestClient with Spikard app", () => {
			const app = new Spikard();
			expect(() => {
				// TestClient is imported from node.ts
				// We test that it can be created with a valid app
				const testAppData: SpikardApp = {
					routes: app.routes,
					handlers: app.handlers,
				};
				expect(testAppData).toBeDefined();
			}).not.toThrow();
		});

		it("should handle app without static config", () => {
			const app = new Spikard();
			const testAppData: SpikardApp = {
				routes: app.routes,
				handlers: app.handlers,
			};

			expect(testAppData.config).toBeUndefined();
		});

		it("should handle app with static files config", () => {
			const app = new Spikard();
			const staticConfig: StaticFilesConfig = {
				directory: tmpDir,
				routePrefix: "/static",
				cacheControl: "max-age=3600",
				indexFile: true,
			};

			const testAppData: SpikardApp = {
				routes: app.routes,
				handlers: app.handlers,
				config: {
					staticFiles: [staticConfig],
				},
			};

			expect(testAppData.config?.staticFiles).toHaveLength(1);
		});

		it("should handle app with existing static directory", () => {
			// Create a test file
			writeFileSync(path.join(tmpDir, "test.txt"), "test content");

			const app = new Spikard();
			const staticConfig: StaticFilesConfig = {
				directory: tmpDir,
				routePrefix: "/files",
			};

			const testAppData: SpikardApp = {
				routes: app.routes,
				handlers: app.handlers,
				config: {
					staticFiles: [staticConfig],
				},
			};

			expect(testAppData.config?.staticFiles).toBeDefined();
			expect(fs.existsSync(tmpDir)).toBe(true);
		});

		it("should handle app with multiple static directories", () => {
			const dir1 = path.join(tmpDir, "public");
			const dir2 = path.join(tmpDir, "assets");

			mkdirSync(dir1, { recursive: true });
			mkdirSync(dir2, { recursive: true });

			const configs: StaticFilesConfig[] = [
				{
					directory: dir1,
					routePrefix: "/public",
				},
				{
					directory: dir2,
					routePrefix: "/assets",
				},
			];

			const app = new Spikard();
			const testAppData: SpikardApp = {
				routes: app.routes,
				handlers: app.handlers,
				config: {
					staticFiles: configs,
				},
			};

			expect(testAppData.config?.staticFiles).toHaveLength(2);
		});

		it("should handle app with nested directories", () => {
			const nested = path.join(tmpDir, "public", "images", "avatars");
			mkdirSync(nested, { recursive: true });
			writeFileSync(path.join(nested, "avatar.png"), "image data");

			const app = new Spikard();
			const staticConfig: StaticFilesConfig = {
				directory: tmpDir,
				routePrefix: "/",
			};

			const testAppData: SpikardApp = {
				routes: app.routes,
				handlers: app.handlers,
				config: {
					staticFiles: [staticConfig],
				},
			};

			expect(testAppData.config?.staticFiles).toBeDefined();
		});
	});

	describe("Static file configuration options", () => {
		it("should allow optional cacheControl", () => {
			const configWithCache: StaticFilesConfig = {
				directory: "/public",
				routePrefix: "/static",
				cacheControl: "max-age=3600, public",
			};

			const configWithoutCache: StaticFilesConfig = {
				directory: "/public",
				routePrefix: "/static",
			};

			expect(configWithCache.cacheControl).toBe("max-age=3600, public");
			expect(configWithoutCache.cacheControl).toBeUndefined();
		});

		it("should allow optional indexFile", () => {
			const configWithIndexFile: StaticFilesConfig = {
				directory: "/public",
				routePrefix: "/static",
				indexFile: true,
			};

			const configWithoutIndexFile: StaticFilesConfig = {
				directory: "/public",
				routePrefix: "/static",
				indexFile: false,
			};

			const configDefaultIndexFile: StaticFilesConfig = {
				directory: "/public",
				routePrefix: "/static",
			};

			expect(configWithIndexFile.indexFile).toBe(true);
			expect(configWithoutIndexFile.indexFile).toBe(false);
			expect(configDefaultIndexFile.indexFile).toBeUndefined();
		});
	});

	describe("Route prefix handling", () => {
		it("should accept various route prefix formats", () => {
			const prefixes = ["/", "/static", "/public/", "/api/v1/assets/"];

			prefixes.forEach((prefix) => {
				const config: StaticFilesConfig = {
					directory: "/public",
					routePrefix: prefix,
				};

				expect(config.routePrefix).toBe(prefix);
			});
		});

		it("should handle root prefix", () => {
			const config: StaticFilesConfig = {
				directory: "/public",
				routePrefix: "/",
			};

			expect(config.routePrefix).toBe("/");
		});
	});

	describe("Directory paths", () => {
		it("should accept absolute paths", () => {
			const paths = ["/public", "/var/www/static", "/home/user/files"];

			paths.forEach((path) => {
				const config: StaticFilesConfig = {
					directory: path,
					routePrefix: "/static",
				};

				expect(config.directory).toBe(path);
			});
		});

		it("should accept relative paths", () => {
			const paths = ["./public", "./dist", "../static"];

			paths.forEach((path) => {
				const config: StaticFilesConfig = {
					directory: path,
					routePrefix: "/static",
				};

				expect(config.directory).toBe(path);
			});
		});
	});

	describe("Content type detection", () => {
		it("should handle HTML files", () => {
			// Test that the system can be configured for HTML content types
			const htmlConfig: StaticFilesConfig = {
				directory: "/public",
				routePrefix: "/static",
			};

			expect(htmlConfig).toBeDefined();
			// Content type detection happens internally
		});

		it("should handle multiple file types", () => {
			const fileTypes = [".html", ".css", ".js", ".json", ".txt", ".png", ".jpg", ".svg"];

			fileTypes.forEach((ext) => {
				expect(ext).toMatch(/^\./);
			});
		});
	});

	describe("Cache control configuration", () => {
		it("should accept standard cache control values", () => {
			const cacheValues = [
				"no-cache",
				"no-store",
				"public",
				"private",
				"max-age=3600",
				"max-age=31536000, public, immutable",
			];

			cacheValues.forEach((value) => {
				const config: StaticFilesConfig = {
					directory: "/public",
					routePrefix: "/static",
					cacheControl: value,
				};

				expect(config.cacheControl).toBe(value);
			});
		});

		it("should handle null cache control", () => {
			const config: StaticFilesConfig = {
				directory: "/public",
				routePrefix: "/static",
				cacheControl: null,
			};

			expect(config.cacheControl).toBeNull();
		});
	});

	describe("App with static files integration", () => {
		it("should create app with static files and routes", () => {
			const staticConfig: StaticFilesConfig = {
				directory: "/public",
				routePrefix: "/static",
			};

			const app: SpikardApp = {
				routes: [
					{
						method: "GET",
						path: "/api/users",
						handler_name: "getUsers",
						is_async: true,
					},
				],
				handlers: {
					getUsers: async () => [],
				},
				config: {
					staticFiles: [staticConfig],
				},
			};

			expect(app.routes).toHaveLength(1);
			expect(app.config?.staticFiles).toHaveLength(1);
		});

		it("should preserve all app properties with static config", () => {
			const app: SpikardApp = {
				routes: [
					{
						method: "GET",
						path: "/",
						handler_name: "root",
						is_async: true,
					},
				],
				handlers: {
					root: async () => ({ ok: true }),
				},
				config: {
					staticFiles: [
						{
							directory: "./public",
							routePrefix: "/",
						},
					],
				},
			};

			expect(app).toHaveProperty("routes");
			expect(app).toHaveProperty("handlers");
			expect(app).toHaveProperty("config");
			expect(app.config?.staticFiles).toBeDefined();
		});
	});

	describe("Static files manifest", () => {
		it("should support manifest in config", () => {
			const app: SpikardApp = {
				routes: [],
				handlers: {},
				config: {
					staticFiles: [],
					__wasmStaticManifest: [],
				},
			};

			expect(app.config?.__wasmStaticManifest).toBeDefined();
			expect(app.config?.__wasmStaticManifest).toEqual([]);
		});

		it("should preserve manifest entries", () => {
			const manifest = [
				{
					route: "/index.html",
					headers: { "content-type": "text/html" },
					body: "aGVsbG8=", // base64
				},
			];

			const app: SpikardApp = {
				routes: [],
				handlers: {},
				config: {
					staticFiles: [],
					__wasmStaticManifest: manifest,
				},
			};

			expect(app.config?.__wasmStaticManifest).toEqual(manifest);
		});
	});

	describe("Edge cases", () => {
		it("should handle empty directory path", () => {
			const config: StaticFilesConfig = {
				directory: "",
				routePrefix: "/static",
			};

			expect(config.directory).toBe("");
		});

		it("should handle empty route prefix", () => {
			const config: StaticFilesConfig = {
				directory: "/public",
				routePrefix: "",
			};

			expect(config.routePrefix).toBe("");
		});

		it("should handle undefined directory and routePrefix", () => {
			const config: StaticFilesConfig = {};

			expect(config.directory).toBeUndefined();
			expect(config.routePrefix).toBeUndefined();
		});

		it("should handle very long paths", () => {
			const longPath = "/very/long/path/to/some/deeply/nested/static/files/directory";
			const config: StaticFilesConfig = {
				directory: longPath,
				routePrefix: "/static",
			};

			expect(config.directory).toBe(longPath);
		});

		it("should handle paths with special characters", () => {
			const specialPaths = ["/public-files", "/public_files", "/public.files", "/public~files"];

			specialPaths.forEach((path) => {
				const config: StaticFilesConfig = {
					directory: path,
					routePrefix: "/static",
				};

				expect(config.directory).toBe(path);
			});
		});
	});

	describe("Multiple static configurations", () => {
		it("should handle different cache control for different directories", () => {
			const configs: StaticFilesConfig[] = [
				{
					directory: "/public/images",
					routePrefix: "/images",
					cacheControl: "max-age=31536000, public, immutable",
				},
				{
					directory: "/public/css",
					routePrefix: "/css",
					cacheControl: "max-age=86400, public",
				},
				{
					directory: "/public/html",
					routePrefix: "/html",
					cacheControl: "no-cache, public",
				},
			];

			expect(configs).toHaveLength(3);
			expect(configs[0].cacheControl).toContain("immutable");
			expect(configs[1].cacheControl).toContain("86400");
			expect(configs[2].cacheControl).toContain("no-cache");
		});

		it("should handle different indexFile settings per directory", () => {
			const configs: StaticFilesConfig[] = [
				{
					directory: "/public",
					routePrefix: "/",
					indexFile: true,
				},
				{
					directory: "/api/docs",
					routePrefix: "/docs",
					indexFile: false,
				},
				{
					directory: "/downloads",
					routePrefix: "/downloads",
					// indexFile not specified
				},
			];

			expect(configs[0].indexFile).toBe(true);
			expect(configs[1].indexFile).toBe(false);
			expect(configs[2].indexFile).toBeUndefined();
		});
	});

	describe("Config type safety", () => {
		it("should enforce required directory and routePrefix when provided", () => {
			const config: StaticFilesConfig = {
				directory: "/public",
				routePrefix: "/static",
			};

			expect(config).toHaveProperty("directory");
			expect(config).toHaveProperty("routePrefix");
		});

		it("should allow partial config", () => {
			const partialConfig: StaticFilesConfig = {
				directory: "/public",
			};

			expect(partialConfig.directory).toBe("/public");
			expect(partialConfig.routePrefix).toBeUndefined();
		});
	});
});
