use criterion::{black_box, Criterion, criterion_group, criterion_main};

fn day01_benchmark(c: &mut Criterion) {
    let input1 = black_box(include_str!("../input1.txt"));
    let input2 = black_box(include_str!("../input2.txt"));

    c.bench_function("part 1", |b| b.iter(|| day01::part1::process(input1)));
    c.bench_function("part 2", |b| b.iter(|| day01::part2::process(input2)));
}

criterion_group!(benches, day01_benchmark);
criterion_main!(benches);