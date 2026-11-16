use crate::cli::{Agent, Options};
use std::process::{Command, ExitStatus};

#[cfg(unix)]
use std::os::unix::process::ExitStatusExt;

#[derive(Debug, PartialEq, Eq)]
pub struct Invocation {
    pub executable: &'static str,
    pub arguments: Vec<String>,
}

pub fn invocation(options: &Options) -> Invocation {
    match options.agent {
        Agent::Codex => codex_invocation(options),
        Agent::Claude => claude_invocation(options),
        Agent::OpenCode => opencode_invocation(options),
    }
}

pub fn claude_invocation(options: &Options) -> Invocation {
    let mut arguments = Vec::new();
    if let Some(model) = &options.model {
        arguments.extend(["--model".to_string(), model.clone()]);
    }
    if let Some(effort) = &options.effort {
        arguments.extend(["--effort".to_string(), effort.clone()]);
    }
    arguments.push("--".to_string());
    arguments.push(options.prompt.clone());
    Invocation {
        executable: "claude",
        arguments,
    }
}

// OpenCode mini is an officially documented interactive interface that
// accepts an initial prompt. Native OpenCode remains responsible for model
// availability, permissions, sessions and sandboxing.
pub fn opencode_invocation(options: &Options) -> Invocation {
    let mut arguments = vec!["mini".to_string()];
    if let Some(model) = &options.model {
        arguments.extend(["--model".to_string(), model.clone()]);
    }
    arguments.extend(["--prompt".to_string(), options.prompt.clone()]);
    Invocation {
        executable: "opencode",
        arguments,
    }
}

pub fn codex_invocation(options: &Options) -> Invocation {
    let mut arguments = Vec::new();
    if options.oss {
        arguments.push("--oss".to_string());
    }
    if let Some(provider) = &options.local_provider {
        arguments.push("--local-provider".to_string());
        arguments.push(provider.clone());
    }
    if let Some(model) = &options.model {
        arguments.push("--model".to_string());
        arguments.push(model.clone());
    }
    if let Some(effort) = &options.effort {
        arguments.push("-c".to_string());
        arguments.push(format!("model_reasoning_effort=\"{effort}\""));
    }

    // Codex receives the original prompt as a single argument. Never
    // interpolate user content into a shell command or add instructions.
    arguments.push("--".to_string());
    arguments.push(options.prompt.clone());
    Invocation {
        executable: "codex",
        arguments,
    }
}

// A signalled Unix child has no numeric status. Preserve the usual
// 128+signal convention so callers can distinguish Ctrl+C from exit 1.
fn agent_exit_code(status: ExitStatus) -> i32 {
    #[cfg(unix)]
    if let Some(signal) = status.signal() {
        return 128 + signal;
    }
    status.code().unwrap_or(1)
}

impl Invocation {
    pub fn display(&self) -> String {
        let mut parts = vec![self.executable.to_string()];
        for argument in &self.arguments {
            parts.push(format!("{argument:?}"));
        }
        parts.join(" ")
    }

    pub fn execute(&self) -> Result<i32, String> {
        // Resolve the native executable explicitly on Windows. Rust may invoke
        // .bat/.cmd files through cmd.exe; developer prompts must never reach
        // a batch-file trampoline.
        #[cfg(windows)]
        let executable = crate::doctor::native_executable(self.executable).ok_or_else(|| {
            format!(
                "failed to start {}: native .exe not found on PATH (.cmd/.bat shims are not launched)",
                self.executable
            )
        })?;
        #[cfg(not(windows))]
        let executable = self.executable;

        let status = Command::new(executable)
            .args(&self.arguments)
            .status()
            .map_err(|error| {
                let hint = if cfg!(windows) {
                    "; put a native .exe on PATH (.cmd/.bat shims are not launched)"
                } else {
                    ""
                };
                format!("failed to start {}: {error}{hint}", self.executable)
            })?;
        Ok(agent_exit_code(status))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(unix)]
    #[test]
    fn signalled_agent_exit_uses_shell_convention() {
        assert_eq!(agent_exit_code(ExitStatus::from_raw(2)), 130);
        assert_eq!(agent_exit_code(ExitStatus::from_raw(15)), 143);
        assert_eq!(agent_exit_code(ExitStatus::from_raw(7 << 8)), 7);
    }

    #[test]
    fn preserves_prompt_as_single_argument() {
        let options = Options {
            prompt: "fix \"login\" && echo unsafe".to_string(),
            ..Options::default()
        };
        let invocation = codex_invocation(&options);
        assert_eq!(invocation.arguments.len(), 2);
        assert_eq!(invocation.arguments[0], "--");
        assert_eq!(invocation.arguments[1], options.prompt);
    }

    #[test]
    fn other_agents_preserve_prompt_as_one_argument() {
        let prompt = "Fix bug ; echo not-executed".to_string();
        let mut opts = Options {
            prompt: prompt.clone(),
            model: Some("known-model".into()),
            ..Options::default()
        };
        opts.agent = Agent::Claude;
        let claude = invocation(&opts);
        assert_eq!(claude.executable, "claude");
        assert_eq!(claude.arguments.last(), Some(&prompt));
        opts.agent = Agent::OpenCode;
        let opencode = invocation(&opts);
        assert_eq!(opencode.executable, "opencode");
        assert_eq!(
            opencode.arguments,
            vec![
                "mini",
                "--model",
                "known-model",
                "--prompt",
                prompt.as_str()
            ]
        );
    }

    #[test]
    fn only_user_overrides_are_forwarded() {
        let options = Options {
            prompt: "Fix bug".into(),
            oss: true,
            local_provider: Some("ollama".into()),
            model: Some("my-local-model".into()),
            effort: Some("low".into()),
            ..Options::default()
        };
        let invocation = codex_invocation(&options);
        assert_eq!(
            invocation.arguments,
            vec![
                "--oss",
                "--local-provider",
                "ollama",
                "--model",
                "my-local-model",
                "-c",
                "model_reasoning_effort=\"low\"",
                "--",
                "Fix bug"
            ]
        );
    }
}
