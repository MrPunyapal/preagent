use serde::de::value::{Error, MapDeserializer};
use serde::Deserialize;

#[derive(Debug, PartialEq, serde_derive::Deserialize)]
#[serde(deny_unknown_fields)]
struct Strict {
    #[serde(alias = "quantity")]
    count: u32,
}

fn parse(entries: &[(&str, u32)]) -> Result<Strict, Error> {
    let values = entries.iter().map(|(k,v)| (*k,*v));
    Strict::deserialize(MapDeserializer::new(values))
}

#[test]
fn unknown_fields_are_rejected_in_a_strict_derived_struct() {
    assert!(parse(&[("count", 7), ("extra", 9)]).is_err());
    assert!(parse(&[("unknown", 1)]).is_err());
}

#[test]
fn known_fields_and_configured_aliases_still_deserialize() {
    assert_eq!(parse(&[("count", 7)]).unwrap(), Strict { count: 7 });
    assert_eq!(parse(&[("quantity", 8)]).unwrap(), Strict { count: 8 });
}
