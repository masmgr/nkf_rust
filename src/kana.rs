use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::LazyLock;

/// Half-width katakana to full-width katakana mapping.
/// Maps (half-width char, optional dakuten/handakuten) -> full-width char.
static HW_TO_FW_KATAKANA: &[(char, char)] = &[
    ('\u{FF66}', '\u{30F2}'), // ｦ -> ヲ
    ('\u{FF67}', '\u{30A1}'), // ｧ -> ァ
    ('\u{FF68}', '\u{30A3}'), // ｨ -> ィ
    ('\u{FF69}', '\u{30A5}'), // ｩ -> ゥ
    ('\u{FF6A}', '\u{30A7}'), // ｪ -> ェ
    ('\u{FF6B}', '\u{30A9}'), // ｫ -> ォ
    ('\u{FF6C}', '\u{30E3}'), // ｬ -> ャ
    ('\u{FF6D}', '\u{30E5}'), // ｭ -> ュ
    ('\u{FF6E}', '\u{30E7}'), // ｮ -> ョ
    ('\u{FF6F}', '\u{30C3}'), // ｯ -> ッ
    ('\u{FF70}', '\u{30FC}'), // ｰ -> ー
    ('\u{FF71}', '\u{30A2}'), // ｱ -> ア
    ('\u{FF72}', '\u{30A4}'), // ｲ -> イ
    ('\u{FF73}', '\u{30A6}'), // ｳ -> ウ
    ('\u{FF74}', '\u{30A8}'), // ｴ -> エ
    ('\u{FF75}', '\u{30AA}'), // ｵ -> オ
    ('\u{FF76}', '\u{30AB}'), // ｶ -> カ
    ('\u{FF77}', '\u{30AD}'), // ｷ -> キ
    ('\u{FF78}', '\u{30AF}'), // ｸ -> ク
    ('\u{FF79}', '\u{30B1}'), // ｹ -> ケ
    ('\u{FF7A}', '\u{30B3}'), // ｺ -> コ
    ('\u{FF7B}', '\u{30B5}'), // ｻ -> サ
    ('\u{FF7C}', '\u{30B7}'), // ｼ -> シ
    ('\u{FF7D}', '\u{30B9}'), // ｽ -> ス
    ('\u{FF7E}', '\u{30BB}'), // ｾ -> セ
    ('\u{FF7F}', '\u{30BD}'), // ｿ -> ソ
    ('\u{FF80}', '\u{30BF}'), // ﾀ -> タ
    ('\u{FF81}', '\u{30C1}'), // ﾁ -> チ
    ('\u{FF82}', '\u{30C4}'), // ﾂ -> ツ
    ('\u{FF83}', '\u{30C6}'), // ﾃ -> テ
    ('\u{FF84}', '\u{30C8}'), // ﾄ -> ト
    ('\u{FF85}', '\u{30CA}'), // ﾅ -> ナ
    ('\u{FF86}', '\u{30CB}'), // ﾆ -> ニ
    ('\u{FF87}', '\u{30CC}'), // ﾇ -> ヌ
    ('\u{FF88}', '\u{30CD}'), // ﾈ -> ネ
    ('\u{FF89}', '\u{30CE}'), // ﾉ -> ノ
    ('\u{FF8A}', '\u{30CF}'), // ﾊ -> ハ
    ('\u{FF8B}', '\u{30D2}'), // ﾋ -> ヒ
    ('\u{FF8C}', '\u{30D5}'), // ﾌ -> フ
    ('\u{FF8D}', '\u{30D8}'), // ﾍ -> ヘ
    ('\u{FF8E}', '\u{30DB}'), // ﾎ -> ホ
    ('\u{FF8F}', '\u{30DE}'), // ﾏ -> マ
    ('\u{FF90}', '\u{30DF}'), // ﾐ -> ミ
    ('\u{FF91}', '\u{30E0}'), // ﾑ -> ム
    ('\u{FF92}', '\u{30E1}'), // ﾒ -> メ
    ('\u{FF93}', '\u{30E2}'), // ﾓ -> モ
    ('\u{FF94}', '\u{30E4}'), // ﾔ -> ヤ
    ('\u{FF95}', '\u{30E6}'), // ﾕ -> ユ
    ('\u{FF96}', '\u{30E8}'), // ﾖ -> ヨ
    ('\u{FF97}', '\u{30E9}'), // ﾗ -> ラ
    ('\u{FF98}', '\u{30EA}'), // ﾘ -> リ
    ('\u{FF99}', '\u{30EB}'), // ﾙ -> ル
    ('\u{FF9A}', '\u{30EC}'), // ﾚ -> レ
    ('\u{FF9B}', '\u{30ED}'), // ﾛ -> ロ
    ('\u{FF9C}', '\u{30EF}'), // ﾜ -> ワ
    ('\u{FF9D}', '\u{30F3}'), // ﾝ -> ン
];

