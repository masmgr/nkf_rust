use std::fs;
use std::io::{self, Read, Write};

use crate::convert;
use crate::detect;
use crate::encoding_type::EncodingType;
use crate::error::NkfError;
use crate::kana::{self, ZenMode};
use crate::line_ending::{self, LineEnding};
use crate::mime::{self, MimeDecodeMode, MimeEncodeMode};

#[derive(Debug)]
pub struct NkfOptions {
    pub input_encoding: Option<EncodingType>,
    pub output_encoding: Option<EncodingType>,
    pub line_ending: Option<LineEnding>,
    pub mime_decode: Option<MimeDecodeMode>,
    pub mime_encode: Option<MimeEncodeMode>,
    pub zen_mode: Option<ZenMode>,
    pub preserve_hw_kana: bool,
    pub guess_mode: bool,
    pub overwrite: bool,
    pub show_help: bool,
    pub show_version: bool,
    pub files: Vec<String>,
}

impl Default for NkfOptions {
    fn default() -> Self {
        NkfOptions {
            input_encoding: None,
            output_encoding: None,
            line_ending: None,
            mime_decode: None,
            mime_encode: None,
            zen_mode: None,
            preserve_hw_kana: false,
            guess_mode: false,
            overwrite: false,
            show_help: false,
            show_version: false,
            files: Vec::new(),
        }
    }
}

/// Process input bytes according to the given options.
pub fn process(input: &[u8], options: &NkfOptions) -> Result<Vec<u8>, NkfError> {
    // Step 1: MIME decode (if enabled)
    let input = if let Some(mime_mode) = options.mime_decode {
        mime::mime_decode(input, mime_mode)
    } else {
        input.to_vec()
    };

    // Step 2: Detect input encoding (if not specified)
    let detection = detect::detect(&input);
    let input_encoding = options.input_encoding.unwrap_or(detection.encoding);

    // Step 3: If guess mode, just return the encoding name
    if options.guess_mode {
        let name = input_encoding.display_name();
        return Ok(format!("{}\n", name).into_bytes());
    }

    // Step 4: Determine output encoding (default: UTF-8)
    let output_encoding = options.output_encoding.unwrap_or(EncodingType::Utf8);

    // Step 5: Decode to UTF-8
    let utf8 = convert::decode_to_utf8(&input, input_encoding)?;

    // Step 6: Apply kana/zen conversions on UTF-8
    let utf8 = if let Some(zen_mode) = options.zen_mode {
        kana::apply_zen_conversion(&utf8, zen_mode)
    } else {
        utf8
    };

    // Step 7: Convert half-width kana to full-width (unless -x is set)
    let utf8 = if !options.preserve_hw_kana {
        kana::hw_to_fw_katakana(&utf8)
    } else {
        utf8
    };

    // Step 8: Convert line endings (if specified)
    let utf8 = if let Some(le) = options.line_ending {
        line_ending::convert_line_endings(&utf8, le)
    } else {
        utf8
    };

    // Step 9: MIME encode (if enabled)
    let utf8 = if let Some(mime_mode) = options.mime_encode {
        let charset = output_encoding.display_name();
        mime::mime_encode(&utf8, mime_mode, charset)
    } else {
        utf8
    };

    // Step 10: Encode to target encoding
    convert::encode_from_utf8(&utf8, output_encoding)
}

/// Process a single file.
pub fn process_file(path: &str, options: &NkfOptions) -> Result<(), NkfError> {
    let input = fs::read(path)?;
    let output = process(&input, options)?;

    if options.overwrite {
        fs::write(path, &output)?;
    } else {
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        handle.write_all(&output)?;
    }

    Ok(())
}

/// Process stdin to stdout.
pub fn process_stdin(options: &NkfOptions) -> Result<(), NkfError> {
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input)?;

    let output = process(&input, options)?;

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    handle.write_all(&output)?;

    Ok(())
}

/// Main entry point: process files or stdin based on options.
pub fn run(options: &NkfOptions) -> Result<(), NkfError> {
    if options.files.is_empty() {
        process_stdin(options)
    } else {
        for file in &options.files {
            process_file(file, options)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_utf8_to_sjis() {
        let input = "日本語".as_bytes();
        let options = NkfOptions {
            output_encoding: Some(EncodingType::ShiftJis),
            ..Default::default()
        };
        let result = process(input, &options).unwrap();
        assert_eq!(result, vec![0x93, 0xFA, 0x96, 0x7B, 0x8C, 0xEA]);
    }

    #[test]
    fn test_process_guess_mode() {
        let input = "日本語".as_bytes();
        let options = NkfOptions {
            guess_mode: true,
            ..Default::default()
        };
        let result = process(input, &options).unwrap();
        assert_eq!(String::from_utf8(result).unwrap(), "UTF-8\n");
    }

    #[test]
    fn test_process_guess_sjis() {
        let sjis: &[u8] = &[0x93, 0xFA, 0x96, 0x7B, 0x8C, 0xEA];
        let options = NkfOptions {
            guess_mode: true,
            ..Default::default()
        };
        let result = process(sjis, &options).unwrap();
        assert_eq!(String::from_utf8(result).unwrap(), "Shift_JIS\n");
    }

    #[test]
    fn test_process_with_line_ending() {
        let input = "a\nb\nc".as_bytes();
        let options = NkfOptions {
            line_ending: Some(LineEnding::CrLf),
            ..Default::default()
        };
        let result = process(input, &options).unwrap();
        assert_eq!(String::from_utf8(result).unwrap(), "a\r\nb\r\nc");
    }

    #[test]
    fn test_process_sjis_to_eucjp() {
        let sjis: &[u8] = &[0x93, 0xFA, 0x96, 0x7B, 0x8C, 0xEA];
        let options = NkfOptions {
            input_encoding: Some(EncodingType::ShiftJis),
            output_encoding: Some(EncodingType::EucJp),
            ..Default::default()
        };
        let result = process(sjis, &options).unwrap();
        assert_eq!(result, vec![0xC6, 0xFC, 0xCB, 0xDC, 0xB8, 0xEC]);
    }

    #[test]
    fn test_process_auto_detect_and_convert() {
        // Shift_JIS "日本語" -> auto-detect -> output as UTF-8
        let sjis: &[u8] = &[0x93, 0xFA, 0x96, 0x7B, 0x8C, 0xEA];
        let options = NkfOptions {
            output_encoding: Some(EncodingType::Utf8),
            ..Default::default()
        };
        let result = process(sjis, &options).unwrap();
        assert_eq!(String::from_utf8(result).unwrap(), "日本語");
    }
}
