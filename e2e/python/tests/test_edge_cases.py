"""E2E tests for edge_cases."""

from spikard.testing import TestClient
from app.main import (
    create_app_edge_cases_11_utf8_query_parameter,
    create_app_edge_cases_12_percent_encoded_special_chars,
    create_app_edge_cases_13_empty_string_query_param_preserved,
    create_app_edge_cases_14_large_integer_boundary,
    create_app_edge_cases_15_float_precision_preservation,
    create_app_edge_cases_16_negative_zero_handling,
    create_app_edge_cases_17_extremely_long_string,
    create_app_edge_cases_18_unicode_normalization,
    create_app_edge_cases_19_emoji_in_strings,
    create_app_edge_cases_20_null_byte_in_string,
    create_app_edge_cases_21_scientific_notation_number,
    create_app_edge_cases_22_leading_zeros_integer,
    create_app_edge_cases_23_deeply_nested_json_limit,
    create_app_edge_cases_24_array_with_holes,
    create_app_edge_cases_deeply_nested_structure_10_levels,
    create_app_edge_cases_empty_and_null_value_handling,
    create_app_edge_cases_float_precision_and_rounding,
    create_app_edge_cases_large_integer_boundary_values,
    create_app_edge_cases_special_string_values_and_escaping,
    create_app_edge_cases_unicode_and_emoji_handling,
)


async def test_19_emoji_in_strings() -> None:
    """Emoji characters should be handled correctly in string fields."""

    app = create_app_edge_cases_19_emoji_in_strings()
    client = TestClient(app)

    json_data = {"text": "Hello 👋 World 🌍"}
    response = await client.post("/messages", json=json_data)

    assert response.status_code == 201
    response_data = response.json()
    assert "text" in response_data
    assert response_data["text"] == "Hello 👋 World 🌍"


async def test_12_percent_encoded_special_chars() -> None:
    """Percent-encoded special characters should be decoded correctly."""

    app = create_app_edge_cases_12_percent_encoded_special_chars()
    client = TestClient(app)

    params = {
        "term": "hi there",
    }
    response = await client.get("/search?term=hi%20there", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert "term" in response_data
    assert response_data["term"] == "hi there"


async def test_special_string_values_and_escaping() -> None:
    """Tests handling of special characters, null bytes, and escape sequences."""

    app = create_app_edge_cases_special_string_values_and_escaping()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/json",
    }
    json_data = {
        "backslashes": "C:\\\\Users\\\\Path",
        "empty_string": "",
        "quotes": "He said \"hello\" and 'goodbye'",
        "special_chars": "!@#$%^&*()_+-=[]{}|;':\",./<>?",
        "tabs_newlines": "line1\n\tline2\r\nline3",
        "unicode_escapes": "\\u0048\\u0065\\u006c\\u006c\\u006f",
        "whitespace": "   ",
    }
    response = await client.post("/strings/", headers=headers, json=json_data)

    assert response.status_code == 200
    response_data = response.json()
    assert "backslashes" in response_data
    assert response_data["backslashes"] == "C:\\\\Users\\\\Path"
    assert "empty_string" in response_data
    assert response_data["empty_string"] == ""
    assert "quotes" in response_data
    assert response_data["quotes"] == "He said \"hello\" and 'goodbye'"
    assert "special_chars" in response_data
    assert response_data["special_chars"] == "!@#$%^&*()_+-=[]{}|;':\",./<>?"
    assert "tabs_newlines" in response_data
    assert response_data["tabs_newlines"] == "line1\n\tline2\r\nline3"
    assert "unicode_escapes" in response_data
    assert response_data["unicode_escapes"] == "Hello"
    assert "whitespace" in response_data
    assert response_data["whitespace"] == "   "


async def test_15_float_precision_preservation() -> None:
    """High-precision floating point numbers should be preserved without loss."""

    app = create_app_edge_cases_15_float_precision_preservation()
    client = TestClient(app)

    json_data = {"value": 3.141592653589793}
    response = await client.post("/calculate", json=json_data)

    assert response.status_code == 201
    response_data = response.json()
    assert "value" in response_data
    assert response_data["value"] == 3.141592653589793


async def test_13_empty_string_query_param_preserved() -> None:
    """Empty string query parameter should be preserved, not treated as missing."""

    app = create_app_edge_cases_13_empty_string_query_param_preserved()
    client = TestClient(app)

    params = {
        "filter": "",
    }
    response = await client.get("/items?filter=", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert "filter" in response_data
    assert response_data["filter"] == ""


async def test_24_array_with_holes() -> None:
    """Array indices with gaps should be rejected in form data."""

    app = create_app_edge_cases_24_array_with_holes()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/x-www-form-urlencoded",
    }
    json_data = "items[0]=first&items[2]=third&items[5]=sixth"
    response = await client.post("/items", headers=headers, json=json_data)

    assert response.status_code == 200
    response_data = response.json()
    assert "items" in response_data
    assert len(response_data["items"]) == 3
    assert response_data["items"][0] == "first"
    assert response_data["items"][1] == "third"
    assert response_data["items"][2] == "sixth"


