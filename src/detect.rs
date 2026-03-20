use chardetng::EncodingDetector;

use crate::encoding_type::{self, EncodingType};

#[derive(Debug)]
pub struct DetectionResult {
    pub encoding: EncodingType,
    pub had_bom: bool,
}

#[must_use]
pub fn detect(input: &[u8]) -> DetectionResult {
    if input.is_empty() {
        return DetectionResult {
            encoding: EncodingType::Ascii,
            had_bom: false,
        };
    }

    // 1. BOM detection
    if input.starts_with(encoding_type::BOM_UTF8) {
        return DetectionResult {
            encoding: EncodingType::Utf8Bom,
            had_bom: true,
        };
    }
    if input.starts_with(encoding_type::BOM_UTF16_BE) {
        return DetectionResult {
            encoding: EncodingType::Utf16Be,
            had_bom: true,
        };
    }
    if input.starts_with(encoding_type::BOM_UTF16_LE) {
        return DetectionResult {
            encoding: EncodingType::Utf16Le,
            had_bom: true,
        };
    }

    // 2. ISO-2022-JP escape sequence detection
    if contains_iso2022jp_escape(input) {
        return DetectionResult {
            encoding: EncodingType::Iso2022Jp,
            had_bom: false,
        };
    }

    // 3. Pure ASCII check
    if input.iter().all(|&b| b < 0x80) {
        return DetectionResult {
            encoding: EncodingType::Ascii,
            had_bom: false,
        };
    }

    // 4. chardetng detection
    let mut detector = EncodingDetector::new();
    detector.feed(input, true);
    let enc = detector.guess(Some(b"jp"), true);

    let encoding = EncodingType::from_encoding_rs(enc).unwrap_or(EncodingType::Utf8);

    DetectionResult {
        encoding,
        had_bom: false,
    }
}

fn contains_iso2022jp_escape(input: &[u8]) -> bool {
    let mut i = 0;
    while i + 2 < input.len() {
        if input[i] == 0x1B {
            // ESC $ B, ESC $ @, ESC $ ( D — switch to JIS X 0208/0212
            if i + 2 < input.len() && input[i + 1] == b'$' {
                let c = input[i + 2];
                if c == b'B' || c == b'@' {
                    return true;
                }
                // ESC $ ( D (JIS X 0212)
                if c == b'(' && i + 3 < input.len() && input[i + 3] == b'D' {
                    return true;
                }
            }
            // ESC ( B, ESC ( J — switch back to ASCII/JIS Roman
            if i + 2 < input.len() && input[i + 1] == b'(' {
                let c = input[i + 2];
                if c == b'B' || c == b'J' {
                    return true;
                }
            }
        }
        i += 1;
    }
    false
}

#[cfg(test)]
#[path = "tests/detect_tests.rs"]
mod tests;
