/// Fold lines at the specified column width.
/// CJK characters are counted as 2 columns wide.
#[must_use]
pub fn fold_lines(input: &str, columns: usize) -> String {
    let mut result = String::with_capacity(input.len());

    let ends_with_newline = input.ends_with('\n');
    let trimmed = if ends_with_newline {
        &input[..input.len() - 1]
    } else {
        input
    };

    let mut lines = trimmed.split('\n').peekable();
    while let Some(line) = lines.next() {
        if !line.is_empty() {
            fold_single_line(line, columns, &mut result);
        }
        if lines.peek().is_some() {
            result.push('\n');
        }
    }

    if ends_with_newline {
        result.push('\n');
    }

    result
}

fn fold_single_line(line: &str, columns: usize, result: &mut String) {
    let mut col = 0;
    let mut last_space_pos = None; // byte position in result where last space was added
    let mut line_start = result.len();

    for c in line.chars() {
        let w = char_width(c);

        if col + w > columns {
            // Try to break at the last space
            if let Some(space_pos) = last_space_pos {
                // Replace the space with a newline
                unsafe {
                    result.as_mut_vec()[space_pos] = b'\n';
                }
                // Recalculate column from the break point
                col = result[space_pos + 1..]
                    .chars()
                    .map(char_width)
                    .sum::<usize>();
                line_start = space_pos + 1;
                last_space_pos = None;
            } else {
                // Hard break
                result.push('\n');
                col = 0;
                line_start = result.len();
            }
        }

        if c == ' ' && col > 0 {
            last_space_pos = Some(result.len());
        }

        result.push(c);
        col += w;

        // Reset space tracking on newline
        if c == '\n' {
            col = 0;
            last_space_pos = None;
            line_start = result.len();
        }
    }

    let _ = line_start; // suppress unused warning
}

fn char_width(c: char) -> usize {
    let cp = c as u32;
    // CJK Unified Ideographs and common fullwidth ranges
    if is_wide_char(cp) {
        2
    } else {
        1
    }
}

fn is_wide_char(cp: u32) -> bool {
    // CJK Unified Ideographs
    (0x4E00..=0x9FFF).contains(&cp)
    // CJK Unified Ideographs Extension A
    || (0x3400..=0x4DBF).contains(&cp)
    // CJK Unified Ideographs Extension B+
    || (0x20000..=0x2A6DF).contains(&cp)
    // CJK Compatibility Ideographs
    || (0xF900..=0xFAFF).contains(&cp)
    // Hiragana
    || (0x3040..=0x309F).contains(&cp)
    // Katakana
    || (0x30A0..=0x30FF).contains(&cp)
    // Fullwidth Forms
    || (0xFF01..=0xFF60).contains(&cp)
    || (0xFFE0..=0xFFE6).contains(&cp)
    // CJK Symbols and Punctuation
    || (0x3000..=0x303F).contains(&cp)
    // Hangul Syllables
    || (0xAC00..=0xD7AF).contains(&cp)
    // Katakana Phonetic Extensions
    || (0x31F0..=0x31FF).contains(&cp)
    // Enclosed CJK Letters and Months
    || (0x3200..=0x32FF).contains(&cp)
    // CJK Compatibility
    || (0x3300..=0x33FF).contains(&cp)
}

#[cfg(test)]
#[path = "tests/fold_tests.rs"]
mod tests;
