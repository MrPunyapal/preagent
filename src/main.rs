mod agent;
mod cli;
mod context;
mod router;

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
            if options.json {
                println!("{}", decision.to_json());
            } else {
                println!("{}", decision.to_text());
            }
            ExitCode::SUCCESS
        }
        Action::Run(options) => run(options),
    }
}

fn run(options: Options) -> ExitCode {
    let ctx = context::discover(!options.no_git);
    let decision = router::classify(&options.prompt, &ctx);

    // No unverified model choice or prompt rewrite. Only user-specified
    // overrides are forwarded to Codex.
    let invocation = agent::codex_invocation(&options);
    if options.dry_run {
        println!("{}", decision.to_text());
        println!("Codex arguments: {}", invocation.display());
        println!("No agent was started. Original prompt is unchanged.");
        return ExitCode::SUCCESS;
    }

    eprintln!(
        "preagent: {} task, {} tier (advisory); launching Codex with unchanged prompt",
        decision.intent.as_str(),
        decision.tier.as_str()
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
