use encoding_rs::{EUC_JP, Encoding, ISO_2022_JP, SHIFT_JIS, UTF_8, UTF_16BE, UTF_16LE};

pub const BOM_UTF8: &[u8] = &[0xEF, 0xBB, 0xBF];
pub const BOM_UTF16_BE: &[u8] = &[0xFE, 0xFF];
pub const BOM_UTF16_LE: &[u8] = &[0xFF, 0xFE];
pub const BOM_UTF32_BE: &[u8] = &[0x00, 0x00, 0xFE, 0xFF];
pub const BOM_UTF32_LE: &[u8] = &[0xFF, 0xFE, 0x00, 0x00];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncodingType {
    Ascii,
    Utf8,
    Utf8Bom,
    Utf16Be,
    Utf16Le,
    ShiftJis,
    EucJp,
    Iso2022Jp,
    Utf32Be,
    Utf32Le,
}

impl EncodingType {
    #[must_use]
    pub fn to_encoding_rs(&self) -> &'static Encoding {
        match self {
            EncodingType::Ascii | EncodingType::Utf8 | EncodingType::Utf8Bom => UTF_8,
            EncodingType::Utf16Be => UTF_16BE,
            EncodingType::Utf16Le => UTF_16LE,
            EncodingType::Utf32Be | EncodingType::Utf32Le => UTF_8, // encoding_rs doesn't support UTF-32; handled manually
            EncodingType::ShiftJis => SHIFT_JIS,
            EncodingType::EucJp => EUC_JP,
            EncodingType::Iso2022Jp => ISO_2022_JP,
        }
    }

    #[must_use]
    pub fn from_encoding_rs(enc: &'static Encoding) -> Option<Self> {
        const ENCODING_MAP: &[(&Encoding, EncodingType)] = &[
            (UTF_8, EncodingType::Utf8),
            (SHIFT_JIS, EncodingType::ShiftJis),
            (EUC_JP, EncodingType::EucJp),
            (ISO_2022_JP, EncodingType::Iso2022Jp),
            (UTF_16BE, EncodingType::Utf16Be),
            (UTF_16LE, EncodingType::Utf16Le),
        ];
        ENCODING_MAP
            .iter()
            .find(|(e, _)| *e == enc)
            .map(|(_, t)| *t)
    }

    #[must_use]
    pub fn display_name(&self) -> &'static str {
        match self {
            EncodingType::Ascii => "ASCII",
            EncodingType::Utf8 => "UTF-8",
            EncodingType::Utf8Bom => "UTF-8 (BOM)",
            EncodingType::Utf16Be => "UTF-16BE",
            EncodingType::Utf16Le => "UTF-16LE",
            EncodingType::ShiftJis => "Shift_JIS",
            EncodingType::EucJp => "EUC-JP",
            EncodingType::Iso2022Jp => "ISO-2022-JP",
            EncodingType::Utf32Be => "UTF-32BE",
            EncodingType::Utf32Le => "UTF-32LE",
        }
    }

    /// Parse encoding name (case-insensitive) to EncodingType.
    #[must_use]
    pub fn from_name(name: &str) -> Option<Self> {
        match name.to_ascii_uppercase().replace('-', "").replace('_', "").as_str() {
            "ASCII" => Some(EncodingType::Ascii),
            "UTF8" => Some(EncodingType::Utf8),
            "UTF8BOM" => Some(EncodingType::Utf8Bom),
            "UTF16" | "UTF16BE" => Some(EncodingType::Utf16Be),
            "UTF16LE" => Some(EncodingType::Utf16Le),
            "UTF32" | "UTF32BE" => Some(EncodingType::Utf32Be),
            "UTF32LE" => Some(EncodingType::Utf32Le),
            "SHIFTJIS" | "SJIS" | "CP932" | "WINDOWS31J" => Some(EncodingType::ShiftJis),
            "EUCJP" => Some(EncodingType::EucJp),
            "ISO2022JP" | "JIS" => Some(EncodingType::Iso2022Jp),
            _ => None,
        }
    }
}

impl std::fmt::Display for EncodingType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

#[cfg(test)]
#[path = "tests/encoding_type_tests.rs"]
mod tests;
