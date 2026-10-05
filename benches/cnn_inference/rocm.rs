#[cfg(not(feature = "cnn-backend-rocm"))]
compile_error!("rocm backend not selected");

#[path = "common.rs"]
mod common;

use criterion::{Criterion, criterion_group, criterion_main};

fn benchmark(criterion: &mut Criterion) {
	common::benchmark_inference(criterion, "rocm");
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
