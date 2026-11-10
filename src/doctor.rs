//! Local diagnostics: inspect PATH without executing providers or sending data.
use std::env;
use std::net::{SocketAddr, TcpStream};
use std::path::Path;
use std::time::Duration;

pub struct Diagnostics {
    git: bool,
    codex: bool,
    claude: bool,
    opencode: bool,
    ollama: bool,
}

fn executable_exists(name: &str) -> bool {
    let Some(path) = env::var_os("PATH") else {
        return false;
    };
    let suffixes: &[&str] = if cfg!(windows) {
        &[".exe", ".cmd", ".bat", ""]
    } else {
        &[""]
    };
    env::split_paths(&path).any(|directory| {
        suffixes.iter().any(|suffix| {
            let candidate = directory.join(format!("{name}{suffix}"));
            Path::new(&candidate).is_file()
        })
    })
}

pub fn check() -> Diagnostics {
    let port = env::var("PREAGENT_OLLAMA_PORT")
        .ok()
        .and_then(|value| value.parse::<u16>().ok())
        .filter(|port| *port > 0)
        .unwrap_or(11434);
    Diagnostics {
        git: executable_exists("git"),
        codex: executable_exists("codex"),
        claude: executable_exists("claude"),
        opencode: executable_exists("opencode"),
        ollama: TcpStream::connect_timeout(
            &SocketAddr::from(([127, 0, 0, 1], port)),
            Duration::from_millis(250),
        ).is_ok(),
    }
}

impl Diagnostics {
    pub fn to_text(&self) -> String {
        let state = |found| if found { "detected" } else { "not detected" };
        format!(
            "Preagent local diagnostics (no cloud calls)\nGit: {}\nCodex: {}\nClaude Code: {}\nOpenCode: {}\nOllama loopback: {}\n\
             CLI detection checks PATH only; no credentials or model access were tested.",
            state(self.git),
            state(self.codex),
            state(self.claude),
            state(self.opencode),
            state(self.ollama)
        )
    }

    pub fn to_json(&self) -> String {
        format!(
            "{{\"git\":{},\"codex\":{},\"claude\":{},\"opencode\":{},\"ollama_loopback\":{}}}",
            self.git, self.codex, self.claude, self.opencode, self.ollama
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diagnostics_never_require_an_installed_agent() {
        let result = check();
        assert!(result.to_json().contains("\"ollama_loopback\":"));
        assert!(result.to_text().contains("no cloud calls"));
    }
}
