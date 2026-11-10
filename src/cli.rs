#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Agent {
    #[default]
    Codex,
    Claude,
    OpenCode,
}

impl Agent {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Codex => "codex",
            Self::Claude => "claude",
            Self::OpenCode => "opencode",
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Options {
    pub prompt: String,
    pub agent: Agent,
    pub no_git: bool,
    pub json: bool,
    pub dry_run: bool,
    pub oss: bool,
    pub local_provider: Option<String>,
    pub model: Option<String>,
    pub slm: Option<String>,
    pub slm_always: bool,
    pub effort: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Action {
    Help,
    Version,
    Route(Options),
    Run(Options),
    Eval { json: bool },
}

pub const HELP: &str = "preagent - offline-first pre-agent for coding assistants

USAGE
    preagent route [--json] [--no-git] [--slm MODEL] [--slm-always] <prompt>
    preagent run [--agent codex|claude|opencode] [--dry-run] [--oss]
                 [--local-provider ollama|lmstudio] [--model NAME]
                 [--effort LEVEL] [--no-git] <prompt>
    preagent eval [--json]
    preagent --help
    preagent --version

COMMANDS
    route    Classify a prompt locally without calling an AI service.
    run      Start the selected coding agent with the ORIGINAL prompt.
    eval     Score the built-in, labeled routing smoke corpus (no AI calls).

OPTIONS
    --json             Machine-readable route or evaluation output.
    --slm MODEL        Use installed Ollama model via loopback (route only).
    --slm-always       Consult SLM even on unambiguous prompts.
    --no-git           Do not inspect local Git metadata.
    --dry-run          Preview native agent arguments without launching it.
    --agent NAME       Choose codex (default), claude or opencode.
    --oss              Request Codex's local open-model provider mode.
    --local-provider   Explicit provider: ollama or lmstudio (requires --oss).
    --model            User-selected Codex model (never guessed).
    --effort           User-selected Codex effort (not automatically applied).
    --                 End options; useful for prompts starting with hyphens.

Routing is advisory, not calibrated. Optional SLM inference needs a locally
running Ollama and an installed model; no model is downloaded automatically.
Automatic model choice and prompt improvements are future milestones.
A Codex cloud model may incur provider charges. Use --oss with a verified
local model and runtime for a fully local execution workflow.";

pub fn parse(args: impl IntoIterator<Item = String>) -> Result<Action, String> {
    let mut args = args.into_iter();
    let Some(command) = args.next() else {
        return Ok(Action::Help);
    };
    match command.as_str() {
        "-h" | "--help" | "help" => return Ok(Action::Help),
        "-V" | "--version" | "version" => return Ok(Action::Version),
        "route" | "run" | "eval" => {}
        _ => return Err(format!("unknown command '{command}'")),
    }

    if command == "eval" {
        let mut json = false;
        for option in args {
            if option == "--json" {
                json = true;
            } else {
                return Err(format!("unknown option '{option}' for eval"));
            }
        }
        return Ok(Action::Eval { json });
    }

    let is_run = command == "run";
    let mut options = Options::default();
    let mut prompt_words = Vec::new();
    while let Some(argument) = args.next() {
        match argument.as_str() {
            "--" => {
                prompt_words.extend(args);
                break;
            }
            "--no-git" => options.no_git = true,
            "--slm" if !is_run => options.slm = Some(next_value(&mut args, "--slm")?),
            "--slm-always" if !is_run => options.slm_always = true,
            "--json" if !is_run => options.json = true,
            "--dry-run" if is_run => options.dry_run = true,
            "--agent" if is_run => {
                options.agent = match next_value(&mut args, "--agent")?.as_str() {
                    "codex" => Agent::Codex,
                    "claude" => Agent::Claude,
                    "opencode" => Agent::OpenCode,
                    _ => return Err("--agent must be codex, claude or opencode".to_string()),
                };
            }
            "--oss" if is_run => options.oss = true,
            "--model" if is_run => options.model = Some(next_value(&mut args, "--model")?),
            "--effort" if is_run => options.effort = Some(next_value(&mut args, "--effort")?),
            "--local-provider" if is_run => {
                options.local_provider = Some(next_value(&mut args, "--local-provider")?);
            }
            _ if argument.starts_with('-') => {
                return Err(format!("unknown option '{argument}' for {command}"));
            }
            _ => prompt_words.push(argument),
        }
    }

    options.prompt = prompt_words.join(" ");
    if options.prompt.trim().is_empty() {
        return Err("a non-empty prompt is required".to_string());
    }
    if is_run {
        if options.agent != Agent::Codex && (options.oss || options.local_provider.is_some()) {
            return Err("--oss and --local-provider are Codex-only options".to_string());
        }
        if options.agent == Agent::OpenCode && options.effort.is_some() {
            return Err(
                "--effort is not portable to OpenCode; set a native model variant".to_string(),
            );
        }
        if let Some(provider) = &options.local_provider {
            if provider != "ollama" && provider != "lmstudio" {
                return Err("local provider must be 'ollama' or 'lmstudio'".to_string());
            }
            if !options.oss {
                return Err("--local-provider requires --oss".to_string());
            }
        }
        if let Some(effort) = &options.effort {
            if !["minimal", "low", "medium", "high", "xhigh", "max"].contains(&effort.as_str()) {
                return Err("unsupported effort label".to_string());
            }
            if options.agent == Agent::Claude
                && !["low", "medium", "high", "max"].contains(&effort.as_str())
            {
                return Err("Claude effort must be low, medium, high, or max".to_string());
            }
        }
        Ok(Action::Run(options))
    } else {
        if options.slm_always && options.slm.is_none() {
            return Err("--slm-always requires --slm MODEL".to_string());
        }
        Ok(Action::Route(options))
    }
}

fn next_value(args: &mut impl Iterator<Item = String>, option: &str) -> Result<String, String> {
    match args.next() {
        Some(value) if !value.is_empty() && !value.starts_with("--") => Ok(value),
        _ => Err(format!("missing value for {option}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_words(words: &[&str]) -> Result<Action, String> {
        parse(words.iter().map(|s| (*s).to_string()))
    }

    #[test]
    fn eval_has_no_side_effects_or_prompt() {
        assert_eq!(
            parse_words(&["eval"]).unwrap(),
            Action::Eval { json: false }
        );
        assert_eq!(
            parse_words(&["eval", "--json"]).unwrap(),
            Action::Eval { json: true }
        );
        assert!(parse_words(&["eval", "--model", "foo"]).is_err());
    }

    #[test]
    fn local_slm_requires_explicit_route_opt_in() {
        let Action::Route(options) =
            parse_words(&["route", "--slm", "qwen3.5:2b", "--slm-always", "fix"]).unwrap()
        else {
            panic!("expected route");
        };
        assert_eq!(options.slm.as_deref(), Some("qwen3.5:2b"));
        assert!(options.slm_always);
        assert!(parse_words(&["route", "--slm-always", "fix"]).is_err());
        assert!(parse_words(&["run", "--slm", "model", "fix"]).is_err());
    }

    #[test]
    fn agent_specific_options_are_validated() {
        assert!(parse_words(&["run", "--agent", "claude", "--oss", "fix"]).is_err());
        assert!(parse_words(&["run", "--agent", "opencode", "--effort", "high", "fix"]).is_err());
        assert!(parse_words(&["run", "--agent", "claude", "--effort", "xhigh", "fix"]).is_err());
        assert!(parse_words(&["run", "--agent", "other", "fix"]).is_err());
        let Action::Run(options) = parse_words(&["run", "--agent", "opencode", "Fix bug"]).unwrap()
        else {
            panic!("expected run");
        };
        assert_eq!(options.agent, Agent::OpenCode);
    }

    #[test]
    fn defaults_to_help() {
        assert_eq!(parse_words(&[]).unwrap(), Action::Help);
    }

    #[test]
    fn route_accepts_json_and_multiple_words() {
        let Action::Route(options) =
            parse_words(&["route", "--json", "fix", "the", "bug"]).unwrap()
        else {
            panic!("expected route");
        };
        assert!(options.json);
        assert_eq!(options.prompt, "fix the bug");
    }

    #[test]
    fn preserves_leading_hyphen_with_separator() {
        let Action::Route(options) = parse_words(&["route", "--", "--fix it"]).unwrap() else {
            panic!("expected route");
        };
        assert_eq!(options.prompt, "--fix it");
    }

    #[test]
    fn rejects_invalid_provider() {
        assert!(parse_words(&["run", "--oss", "--local-provider", "other", "fix"]).is_err());
        assert!(parse_words(&["run", "--local-provider", "ollama", "fix"]).is_err());
    }

    #[test]
    fn rejects_unknown_flags_and_missing_values() {
        assert!(parse_words(&["route", "--model", "abc"]).is_err());
        assert!(parse_words(&["run", "--model"]).is_err());
        assert!(parse_words(&["route", "   "]).is_err());
    }

    #[test]
    fn allows_explicit_user_model_and_effort() {
        let Action::Run(options) = parse_words(&[
            "run",
            "--model",
            "installed-model",
            "--effort",
            "low",
            "fix",
        ])
        .unwrap() else {
            panic!("expected run");
        };
        assert_eq!(options.model.as_deref(), Some("installed-model"));
        assert_eq!(options.effort.as_deref(), Some("low"));
    }
}
