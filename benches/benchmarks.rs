#[macro_use]
extern crate criterion;

use criterion::Criterion;
use mlc::logger;
use mlc::markup::MarkupType;
use mlc::Config;

fn end_to_end_benchmark() {
    let config = Config {
        folder: String::from("./benches/benchmark"),
        log_level: logger::LogLevel::Debug,
        markup_types: vec![MarkupType::Markdown],
    };
    let _ = mlc::run(&config);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("End to end benchmark", |b| b.iter(|| end_to_end_benchmark()));
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = criterion_benchmark
}
criterion_main!(benches);