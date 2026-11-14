use globset::GlobBuilder;

#[test]
fn escaped_metacharacters_match_only_literals() {
    for (pattern, literal, other) in [
        (r"\?", "?", "*"),
        (r"\*", "*", "x"),
        (r"\[", "[", "x"),
        (r"\{", "{", "x"),
    ] {
        let matcher = GlobBuilder::new(pattern)
            .backslash_escape(true)
            .build()
            .unwrap()
            .compile_matcher();
        assert!(matcher.is_match(literal), "{pattern} must match its literal");
        assert!(!matcher.is_match(other), "{pattern} must not match another literal");
    }
}

#[test]
fn ordinary_wildcards_are_still_wildcards() {
    let matcher = GlobBuilder::new("?").backslash_escape(true).build().unwrap().compile_matcher();
    assert!(matcher.is_match("x"));
    assert!(!matcher.is_match("two"));
}
