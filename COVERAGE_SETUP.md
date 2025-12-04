# Python Code Coverage Configuration

This document outlines the pytest-cov setup for the Spikard Python package with 80% minimum coverage enforcement per CLAUDE.md requirements.

## Configuration Files

### 1. `/Users/naamanhirschfeld/workspace/spikard/pyproject.toml`

The root `pyproject.toml` contains the primary coverage configuration:

```toml
[tool.coverage.run]
branch = true
omit = [
  "packages/python/tests/*",
  "packages/python/spikard/tests/*",
  "scripts/*",
  "*/__pycache__/*",
  "*/site-packages/*",
]
source = [ "packages/python/spikard" ]
relative_files = true

[tool.coverage.report]
exclude_lines = [
  "if TYPE_CHECKING:",
  "except ImportError:",
  "pragma: no cover",
  "if sys.version_info",
  "raise NotImplementedError",
  "if __name__ == .__main__.:",
  "def __repr__",
  "class .*\\bProtocol\\):",
  "@(abc\\.)?abstractmethod",
  "def __str__",
  "raise AssertionError",
  "raise NotImplementedError",
]
skip_covered = false
precision = 2

[tool.coverage.html]
directory = "htmlcov"

[tool.coverage.lcov]
output = "coverage.lcov"
```

**Key Settings:**
- **Source**: Points to `packages/python/spikard/` (the Python package, not Rust bindings)
- **Branch Coverage**: Enabled for condition/branch analysis
- **Omit**: Excludes test files, cache, and generated code
- **Precision**: 2 decimal places for coverage percentage

### 2. `/Users/naamanhirschfeld/workspace/spikard/.coveragerc`

A backup coverage configuration file for direct coverage.py invocation:

```ini
[run]
branch = true
source = packages/python/spikard
relative_files = true
fail_under = 80  # Enforces 80% minimum threshold

[report]
exclude_lines = ...
skip_covered = false
precision = 2

[html]
directory = htmlcov

[lcov]
output = coverage.lcov
```

**Note**: The `fail_under = 80` setting enforces the minimum coverage threshold.

### 3. `/Users/naamanhirschfeld/workspace/spikard/Taskfile.yaml`

The Taskfile contains the main coverage task:

```yaml
cov:python:
  desc: "Generate Python code coverage report with 80% minimum enforcement"
  dir: packages/python
  deps:
    - build:py
  cmds:
    - uv run pytest tests/ --cov=spikard --cov-report=html --cov-report=term --cov-report=lcov:coverage.lcov --cov-fail-under=80
    - 'echo ""'
    - echo "Python coverage report generated:"
    - 'echo "  - HTML: htmlcov/index.html"'
    - 'echo "  - LCOV: coverage.lcov"'
```

**Features:**
- Depends on `build:py` to ensure bindings are built
- Generates three report formats: HTML, terminal, and LCOV
- Enforces 80% minimum coverage with `--cov-fail-under=80`
- Runs in the `packages/python` directory
- Exits with status 1 if coverage falls below 80%

## Dependencies

pytest-cov is already included in the dev dependencies in `/Users/naamanhirschfeld/workspace/spikard/pyproject.toml`:

```toml
[dependency-groups]
dev = [
  ...
  "pytest-cov>=7.0.0",
  ...
]
```

## Usage

### Run Coverage Check
```bash
# Run coverage with 80% minimum threshold enforcement
task cov:python

# Alternative: Direct command in packages/python directory
cd packages/python && uv run pytest tests/ --cov=spikard --cov-report=html --cov-report=term --cov-fail-under=80
```

### View Coverage Reports
- **HTML Report**: `packages/python/htmlcov/index.html` (interactive, shows per-file coverage)
- **LCOV Report**: `packages/python/coverage.lcov` (machine-readable format for CI/CD)
- **Terminal Output**: Printed directly to stdout during test run

## Current Coverage Status

As of the last run:

```
TOTAL                                           1827    972    47%
```

### Modules Below 80% Coverage

