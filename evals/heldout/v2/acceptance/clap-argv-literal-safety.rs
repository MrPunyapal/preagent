use clap_builder::{Arg, Command};

fn command() -> Command {
    Command::new("fixture").arg(Arg::new("file").required(true))
}

#[test]
fn shell_like_arguments_after_terminator_are_literal_values() {
    for arg in ["--$(touch)", "--$HOME", "$PATH;literal"] {
        let matches = command()
            .try_get_matches_from(["fixture", "--", arg])
            .expect("no expansion or option parsing after --");
        assert_eq!(matches.get_one::<String>("file").map(String::as_str), Some(arg));
    }
}

#[test]
fn unrecognized_options_before_terminator_are_rejected() {
    assert!(command().try_get_matches_from(["fixture", "--unknown"]).is_err());
}
