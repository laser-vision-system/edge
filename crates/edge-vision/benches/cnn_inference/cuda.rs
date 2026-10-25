#![allow(
	missing_docs,
	clippy::missing_docs_in_private_items,
	reason = "Benchmark"
)]

#[cfg(not(feature = "cnn-backend-cuda"))]
compile_error!("cuda backend not selected");

#[path = "common.rs"]
mod common;

use criterion::{
	Criterion,
	criterion_group,
	criterion_main,
};

fn benchmark(criterion: &mut Criterion) {
	common::benchmark_inference(criterion, "cuda");
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
