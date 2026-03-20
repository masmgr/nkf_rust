use super::*;

#[test]
fn test_lf_to_crlf() {
    assert_eq!(
        convert_line_endings("a\nb\nc", LineEnding::CrLf),
        "a\r\nb\r\nc"
    );
}

#[test]
fn test_crlf_to_lf() {
    assert_eq!(
        convert_line_endings("a\r\nb\r\nc", LineEnding::Lf),
        "a\nb\nc"
    );
}

#[test]
fn test_cr_to_lf() {
    assert_eq!(convert_line_endings("a\rb\rc", LineEnding::Lf), "a\nb\nc");
}

#[test]
fn test_lf_to_cr() {
    assert_eq!(convert_line_endings("a\nb\nc", LineEnding::Cr), "a\rb\rc");
}

#[test]
fn test_mixed_to_lf() {
    assert_eq!(
        convert_line_endings("a\r\nb\rc\nd", LineEnding::Lf),
        "a\nb\nc\nd"
    );
}

#[test]
fn test_no_line_endings() {
    assert_eq!(convert_line_endings("hello", LineEnding::CrLf), "hello");
}

#[test]
fn test_multiple_consecutive_crlf() {
    assert_eq!(
        convert_line_endings("a\r\n\r\nb", LineEnding::Lf),
        "a\n\nb"
    );
    assert_eq!(
        convert_line_endings("a\n\nb", LineEnding::CrLf),
        "a\r\n\r\nb"
    );
}
