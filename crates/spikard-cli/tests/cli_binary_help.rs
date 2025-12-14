use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn spikard_binary_help_exits_successfully() {
    let mut command = Command::new(assert_cmd::cargo::cargo_bin!("spikard"));
    command
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("spikard"));
}
