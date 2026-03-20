use super::*;

#[test]
fn test_round_trip() {
    let types = [
        EncodingType::Utf8,
        EncodingType::ShiftJis,
        EncodingType::EucJp,
        EncodingType::Iso2022Jp,
        EncodingType::Utf16Be,
        EncodingType::Utf16Le,
    ];
    for t in &types {
        let enc = t.to_encoding_rs();
        let back = EncodingType::from_encoding_rs(enc).unwrap();
        assert_eq!(*t, back, "round-trip failed for {t:?}");
    }
}

#[test]
fn test_display_name() {
    assert_eq!(EncodingType::ShiftJis.display_name(), "Shift_JIS");
    assert_eq!(EncodingType::Iso2022Jp.display_name(), "ISO-2022-JP");
    assert_eq!(EncodingType::Utf32Be.display_name(), "UTF-32BE");
    assert_eq!(EncodingType::Utf32Le.display_name(), "UTF-32LE");
}

#[test]
fn test_from_name() {
    assert_eq!(EncodingType::from_name("UTF-8"), Some(EncodingType::Utf8));
    assert_eq!(EncodingType::from_name("utf-8"), Some(EncodingType::Utf8));
    assert_eq!(EncodingType::from_name("UTF8"), Some(EncodingType::Utf8));
    assert_eq!(EncodingType::from_name("Shift_JIS"), Some(EncodingType::ShiftJis));
    assert_eq!(EncodingType::from_name("SJIS"), Some(EncodingType::ShiftJis));
    assert_eq!(EncodingType::from_name("CP932"), Some(EncodingType::ShiftJis));
    assert_eq!(EncodingType::from_name("EUC-JP"), Some(EncodingType::EucJp));
    assert_eq!(EncodingType::from_name("EUCJP"), Some(EncodingType::EucJp));
    assert_eq!(EncodingType::from_name("ISO-2022-JP"), Some(EncodingType::Iso2022Jp));
    assert_eq!(EncodingType::from_name("JIS"), Some(EncodingType::Iso2022Jp));
    assert_eq!(EncodingType::from_name("UTF-32BE"), Some(EncodingType::Utf32Be));
    assert_eq!(EncodingType::from_name("UTF-32LE"), Some(EncodingType::Utf32Le));
    assert_eq!(EncodingType::from_name("unknown"), None);
}
