use serde_core::de::value::{Error, I16Deserializer};
use serde_core::Deserialize;

#[test]
fn signed_byte_rejects_out_of_range_integers() {
    let above = i8::deserialize(I16Deserializer::<Error>::new(128));
    let below = i8::deserialize(I16Deserializer::<Error>::new(-129));
    assert!(above.is_err(), "i8 must reject 128 instead of truncating to -128");
    assert!(below.is_err(), "i8 must reject -129 instead of wrapping to 127");
}

#[test]
fn signed_byte_preserves_representable_boundaries() {
    assert_eq!(i8::deserialize(I16Deserializer::<Error>::new(-128)).unwrap(), -128);
    assert_eq!(i8::deserialize(I16Deserializer::<Error>::new(127)).unwrap(), 127);
    assert_eq!(i8::deserialize(I16Deserializer::<Error>::new(0)).unwrap(), 0);
}
