// benches/two_sum_benchmark.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hello_cargo_lib::Solution;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("two_sum", |b| {
        b.iter(|| {
            let nums = black_box((1..=100000).collect::<Vec<_>>()); // Increased test data volume
            let target = black_box(199999); // Adjust target accordingly
            Solution::two_sum(nums, target)
        })
    });
}

pub fn criterion_benchmark_bubble(c: &mut Criterion) {
    c.bench_function("two_sum_bubble", |b| {
        b.iter(|| {
            let nums = black_box((1..=100000).collect::<Vec<_>>()); // Increased test data volume
            let target = black_box(199999); // Adjust target accordingly
            Solution::two_sum_bubble(nums, target)
        })
    });
}

criterion_group!(benches, criterion_benchmark, criterion_benchmark_bubble);
criterion_main!(benches);