| Module | Coverage | Status |
|--------|----------|--------|
| spikard/__init__.py | 100% | ✅ PASS |
| spikard/_internal/__init__.py | 100% | ✅ PASS |
| spikard/types.py | 100% | ✅ PASS |
| spikard/_internal/parsed_signature.py | 91% | ✅ PASS |
| spikard/_internal/async_generator_wrapper.py | 0% | ❌ FAIL |
| spikard/_internal/constraints.py | 7% | ❌ FAIL |
| spikard/_internal/converters.py | 13% | ❌ FAIL |
| spikard/_internal/field_definition.py | 50% | ❌ FAIL |
| spikard/_internal/json_schema.py | 10% | ❌ FAIL |
| spikard/_internal/serialization.py | 75% | ❌ FAIL |
| spikard/_internal/types.py | 64% | ❌ FAIL |
| spikard/_internal/utils.py | 49% | ❌ FAIL |
| spikard/app.py | 54% | ❌ FAIL |
| spikard/background.py | 31% | ❌ FAIL |
| spikard/config.py | 88% | ✅ PASS |
| spikard/datastructures.py | 87% | ✅ PASS |
| spikard/di.py | 82% | ✅ PASS |
| spikard/introspection.py | 68% | ❌ FAIL |
| spikard/params.py | 55% | ❌ FAIL |
| spikard/request.py | 83% | ✅ PASS |
| spikard/routing.py | 16% | ❌ FAIL |
| spikard/schema.py | 57% | ❌ FAIL |
| spikard/sse.py | 25% | ❌ FAIL |
| spikard/testing.py | 68% | ❌ FAIL |
| spikard/websocket.py | 13% | ❌ FAIL |

**Total**: 47% (Target: 80%)

### Failing Test
One test is currently failing:
- `tests/test_doc_di_snippet.py::test_di_snippet_builds_app` - ValueError: 'db_pool' does not have a type annotation

This test failure prevents accurate coverage reporting; it should be fixed before running coverage checks for CI/CD.

## Configuration Details

### Excluded Lines
The following patterns are excluded from coverage reports:
- Type checking imports (`if TYPE_CHECKING:`)
- Missing imports (`except ImportError:`)
- Pragma comments (`pragma: no cover`)
- Python version checks
- Not implemented placeholders
- Main module guards (`if __name__ == "__main__":`)
- Magic methods (`__repr__`, `__str__`)
- Protocols and abstract methods

### Omitted Files
- All test files (`packages/python/tests/*`)
- Python cache (`__pycache__`)
- Site packages
- Scripts directory

## Integration with CI/CD

The `task cov:python` command is designed to be run in CI/CD pipelines:

```bash
# Will fail with exit code 1 if coverage < 80%
task cov:python

# Only generate report without threshold enforcement
cd packages/python && uv run pytest tests/ --cov=spikard --cov-report=html --cov-report=term
```

## Improving Coverage

To improve coverage for low-performing modules:

1. **Identify uncovered lines**: Open `htmlcov/index.html` and click on modules to see which lines are not covered
2. **Add tests**: Create test cases for uncovered paths in `packages/python/tests/`
3. **Run iteratively**: Use `task cov:python` to track progress
4. **Target high-impact modules first**: Focus on modules with the most uncovered statements

### Priority Modules to Test
1. `spikard/_internal/async_generator_wrapper.py` (0% - 45 statements)
2. `spikard/_internal/constraints.py` (7% - 59 statements)
3. `spikard/_internal/converters.py` (13% - 227 statements)
4. `spikard/routing.py` (16% - 73 statements)
5. `spikard/websocket.py` (13% - 45 statements)

## CLAUDE.md Compliance

This configuration enforces the CLAUDE.md requirement for **80%+ minimum coverage**:

- Coverage target: **80%** per CLAUDE.md
- Testing approach: **Fixture-driven** (defined in docs/adr/0003-validation-and-fixtures.md)
- Test framework: **pytest** with parametrized fixtures
- Coverage tool: **pytest-cov** with branch analysis enabled
- Reports: HTML (interactive), LCOV (CI/CD), and terminal output

See `/Users/naamanhirschfeld/workspace/spikard/CLAUDE.md` for complete project guidelines.
