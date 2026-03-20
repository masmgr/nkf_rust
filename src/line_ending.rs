use std::borrow::Cow;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineEnding {
    Lf,
    CrLf,
    Cr,
}

/// Convert line endings, returning Cow to avoid allocation when no conversion is needed.
#[must_use]
pub fn convert_line_endings_cow(input: &str, target: LineEnding) -> Cow<'_, str> {
    let has_cr = input.as_bytes().contains(&b'\r');
    let has_lf = input.as_bytes().contains(&b'\n');

    // If target is LF and there's no CR, input is already normalized
    if target == LineEnding::Lf && !has_cr {
        return Cow::Borrowed(input);
    }
    // If no line endings at all, no conversion needed
    if !has_cr && !has_lf {
        return Cow::Borrowed(input);
    }
    Cow::Owned(convert_line_endings(input, target))
}

/// Convert line endings in a UTF-8 string.
/// Single-pass: normalizes all line endings (CRLF, CR, LF) and converts to the target in one traversal.
#[must_use]
pub fn convert_line_endings(input: &str, target: LineEnding) -> String {
    let target_str = match target {
        LineEnding::Lf => "\n",
        LineEnding::CrLf => "\r\n",
        LineEnding::Cr => "\r",
    };
    let mut result = String::with_capacity(input.len());
    let bytes = input.as_bytes();
    let mut start = 0;
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'\r' {
            result.push_str(&input[start..i]);
            result.push_str(target_str);
            // Skip \n after \r (CRLF treated as single line ending)
            if i + 1 < bytes.len() && bytes[i + 1] == b'\n' {
                i += 1;
            }
            i += 1;
            start = i;
        } else if bytes[i] == b'\n' {
            result.push_str(&input[start..i]);
            result.push_str(target_str);
            i += 1;
            start = i;
        } else {
            i += 1;
        }
    }
    result.push_str(&input[start..]);
    result
}

#[cfg(test)]
#[path = "tests/line_ending_tests.rs"]
mod tests;
