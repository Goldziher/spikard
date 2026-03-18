use assert_cmd::Command;
use predicates::prelude::*;

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
