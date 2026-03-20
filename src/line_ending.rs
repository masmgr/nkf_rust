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
#[path = "tests/line_ending_tests.rs"]
mod tests;
