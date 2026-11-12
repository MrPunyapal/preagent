use byteorder::{BigEndian, ByteOrder, LittleEndian};

#[test]
fn signed_64bit_partial_widths_preserve_sign() {
    for width in [1usize, 2, 3, 5, 7, 8] {
        for value in [-1i64, -2, -17, 0, 1, 2, 17] {
            let mut be = [0u8; 8];
            let mut le = [0u8; 8];
            BigEndian::write_int(&mut be, value, width);
            LittleEndian::write_int(&mut le, value, width);
            assert_eq!(BigEndian::read_int(&be, width), value, "BE width={width}");
            assert_eq!(LittleEndian::read_int(&le, width), value, "LE width={width}");
        }
    }
    assert_eq!(BigEndian::read_int(&[0x80, 0, 0], 3), -8_388_608);
    assert_eq!(LittleEndian::read_int(&[0, 0, 0x80], 3), -8_388_608);
}

#[test]
fn signed_128bit_partial_widths_preserve_sign() {
    for width in [1usize, 3, 7, 9, 12, 15, 16] {
        for value in [-1i128, -2, -17, 0, 1, 2, 17] {
            let mut be = [0u8; 16];
            let mut le = [0u8; 16];
            BigEndian::write_int128(&mut be, value, width);
            LittleEndian::write_int128(&mut le, value, width);
            assert_eq!(BigEndian::read_int128(&be, width), value, "BE width={width}");
            assert_eq!(LittleEndian::read_int128(&le, width), value, "LE width={width}");
        }
    }
    assert_eq!(BigEndian::read_int128(&[0x80, 0, 0], 3), -8_388_608);
    assert_eq!(LittleEndian::read_int128(&[0, 0, 0x80], 3), -8_388_608);
}