const DAKUTEN: char = '\u{FF9E}'; // ﾞ (half-width dakuten)
const HANDAKUTEN: char = '\u{FF9F}'; // ﾟ (half-width handakuten)

/// O(1) half-width -> full-width lookup
static HW_TO_FW_MAP: LazyLock<HashMap<char, char>> = LazyLock::new(|| {
    HW_TO_FW_KATAKANA.iter().copied().collect()
});

/// O(1) full-width -> (half-width, optional combining mark) lookup
/// Built from the forward table + dakuten/handakuten tables
static FW_TO_HW_MAP: LazyLock<HashMap<char, (char, Option<char>)>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    // Basic mappings (no combining mark)
    for &(hw, fw) in HW_TO_FW_KATAKANA {
        map.insert(fw, (hw, None));
    }
    // Dakuten variants: voiced full-width -> (base half-width, dakuten)
    for &(hw, fw) in HW_TO_FW_KATAKANA {
        if let Some(voiced) = apply_dakuten(fw) {
            map.insert(voiced, (hw, Some(DAKUTEN)));
        }
    }
    // Handakuten variants: semi-voiced full-width -> (base half-width, handakuten)
    for &(hw, fw) in HW_TO_FW_KATAKANA {
        if let Some(semi_voiced) = apply_handakuten(fw) {
            map.insert(semi_voiced, (hw, Some(HANDAKUTEN)));
        }
    }
    map
});

/// Characters that can take dakuten (voiced mark).
/// Maps base full-width -> dakuten full-width.
fn apply_dakuten(c: char) -> Option<char> {
    match c {
        '\u{30AB}' => Some('\u{30AC}'), // カ -> ガ
        '\u{30AD}' => Some('\u{30AE}'), // キ -> ギ
        '\u{30AF}' => Some('\u{30B0}'), // ク -> グ
        '\u{30B1}' => Some('\u{30B2}'), // ケ -> ゲ
        '\u{30B3}' => Some('\u{30B4}'), // コ -> ゴ
        '\u{30B5}' => Some('\u{30B6}'), // サ -> ザ
        '\u{30B7}' => Some('\u{30B8}'), // シ -> ジ
        '\u{30B9}' => Some('\u{30BA}'), // ス -> ズ
        '\u{30BB}' => Some('\u{30BC}'), // セ -> ゼ
        '\u{30BD}' => Some('\u{30BE}'), // ソ -> ゾ
        '\u{30BF}' => Some('\u{30C0}'), // タ -> ダ
        '\u{30C1}' => Some('\u{30C2}'), // チ -> ヂ
        '\u{30C4}' => Some('\u{30C5}'), // ツ -> ヅ
        '\u{30C6}' => Some('\u{30C7}'), // テ -> デ
        '\u{30C8}' => Some('\u{30C9}'), // ト -> ド
        '\u{30CF}' => Some('\u{30D0}'), // ハ -> バ
        '\u{30D2}' => Some('\u{30D3}'), // ヒ -> ビ
        '\u{30D5}' => Some('\u{30D6}'), // フ -> ブ
        '\u{30D8}' => Some('\u{30D9}'), // ヘ -> ベ
        '\u{30DB}' => Some('\u{30DC}'), // ホ -> ボ
        '\u{30A6}' => Some('\u{30F4}'), // ウ -> ヴ
        _ => None,
    }
}

