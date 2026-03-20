use std::borrow::Cow;

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
    decode_to_utf8_cow(input, from).map(Cow::into_owned)
}

/// Decode bytes to UTF-8 string, returning Cow to avoid allocation when input is already valid UTF-8.
pub fn decode_to_utf8_cow(input: &[u8], from: EncodingType) -> Result<Cow<'_, str>, NkfError> {
    let input = strip_bom(input, from);

    match from {
        EncodingType::Ascii | EncodingType::Utf8 | EncodingType::Utf8Bom => {
            std::str::from_utf8(input)
                .map(Cow::Borrowed)
                .map_err(|e| NkfError::Conversion(format!("Invalid UTF-8: {e}")))
        }
        EncodingType::Utf32Be => decode_utf32(input, true).map(Cow::Owned),
        EncodingType::Utf32Le => decode_utf32(input, false).map(Cow::Owned),
        _ => {
            let encoding = from.to_encoding_rs();
            // encoding_rs replaces unmappable chars with U+FFFD; we accept that behavior
            let (result, _, _) = encoding.decode(input);
            Ok(result)
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
        EncodingType::Utf32Be => Ok(encode_utf32(input, true)),
        EncodingType::Utf32Le => Ok(encode_utf32(input, false)),
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
    let mut result = Vec::with_capacity(input.len() * 2);
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

fn decode_utf32(input: &[u8], big_endian: bool) -> Result<String, NkfError> {
    if !input.len().is_multiple_of(4) {
        return Err(NkfError::Conversion(
            "Invalid UTF-32 input length".to_string(),
        ));
    }
    let mut result = String::with_capacity(input.len());
    for chunk in input.chunks(4) {
        let cp = if big_endian {
            u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]])
        } else {
            u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]])
        };
        let c = char::from_u32(cp).ok_or_else(|| {
            NkfError::Conversion(format!("Invalid UTF-32 code point: {cp:#X}"))
        })?;
        result.push(c);
    }
    Ok(result)
}

fn encode_utf32(input: &str, big_endian: bool) -> Vec<u8> {
    let mut result = Vec::with_capacity(input.len() * 2);
    for c in input.chars() {
        let cp = c as u32;
        let bytes = if big_endian {
            cp.to_be_bytes()
        } else {
            cp.to_le_bytes()
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
        EncodingType::Utf32Be => {
            input
                .strip_prefix(encoding_type::BOM_UTF32_BE)
                .unwrap_or(input)
        }
        EncodingType::Utf32Le => {
            input
                .strip_prefix(encoding_type::BOM_UTF32_LE)
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
