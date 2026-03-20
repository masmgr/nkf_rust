use super::*;

#[test]
fn test_utf8_to_shift_jis() {
    let input = "日本語".as_bytes();
    let result = convert(input, EncodingType::Utf8, EncodingType::ShiftJis).unwrap();
    // "日本語" in Shift_JIS
    assert_eq!(result, vec![0x93, 0xFA, 0x96, 0x7B, 0x8C, 0xEA]);
}

#[test]
fn test_shift_jis_to_utf8() {
    let input: &[u8] = &[0x93, 0xFA, 0x96, 0x7B, 0x8C, 0xEA];
    let result = convert(input, EncodingType::ShiftJis, EncodingType::Utf8).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "日本語");
}

#[test]
fn test_utf8_to_euc_jp() {
    let input = "日本語".as_bytes();
    let result = convert(input, EncodingType::Utf8, EncodingType::EucJp).unwrap();
    assert_eq!(result, vec![0xC6, 0xFC, 0xCB, 0xDC, 0xB8, 0xEC]);
}

#[test]
fn test_euc_jp_to_utf8() {
    let input: &[u8] = &[0xC6, 0xFC, 0xCB, 0xDC, 0xB8, 0xEC];
    let result = convert(input, EncodingType::EucJp, EncodingType::Utf8).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "日本語");
}

#[test]
fn test_shift_jis_to_euc_jp() {
    let sjis: &[u8] = &[0x93, 0xFA, 0x96, 0x7B, 0x8C, 0xEA];
    let result = convert(sjis, EncodingType::ShiftJis, EncodingType::EucJp).unwrap();
    assert_eq!(result, vec![0xC6, 0xFC, 0xCB, 0xDC, 0xB8, 0xEC]);
}

#[test]
fn test_utf8_to_iso2022jp() {
    let input = "日本語".as_bytes();
    let result = convert(input, EncodingType::Utf8, EncodingType::Iso2022Jp).unwrap();
    // Should contain ESC sequences
    assert!(result.contains(&0x1B));
}

#[test]
fn test_utf8_to_utf16be() {
    let input = "AB".as_bytes();
    let result = convert(input, EncodingType::Utf8, EncodingType::Utf16Be).unwrap();
    assert_eq!(result, vec![0x00, 0x41, 0x00, 0x42]);
}

#[test]
fn test_utf8_to_utf16le() {
    let input = "AB".as_bytes();
    let result = convert(input, EncodingType::Utf8, EncodingType::Utf16Le).unwrap();
    assert_eq!(result, vec![0x41, 0x00, 0x42, 0x00]);
}

#[test]
fn test_strip_utf8_bom() {
    let input = b"\xEF\xBB\xBFHello";
    let result = convert(input, EncodingType::Utf8Bom, EncodingType::Utf8).unwrap();
    assert_eq!(result, b"Hello");
}

#[test]
fn test_same_encoding() {
    let input = "テスト".as_bytes();
    let result = convert(input, EncodingType::Utf8, EncodingType::Utf8).unwrap();
    assert_eq!(result, input);
}
