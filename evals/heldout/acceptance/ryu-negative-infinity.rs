use ryu::Buffer;

#[test]
fn negative_infinity_keeps_its_sign_for_both_float_sizes() {
    let mut buffer = Buffer::new();
    assert_eq!(buffer.format(f32::INFINITY), "inf");
    assert_eq!(buffer.format(f32::NEG_INFINITY), "-inf");
    assert_eq!(buffer.format(f64::INFINITY), "inf");
    assert_eq!(buffer.format(f64::NEG_INFINITY), "-inf");
}

#[test]
fn nan_and_finite_values_remain_unchanged() {
    let mut buffer = Buffer::new();
    assert_eq!(buffer.format(f32::NAN), "NaN");
    assert_eq!(buffer.format(f64::NAN), "NaN");
    assert_eq!(buffer.format(1.25f32), "1.25");
    assert_eq!(buffer.format(1.25f64), "1.25");
}
