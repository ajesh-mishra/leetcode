use criterion::{criterion_group, criterion_main, Criterion};
use contains_duplicate::*;

pub fn bench_contains_duplicate(c: &mut Criterion) {
    let mut group = c.benchmark_group("contains_duplicate");
    
    group.bench_function("HashMap", |b| 
        b.iter(|| Solution::contains_duplicate(vec![1,1,1,3,3,4,3,2,4,2]))
    );
    
    group.bench_function("Vector", |b| 
        b.iter(|| Solution::contains_duplicate_2(vec![1,1,1,3,3,4,3,2,4,2]))
    );
    
    group.finish();
}

criterion_group!(benches, bench_contains_duplicate);
criterion_main!(benches);