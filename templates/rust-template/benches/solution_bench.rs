#[path = "../src/solution.rs"]
mod solution;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    // Zde pište vaše benchmarky
    // --
    // Write your benchmarks here

    // Vždy předávejte argumenty metodám za pomocí `black_box` metody. Tato metoda zabrání nechtěným optimalizacím compileru (jako počítání výsledků při kompilaci)
    // --
    // Always pass the arguments to function using the `black_box` method. This will disable unwanted compiler optimizations (like calculating results at compile time)

    // Názorný příklad
    // --
    // Example benchmark

    // c.bench_function("solution exmaple", |b| {
    //     b.iter(|| solution::algorithm(black_box(--Add argument--)))
    // });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
