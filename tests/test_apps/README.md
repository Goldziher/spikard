# Test Apps for Published Packages

This directory contains minimal test applications that validate published Spikard packages across all supported language bindings. These tests ensure that end users can successfully install and use Spikard from package registries (PyPI, npm, RubyGems, Packagist).

## Three-Tier Testing Strategy

| Tier | Scope | Location | Purpose |
|------|-------|----------|---------|
| **Unit** | Pure functions, fast | `crates/*/tests/` | Validate core logic |
| **Integration** | Real DB, fixtures | `packages/python/tests/` | Validate workspace bindings |
| **E2E** | Published packages | `tests/test_apps/` | Validate end-user experience |

This directory implements **Tier 3 (E2E)** testing.

## Directory Structure

```
tests/test_apps/
├── scripts/
│   ├── update-versions.sh       # Update version pins across all apps
│   ├── run-all.sh               # Run all test apps sequentially
│   └── validate-published.sh    # Check registry availability
├── python/
│   ├── pyproject.toml           # Exact version pin (spikard==0.7.0)
│   ├── app.py                   # Minimal server
│   ├── test_published.py        # Core functionality tests
│   └── README.md                # Python-specific docs
├── node/                        # (Future: TypeScript/Node.js test app)
├── ruby/                        # (Future: Ruby test app)
├── php/                         # (Future: PHP test app)
└── README.md                    # This file
```

## Usage

### After Publishing a New Version

1. **Validate registry availability**:
   ```bash
   ./scripts/validate-published.sh 0.7.0
   ```

2. **Update version pins**:
   ```bash
   ./scripts/update-versions.sh 0.7.0
   ```

3. **Run all test apps**:
   ```bash
   ./scripts/run-all.sh
   ```

4. **Commit updated pins**:
   ```bash
   git add tests/test_apps/
   git commit -m "Update test apps to v0.10.1"
   ```

### Manual Testing (Individual App)

**Python**:
```bash
cd tests/test_apps/python
uv venv
uv pip install -e .
uv run pytest test_published.py -v
```

**Node** (when implemented):
```bash
cd tests/test_apps/node
pnpm install
pnpm test
```

**Ruby** (when implemented):
```bash
cd tests/test_apps/ruby
bundle install
bundle exec rspec
```

**PHP** (when implemented):
```bash
cd tests/test_apps/php
composer install
composer test
```

## Test App Requirements

Each test app MUST:

1. **Exact version pinning**: Use `==` (Python), exact version (npm/Composer/Bundler), no wildcards
2. **Version assertion**: Test MUST validate `__version__` matches expected
3. **Core functionality**: Test basic server startup, routing, middleware
4. **Minimal dependencies**: Only test published package + test framework
5. **Fresh install**: Each run uses clean virtual environment/node_modules
6. **Documentation**: Language-specific README with troubleshooting

## Version Management

Version pins are managed via `scripts/update-versions.sh`. This ensures:

- All test apps use the **same published version**
- No accidental workspace dependencies
- Tests validate actual end-user experience
- CI can automatically update after releases

## CI Integration

GitHub Actions workflow (`.github/workflows/test-published.yml`):

```yaml
name: Test Published Packages

on:
  workflow_dispatch:
    inputs:
      version:
        description: 'Version to test (e.g., 0.7.0)'
        required: true

jobs:
  validate-and-test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Validate registry availability
        run: ./tests/test_apps/scripts/validate-published.sh ${{ github.event.inputs.version }}

      - name: Run all test apps
        run: ./tests/test_apps/scripts/run-all.sh
```

## Troubleshooting

### PyPI Package Not Found
- Verify publication: `curl -sSf https://pypi.org/pypi/spikard/0.7.0/json`
- Wait 5-10 minutes for CDN propagation
- Check PyPI status: https://status.python.org/

### npm Package Not Found
- Verify publication: `npm view @spikard/node@0.10.1`
- Wait for npm registry sync
- Check npm status: https://status.npmjs.org/

### Version Mismatch in Tests
- Ensure `update-versions.sh` ran successfully
- Check `git diff` for uncommitted changes
- Clear package caches (pip cache, npm cache, etc.)

### Import Errors
- Python: Check virtual environment activation
- Node: Delete `node_modules/` and re-run `pnpm install`
- Ruby: Run `bundle clean --force` then `bundle install`
- PHP: Run `composer clear-cache` then `composer install`

## Adding New Language Support

When adding a new language binding:

1. Create `tests/test_apps/<language>/` directory
2. Add manifest with exact version pin
3. Create minimal `app.<ext>` server
4. Add `test_published.<ext>` with version assertion
5. Document in language-specific README.md
6. Update `scripts/update-versions.sh`
7. Update `scripts/run-all.sh`
8. Update this README

## Related Documentation

- **Workspace Testing**: `packages/python/tests/README.md`
- **Rust Unit Tests**: `crates/spikard/README.md`
- **CI/CD Pipeline**: `.github/workflows/`
- **Release Process**: `docs/RELEASE.md`
