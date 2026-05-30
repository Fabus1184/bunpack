#[test]
fn test1() {
    let bytes: [u8; 4] = i32::to_le_bytes(12345678);
    let value: i32 = bunpack::unpack!("i", &bytes);
    assert_eq!(value, 12345678);
}

#[test]
fn test2() {
    let bytes = bunpack::pack!("ifcQ", 42, 98_765.43, '█', 0x1234567890abcdefu64);
    let value: (i32, f32, char, u64) = bunpack::unpack!("ifcQ", &bytes);
    assert_eq!(value, (42, 98_765.43, '█', 0x1234567890abcdefu64));
}

#[test]
fn test_str() {
    let str = "Hello, World!";
    let bytes = bunpack::pack!("<s", str);
    let value: [u8; 13] = bunpack::unpack!("[B;13]", &bytes);
    assert_eq!(value, *str.as_bytes());
}

#[test]
fn test_nested() {
    let values = [
        [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]],
        [[10.0, 11.0, 12.0], [13.0, 14.0, 15.0], [16.0, 17.0, 18.0]],
        [[19.0, 20.0, 21.0], [22.0, 23.0, 24.0], [25.0, 26.0, 27.0]],
    ];
    let bytes = bunpack::pack!("<[[[f;3];3];3]", values);
    let unpacked: [[[f32; 3]; 3]; 3] = bunpack::unpack!("<[[[f;3];3];3]", &bytes);
    assert_eq!(unpacked, values);
}

#[test]
fn test_nested2() {
    let values = [(1.0, 2.0, true), (3.0, 4.0, false), (5.0, 6.0, true)];

    let bytes = bunpack::pack!("<[ff?;3]", values);
    let unpacked: [(f32, f32, bool); 3] = bunpack::unpack!("<[ff?;3]", &bytes);
    assert_eq!(unpacked, values);
}

#[test]
fn test_transmut() {
    let bytes = bunpack::pack!("<fff", 1.0, 2.0, 3.0);
    let unpacked: [u8; 3 * size_of::<f32>()] = bunpack::unpack!("<[B;12]", &bytes);
    assert_eq!(
        unpacked.as_slice(),
        [
            1.0_f32.to_le_bytes(),
            2.0_f32.to_le_bytes(),
            3.0_f32.to_le_bytes()
        ]
        .concat()
    );
}

#[test]
fn test_nested_combined() {
    let values = [(1, 1.0, '🐈'), (2, 2.0, '🐕'), (3, 3.0, '🦅')];

    let bytes = bunpack::pack!("<[ifc; 3]", values);
    let unpacked: [(i32, f32, char); 3] = bunpack::unpack!("<[ifc; 3]", &bytes);
    assert_eq!(unpacked, values);
}

#[test]
fn test_byte_slice() {
    let str = "Hello, World!";

    let bytes = bunpack::pack!("<p", str.as_bytes());
    let unpacked: [u8; 13] = bunpack::unpack!("[B;13]", &bytes);
    assert_eq!(unpacked, *str.as_bytes());
}

#[test]
fn test_big_endian() {
    let bytes = bunpack::pack!(">H", 0x1234);
    let unpacked: u16 = bunpack::unpack!("<H", &bytes);
    assert_eq!(unpacked, 0x3412);
}

#[test]
fn test_read_write() {
    use std::io::Cursor;

    let mut cursor = Cursor::new(Vec::new());
    bunpack::pack_write!(&mut cursor, "<i?", 42, true).unwrap();

    assert_eq!(cursor.get_ref().as_slice(), &[42, 0, 0, 0, 1]);

    cursor.set_position(0);
    let value: (i32, bool) = bunpack::unpack_read!(&mut cursor, "<i?").unwrap();
    assert_eq!(value, (42, true));
}
