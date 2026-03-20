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
