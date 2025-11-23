# Installation and Usage Guide

## Quick Start

```bash
# Install dependencies
bundle install

# Run server (default port 8000)
ruby server.rb

# Run server on custom port
ruby server.rb 3000

# Run via config.ru
rackup -p 8000
```

## Dependencies

All dependencies are listed in `Gemfile`:

```ruby
gem 'roda', '~> 3.97'    # Routing framework
gem 'puma', '~> 7.1'     # HTTP server
gem 'rack', '~> 3.2'     # Rack interface
gem 'rackup', '~> 2.2'   # Rack server runner
```

**Note**: NO dry-schema dependency (validation removed).

## Installation Steps

### 1. Install Ruby

Ensure Ruby 3.2+ is installed:

```bash
ruby --version
# Should show: ruby 3.2.x
```

### 2. Install Bundler

```bash
gem install bundler
```

### 3. Install Dependencies

```bash
cd /path/to/roda-raw
bundle install
```

### 4. Verify Installation

```bash
ruby -c server.rb
# Should output: Syntax OK
```

## Running the Server

### Method 1: Direct Execution

```bash
ruby server.rb [port]
```

Examples:
```bash
ruby server.rb          # Port 8000
ruby server.rb 3000     # Port 3000
ruby server.rb 8080     # Port 8080
```

### Method 2: Via config.ru

```bash
rackup -p 8000
```

### Method 3: Background Process

```bash
# Start in background
ruby server.rb 8000 &

# Get process ID
PID=$!

# Stop server
kill $PID
```

## Testing the Server

### Quick Test

```bash
# Test health endpoint
curl http://localhost:8000/health

# Expected output:
# {"status":"ok"}
```

### Run Test Suite

```bash
./test.sh
```

This will:
1. Start the server
2. Test all 18 endpoints
3. Verify responses
4. Stop the server
5. Show that invalid data is accepted (no validation)

### Manual Testing

```bash
# Small JSON payload
curl -X POST http://localhost:8000/json/small \
  -H "Content-Type: application/json" \
  -d '{"name":"test","description":"item","price":9.99}'

# Path parameters
curl http://localhost:8000/path/simple/123

# Query parameters
curl "http://localhost:8000/query/few?q=test&page=1&limit=10"
```

## Verifying No Validation

The key feature of roda-raw is that it accepts ANY JSON:

```bash
# This should work even though fields are wrong
curl -X POST http://localhost:8000/json/small \
  -H "Content-Type: application/json" \
  -d '{"wrong":"fields","price":"not-a-number"}'

# Server will echo back whatever you send
```

## Troubleshooting

### Port Already in Use

```bash
# Check if port is in use
lsof -i :8000

# Kill process using port
kill -9 <PID>
```

### Dependencies Not Found

```bash
# Clean and reinstall
rm -rf .bundle vendor
bundle install
```

### Server Not Responding

```bash
# Check if server is running
ps aux | grep ruby

# Check logs
tail -f log/puma.log  # if logging enabled
```

## Production Deployment

For production use, consider:

1. **Process Manager**: Use systemd, supervisord, or PM2
2. **Reverse Proxy**: Put Nginx or Apache in front
3. **Multiple Workers**: Increase Puma threads/workers
4. **Monitoring**: Add logging and metrics

Example Puma config (`config/puma.rb`):

```ruby
threads 1, 5
workers 2
bind 'tcp://0.0.0.0:8000'
preload_app!
```

## Performance Tips

1. **Single-threaded for benchmarking**: Keep `Threads: '1:1'`
2. **Multi-threaded for production**: Increase threads
3. **Disable logging**: Keep `Silent: true` for benchmarks
4. **Use Ruby 3.2+**: Better performance than older versions

## Next Steps

- Compare with validated `roda` server
- Run benchmarks using wrk or ab
- Measure throughput and latency differences
- See `COMPARISON.md` for architectural details
