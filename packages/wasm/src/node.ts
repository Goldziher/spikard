import fs from "node:fs";
import path from "node:path";
import { gunzipSync } from "node:zlib";
import type { StaticFilesConfig, StaticManifestEntry } from "./config";
import type { SpikardApp } from "./index";
import { __setGunzipImplementation, TestClient as BaseTestClient } from "./testing";

__setGunzipImplementation((bytes) => new Uint8Array(gunzipSync(bytes)));

class NodeTestClient extends BaseTestClient {
	constructor(app: SpikardApp) {
		super(withStaticManifest(app));
	}
}

export const TestClient = NodeTestClient;
export { type LifecycleHookFunction, type LifecycleHooks, Spikard } from "./app";
export * as background from "./background";
export type {
	ApiKeyConfig,
	CompressionConfig,
	ContactInfo,
	JwtConfig,
	LicenseInfo,
	OpenApiConfig,
	RateLimitConfig,
	SecuritySchemeInfo,
	ServerConfig,
	ServerInfo,
	StaticFilesConfig,
} from "./config";
export type { CorsConfig, FileParam, JsonSchema, RouteMetadata, SpikardApp } from "./index";
export type { Body, Path, Query, QueryDefault } from "./params";
export type { Request } from "./request";
export { del, get, patch, post, put, type RouteOptions, route } from "./routing";
export { createFetchHandler, runServer, type ServerOptions } from "./server";
export { StreamingResponse, type StreamingResponseInit } from "./streaming";
export type { TestResponse } from "./testing";
export type {
	Base64EncodedBody,
	HandlerFunction,
	HandlerPayload,
	HandlerResult,
	JsonPrimitive,
	JsonRecord,
	JsonValue,
	MaybePromise,
	StructuredHandlerResponse,
} from "./types";

function withStaticManifest(app: SpikardApp): SpikardApp {
	const config = app.config;
	if (!config || !config.staticFiles || config.staticFiles.length === 0) {
		return app;
	}
	const manifest = buildStaticManifest(config.staticFiles);
	if (manifest.length === 0) {
		return app;
	}
	return {
		...app,
		config: {
			...config,
			__wasmStaticManifest: manifest,
		},
	};
}

function buildStaticManifest(configs: StaticFilesConfig[]): StaticManifestEntry[] {
	const manifest: StaticManifestEntry[] = [];
	for (const config of configs) {
		if (!config.directory || !config.routePrefix) {
			continue;
		}
		if (!fs.existsSync(config.directory)) {
			continue;
		}
		const files = listFiles(config.directory);
		for (const filePath of files) {
			const relative = path.relative(config.directory, filePath).split(path.sep).join("/");
			// Remove trailing slashes safely using string methods (avoids ReDoS vulnerability)
			let normalizedPrefix = config.routePrefix;
			while (normalizedPrefix.endsWith("/")) {
				normalizedPrefix = normalizedPrefix.slice(0, -1);
			}
			const route = normalizeRoute(`${normalizedPrefix}/${relative}`);
			const headers = buildStaticHeaders(filePath, config.cacheControl);
			const body = fs.readFileSync(filePath).toString("base64");
			manifest.push({ route, headers, body });
		}

		if (config.indexFile ?? true) {
			const indexPath = path.join(config.directory, "index.html");
			if (fs.existsSync(indexPath)) {
				const headers = buildStaticHeaders(indexPath, config.cacheControl);
				const body = fs.readFileSync(indexPath).toString("base64");
				const prefix = normalizeRoute(config.routePrefix);
				manifest.push({ route: prefix, headers: { ...headers }, body });
				if (!prefix.endsWith("/")) {
					manifest.push({ route: `${prefix}/`, headers: { ...headers }, body });
				}
			}
		}
	}
	return manifest;
}

function listFiles(root: string): string[] {
	const entries: string[] = [];
	const stack: string[] = [root];
	while (stack.length > 0) {
		const current = stack.pop();
		if (!current) {
			continue;
		}
		const stats = fs.statSync(current);
		if (stats.isDirectory()) {
			for (const child of fs.readdirSync(current)) {
				stack.push(path.join(current, child));
			}
		} else {
			entries.push(current);
		}
	}
	return entries;
}

function buildStaticHeaders(filePath: string, cacheControl?: string | null): Record<string, string> {
	const headers: Record<string, string> = {
		"content-type": lookupContentType(filePath),
	};
	if (cacheControl) {
		headers["cache-control"] = cacheControl;
	}
	return headers;
}

function lookupContentType(filePath: string): string {
	const ext = path.extname(filePath).toLowerCase();
	switch (ext) {
		case ".html":
		case ".htm":
			return "text/html";
		case ".txt":
			return "text/plain";
		case ".json":
			return "application/json";
		case ".js":
			return "application/javascript";
		case ".css":
			return "text/css";
		default:
			return "application/octet-stream";
	}
}

function normalizeRoute(route: string): string {
	const normalized = route.replace(/\/+/g, "/");
	return normalized.startsWith("/") ? normalized : `/${normalized}`;
}
