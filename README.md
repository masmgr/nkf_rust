# nkf_rust

Rust implementation of [nkf (Network Kanji Filter)](https://ja.wikipedia.org/wiki/Nkf) — a tool for Japanese character encoding conversion.

## Features

- Encoding conversion: UTF-8, Shift_JIS, EUC-JP, ISO-2022-JP, UTF-16BE/LE
- Automatic encoding detection (BOM, escape sequences, chardetng)
- Line ending conversion (LF / CRLF / CR)
- Half-width ↔ full-width katakana conversion (with dakuten/handakuten support)
- Full-width ASCII → half-width ASCII conversion
- MIME Base64 / Quoted-Printable encode/decode (RFC 2047)
- nkf-compatible compound flag syntax (e.g. `-sLu`)

## Installation

```bash
cargo install --path .
```

## Usage

```bash
# Encoding conversion
echo "日本語" | nkf -s          # UTF-8 → Shift_JIS
nkf -e input.txt               # → EUC-JP
nkf -w -S input.txt            # Shift_JIS → UTF-8

# Encoding detection
nkf -g input.txt
nkf --guess input.txt

# Line ending conversion
nkf -Lw input.txt              # → CRLF
nkf -Lu input.txt              # → LF

# Compound flags
nkf -sLu input.txt             # Shift_JIS output + LF line endings

# Overwrite files in-place
nkf --overwrite -w input.txt

# MIME
nkf -mB encoded.txt            # Base64 decode
nkf -MB input.txt              # Base64 encode
```

## Library Usage

```rust
use nkf_rust::{EncodingType, NkfOptions, convert, detect, process};

// Detect encoding
let result = detect(b"\x93\xFA\x96\x7B\x8C\xEA");
println!("{}", result.encoding); // Shift_JIS

// Convert encoding
let sjis = convert(
    "日本語".as_bytes(),
    EncodingType::Utf8,
    EncodingType::ShiftJis,
).unwrap();

// Full pipeline with options
let options = NkfOptions {
    output_encoding: Some(EncodingType::ShiftJis),
    ..Default::default()
};
let output = process("日本語".as_bytes(), &options).unwrap();
```

## Options

| Flag | Description |
|------|-------------|
| `-j` | Output ISO-2022-JP |
| `-s` | Output Shift_JIS |
| `-e` | Output EUC-JP |
| `-w` | Output UTF-8 (default) |
| `-w16B` / `-w16L` | Output UTF-16BE / UTF-16LE |
| `-J` / `-S` / `-E` / `-W` | Specify input encoding |
| `-g`, `--guess` | Detect and print encoding |
| `-Lu` / `-Lw` / `-Lm` | Line endings: LF / CRLF / CR |
| `-mB` / `-mQ` / `-m0` | MIME decode: Base64 / QP / None |
| `-MB` / `-MQ` | MIME encode: Base64 / QP |
| `-x` | Preserve half-width kana |
| `-Z[0-4]` | Full-width conversion |
| `--overwrite` | Overwrite input files |

## License

MIT
