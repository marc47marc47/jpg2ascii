use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use jpg2ascii::{convert_path_to_ascii, Config};

pub fn bench_convert(c: &mut Criterion) {
    let img = "image2ascii/convert/testdata/husky_300x300.jpg";
    let mut group = c.benchmark_group("convert");
    for &w in &[80u32, 120u32, 160u32] {
        group.throughput(Throughput::Elements((w as u64) * 100));
        group.bench_with_input(BenchmarkId::from_parameter(w), &w, |b, &width| {
            b.iter(|| {
                let cfg = Config { width: Some(width), ..Default::default() };
                let _ = convert_path_to_ascii(img, &cfg).unwrap();
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_convert);
criterion_main!(benches);

