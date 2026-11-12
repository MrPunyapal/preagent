//! Local pinned fixtures and acceptance grading, never remote clones or paid agents.
use super::{fixtures, Case};
use crate::slm;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

const MAX_SNAPSHOT_BYTES: u64 = 256 * 1024 * 1024;
const MAX_SNAPSHOT_FILES: usize = 10_000;
const GRADE_TIMEOUT: Duration = Duration::from_secs(180);

fn git(dir: &Path, args: &[&str]) -> Result<String, String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(args)
        .output()
        .map_err(|e| format!("git is not available: {e}"))?;
    if !output.status.success() {
        return Err(format!(
            "git {} failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn verify_commit(case: Case, source: &Path) -> Result<(), String> {
    if !source.is_dir() {
        return Err("a local, already cloned repository directory is required".into());
    }
    let actual = git(source, &["rev-parse", "HEAD"])?;
    if actual != case.sha {
        return Err(format!(
            "task {} requires pinned {}, but local HEAD is {}",
            case.id, case.sha, actual
        ));
    }
    Ok(())
}

fn apply_patch(destination: &Path, patch: &str, check: bool) -> Result<(), String> {
    let mut child = Command::new("git")
        .arg("-C")
        .arg(destination)
        .arg("apply")
        .args(if check {
            vec!["--check", "-"]
        } else {
            vec!["-"]
        })
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("cannot start git apply: {e}"))?;
    child
        .stdin
        .take()
        .ok_or("cannot pipe patch to git apply")?
        .write_all(patch.as_bytes())
        .map_err(|e| format!("cannot feed patch: {e}"))?;
    let result = child
        .wait_with_output()
        .map_err(|e| format!("cannot finish git apply: {e}"))?;
    if !result.status.success() {
        return Err(format!(
            "pinned fixture patch failed to {}: {}",
            if check { "validate" } else { "apply" },
            String::from_utf8_lossy(&result.stderr)
        ));
    }
    Ok(())
}

// Use "." from the source directory: on Windows an absolute "D:\\..." path
// passed as the clone source may be interpreted as an scp-style remote.
// Keep the destination absolute so relative caller paths are not re-based.
fn clone_local_repo(source: &Path, output: &Path) -> Result<std::process::Output, String> {
    let destination = if output.is_absolute() {
        output.to_path_buf()
    } else {
        std::env::current_dir()
            .map_err(|e| format!("cannot resolve destination directory: {e}"))?
            .join(output)
    };
    Command::new("git")
        .args(["clone", "--quiet", "--no-hardlinks", "--"])
        .arg(".")
        .arg(&destination)
        .current_dir(source)
        .output()
        .map_err(|e| format!("git clone from local source failed: {e}"))
}

pub fn prepare(case: Case, source: &Path, output: &Path) -> Result<String, String> {
    verify_commit(case, source)?;
    if fs::symlink_metadata(output).is_ok() {
        return Err("destination exists; never overwrite an existing directory".into());
    }
    if !git(source, &["status", "--porcelain", "--untracked-files=no"])?.is_empty() {
        return Err(
            "source contains tracked working-tree changes; use a clean pinned checkout".into(),
        );
    }
    let source = fs::canonicalize(source)
        .map_err(|e| format!("cannot resolve local source directory: {e}"))?;
    let copied = clone_local_repo(&source, output)?;
    if !copied.status.success() {
        return Err(format!(
            "git clone from local source failed: {}",
            String::from_utf8_lossy(&copied.stderr)
        ));
    }
    let checkout = Command::new("git")
        .arg("-C")
        .arg(output)
        .args(["checkout", "--quiet", "--detach", case.sha])
        .output()
        .map_err(|e| format!("cannot checkout pinned commit: {e}"))?;
    if !checkout.status.success() {
        return Err("local clone is missing the required pinned commit".into());
    }
    let patch = fixtures(case.id).ok_or("fixture patch is unavailable")?.0;
    apply_patch(output, patch, true)?;
    apply_patch(output, patch, false)?;
    Ok(format!(
        "Prepared task {} from pinned {} in {}\nSeed applied. Run heldout grade before and after an agent repair; no acceptance tests were shown to the agent.",
        case.id, case.sha, output.display()
    ))
}

struct Scratch(std::path::PathBuf);

impl Scratch {
    fn new() -> Result<Self, String> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| format!("invalid system clock: {e}"))?
            .as_nanos();
        let path =
            std::env::temp_dir().join(format!("preagent-heldout-{}-{}", std::process::id(), now));
        fs::create_dir(&path).map_err(|e| format!("cannot create grading scratch: {e}"))?;
        Ok(Self(path))
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

struct SnapshotLimits {
    files: usize,
    bytes: u64,
}

fn copy_snapshot(source: &Path, dest: &Path, limits: &mut SnapshotLimits) -> Result<(), String> {
    for item in fs::read_dir(source).map_err(|e| format!("cannot list candidate files: {e}"))? {
        let item = item.map_err(|e| format!("cannot inspect candidate file: {e}"))?;
        let name = item.file_name();
        if [".git", "target", "node_modules", ".venv", "__pycache__"]
            .iter()
            .any(|excluded| name == *excluded)
        {
            continue;
        }
        let original = item.path();
        let metadata = fs::symlink_metadata(&original)
            .map_err(|e| format!("cannot stat candidate entry: {e}"))?;
        if metadata.file_type().is_symlink() {
            return Err(format!(
                "candidate contains symlink at {}; refusing to follow it",
                original.display()
            ));
        }
        let destination = dest.join(&name);
        if metadata.is_dir() {
            fs::create_dir(&destination)
                .map_err(|e| format!("cannot create grading directory: {e}"))?;
            copy_snapshot(&original, &destination, limits)?;
        } else if metadata.is_file() {
            limits.files += 1;
            limits.bytes = limits.bytes.saturating_add(metadata.len());
            if limits.files > MAX_SNAPSHOT_FILES || limits.bytes > MAX_SNAPSHOT_BYTES {
                return Err("candidate snapshot exceeds file count/size limits".into());
            }
            fs::copy(&original, &destination)
                .map_err(|e| format!("cannot copy candidate file: {e}"))?;
        } else {
            return Err(format!(
                "unsupported candidate entry {}",
                original.display()
            ));
        }
    }
    Ok(())
}

fn cargo_acceptance(dir: &Path) -> Result<(String, i32, u128, String), String> {
    let stdout = fs::File::create(dir.join(".preagent-grade-stdout"))
        .map_err(|e| format!("cannot create grading output: {e}"))?;
    let stderr = fs::File::create(dir.join(".preagent-grade-stderr"))
        .map_err(|e| format!("cannot create grading errors: {e}"))?;
    let started = Instant::now();
    let mut child = Command::new("cargo")
        .args(["test", "--offline", "--test", "preagent_holdout"])
        .current_dir(dir)
        .env("CARGO_NET_OFFLINE", "true")
        .stdin(Stdio::null())
        .stdout(Stdio::from(stdout))
        .stderr(Stdio::from(stderr))
        .spawn()
        .map_err(|e| format!("cargo unavailable for local acceptance grading: {e}"))?;
    let code = loop {
        match child
            .try_wait()
            .map_err(|e| format!("cannot poll cargo: {e}"))?
        {
            Some(status) => break status.code().unwrap_or(1),
            None if started.elapsed() > GRADE_TIMEOUT => {
                let _ = child.kill();
                let _ = child.wait();
                return Err("local acceptance test exceeded 180-second timeout".into());
            }
            None => thread::sleep(Duration::from_millis(100)),
        }
    };
    let output = fs::read_to_string(dir.join(".preagent-grade-stdout"))
        .map_err(|e| format!("cannot read grading output: {e}"))?;
    let errors = fs::read_to_string(dir.join(".preagent-grade-stderr"))
        .map_err(|e| format!("cannot read grading errors: {e}"))?;
    let status = if code == 0 && output.contains("test result: ok.") {
        "acceptance_passed"
    } else if output.contains("test result: FAILED.") {
        "acceptance_failed"
    } else {
        "infrastructure_or_build_failure"
    };
    let detail = if status == "infrastructure_or_build_failure" {
        errors.chars().take(2000).collect()
    } else {
        String::new()
    };
    Ok((status.into(), code, started.elapsed().as_millis(), detail))
}

pub fn grade(case: Case, candidate: &Path) -> Result<String, String> {
    verify_commit(case, candidate)?;
    let scratch = Scratch::new()?;
    let mut limits = SnapshotLimits { files: 0, bytes: 0 };
    copy_snapshot(candidate, &scratch.0, &mut limits)?;
    let test_path = scratch.0.join("tests").join("preagent_holdout.rs");
    if test_path.exists() {
        return Err(
            "candidate already has tests/preagent_holdout.rs; refusing to overwrite".into(),
        );
    }
    fs::create_dir_all(test_path.parent().ok_or("invalid test path")?)
        .map_err(|e| format!("cannot create grading tests directory: {e}"))?;
    let acceptance = fixtures(case.id).ok_or("acceptance test missing")?.1;
    fs::write(&test_path, acceptance)
        .map_err(|e| format!("cannot inject acceptance tests: {e}"))?;
    let (status, exit_code, elapsed_ms, detail) = cargo_acceptance(&scratch.0)?;
    Ok(format!(
        "{{\"task_id\":{},\"pinned_commit\":{},\"acceptance_status\":{},\"test_exit_code\":{},\"test_elapsed_ms\":{},\"copied_files\":{},\"copied_bytes\":{},\"infrastructure_detail\":{},\"task_correctness\":null}}",
        slm::json_quote(case.id),
        slm::json_quote(case.sha),
        slm::json_quote(&status),
        exit_code,
        elapsed_ms,
        limits.files,
        limits.bytes,
        slm::json_quote(&detail)
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prep_never_overwrites_existing_work() {
        let case = super::super::cases().unwrap()[0];
        let error = prepare(case, Path::new("."), Path::new(".")).unwrap_err();
        assert!(error.contains("pinned") || error.contains("repository"));
    }

    #[test]
    fn cloning_an_absolute_local_checkout_does_not_look_like_a_remote() {
        let scratch = Scratch::new().unwrap();
        let source = scratch.0.join("local-source");
        let destination = scratch.0.join("cloned-workspace");
        fs::create_dir(&source).unwrap();
        assert!(Command::new("git")
            .args(["init", "--quiet"])
            .arg(&source)
            .status()
            .unwrap()
            .success());
        let result = clone_local_repo(&fs::canonicalize(&source).unwrap(), &destination).unwrap();
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert!(destination.join(".git").exists());
    }

    #[test]
    fn scratch_is_unique_and_cleaned_up() {
        let scratch = Scratch::new().unwrap();
        let path = scratch.0.clone();
        assert!(path.is_dir());
        drop(scratch);
        assert!(!path.exists());
    }
}