/// Characters that can take handakuten (semi-voiced mark).
fn apply_handakuten(c: char) -> Option<char> {
    match c {
        '\u{30CF}' => Some('\u{30D1}'), // ハ -> パ
        '\u{30D2}' => Some('\u{30D4}'), // ヒ -> ピ
        '\u{30D5}' => Some('\u{30D7}'), // フ -> プ
        '\u{30D8}' => Some('\u{30DA}'), // ヘ -> ペ
        '\u{30DB}' => Some('\u{30DD}'), // ホ -> ポ
        _ => None,
    }
}

/// Convert half-width katakana to full-width, returning Cow to avoid allocation when no half-width kana found.
#[must_use]
pub fn hw_to_fw_katakana_cow(input: &str) -> Cow<'_, str> {
    // Quick scan: check for half-width katakana range (U+FF65..U+FF9F) or dakuten/handakuten
    let has_hw_kana = input.chars().any(|c| ('\u{FF65}'..='\u{FF9F}').contains(&c));
    if !has_hw_kana {
        return Cow::Borrowed(input);
    }
    Cow::Owned(hw_to_fw_katakana(input))
}

/// Convert half-width katakana to full-width katakana.
/// Handles dakuten and handakuten combinations.
#[must_use]
pub fn hw_to_fw_katakana(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        // Check if this is a half-width katakana
        if let Some(fw) = hw_kana_to_fw(c) {
            // Look ahead for dakuten/handakuten
            if let Some(&next) = chars.peek() {
                if next == DAKUTEN {
                    if let Some(voiced) = apply_dakuten(fw) {
                        result.push(voiced);
                        chars.next();
                        continue;
                    }
                } else if next == HANDAKUTEN
                    && let Some(semi_voiced) = apply_handakuten(fw)
                {
                    result.push(semi_voiced);
                    chars.next();
                    continue;
                }
            }
            result.push(fw);
        } else if c == '\u{FF65}' {
            // ･ (half-width middle dot) -> ・ (full-width middle dot)
            result.push('\u{30FB}');
        } else if c == DAKUTEN {
            result.push('\u{309B}'); // standalone dakuten
        } else if c == HANDAKUTEN {
            result.push('\u{309C}'); // standalone handakuten
        } else {
            result.push(c);
        }
    }

    result
}

fn hw_kana_to_fw(c: char) -> Option<char> {
    HW_TO_FW_MAP.get(&c).copied()
}

/// Convert full-width katakana to half-width katakana.
#[must_use]
pub fn fw_to_hw_katakana(input: &str) -> String {
    let mut result = String::with_capacity(input.len());

    for c in input.chars() {
        if let Some((hw, dakuten_mark)) = fw_kana_to_hw(c) {
            result.push(hw);
            if let Some(dm) = dakuten_mark {
                result.push(dm);
            }
        } else if c == '\u{30FB}' {
            result.push('\u{FF65}'); // ・ -> ･
        } else {
            result.push(c);
        }
    }

    result
}

fn fw_kana_to_hw(c: char) -> Option<(char, Option<char>)> {
    FW_TO_HW_MAP.get(&c).copied()
}

/// Convert full-width ASCII/digits (U+FF01-U+FF5E) to half-width ASCII (U+0021-U+007E).
#[must_use]
pub fn fw_to_hw_ascii(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            let cp = c as u32;
            if (0xFF01..=0xFF5E).contains(&cp) {
                char::from_u32(cp - 0xFF01 + 0x0021).unwrap_or(c)
            } else if c == '\u{3000}' {
                // Full-width space -> ASCII space
                ' '
            } else {
                c
            }
        })
        .collect()
}

/// Convert full-width space to specified replacement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZenMode {
    /// -Z0: Convert full-width alphabet to ASCII
    AlphaToAscii,
    /// -Z1: Convert full-width space to single space
    SpaceToOne,
    /// -Z2: Convert full-width space to two spaces
    SpaceToTwo,
    /// -Z3: Convert to HTML entities (not yet implemented)
    HtmlEntity,
    /// -Z4: Convert full-width Katakana to half-width
    KatakanaToHw,
}

