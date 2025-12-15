#[test]
fn spikard_help_succeeds() {
    let exe = env!("CARGO_BIN_EXE_spikard");
    let output = std::process::Command::new(exe)
        .arg("--help")
        .output()
        .expect("run failed");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("spikard"));
}

#[test]
fn spikard_generate_help_succeeds() {
    let exe = env!("CARGO_BIN_EXE_spikard");
    let output = std::process::Command::new(exe)
        .args(["generate", "--help"])
        .output()
        .expect("run failed");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("generate"));
}
