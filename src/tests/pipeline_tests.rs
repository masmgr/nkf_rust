use super::*;

#[test]
fn test_process_utf8_to_sjis() {
    let input = "日本語".as_bytes();
    let options = NkfOptions {
        output_encoding: Some(EncodingType::ShiftJis),
        ..Default::default()
    };
    let result = process(input, &options).unwrap();
    assert_eq!(result, vec![0x93, 0xFA, 0x96, 0x7B, 0x8C, 0xEA]);
}

#[test]
fn test_process_guess_mode() {
    let input = "日本語".as_bytes();
    let options = NkfOptions {
        guess_mode: true,
        ..Default::default()
    };
    let result = process(input, &options).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "UTF-8\n");
}

#[test]
fn test_process_guess_sjis() {
    let sjis: &[u8] = &[0x93, 0xFA, 0x96, 0x7B, 0x8C, 0xEA];
    let options = NkfOptions {
        guess_mode: true,
        ..Default::default()
    };
    let result = process(sjis, &options).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "Shift_JIS\n");
}

#[test]
fn test_process_with_line_ending() {
    let input = "a\nb\nc".as_bytes();
    let options = NkfOptions {
        line_ending: Some(LineEnding::CrLf),
        ..Default::default()
    };
    let result = process(input, &options).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "a\r\nb\r\nc");
}

#[test]
fn test_process_sjis_to_eucjp() {
    let sjis: &[u8] = &[0x93, 0xFA, 0x96, 0x7B, 0x8C, 0xEA];
    let options = NkfOptions {
        input_encoding: Some(EncodingType::ShiftJis),
        output_encoding: Some(EncodingType::EucJp),
        ..Default::default()
    };
    let result = process(sjis, &options).unwrap();
    assert_eq!(result, vec![0xC6, 0xFC, 0xCB, 0xDC, 0xB8, 0xEC]);
}

#[test]
fn test_process_auto_detect_and_convert() {
    // Shift_JIS "日本語" -> auto-detect -> output as UTF-8
    let sjis: &[u8] = &[0x93, 0xFA, 0x96, 0x7B, 0x8C, 0xEA];
    let options = NkfOptions {
        output_encoding: Some(EncodingType::Utf8),
        ..Default::default()
    };
    let result = process(sjis, &options).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "日本語");
}

#[test]
fn test_process_empty_input() {
    let options = NkfOptions::default();
    let result = process(b"", &options).unwrap();
    assert!(result.is_empty());
}

#[test]
fn test_process_zen_conversion() {
    // Full-width "ＡＢＣ" -> half-width "ABC"
    let input = "\u{FF21}\u{FF22}\u{FF23}".as_bytes();
    let options = NkfOptions {
        zen_mode: Some(ZenMode::AlphaToAscii),
        preserve_hw_kana: true,
        ..Default::default()
    };
    let result = process(input, &options).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "ABC");
}

#[test]
fn test_process_preserve_hw_kana() {
    // Half-width katakana ｱｲｳ
    let input = "\u{FF71}\u{FF72}\u{FF73}".as_bytes();

    // With preserve_hw_kana: half-width kana should remain
    let options = NkfOptions {
        preserve_hw_kana: true,
        ..Default::default()
    };
    let result = process(input, &options).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "\u{FF71}\u{FF72}\u{FF73}");

    // Without preserve_hw_kana: half-width kana -> full-width
    let options = NkfOptions {
        preserve_hw_kana: false,
        ..Default::default()
    };
    let result = process(input, &options).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "アイウ");
}

#[test]
fn test_process_bom_handling() {
    let input = b"\xEF\xBB\xBFHello";

    // Output as UTF-8 (no BOM)
    let options = NkfOptions {
        output_encoding: Some(EncodingType::Utf8),
        preserve_hw_kana: true,
        ..Default::default()
    };
    let result = process(input, &options).unwrap();
    assert_eq!(result, b"Hello");

    // Output as UTF-8 with BOM
    let options = NkfOptions {
        output_encoding: Some(EncodingType::Utf8Bom),
        preserve_hw_kana: true,
        ..Default::default()
    };
    let result = process(input, &options).unwrap();
    assert!(result.starts_with(&[0xEF, 0xBB, 0xBF]));
    assert_eq!(&result[3..], b"Hello");
}

#[test]
fn test_process_mime_decode_then_convert() {
    // Base64 encode "Hello" as an RFC 2047 encoded-word
    let input = b"=?UTF-8?B?SGVsbG8=?=";
    let options = NkfOptions {
        mime_decode: Some(MimeDecodeMode::Base64),
        preserve_hw_kana: true,
        ..Default::default()
    };
    let result = process(input, &options).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "Hello");
}

#[test]
fn test_process_mime_encode() {
    let input = "Hello".as_bytes();
    let options = NkfOptions {
        mime_encode: Some(MimeEncodeMode::Base64),
        preserve_hw_kana: true,
        ..Default::default()
    };
    let result = process(input, &options).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.starts_with("=?UTF-8?B?"));
    assert!(result_str.ends_with("?="));
}
