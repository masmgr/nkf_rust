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
