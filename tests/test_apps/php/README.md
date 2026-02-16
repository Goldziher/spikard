# Spikard PHP Test App

## Purpose

Test application that validates the published `spikard/spikard` Composer package (v0.11.0) works correctly in a real PHP environment.

## Setup

```bash
cd tests/test_apps/php
composer install
```

## Run Tests

```bash
composer test
```

Run static analysis:
```bash
composer stan
```

## Troubleshooting

### Package not found
- Verify `spikard/spikard:0.7.0` is published to Packagist
- Check package availability: `composer show spikard/spikard -a`
- Try clearing cache: `composer clear-cache`

### Extension not loaded
- Ensure ext-php-rs native extension compiled successfully
- Check PHP extensions: `php -m | grep spikard`
- Verify extension path in `php.ini` or `conf.d/`

### Test failures
- Confirm PHP 8.2+ is installed: `php -v`
- Check server starts on random port (0)
- Verify strict_types=1 is declared in all files

### PHPStan errors
- Ensure all type declarations are present
- Never use mixed types or @phpstan-ignore
- Check PSR-4 autoloading is configured correctly

### Composer autoload issues
- Run `composer dump-autoload -o`
- Verify PSR-4 namespace matches directory structure
- Ensure extension .so/.dll file is in correct location
