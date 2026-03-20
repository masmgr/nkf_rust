use crate::encoding_type::EncodingType;
use crate::error::NkfError;

/// Convert a byte buffer from one encoding to another via UTF-8 intermediate.
pub fn convert(input: &[u8], from: EncodingType, to: EncodingType) -> Result<Vec<u8>, NkfError> {
    // Strip BOM if present
    let input = strip_bom(input, from);

    // Same encoding: return as-is (unless BOM was stripped)
    if from == to {
        return Ok(input.to_vec());
    }

    // Step 1: Decode to UTF-8
    let utf8 = decode_to_utf8(input, from)?;

    // Step 2: Encode from UTF-8 to target
    encode_from_utf8(&utf8, to)
}

/// Decode bytes to UTF-8 string.
pub fn decode_to_utf8(input: &[u8], from: EncodingType) -> Result<String, NkfError> {
    let input = strip_bom(input, from);

    match from {
        EncodingType::Ascii | EncodingType::Utf8 | EncodingType::Utf8Bom => {
            String::from_utf8(input.to_vec()).map_err(|e| {
                NkfError::Conversion(format!("Invalid UTF-8: {}", e))
            })
        }
        _ => {
            let encoding = from.to_encoding_rs();
            let (result, _, had_errors) = encoding.decode(input);
            if had_errors {
                // encoding_rs replaces unmappable chars with U+FFFD, not a hard error
                // We still return the result but could warn
            }
            let _ = had_errors;
            Ok(result.into_owned())
        }
    }
}

/// Encode a UTF-8 string to target encoding bytes.
pub fn encode_from_utf8(input: &str, to: EncodingType) -> Result<Vec<u8>, NkfError> {
    match to {
        EncodingType::Ascii | EncodingType::Utf8 => Ok(input.as_bytes().to_vec()),
        EncodingType::Utf8Bom => {
            let mut result = vec![0xEF, 0xBB, 0xBF];
            result.extend_from_slice(input.as_bytes());
            Ok(result)
        }
        EncodingType::Utf16Be => {
            let mut result = Vec::new();
            for c in input.encode_utf16() {
                result.extend_from_slice(&c.to_be_bytes());
            }
            Ok(result)
        }
        EncodingType::Utf16Le => {
            let mut result = Vec::new();
            for c in input.encode_utf16() {
                result.extend_from_slice(&c.to_le_bytes());
            }
            Ok(result)
        }
        _ => {
            let encoding = to.to_encoding_rs();
            let (result, _, had_errors) = encoding.encode(input);
            if had_errors {
                return Err(NkfError::Conversion(format!(
                    "Some characters cannot be represented in {}",
                    to.display_name()
                )));
            }
            Ok(result.into_owned())
        }
    }
}

fn strip_bom(input: &[u8], encoding: EncodingType) -> &[u8] {
    match encoding {
        EncodingType::Utf8Bom | EncodingType::Utf8 => {
            if input.len() >= 3 && input[0] == 0xEF && input[1] == 0xBB && input[2] == 0xBF {
                &input[3..]
            } else {
                input
            }
        }
        EncodingType::Utf16Be => {
            if input.len() >= 2 && input[0] == 0xFE && input[1] == 0xFF {
                &input[2..]
            } else {
                input
            }
        }
        EncodingType::Utf16Le => {
            if input.len() >= 2 && input[0] == 0xFF && input[1] == 0xFE {
                &input[2..]
            } else {
                input
            }
        }
        _ => input,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_utf8_to_shift_jis() {
        let input = "日本語".as_bytes();
        let result = convert(input, EncodingType::Utf8, EncodingType::ShiftJis).unwrap();
        // "日本語" in Shift_JIS
        assert_eq!(result, vec![0x93, 0xFA, 0x96, 0x7B, 0x8C, 0xEA]);
    }

    #[test]
    fn test_shift_jis_to_utf8() {
        let input: &[u8] = &[0x93, 0xFA, 0x96, 0x7B, 0x8C, 0xEA];
        let result = convert(input, EncodingType::ShiftJis, EncodingType::Utf8).unwrap();
        assert_eq!(String::from_utf8(result).unwrap(), "日本語");
    }

    #[test]
    fn test_utf8_to_euc_jp() {
        let input = "日本語".as_bytes();
        let result = convert(input, EncodingType::Utf8, EncodingType::EucJp).unwrap();
        assert_eq!(result, vec![0xC6, 0xFC, 0xCB, 0xDC, 0xB8, 0xEC]);
    }

    #[test]
    fn test_euc_jp_to_utf8() {
        let input: &[u8] = &[0xC6, 0xFC, 0xCB, 0xDC, 0xB8, 0xEC];
        let result = convert(input, EncodingType::EucJp, EncodingType::Utf8).unwrap();
        assert_eq!(String::from_utf8(result).unwrap(), "日本語");
    }

    #[test]
    fn test_shift_jis_to_euc_jp() {
        let sjis: &[u8] = &[0x93, 0xFA, 0x96, 0x7B, 0x8C, 0xEA];
        let result = convert(sjis, EncodingType::ShiftJis, EncodingType::EucJp).unwrap();
        assert_eq!(result, vec![0xC6, 0xFC, 0xCB, 0xDC, 0xB8, 0xEC]);
    }

    #[test]
    fn test_utf8_to_iso2022jp() {
        let input = "日本語".as_bytes();
        let result = convert(input, EncodingType::Utf8, EncodingType::Iso2022Jp).unwrap();
        // Should contain ESC sequences
        assert!(result.contains(&0x1B));
    }

    #[test]
    fn test_utf8_to_utf16be() {
        let input = "AB".as_bytes();
        let result = convert(input, EncodingType::Utf8, EncodingType::Utf16Be).unwrap();
        assert_eq!(result, vec![0x00, 0x41, 0x00, 0x42]);
    }

    #[test]
    fn test_utf8_to_utf16le() {
        let input = "AB".as_bytes();
        let result = convert(input, EncodingType::Utf8, EncodingType::Utf16Le).unwrap();
        assert_eq!(result, vec![0x41, 0x00, 0x42, 0x00]);
    }

    #[test]
    fn test_strip_utf8_bom() {
        let input = b"\xEF\xBB\xBFHello";
        let result = convert(input, EncodingType::Utf8Bom, EncodingType::Utf8).unwrap();
        assert_eq!(result, b"Hello");
    }

    #[test]
    fn test_same_encoding() {
        let input = "テスト".as_bytes();
        let result = convert(input, EncodingType::Utf8, EncodingType::Utf8).unwrap();
        assert_eq!(result, input);
    }
}
