# Spikard Ruby Test App

## Purpose

Test application that validates the published `spikard` Ruby gem (v0.11.0) works correctly in a real Ruby environment.

## Setup

```bash
cd tests/test_apps/ruby
bundle install
```

## Run Tests

```bash
bundle exec rspec
```

## Troubleshooting

### Gem not found
- Verify `spikard-0.7.0.gem` is published to RubyGems.org
- Check gem availability: `gem search spikard -r`
- Try clearing gem cache: `gem cleanup`

### Native extension build failures
- Ensure Rust toolchain is installed (magnus requires it)
- Check Ruby headers: `ruby -rrbconfig -e 'puts RbConfig::CONFIG["rubyhdrdir"]'`
- Verify rb-sys compatibility with your Ruby version

### Test failures
- Confirm server starts on random port (0)
- Check Net::HTTP is available (bundled with Ruby)
- Verify handler blocks match expected signatures

### LoadError
- Ensure magnus native extension compiled successfully
- Check platform compatibility (x86_64, arm64)
- Verify shared library (.so/.bundle/.dylib) is present
