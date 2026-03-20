use chardetng::EncodingDetector;

use crate::encoding_type::EncodingType;

#[derive(Debug)]
pub struct DetectionResult {
    pub encoding: EncodingType,
    pub had_bom: bool,
}

pub fn detect(input: &[u8]) -> DetectionResult {
    if input.is_empty() {
        return DetectionResult {
            encoding: EncodingType::Ascii,
            had_bom: false,
        };
    }

    // 1. BOM detection
    if input.len() >= 3 && input[0] == 0xEF && input[1] == 0xBB && input[2] == 0xBF {
        return DetectionResult {
            encoding: EncodingType::Utf8Bom,
            had_bom: true,
        };
    }
    if input.len() >= 2 {
        if input[0] == 0xFE && input[1] == 0xFF {
            return DetectionResult {
                encoding: EncodingType::Utf16Be,
                had_bom: true,
            };
        }
        if input[0] == 0xFF && input[1] == 0xFE {
            return DetectionResult {
                encoding: EncodingType::Utf16Le,
                had_bom: true,
            };
        }
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
mod tests {
    use super::*;

    #[test]
    fn test_detect_ascii() {
        let result = detect(b"Hello, World!");
        assert_eq!(result.encoding, EncodingType::Ascii);
        assert!(!result.had_bom);
    }

    #[test]
    fn test_detect_utf8_bom() {
        let input = b"\xEF\xBB\xBFHello";
        let result = detect(input);
        assert_eq!(result.encoding, EncodingType::Utf8Bom);
        assert!(result.had_bom);
    }

    #[test]
    fn test_detect_utf16be_bom() {
        let input = b"\xFE\xFF\x00H\x00e";
        let result = detect(input);
        assert_eq!(result.encoding, EncodingType::Utf16Be);
        assert!(result.had_bom);
    }

    #[test]
    fn test_detect_utf16le_bom() {
        let input = b"\xFF\xFEH\x00e\x00";
        let result = detect(input);
        assert_eq!(result.encoding, EncodingType::Utf16Le);
        assert!(result.had_bom);
    }

    #[test]
    fn test_detect_iso2022jp() {
        // ESC $ B indicates ISO-2022-JP
        let input = b"\x1B$B$3$s$K$A$O\x1B(B";
        let result = detect(input);
        assert_eq!(result.encoding, EncodingType::Iso2022Jp);
    }

    #[test]
    fn test_detect_empty() {
        let result = detect(b"");
        assert_eq!(result.encoding, EncodingType::Ascii);
    }

    #[test]
    fn test_detect_utf8() {
        // "日本語" in UTF-8
        let input = "日本語テスト".as_bytes();
        let result = detect(input);
        assert_eq!(result.encoding, EncodingType::Utf8);
    }

    #[test]
    fn test_detect_shift_jis() {
        // "日本語" in Shift_JIS
        let input: &[u8] = &[
            0x93, 0xFA, 0x96, 0x7B, 0x8C, 0xEA, 0x83, 0x65, 0x83, 0x58, 0x83, 0x67,
        ];
        let result = detect(input);
        assert_eq!(result.encoding, EncodingType::ShiftJis);
    }

    #[test]
    fn test_detect_euc_jp() {
        // "日本語" in EUC-JP
        let input: &[u8] = &[
            0xC6, 0xFC, 0xCB, 0xDC, 0xB8, 0xEC, 0xA5, 0xC6, 0xA5, 0xB9, 0xA5, 0xC8,
        ];
        let result = detect(input);
        assert_eq!(result.encoding, EncodingType::EucJp);
    }
}
