use criterion::{criterion_group, criterion_main, Criterion};

mod case;
mod data;

// to boot benchmark of this
// `cargo bench --bench list_bench -- --sample-size 100`

fn bench_list(c: &mut Criterion) {
    let mut r = data::list_data::empty_list();
    c.bench_function("RList push", move |b| {
        b.iter_with_large_drop(|| case::list_case::push_case(&mut r, 10));
    });
}

criterion_group!(benches, bench_list);
criterion_main!(benches);
