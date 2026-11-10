use std::process::Command;

fn preagent(arguments: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_preagent"))
        .args(arguments)
        .output()
        .expect("preagent binary should run")
}

#[test]
fn route_json_is_machine_readable_and_uses_no_model_override() {
    let output = preagent(&["route", "--no-git", "--json", "Fix authentication bug"]);
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.starts_with('{'));
    assert!(stdout.contains("\"tier\":\"complex\""));
    assert!(stdout.contains("\"model_override\":null"));
}

#[test]
fn run_dry_run_does_not_need_codex_installed() {
    let output = preagent(&[
        "run",
        "--dry-run",
        "--no-git",
        "--oss",
        "--local-provider",
        "ollama",
        "Fix typo in README",
    ]);
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("No agent was started"));
    assert!(stdout.contains("--oss"));
}

#[test]
fn rejects_invalid_flags() {
    let output = preagent(&["route", "--bad-option", "prompt"]);
    assert_eq!(output.status.code(), Some(2));
}
