use regex_syntax::ast::{Ast, ClassSet, ClassSetItem};
use regex_syntax::ast::parse::Parser;

fn escaped_class_character(pattern: &str) -> char {
    let ast = Parser::new().parse(pattern).unwrap();
    let Ast::ClassBracketed(ref class) = ast else {
        panic!("expected a bracketed class")
    };
    let ClassSet::Item(ref item) = class.kind else {
        panic!("expected a class item")
    };
    match item {
        ClassSetItem::Literal(lit) => lit.c,
        ClassSetItem::Union(union) => union.items.iter().find_map(|item| match item {
            ClassSetItem::Literal(lit) => Some(lit.c),
            _ => None,
        }).expect("expected a literal in class union"),
        other => panic!("expected a literal, got {other:?}"),
    }
}

#[test]
fn escaped_hyphen_stays_literal_in_character_classes() {
    assert_eq!(escaped_class_character(r"[\-]"), '-');
    assert_eq!(escaped_class_character(r"[\]]"), ']');
}

#[test]
fn invalid_descending_ranges_still_fail() {
    assert!(Parser::new().parse("[z-a]").is_err());
}
