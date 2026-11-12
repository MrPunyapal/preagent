use std::io::Write;
use std::process::Command;
use std::process::Stdio;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static SCRATCH_ID: AtomicU64 = AtomicU64::new(0);

struct Scratch(std::path::PathBuf);

impl Scratch {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "preagent-test-{}-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
            SCRATCH_ID.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&path).unwrap();
        Self(path)
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

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

#[test]
fn native_agent_processes_preserve_arguments_cwd_stdio_and_exit_codes() {
    let scratch = Scratch::new();
    let stub = scratch
        .0
        .join(format!("codex{}", std::env::consts::EXE_SUFFIX));
    assert!(Command::new("rustc")
        .arg(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/support/agent_stub.rs"
        ))
        .arg("-o")
        .arg(&stub)
        .status()
        .unwrap()
        .success());
    for agent in ["claude", "opencode"] {
        std::fs::copy(
            &stub,
            scratch
                .0
                .join(format!("{agent}{}", std::env::consts::EXE_SUFFIX)),
        )
        .unwrap();
    }
    let mut paths = vec![scratch.0.clone()];
    paths.extend(std::env::split_paths(
        &std::env::var_os("PATH").unwrap_or_default(),
    ));
    let path = std::env::join_paths(paths).unwrap();
    let prompts = [
        "  Fix \"登录\" 🦀\r\nsecond line & echo unsafe | %PATH% !x! $(whoami) `x` \\",
        "--help",
        r#"fix the \"quoted\" path C:\folder with spaces\"#,
    ];
    for agent in ["codex", "claude", "opencode"] {
        for prompt in prompts {
            let mut command = Command::new(env!("CARGO_BIN_EXE_preagent"));
            command.args(["run", "--no-git", "--agent", agent, "--model", "test-model"]);
            if agent != "opencode" {
                command.args(["--effort", "high"]);
            }
            let mut child = command
                .args(["--", prompt])
                .env("PATH", &path)
                .env("PREAGENT_TEST_EXIT", "37")
                .current_dir(&scratch.0)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .unwrap();
            child
                .stdin
                .take()
                .unwrap()
                .write_all(b"hello stdin\n")
                .unwrap();
            let output = child.wait_with_output().unwrap();
            assert_eq!(output.status.code(), Some(37));
            let stdout = String::from_utf8(output.stdout).unwrap();
            let mut arguments = vec!["--model".to_string(), "test-model".to_string()];
            if agent == "opencode" {
                arguments.insert(0, "mini".into());
                arguments.push("--prompt".into());
            } else {
                arguments.extend(if agent == "claude" {
                    vec!["--effort".into(), "high".into()]
                } else {
                    vec!["-c".into(), "model_reasoning_effort=\"high\"".into()]
                });
                arguments.push("--".into());
            }
            arguments.push(prompt.into());
            let received: Vec<_> = stdout
                .lines()
                .filter(|line| line.starts_with("ARG "))
                .collect();
            let expected: Vec<_> = arguments.iter().map(|arg| format!("ARG {arg:?}")).collect();
            assert_eq!(received, expected, "agent {agent}");
            assert!(stdout.contains(&format!("CWD {:?}", scratch.0.canonicalize().unwrap())));
            assert!(stdout.contains("STDIN \"hello stdin\\n\""));
            assert!(String::from_utf8(output.stderr)
                .unwrap()
                .contains("STUB STDERR"));
        }
    }
    #[cfg(windows)]
    {
        let output = Command::new(env!("CARGO_BIN_EXE_preagent"))
            .args(["run", "--no-git", "test"])
            .env("PATH", &path)
            .env("PREAGENT_TEST_EXIT", "3010")
            .stdin(Stdio::null())
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(3010));
    }
}

#[test]
fn git_metadata_handles_root_security_unicode_and_renames() {
    let scratch = Scratch::new();
    let git = |args: &[&str]| {
        let output = Command::new("git")
            .args(args)
            .current_dir(&scratch.0)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
    };
    git(&["init", "--quiet"]);
    std::fs::create_dir(scratch.0.join("auth")).unwrap();
    std::fs::write(scratch.0.join("auth/登录.rs"), "initial").unwrap();
    std::fs::write(scratch.0.join("README.md"), "initial").unwrap();
    git(&["add", "."]);
    git(&[
        "-c",
        "user.name=Preagent Test",
        "-c",
        "user.email=test@example.invalid",
        "-c",
        "commit.gpgsign=false",
        "-c",
        "core.hooksPath=",
        "commit",
        "--quiet",
        "-m",
        "fixture",
    ]);
    git(&["mv", "auth/登录.rs", "auth/重命名.rs"]);
    std::fs::write(scratch.0.join("README.md"), "modified").unwrap();
    std::fs::write(scratch.0.join("untracked.txt"), "excluded").unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_preagent"))
        .args(["route", "--json", "Review these changes"])
        .current_dir(&scratch.0)
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("\"is_repo\":true"));
    assert!(stdout.contains("\"changed_files\":2"));
    assert!(stdout.contains("\"tier\":\"complex\""));
    let output = Command::new(env!("CARGO_BIN_EXE_preagent"))
        .args(["route", "--json", "Fix README typo"])
        .current_dir(&scratch.0)
        .output()
        .unwrap();
    assert!(String::from_utf8(output.stdout)
        .unwrap()
        .contains("\"tier\":\"simple\""));
}

#[test]
fn absent_agents_and_invalid_slm_fail_usefully_without_network() {
    let scratch = Scratch::new();
    // A Windows npm shell shim must neither be executed nor reported as native.
    #[cfg(windows)]
    {
        std::fs::write(
            scratch.0.join("claude.cmd"),
            "@echo unexpected execution\r\n",
        )
        .unwrap();
        let output = Command::new(env!("CARGO_BIN_EXE_preagent"))
            .args(["doctor", "--json"])
            .env("PATH", &scratch.0)
            .current_dir(&scratch.0)
            .output()
            .unwrap();
        assert!(String::from_utf8(output.stdout)
            .unwrap()
            .contains("\"claude\":false"));
    }
    let output = Command::new(env!("CARGO_BIN_EXE_preagent"))
        .args(["run", "--no-git", "--agent", "claude", "test"])
        .env("PATH", &scratch.0)
        .current_dir(&scratch.0)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    assert!(String::from_utf8(output.stderr)
        .unwrap()
        .contains("failed to start claude"));
    let output = preagent(&[
        "route",
        "--no-git",
        "--json",
        "--slm",
        "bad?model",
        "--slm-always",
        "Fix authentication bug",
    ]);
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("\"slm_status\":\"unavailable\""));
    assert!(stdout.contains("\"slm_error\":\"invalid local model identifier\""));
    assert!(stdout.contains("\"recommended_tier\":\"complex\""));
}


#[test]
fn heldout_pilot_is_separate_from_smoke_and_has_no_agent_outcome() {
    let listing = preagent(&["heldout", "list", "--json"]);
    assert!(listing.status.success());
    let list = String::from_utf8(listing.stdout).unwrap();
    assert!(list.contains("\"suite\":\"heldout-v1\""));
    assert!(list.contains("itoa-decimal-pairs"));

    let output = preagent(&["heldout", "route", "--json"]);
    assert!(output.status.success());
    let route = String::from_utf8(output.stdout).unwrap();
    assert!(route.contains("\"task_correctness\":null"));
    assert!(route.contains("\"slm_model\":null"));
    assert!(route.contains("\"slm_status\":\"not_requested\""));

    let unknown = preagent(&["heldout", "prepare", "not-a-task", ".", "not-created"]);
    assert!(!unknown.status.success());
}
