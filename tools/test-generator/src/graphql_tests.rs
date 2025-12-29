//! GraphQL test generator
//!
//! Generates Vitest test suites from GraphQL fixtures for e2e testing.

use crate::codegen_utils::{
    escape_string, format_property_access, json_to_typescript, json_to_typescript_string,
};
use crate::graphql::{GraphQLFixture, load_graphql_fixtures};
use crate::ts_target::TypeScriptTarget;
use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Generate Node.js GraphQL test suite from fixtures
pub fn generate_graphql_tests(fixtures_dir: &Path, output_dir: &Path, target: &TypeScriptTarget) -> Result<()> {
    println!("Generating GraphQL tests...");

    let tests_dir = output_dir.join("tests");
    fs::create_dir_all(&tests_dir)?;

    let fixtures = load_graphql_fixtures(fixtures_dir)?;
    if fixtures.is_empty() {
        println!("  No GraphQL fixtures found");
        return Ok(());
    }

    // Group fixtures by category (queries, mutations, etc.)
    let mut fixtures_by_category: HashMap<String, Vec<_>> = HashMap::new();
    for fixture in fixtures {
        let category = fixture.operation_type.clone();
        fixtures_by_category.entry(category).or_insert_with(Vec::new).push(fixture);
    }

    let test_suffix = if matches!(target.runtime, crate::ts_target::Runtime::Deno) {
        "_test.ts"
    } else {
        ".spec.ts"
    };

    // Generate test file for each category
    for (category, category_fixtures) in fixtures_by_category.iter() {
        let test_content = generate_graphql_test_file(category, category_fixtures, target)?;
        let test_file = tests_dir.join(format!("graphql_{}{}", category, test_suffix));
        fs::write(&test_file, test_content)?;
        println!(
            "  ✓ Generated tests/graphql_{}{} ({} tests)",
            category,
            test_suffix,
            category_fixtures.len()
        );
    }

    Ok(())
}

/// Generate test file for a specific category
fn generate_graphql_test_file(category: &str, fixtures: &[GraphQLFixture], target: &TypeScriptTarget) -> Result<String> {
    let mut code = String::new();

    code.push_str(&format!("/**\n * GraphQL {} tests\n * @generated\n */\n\n", category));
    code.push_str(&format!(
        "import {{ TestClient }} from \"{}\";\n",
        target.binding_package
    ));

    match target.runtime {
        crate::ts_target::Runtime::Deno => {
            code.push_str("import { assertEquals } from \"jsr:@std/assert@1\";\n");
        }
        _ => {
            code.push_str("import { describe, expect, test } from \"vitest\";\n");
        }
    }

    code.push_str(&format!(
        "import {{ createAppGraphql{} }} from \"../app/main.ts\";\n\n",
        capitalize(category)
    ));

    code.push_str(&format!("describe(\"GraphQL {}\", () => {{\n", category));

    for fixture in fixtures {
        let test_function = generate_graphql_test_function(fixture)?;
        code.push_str(&test_function);
        code.push('\n');
    }

    code.push_str("});\n");

    if matches!(target.runtime, crate::ts_target::Runtime::Deno) {
        code = convert_graphql_to_deno_syntax(&code, category);
    }

    Ok(code)
}

