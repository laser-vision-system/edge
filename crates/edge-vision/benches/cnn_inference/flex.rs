#![allow(
	missing_docs,
	clippy::missing_docs_in_private_items,
	reason = "Benchmark"
)]

#[cfg(not(feature = "cnn-backend-flex"))]
compile_error!("flex backend not selected");

#[path = "common.rs"]
mod common;

use criterion::{
	Criterion,
	criterion_group,
	criterion_main,
};

fn benchmark(criterion: &mut Criterion) {
	common::benchmark_inference(criterion, "flex");
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
