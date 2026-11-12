use itoa::Buffer;

#[test]
fn decimal_pairs_keep_zeros_and_exact_digits() {
    let mut buf = Buffer::new();
    assert_eq!(buf.format(0u64), "0");
    assert_eq!(buf.format(100u64), "100");
    assert_eq!(buf.format(1000u64), "1000");
    assert_eq!(buf.format(1005u64), "1005");
    assert_eq!(buf.format(100_001u64), "100001");
    assert_eq!(buf.format(10_000_000u64), "10000000");
    assert_eq!(buf.format(-100_001i64), "-100001");
    assert_eq!(buf.format(u64::MAX), "18446744073709551615");
    assert_eq!(buf.format(i128::MIN), "-170141183460469231731687303715884105728");
}

#[test]
fn signed_and_unsigned_agree_with_standard_decimal_formatting() {
    let mut buf = Buffer::new();
    for input in [1u64, 10, 20, 100, 101, 200, 900, 1_001, 99_009, 12_000_003] {
        assert_eq!(buf.format(input), input.to_string());
    }
    for input in [-1i64, -10, -100, -1000, -100_050, i64::MIN] {
        assert_eq!(buf.format(input), input.to_string());
    }
}
