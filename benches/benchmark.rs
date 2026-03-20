use criterion::{Criterion, black_box, criterion_group, criterion_main};
use nkf_rust::encoding_type::EncodingType;
use nkf_rust::pipeline::{NkfOptions, process};

fn make_utf8_japanese(size: usize) -> Vec<u8> {
    let base = "日本語のテキストです。これはベンチマーク用のサンプルデータです。\n";
    base.repeat(size).into_bytes()
}

fn make_sjis_data() -> Vec<u8> {
    let utf8 = "日本語のテキストです。サンプルデータ。\n".repeat(100);
    let encoding = encoding_rs::SHIFT_JIS;
    let (result, _, _) = encoding.encode(&utf8);
    result.into_owned()
}

fn make_hw_kana_text(size: usize) -> Vec<u8> {
    let base = "ｱｲｳｴｵｶﾞｷﾞｸﾞｹﾞｺﾞｻﾞｼﾞｽﾞｾﾞｿﾞﾊﾟﾋﾟﾌﾟﾍﾟﾎﾟ\n";
    base.repeat(size).into_bytes()
}

fn make_mixed_line_endings(size: usize) -> Vec<u8> {
    let base = "行1\r\n行2\r行3\n行4\r\n";
    base.repeat(size).into_bytes()
}

fn bench_utf8_passthrough(c: &mut Criterion) {
    let input = make_utf8_japanese(200);
    let options = NkfOptions::default();
    c.bench_function("utf8_passthrough", |b| {
        b.iter(|| process(black_box(&input), black_box(&options)));
    });
}

fn bench_sjis_to_utf8(c: &mut Criterion) {
    let input = make_sjis_data();
    let options = NkfOptions {
        output_encoding: Some(EncodingType::Utf8),
        ..Default::default()
    };
    c.bench_function("sjis_to_utf8", |b| {
        b.iter(|| process(black_box(&input), black_box(&options)));
    });
}

fn bench_line_ending_conversion(c: &mut Criterion) {
    let input = make_mixed_line_endings(200);
    let options = NkfOptions {
        line_ending: Some(nkf_rust::line_ending::LineEnding::CrLf),
        ..Default::default()
    };
    c.bench_function("line_ending_crlf", |b| {
        b.iter(|| process(black_box(&input), black_box(&options)));
    });
}

fn bench_hw_kana_conversion(c: &mut Criterion) {
    let input = make_hw_kana_text(200);
    let options = NkfOptions::default();
    c.bench_function("hw_kana_to_fw", |b| {
        b.iter(|| process(black_box(&input), black_box(&options)));
    });
}

criterion_group!(
    benches,
    bench_utf8_passthrough,
    bench_sjis_to_utf8,
    bench_line_ending_conversion,
    bench_hw_kana_conversion,
);
criterion_main!(benches);
