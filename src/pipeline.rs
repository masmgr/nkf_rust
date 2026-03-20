use std::borrow::Cow;
use std::fs;
use std::io::{self, Read, Write};

use crate::convert;
use crate::detect;
use crate::encoding_type::EncodingType;
use crate::error::NkfError;
use crate::fold;
use crate::input_decode;
use crate::kana::{self, HiraganaMode, ZenMode};
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
    pub hiragana_mode: Option<HiraganaMode>,
    pub fold_columns: Option<usize>,
    pub url_input: bool,
    pub cap_input: bool,
    pub numchar_input: bool,
    pub guess_mode: bool,
    pub overwrite: bool,
    pub show_help: bool,
    pub show_version: bool,
    pub files: Vec<String>,
}

/// Process input bytes according to the given options.
pub fn process(input: &[u8], options: &NkfOptions) -> Result<Vec<u8>, NkfError> {
    // Step 0: Input decoding (URL, cap) — use Cow to avoid allocation when not needed
    let input: Cow<'_, [u8]> = if options.url_input {
        input_decode::decode_url_input_cow(input)
    } else {
        Cow::Borrowed(input)
    };
    let input: Cow<'_, [u8]> = if options.cap_input {
        input_decode::decode_cap_input_cow(&input)
    } else {
        input
    };

    // Step 1: MIME decode (if enabled)
    let input: Cow<'_, [u8]> = if let Some(mime_mode) = options.mime_decode {
        mime::mime_decode_cow(&input, mime_mode)
    } else {
        input
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

    // Step 5: Decode to UTF-8 — Cow avoids allocation for UTF-8 input
    let utf8: Cow<'_, str> = convert::decode_to_utf8_cow(&input, input_encoding)?;

    // Step 5.5: Numeric character reference decoding (operates on UTF-8 text)
    let utf8: Cow<'_, str> = if options.numchar_input {
        match input_decode::decode_numchar_input_cow(&utf8) {
            Cow::Borrowed(_) => utf8,
            Cow::Owned(s) => Cow::Owned(s),
        }
    } else {
        utf8
    };

    // Step 6: Apply kana/zen conversions on UTF-8
    let utf8: Cow<'_, str> = if let Some(zen_mode) = options.zen_mode {
        match kana::apply_zen_conversion_cow(&utf8, zen_mode) {
            Cow::Borrowed(_) => utf8,
            Cow::Owned(s) => Cow::Owned(s),
        }
    } else {
        utf8
    };

    // Step 6.5: Apply hiragana/katakana conversion
    let utf8: Cow<'_, str> = if let Some(mode) = options.hiragana_mode {
        match kana::apply_hiragana_conversion_cow(&utf8, mode) {
            Cow::Borrowed(_) => utf8,
            Cow::Owned(s) => Cow::Owned(s),
        }
    } else {
        utf8
    };

    // Step 7: Convert half-width kana to full-width (unless -x is set)
    let utf8: Cow<'_, str> = if options.preserve_hw_kana {
        utf8
    } else {
        match kana::hw_to_fw_katakana_cow(&utf8) {
            Cow::Borrowed(_) => utf8,
            Cow::Owned(s) => Cow::Owned(s),
        }
    };

    // Step 8: Convert line endings (if specified)
    let utf8: Cow<'_, str> = if let Some(le) = options.line_ending {
        match line_ending::convert_line_endings_cow(&utf8, le) {
            Cow::Borrowed(_) => utf8,
            Cow::Owned(s) => Cow::Owned(s),
        }
    } else {
        utf8
    };

    // Step 8.5: Fold lines (if specified)
    let utf8: Cow<'_, str> = if let Some(cols) = options.fold_columns {
        Cow::Owned(fold::fold_lines(&utf8, cols))
    } else {
        utf8
    };

    // Step 9: MIME encode (if enabled)
    let utf8: Cow<'_, str> = if let Some(mime_mode) = options.mime_encode {
        let charset = output_encoding.display_name();
        Cow::Owned(mime::mime_encode(&utf8, mime_mode, charset))
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
