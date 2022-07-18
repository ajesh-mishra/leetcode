use criterion::{criterion_group, criterion_main, Criterion};
use summary_ranges::Solution;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function(
        "summary ranges", 
        |b| b.iter(
            || Solution::summary_ranges(vec![0, 2, 3, 4, 6, 8, 9])
        )
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);