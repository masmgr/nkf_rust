use crate::encoding_type::{self, EncodingType};
use crate::error::NkfError;

/// Convert a byte buffer from one encoding to another via UTF-8 intermediate.
pub fn convert(input: &[u8], from: EncodingType, to: EncodingType) -> Result<Vec<u8>, NkfError> {
    // decode_to_utf8 handles BOM stripping internally
    let utf8 = decode_to_utf8(input, from)?;
    encode_from_utf8(&utf8, to)
}

/// Decode bytes to UTF-8 string.
pub fn decode_to_utf8(input: &[u8], from: EncodingType) -> Result<String, NkfError> {
    let input = strip_bom(input, from);

    match from {
        EncodingType::Ascii | EncodingType::Utf8 | EncodingType::Utf8Bom => {
            String::from_utf8(input.to_vec())
                .map_err(|e| NkfError::Conversion(format!("Invalid UTF-8: {e}")))
        }
        _ => {
            let encoding = from.to_encoding_rs();
            // encoding_rs replaces unmappable chars with U+FFFD; we accept that behavior
            let (result, _, _) = encoding.decode(input);
            Ok(result.into_owned())
        }
    }
}

/// Encode a UTF-8 string to target encoding bytes.
pub fn encode_from_utf8(input: &str, to: EncodingType) -> Result<Vec<u8>, NkfError> {
    match to {
        EncodingType::Ascii | EncodingType::Utf8 => Ok(input.as_bytes().to_vec()),
        EncodingType::Utf8Bom => {
            let mut result = encoding_type::BOM_UTF8.to_vec();
            result.extend_from_slice(input.as_bytes());
            Ok(result)
        }
        EncodingType::Utf16Be => Ok(encode_utf16(input, true)),
        EncodingType::Utf16Le => Ok(encode_utf16(input, false)),
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

fn encode_utf16(input: &str, big_endian: bool) -> Vec<u8> {
    let mut result = Vec::new();
    for c in input.encode_utf16() {
        let bytes = if big_endian {
            c.to_be_bytes()
        } else {
            c.to_le_bytes()
        };
        result.extend_from_slice(&bytes);
    }
    result
}

fn strip_bom(input: &[u8], encoding: EncodingType) -> &[u8] {
    match encoding {
        EncodingType::Utf8Bom | EncodingType::Utf8 => {
            input
                .strip_prefix(encoding_type::BOM_UTF8)
                .unwrap_or(input)
        }
        EncodingType::Utf16Be => {
            input
                .strip_prefix(encoding_type::BOM_UTF16_BE)
                .unwrap_or(input)
        }
        EncodingType::Utf16Le => {
            input
                .strip_prefix(encoding_type::BOM_UTF16_LE)
                .unwrap_or(input)
        }
        _ => input,
    }
}

#[cfg(test)]
#[path = "tests/convert_tests.rs"]
mod tests;
