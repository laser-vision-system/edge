#[cfg(not(feature = "cnn-backend-wgpu"))]
compile_error!("wgpu backend not selected");

#[path = "common.rs"]
mod common;

use criterion::{Criterion, criterion_group, criterion_main};

fn benchmark(criterion: &mut Criterion) {
	common::benchmark_inference(criterion, "wgpu");
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
