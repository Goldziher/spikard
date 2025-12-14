use assert_cmd::cargo::cargo_bin_cmd;
use predicates::str::contains;

#[test]
fn cli_help_exits_successfully() {
    cargo_bin_cmd!("spikard")
        .arg("--help")
        .assert()
        .success()
        .stdout(contains("Spikard"));
}
