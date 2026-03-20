use std::fs;
use std::io::{self, Read, Write};

use crate::convert;
use crate::detect;
use crate::encoding_type::EncodingType;
use crate::error::NkfError;
use crate::kana::{self, ZenMode};
use crate::line_ending::{self, LineEnding};
use crate::mime::{self, MimeDecodeMode, MimeEncodeMode};

#[derive(Debug, Default)]
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
        return Ok(format!("{name}\n").into_bytes());
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
    let utf8 = if options.preserve_hw_kana {
        utf8
    } else {
        kana::hw_to_fw_katakana(&utf8)
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
#[path = "tests/pipeline_tests.rs"]
mod tests;
