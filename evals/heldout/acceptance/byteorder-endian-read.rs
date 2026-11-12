use byteorder::{BigEndian, ByteOrder, LittleEndian};

#[test]
fn big_endian_u16_decoding_uses_network_byte_order() {
    assert_eq!(BigEndian::read_u16(&[0x12, 0x34]), 0x1234);
    assert_eq!(BigEndian::read_u16(&[0x80, 0x01]), 0x8001);
    assert_eq!(BigEndian::read_u16(&[0xff, 0x00]), 0xff00);
    assert_eq!(BigEndian::read_u16(&[0x00, 0x01]), 1);
}

#[test]
fn little_endian_and_wider_decoding_are_not_regressed() {
    assert_eq!(LittleEndian::read_u16(&[0x12, 0x34]), 0x3412);
    assert_eq!(BigEndian::read_u32(&[0x01, 0x23, 0x45, 0x67]), 0x01234567);
    assert_eq!(LittleEndian::read_u32(&[0x01, 0x23, 0x45, 0x67]), 0x67452301);
    let mut bytes = [0u8; 2];
    BigEndian::write_u16(&mut bytes, 0x1234);
    assert_eq!(bytes, [0x12, 0x34]);
    assert_eq!(BigEndian::read_u16(&bytes), 0x1234);
}
