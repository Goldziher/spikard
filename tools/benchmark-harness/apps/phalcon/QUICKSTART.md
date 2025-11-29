# Phalcon Benchmark Server - Quick Start Guide

Get the Phalcon benchmark server running in 5 minutes!

## Prerequisites

- PHP 8.2 or higher
- Phalcon 5.9 extension installed
- Composer
- curl (for testing)

## Quick Start

### 1. Install Dependencies

```bash
cd /path/to/tools/benchmark-harness/apps/phalcon
composer install
```

### 2. Verify Setup

```bash
# Check PHP version
php -v

# Check Phalcon is installed
php -m | grep phalcon

# Run verification script
./verify.sh
```

### 3. Start the Server

```bash
php server.php 8000
```

The server will start on `http://localhost:8000`

### 4. Test Endpoints

Open another terminal:

```bash
# Health check
curl http://localhost:8000/health

# Create a user
curl -X POST http://localhost:8000/users \
  -H "Content-Type: application/json" \
  -d '{"name":"John Doe","email":"john@example.com"}'

# Get user (replace 1 with actual ID)
curl http://localhost:8000/users/1

# Update user
curl -X PUT http://localhost:8000/users/1 \
  -H "Content-Type: application/json" \
  -d '{"name":"Jane Doe"}'

# Delete user
curl -X DELETE http://localhost:8000/users/1
```

## Using Docker

```bash
# Build and run with Docker Compose
docker-compose up

# Or build and run manually
docker build -t phalcon-benchmark .
docker run -p 8000:8000 phalcon-benchmark
```

## Benchmarking with Apache Bench

```bash
# Warmup
ab -n 100 -c 10 http://localhost:8000/health

# User creation benchmark
ab -n 1000 -c 50 -p user.json -T application/json http://localhost:8000/users

# Retrieval benchmark
ab -n 1000 -c 50 http://localhost:8000/users/1

# Update benchmark
ab -n 1000 -c 50 -p user.json -T application/json -X PUT http://localhost:8000/users/1
```

Create `user.json`:
```json
{
  "name": "Benchmark User",
  "email": "bench@example.com",
  "age": 25
}
```

## Troubleshooting

### Phalcon Not Installed

```bash
# On macOS with Homebrew
brew install phalcon

# Or build from source
git clone --depth 1 --branch 5.9.x https://github.com/phalcon/cphalcon.git
cd cphalcon/build
./install
```

### Port Already in Use

```bash
# Use a different port
php server.php 8080

# Or kill existing process
lsof -i :8000
kill -9 <PID>
```

### Composer Issues

```bash
# Update composer
composer self-update

# Clear cache and reinstall
rm -rf vendor composer.lock
composer install
```

## Performance Tips

1. **Use PHP 8.2+**: Latest version has better performance
2. **Enable OPcache**: `php.ini: opcache.enable=1`
3. **Tune memory**: `php server.php -d memory_limit=256M 8000`
4. **Use multiple workers**: See production deployment docs

## Next Steps

- Review the full [README.md](README.md)
- Check out [Phalcon documentation](https://phalcon.io/docs/)
- Run the verification script: `./verify.sh`
- Compare with other frameworks using the benchmark harness

## Support

- GitHub Issues: https://github.com/phalcon/cphalcon/issues
- Phalcon Forum: https://phalcon.io/forum/
- Documentation: https://phalcon.io/docs/
