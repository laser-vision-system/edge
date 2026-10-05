#[cfg(not(feature = "cnn-backend-tch"))]
compile_error!("tch backend not selected");

#[path = "common.rs"]
mod common;

use criterion::{Criterion, criterion_group, criterion_main};

fn benchmark(criterion: &mut Criterion) {
	common::benchmark_inference(criterion, "tch");
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
