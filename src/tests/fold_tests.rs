use super::*;

#[test]
fn test_fold_short_line() {
    assert_eq!(fold_lines("hello", 60), "hello");
}

#[test]
fn test_fold_at_space() {
    let input = "hello world this is a test";
    let result = fold_lines(input, 15);
    // Should break at word boundaries
    assert!(result.contains('\n'));
    for line in result.lines() {
        // Each line should be within column limit (approximately)
        assert!(line.len() <= 16, "Line too long: '{line}'");
    }
}

#[test]
fn test_fold_preserves_existing_newlines() {
    let input = "line1\nline2\nline3";
    let result = fold_lines(input, 60);
    assert_eq!(result, "line1\nline2\nline3");
}

#[test]
fn test_fold_long_word() {
    let input = "abcdefghijklmnopqrstuvwxyz";
    let result = fold_lines(input, 10);
    // Should hard-break since there's no space
    assert!(result.contains('\n'));
}

#[test]
fn test_fold_cjk_width() {
    // CJK characters are 2 columns wide
    // 5 CJK chars = 10 columns
    let input = "あいうえお";
    let result = fold_lines(input, 10);
    assert_eq!(result, "あいうえお"); // exactly fits
}

#[test]
fn test_fold_cjk_overflow() {
    // 6 CJK chars = 12 columns, fold at 10
    let input = "あいうえおか";
    let result = fold_lines(input, 10);
    assert!(result.contains('\n'));
}

#[test]
fn test_fold_empty() {
    assert_eq!(fold_lines("", 60), "");
}

#[test]
fn test_fold_trailing_newline() {
    let input = "hello\n";
    let result = fold_lines(input, 60);
    assert_eq!(result, "hello\n");
}
