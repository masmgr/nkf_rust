# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Rust implementation of nkf (Network Kanji Filter) — a classic Unix tool for Japanese character encoding conversion. Provides both a CLI binary (`nkf`) and a library crate (`nkf_rust`).

## Build & Test Commands

```bash
cargo build              # Build the project
cargo test               # Run all tests
cargo test -- <name>     # Run a single test by name (e.g., cargo test test_detect_shift_jis)
cargo test --lib         # Run library tests only
cargo run -- [options]   # Run the CLI
```

## Architecture

The processing pipeline in `pipeline::process()` defines the core data flow:

**Input bytes → MIME decode → encoding detection → decode to UTF-8 → kana/zen conversion → half-width kana conversion → line ending conversion → MIME encode → encode to target encoding → output bytes**

All transformations operate on UTF-8 as the intermediate representation. The pipeline steps are:

1. **cli** — Parses nkf-style compound flags (e.g., `-sLu` = Shift_JIS output + LF line endings) into `NkfOptions`
2. **detect** — Encoding detection: BOM check → ISO-2022-JP escape sequences → ASCII check → `chardetng` (with Japanese locale hint)
3. **convert** — Encoding conversion via `encoding_rs`. Handles BOM stripping. UTF-16 BE/LE encoding is done manually (not via encoding_rs)
4. **kana** — Half-width ↔ full-width katakana conversion with dakuten/handakuten combining support; full-width ASCII to half-width
5. **mime** — RFC 2047 encoded-word decode/encode (Base64 and Quoted-Printable)
6. **line_ending** — Normalizes all line endings to LF first, then converts to target

Key types:
- `NkfOptions` (pipeline.rs) — Central options struct, flows from CLI parsing through the entire pipeline
- `EncodingType` (encoding_type.rs) — Enum bridging between nkf encoding names and `encoding_rs` types
- `NkfError` (error.rs) — Error enum with `From<io::Error>` impl

## Conventions

- Rust edition 2024
- Tests are co-located in each module (`#[cfg(test)] mod tests`)
- The binary name is `nkf`, the library crate is `nkf_rust`
