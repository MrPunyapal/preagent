use regex_lite::Regex;

const SUITE: &str = include_str!("lib.rs");
const UTF8_CASES: &str = include_str!("../../testdata/utf8.toml");

#[test]
fn regression_suite_loads_zero_width_utf8_boundary_cases() {
    assert!(SUITE.contains("load!(\"utf8\");"));
    assert!(UTF8_CASES.contains("name = \"empty-utf8yes\""));
    assert!(UTF8_CASES.contains("matches = [[0, 0], [3, 3]]"));
}

#[test]
fn empty_matches_never_split_a_multibyte_codepoint() {
    let re = Regex::new("").unwrap();
    let got: Vec<(usize, usize)> = re.find_iter("☃").map(|m| (m.start(), m.end())).collect();
    assert_eq!(got, vec![(0, 0), (3, 3)]);
}
