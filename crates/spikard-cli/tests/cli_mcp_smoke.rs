use assert_cmd::Command;
use predicates::prelude::*;
use rmcp::{
    ClientHandler, RoleClient, ServiceExt,
    model::{CallToolRequestParams, ClientInfo},
    service::RunningService,
};
use serde_json::json;
use std::process::Stdio;
use tempfile::TempDir;
use tokio::{
    process::Command as TokioCommand,
    time::{Duration, timeout},
};

#[derive(Debug, Clone, Default)]
struct DummyClientHandler;

impl ClientHandler for DummyClientHandler {
    fn get_info(&self) -> ClientInfo {
        ClientInfo::default()
    }
}

async fn spawn_stdio_client() -> anyhow::Result<(RunningService<RoleClient, DummyClientHandler>, tokio::process::Child)>
{
    let mut child = TokioCommand::new(assert_cmd::cargo::cargo_bin!("spikard"))
        .arg("mcp")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        .spawn()?;

    let stdout = child.stdout.take().expect("child stdout should be piped");
    let stdin = child.stdin.take().expect("child stdin should be piped");
    let client = DummyClientHandler.serve((stdout, stdin)).await?;
    Ok((client, child))
}

async fn shutdown_client(
    client: &mut RunningService<RoleClient, DummyClientHandler>,
    child: &mut tokio::process::Child,
) -> anyhow::Result<()> {
    let _ = client.close().await?;
    let _ = timeout(Duration::from_secs(5), child.wait()).await?;
    Ok(())
}

