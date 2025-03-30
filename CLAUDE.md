# Spikard

## Project Description

Spikard is a universal LLM client that provides a standard interface for interacting with
various LLM providers.

## Python Standards

- Use Python 3.9+ features
- Target Python 3.12+ typing using `from __future__ import annotations`
- Fully type all function arguments and return values
- Add docstrings using Google style format
- DO NOT ADD DOCSTRINGS IN TESTS

## Project Structure

- Base abstract LLM client class in `base.py`
- Custom exceptions in `exceptions.py`
- Provider-specific implementations extend the base class
- Maintain 100% test coverage for all code

## Testing

```bash
uv run python -m pytest                                      # Run all tests
uv run python -m pytest tests/<path> -v                      # Verbose test output
uv run python -m pytest -xvs tests/path/to/test.py::test_name # Specific test function
uv run python -m pytest --cov=spikard                        # Run tests with coverage
```

## Code Quality

```bash
uv run pre-commit run --all-files # Run all checks (ruff, mypy, etc.)
uv run ruff check --fix .         # Lint code
uv run ruff format .              # Format code
uv run mypy                       # Type checking
```

## Development Workflow

- Follow the abstract base class pattern for new provider implementations
- Ensure 100% test coverage for all new code
- Use proper error handling with custom exceptions
- Maintain backward compatibility
- Keep dependencies minimal by using optional dependencies for providers

**IMPORTANT**:

- Do not add inline comments
- Always use proper type annotations
- Provider-specific implementations should be in separate modules
- All public APIs must be properly documented and typed
