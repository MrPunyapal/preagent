use clap_builder::{Arg, ArgAction, Command};

fn command() -> Command {
    Command::new("fixture")
        .arg(Arg::new("verbose").long("verbose").action(ArgAction::SetTrue))
        .arg(Arg::new("file").required(true))
}

#[test]
fn after_double_dash_option_like_strings_are_literal_positionals() {
    let matches = command()
        .try_get_matches_from(["fixture", "--", "--verbose"])
        .expect("an option-like positional after -- must be accepted");
    assert_eq!(matches.get_one::<String>("file").map(String::as_str), Some("--verbose"));
    assert!(!matches.get_flag("verbose"));
}

#[test]
fn options_before_double_dash_keep_working() {
    let matches = command()
        .try_get_matches_from(["fixture", "--verbose", "--", "-file"])
        .unwrap();
    assert_eq!(matches.get_one::<String>("file").map(String::as_str), Some("-file"));
    assert!(matches.get_flag("verbose"));
}
