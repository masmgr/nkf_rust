#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineEnding {
    Lf,
    CrLf,
    Cr,
}

/// Convert line endings in a UTF-8 string.
/// First normalizes all line endings to LF, then converts to the target.
#[must_use]
pub fn convert_line_endings(input: &str, target: LineEnding) -> String {
    // Normalize: replace CRLF first, then standalone CR -> LF
    let normalized = input.replace("\r\n", "\n").replace('\r', "\n");

    match target {
        LineEnding::Lf => normalized,
        LineEnding::CrLf => normalized.replace('\n', "\r\n"),
        LineEnding::Cr => normalized.replace('\n', "\r"),
    }
}

#[cfg(test)]
mod tests {
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
}
