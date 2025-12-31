# Configuration

Spikard favors code-based configuration with environment overrides. The same knobs exist regardless of binding.

## Basic ServerConfig

Create and configure your server with explicit settings:

=== "Python"

    --8<-- "snippets/python/config_server.md"

=== "TypeScript"

    --8<-- "snippets/typescript/config_server.md"

=== "Ruby"

    --8<-- "snippets/ruby/config_server.md"

=== "PHP"

    --8<-- "snippets/php/config_server.md"

=== "Rust"

    --8<-- "snippets/rust/config_server.md"

## Environment Variable Overrides

Override config at runtime without changing code:

=== "Python"

    --8<-- "snippets/python/config_environment.md"

=== "TypeScript"

    --8<-- "snippets/typescript/config_environment.md"

=== "Ruby"

    --8<-- "snippets/ruby/config_environment.md"

=== "PHP"

    --8<-- "snippets/php/config_environment.md"

=== "Rust"

    --8<-- "snippets/rust/config_environment.md"

Run with: `SPIKARD_PORT=8080 SPIKARD_WORKERS=4 python app.py`

## Production Configuration

Full production setup with compression, rate limiting, and monitoring:

=== "Python"

    --8<-- "snippets/python/config_production.md"

=== "TypeScript"

    --8<-- "snippets/typescript/config_production.md"

=== "Ruby"

    --8<-- "snippets/ruby/config_production.md"

=== "PHP"

    --8<-- "snippets/php/config_production.md"

=== "Rust"

    --8<-- "snippets/rust/config_production.md"

## Server Settings

- `host` / `port` – network binding for the HTTP server
- `workers` – concurrency; defaults to 1 (use CPU count in production)
- `keep_alive` / `request_timeout` – tune to match upstream load balancers
- `max_body_size` – prevent memory exhaustion from large uploads

## Middleware Defaults

- Logging/tracing enabled by default with request IDs
- CORS/compression configurable per app or per route
- Add custom middleware to inject tenant data, auth, or feature flags

## Validation Controls

- Enable/disable request and response validation globally or per route
- Provide JSON Schemas or rely on derived DTOs
- Customize error formatting for clients

## TLS and HTTP/2 (via Reverse Proxy)

Spikard listens on plain HTTP. Use a reverse proxy for TLS termination:

=== "nginx"

    ```nginx
    upstream spikard_backend {
        server 127.0.0.1:8080;
    }

    server {
        listen 443 ssl http2;
        server_name api.example.com;

        ssl_certificate /etc/ssl/certs/api.example.com.crt;
        ssl_certificate_key /etc/ssl/private/api.example.com.key;
        ssl_protocols TLSv1.2 TLSv1.3;
        ssl_ciphers HIGH:!aNULL:!MD5;

        location / {
            proxy_pass http://spikard_backend;
            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_set_header X-Forwarded-Proto $scheme;
        }
    }
    ```

=== "Caddy"

    ```caddyfile
    api.example.com {
        reverse_proxy localhost:8080
    }
    ```

=== "Traefik"

    ```yaml
    # docker-compose.yml
    services:
      traefik:
        image: traefik:v2.10
        command:
          - --entrypoints.websecure.address=:443
          - --certificatesresolvers.myresolver.acme.tlschallenge=true
        ports:
          - "443:443"
        labels:
          - "traefik.http.routers.api.rule=Host(`api.example.com`)"
          - "traefik.http.routers.api.tls.certresolver=myresolver"
          - "traefik.http.services.api.loadbalancer.server.port=8080"
    ```

## Verify It Works

=== "Health Check"

    ```bash
    curl http://localhost:8080/health
    # Expected: {"status":"ok"}
    ```

=== "OpenAPI Docs"

    ```bash
    # Swagger UI
    open http://localhost:8080/docs

    # Redoc
    open http://localhost:8080/redoc

    # Raw spec
    curl http://localhost:8080/openapi.json
    ```

=== "Rate Limiting"

    ```bash
    # Trigger rate limit
    for i in {1..150}; do curl http://localhost:8080/health; done
    # Expected: 429 Too Many Requests after burst limit
    ```

A detailed option matrix lives in [Reference: Configuration](../reference/configuration.md). Use the Taskfile (`task docs:serve`) to iterate on examples locally.
