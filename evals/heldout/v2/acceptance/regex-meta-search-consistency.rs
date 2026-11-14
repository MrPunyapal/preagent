use regex_automata::{meta::Regex, Match, Span};

#[test]
fn zero_width_find_and_captures_agree() {
    let regex = Regex::new(r"(?m)^").unwrap();
    let text = "one\ntwo";
    assert_eq!(regex.find(text), Some(Match::must(0, 0..0)));
    let mut captures = regex.create_captures();
    regex.captures(text, &mut captures);
    assert_eq!(captures.get_group(0), Some(Span::from(0..0)));
}

#[test]
fn nonempty_and_missing_searches_remain_consistent() {
    let regex = Regex::new("ab").unwrap();
    assert_eq!(regex.find("xxabyy"), Some(Match::must(0, 2..4)));
    let mut captures = regex.create_captures();
    regex.captures("xxabyy", &mut captures);
    assert_eq!(captures.get_group(0), Some(Span::from(2..4)));
    assert_eq!(regex.find("zz"), None);
    regex.captures("zz", &mut captures);
    assert!(!captures.is_match());
}
