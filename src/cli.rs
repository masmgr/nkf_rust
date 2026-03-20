use crate::encoding_type::EncodingType;
use crate::error::NkfError;
use crate::kana::ZenMode;
use crate::line_ending::LineEnding;
use crate::pipeline::NkfOptions;

/// Pre-parse argv to normalize nkf-style compound flags.
/// Returns normalized args suitable for standard parsing.
pub fn parse_args(args: Vec<String>) -> Result<NkfOptions, NkfError> {
    let mut options = NkfOptions::default();
    let mut i = 0;
    let args: Vec<String> = args.into_iter().skip(1).collect(); // skip program name

    while i < args.len() {
        let arg = &args[i];

        if !arg.starts_with('-') || arg == "-" {
            // File argument or stdin marker
            if arg == "-" {
                // stdin explicitly
            } else {
                options.files.push(arg.clone());
            }
            i += 1;
            continue;
        }

        // Handle long options
        if arg.starts_with("--") {
            match arg.as_str() {
                "--guess" => options.guess_mode = true,
                "--overwrite" => options.overwrite = true,
                "--help" => options.show_help = true,
                "--version" => options.show_version = true,
                _ => return Err(NkfError::InvalidArgs(format!("Unknown option: {arg}"))),
            }
            i += 1;
            continue;
        }

        // Handle short options (may be compound)
        let bytes = arg.as_bytes();
        let mut j = 1; // skip leading '-'

        while j < bytes.len() {
            match bytes[j] {
                // Output encoding
                b'j' => options.output_encoding = Some(EncodingType::Iso2022Jp),
                b's' => options.output_encoding = Some(EncodingType::ShiftJis),
                b'e' => options.output_encoding = Some(EncodingType::EucJp),
                b'w' => {
                    // -w, -w8, -w80, -w16B, -w16L, -w16B0, -w16L0
                    let rest = &arg[j + 1..];
                    options.output_encoding = Some(parse_utf_output(rest));
                    j = bytes.len(); // consumed rest
                    continue;
                }

                // Input encoding
                b'J' => options.input_encoding = Some(EncodingType::Iso2022Jp),
                b'S' => options.input_encoding = Some(EncodingType::ShiftJis),
                b'E' => options.input_encoding = Some(EncodingType::EucJp),
                b'W' => {
                    let rest = &arg[j + 1..];
                    options.input_encoding = Some(parse_utf_input(rest));
                    j = bytes.len();
                    continue;
                }

                // Guess
                b'g' => options.guess_mode = true,

                // Line ending: -Lu, -Lw, -Lm
                b'L' => {
                    if j + 1 < bytes.len() {
                        match bytes[j + 1] {
                            b'u' => options.line_ending = Some(LineEnding::Lf),
                            b'w' => options.line_ending = Some(LineEnding::CrLf),
                            b'm' => options.line_ending = Some(LineEnding::Cr),
                            _ => {
                                return Err(NkfError::InvalidArgs(format!(
                                    "Unknown line ending mode: -L{}",
                                    bytes[j + 1] as char
                                )));
                            }
                        }
                        j += 1;
                    } else {
                        return Err(NkfError::InvalidArgs(
                            "-L requires a mode (u, w, or m)".to_string(),
                        ));
                    }
                }

                // MIME decode: -m[BQSN0]
                b'm' => {
                    let mode_char = if j + 1 < bytes.len() {
                        j += 1;
                        std::str::from_utf8(&bytes[j..=j]).unwrap_or("")
                    } else {
                        ""
                    };
                    options.mime_decode = Some(crate::mime::parse_mime_decode_mode(mode_char)?);
                }

                // MIME encode: -M[BQ]
                b'M' => {
                    let mode_char = if j + 1 < bytes.len() {
                        j += 1;
                        std::str::from_utf8(&bytes[j..=j]).unwrap_or("")
                    } else {
                        ""
                    };
                    options.mime_encode = Some(crate::mime::parse_mime_encode_mode(mode_char)?);
                }

                // Half-width kana preservation
                b'x' => options.preserve_hw_kana = true,

                // Zen (full-width to half-width) conversion: -Z[0-4]
                b'Z' => {
                    let mode = if j + 1 < bytes.len() {
                        j += 1;
                        match bytes[j] {
                            b'0' => ZenMode::AlphaToAscii,
                            b'2' => ZenMode::SpaceToTwo,
                            b'3' => ZenMode::HtmlEntity,
                            b'4' => ZenMode::KatakanaToHw,
                            _ => ZenMode::SpaceToOne,
                        }
                    } else {
                        ZenMode::SpaceToOne
                    };
                    options.zen_mode = Some(mode);
                }

                _ => {
                    return Err(NkfError::InvalidArgs(format!(
                        "Unknown option: -{}",
                        bytes[j] as char
                    )));
                }
            }
            j += 1;
        }
        i += 1;
    }

    Ok(options)
}

fn parse_utf_output(rest: &str) -> EncodingType {
    match rest {
        "16B" | "16B0" | "16" => EncodingType::Utf16Be, // default UTF-16 is BE
        "16L" | "16L0" => EncodingType::Utf16Le,
        _ => EncodingType::Utf8,
    }
}

fn parse_utf_input(rest: &str) -> EncodingType {
    match rest {
        "16B" | "16B0" | "16" => EncodingType::Utf16Be,
        "16L" | "16L0" => EncodingType::Utf16Le,
        _ => EncodingType::Utf8,
    }
}

pub fn print_help() {
    println!(
        "nkf - Network Kanji Filter (Rust implementation)

Usage: nkf [options] [file ...]

Output encoding:
  -j        ISO-2022-JP (JIS)
  -s        Shift_JIS
  -e        EUC-JP
  -w        UTF-8 (default)
  -w16B     UTF-16BE
  -w16L     UTF-16LE

Input encoding:
  -J        ISO-2022-JP
  -S        Shift_JIS
  -E        EUC-JP
  -W        UTF-8

Detection:
  -g, --guess   Detect and print encoding

Line endings:
  -Lu       Unix (LF)
  -Lw       Windows (CRLF)
  -Lm       Mac (CR)

MIME:
  -mB       MIME Base64 decode
  -mQ       MIME Quoted-Printable decode
  -m0       No MIME decode
  -MB       MIME Base64 encode
  -MQ       MIME Quoted-Printable encode

Kana:
  -x        Preserve half-width kana
  -Z[0-4]   Full-width conversion

Other:
  --overwrite   Overwrite input files
  --help        Show this help
  --version     Show version"
    );
}

pub fn print_version() {
    println!("nkf_rust {}", env!("CARGO_PKG_VERSION"));
}

#[cfg(test)]
#[path = "tests/cli_tests.rs"]
mod tests;
