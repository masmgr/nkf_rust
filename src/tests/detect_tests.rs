use super::*;

#[test]
fn test_detect_ascii() {
    let result = detect(b"Hello, World!");
    assert_eq!(result.encoding, EncodingType::Ascii);
    assert!(!result.had_bom);
}

#[test]
fn test_detect_utf8_bom() {
    let input = b"\xEF\xBB\xBFHello";
    let result = detect(input);
    assert_eq!(result.encoding, EncodingType::Utf8Bom);
    assert!(result.had_bom);
}

#[test]
fn test_detect_utf16be_bom() {
    let input = b"\xFE\xFF\x00H\x00e";
    let result = detect(input);
    assert_eq!(result.encoding, EncodingType::Utf16Be);
    assert!(result.had_bom);
}

#[test]
fn test_detect_utf16le_bom() {
    let input = b"\xFF\xFEH\x00e\x00";
    let result = detect(input);
    assert_eq!(result.encoding, EncodingType::Utf16Le);
    assert!(result.had_bom);
}

#[test]
fn test_detect_iso2022jp() {
    // ESC $ B indicates ISO-2022-JP
    let input = b"\x1B$B$3$s$K$A$O\x1B(B";
    let result = detect(input);
    assert_eq!(result.encoding, EncodingType::Iso2022Jp);
}

#[test]
fn test_detect_empty() {
    let result = detect(b"");
    assert_eq!(result.encoding, EncodingType::Ascii);
}

#[test]
fn test_detect_utf8() {
    // "日本語" in UTF-8
    let input = "日本語テスト".as_bytes();
    let result = detect(input);
    assert_eq!(result.encoding, EncodingType::Utf8);
}

#[test]
fn test_detect_shift_jis() {
    // "日本語" in Shift_JIS
    let input: &[u8] = &[
        0x93, 0xFA, 0x96, 0x7B, 0x8C, 0xEA, 0x83, 0x65, 0x83, 0x58, 0x83, 0x67,
    ];
    let result = detect(input);
    assert_eq!(result.encoding, EncodingType::ShiftJis);
}

#[test]
fn test_detect_euc_jp() {
    // "日本語" in EUC-JP
    let input: &[u8] = &[
        0xC6, 0xFC, 0xCB, 0xDC, 0xB8, 0xEC, 0xA5, 0xC6, 0xA5, 0xB9, 0xA5, 0xC8,
    ];
    let result = detect(input);
    assert_eq!(result.encoding, EncodingType::EucJp);
}
