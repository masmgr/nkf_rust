use std::borrow::Cow;

/// Decode URL-encoded (%xx) byte sequences, returning Cow to avoid allocation when no encoding found.
#[must_use]
pub fn decode_url_input_cow(input: &[u8]) -> Cow<'_, [u8]> {
    if !input.contains(&b'%') {
        return Cow::Borrowed(input);
    }
    Cow::Owned(decode_url_input(input))
}

/// Decode hex-encoded (:xx) byte sequences, returning Cow to avoid allocation when no encoding found.
#[must_use]
pub fn decode_cap_input_cow(input: &[u8]) -> Cow<'_, [u8]> {
    if !input.contains(&b':') {
        return Cow::Borrowed(input);
    }
    Cow::Owned(decode_cap_input(input))
}

/// Decode numeric character references, returning Cow to avoid allocation when no references found.
#[must_use]
pub fn decode_numchar_input_cow(input: &str) -> Cow<'_, str> {
    if !input.contains("&#") {
        return Cow::Borrowed(input);
    }
    Cow::Owned(decode_numchar_input(input))
}

/// Decode URL-encoded (%xx) byte sequences.
#[must_use]
pub fn decode_url_input(input: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity(input.len());
    let mut i = 0;
    while i < input.len() {
        if input[i] == b'%' && i + 2 < input.len()
            && let Some(byte) = decode_hex_pair(input[i + 1], input[i + 2])
        {
            result.push(byte);
            i += 3;
            continue;
        }
        result.push(input[i]);
        i += 1;
    }
    result
}

/// Decode hex-encoded (:xx) byte sequences.
#[must_use]
pub fn decode_cap_input(input: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity(input.len());
    let mut i = 0;
    while i < input.len() {
        if input[i] == b':' && i + 2 < input.len()
            && let Some(byte) = decode_hex_pair(input[i + 1], input[i + 2])
        {
            result.push(byte);
            i += 3;
            continue;
        }
        result.push(input[i]);
        i += 1;
    }
    result
}

fn decode_hex_pair(h: u8, l: u8) -> Option<u8> {
    let high = hex_digit(h)?;
    let low = hex_digit(l)?;
    Some(high << 4 | low)
}

fn hex_digit(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'A'..=b'F' => Some(b - b'A' + 10),
        b'a'..=b'f' => Some(b - b'a' + 10),
        _ => None,
    }
}

/// Decode numeric character references (&#nnnn; and &#xhhhh;) in a UTF-8 string.
#[must_use]
pub fn decode_numchar_input(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if chars[i] == '&' && i + 2 < chars.len() && chars[i + 1] == '#' {
            // Try to parse &#xHHHH; or &#NNNN;
            let (is_hex, start) = if i + 3 < chars.len() && (chars[i + 2] == 'x' || chars[i + 2] == 'X') {
                (true, i + 3)
            } else {
                (false, i + 2)
            };

            // Find the semicolon
            let mut end = start;
            while end < chars.len() && chars[end] != ';' && end - start < 10 {
                end += 1;
            }

            if end < chars.len() && chars[end] == ';' && end > start {
                let num_str: String = chars[start..end].iter().collect();
                let parsed = if is_hex {
                    u32::from_str_radix(&num_str, 16).ok()
                } else {
                    num_str.parse::<u32>().ok()
                };

                if let Some(cp) = parsed
                    && let Some(c) = char::from_u32(cp)
                {
                    result.push(c);
                    i = end + 1;
                    continue;
                }
            }
        }
        result.push(chars[i]);
        i += 1;
    }

    result
}

#[cfg(test)]
#[path = "tests/input_decode_tests.rs"]
mod tests;
