use crate::cli::{Agent, Options};
use std::process::Command;

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

impl Invocation {
    pub fn display(&self) -> String {
        let mut parts = vec![self.executable.to_string()];
        for argument in &self.arguments {
            parts.push(format!("{argument:?}"));
        }
        parts.join(" ")
    }

    pub fn execute(&self) -> Result<i32, String> {
        let status = Command::new(self.executable)
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
        Ok(status.code().unwrap_or(1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
