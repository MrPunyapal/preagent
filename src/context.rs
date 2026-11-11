use std::process::Command;

#[derive(Debug, Clone, Default)]
pub struct RepoContext {
    pub is_repo: bool,
    // Only tracked changed-file names. Never read code or file contents.
    pub changed_paths: Vec<String>,
}

pub fn discover(enabled: bool) -> RepoContext {
    if !enabled {
        return RepoContext::default();
    }
    let Some(result) = git(&["rev-parse", "--is-inside-work-tree"]) else {
        return RepoContext::default();
    };
    if result.trim() != "true" {
        return RepoContext::default();
    }
    let changed_paths = parse_status(
        &git(&["status", "--porcelain=v1", "-z", "--untracked-files=no"]).unwrap_or_default(),
    );

    RepoContext {
        is_repo: true,
        changed_paths,
    }
}

fn parse_status(status: &str) -> Vec<String> {
    // With -z Git emits unquoted paths and puts a rename's destination first.
    let mut records = status.split('\0').filter(|record| !record.is_empty());
    let mut paths = Vec::new();
    while let Some(record) = records.next() {
        let Some(path) = record.get(3..) else {
            continue;
        };
        paths.push(path.to_string());
        if record.as_bytes()[..2]
            .iter()
            .any(|c| *c == b'R' || *c == b'C')
        {
            records.next();
        }
        if paths.len() == 100 {
            break;
        }
    }
    paths
}

fn git(arguments: &[&str]) -> Option<String> {
    let output = Command::new("git")
        .args(arguments)
        .env("GIT_OPTIONAL_LOCKS", "0")
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    String::from_utf8(output.stdout).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn disabled_repo_context_is_empty() {
        let context = discover(false);
        assert!(!context.is_repo);
        assert!(context.changed_paths.is_empty());
    }

    #[test]
    fn nul_status_preserves_unicode_quotes_newlines_and_rename_destination() {
        assert_eq!(
            parse_status(" M auth/登录.rs\0R  security/new -> name.rs\0old.rs\0 M a\"b\nc.rs\0"),
            vec!["auth/登录.rs", "security/new -> name.rs", "a\"b\nc.rs"]
        );
        assert_eq!(parse_status(&" M a.rs\0".repeat(200)).len(), 100);
    }
}