/// Generate a single GraphQL test function
fn generate_graphql_test_function(fixture: &GraphQLFixture) -> Result<String> {
    let mut code = String::new();
    let test_name = sanitize_test_name(&fixture.name);

    code.push_str(&format!("\ttest(\"{}\", async () => {{\n", test_name));
    code.push_str(&format!(
        "\t\tconst app = createAppGraphql{}();\n",
        capitalize(&fixture.operation_type)
    ));
    code.push_str("\t\tconst client = new TestClient(app);\n\n");

    // Build GraphQL request body
    code.push_str("\t\tconst response = await client.post(\n");
    code.push_str(&format!("\t\t\t\"{}\",\n", fixture.endpoint));
    code.push_str("\t\t\t{\n");
    code.push_str(&format!("\t\t\t\tjson: {{\n"));

    // Use request from fixture
    let request = &fixture.request;
    {
        // GraphQL query (optional for persisted queries)
        code.push_str(&format!(
            "\t\t\t\t\tquery: {},\n",
            json_to_typescript_string(request.query.as_deref().unwrap_or(""))
        ));

        // Variables (if present)
        if let Some(ref variables) = request.variables {
            code.push_str(&format!(
                "\t\t\t\t\tvariables: {},\n",
                json_to_typescript(variables)
            ));
        } else {
            code.push_str("\t\t\t\t\tvariables: null,\n");
        }

        // Operation name (if present)
        if let Some(ref op_name) = request.operation_name {
            code.push_str(&format!(
                "\t\t\t\t\toperationName: \"{}\",\n",
                escape_string(op_name)
            ));
        } else {
            code.push_str("\t\t\t\t\toperationName: null,\n");
        }
    }

    code.push_str("\t\t\t\t},\n");
    code.push_str("\t\t\t},\n");
    code.push_str("\t\t);\n\n");

    // Get expected response
    let expected_response = &fixture.expected_response;
    {
        // Status code assertion
        code.push_str(&format!(
            "\t\texpect(response.statusCode).toBe({});\n",
            expected_response.status_code
        ));

        // Parse response as JSON
        code.push_str("\t\tconst responseBody = response.json();\n\n");

        // GraphQL data assertions
        if let Some(ref expected_data) = expected_response.data {
            code.push_str("\t\tconst data = responseBody.data;\n");
            generate_graphql_data_assertions(&mut code, expected_data, "data", 2);
        }

        // GraphQL errors assertions
        if let Some(ref expected_errors) = expected_response.errors {
            code.push_str("\t\tconst errors = responseBody.errors;\n");
            code.push_str("\t\texpect(errors).toBeDefined();\n");
            code.push_str(&format!(
                "\t\texpect(errors?.length).toBe({});\n",
                expected_errors.len()
            ));

            for (idx, error) in expected_errors.iter().enumerate() {
                code.push_str(&format!(
                    "\t\texpect(errors?.[{}]?.message).toContain(\"{}\");\n",
                    idx,
                    escape_string(&error.message)
                ));
            }
        } else {
            // No errors expected
            code.push_str("\t\tconst errors = responseBody.errors;\n");
            code.push_str("\t\texpect(errors?.length ?? 0).toBe(0);\n");
        }
    }

    code.push_str("\t});\n");

    Ok(code)
}

/// Generate assertions for GraphQL data
fn generate_graphql_data_assertions(code: &mut String, body: &Value, path: &str, indent_level: usize) {
    let indent = "\t".repeat(indent_level);

    match body {
        Value::Object(obj) => {
            for (key, value) in obj {
                let new_path = format_property_access(path, key);
                code.push_str(&format!("{}expect({}).toHaveProperty(\"{}\");\n", indent, path, key));

                match value {
                    Value::Object(_) => {
                        generate_graphql_data_assertions(code, value, &new_path, indent_level);
                    }
                    Value::Array(_) => {
                        generate_graphql_data_assertions(code, value, &new_path, indent_level);
                    }
                    _ => {
                        code.push_str(&format!(
                            "{}expect({}).toBe({});\n",
                            indent,
                            new_path,
                            json_to_typescript(value)
                        ));
                    }
                }
            }
        }
        Value::Array(arr) => {
            code.push_str(&format!("{}expect({}.length).toBe({});\n", indent, path, arr.len()));
            for (idx, item) in arr.iter().enumerate() {
                let new_path = format!("{}[{}]", path, idx);
                generate_graphql_data_assertions(code, item, &new_path, indent_level);
            }
        }
        _ => {
            code.push_str(&format!(
                "{}expect({}).toBe({});\n",
                indent,
                path,
                json_to_typescript(body)
            ));
        }
    }
}

/// Sanitize fixture name for test function
fn sanitize_test_name(name: &str) -> String {
    let mut result = name.replace(
        [
            ' ', '/', '.', '(', ')', '=', ',', ':', '+', '<', '>', '[', ']', '\'', '"',
        ],
        " ",
    );

    while result.contains("  ") {
        result = result.replace("  ", " ");
    }

    result.trim().to_string()
}

/// Capitalize first letter
fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

/// Convert Vitest syntax to Deno.test() syntax with proper assertion handling
fn convert_graphql_to_deno_syntax(code: &str, category: &str) -> String {
    let mut result = code.to_string();

    // Remove describe block wrapper
    let mut filtered_lines: Vec<&str> = result
        .lines()
        .filter(|line| {
            let trimmed = line.trim_start();
            !trimmed.starts_with(&format!("describe(\"GraphQL {}\"", category))
        })
        .collect();

    while matches!(filtered_lines.last(), Some(l) if l.trim().is_empty()) {
        filtered_lines.pop();
    }
    if matches!(filtered_lines.last(), Some(l) if l.trim() == "});") {
        filtered_lines.pop();
    }
    result = filtered_lines.join("\n");

    // Convert test() to Deno.test()
    let lines: Vec<String> = result
        .lines()
        .map(|line| {
            if line.contains("test(\"") {
                if let Some(start) = line.find("test(\"") {
                    if let Some(end) = line[start + 6..].find("\"") {
                        let test_name = &line[start + 6..start + 6 + end];
                        return line.replace(
                            &format!("test(\"{}\"", test_name),
                            &format!("Deno.test(\"GraphQL {}: {}\"", category, test_name),
                        );
                    }
                }
            }
            line.to_string()
        })
        .collect();
    result = lines.join("\n");

    // Convert expect() assertions to Deno std/assert helpers
    result = convert_assertions_to_deno(&result);

    // Remove import from vitest
    result = result.replace("import { describe, expect, test } from \"vitest\";\n", "");

    // Add Deno assert import if needed
    if result.contains("assertEquals(") || result.contains("assert(") {
        let import_line = "import { assertEquals, assert } from \"jsr:@std/assert@1\";\n";
        if !result.contains(import_line) {
            if let Some(pos) = result.find("import {") {
                result = format!("{}{}", import_line, &result[pos..]);
            }
        }
    }

    result
}