async def test_21_scientific_notation_number() -> None:
    """Numbers in scientific notation should be parsed correctly."""

    app = create_app_edge_cases_21_scientific_notation_number()
    client = TestClient(app)

    json_data = {"value": 123000.0}
    response = await client.post("/calculate", json=json_data)

    assert response.status_code == 201
    response_data = response.json()
    assert "value" in response_data
    assert response_data["value"] == 123000


async def test_float_precision_and_rounding() -> None:
    """Tests floating point precision and rounding behavior."""

    app = create_app_edge_cases_float_precision_and_rounding()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/json",
    }
    json_data = {
        "expected_sum": 0.3,
        "precise_value": 3.141592653589793,
        "value1": 0.1,
        "value2": 0.2,
        "very_large": 1.7976931348623157e308,
        "very_small": 1e-10,
    }
    response = await client.post("/calculations/", headers=headers, json=json_data)

    assert response.status_code == 200
    response_data = response.json()
    assert "precise_value" in response_data
    assert response_data["precise_value"] == 3.141592653589793
    assert "sum" in response_data
    assert response_data["sum"] == 0.30000000000000004
    assert "very_large" in response_data
    assert response_data["very_large"] == 1.7976931348623157e308
    assert "very_small" in response_data
    assert response_data["very_small"] == 1e-10


async def test_unicode_and_emoji_handling() -> None:
    """Tests proper handling of Unicode characters and emojis."""

    app = create_app_edge_cases_unicode_and_emoji_handling()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/json; charset=utf-8",
    }
    json_data = {
        "description": "Best café in München 🇩🇪",
        "emoji_reactions": "👍❤️😂🎉",
        "name": "Coffee Shop ☕",
        "tags": ["食べ物", "音楽", "💰"],
    }
    response = await client.post("/items/", headers=headers, json=json_data)

    assert response.status_code == 200
    response_data = response.json()
    assert "description" in response_data
    assert response_data["description"] == "Best café in München 🇩🇪"
    assert "emoji_reactions" in response_data
    assert response_data["emoji_reactions"] == "👍❤️😂🎉"
    assert "id" in response_data
    assert response_data["id"] == 1
    assert "name" in response_data
    assert response_data["name"] == "Coffee Shop ☕"
    assert "tags" in response_data
    assert len(response_data["tags"]) == 3
    assert response_data["tags"][0] == "食べ物"
    assert response_data["tags"][1] == "音楽"
    assert response_data["tags"][2] == "💰"


