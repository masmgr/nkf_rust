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

#[test]
fn test_zen_mode_space_to_two() {
    // Full-width space -> two ASCII spaces
    let result = apply_zen_conversion("\u{3000}テスト", ZenMode::SpaceToTwo);
    assert!(result.starts_with("  "));
    assert!(result.contains("テスト"));
}

#[test]
fn test_zen_mode_katakana_to_hw() {
    let result = apply_zen_conversion("アイウ", ZenMode::KatakanaToHw);
    assert_eq!(result, "\u{FF71}\u{FF72}\u{FF73}");
}

#[test]
fn test_dakuten_on_unsupported_char() {
    // ｱ (a) followed by dakuten - ア doesn't support dakuten
    let input = "\u{FF71}\u{FF9E}";
    let result = hw_to_fw_katakana(input);
    // Should produce ア + standalone dakuten ゛
    assert_eq!(result, "ア\u{309B}");
}

#[test]
fn test_hw_to_fw_empty_string() {
    assert_eq!(hw_to_fw_katakana(""), "");
    assert_eq!(fw_to_hw_katakana(""), "");
    assert_eq!(fw_to_hw_ascii(""), "");
}

#[test]
fn test_hiragana_katakana_to_hiragana() {
    assert_eq!(
        apply_hiragana_conversion("アイウエオ", HiraganaMode::KatakanaToHiragana),
        "あいうえお"
    );
}

#[test]
fn test_hiragana_hiragana_to_katakana() {
    assert_eq!(
        apply_hiragana_conversion("あいうえお", HiraganaMode::HiraganaToKatakana),
        "アイウエオ"
    );
}

#[test]
fn test_hiragana_toggle() {
    assert_eq!(
        apply_hiragana_conversion("あアいイ", HiraganaMode::Toggle),
        "アあイい"
    );
}

#[test]
fn test_hiragana_non_kana_unchanged() {
    assert_eq!(
        apply_hiragana_conversion("Hello123", HiraganaMode::KatakanaToHiragana),
        "Hello123"
    );
}

#[test]
fn test_hiragana_mixed() {
    assert_eq!(
        apply_hiragana_conversion("カタカナとひらがな", HiraganaMode::Toggle),
        "かたかなトヒラガナ"
    );
}

#[test]
fn test_zen_html_entity() {
    assert_eq!(
        apply_zen_conversion("<p class=\"test\">&</p>", ZenMode::HtmlEntity),
        "&lt;p class=&quot;test&quot;&gt;&amp;&lt;/p&gt;"
    );
}

#[test]
fn test_zen_html_entity_no_special_chars() {
    assert_eq!(
        apply_zen_conversion("hello world", ZenMode::HtmlEntity),
        "hello world"
    );
}
