use super::*;

#[test]
fn test_base64_decode() {
    let encoded = general_purpose::STANDARD.encode(b"Hello, World!");
    let result = decode_base64(encoded.as_bytes());
    assert_eq!(result, b"Hello, World!");
}

#[test]
fn test_quoted_printable_decode() {
    let input = b"Hello=20World";
    let result = decode_quoted_printable(input);
    assert_eq!(result, b"Hello World");
}

#[test]
fn test_rfc2047_base64() {
    // =?UTF-8?B?44GT44KT44Gr44Gh44Gv?= = "こんにちは" in UTF-8 base64
    let encoded = general_purpose::STANDARD.encode("こんにちは".as_bytes());
    let input = format!("=?UTF-8?B?{encoded}?=");
    let result = mime_decode(input.as_bytes(), MimeDecodeMode::Base64);
    assert_eq!(String::from_utf8(result).unwrap(), "こんにちは");
}

#[test]
fn test_mime_encode_base64() {
    let result = mime_encode("Hello", MimeEncodeMode::Base64, "UTF-8");
    assert!(result.starts_with("=?UTF-8?B?"));
    assert!(result.ends_with("?="));
}

#[test]
fn test_mime_encode_qp() {
    let result = mime_encode("Hello", MimeEncodeMode::QuotedPrintable, "UTF-8");
    assert!(result.starts_with("=?UTF-8?Q?"));
    assert!(result.ends_with("?="));
}

#[test]
fn test_mime_decode_none() {
    let input = b"Hello";
    let result = mime_decode(input, MimeDecodeMode::None);
    assert_eq!(result, input);
}

#[test]
fn test_multiple_encoded_words() {
    let word1 = general_purpose::STANDARD.encode(b"Hello");
    let word2 = general_purpose::STANDARD.encode(b"World");
    let input = format!("=?UTF-8?B?{word1}?= =?UTF-8?B?{word2}?=");
    let result = mime_decode(input.as_bytes(), MimeDecodeMode::Base64);
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("Hello"));
    assert!(result_str.contains("World"));
}

#[test]
fn test_incomplete_encoded_word() {
    // Missing closing ?=
    let input = b"=?UTF-8?B?SGVsbG8";
    let result = mime_decode(input, MimeDecodeMode::Base64);
    // Should fall back to raw base64 decode or return original
    // The "=?" will be found but pattern won't complete, so it falls through
    assert!(!result.is_empty());
}

#[test]
fn test_empty_base64_payload() {
    let input = b"=?UTF-8?B??=";
    let result = mime_decode(input, MimeDecodeMode::Base64);
    // Empty base64 decodes to empty string within the encoded-word
    let result_str = String::from_utf8(result).unwrap();
    assert!(!result_str.contains("=?"));
}

#[test]
fn test_qp_soft_line_break() {
    let input = b"Hello=\r\nWorld";
    let result = decode_quoted_printable(input);
    assert_eq!(result, b"HelloWorld");
}

#[test]
fn test_qp_underscore_to_space() {
    let input = b"Hello_World";
    let result = decode_quoted_printable(input);
    assert_eq!(result, b"Hello World");
}
