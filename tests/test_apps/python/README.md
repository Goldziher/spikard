# Python Test App for Published Spikard Package

This directory contains a minimal Python application that validates the published Spikard package from PyPI. It ensures that end users can successfully install and use Spikard in a fresh Python environment.

## Purpose

This is a **Tier 3 (E2E)** test that validates:

1. ✅ Published package installs correctly from PyPI
2. ✅ Core functionality works as expected
3. ✅ Version matches the published release
4. ✅ No workspace dependencies leak through
5. ✅ Clean virtual environment installation

## Files

- **`pyproject.toml`**: Manifest with exact version pin (`spikard==0.6.0`)
- **`app.py`**: Minimal server with routes (hello, echo, error)
- **`test_published.py`**: Core functionality tests with version assertion
- **`README.md`**: This file

## Requirements

- Python 3.10+
- uv (recommended) or pip
- No workspace dependencies

## Running Tests

### Using uv (recommended)

```bash
# Install dependencies in clean virtual environment
uv venv
uv pip install -e .

# Run tests
uv run pytest test_published.py -v

# Clean up
rm -rf .venv
```

### Using pip

```bash
# Create clean virtual environment
python -m venv .venv
source .venv/bin/activate  # On Windows: .venv\Scripts\activate

# Install dependencies
pip install -e .

# Run tests
pytest test_published.py -v

# Clean up
deactivate
rm -rf .venv
```

## Test Coverage

The test suite validates:

### Version Validation
- `test_version_is_correct()`: Asserts `spikard.__version__ == "0.6.0"`

### Core Functionality
- `test_server_can_be_created()`: Server instantiation
- `test_hello_handler_returns_correct_response()`: Basic routing
- `test_echo_handler_echoes_request_data()`: Request/response handling
- `test_error_handler_returns_error_response()`: Error handling

### Package Structure
- `test_package_exports_expected_symbols()`: Public API validation
- `test_multiple_routes_can_be_registered()`: Route registration
- `test_imports_work_correctly()`: Import paths

## Expected Output

```
tests/test_apps/python/test_published.py::test_version_is_correct PASSED
tests/test_apps/python/test_published.py::test_server_can_be_created PASSED
tests/test_apps/python/test_published.py::test_hello_handler_returns_correct_response PASSED
tests/test_apps/python/test_published.py::test_echo_handler_echoes_request_data PASSED
tests/test_apps/python/test_published.py::test_error_handler_returns_error_response PASSED
tests/test_apps/python/test_published.py::test_package_exports_expected_symbols PASSED
tests/test_apps/python/test_published.py::test_multiple_routes_can_be_registered PASSED
tests/test_apps/python/test_published.py::test_imports_work_correctly PASSED

=============================== 8 passed in 0.15s ================================
```

## Troubleshooting

### Package Not Found on PyPI

**Error**: `ERROR: Could not find a version that satisfies the requirement spikard==0.6.0`

**Solutions**:
1. Verify publication: `curl -sSf https://pypi.org/pypi/spikard/0.6.0/json`
2. Wait 5-10 minutes for CDN propagation
3. Check PyPI status: https://status.python.org/
4. Run validation script: `../../scripts/validate-published.sh 0.6.0`

### Version Mismatch

**Error**: `AssertionError: Version mismatch: expected 0.6.0, got 0.5.0`

**Solutions**:
1. Check `pyproject.toml` has correct pin: `spikard==0.6.0`
2. Clear pip cache: `uv cache clean` or `pip cache purge`
3. Delete `.venv/` and reinstall
4. Run update script: `../../scripts/update-versions.sh 0.6.0`

### Import Errors

**Error**: `ModuleNotFoundError: No module named 'spikard'`

**Solutions**:
1. Ensure virtual environment is activated
2. Verify installation: `uv pip list | grep spikard` or `pip list | grep spikard`
3. Reinstall: `uv pip install --force-reinstall spikard==0.6.0`

### Test Failures

**Error**: `AttributeError: module 'spikard' has no attribute 'Server'`

**Solutions**:
1. Check package version: `python -c "import spikard; print(spikard.__version__)"`
2. Verify PyPI package integrity
3. Report issue if published package is broken

### Workspace Dependencies Detected

**Error**: Tests pass but version is from local build (e.g., `0.6.0+local`)

**Solutions**:
1. Ensure `PYTHONPATH` is not set
2. Delete any `*.egg-info` directories
3. Use completely fresh virtual environment
4. Verify no `editable-installs` in `pip list`

## CI Integration

This test app is designed to run in CI via `scripts/run-all.sh`:

```yaml
# .github/workflows/test-published.yml
- name: Test Python published package
  run: |
    cd tests/test_apps/python
    uv venv
    uv pip install -e .
    uv run pytest test_published.py -v
```

## Updating Version Pin

**Manual**:
```bash
# Edit pyproject.toml
sed -i 's/spikard==.*/spikard==0.7.0/' pyproject.toml
```

**Automated** (recommended):
```bash
# From repository root
./tests/test_apps/scripts/update-versions.sh 0.7.0
```

## Design Principles

1. **Exact version pinning**: Use `==` not `~=` or `>=`
2. **Minimal dependencies**: Only spikard + test framework
3. **Fresh environment**: Each run uses clean virtual environment
4. **No workspace leakage**: No imports from `../../crates/` or `../../packages/`
5. **Version assertion**: First test MUST validate `__version__`

## Related Documentation

- **Main README**: `../README.md`
- **Workspace Tests**: `../../../packages/python/tests/README.md`
- **Release Process**: `../../../docs/RELEASE.md`
