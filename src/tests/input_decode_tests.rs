use super::*;

#[test]
fn test_decode_url_basic() {
    let input = b"Hello%20World";
    assert_eq!(decode_url_input(input), b"Hello World");
}

#[test]
fn test_decode_url_japanese() {
    // %E3%81%82 = あ in UTF-8
    let input = b"%E3%81%82";
    let result = decode_url_input(input);
    assert_eq!(std::str::from_utf8(&result).unwrap(), "あ");
}

#[test]
fn test_decode_url_no_encoding() {
    let input = b"plain text";
    assert_eq!(decode_url_input(input), b"plain text");
}

#[test]
fn test_decode_url_incomplete() {
    let input = b"test%2";
    assert_eq!(decode_url_input(input), b"test%2");
}

#[test]
fn test_decode_url_invalid_hex() {
    let input = b"test%GG";
    assert_eq!(decode_url_input(input), b"test%GG");
}

#[test]
fn test_decode_cap_basic() {
    let input = b"Hello:20World";
    assert_eq!(decode_cap_input(input), b"Hello World");
}

#[test]
fn test_decode_cap_no_encoding() {
    let input = b"plain text";
    assert_eq!(decode_cap_input(input), b"plain text");
}

#[test]
fn test_decode_numchar_decimal() {
    assert_eq!(decode_numchar_input("&#12354;"), "あ"); // U+3042
}

#[test]
fn test_decode_numchar_hex() {
    assert_eq!(decode_numchar_input("&#x3042;"), "あ");
}

#[test]
fn test_decode_numchar_hex_upper() {
    assert_eq!(decode_numchar_input("&#X3042;"), "あ");
}

#[test]
fn test_decode_numchar_mixed() {
    assert_eq!(
        decode_numchar_input("Hello &#x3042;&#12356;!"),
        "Hello あい!"
    );
}

#[test]
fn test_decode_numchar_no_semicolon() {
    assert_eq!(decode_numchar_input("&#12354"), "&#12354");
}

#[test]
fn test_decode_numchar_invalid() {
    assert_eq!(decode_numchar_input("&#xZZZZ;"), "&#xZZZZ;");
}

#[test]
fn test_decode_numchar_ascii() {
    assert_eq!(decode_numchar_input("&#65;"), "A");
}
