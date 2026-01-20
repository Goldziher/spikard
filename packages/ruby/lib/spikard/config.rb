# frozen_string_literal: true

module Spikard
  # Compression configuration for response compression middleware.
  #
  # Spikard supports gzip and brotli compression for responses.
  # Compression is applied based on Accept-Encoding headers.
  #
  # @example
  #   compression = CompressionConfig.new(
  #     gzip: true,
  #     brotli: true,
  #     min_size: 1024,
  #     quality: 6
  #   )
  class CompressionConfig
    attr_accessor :gzip, :brotli, :min_size, :quality

    # @param gzip [Boolean] Enable gzip compression (default: true)
    # @param brotli [Boolean] Enable brotli compression (default: true)
    # @param min_size [Integer] Minimum response size in bytes to compress (default: 1024)
    # @param quality [Integer] Compression quality level (0-11 for brotli, 0-9 for gzip, default: 6)
    def initialize(gzip: true, brotli: true, min_size: 1024, quality: 6)
      @gzip = normalize_boolean('gzip', gzip)
      @brotli = normalize_boolean('brotli', brotli)
      @min_size = normalize_nonnegative_integer('min_size', min_size)
      @quality = normalize_quality(quality)
    end

    private

    def normalize_boolean(name, value)
      return value if value == true || value == false

      raise ArgumentError, "#{name} must be a boolean"
    end

    def normalize_nonnegative_integer(name, value)
      unless value.is_a?(Integer)
        raise ArgumentError, "#{name} must be an Integer"
      end
      return value if value >= 0

      raise ArgumentError, "#{name} must be >= 0"
    end

    def normalize_quality(value)
      unless value.is_a?(Integer) || value.is_a?(Float)
        raise ArgumentError, 'quality must be a number'
      end

      normalized = value.to_i
      return normalized if normalized.between?(0, 11)

      raise ArgumentError, 'quality must be between 0 and 11'
    end
  end

  # Rate limiting configuration using Generic Cell Rate Algorithm (GCRA).
  #
  # By default, rate limits are applied per IP address.
  #
  # @example
  #   rate_limit = RateLimitConfig.new(
  #     per_second: 100,
  #     burst: 200,
  #     ip_based: true
  #   )
  class RateLimitConfig
    attr_accessor :per_second, :burst, :ip_based

    # @param per_second [Integer] Maximum requests per second
    # @param burst [Integer] Burst allowance - allows temporary spikes
    # @param ip_based [Boolean] Apply rate limits per IP address (default: true)
    def initialize(per_second:, burst:, ip_based: true)
      @per_second = per_second
      @burst = burst
      @ip_based = ip_based
    end
  end

  # JWT authentication configuration.
  #
  # Validates JWT tokens using the specified secret and algorithm.
  # Tokens are expected in the Authorization header as "Bearer <token>".
  #
  # Supported algorithms:
  # - HS256, HS384, HS512 (HMAC with SHA)
  # - RS256, RS384, RS512 (RSA signatures)
  # - ES256, ES384, ES512 (ECDSA signatures)
  # - PS256, PS384, PS512 (RSA-PSS signatures)
  #
  # @example
  #   jwt = JwtConfig.new(
  #     secret: 'your-secret-key',
  #     algorithm: 'HS256',
  #     audience: ['api.example.com'],
  #     issuer: 'auth.example.com',
  #     leeway: 30
  #   )
  class JwtConfig
    attr_accessor :secret, :algorithm, :audience, :issuer, :leeway

    # @param secret [String] Secret key for JWT validation
    # @param algorithm [String] JWT algorithm (default: "HS256")
    # @param audience [Array<String>, nil] Expected audience claim(s)
    # @param issuer [String, nil] Expected issuer claim
    # @param leeway [Integer] Time leeway in seconds for exp/nbf/iat claims (default: 0)
    def initialize(secret:, algorithm: 'HS256', audience: nil, issuer: nil, leeway: 0)
      @secret = secret
      @algorithm = algorithm
      @audience = audience
      @issuer = issuer
      @leeway = leeway
    end
  end

  # API key authentication configuration.
  #
  # Validates API keys from request headers. Keys are matched exactly.
  #
  # @example
  #   api_key = ApiKeyConfig.new(
  #     keys: ['key-1', 'key-2', 'key-3'],
  #     header_name: 'X-API-Key'
  #   )
  class ApiKeyConfig
    attr_accessor :keys, :header_name

    # @param keys [Array<String>] List of valid API keys
    # @param header_name [String] HTTP header name to check for API key (default: "X-API-Key")
    def initialize(keys:, header_name: 'X-API-Key')
      @keys = keys
      @header_name = header_name
    end
  end

  # Static file serving configuration.
  #
  # Serves files from a directory at a given route prefix.
  # Multiple static file configurations can be registered.
  #
  # @example
  #   static = StaticFilesConfig.new(
  #     directory: './public',
  #     route_prefix: '/static',
  #     index_file: true,
  #     cache_control: 'public, max-age=3600'
  #   )
  class StaticFilesConfig
    attr_accessor :directory, :route_prefix, :index_file, :cache_control

    # @param directory [String] Directory path containing static files
    # @param route_prefix [String] URL prefix for serving static files (e.g., "/static")
    # @param index_file [Boolean] Serve index.html for directory requests (default: true)
    # @param cache_control [String, nil] Optional Cache-Control header value (e.g., "public, max-age=3600")
    def initialize(directory:, route_prefix:, index_file: true, cache_control: nil)
      @directory = directory
      @route_prefix = route_prefix
      @index_file = index_file
      @cache_control = cache_control
    end
  end

  # Contact information for OpenAPI documentation.
  #
  # @example
  #   contact = ContactInfo.new(
  #     name: 'API Team',
  #     email: 'api@example.com',
  #     url: 'https://example.com'
  #   )
  class ContactInfo
    attr_accessor :name, :email, :url

    # @param name [String, nil] Name of the contact person/organization
    # @param email [String, nil] Email address for contact
    # @param url [String, nil] URL for contact information
    def initialize(name: nil, email: nil, url: nil)
      @name = name
      @email = email
      @url = url
    end
  end

  # License information for OpenAPI documentation.
  #
  # @example
  #   license = LicenseInfo.new(
  #     name: 'MIT',
  #     url: 'https://opensource.org/licenses/MIT'
  #   )
  class LicenseInfo
    attr_accessor :name, :url

    # @param name [String] License name (e.g., "MIT", "Apache 2.0")
    # @param url [String, nil] URL to the full license text
    def initialize(name:, url: nil)
      @name = name
      @url = url
    end
  end

  # Server information for OpenAPI documentation.
  #
  # Multiple servers can be specified for different environments.
  #
  # @example
  #   server = ServerInfo.new(
  #     url: 'https://api.example.com',
  #     description: 'Production'
  #   )
  class ServerInfo
    attr_accessor :url, :description

    # @param url [String] Server URL (e.g., "https://api.example.com")
    # @param description [String, nil] Description of the server (e.g., "Production", "Staging")
    def initialize(url:, description: nil)
      @url = url
      @description = description
    end
  end

  # Security scheme configuration for OpenAPI documentation.
  #
  # Supports HTTP (Bearer/JWT) and API Key authentication schemes.
  #
  # @example HTTP Bearer
  #   scheme = SecuritySchemeInfo.new(
  #     type: 'http',
  #     scheme: 'bearer',
  #     bearer_format: 'JWT'
  #   )
  #
  # @example API Key
  #   scheme = SecuritySchemeInfo.new(
  #     type: 'apiKey',
  #     location: 'header',
  #     name: 'X-API-Key'
  #   )
  class SecuritySchemeInfo
    attr_accessor :type, :scheme, :bearer_format, :location, :name

    # @param type [String] Security scheme type ("http" or "apiKey")
    # @param scheme [String, nil] HTTP scheme (e.g., "bearer", "basic") - for type="http"
    # @param bearer_format [String, nil] Format hint for Bearer tokens (e.g., "JWT") - for type="http"
    # @param location [String, nil] Where to look for the API key ("header", "query", or "cookie") - for type="apiKey"
    # @param name [String, nil] Parameter name (e.g., "X-API-Key") - for type="apiKey"
    def initialize(type:, scheme: nil, bearer_format: nil, location: nil, name: nil)
      @type = type
      @scheme = scheme
      @bearer_format = bearer_format
      @location = location
      @name = name

      validate!
    end

    private

    def validate!
      case @type
      when 'http'
        raise ArgumentError, 'scheme is required for type="http"' if @scheme.nil?
      when 'apiKey'
        raise ArgumentError, 'location and name are required for type="apiKey"' if @location.nil? || @name.nil?
      else
        raise ArgumentError, "type must be 'http' or 'apiKey', got: #{@type.inspect}"
      end
    end
  end

  # OpenAPI 3.1.0 documentation configuration.
  #
  # Spikard can automatically generate OpenAPI documentation from your routes.
  # When enabled, it serves:
  # - Swagger UI at /docs (customizable)
  # - Redoc at /redoc (customizable)
  # - OpenAPI JSON spec at /openapi.json (customizable)
  #
  # Security schemes are auto-detected from middleware configuration.
  # Schemas are generated from your route type hints and validation.
  #
  # @example
  #   openapi = OpenApiConfig.new(
  #     enabled: true,
  #     title: 'My API',
  #     version: '1.0.0',
  #     description: 'A great API built with Spikard',
  #     contact: ContactInfo.new(
  #       name: 'API Team',
  #       email: 'api@example.com',
  #       url: 'https://example.com'
  #     ),
  #     license: LicenseInfo.new(
  #       name: 'MIT',
  #       url: 'https://opensource.org/licenses/MIT'
  #     ),
  #     servers: [
  #       ServerInfo.new(url: 'https://api.example.com', description: 'Production'),
  #       ServerInfo.new(url: 'http://localhost:8000', description: 'Development')
  #     ]
  #   )
  class OpenApiConfig
    attr_accessor :enabled, :title, :version, :description,
                  :swagger_ui_path, :redoc_path, :openapi_json_path,
                  :contact, :license, :servers, :security_schemes

    # @param enabled [Boolean] Enable OpenAPI generation (default: false for zero overhead)
    # @param title [String] API title (default: "API")
    # @param version [String] API version (default: "1.0.0")
    # @param description [String, nil] API description (supports Markdown)
    # @param swagger_ui_path [String] Path to serve Swagger UI (default: "/docs")
    # @param redoc_path [String] Path to serve Redoc (default: "/redoc")
    # @param openapi_json_path [String] Path to serve OpenAPI JSON spec (default: "/openapi.json")
    # @param contact [ContactInfo, nil] Contact information for the API
    # @param license [LicenseInfo, nil] License information for the API
    # @param servers [Array<ServerInfo>] List of server URLs for different environments (default: [])
    # @param security_schemes [Hash<String, SecuritySchemeInfo>] Custom security schemes (auto-detected if not provided)
    def initialize(
      enabled: false,
      title: 'API',
      version: '1.0.0',
      description: nil,
      swagger_ui_path: '/docs',
      redoc_path: '/redoc',
      openapi_json_path: '/openapi.json',
      contact: nil,
      license: nil,
      servers: [],
      security_schemes: {}
    )
      @enabled = enabled
      @title = title
      @version = version
      @description = description
      @swagger_ui_path = swagger_ui_path
      @redoc_path = redoc_path
      @openapi_json_path = openapi_json_path
      @contact = contact
      @license = license
      @servers = servers
      @security_schemes = security_schemes
    end
  end

  # Complete server configuration for Spikard.
  #
  # This is the main configuration object that controls all aspects of the server
  # including network settings, middleware, authentication, and more.
  #
  # @example
  #   config = ServerConfig.new(
  #     host: '0.0.0.0',
  #     port: 8080,
  #     workers: 4,
  #     compression: CompressionConfig.new(quality: 9),
  #     rate_limit: RateLimitConfig.new(per_second: 100, burst: 200),
  #     static_files: [
  #       StaticFilesConfig.new(
  #         directory: './public',
  #         route_prefix: '/static'
  #       )
  #     ],
  #     openapi: OpenApiConfig.new(
  #       enabled: true,
  #       title: 'My API',
  #       version: '1.0.0'
  #     )
  #   )
  class ServerConfig
    attr_accessor :host, :port, :workers,
                  :enable_request_id, :max_body_size, :request_timeout,
                  :compression, :rate_limit, :jwt_auth, :api_key_auth,
                  :static_files, :graceful_shutdown, :shutdown_timeout,
                  :openapi

    # @param host [String] Host address to bind to (default: "127.0.0.1")
    # @param port [Integer] Port number to listen on (default: 8000, range: 1-65535)
    # @param workers [Integer] Number of worker processes (default: 1)
    # @param enable_request_id [Boolean] Add X-Request-ID header to responses (default: true)
    # @param max_body_size [Integer, nil] Maximum request body size in bytes (default: 10MB, nil for unlimited)
    # @param request_timeout [Integer, nil] Request timeout in seconds (default: 30, nil for no timeout)
    # @param compression [CompressionConfig, nil] Response compression configuration (default: enabled with defaults)
    # @param rate_limit [RateLimitConfig, nil] Rate limiting configuration (default: nil/disabled)
    # @param jwt_auth [JwtConfig, nil] JWT authentication configuration (default: nil/disabled)
    # @param api_key_auth [ApiKeyConfig, nil] API key authentication configuration (default: nil/disabled)
    # @param static_files [Array<StaticFilesConfig>] List of static file serving configurations (default: [])
    # @param graceful_shutdown [Boolean] Enable graceful shutdown (default: true)
    # @param shutdown_timeout [Integer] Graceful shutdown timeout in seconds (default: 30)
    # @param openapi [OpenApiConfig, nil] OpenAPI configuration (default: nil/disabled)
    def initialize(
      host: '127.0.0.1',
      port: 8000,
      workers: 1,
      enable_request_id: true,
      max_body_size: 10 * 1024 * 1024, # 10MB
      request_timeout: 30,
      compression: CompressionConfig.new,
      rate_limit: nil,
      jwt_auth: nil,
      api_key_auth: nil,
      static_files: [],
      graceful_shutdown: true,
      shutdown_timeout: 30,
      openapi: nil
    )
      @host = host
      @port = normalize_port(port)
      @workers = normalize_workers(workers)
      @enable_request_id = normalize_boolean('enable_request_id', enable_request_id)
      @max_body_size = normalize_optional_nonnegative_integer('max_body_size', max_body_size)
      @request_timeout = normalize_timeout('request_timeout', request_timeout)
      @compression = compression
      @rate_limit = rate_limit
      @jwt_auth = jwt_auth
      @api_key_auth = api_key_auth
      @static_files = normalize_static_files(static_files)
      @graceful_shutdown = normalize_boolean('graceful_shutdown', graceful_shutdown)
      @shutdown_timeout = normalize_timeout('shutdown_timeout', shutdown_timeout)
      @openapi = openapi
    end

    private

    def normalize_port(port)
      unless port.is_a?(Integer)
        raise ArgumentError, 'port must be an Integer'
      end
      return port if port.between?(1, 65_535)

      raise ArgumentError, 'port must be between 1 and 65535'
    end

    def normalize_workers(workers)
      unless workers.is_a?(Integer)
        raise ArgumentError, 'workers must be an Integer'
      end
      return workers if workers >= 1

      raise ArgumentError, 'workers must be >= 1'
    end

    def normalize_boolean(name, value)
      return value if value == true || value == false

      raise ArgumentError, "#{name} must be a boolean"
    end

    def normalize_optional_nonnegative_integer(name, value)
      return nil if value.nil?
      unless value.is_a?(Integer)
        raise ArgumentError, "#{name} must be an Integer"
      end
      return value if value >= 0

      raise ArgumentError, "#{name} must be >= 0"
    end

    def normalize_timeout(name, value)
      return nil if value.nil?
      unless value.is_a?(Integer) || value.is_a?(Float)
        raise ArgumentError, "#{name} must be a number"
      end

      normalized = value.to_i
      return normalized if normalized >= 0

      raise ArgumentError, "#{name} must be >= 0"
    end

    def normalize_static_files(static_files)
      return [] if static_files.nil?
      unless static_files.is_a?(Array)
        raise ArgumentError, 'static_files must be an Array'
      end

      static_files.each do |entry|
        next if entry.is_a?(StaticFilesConfig)

        raise ArgumentError, 'static_files entries must be StaticFilesConfig'
      end
      static_files
    end
  end
end
