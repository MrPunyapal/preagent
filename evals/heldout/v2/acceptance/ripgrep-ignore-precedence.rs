use ignore::gitignore::GitignoreBuilder;

fn matcher(root: &str, rules: &str) -> ignore::gitignore::Gitignore {
    let mut builder = GitignoreBuilder::new(root);
    builder.add_str(None, rules).unwrap();
    builder.build().unwrap()
}

#[test]
fn last_matching_whitelist_rule_overrides_ignore() {
    let rules = matcher(".", "*.tmp\n!important.tmp\n");
    assert!(rules.matched("unimportant.tmp", false).is_ignore());
    assert!(rules.matched("important.tmp", false).is_whitelist());
    assert!(!rules.matched("other.rs", false).is_ignore());
}

#[test]
fn child_rules_do_not_change_parent_matcher() {
    let parent = matcher("workspace", "*.log\n");
    let child = matcher("workspace/sub", "*.log\n!keep.log\n");
    assert!(parent.matched("workspace/sub/keep.log", false).is_ignore());
    assert!(child.matched("workspace/sub/keep.log", false).is_whitelist());
    assert!(child.matched("workspace/sub/other.log", false).is_ignore());
}
