//! Java HTTP e2e test generation (server-pattern slice from alef core).
//!
//! This module owns the server-pattern files for spikard's Java e2e suite:
//! - `HarnessMain.java` — spawns the SUT as an HTTP server via the binding
//! - `FixtureLoader.java` — loads fixture JSON files from classpath resources
//! - `*Test.java` — per-category JUnit 5 test files with server-pattern `@BeforeAll`/`@AfterAll` harness spawn
//!
//! The shared client-pattern files (pom.xml, project scaffold, mock-server listener, test method bodies)
//! stay generic in alef. Only the server-pattern slice (harness spawn and per-test fixture loading) lives here.
//!
//! For now, this module emits HarnessMain.java and FixtureLoader.java only. Test file generation
//! with full @BeforeAll/@AfterAll harness spawn blocks remains in alef; after full port completion
//! and alef updates to skip Java test emission, this module will handle full test file generation.

use alef::GeneratedFile;
use alef::ResolvedCrateConfig;
use alef::core::hash::{self, CommentStyle};
use alef::e2e::config::E2eConfig;
use alef::e2e::fixture::FixtureGroup;
use anyhow::Result;
use minijinja::{Environment, context};
use std::path::PathBuf;

// ---------------------------------------------------------------------------
// Template environment
// ---------------------------------------------------------------------------

/// Build the private template environment holding the Java HTTP templates.
fn make_env() -> Environment<'static> {
    let mut env = Environment::new();
    env.set_trim_blocks(true);
    env.set_lstrip_blocks(true);
    env.set_keep_trailing_newline(true);
    env.add_template_owned(
        "java/harness_main.jinja".to_owned(),
        include_str!("../../templates/java/harness_main.jinja").to_owned(),
    )
    .expect("built-in template parse failed");
    env
}

/// Render a named template from the local environment.
fn render(env: &Environment<'static>, name: &str, ctx: minijinja::Value) -> String {
    env.get_template(name)
        .expect("template must exist")
        .render(ctx)
        .unwrap_or_default()
}

// ---------------------------------------------------------------------------
// HarnessMain renderer (ported from alef `java/project.rs::render_harness_main`).
// ---------------------------------------------------------------------------

/// Render HarnessMain.java for server-pattern e2e tests.
///
/// This harness loads fixtures from classpath resources, registers handlers via
/// the app binding, and serves on a port read from SUT_URL env var or the
/// configured default. Tests hit the real SUT at /fixtures/<fixture_id>{path}.
///
/// Ported verbatim from alef's `java/project.rs::render_harness_main`.
#[must_use]
fn render_harness_main(
    e2e_config: &E2eConfig,
    groups: &[FixtureGroup],
    java_group_id: &str,
    binding_pkg: &str,
) -> String {
    let host = &e2e_config.harness.host;
    let port = e2e_config.harness.port;
    let app_class_owned = e2e_config.harness.app_class_for_lang("java");
    let app_class = app_class_owned.as_deref().unwrap_or("App");
    let run_method_owned = e2e_config.harness.run_method_for_lang("java");
    let run_method = run_method_owned.as_deref().unwrap_or("run");
    // Java methods are camelCase by convention. `register_method_idiomatic`
    // honors `[crates.e2e.harness.overrides.java]` first, then converts the
    // canonical name to camelCase (e.g. `register_route` → `registerRoute`).
    // The actual Java facade method is `registerAppRoute`, so expand bare `route` to it.
    let register_method = e2e_config
        .harness
        .register_method_idiomatic("java")
        .unwrap_or_else(|| "registerAppRoute".to_string());
    let register_method = if register_method == "route" {
        "registerAppRoute".to_string()
    } else {
        register_method
    };
    let body_field = &e2e_config.harness.response_body_field;

    // Collect all HTTP fixtures for this harness to register.
    let mut fixture_ids: Vec<String> = Vec::new();
    for group in groups {
        for fixture in &group.fixtures {
            if fixture.http.is_some() {
                fixture_ids.push(fixture.id.clone());
            }
        }
    }

    let default_harness_port = E2eConfig::default().harness.port;

    let ctx = context! {
        java_group_id => java_group_id,
        binding_pkg => binding_pkg,
        app_class => app_class,
        run_method => run_method,
        register_method => register_method.as_str(),
        response_body_field => body_field.as_str(),
        host => host,
        port => port,
        default_port => default_harness_port,
        fixture_ids => fixture_ids,
    };

    render(&make_env(), "java/harness_main.jinja", ctx)
}

// ---------------------------------------------------------------------------
// FixtureLoader renderer (ported from alef `java/project.rs::render_fixture_loader`).
// ---------------------------------------------------------------------------

