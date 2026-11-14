use regex_automata::meta::Regex;

#[test]
fn explicit_nfa_size_limit_rejects_large_unicode_class() {
    let result = Regex::builder()
        .configure(Regex::config().nfa_size_limit(Some(20 * (1 << 10))))
        .build(r"\pL");
    assert!(result.is_err(),
        "NFA allocation must respect an explicit small limit");
}

#[test]
fn reasonable_limits_allow_small_valid_patterns() {
    let re = Regex::builder()
        .configure(Regex::config().nfa_size_limit(Some(1 << 20)))
        .build(r"[a-z]+")
        .expect("a small expression must compile");
    assert_eq!(re.find("abc123").unwrap().range(), 0..3);
}
