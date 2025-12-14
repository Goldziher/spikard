use std::process::Command;

#[test]
fn cli_help_works() {
    let exe = env!("CARGO_BIN_EXE_spikard");
    let output = Command::new(exe).arg("--help").output().expect("run failed");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Spikard"));
}

#[test]
fn cli_features_works() {
    let exe = env!("CARGO_BIN_EXE_spikard");
    let output = Command::new(exe).arg("features").output().expect("run failed");
    assert!(output.status.success());
}
