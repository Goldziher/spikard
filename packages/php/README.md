# Spikard PHP bindings (WIP)

Early scaffolding for the upcoming PHP bindings powered by `ext-php-rs` over
the Rust `spikard` core. The PHP package will stay thin and delegate all
middleware and validation to the shared Rust runtime to mirror the existing
Python/Node/Ruby adapters.

## Commands

```bash
composer install --no-interaction --no-progress
composer run lint         # phpstan (max level)
composer run lint:fix     # php-cs-fixer + phpstan
composer run format       # php-cs-fixer (with fixes)
composer run format:check # php-cs-fixer (dry-run)
composer run test         # phpunit
```

## Notes

- Keep binding logic in Rust (`crates/spikard-php`) and expose configuration
  APIs only; no middleware duplication.
- Tests follow the fixture-first approach used in other bindings; wire fixtures
  from `testing_data/` once the FFI surface is implemented.