async def test_17_extremely_long_string() -> None:
    """Very long string values should be validated against maxLength."""

    app = create_app_edge_cases_17_extremely_long_string()
    client = TestClient(app)

    json_data = {
        "content": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
    }
    response = await client.post("/text", json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_11_utf8_query_parameter() -> None:
    """Query parameter with UTF-8 characters should be handled correctly."""

    app = create_app_edge_cases_11_utf8_query_parameter()
    client = TestClient(app)

    params = {
        "term": "café",
    }
    response = await client.get("/search", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert "term" in response_data
    assert response_data["term"] == "café"


async def test_18_unicode_normalization() -> None:
    """Unicode characters with combining diacritics should be handled correctly."""

    app = create_app_edge_cases_18_unicode_normalization()
    client = TestClient(app)

    json_data = {"name": "café"}
    response = await client.post("/users", json=json_data)

    assert response.status_code == 201
    response_data = response.json()
    assert "name" in response_data
    assert response_data["name"] == "café"


async def test_20_null_byte_in_string() -> None:
    """Null byte character in strings should be rejected for security."""

    app = create_app_edge_cases_20_null_byte_in_string()
    client = TestClient(app)

    json_data = {"filename": "file\0.txt"}
    response = await client.post("/files", json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_23_deeply_nested_json_limit() -> None:
    """Extremely deeply nested JSON should be rejected to prevent DoS."""

    app = create_app_edge_cases_23_deeply_nested_json_limit()
    client = TestClient(app)

    json_data = {
        "nested": {
            "nested": {
                "nested": {
                    "nested": {
                        "nested": {
                            "nested": {
                                "nested": {
                                    "nested": {
                                        "nested": {
                                            "nested": {
                                                "nested": {
                                                    "nested": {
                                                        "nested": {
                                                            "nested": {
                                                                "nested": {
                                                                    "nested": {
                                                                        "nested": {
                                                                            "nested": {
                                                                                "nested": {
                                                                                    "nested": {
                                                                                        "nested": {
                                                                                            "nested": {
                                                                                                "nested": {
                                                                                                    "nested": {
                                                                                                        "nested": {
                                                                                                            "nested": {
                                                                                                                "nested": {
                                                                                                                    "nested": {
                                                                                                                        "nested": {
                                                                                                                            "nested": {
                                                                                                                                "nested": {
                                                                                                                                    "nested": {
                                                                                                                                        "nested": {
                                                                                                                                            "nested": {
                                                                                                                                                "nested": {
                                                                                                                                                    "nested": {
                                                                                                                                                        "nested": {
                                                                                                                                                            "nested": {
                                                                                                                                                                "nested": {
                                                                                                                                                                    "nested": {
                                                                                                                                                                        "nested": {
                                                                                                                                                                            "nested": {
                                                                                                                                                                                "nested": {
                                                                                                                                                                                    "nested": {
                                                                                                                                                                                        "nested": {
                                                                                                                                                                                            "nested": {
                                                                                                                                                                                                "nested": {
                                                                                                                                                                                                    "nested": {
                                                                                                                                                                                                        "nested": {
                                                                                                                                                                                                            "value": "deep"
                                                                                                                                                                                                        }
                                                                                                                                                                                                    }
                                                                                                                                                                                                }
                                                                                                                                                                                            }
                                                                                                                                                                                        }
                                                                                                                                                                                    }
                                                                                                                                                                                }
                                                                                                                                                                            }
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                }
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    response = await client.post("/data", json=json_data)

    assert response.status_code == 400
    response_data = response.json()
    assert "error" in response_data
    assert response_data["error"] == "Request body exceeds maximum nesting depth of 32"


async def test_14_large_integer_boundary() -> None:
    """Very large integer at JavaScript MAX_SAFE_INTEGER boundary should be handled."""

    app = create_app_edge_cases_14_large_integer_boundary()
    client = TestClient(app)

    params = {
        "id": "9007199254740991",
    }
    response = await client.get("/items", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert "id" in response_data
    assert response_data["id"] == 9007199254740991


async def test_22_leading_zeros_integer() -> None:
    """Integer values with leading zeros should be parsed as decimal (not octal)."""

    app = create_app_edge_cases_22_leading_zeros_integer()
    client = TestClient(app)

    params = {
        "value": "0123",
    }
    response = await client.get("/data", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert "value" in response_data
    assert response_data["value"] == 123


async def test_large_integer_boundary_values() -> None:
    """Tests handling of very large integer values near system limits."""

    app = create_app_edge_cases_large_integer_boundary_values()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/json",
    }
    json_data = {
        "large_int": 9223372036854775807,
        "max_safe_int": 9007199254740991,
        "negative_large": -9223372036854775808,
    }
    response = await client.post("/numbers/", headers=headers, json=json_data)

    assert response.status_code == 200
    response_data = response.json()
    assert "large_int" in response_data
    assert response_data["large_int"] == 9223372036854775807
    assert "max_safe_int" in response_data
    assert response_data["max_safe_int"] == 9007199254740991
    assert "negative_large" in response_data
    assert response_data["negative_large"] == -9223372036854775808


async def test_deeply_nested_structure_10_levels() -> None:
    """Tests handling of deeply nested JSON objects."""

    app = create_app_edge_cases_deeply_nested_structure_10_levels()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/json",
    }
    json_data = {
        "level1": {
            "level2": {
                "level3": {
                    "level4": {
                        "level5": {
                            "level6": {"level7": {"level8": {"level9": {"level10": {"depth": 10, "value": "deep"}}}}}
                        }
                    }
                }
            }
        }
    }
    response = await client.post("/nested/", headers=headers, json=json_data)

    assert response.status_code == 200
    response_data = response.json()
    assert "max_depth" in response_data
    assert response_data["max_depth"] == 10
    assert "message" in response_data
    assert response_data["message"] == "Processed deeply nested structure"
    assert "value_found" in response_data
    assert response_data["value_found"] == "deep"


async def test_empty_and_null_value_handling() -> None:
    """Tests distinction between null, empty, and missing values."""

    app = create_app_edge_cases_empty_and_null_value_handling()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/json",
    }
    json_data = {
        "empty_array": [],
        "empty_object": {},
        "empty_string": "",
        "explicit_null": None,
        "false_boolean": False,
        "zero_number": 0,
    }
    response = await client.post("/nulls/", headers=headers, json=json_data)

    assert response.status_code == 200
    response_data = response.json()
    assert "empty_array_length" in response_data
    assert response_data["empty_array_length"] == 0
    assert "empty_object_keys" in response_data
    assert response_data["empty_object_keys"] == 0
    assert "empty_string_length" in response_data
    assert response_data["empty_string_length"] == 0
    assert "explicit_null_is_null" in response_data
    assert response_data["explicit_null_is_null"] == True
    assert "false_is_false" in response_data
    assert response_data["false_is_false"] == True
    assert "zero_is_falsy" in response_data
    assert response_data["zero_is_falsy"] == True


async def test_16_negative_zero_handling() -> None:
    """Negative zero (-0.0) should be handled correctly in numeric fields."""

    app = create_app_edge_cases_16_negative_zero_handling()
    client = TestClient(app)

    json_data = {"offset": -0.0}
    response = await client.post("/data", json=json_data)

    assert response.status_code == 201
    response_data = response.json()
    assert "offset" in response_data
    assert response_data["offset"] == 0
