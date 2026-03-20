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

#[test]
fn test_line_ending_shortcuts() {
    let opts = parse(&["-d"]).unwrap();
    assert_eq!(opts.line_ending, Some(LineEnding::Lf));

    let opts = parse(&["-c"]).unwrap();
    assert_eq!(opts.line_ending, Some(LineEnding::CrLf));
}

#[test]
fn test_system_presets() {
    let opts = parse(&["--jis"]).unwrap();
    assert_eq!(opts.output_encoding, Some(EncodingType::Iso2022Jp));

    let opts = parse(&["--euc"]).unwrap();
    assert_eq!(opts.output_encoding, Some(EncodingType::EucJp));

    let opts = parse(&["--sjis"]).unwrap();
    assert_eq!(opts.output_encoding, Some(EncodingType::ShiftJis));

    let opts = parse(&["--unix"]).unwrap();
    assert_eq!(opts.line_ending, Some(LineEnding::Lf));

    let opts = parse(&["--mac"]).unwrap();
    assert_eq!(opts.line_ending, Some(LineEnding::Cr));

    let opts = parse(&["--windows"]).unwrap();
    assert_eq!(opts.line_ending, Some(LineEnding::CrLf));

    let opts = parse(&["--msdos"]).unwrap();
    assert_eq!(opts.line_ending, Some(LineEnding::CrLf));
}

#[test]
fn test_ic_oc_options() {
    let opts = parse(&["--ic=Shift_JIS"]).unwrap();
    assert_eq!(opts.input_encoding, Some(EncodingType::ShiftJis));

    let opts = parse(&["--oc=EUC-JP"]).unwrap();
    assert_eq!(opts.output_encoding, Some(EncodingType::EucJp));

    let opts = parse(&["--ic=UTF-8", "--oc=ISO-2022-JP"]).unwrap();
    assert_eq!(opts.input_encoding, Some(EncodingType::Utf8));
    assert_eq!(opts.output_encoding, Some(EncodingType::Iso2022Jp));
}

#[test]
fn test_ic_oc_aliases() {
    let opts = parse(&["--ic=SJIS"]).unwrap();
    assert_eq!(opts.input_encoding, Some(EncodingType::ShiftJis));

    let opts = parse(&["--ic=CP932"]).unwrap();
    assert_eq!(opts.input_encoding, Some(EncodingType::ShiftJis));

    let opts = parse(&["--oc=JIS"]).unwrap();
    assert_eq!(opts.output_encoding, Some(EncodingType::Iso2022Jp));
}

#[test]
fn test_hiragana_flags() {
    use crate::kana::HiraganaMode;

    let opts = parse(&["-h1"]).unwrap();
    assert_eq!(opts.hiragana_mode, Some(HiraganaMode::KatakanaToHiragana));

    let opts = parse(&["-h2"]).unwrap();
    assert_eq!(opts.hiragana_mode, Some(HiraganaMode::HiraganaToKatakana));

    let opts = parse(&["-h3"]).unwrap();
    assert_eq!(opts.hiragana_mode, Some(HiraganaMode::Toggle));

    let opts = parse(&["--hiragana"]).unwrap();
    assert_eq!(opts.hiragana_mode, Some(HiraganaMode::KatakanaToHiragana));

    let opts = parse(&["--katakana"]).unwrap();
    assert_eq!(opts.hiragana_mode, Some(HiraganaMode::HiraganaToKatakana));
}

#[test]
fn test_fold_flags() {
    let opts = parse(&["-f"]).unwrap();
    assert_eq!(opts.fold_columns, Some(60));

    let opts = parse(&["-f40"]).unwrap();
    assert_eq!(opts.fold_columns, Some(40));

    let opts = parse(&["-F"]).unwrap();
    assert_eq!(opts.fold_columns, Some(80));
}

#[test]
fn test_utf32_flags() {
    let opts = parse(&["-w32"]).unwrap();
    assert_eq!(opts.output_encoding, Some(EncodingType::Utf32Be));

    let opts = parse(&["-w32B"]).unwrap();
    assert_eq!(opts.output_encoding, Some(EncodingType::Utf32Be));

    let opts = parse(&["-w32L"]).unwrap();
    assert_eq!(opts.output_encoding, Some(EncodingType::Utf32Le));

    let opts = parse(&["-W32"]).unwrap();
    assert_eq!(opts.input_encoding, Some(EncodingType::Utf32Be));

    let opts = parse(&["-W32L"]).unwrap();
    assert_eq!(opts.input_encoding, Some(EncodingType::Utf32Le));
}

#[test]
fn test_input_decode_flags() {
    let opts = parse(&["--url-input"]).unwrap();
    assert!(opts.url_input);

    let opts = parse(&["--cap-input"]).unwrap();
    assert!(opts.cap_input);

    let opts = parse(&["--numchar-input"]).unwrap();
    assert!(opts.numchar_input);
}
