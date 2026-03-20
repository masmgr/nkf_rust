# nkf_rust

Rust implementation of [nkf (Network Kanji Filter)](https://ja.wikipedia.org/wiki/Nkf) — a tool for Japanese character encoding conversion.

## Features

- Encoding conversion: UTF-8, Shift_JIS, EUC-JP, ISO-2022-JP, UTF-16BE/LE, UTF-32BE/LE
- Automatic encoding detection (BOM, escape sequences, chardetng)
- Line ending conversion (LF / CRLF / CR)
- Half-width ↔ full-width katakana conversion (with dakuten/handakuten support)
- Full-width ASCII → half-width ASCII conversion
- Hiragana ↔ katakana conversion
- MIME Base64 / Quoted-Printable encode/decode (RFC 2047)
- Line folding with CJK double-width character support
- Input decoding: URL-encoded (%xx), hex-encoded (:xx), numeric character references (&#nnnn;)
- HTML entity conversion (`&amp;`, `&lt;`, `&gt;`, `&quot;`)
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
nkf -w16B input.txt            # → UTF-16BE
nkf -w32L input.txt            # → UTF-32LE
nkf --oc=CP932 input.txt       # → CP932 (Shift_JIS alias)

# Encoding detection
nkf -g input.txt
nkf --guess input.txt

# Line ending conversion
nkf -Lw input.txt              # → CRLF
nkf -Lu input.txt              # → LF
nkf -d input.txt               # → LF (shortcut)
nkf -c input.txt               # → CRLF (shortcut)

# Kana conversion
nkf -Z1 input.txt              # Full-width space → ASCII space
nkf -Z3 input.txt              # Convert &, <, >, " to HTML entities
nkf -h1 input.txt              # Katakana → Hiragana
nkf -h2 input.txt              # Hiragana → Katakana
nkf --hiragana input.txt       # Same as -h1
nkf --katakana input.txt       # Same as -h2

# Line folding
nkf -f input.txt               # Fold at 60 columns (default)
nkf -f40 input.txt             # Fold at 40 columns
nkf -F input.txt               # Fold at 80 columns

# Input decoding
nkf --url-input input.txt      # Decode %xx sequences
nkf --numchar-input input.txt  # Decode &#nnnn; references

# Compound flags
nkf -sLu input.txt             # Shift_JIS output + LF line endings

# MIME
nkf -mB encoded.txt            # Base64 decode
nkf -MB input.txt              # Base64 encode

# Overwrite files in-place
nkf --overwrite -w input.txt
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
| `-w32B` / `-w32L` | Output UTF-32BE / UTF-32LE |
| `--oc=NAME` | Specify output encoding by name |
| `-J` / `-S` / `-E` / `-W` | Specify input encoding |
| `--ic=NAME` | Specify input encoding by name |
| `-g`, `--guess` | Detect and print encoding |
| `-Lu` / `-Lw` / `-Lm` | Line endings: LF / CRLF / CR |
| `-d` / `-c` | Line endings: LF / CRLF (shortcuts) |
| `--unix` / `--windows` / `--mac` | Line endings by platform name |
| `-mB` / `-mQ` / `-m0` | MIME decode: Base64 / QP / None |
| `-MB` / `-MQ` | MIME encode: Base64 / QP |
| `-x` | Preserve half-width kana |
| `-Z[0-4]` | Full-width conversion (Z3=HTML entities) |
| `-h1` / `-h2` / `-h3` | Katakana↔Hiragana / Hiragana↔Katakana / Toggle |
| `--hiragana` / `--katakana` | Same as -h1 / -h2 |
| `-f[cols]` | Fold lines at specified width (default: 60) |
| `-F` | Fold lines at 80 columns |
| `--url-input` | Decode URL-encoded (%xx) input |
| `--cap-input` | Decode hex-encoded (:xx) input |
| `--numchar-input` | Decode numeric character references |
| `--overwrite` | Overwrite input files |
| `--jis` / `--euc` / `--sjis` | Encoding presets |