/// Convert individual expect() assertions to proper Deno assertions
fn convert_assertions_to_deno(code: &str) -> String {
    let mut result = String::new();
    let mut pos = 0;

    while pos < code.len() {
        // Look for expect( patterns
        if code[pos..].starts_with("expect(") {
            // Extract the expression inside expect()
            let (expr, expr_len) = extract_expect_expr(&code[pos + 7..]);
            let after_expr_pos = pos + 7 + expr_len;

            // Now check what comes after the closing paren
            if after_expr_pos >= code.len() {
                // End of string, shouldn't happen with well-formed code
                result.push_str(&format!("expect({})", expr));
                pos = after_expr_pos;
            } else {
                let remaining = &code[after_expr_pos..];

                if remaining.starts_with(".toBe(") {
                    // expect(value).toBe(expected) → assertEquals(value, expected)
                    let (expected, expected_len) = extract_expect_expr(&remaining[6..]);
                    result.push_str(&format!("assertEquals({}, {})", expr, expected));
                    pos = after_expr_pos + 6 + expected_len; // expected_len already includes closing paren
                } else if remaining.starts_with(".toContain(") {
                    // expect(value).toContain(substring) → assert(value.includes(substring))
                    let (substring, substring_len) = extract_expect_expr(&remaining[11..]);
                    result.push_str(&format!("assert({}.includes({}))", expr, substring));
                    pos = after_expr_pos + 11 + substring_len; // substring_len already includes closing paren
                } else if remaining.starts_with(".toBeDefined()") {
                    // expect(value).toBeDefined() → assert(value !== undefined)
                    result.push_str(&format!("assert({} !== undefined)", expr));
                    pos = after_expr_pos + 14; // length of ".toBeDefined()"
                } else if remaining.starts_with(".toHaveProperty(") {
                    // expect(object).toHaveProperty("key") → assert(object.hasOwnProperty("key"))
                    let (key, key_len) = extract_expect_expr(&remaining[16..]);
                    result.push_str(&format!("assert({}.hasOwnProperty({}))", expr, key));
                    pos = after_expr_pos + 16 + key_len; // key_len already includes closing paren
                } else {
                    // Fallback for unknown assertion types - just output as-is
                    result.push_str(&format!("expect({})", expr));
                    pos = after_expr_pos;
                }
            }
        } else {
            if let Some(ch) = code.chars().nth(pos) {
                result.push(ch);
                pos += ch.len_utf8();
            } else {
                break;
            }
        }
    }

    result
}

/// Extract the expression inside parentheses, handling nested structures
/// Returns the expression and the number of bytes consumed (including closing paren)
fn extract_expect_expr(input: &str) -> (String, usize) {
    let mut expr = String::new();
    let mut paren_depth = 1; // We're already inside the opening paren
    let mut in_string = false;
    let mut string_char = '\0';
    let mut byte_count = 0;
    let mut iter = input.char_indices().peekable();

    while let Some((idx, ch)) = iter.next() {
        byte_count = idx + ch.len_utf8();
        match ch {
            '"' | '\'' | '`' if !in_string => {
                in_string = true;
                string_char = ch;
                expr.push(ch);
            }
            c if in_string && c == string_char => {
                in_string = false;
                expr.push(ch);
            }
            '\\' if in_string => {
                expr.push(ch);
                if let Some((next_idx, next_ch)) = iter.next() {
                    expr.push(next_ch);
                    byte_count = next_idx + next_ch.len_utf8();
                }
            }
            '(' if !in_string => {
                paren_depth += 1;
                expr.push(ch);
            }
            ')' if !in_string => {
                paren_depth -= 1;
                if paren_depth == 0 {
                    break;
                }
                expr.push(ch);
            }
            _ => {
                expr.push(ch);
            }
        }
    }

    (expr, byte_count)
}
