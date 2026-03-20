use base64::{Engine as _, engine::general_purpose};

use crate::error::NkfError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MimeDecodeMode {
    Base64,
    QuotedPrintable,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MimeEncodeMode {
    Base64,
    QuotedPrintable,
}

/// Decode MIME encoded-words in input bytes (RFC 2047).
/// Handles patterns like =?charset?B?base64data?= and =?charset?Q?qpdata?=
pub fn mime_decode(input: &[u8], mode: MimeDecodeMode) -> Vec<u8> {
    if mode == MimeDecodeMode::None {
        return input.to_vec();
    }

    let input_str = match std::str::from_utf8(input) {
        Ok(s) => s,
        Err(_) => return input.to_vec(),
    };

    // Try to decode RFC 2047 encoded-words
    let decoded = decode_rfc2047(input_str);
    if decoded != input {
        return decoded;
    }

    // If no RFC 2047 patterns found, try raw decode
    match mode {
        MimeDecodeMode::Base64 => decode_base64(input),
        MimeDecodeMode::QuotedPrintable => decode_quoted_printable(input),
        MimeDecodeMode::None => input.to_vec(),
    }
}

/// Encode content as MIME (RFC 2047 encoded-word).
pub fn mime_encode(input: &str, mode: MimeEncodeMode, charset: &str) -> String {
    match mode {
        MimeEncodeMode::Base64 => {
            let encoded = general_purpose::STANDARD.encode(input.as_bytes());
            format!("=?{}?B?{}?=", charset, encoded)
        }
        MimeEncodeMode::QuotedPrintable => {
            let encoded = encode_quoted_printable(input.as_bytes());
            format!("=?{}?Q?{}?=", charset, encoded)
        }
    }
}

fn decode_rfc2047(input: &str) -> Vec<u8> {
    let mut result = String::new();
    let mut rest = input;
    let mut found = false;

    while let Some(start) = rest.find("=?") {
        result.push_str(&rest[..start]);
        let after_start = &rest[start + 2..];

        // Find charset
        if let Some(q1) = after_start.find('?') {
            let _charset = &after_start[..q1];
            let after_charset = &after_start[q1 + 1..];

            // Find encoding type (B or Q)
            if after_charset.len() >= 2 && after_charset.as_bytes()[1] == b'?' {
                let enc_type = after_charset.as_bytes()[0];
                let after_enc = &after_charset[2..];

                // Find closing ?=
                if let Some(end) = after_enc.find("?=") {
                    let encoded_data = &after_enc[..end];

                    let decoded = match enc_type {
                        b'B' | b'b' => general_purpose::STANDARD
                            .decode(encoded_data)
                            .unwrap_or_else(|_| encoded_data.as_bytes().to_vec()),
                        b'Q' | b'q' => decode_quoted_printable(encoded_data.as_bytes()),
                        _ => encoded_data.as_bytes().to_vec(),
                    };

                    // For now, treat decoded bytes as the charset-encoded content
                    // In a full implementation, we would convert from _charset to UTF-8
                    result.push_str(&String::from_utf8_lossy(&decoded));
                    rest = &after_enc[end + 2..];
                    found = true;
                    continue;
                }
            }
        }

        // No valid pattern found, copy the =? and continue
        result.push_str("=?");
        rest = after_start;
    }

    result.push_str(rest);

    if found {
        result.into_bytes()
    } else {
        input.as_bytes().to_vec()
    }
}

fn decode_base64(input: &[u8]) -> Vec<u8> {
    let filtered: Vec<u8> = input
        .iter()
        .filter(|&&b| b != b'\n' && b != b'\r' && b != b' ')
        .copied()
        .collect();

    general_purpose::STANDARD
        .decode(&filtered)
        .unwrap_or_else(|_| input.to_vec())
}

fn decode_quoted_printable(input: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    let mut i = 0;

    while i < input.len() {
        if input[i] == b'=' {
            if i + 2 < input.len() {
                let hi = input[i + 1];
                let lo = input[i + 2];
                if let (Some(h), Some(l)) = (hex_val(hi), hex_val(lo)) {
                    result.push(h << 4 | l);
                    i += 3;
                    continue;
                }
            }
            // Soft line break (= at end of line)
            if i + 1 < input.len() && (input[i + 1] == b'\n' || input[i + 1] == b'\r') {
                i += 2;
                if i < input.len() && input[i] == b'\n' {
                    i += 1;
                }
                continue;
            }
            result.push(input[i]);
            i += 1;
        } else if input[i] == b'_' {
            // In Q encoding, _ represents space
            result.push(b' ');
            i += 1;
        } else {
            result.push(input[i]);
            i += 1;
        }
    }

    result
}

fn encode_quoted_printable(input: &[u8]) -> String {
    let mut result = String::new();

    for &byte in input {
        if byte == b' ' {
            result.push('_');
        } else if byte.is_ascii_alphanumeric() || byte == b'.' || byte == b'-' || byte == b'*' {
            result.push(byte as char);
        } else {
            result.push_str(&format!("={:02X}", byte));
        }
    }

    result
}

fn hex_val(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'A'..=b'F' => Some(b - b'A' + 10),
        b'a'..=b'f' => Some(b - b'a' + 10),
        _ => None,
    }
}

/// Parse MIME decode mode from nkf -m flag value.
pub fn parse_mime_decode_mode(s: &str) -> Result<MimeDecodeMode, NkfError> {
    match s {
        "B" | "b" => Ok(MimeDecodeMode::Base64),
        "Q" | "q" => Ok(MimeDecodeMode::QuotedPrintable),
        "0" => Ok(MimeDecodeMode::None),
        "" => Ok(MimeDecodeMode::Base64), // default
        _ => Err(NkfError::InvalidMime(format!("Unknown MIME decode mode: {}", s))),
    }
}

/// Parse MIME encode mode from nkf -M flag value.
pub fn parse_mime_encode_mode(s: &str) -> Result<MimeEncodeMode, NkfError> {
    match s {
        "B" | "b" | "" => Ok(MimeEncodeMode::Base64), // default
        "Q" | "q" => Ok(MimeEncodeMode::QuotedPrintable),
        _ => Err(NkfError::InvalidMime(format!("Unknown MIME encode mode: {}", s))),
    }
}

#[cfg(test)]
mod tests {
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
        let input = format!("=?UTF-8?B?{}?=", encoded);
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
}
