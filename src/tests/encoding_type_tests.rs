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
}
