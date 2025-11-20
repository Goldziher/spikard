/**
 * Configuration types for Spikard server.
 *
 * All configuration interfaces mirror the Python dataclass equivalents
 * and map to the Rust ServerConfig structure.
 */

/**
 * Configuration for response compression middleware.
 *
 * Spikard supports gzip and brotli compression for responses.
 * Compression is applied based on Accept-Encoding headers.
 *
 * @example
 * ```typescript
 * const compression: CompressionConfig = {
 *   gzip: true,
 *   brotli: true,
 *   minSize: 2048,  // Only compress responses >= 2KB
 *   quality: 9,     // Maximum compression
 * };
 * ```
 */
export interface CompressionConfig {
	/** Enable gzip compression (default: true) */
	gzip?: boolean;
	/** Enable brotli compression (default: true) */
	brotli?: boolean;
	/** Minimum response size in bytes to compress (default: 1024) */
	minSize?: number;
	/** Compression quality level (0-11 for brotli, 0-9 for gzip, default: 6) */
	quality?: number;
}

/**
 * Configuration for rate limiting middleware.
 *
 * Uses the Generic Cell Rate Algorithm (GCRA) for smooth rate limiting.
 * By default, rate limits are applied per IP address.
 *
 * @example
 * ```typescript
 * const rateLimit: RateLimitConfig = {
 *   perSecond: 10,  // 10 requests per second
 *   burst: 20,      // Allow bursts up to 20
 *   ipBased: true,  // Per IP address
 * };
 * ```
 */
export interface RateLimitConfig {
	/** Maximum requests per second */
	perSecond: number;
	/** Burst allowance - allows temporary spikes */
	burst: number;
	/** Apply rate limits per IP address (default: true) */
	ipBased?: boolean;
}

/**
 * Configuration for JWT authentication middleware.
 *
 * Validates JWT tokens using the specified secret and algorithm.
 * Tokens are expected in the Authorization header as "Bearer <token>".
 *
 * Supported algorithms:
 * - HS256, HS384, HS512 (HMAC with SHA)
 * - RS256, RS384, RS512 (RSA signatures)
 * - ES256, ES384, ES512 (ECDSA signatures)
 * - PS256, PS384, PS512 (RSA-PSS signatures)
 *
 * @example
 * ```typescript
 * const jwtAuth: JwtConfig = {
 *   secret: "your-secret-key",
 *   algorithm: "HS256",
 *   audience: ["https://api.example.com"],
 *   issuer: "https://auth.example.com",
 *   leeway: 10,
 * };
 * ```
 */
export interface JwtConfig {
	/** Secret key for JWT validation */
	secret: string;
	/** JWT algorithm (default: "HS256") */
	algorithm?: string;
	/** Expected audience claim(s) */
	audience?: string[];
	/** Expected issuer claim */
	issuer?: string;
	/** Time leeway in seconds for exp/nbf/iat claims (default: 0) */
	leeway?: number;
}

/**
 * Configuration for API key authentication middleware.
 *
 * Validates API keys from request headers. Keys are matched exactly.
 *
 * @example
 * ```typescript
 * const apiKeyAuth: ApiKeyConfig = {
 *   keys: ["secret-key-1", "secret-key-2"],
 *   headerName: "X-API-Key",
 * };
 * ```
 */
export interface ApiKeyConfig {
	/** List of valid API keys */
	keys: string[];
	/** HTTP header name to check for API key (default: "X-API-Key") */
	headerName?: string;
}

/**
 * Contact information for OpenAPI documentation.
 *
 * @example
 * ```typescript
 * const contact: ContactInfo = {
 *   name: "API Support",
 *   email: "support@example.com",
 *   url: "https://example.com/support",
 * };
 * ```
 */
export interface ContactInfo {
	/** Name of the contact person/organization */
	name?: string;
	/** Email address for contact */
	email?: string;
	/** URL for contact information */
	url?: string;
}

/**
 * License information for OpenAPI documentation.
 *
 * @example
 * ```typescript
 * const license: LicenseInfo = {
 *   name: "MIT",
 *   url: "https://opensource.org/licenses/MIT",
 * };
 * ```
 */
export interface LicenseInfo {
	/** License name (e.g., "MIT", "Apache 2.0") */
	name: string;
	/** URL to the full license text */
	url?: string;
}