/// Apply zen conversion, returning Cow to avoid allocation when no conversion is needed.
#[must_use]
pub fn apply_zen_conversion_cow(input: &str, mode: ZenMode) -> Cow<'_, str> {
    match mode {
        ZenMode::AlphaToAscii | ZenMode::SpaceToOne => {
            if !input.chars().any(|c| {
                let cp = c as u32;
                (0xFF01..=0xFF5E).contains(&cp) || c == '\u{3000}'
            }) {
                return Cow::Borrowed(input);
            }
            Cow::Owned(fw_to_hw_ascii(input))
        }
        ZenMode::SpaceToTwo => {
            if !input.contains('\u{3000}') {
                return Cow::Borrowed(input);
            }
            Cow::Owned(input.replace('\u{3000}', "  "))
        }
        ZenMode::HtmlEntity => {
            if !input.chars().any(|c| matches!(c, '&' | '<' | '>' | '"')) {
                return Cow::Borrowed(input);
            }
            Cow::Owned(apply_zen_conversion(input, mode))
        }
        ZenMode::KatakanaToHw => {
            if !input.chars().any(|c| FW_TO_HW_MAP.contains_key(&c) || c == '\u{30FB}') {
                return Cow::Borrowed(input);
            }
            Cow::Owned(fw_to_hw_katakana(input))
        }
    }
}

#[must_use]
pub fn apply_zen_conversion(input: &str, mode: ZenMode) -> String {
    match mode {
        ZenMode::AlphaToAscii | ZenMode::SpaceToOne => fw_to_hw_ascii(input),
        ZenMode::SpaceToTwo => input.replace('\u{3000}', "  "),
        ZenMode::HtmlEntity => {
            let mut result = String::with_capacity(input.len());
            for c in input.chars() {
                match c {
                    '&' => result.push_str("&amp;"),
                    '<' => result.push_str("&lt;"),
                    '>' => result.push_str("&gt;"),
                    '"' => result.push_str("&quot;"),
                    _ => result.push(c),
                }
            }
            result
        }
        ZenMode::KatakanaToHw => fw_to_hw_katakana(input),
    }
}

/// Hiragana/Katakana conversion mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HiraganaMode {
    /// -h1: Katakana to Hiragana
    KatakanaToHiragana,
    /// -h2: Hiragana to Katakana
    HiraganaToKatakana,
    /// -h3: Toggle both directions
    Toggle,
}

/// Apply hiragana/katakana conversion, returning Cow to avoid allocation when no conversion needed.
#[must_use]
pub fn apply_hiragana_conversion_cow(input: &str, mode: HiraganaMode) -> Cow<'_, str> {
    let needs_conversion = input.chars().any(|c| {
        let cp = c as u32;
        match mode {
            HiraganaMode::KatakanaToHiragana => (0x30A1..=0x30F6).contains(&cp),
            HiraganaMode::HiraganaToKatakana => (0x3041..=0x3096).contains(&cp),
            HiraganaMode::Toggle => {
                (0x30A1..=0x30F6).contains(&cp) || (0x3041..=0x3096).contains(&cp)
            }
        }
    });
    if !needs_conversion {
        return Cow::Borrowed(input);
    }
    Cow::Owned(apply_hiragana_conversion(input, mode))
}

/// Apply hiragana/katakana conversion based on mode.
#[must_use]
pub fn apply_hiragana_conversion(input: &str, mode: HiraganaMode) -> String {
    input
        .chars()
        .map(|c| {
            let cp = c as u32;
            match mode {
                HiraganaMode::KatakanaToHiragana => {
                    // Katakana U+30A1..U+30F6 -> Hiragana U+3041..U+3096
                    if (0x30A1..=0x30F6).contains(&cp) {
                        char::from_u32(cp - 0x60).unwrap_or(c)
                    } else {
                        c
                    }
                }
                HiraganaMode::HiraganaToKatakana => {
                    // Hiragana U+3041..U+3096 -> Katakana U+30A1..U+30F6
                    if (0x3041..=0x3096).contains(&cp) {
                        char::from_u32(cp + 0x60).unwrap_or(c)
                    } else {
                        c
                    }
                }
                HiraganaMode::Toggle => {
                    if (0x30A1..=0x30F6).contains(&cp) {
                        char::from_u32(cp - 0x60).unwrap_or(c)
                    } else if (0x3041..=0x3096).contains(&cp) {
                        char::from_u32(cp + 0x60).unwrap_or(c)
                    } else {
                        c
                    }
                }
            }
        })
        .collect()
}

#[cfg(test)]
#[path = "tests/kana_tests.rs"]
mod tests;
