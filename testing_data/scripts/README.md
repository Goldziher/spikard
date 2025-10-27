# Testing Data Scripts

Utility scripts for working with test fixtures.

## 📋 Scripts

### `validate.py`

Validates all test fixtures against their JSON schemas.

**Usage:**
```bash
# Validate all fixtures in all categories
python testing_data/scripts/validate.py

# Validate a specific category
python testing_data/scripts/validate.py multipart
python testing_data/scripts/validate.py query_params
```

**Requirements:**
```bash
pip install jsonschema
```

**Output:**
- ✅ Valid fixtures
- ❌ Invalid fixtures with detailed error messages
- Exit code 0 if all valid, 1 if any invalid

### `loader.py`

Helper utilities for loading fixtures in test code.

**Functions:**

- `load_fixture(category, name)` - Load a single fixture
- `load_all_fixtures(category)` - Load all fixtures in a category
- `get_fixture_names(category)` - Get list of fixture names
- `load_file_content(category, filename)` - Load referenced file content
- `pytest_parametrize_fixtures(category)` - Generate pytest parametrize args

**Usage in tests:**
```python
from testing_data.scripts.loader import load_fixture, pytest_parametrize_fixtures

# Single fixture
def test_upload():
    fixture = load_fixture("multipart", "01_simple_file_upload")
    assert fixture["name"] == "Simple file upload"

# Parametrize all fixtures
@pytest.mark.parametrize(*pytest_parametrize_fixtures("multipart"))
def test_all_multipart(fixture):
    assert "request" in fixture
    assert "expected_response" in fixture
```

## 🔧 Development

### Adding a New Script

1. Create the script in this directory
2. Make it executable: `chmod +x script_name.py`
3. Add shebang: `#!/usr/bin/env python3`
4. Document it here in this README
5. Add usage examples

### Script Guidelines

- Use type hints
- Add docstrings
- Handle errors gracefully
- Provide clear error messages
- Support both CLI and programmatic usage
- Include usage examples in docstrings
