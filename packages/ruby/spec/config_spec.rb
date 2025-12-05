# frozen_string_literal: true

require 'spec_helper'

RSpec.describe 'Spikard Config Classes' do
  describe Spikard::CompressionConfig do
    describe 'initialization with defaults' do
      subject(:config) { described_class.new }

      it 'enables gzip compression by default' do
        expect(config.gzip).to be true
      end

      it 'enables brotli compression by default' do
        expect(config.brotli).to be true
      end

      it 'sets minimum size to 1024 bytes by default' do
        expect(config.min_size).to eq(1024)
      end

      it 'sets quality to 6 by default' do
        expect(config.quality).to eq(6)
      end
    end

    describe 'initialization with custom parameters' do
      it 'accepts custom gzip setting' do
        config = described_class.new(gzip: false)
        expect(config.gzip).to be false
      end

      it 'accepts custom brotli setting' do
        config = described_class.new(brotli: false)
        expect(config.brotli).to be false
      end

      it 'accepts custom minimum size' do
        config = described_class.new(min_size: 2048)
        expect(config.min_size).to eq(2048)
      end

      it 'accepts custom quality level' do
        config = described_class.new(quality: 9)
        expect(config.quality).to eq(9)
      end

      it 'accepts all custom parameters together' do
        config = described_class.new(
          gzip: false,
          brotli: true,
          min_size: 4096,
          quality: 11
        )

        expect(config.gzip).to be false
        expect(config.brotli).to be true
        expect(config.min_size).to eq(4096)
        expect(config.quality).to eq(11)
      end
    end

    describe 'attribute accessors' do
      let(:config) { described_class.new }

      it 'allows gzip to be modified' do
        config.gzip = false
        expect(config.gzip).to be false
      end

      it 'allows brotli to be modified' do
        config.brotli = false
        expect(config.brotli).to be false
      end

      it 'allows min_size to be modified' do
        config.min_size = 5000
        expect(config.min_size).to eq(5000)
      end

      it 'allows quality to be modified' do
        config.quality = 8
        expect(config.quality).to eq(8)
      end
    end
  end

  describe Spikard::RateLimitConfig do
    describe 'required parameters' do
      it 'requires per_second parameter' do
        expect { described_class.new(burst: 100) }.to raise_error(ArgumentError)
      end

      it 'requires burst parameter' do
        expect { described_class.new(per_second: 100) }.to raise_error(ArgumentError)
      end
    end

    describe 'initialization with parameters' do
      it 'sets per_second from parameter' do
        config = described_class.new(per_second: 100, burst: 200)
        expect(config.per_second).to eq(100)
      end

      it 'sets burst from parameter' do
        config = described_class.new(per_second: 100, burst: 200)
        expect(config.burst).to eq(200)
      end

      it 'enables IP-based rate limiting by default' do
        config = described_class.new(per_second: 100, burst: 200)
        expect(config.ip_based).to be true
      end

      it 'allows IP-based to be customized' do
        config = described_class.new(per_second: 100, burst: 200, ip_based: false)
        expect(config.ip_based).to be false
      end
    end

    describe 'attribute accessors' do
      let(:config) { described_class.new(per_second: 100, burst: 200) }

      it 'allows per_second to be modified' do
        config.per_second = 150
        expect(config.per_second).to eq(150)
      end

      it 'allows burst to be modified' do
        config.burst = 300
        expect(config.burst).to eq(300)
      end

      it 'allows ip_based to be modified' do
        config.ip_based = false
        expect(config.ip_based).to be false
      end
    end

    describe 'various rate limit scenarios' do
      it 'supports low rate limits' do
        config = described_class.new(per_second: 1, burst: 5)
        expect(config.per_second).to eq(1)
        expect(config.burst).to eq(5)
      end

      it 'supports high rate limits' do
        config = described_class.new(per_second: 10_000, burst: 50_000)
        expect(config.per_second).to eq(10_000)
        expect(config.burst).to eq(50_000)
      end
    end
  end

  describe Spikard::StaticFilesConfig do
    describe 'required parameters' do
      it 'requires directory parameter' do
        expect { described_class.new(route_prefix: '/static') }.to raise_error(ArgumentError)
      end

      it 'requires route_prefix parameter' do
        expect { described_class.new(directory: './public') }.to raise_error(ArgumentError)
      end
    end

    describe 'initialization with parameters' do
      subject(:config) do
        described_class.new(directory: './public', route_prefix: '/static')
      end

      it 'sets directory from parameter' do
        expect(config.directory).to eq('./public')
      end

      it 'sets route_prefix from parameter' do
        expect(config.route_prefix).to eq('/static')
      end

      it 'enables index file serving by default' do
        expect(config.index_file).to be true
      end

      it 'sets no cache control by default' do
        expect(config.cache_control).to be_nil
      end
    end

    describe 'initialization with optional parameters' do
      it 'accepts custom index_file setting' do
        config = described_class.new(
          directory: './public',
          route_prefix: '/static',
          index_file: false
        )
        expect(config.index_file).to be false
      end

      it 'accepts custom cache_control header' do
        config = described_class.new(
          directory: './public',
          route_prefix: '/static',
          cache_control: 'public, max-age=3600'
        )
        expect(config.cache_control).to eq('public, max-age=3600')
      end

      it 'accepts all custom parameters together' do
        config = described_class.new(
          directory: './uploads',
          route_prefix: '/files',
          index_file: false,
          cache_control: 'private, max-age=0'
        )

        expect(config.directory).to eq('./uploads')
        expect(config.route_prefix).to eq('/files')
        expect(config.index_file).to be false
        expect(config.cache_control).to eq('private, max-age=0')
      end
    end

    describe 'attribute accessors' do
      let(:config) do
        described_class.new(directory: './public', route_prefix: '/static')
      end

      it 'allows directory to be modified' do
        config.directory = './assets'
        expect(config.directory).to eq('./assets')
      end

      it 'allows route_prefix to be modified' do
        config.route_prefix = '/assets'
        expect(config.route_prefix).to eq('/assets')
      end

      it 'allows index_file to be modified' do
        config.index_file = false
        expect(config.index_file).to be false
      end

      it 'allows cache_control to be modified' do
        config.cache_control = 'max-age=86400'
        expect(config.cache_control).to eq('max-age=86400')
      end
    end

    describe 'various directory and route scenarios' do
      it 'handles absolute paths' do
        config = described_class.new(
          directory: '/var/www/html',
          route_prefix: '/static'
        )
        expect(config.directory).to eq('/var/www/html')
      end

      it 'handles nested route prefixes' do
        config = described_class.new(
          directory: './public',
          route_prefix: '/api/v1/static'
        )
        expect(config.route_prefix).to eq('/api/v1/static')
      end

      it 'handles various cache control values' do
        values = [
          'public, max-age=3600',
          'private, no-cache',
          'max-age=0, must-revalidate',
          'immutable'
        ]

        values.each do |cache_value|
          config = described_class.new(
            directory: './public',
            route_prefix: '/static',
            cache_control: cache_value
          )
          expect(config.cache_control).to eq(cache_value)
        end
      end
    end
  end

  describe Spikard::ContactInfo do
    describe 'initialization with defaults' do
      subject(:contact) { described_class.new }

      it 'sets name to nil by default' do
        expect(contact.name).to be_nil
      end

      it 'sets email to nil by default' do
        expect(contact.email).to be_nil
      end

      it 'sets url to nil by default' do
        expect(contact.url).to be_nil
      end
    end

    describe 'initialization with parameters' do
      it 'accepts name parameter' do
        contact = described_class.new(name: 'API Team')
        expect(contact.name).to eq('API Team')
      end

      it 'accepts email parameter' do
        contact = described_class.new(email: 'api@example.com')
        expect(contact.email).to eq('api@example.com')
      end

      it 'accepts url parameter' do
        contact = described_class.new(url: 'https://example.com')
        expect(contact.url).to eq('https://example.com')
      end

      it 'accepts all parameters' do
        contact = described_class.new(
          name: 'API Team',
          email: 'api@example.com',
          url: 'https://example.com'
        )

        expect(contact.name).to eq('API Team')
        expect(contact.email).to eq('api@example.com')
        expect(contact.url).to eq('https://example.com')
      end
    end

    describe 'attribute accessors' do
      let(:contact) { described_class.new }

      it 'allows name to be modified' do
        contact.name = 'Support Team'
        expect(contact.name).to eq('Support Team')
      end

      it 'allows email to be modified' do
        contact.email = 'support@example.com'
        expect(contact.email).to eq('support@example.com')
      end

      it 'allows url to be modified' do
        contact.url = 'https://support.example.com'
        expect(contact.url).to eq('https://support.example.com')
      end
    end
  end

  describe Spikard::LicenseInfo do
    describe 'required parameters' do
      it 'requires name parameter' do
        expect { described_class.new }.to raise_error(ArgumentError)
      end
    end

    describe 'initialization with parameters' do
      it 'sets name from parameter' do
        license = described_class.new(name: 'MIT')
        expect(license.name).to eq('MIT')
      end

      it 'sets url to nil by default' do
        license = described_class.new(name: 'MIT')
        expect(license.url).to be_nil
      end

      it 'accepts url parameter' do
        license = described_class.new(
          name: 'MIT',
          url: 'https://opensource.org/licenses/MIT'
        )
        expect(license.url).to eq('https://opensource.org/licenses/MIT')
      end
    end

    describe 'attribute accessors' do
      let(:license) { described_class.new(name: 'Apache 2.0') }

      it 'allows name to be modified' do
        license.name = 'GPL-3.0'
        expect(license.name).to eq('GPL-3.0')
      end

      it 'allows url to be modified' do
        license.url = 'https://www.gnu.org/licenses/gpl-3.0.html'
        expect(license.url).to eq('https://www.gnu.org/licenses/gpl-3.0.html')
      end
    end

    describe 'various license types' do
      %w[MIT Apache-2.0 GPL-3.0 BSD-3-Clause ISC].each do |license_type|
        it "supports #{license_type} license" do
          license = described_class.new(name: license_type)
          expect(license.name).to eq(license_type)
        end
      end
    end
  end

  describe Spikard::ServerInfo do
    describe 'required parameters' do
      it 'requires url parameter' do
        expect { described_class.new }.to raise_error(ArgumentError)
      end
    end

    describe 'initialization with parameters' do
      it 'sets url from parameter' do
        server = described_class.new(url: 'https://api.example.com')
        expect(server.url).to eq('https://api.example.com')
      end

      it 'sets description to nil by default' do
        server = described_class.new(url: 'https://api.example.com')
        expect(server.description).to be_nil
      end

      it 'accepts description parameter' do
        server = described_class.new(
          url: 'https://api.example.com',
          description: 'Production'
        )
        expect(server.description).to eq('Production')
      end
    end

    describe 'attribute accessors' do
      let(:server) { described_class.new(url: 'https://api.example.com') }

      it 'allows url to be modified' do
        server.url = 'https://staging-api.example.com'
        expect(server.url).to eq('https://staging-api.example.com')
      end

      it 'allows description to be modified' do
        server.description = 'Staging'
        expect(server.description).to eq('Staging')
      end
    end

    describe 'various server configurations' do
      it 'supports production server' do
        server = described_class.new(
          url: 'https://api.example.com',
          description: 'Production'
        )
        expect(server.url).to eq('https://api.example.com')
        expect(server.description).to eq('Production')
      end

      it 'supports localhost server' do
        server = described_class.new(
          url: 'http://localhost:8000',
          description: 'Development'
        )
        expect(server.url).to eq('http://localhost:8000')
        expect(server.description).to eq('Development')
      end

      it 'supports multiple environment servers' do
        servers = [
          described_class.new(url: 'https://api.example.com', description: 'Production'),
          described_class.new(url: 'https://staging-api.example.com', description: 'Staging'),
          described_class.new(url: 'http://localhost:8000', description: 'Development')
        ]

        expect(servers.count).to eq(3)
        expect(servers[0].description).to eq('Production')
        expect(servers[1].description).to eq('Staging')
        expect(servers[2].description).to eq('Development')
      end
    end
  end

  describe Spikard::SecuritySchemeInfo do
    describe 'HTTP bearer scheme' do
      it 'requires scheme for HTTP type' do
        expect do
          described_class.new(type: 'http')
        end.to raise_error(ArgumentError, /scheme is required/)
      end

      it 'creates HTTP bearer scheme' do
        scheme = described_class.new(type: 'http', scheme: 'bearer')
        expect(scheme.type).to eq('http')
        expect(scheme.scheme).to eq('bearer')
      end

      it 'accepts bearer_format for JWT' do
        scheme = described_class.new(
          type: 'http',
          scheme: 'bearer',
          bearer_format: 'JWT'
        )
        expect(scheme.bearer_format).to eq('JWT')
      end

      it 'accepts basic scheme' do
        scheme = described_class.new(type: 'http', scheme: 'basic')
        expect(scheme.scheme).to eq('basic')
      end
    end

    describe 'API Key scheme' do
      it 'requires location and name for apiKey type' do
        expect do
          described_class.new(type: 'apiKey')
        end.to raise_error(ArgumentError, /location and name are required/)
      end

      it 'requires location specifically' do
        expect do
          described_class.new(type: 'apiKey', name: 'X-API-Key')
        end.to raise_error(ArgumentError, /location and name are required/)
      end

      it 'requires name specifically' do
        expect do
          described_class.new(type: 'apiKey', location: 'header')
        end.to raise_error(ArgumentError, /location and name are required/)
      end

      it 'creates header-based API key scheme' do
        scheme = described_class.new(
          type: 'apiKey',
          location: 'header',
          name: 'X-API-Key'
        )
        expect(scheme.type).to eq('apiKey')
        expect(scheme.location).to eq('header')
        expect(scheme.name).to eq('X-API-Key')
      end

      it 'creates query-based API key scheme' do
        scheme = described_class.new(
          type: 'apiKey',
          location: 'query',
          name: 'api_key'
        )
        expect(scheme.location).to eq('query')
        expect(scheme.name).to eq('api_key')
      end

      it 'creates cookie-based API key scheme' do
        scheme = described_class.new(
          type: 'apiKey',
          location: 'cookie',
          name: 'session_token'
        )
        expect(scheme.location).to eq('cookie')
        expect(scheme.name).to eq('session_token')
      end
    end

    describe 'validation' do
      it 'rejects invalid type' do
        expect do
          described_class.new(type: 'oauth2')
        end.to raise_error(ArgumentError, /type must be 'http' or 'apiKey'/)
      end

      it 'rejects unknown type with helpful message' do
        expect do
          described_class.new(type: 'unknown')
        end.to raise_error(ArgumentError, /type must be 'http' or 'apiKey'/)
      end
    end

    describe 'attribute accessors' do
      let(:scheme) do
        described_class.new(type: 'http', scheme: 'bearer', bearer_format: 'JWT')
      end

      it 'allows type to be read' do
        expect(scheme.type).to eq('http')
      end

      it 'allows scheme to be read' do
        expect(scheme.scheme).to eq('bearer')
      end

      it 'allows bearer_format to be read' do
        expect(scheme.bearer_format).to eq('JWT')
      end
    end
  end

  describe Spikard::OpenApiConfig do
    describe 'initialization with defaults' do
      subject(:config) { described_class.new }

      it 'disables OpenAPI by default' do
        expect(config.enabled).to be false
      end

      it 'sets title to "API" by default' do
        expect(config.title).to eq('API')
      end

      it 'sets version to "1.0.0" by default' do
        expect(config.version).to eq('1.0.0')
      end

      it 'sets description to nil by default' do
        expect(config.description).to be_nil
      end

      it 'sets swagger_ui_path to /docs by default' do
        expect(config.swagger_ui_path).to eq('/docs')
      end

      it 'sets redoc_path to /redoc by default' do
        expect(config.redoc_path).to eq('/redoc')
      end

      it 'sets openapi_json_path to /openapi.json by default' do
        expect(config.openapi_json_path).to eq('/openapi.json')
      end

      it 'sets contact to nil by default' do
        expect(config.contact).to be_nil
      end

      it 'sets license to nil by default' do
        expect(config.license).to be_nil
      end

      it 'sets servers to empty array by default' do
        expect(config.servers).to eq([])
      end

      it 'sets security_schemes to empty hash by default' do
        expect(config.security_schemes).to eq({})
      end
    end

    describe 'initialization with enabled flag' do
      it 'creates disabled config with minimal parameters' do
        config = described_class.new(enabled: false)
        expect(config.enabled).to be false
      end

      it 'creates enabled config with required parameters' do
        config = described_class.new(
          enabled: true,
          title: 'My API',
          version: '2.0.0'
        )

        expect(config.enabled).to be true
        expect(config.title).to eq('My API')
        expect(config.version).to eq('2.0.0')
      end
    end

    describe 'initialization with custom parameters' do
      it 'accepts custom title' do
        config = described_class.new(title: 'Custom API')
        expect(config.title).to eq('Custom API')
      end

      it 'accepts custom version' do
        config = described_class.new(version: '3.0.0')
        expect(config.version).to eq('3.0.0')
      end

      it 'accepts custom description' do
        config = described_class.new(
          description: 'A comprehensive API documentation'
        )
        expect(config.description).to eq('A comprehensive API documentation')
      end

      it 'accepts custom swagger_ui_path' do
        config = described_class.new(swagger_ui_path: '/swagger')
        expect(config.swagger_ui_path).to eq('/swagger')
      end

      it 'accepts custom redoc_path' do
        config = described_class.new(redoc_path: '/api-docs')
        expect(config.redoc_path).to eq('/api-docs')
      end

      it 'accepts custom openapi_json_path' do
        config = described_class.new(openapi_json_path: '/spec.json')
        expect(config.openapi_json_path).to eq('/spec.json')
      end

      it 'accepts contact info' do
        contact = Spikard::ContactInfo.new(
          name: 'API Team',
          email: 'api@example.com'
        )
        config = described_class.new(contact: contact)
        expect(config.contact).to eq(contact)
        expect(config.contact.name).to eq('API Team')
      end

      it 'accepts license info' do
        license = Spikard::LicenseInfo.new(name: 'MIT')
        config = described_class.new(license: license)
        expect(config.license).to eq(license)
        expect(config.license.name).to eq('MIT')
      end

      it 'accepts servers array' do
        servers = [
          Spikard::ServerInfo.new(url: 'https://api.example.com', description: 'Production'),
          Spikard::ServerInfo.new(url: 'http://localhost:8000', description: 'Development')
        ]
        config = described_class.new(servers: servers)
        expect(config.servers).to eq(servers)
        expect(config.servers.count).to eq(2)
      end

      it 'accepts security_schemes hash' do
        schemes = {
          'bearerAuth' => Spikard::SecuritySchemeInfo.new(type: 'http', scheme: 'bearer')
        }
        config = described_class.new(security_schemes: schemes)
        expect(config.security_schemes).to eq(schemes)
      end
    end

    describe 'attribute accessors' do
      let(:config) { described_class.new }

      it 'allows enabled to be modified' do
        config.enabled = true
        expect(config.enabled).to be true
      end

      it 'allows title to be modified' do
        config.title = 'New Title'
        expect(config.title).to eq('New Title')
      end

      it 'allows version to be modified' do
        config.version = '2.5.0'
        expect(config.version).to eq('2.5.0')
      end

      it 'allows description to be modified' do
        config.description = 'Updated description'
        expect(config.description).to eq('Updated description')
      end

      it 'allows swagger_ui_path to be modified' do
        config.swagger_ui_path = '/swagger-ui'
        expect(config.swagger_ui_path).to eq('/swagger-ui')
      end

      it 'allows redoc_path to be modified' do
        config.redoc_path = '/redoc-ui'
        expect(config.redoc_path).to eq('/redoc-ui')
      end

      it 'allows openapi_json_path to be modified' do
        config.openapi_json_path = '/api/spec.json'
        expect(config.openapi_json_path).to eq('/api/spec.json')
      end

      it 'allows contact to be modified' do
        contact = Spikard::ContactInfo.new(name: 'Support')
        config.contact = contact
        expect(config.contact).to eq(contact)
      end

      it 'allows license to be modified' do
        license = Spikard::LicenseInfo.new(name: 'Apache-2.0')
        config.license = license
        expect(config.license).to eq(license)
      end

      it 'allows servers to be modified' do
        servers = [
          Spikard::ServerInfo.new(url: 'https://api.example.com')
        ]
        config.servers = servers
        expect(config.servers).to eq(servers)
      end

      it 'allows security_schemes to be modified' do
        schemes = {
          'apiKey' => Spikard::SecuritySchemeInfo.new(
            type: 'apiKey',
            location: 'header',
            name: 'X-API-Key'
          )
        }
        config.security_schemes = schemes
        expect(config.security_schemes).to eq(schemes)
      end
    end

    describe 'complete configuration' do
      it 'builds a complete OpenAPI configuration' do
        contact = Spikard::ContactInfo.new(
          name: 'API Team',
          email: 'api@example.com',
          url: 'https://example.com'
        )
        license = Spikard::LicenseInfo.new(
          name: 'MIT',
          url: 'https://opensource.org/licenses/MIT'
        )
        servers = [
          Spikard::ServerInfo.new(
            url: 'https://api.example.com',
            description: 'Production'
          ),
          Spikard::ServerInfo.new(
            url: 'http://localhost:8000',
            description: 'Development'
          )
        ]
        security_schemes = {
          'bearerAuth' => Spikard::SecuritySchemeInfo.new(
            type: 'http',
            scheme: 'bearer',
            bearer_format: 'JWT'
          )
        }

        config = described_class.new(
          enabled: true,
          title: 'My API',
          version: '1.0.0',
          description: 'A comprehensive API',
          swagger_ui_path: '/docs',
          redoc_path: '/redoc',
          openapi_json_path: '/openapi.json',
          contact: contact,
          license: license,
          servers: servers,
          security_schemes: security_schemes
        )

        expect(config.enabled).to be true
        expect(config.title).to eq('My API')
        expect(config.version).to eq('1.0.0')
        expect(config.description).to eq('A comprehensive API')
        expect(config.contact.name).to eq('API Team')
        expect(config.license.name).to eq('MIT')
        expect(config.servers.count).to eq(2)
        expect(config.security_schemes.count).to eq(1)
      end
    end
  end

  describe Spikard::ServerConfig do
    describe 'initialization with defaults' do
      subject(:config) { described_class.new }

      it 'binds to 127.0.0.1 by default' do
        expect(config.host).to eq('127.0.0.1')
      end

      it 'listens on port 8000 by default' do
        expect(config.port).to eq(8000)
      end

      it 'uses 1 worker by default' do
        expect(config.workers).to eq(1)
      end

      it 'enables request ID by default' do
        expect(config.enable_request_id).to be true
      end

      it 'sets max body size to 10MB by default' do
        expect(config.max_body_size).to eq(10 * 1024 * 1024)
      end

      it 'sets request timeout to 30 seconds by default' do
        expect(config.request_timeout).to eq(30)
      end

      it 'creates default compression config' do
        expect(config.compression).to be_a(Spikard::CompressionConfig)
      end

      it 'has no rate limit by default' do
        expect(config.rate_limit).to be_nil
      end

      it 'has no JWT auth by default' do
        expect(config.jwt_auth).to be_nil
      end

      it 'has no API key auth by default' do
        expect(config.api_key_auth).to be_nil
      end

      it 'has no static files by default' do
        expect(config.static_files).to eq([])
      end

      it 'enables graceful shutdown by default' do
        expect(config.graceful_shutdown).to be true
      end

      it 'sets shutdown timeout to 30 seconds by default' do
        expect(config.shutdown_timeout).to eq(30)
      end

      it 'has no OpenAPI config by default' do
        expect(config.openapi).to be_nil
      end
    end

    describe 'initialization with network parameters' do
      it 'accepts custom host' do
        config = described_class.new(host: '0.0.0.0')
        expect(config.host).to eq('0.0.0.0')
      end

      it 'accepts custom port' do
        config = described_class.new(port: 9000)
        expect(config.port).to eq(9000)
      end

      it 'accepts custom workers' do
        config = described_class.new(workers: 4)
        expect(config.workers).to eq(4)
      end

      it 'accepts custom max_body_size' do
        config = described_class.new(max_body_size: 50 * 1024 * 1024)
        expect(config.max_body_size).to eq(50 * 1024 * 1024)
      end

      it 'accepts custom request_timeout' do
        config = described_class.new(request_timeout: 60)
        expect(config.request_timeout).to eq(60)
      end
    end

    describe 'initialization with middleware parameters' do
      it 'accepts custom compression config' do
        compression = Spikard::CompressionConfig.new(quality: 9)
        config = described_class.new(compression: compression)
        expect(config.compression).to eq(compression)
        expect(config.compression.quality).to eq(9)
      end

      it 'accepts custom rate limit config' do
        rate_limit = Spikard::RateLimitConfig.new(per_second: 100, burst: 200)
        config = described_class.new(rate_limit: rate_limit)
        expect(config.rate_limit).to eq(rate_limit)
        expect(config.rate_limit.per_second).to eq(100)
      end

      it 'accepts static files config' do
        static = Spikard::StaticFilesConfig.new(
          directory: './public',
          route_prefix: '/static'
        )
        config = described_class.new(static_files: [static])
        expect(config.static_files.count).to eq(1)
        expect(config.static_files[0]).to eq(static)
      end

      it 'accepts multiple static files configs' do
        static1 = Spikard::StaticFilesConfig.new(
          directory: './public',
          route_prefix: '/static'
        )
        static2 = Spikard::StaticFilesConfig.new(
          directory: './uploads',
          route_prefix: '/files'
        )
        config = described_class.new(static_files: [static1, static2])
        expect(config.static_files.count).to eq(2)
      end

      it 'accepts OpenAPI config' do
        openapi = Spikard::OpenApiConfig.new(
          enabled: true,
          title: 'My API',
          version: '1.0.0'
        )
        config = described_class.new(openapi: openapi)
        expect(config.openapi).to eq(openapi)
        expect(config.openapi.enabled).to be true
      end
    end

    describe 'initialization with shutdown parameters' do
      it 'accepts custom graceful_shutdown' do
        config = described_class.new(graceful_shutdown: false)
        expect(config.graceful_shutdown).to be false
      end

      it 'accepts custom shutdown_timeout' do
        config = described_class.new(shutdown_timeout: 60)
        expect(config.shutdown_timeout).to eq(60)
      end
    end

    describe 'initialization with request ID parameter' do
      it 'accepts custom enable_request_id' do
        config = described_class.new(enable_request_id: false)
        expect(config.enable_request_id).to be false
      end
    end

    describe 'attribute accessors' do
      let(:config) { described_class.new }

      it 'allows host to be modified' do
        config.host = '0.0.0.0'
        expect(config.host).to eq('0.0.0.0')
      end

      it 'allows port to be modified' do
        config.port = 3000
        expect(config.port).to eq(3000)
      end

      it 'allows workers to be modified' do
        config.workers = 8
        expect(config.workers).to eq(8)
      end

      it 'allows enable_request_id to be modified' do
        config.enable_request_id = false
        expect(config.enable_request_id).to be false
      end

      it 'allows max_body_size to be modified' do
        config.max_body_size = 100 * 1024 * 1024
        expect(config.max_body_size).to eq(100 * 1024 * 1024)
      end

      it 'allows request_timeout to be modified' do
        config.request_timeout = 120
        expect(config.request_timeout).to eq(120)
      end

      it 'allows compression to be modified' do
        new_compression = Spikard::CompressionConfig.new(gzip: false)
        config.compression = new_compression
        expect(config.compression).to eq(new_compression)
      end

      it 'allows rate_limit to be modified' do
        rate_limit = Spikard::RateLimitConfig.new(per_second: 50, burst: 100)
        config.rate_limit = rate_limit
        expect(config.rate_limit).to eq(rate_limit)
      end

      it 'allows static_files to be modified' do
        static = [
          Spikard::StaticFilesConfig.new(
            directory: './uploads',
            route_prefix: '/files'
          )
        ]
        config.static_files = static
        expect(config.static_files).to eq(static)
      end

      it 'allows graceful_shutdown to be modified' do
        config.graceful_shutdown = false
        expect(config.graceful_shutdown).to be false
      end

      it 'allows shutdown_timeout to be modified' do
        config.shutdown_timeout = 45
        expect(config.shutdown_timeout).to eq(45)
      end

      it 'allows openapi to be modified' do
        openapi = Spikard::OpenApiConfig.new(enabled: true)
        config.openapi = openapi
        expect(config.openapi).to eq(openapi)
      end
    end

    describe 'complete server configuration' do
      it 'builds a production-like configuration' do
        compression = Spikard::CompressionConfig.new(quality: 9)
        rate_limit = Spikard::RateLimitConfig.new(per_second: 1000, burst: 2000)
        static = Spikard::StaticFilesConfig.new(
          directory: './public',
          route_prefix: '/static'
        )
        openapi = Spikard::OpenApiConfig.new(
          enabled: true,
          title: 'Production API',
          version: '1.0.0'
        )

        config = described_class.new(
          host: '0.0.0.0',
          port: 80,
          workers: 8,
          enable_request_id: true,
          max_body_size: 50 * 1024 * 1024,
          request_timeout: 60,
          compression: compression,
          rate_limit: rate_limit,
          static_files: [static],
          graceful_shutdown: true,
          shutdown_timeout: 30,
          openapi: openapi
        )

        expect(config.host).to eq('0.0.0.0')
        expect(config.port).to eq(80)
        expect(config.workers).to eq(8)
        expect(config.enable_request_id).to be true
        expect(config.max_body_size).to eq(50 * 1024 * 1024)
        expect(config.request_timeout).to eq(60)
        expect(config.compression.quality).to eq(9)
        expect(config.rate_limit.per_second).to eq(1000)
        expect(config.static_files.count).to eq(1)
        expect(config.openapi.enabled).to be true
      end

      it 'builds a development configuration' do
        openapi = Spikard::OpenApiConfig.new(
          enabled: true,
          title: 'Development API',
          version: '1.0.0'
        )

        config = described_class.new(
          host: 'localhost',
          port: 8000,
          workers: 1,
          compression: Spikard::CompressionConfig.new(gzip: true, brotli: false),
          openapi: openapi
        )

        expect(config.host).to eq('localhost')
        expect(config.port).to eq(8000)
        expect(config.workers).to eq(1)
        expect(config.compression.gzip).to be true
        expect(config.openapi.enabled).to be true
      end
    end

    describe 'various port configurations' do
      [80, 443, 3000, 5000, 8000, 8080, 9000, 65_535].each do |port|
        it "supports port #{port}" do
          config = described_class.new(port: port)
          expect(config.port).to eq(port)
        end
      end
    end

    describe 'various host configurations' do
      ['127.0.0.1', '0.0.0.0', 'localhost', '192.168.1.1'].each do |host|
        it "supports host #{host}" do
          config = described_class.new(host: host)
          expect(config.host).to eq(host)
        end
      end
    end

    describe 'various worker configurations' do
      [1, 2, 4, 8, 16].each do |workers|
        it "supports #{workers} workers" do
          config = described_class.new(workers: workers)
          expect(config.workers).to eq(workers)
        end
      end
    end

    describe 'body size configurations' do
      it 'supports 1MB max body size' do
        config = described_class.new(max_body_size: 1024 * 1024)
        expect(config.max_body_size).to eq(1024 * 1024)
      end

      it 'supports 100MB max body size' do
        config = described_class.new(max_body_size: 100 * 1024 * 1024)
        expect(config.max_body_size).to eq(100 * 1024 * 1024)
      end

      it 'supports unlimited body size with nil' do
        config = described_class.new(max_body_size: nil)
        expect(config.max_body_size).to be_nil
      end
    end

    describe 'timeout configurations' do
      [10, 30, 60, 120, 300].each do |timeout|
        it "supports #{timeout} second timeout" do
          config = described_class.new(request_timeout: timeout)
          expect(config.request_timeout).to eq(timeout)
        end
      end

      it 'supports unlimited timeout with nil' do
        config = described_class.new(request_timeout: nil)
        expect(config.request_timeout).to be_nil
      end
    end
  end
end
