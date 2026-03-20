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

#[test]
fn test_mime_decode_base64_flag() {
    use crate::mime::MimeDecodeMode;
    let opts = parse(&["-mB"]).unwrap();
    assert_eq!(opts.mime_decode, Some(MimeDecodeMode::Base64));
}

#[test]
fn test_mime_decode_qp_flag() {
    use crate::mime::MimeDecodeMode;
    let opts = parse(&["-mQ"]).unwrap();
    assert_eq!(opts.mime_decode, Some(MimeDecodeMode::QuotedPrintable));
}

#[test]
fn test_mime_decode_none_flag() {
    use crate::mime::MimeDecodeMode;
    let opts = parse(&["-m0"]).unwrap();
    assert_eq!(opts.mime_decode, Some(MimeDecodeMode::None));
}

#[test]
fn test_mime_encode_base64_flag() {
    use crate::mime::MimeEncodeMode;
    let opts = parse(&["-MB"]).unwrap();
    assert_eq!(opts.mime_encode, Some(MimeEncodeMode::Base64));
}

#[test]
fn test_mime_encode_qp_flag() {
    use crate::mime::MimeEncodeMode;
    let opts = parse(&["-MQ"]).unwrap();
    assert_eq!(opts.mime_encode, Some(MimeEncodeMode::QuotedPrintable));
}

#[test]
fn test_zen_flags() {
    let opts = parse(&["-Z0"]).unwrap();
    assert_eq!(opts.zen_mode, Some(ZenMode::AlphaToAscii));

    let opts = parse(&["-Z"]).unwrap();
    assert_eq!(opts.zen_mode, Some(ZenMode::SpaceToOne));

    let opts = parse(&["-Z2"]).unwrap();
    assert_eq!(opts.zen_mode, Some(ZenMode::SpaceToTwo));

    let opts = parse(&["-Z3"]).unwrap();
    assert_eq!(opts.zen_mode, Some(ZenMode::HtmlEntity));

    let opts = parse(&["-Z4"]).unwrap();
    assert_eq!(opts.zen_mode, Some(ZenMode::KatakanaToHw));
}

#[test]
fn test_utf_input_flags() {
    let opts = parse(&["-W"]).unwrap();
    assert_eq!(opts.input_encoding, Some(EncodingType::Utf8));

    let opts = parse(&["-W16B"]).unwrap();
    assert_eq!(opts.input_encoding, Some(EncodingType::Utf16Be));

    let opts = parse(&["-W16L"]).unwrap();
    assert_eq!(opts.input_encoding, Some(EncodingType::Utf16Le));
}

#[test]
fn test_preserve_hw_kana_flag() {
    let opts = parse(&["-x"]).unwrap();
    assert!(opts.preserve_hw_kana);
}

#[test]
fn test_line_ending_cr() {
    let opts = parse(&["-Lm"]).unwrap();
    assert_eq!(opts.line_ending, Some(LineEnding::Cr));
}

#[test]
fn test_unknown_flag_error() {
    assert!(parse(&["-Q"]).is_err());
    assert!(parse(&["--invalid"]).is_err());
}
