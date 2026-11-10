mod agent;
mod cli;
mod context;
mod doctor;
mod evaluation;
mod router;
mod slm;

use cli::{Action, Options};
use std::process::ExitCode;

fn main() -> ExitCode {
    let action = match cli::parse(std::env::args().skip(1)) {
        Ok(action) => action,
        Err(message) => {
            eprintln!("preagent: {message}");
            eprintln!("Run 'preagent --help' for usage.");
            return ExitCode::from(2);
        }
    };

    match action {
        Action::Help => {
            println!("{}", cli::HELP);
            ExitCode::SUCCESS
        }
        Action::Version => {
            println!("preagent {}", env!("CARGO_PKG_VERSION"));
            ExitCode::SUCCESS
        }
        Action::Route(options) => {
            let ctx = context::discover(!options.no_git);
            let decision = router::classify(&options.prompt, &ctx);
            let mut status = "not_requested";
            let mut suggested = None;
            let mut fallback = None;
            if let Some(model) = options.slm.as_deref() {
                if options.slm_always || decision.uncertain {
                    match slm::classify(&options.prompt, model) {
                        Ok(tier) => {
                            suggested = Some(tier);
                            status = "evaluated";
                        }
                        Err(error) => {
                            fallback = Some(error);
                            status = "unavailable";
                        }
                    }
                } else {
                    status = "skipped_confident_rules";
                }
            }
            let recommended = suggested
                .map(|tier| slm::safer_tier(decision.tier, tier))
                .unwrap_or(decision.tier);
            if options.json {
                let baseline = decision.to_json();
                let prefix = baseline.strip_suffix('}').expect("JSON object");
                let slm_tier = suggested
                    .map(|tier| format!("\"{}\"", tier.as_str()))
                    .unwrap_or_else(|| "null".to_string());
                println!(
                    "{prefix},\"slm_status\":\"{status}\",\"slm_tier\":{slm_tier},\"recommended_tier\":\"{}\"}}",
                    recommended.as_str()
                );
            } else {
                println!("{}", decision.to_text());
                println!("SLM status: {status}");
                if let Some(tier) = suggested {
                    println!("SLM suggested: {} (advisory)", tier.as_str());
                }
                if let Some(message) = fallback {
                    println!("Local SLM fallback: {message}");
                }
                println!("Recommended tier: {} (not applied)", recommended.as_str());
            }
            ExitCode::SUCCESS
        }
        Action::Run(options) => run(options),
        Action::Doctor { json } => {
            let results = doctor::check();
            if json {
                println!("{}", results.to_json());
            } else {
                println!("{}", results.to_text());
            }
            ExitCode::SUCCESS
        }
        Action::Eval { json } => match evaluation::evaluate_builtin() {
            Ok(report) => {
                if json {
                    println!("{}", report.to_json());
                } else {
                    println!("{}", report.to_text());
                }
                ExitCode::SUCCESS
            }
            Err(message) => {
                eprintln!("preagent: benchmark data invalid: {message}");
                ExitCode::FAILURE
            }
        },
    }
}

fn run(options: Options) -> ExitCode {
    let ctx = context::discover(!options.no_git);
    let decision = router::classify(&options.prompt, &ctx);

    // No unverified model choice or prompt rewrite. Only user-specified
    // overrides are forwarded to Codex.
    let invocation = agent::invocation(&options);
    if options.dry_run {
        println!("{}", decision.to_text());
        println!("Agent arguments: {}", invocation.display());
        println!("No agent was started. Original prompt is unchanged.");
        return ExitCode::SUCCESS;
    }

    eprintln!(
        "preagent: {} task, {} tier (advisory); launching {} with unchanged prompt",
        decision.intent.as_str(),
        decision.tier.as_str(),
        options.agent.as_str()
    );
    match invocation.execute() {
        Ok(code) if (0..=255).contains(&code) => ExitCode::from(code as u8),
        Ok(_) => ExitCode::FAILURE,
        Err(message) => {
            eprintln!("preagent: {message}");
            ExitCode::FAILURE
        }
    }
}
