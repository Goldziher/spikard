/**
 * Configuration types for Spikard Node.js bindings
 *
 * These types mirror the Rust ServerConfig and related configuration structs.
 */

/**
 * Compression configuration for response compression middleware.
 *
 * Spikard supports gzip and brotli compression for responses.
 * Compression is applied based on Accept-Encoding headers.
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
 * Rate limiting configuration using Generic Cell Rate Algorithm (GCRA).
 *
 * By default, rate limits are applied per IP address.
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
 * JWT authentication configuration.
 *
 * Validates JWT tokens using the specified secret and algorithm.
 * Tokens are expected in the Authorization header as "Bearer <token>".
 *
 * Supported algorithms:
 * - HS256, HS384, HS512 (HMAC with SHA)
 * - RS256, RS384, RS512 (RSA signatures)
 * - ES256, ES384, ES512 (ECDSA signatures)
 * - PS256, PS384, PS512 (RSA-PSS signatures)
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
 * API key authentication configuration.
 *
 * Validates API keys from request headers. Keys are matched exactly.
 */
export interface ApiKeyConfig {
	/** List of valid API keys */
	keys: string[];
	/** HTTP header name to check for API key (default: "X-API-Key") */
	headerName?: string;
}

/**
 * Static file serving configuration.
 *
 * Serves files from a directory at a given route prefix.
 * Multiple static file configurations can be registered.
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

/**
 * Contact information for OpenAPI documentation.
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
 */
export type SecuritySchemeInfo =
	| {
			/** Security scheme type */
			type: "http";
			/** HTTP scheme (e.g., "bearer", "basic") */
			scheme: string;
			/** Format hint for Bearer tokens (e.g., "JWT") */
			bearerFormat?: string;
	  }
	| {
			/** Security scheme type */
			type: "apiKey";
			/** Where to look for the API key: "header", "query", or "cookie" */
			location: "header" | "query" | "cookie";
			/** Parameter name (e.g., "X-API-Key") */
			name: string;
	  };

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
 *     url: "https://example.com"
 *   },
 *   license: {
 *     name: "MIT",
 *     url: "https://opensource.org/licenses/MIT"
 *   },
 *   servers: [
 *     { url: "https://api.example.com", description: "Production" },
 *     { url: "http://localhost:8000", description: "Development" }
 *   ]
 * };
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
 * Complete server configuration for Spikard.
 *
 * This is the main configuration object that controls all aspects of the server
 * including network settings, middleware, authentication, and more.
 *
 * @example
 * ```typescript
 * import { Spikard } from '@spikard/node';
 *
 * const config: ServerConfig = {
 *   host: "0.0.0.0",
 *   port: 8080,
 *   workers: 4,
 *   compression: {
 *     quality: 9
 *   },
 *   rateLimit: {
 *     perSecond: 100,
 *     burst: 200
 *   },
 *   staticFiles: [
 *     {
 *       directory: "./public",
 *       routePrefix: "/static"
 *     }
 *   ],
 *   openapi: {
 *     enabled: true,
 *     title: "My API",
 *     version: "1.0.0"
 *   }
 * };
 *
 * const app = new Spikard(config);
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

	/** Response compression configuration (default: enabled with defaults) */
	compression?: CompressionConfig | null;
	/** Rate limiting configuration (default: null/disabled) */
	rateLimit?: RateLimitConfig | null;
	/** JWT authentication configuration (default: null/disabled) */
	jwtAuth?: JwtConfig | null;
	/** API key authentication configuration (default: null/disabled) */
	apiKeyAuth?: ApiKeyConfig | null;
	/** List of static file serving configurations (default: empty array) */
	staticFiles?: StaticFilesConfig[];

	/** Enable graceful shutdown (default: true) */
	gracefulShutdown?: boolean;
	/** Graceful shutdown timeout in seconds (default: 30) */
	shutdownTimeout?: number;

	/** OpenAPI configuration (default: null/disabled) */
	openapi?: OpenApiConfig | null;
}
