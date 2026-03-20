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
            if let Some(enc_name) = arg.strip_prefix("--ic=") {
                options.input_encoding = Some(
                    EncodingType::from_name(enc_name)
                        .ok_or_else(|| NkfError::InvalidArgs(format!("Unknown encoding: {enc_name}")))?,
                );
            } else if let Some(enc_name) = arg.strip_prefix("--oc=") {
                options.output_encoding = Some(
                    EncodingType::from_name(enc_name)
                        .ok_or_else(|| NkfError::InvalidArgs(format!("Unknown encoding: {enc_name}")))?,
                );
            } else {
                match arg.as_str() {
                    "--guess" => options.guess_mode = true,
                    "--overwrite" => options.overwrite = true,
                    "--help" => options.show_help = true,
                    "--version" => options.show_version = true,
                    "--jis" => options.output_encoding = Some(EncodingType::Iso2022Jp),
                    "--euc" => options.output_encoding = Some(EncodingType::EucJp),
                    "--sjis" => options.output_encoding = Some(EncodingType::ShiftJis),
                    "--unix" => options.line_ending = Some(LineEnding::Lf),
                    "--mac" => options.line_ending = Some(LineEnding::Cr),
                    "--msdos" | "--windows" => options.line_ending = Some(LineEnding::CrLf),
                    "--hiragana" => options.hiragana_mode = Some(crate::kana::HiraganaMode::KatakanaToHiragana),
                    "--katakana" => options.hiragana_mode = Some(crate::kana::HiraganaMode::HiraganaToKatakana),
                    "--url-input" => options.url_input = true,
                    "--cap-input" => options.cap_input = true,
                    "--numchar-input" => options.numchar_input = true,
                    _ => return Err(NkfError::InvalidArgs(format!("Unknown option: {arg}"))),
                }
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
                    options.output_encoding = Some(parse_utf_variant(rest));
                    j = bytes.len(); // consumed rest
                    continue;
                }

                // Input encoding
                b'J' => options.input_encoding = Some(EncodingType::Iso2022Jp),
                b'S' => options.input_encoding = Some(EncodingType::ShiftJis),
                b'E' => options.input_encoding = Some(EncodingType::EucJp),
                b'W' => {
                    let rest = &arg[j + 1..];
                    options.input_encoding = Some(parse_utf_variant(rest));
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
                    let mode_char = consume_mode_char(bytes, &mut j);
                    options.mime_decode =
                        Some(crate::mime::parse_mime_decode_mode(&mode_char)?);
                }

                // MIME encode: -M[BQ]
                b'M' => {
                    let mode_char = consume_mode_char(bytes, &mut j);
                    options.mime_encode =
                        Some(crate::mime::parse_mime_encode_mode(&mode_char)?);
                }

                // Line ending shortcuts
                b'd' => options.line_ending = Some(LineEnding::Lf),
                b'c' => options.line_ending = Some(LineEnding::CrLf),

                // Hiragana/Katakana conversion: -h1, -h2, -h3
                b'h' => {
                    if j + 1 < bytes.len() {
                        j += 1;
                        match bytes[j] {
                            b'1' => options.hiragana_mode = Some(crate::kana::HiraganaMode::KatakanaToHiragana),
                            b'2' => options.hiragana_mode = Some(crate::kana::HiraganaMode::HiraganaToKatakana),
                            b'3' => options.hiragana_mode = Some(crate::kana::HiraganaMode::Toggle),
                            _ => {
                                return Err(NkfError::InvalidArgs(format!(
                                    "Unknown hiragana mode: -h{}",
                                    bytes[j] as char
                                )));
                            }
                        }
                    } else {
                        return Err(NkfError::InvalidArgs(
                            "-h requires a mode (1, 2, or 3)".to_string(),
                        ));
                    }
                }

                // Line folding: -f[cols]
                b'f' => {
                    let rest = &arg[j + 1..];
                    let cols = if rest.is_empty() {
                        60
                    } else {
                        rest.parse::<usize>().map_err(|_| {
                            NkfError::InvalidArgs(format!("Invalid fold width: {rest}"))
                        })?
                    };
                    options.fold_columns = Some(cols);
                    j = bytes.len();
                    continue;
                }

                // Line folding with default width
                b'F' => {
                    options.fold_columns = Some(80);
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

fn consume_mode_char(bytes: &[u8], j: &mut usize) -> String {
    if *j + 1 < bytes.len() {
        *j += 1;
        std::str::from_utf8(&bytes[*j..=*j])
            .unwrap_or("")
            .to_string()
    } else {
        String::new()
    }
}

fn parse_utf_variant(rest: &str) -> EncodingType {
    match rest {
        "16B" | "16B0" | "16" => EncodingType::Utf16Be, // default UTF-16 is BE
        "16L" | "16L0" => EncodingType::Utf16Le,
        "32B" | "32B0" | "32" => EncodingType::Utf32Be, // default UTF-32 is BE
        "32L" | "32L0" => EncodingType::Utf32Le,
        _ => EncodingType::Utf8,
    }
}

pub fn print_help() {
    println!(
        "nkf - Network Kanji Filter (Rust implementation)

Usage: nkf [options] [file ...]

Output encoding:
  -j            ISO-2022-JP (JIS)
  -s            Shift_JIS
  -e            EUC-JP
  -w            UTF-8 (default)
  -w16B         UTF-16BE
  -w16L         UTF-16LE
  -w32B         UTF-32BE
  -w32L         UTF-32LE
  --oc=NAME     Specify output encoding by name

Input encoding:
  -J            ISO-2022-JP
  -S            Shift_JIS
  -E            EUC-JP
  -W            UTF-8
  --ic=NAME     Specify input encoding by name

Encoding presets:
  --jis         Output as ISO-2022-JP
  --euc         Output as EUC-JP
  --sjis        Output as Shift_JIS

Detection:
  -g, --guess   Detect and print encoding

Line endings:
  -Lu, -d, --unix      Unix (LF)
  -Lw, -c, --windows   Windows (CRLF)
  -Lm, --mac           Mac (CR)

MIME:
  -mB       MIME Base64 decode
  -mQ       MIME Quoted-Printable decode
  -m0       No MIME decode
  -MB       MIME Base64 encode
  -MQ       MIME Quoted-Printable encode

Kana:
  -x            Preserve half-width kana
  -Z[0-4]       Full-width conversion (Z3=HTML entities)
  -h1           Katakana to Hiragana
  -h2           Hiragana to Katakana
  -h3           Katakana/Hiragana toggle
  --hiragana    Same as -h1
  --katakana    Same as -h2

Text processing:
  -f[cols]      Fold lines at specified width (default: 60)
  -F            Fold lines at 80 columns

Input decoding:
  --url-input       Decode URL-encoded (%xx) input
  --cap-input       Decode hex-encoded (:xx) input
  --numchar-input   Decode numeric character references

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