/**
 * Server information for OpenAPI documentation.
 *
 * Multiple servers can be specified for different environments.
 *
 * @example
 * ```typescript
 * const servers: ServerInfo[] = [
 *   { url: "https://api.example.com", description: "Production" },
 *   { url: "http://localhost:8000", description: "Development" },
 * ];
 * ```
 */
export interface ServerInfo {
	/** Server URL (e.g., "https://api.example.com") */
	url: string;
	/** Description of the server (e.g., "Production", "Staging") */
	description?: string;
}

/**
 * Security scheme configuration for OpenAPI documentation.
 *
 * Supports HTTP (Bearer/JWT) and API Key authentication schemes.
 * Security schemes are auto-detected from middleware config if not explicitly provided.
 *
 * @example
 * ```typescript
 * // JWT/Bearer authentication
 * const jwtScheme: SecuritySchemeInfo = {
 *   type: "http",
 *   scheme: "bearer",
 *   bearerFormat: "JWT",
 * };
 *
 * // API Key authentication
 * const apiKeyScheme: SecuritySchemeInfo = {
 *   type: "apiKey",
 *   location: "header",
 *   name: "X-API-Key",
 * };
 * ```
 */
export interface SecuritySchemeInfo {
	/** Scheme type - "http" or "apiKey" */
	type: "http" | "apiKey";
	/** For HTTP type - "bearer", "basic", etc. */
	scheme?: string;
	/** For HTTP Bearer - format hint like "JWT" */
	bearerFormat?: string;
	/** For API key - "header", "query", or "cookie" */
	location?: "header" | "query" | "cookie";
	/** For API key - parameter name (e.g., "X-API-Key") */
	name?: string;
}

/**
 * OpenAPI 3.1.0 documentation configuration.
 *
 * Spikard can automatically generate OpenAPI documentation from your routes.
 * When enabled, it serves:
 * - Swagger UI at /docs (customizable)
 * - Redoc at /redoc (customizable)
 * - OpenAPI JSON spec at /openapi.json (customizable)
 *
 * Security schemes are auto-detected from middleware configuration.
 * Schemas are generated from your route type hints and validation.
 *
 * @example
 * ```typescript
 * const openapi: OpenApiConfig = {
 *   enabled: true,
 *   title: "My API",
 *   version: "1.0.0",
 *   description: "A great API built with Spikard",
 *   contact: {
 *     name: "API Team",
 *     email: "api@example.com",
 *     url: "https://example.com",
 *   },
 *   license: {
 *     name: "MIT",
 *     url: "https://opensource.org/licenses/MIT",
 *   },
 *   servers: [
 *     { url: "https://api.example.com", description: "Production" },
 *     { url: "http://localhost:8000", description: "Development" },
 *   ],
 * };
 *
 * // Swagger UI available at http://localhost:8000/docs
 * // Redoc available at http://localhost:8000/redoc
 * // OpenAPI spec at http://localhost:8000/openapi.json
 * ```
 */
export interface OpenApiConfig {
	/** Enable OpenAPI generation (default: false for zero overhead) */
	enabled?: boolean;
	/** API title (required if enabled) */
	title?: string;
	/** API version (required if enabled) */
	version?: string;
	/** API description (supports Markdown) */
	description?: string;
	/** Path to serve Swagger UI (default: "/docs") */
	swaggerUiPath?: string;
	/** Path to serve Redoc (default: "/redoc") */
	redocPath?: string;
	/** Path to serve OpenAPI JSON spec (default: "/openapi.json") */
	openapiJsonPath?: string;
	/** Contact information for the API */
	contact?: ContactInfo;
	/** License information for the API */
	license?: LicenseInfo;
	/** List of server URLs for different environments */
	servers?: ServerInfo[];
	/** Custom security schemes (auto-detected if not provided) */
	securitySchemes?: Record<string, SecuritySchemeInfo>;
}