#[test]
fn spikard_mcp_help_exits_successfully() {
    let mut command = Command::new(assert_cmd::cargo::cargo_bin!("spikard"));
    command
        .args(["mcp", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Start the Spikard MCP server"))
        .stdout(predicate::str::contains("--transport"));
}

#[test]
fn spikard_mcp_rejects_unknown_transport() {
    let mut command = Command::new(assert_cmd::cargo::cargo_bin!("spikard"));
    command
        .args(["mcp", "--transport", "udp"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Unknown MCP transport 'udp'"));
}

#[test]
fn spikard_mcp_stdio_path_reaches_server_startup() {
    let mut command = Command::new(assert_cmd::cargo::cargo_bin!("spikard"));
    command
        .arg("mcp")
        .write_stdin("")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Failed to start MCP server over stdio"))
        .stderr(predicate::str::contains("initialize request"));
}

#[tokio::test]
async fn spikard_mcp_stdio_supports_initialize_list_and_call() -> anyhow::Result<()> {
    let (mut client, mut child) = spawn_stdio_client().await?;

    let tools = client.list_all_tools().await?;
    let tool_names = tools.iter().map(|tool| tool.name.as_ref()).collect::<Vec<_>>();
    assert!(tool_names.contains(&"get_features"));
    assert!(tool_names.contains(&"generate_openapi"));
    assert!(tool_names.contains(&"generate_asyncapi_bundle"));

    let result = client
        .call_tool(
            CallToolRequestParams::new("get_features").with_arguments(json!({}).as_object().expect("object").clone()),
        )
        .await?;

    let text = result
        .content
        .first()
        .and_then(|content| content.raw.as_text())
        .map(|content| content.text.as_str())
        .expect("expected text tool result");
    assert!(text.contains("\"Rust\""));
    assert!(text.contains("\"Python\""));
    assert!(text.contains("\"Elixir\""));

    shutdown_client(&mut client, &mut child).await?;
    Ok(())
}

#[tokio::test]
async fn spikard_mcp_stdio_can_initialize_a_project() -> anyhow::Result<()> {
    let tmp = TempDir::new()?;
    let project_name = "mcp_init_demo";

    let (mut client, mut child) = spawn_stdio_client().await?;
    let result = client
        .call_tool(
            CallToolRequestParams::new("init_project").with_arguments(
                json!({
                    "name": project_name,
                    "language": "elixir",
                    "directory": tmp.path().display().to_string()
                })
                .as_object()
                .expect("object")
                .clone(),
            ),
        )
        .await?;

    let text = result
        .content
        .first()
        .and_then(|content| content.raw.as_text())
        .map(|content| content.text.as_str())
        .expect("expected text tool result");
    let project_dir = tmp.path().join(project_name);

    assert!(project_dir.exists());
    assert!(project_dir.join("mix.exs").exists());
    assert!(project_dir.join("lib").join("mcp_init_demo.ex").exists());
    assert!(project_dir.join("test").join("mcp_init_demo_test.exs").exists());
    assert!(text.contains("\"files_created\""));
    assert!(text.contains("\"next_steps\""));

    shutdown_client(&mut client, &mut child).await?;
    Ok(())
}

#[tokio::test]
#[allow(clippy::too_many_lines)]
async fn spikard_mcp_stdio_init_project_creates_expected_structures_for_each_binding() -> anyhow::Result<()> {
    let tmp = TempDir::new()?;
    let (mut client, mut child) = spawn_stdio_client().await?;

    let cases = [
        (
            "python",
            "mcp_python_demo",
            vec![
                "pyproject.toml",
                "README.md",
                ".gitignore",
                "src/mcp_python_demo/__init__.py",
                "src/mcp_python_demo/app.py",
                "tests/test_app.py",
            ],
        ),
        (
            "typescript",
            "mcp-ts-demo",
            vec![
                "package.json",
                "tsconfig.json",
                "vitest.config.ts",
                ".gitignore",
                "README.md",
                "src/app.ts",
                "src/server.ts",
                "tests/app.spec.ts",
            ],
        ),
        (
            "rust",
            "mcp_rust_demo",
            vec![
                "Cargo.toml",
                "README.md",
                ".gitignore",
                "src/main.rs",
                "src/lib.rs",
                "tests/integration_test.rs",
            ],
        ),
        (
            "ruby",
            "mcp_ruby_demo",
            vec![
                "Gemfile",
                ".gitignore",
                "README.md",
                "bin/server",
                "lib/mcp_ruby_demo.rb",
                "sig/mcp_ruby_demo.rbs",
                "spec/mcp_ruby_demo_spec.rb",
                "spec/spec_helper.rb",
                ".rspec",
                "Rakefile",
            ],
        ),
        (
            "php",
            "mcp_php_demo",
            vec![
                "composer.json",
                "phpstan.neon",
                "phpunit.xml",
                ".gitignore",
                "README.md",
                "src/AppController.php",
                "bin/server.php",
                "tests/AppTest.php",
            ],
        ),
        (
            "elixir",
            "mcp_elixir_demo",
            vec![
                "mix.exs",
                ".formatter.exs",
                ".gitignore",
                "lib/mcp_elixir_demo.ex",
                "lib/mcp_elixir_demo/router.ex",
                "run.exs",
                "test/mcp_elixir_demo_test.exs",
                "test/test_helper.exs",
            ],
        ),
    ];

    for (language, name, expected_paths) in cases {
        let result = client
            .call_tool(
                CallToolRequestParams::new("init_project").with_arguments(
                    json!({
                        "name": name,
                        "language": language,
                        "directory": tmp.path().display().to_string()
                    })
                    .as_object()
                    .expect("object")
                    .clone(),
                ),
            )
            .await?;

        let text = result
            .content
            .first()
            .and_then(|content| content.raw.as_text())
            .map(|content| content.text.as_str())
            .expect("expected text tool result");
        let project_dir = tmp.path().join(name);

        assert!(project_dir.exists(), "expected {language} project root");
        assert!(text.contains("\"files_created\""), "expected {language} result payload");
        assert!(
            text.contains("\"next_steps\""),
            "expected {language} next_steps payload"
        );

        for expected in expected_paths {
            assert!(
                project_dir.join(expected).exists(),
                "expected {language} to create {expected}"
            );
        }
    }

    shutdown_client(&mut client, &mut child).await?;
    Ok(())
}
