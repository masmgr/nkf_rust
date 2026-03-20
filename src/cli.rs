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
                _ => return Err(NkfError::InvalidArgs(format!("Unknown option: {}", arg))),
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
                        std::str::from_utf8(&bytes[j..j + 1]).unwrap_or("")
                    } else {
                        ""
                    };
                    options.mime_decode = Some(crate::mime::parse_mime_decode_mode(mode_char)?);
                }

                // MIME encode: -M[BQ]
                b'M' => {
                    let mode_char = if j + 1 < bytes.len() {
                        j += 1;
                        std::str::from_utf8(&bytes[j..j + 1]).unwrap_or("")
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
                            b'1' => ZenMode::SpaceToOne,
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
        "" | "8" | "80" => EncodingType::Utf8,
        "16B" | "16B0" => EncodingType::Utf16Be,
        "16L" | "16L0" => EncodingType::Utf16Le,
        "16" => EncodingType::Utf16Be, // default UTF-16 is BE
        _ => EncodingType::Utf8,
    }
}

fn parse_utf_input(rest: &str) -> EncodingType {
    match rest {
        "" | "8" | "80" => EncodingType::Utf8,
        "16B" | "16B0" => EncodingType::Utf16Be,
        "16L" | "16L0" => EncodingType::Utf16Le,
        "16" => EncodingType::Utf16Be,
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
mod tests {
    use super::*;

    fn parse(args: &[&str]) -> Result<NkfOptions, NkfError> {
        let args: Vec<String> = std::iter::once("nkf")
            .chain(args.iter().copied())
            .map(String::from)
            .collect();
        parse_args(args)
    }

    #[test]
    fn test_output_encoding() {
        let opts = parse(&["-s"]).unwrap();
        assert_eq!(opts.output_encoding, Some(EncodingType::ShiftJis));

        let opts = parse(&["-e"]).unwrap();
        assert_eq!(opts.output_encoding, Some(EncodingType::EucJp));

        let opts = parse(&["-j"]).unwrap();
        assert_eq!(opts.output_encoding, Some(EncodingType::Iso2022Jp));

        let opts = parse(&["-w"]).unwrap();
        assert_eq!(opts.output_encoding, Some(EncodingType::Utf8));
    }

    #[test]
    fn test_input_encoding() {
        let opts = parse(&["-S"]).unwrap();
        assert_eq!(opts.input_encoding, Some(EncodingType::ShiftJis));
    }

    #[test]
    fn test_guess_mode() {
        let opts = parse(&["-g"]).unwrap();
        assert!(opts.guess_mode);

        let opts = parse(&["--guess"]).unwrap();
        assert!(opts.guess_mode);
    }

    #[test]
    fn test_line_ending() {
        let opts = parse(&["-Lu"]).unwrap();
        assert_eq!(opts.line_ending, Some(LineEnding::Lf));

        let opts = parse(&["-Lw"]).unwrap();
        assert_eq!(opts.line_ending, Some(LineEnding::CrLf));
    }

    #[test]
    fn test_compound_flags() {
        let opts = parse(&["-sLu"]).unwrap();
        assert_eq!(opts.output_encoding, Some(EncodingType::ShiftJis));
        assert_eq!(opts.line_ending, Some(LineEnding::Lf));
    }

    #[test]
    fn test_file_args() {
        let opts = parse(&["-s", "file1.txt", "file2.txt"]).unwrap();
        assert_eq!(opts.files, vec!["file1.txt", "file2.txt"]);
    }

    #[test]
    fn test_overwrite() {
        let opts = parse(&["--overwrite", "-s"]).unwrap();
        assert!(opts.overwrite);
    }

    #[test]
    fn test_utf16_output() {
        let opts = parse(&["-w16B"]).unwrap();
        assert_eq!(opts.output_encoding, Some(EncodingType::Utf16Be));

        let opts = parse(&["-w16L"]).unwrap();
        assert_eq!(opts.output_encoding, Some(EncodingType::Utf16Le));
    }
}
