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
