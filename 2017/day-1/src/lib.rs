/// b'4' → 4
pub fn ascii_byte_to_val(byte: u8) -> usize {
    (byte - b'0') as usize
}

#[test]
fn test_ascii_byte_to_val() {
    assert_eq!(4, ascii_byte_to_val(b'4'))
}
