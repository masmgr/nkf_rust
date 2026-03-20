use encoding_rs::{EUC_JP, Encoding, ISO_2022_JP, SHIFT_JIS, UTF_8, UTF_16BE, UTF_16LE};

pub const BOM_UTF8: &[u8] = &[0xEF, 0xBB, 0xBF];
pub const BOM_UTF16_BE: &[u8] = &[0xFE, 0xFF];
pub const BOM_UTF16_LE: &[u8] = &[0xFF, 0xFE];

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
}

impl EncodingType {
    #[must_use]
    pub fn to_encoding_rs(&self) -> &'static Encoding {
        match self {
            EncodingType::Ascii | EncodingType::Utf8 | EncodingType::Utf8Bom => UTF_8,
            EncodingType::Utf16Be => UTF_16BE,
            EncodingType::Utf16Le => UTF_16LE,
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