/// Render FixtureLoader.java helper that loads fixture JSON files from classpath.
///
/// This avoids inlining all fixtures as Java string literals, which would exceed
/// Java's 65535-byte limit for large fixture sets. Fixtures are stored as individual
/// JSON files in src/test/resources/fixtures/ and loaded at test runtime.
///
/// Ported verbatim from alef's `java/project.rs::render_fixture_loader`.
#[must_use]
fn render_fixture_loader(java_group_id: &str) -> String {
    let header = hash::header(CommentStyle::DoubleSlash);
    let mut out = header;
    out.push_str(&format!("package {java_group_id}.e2e;\n\n"));
    out.push_str("import com.fasterxml.jackson.databind.JsonNode;\n");
    out.push_str("import com.fasterxml.jackson.databind.ObjectMapper;\n");
    out.push_str("import java.io.IOException;\n");
    out.push_str("import java.io.InputStream;\n");
    out.push_str("import java.util.HashMap;\n");
    out.push_str("import java.util.Map;\n");
    out.push('\n');
    out.push_str("/**\n");
    out.push_str(" * Helper class for loading fixture JSON files from classpath.\n");
    out.push_str(" *\n");
    out.push_str(" * Fixtures are stored as individual JSON files in src/test/resources/fixtures/\n");
    out.push_str(" * to avoid exceeding Java's 65KB string literal limit.\n");
    out.push_str(" */\n");
    out.push_str("public class FixtureLoader {\n");
    out.push_str("    private static final ObjectMapper MAPPER = new ObjectMapper();\n");
    out.push('\n');
    out.push_str("    /**\n");
    out.push_str("     * Load a single fixture by ID from classpath resources.\n");
    out.push_str("     *\n");
    out.push_str("     * @param fixtureId the fixture identifier (e.g., \"smoke_basic\")\n");
    out.push_str("     * @return the parsed fixture as a JsonNode, or null if not found\n");
    out.push_str("     */\n");
    out.push_str("    public static JsonNode loadFixture(String fixtureId) {\n");
    out.push_str("        String resourcePath = \"/fixtures/\" + fixtureId + \".json\";\n");
    out.push_str("        try (InputStream is = FixtureLoader.class.getResourceAsStream(resourcePath)) {\n");
    out.push_str("            if (is == null) {\n");
    out.push_str("                System.err.println(\"Fixture not found: \" + fixtureId);\n");
    out.push_str("                return null;\n");
    out.push_str("            }\n");
    out.push_str("            return MAPPER.readTree(is);\n");
    out.push_str("        } catch (IOException e) {\n");
    out.push_str(
        "            System.err.println(\"Failed to load fixture \" + fixtureId + \": \" + e.getMessage());\n",
    );
    out.push_str("            e.printStackTrace();\n");
    out.push_str("            return null;\n");
    out.push_str("        }\n");
    out.push_str("    }\n");
    out.push('\n');
    out.push_str("    /**\n");
    out.push_str("     * Load all fixtures from the classpath resources directory.\n");
    out.push_str("     *\n");
    out.push_str("     * @return a map of fixture IDs to parsed fixture JsonNodes\n");
    out.push_str("     */\n");
    out.push_str("    public static Map<String, JsonNode> loadAllFixtures() {\n");
    out.push_str("        Map<String, JsonNode> fixtures = new HashMap<>();\n");
    out.push_str("        // Note: Loading all fixtures requires iterating the classpath.\n");
    out.push_str("        // For typical e2e test suites, only the fixtures needed by the\n");
    out.push_str("        // specific test class should be loaded via loadFixture(id).\n");
    out.push_str("        return fixtures;\n");
    out.push_str("    }\n");
    out.push_str("}\n");
    out
}

// ---------------------------------------------------------------------------
// Public emit entry point
// ---------------------------------------------------------------------------

/// Emit Java's server-pattern files.
///
/// Returns the server-pattern `GeneratedFile`s at `e2e/java/...`, gated identically
/// to alef's prior emission: HTTP fixtures present and a harness import configured.
///
/// Files produced:
/// - `e2e/java/src/test/java/{package}/e2e/HarnessMain.java`
/// - `e2e/java/src/test/java/{package}/e2e/FixtureLoader.java`
///
/// For this initial port, Java test file generation is left to alef. After alef is
/// updated to skip Java test emission when spikard-e2e-http is present, this module
/// will be extended to emit full test files with harness spawn blocks.
pub fn emit(
    groups: &[FixtureGroup],
    e2e_config: &E2eConfig,
    config: &ResolvedCrateConfig,
) -> Result<Vec<GeneratedFile>> {
    let has_http = groups.iter().flat_map(|g| g.fixtures.iter()).any(|f| f.http.is_some());
    if !has_http || e2e_config.harness.imports.is_empty() {
        return Ok(vec![]);
    }

    let java_group_id = config.java_group_id();
    let binding_pkg = config.java_package();

    // Base paths: e2e/java/
    let output_base = PathBuf::from(e2e_config.effective_output()).join("java");
    let mut test_base = output_base.join("src").join("test").join("java");
    for segment in java_group_id.split('.') {
        test_base = test_base.join(segment);
    }
    let test_base = test_base.join("e2e");

    let files: Vec<GeneratedFile> = vec![
        // Emit HarnessMain.java (server-pattern harness spawner)
        GeneratedFile {
            path: test_base.join("HarnessMain.java"),
            content: render_harness_main(e2e_config, groups, &java_group_id, &binding_pkg),
            generated_header: true,
        },
        // Emit FixtureLoader.java (fixture resource loader helper)
        GeneratedFile {
            path: test_base.join("FixtureLoader.java"),
            content: render_fixture_loader(&java_group_id),
            generated_header: true,
        },
    ];

    Ok(files)
}