/**
 * Configuration for serving static files.
 *
 * Serves files from a directory at a given route prefix.
 * Multiple static file configurations can be registered.
 *
 * @example
 * ```typescript
 * const staticFiles: StaticFilesConfig[] = [
 *   {
 *     directory: "./public",
 *     routePrefix: "/static",
 *     indexFile: true,
 *     cacheControl: "public, max-age=3600",
 *   },
 *   {
 *     directory: "./assets",
 *     routePrefix: "/assets",
 *     cacheControl: "public, max-age=86400",
 *   },
 * ];
 * ```
 */
export interface StaticFilesConfig {
	/** Directory path containing static files */
	directory: string;
	/** URL prefix for serving static files (e.g., "/static") */
	routePrefix: string;
	/** Serve index.html for directory requests (default: true) */
	indexFile?: boolean;
	/** Optional Cache-Control header value (e.g., "public, max-age=3600") */
	cacheControl?: string;
}

/** @internal */
export interface StaticManifestEntry {
	route: string;
	headers: Record<string, string>;
	body: string;
}

/**
 * Complete server configuration for Spikard.
 *
 * This is the main configuration object that controls all aspects of the server
 * including network settings, middleware, authentication, and more.
 *
 * Network Configuration:
 * - host: Host address to bind to (default: "127.0.0.1")
 * - port: Port number to listen on (default: 8000, range: 1-65535)
 * - workers: Number of worker processes (default: 1)
 *
 * Request Handling:
 * - enableRequestId: Add X-Request-ID header to responses (default: true)
 * - maxBodySize: Maximum request body size in bytes (default: 10MB, 0 or null for unlimited)
 * - requestTimeout: Request timeout in seconds (default: 30, null for no timeout)
 *
 * Middleware:
 * - compression: Response compression configuration (default: enabled with defaults)
 * - rateLimit: Rate limiting configuration (default: null/disabled)
 * - jwtAuth: JWT authentication configuration (default: null/disabled)
 * - apiKeyAuth: API key authentication configuration (default: null/disabled)
 * - staticFiles: List of static file serving configurations (default: empty array)
 *
 * Lifecycle:
 * - gracefulShutdown: Enable graceful shutdown (default: true)
 * - shutdownTimeout: Graceful shutdown timeout in seconds (default: 30)
 *
 * OpenAPI/Documentation:
 * - openapi: OpenAPI configuration (default: null/disabled)
 *
 * @example
 * ```typescript
 * import { Spikard, runServer } from '@spikard/node';
 * import type { ServerConfig } from '@spikard/node';
 *
 * const config: ServerConfig = {
 *   host: "0.0.0.0",
 *   port: 8080,
 *   workers: 4,
 *   compression: { quality: 9 },
 *   rateLimit: { perSecond: 100, burst: 200 },
 *   staticFiles: [{ directory: "./public", routePrefix: "/static" }],
 *   openapi: {
 *     enabled: true,
 *     title: "My API",
 *     version: "1.0.0",
 *   },
 * };
 *
 * const app = new Spikard();
 * // Register routes...
 * runServer(app, config);
 * ```
 */
export interface ServerConfig {
	/** Host address to bind to (default: "127.0.0.1") */
	host?: string;
	/** Port number to listen on (default: 8000, range: 1-65535) */
	port?: number;
	/** Number of worker processes (default: 1) */
	workers?: number;

	/** Add X-Request-ID header to responses (default: true) */
	enableRequestId?: boolean;
	/** Maximum request body size in bytes (default: 10MB, 0 or null for unlimited) */
	maxBodySize?: number | null;
	/** Request timeout in seconds (default: 30, null for no timeout) */
	requestTimeout?: number | null;

	/** Response compression configuration */
	compression?: CompressionConfig | null;
	/** Rate limiting configuration */
	rateLimit?: RateLimitConfig | null;
	/** JWT authentication configuration */
	jwtAuth?: JwtConfig | null;
	/** API key authentication configuration */
	apiKeyAuth?: ApiKeyConfig | null;
	/** Static file serving configurations */
	staticFiles?: StaticFilesConfig[];
	/** @internal Precomputed static manifest for wasm runtimes */
	__wasmStaticManifest?: StaticManifestEntry[];

	/** Enable graceful shutdown (default: true) */
	gracefulShutdown?: boolean;
	/** Graceful shutdown timeout in seconds (default: 30) */
	shutdownTimeout?: number;

	/** OpenAPI documentation configuration */
	openapi?: OpenApiConfig | null;
}
