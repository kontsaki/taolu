use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day1::{find_pair_declarative, find_pair_iterative, get_expenses};

fn benchmark(c: &mut Criterion) {
    c.bench_function("find pair declarative", |b| {
        let input = get_expenses("input");
        b.iter(|| find_pair_declarative(black_box(&input)))
    });

    c.bench_function("find pair iterative", |b| {
        let input = get_expenses("input");
        b.iter(|| find_pair_iterative(black_box(&input)))
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
