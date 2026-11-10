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
    let changed_paths = git(&["status", "--porcelain=v1", "--untracked-files=no"])
        .unwrap_or_default()
        .lines()
        .take(100)
        .filter_map(|line| line.get(3..))
        .map(|path| path.rsplit_once(" -> ").map_or(path, |(_, to)| to).to_string())
        .collect();

    RepoContext {
        is_repo: true,
        changed_paths,
    }
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
}
