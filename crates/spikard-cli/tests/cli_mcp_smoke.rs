use assert_cmd::Command;
use predicates::prelude::*;
use rmcp::{
    ClientHandler, ServiceExt,
    model::{CallToolRequestParams, ClientInfo},
};
use serde_json::json;
use std::process::Stdio;
use tempfile::TempDir;
use tokio::{process::Command as TokioCommand, time::{Duration, timeout}};

#[derive(Debug, Clone, Default)]
struct DummyClientHandler;

impl ClientHandler for DummyClientHandler {
    fn get_info(&self) -> ClientInfo {
        ClientInfo::default()
    }
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

    let tools = client.list_all_tools().await?;
    let tool_names = tools.iter().map(|tool| tool.name.as_ref()).collect::<Vec<_>>();
    assert!(tool_names.contains(&"get_features"));
    assert!(tool_names.contains(&"generate_openapi"));
    assert!(tool_names.contains(&"generate_asyncapi_bundle"));

    let result = client
        .call_tool(
            CallToolRequestParams::new("get_features")
                .with_arguments(json!({}).as_object().expect("object").clone()),
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

    client.cancel().await?;
    let _ = timeout(Duration::from_secs(5), child.wait()).await?;
    Ok(())
}

#[tokio::test]
async fn spikard_mcp_stdio_can_initialize_a_project() -> anyhow::Result<()> {
    let tmp = TempDir::new()?;
    let project_name = "mcp_init_demo";

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

    client.cancel().await?;
    let _ = timeout(Duration::from_secs(5), child.wait()).await?;
    Ok(())
}
