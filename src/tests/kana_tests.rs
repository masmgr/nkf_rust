use super::*;

#[test]
fn test_hw_to_fw_basic() {
    // ｱｲｳ -> アイウ
    let input = "\u{FF71}\u{FF72}\u{FF73}";
    let result = hw_to_fw_katakana(input);
    assert_eq!(result, "アイウ");
}

#[test]
fn test_hw_to_fw_dakuten() {
    // ｶﾞ -> ガ
    let input = "\u{FF76}\u{FF9E}";
    let result = hw_to_fw_katakana(input);
    assert_eq!(result, "ガ");
}

#[test]
fn test_hw_to_fw_handakuten() {
    // ﾊﾟ -> パ
    let input = "\u{FF8A}\u{FF9F}";
    let result = hw_to_fw_katakana(input);
    assert_eq!(result, "パ");
}

#[test]
fn test_fw_to_hw_basic() {
    let result = fw_to_hw_katakana("アイウ");
    assert_eq!(result, "\u{FF71}\u{FF72}\u{FF73}");
}

#[test]
fn test_fw_to_hw_dakuten() {
    let result = fw_to_hw_katakana("ガ");
    assert_eq!(result, "\u{FF76}\u{FF9E}");
}

#[test]
fn test_fw_to_hw_handakuten() {
    let result = fw_to_hw_katakana("パ");
    assert_eq!(result, "\u{FF8A}\u{FF9F}");
}

#[test]
fn test_fw_to_hw_ascii() {
    assert_eq!(fw_to_hw_ascii("ＡＢＣ１２３"), "ABC123");
}

#[test]
fn test_fw_space_to_hw() {
    assert_eq!(fw_to_hw_ascii("　"), " ");
}

#[test]
fn test_mixed_content_preserved() {
    let input = "Hello\u{FF71}World";
    let result = hw_to_fw_katakana(input);
    assert_eq!(result, "HelloアWorld");
}

#[test]
fn test_round_trip() {
    let original = "アイウエオカキクケコ";
    let hw = fw_to_hw_katakana(original);
    let fw = hw_to_fw_katakana(&hw);
    assert_eq!(fw, original);
}
