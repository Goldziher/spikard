//! GraphQL test generator
//!
//! Generates Vitest test suites from GraphQL fixtures for e2e testing.

use crate::codegen_utils::{escape_string, format_property_access, json_to_typescript, json_to_typescript_string};
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
        fixtures_by_category
            .entry(category)
            .or_insert_with(Vec::new)
            .push(fixture);
    }

    let test_suffix = ".spec.ts";

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
fn generate_graphql_test_file(
    category: &str,
    fixtures: &[GraphQLFixture],
    target: &TypeScriptTarget,
) -> Result<String> {
    let mut code = String::new();

    code.push_str(&format!("/**\n * GraphQL {} tests\n * @generated\n */\n\n", category));
    code.push_str(&format!(
        "import {{ TestClient }} from \"{}\";\n",
        target.binding_package
    ));
    code.push_str("import { describe, expect, test } from \"vitest\";\n");

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
    code.push_str(&format!(
        "\t\t\t\theaders: {{ \"x-spikard-fixture\": \"{}\" }},\n",
        escape_string(&fixture.name)
    ));
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
            code.push_str(&format!("\t\t\t\t\tvariables: {},\n", json_to_typescript(variables)));
        } else {
            code.push_str("\t\t\t\t\tvariables: null,\n");
        }

        // Operation name (if present)
        if let Some(ref op_name) = request.operation_name {
            code.push_str(&format!("\t\t\t\t\toperationName: \"{}\",\n", escape_string(op_name)));
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
