use std::{hint::black_box, time::Instant};

use criterion::{Criterion, Throughput};
use edge::vision_processor::{BatchItem, Cnn, VisionInterface as _};
use image::{Rgb, RgbImage};

pub fn benchmark_inference(criterion: &mut Criterion, backend: &str) {
	let mut cnn = Cnn::new().expect("CNN benchmark requires a valid model");
	let image = RgbImage::from_pixel(640, 480, Rgb([255, 40, 90]));
	let mut group =
		criterion.benchmark_group(format!("cnn_inference_{backend}"));

	for batch_size in [1, 10] {
		let mut states =
			(0..batch_size).map(|_| cnn.new_state()).collect::<Vec<_>>();
		let mut batch = states
			.iter_mut()
			.enumerate()
			.map(|(index, state)| {
				BatchItem::new(index as u64, state, image.clone(), true, false)
			})
			.collect::<Vec<_>>();

		group.throughput(Throughput::Elements(batch_size));
		group.bench_function(format!("batch_{batch_size}"), |bencher| {
			bencher.iter_custom(|iterations| {
				let batch = black_box(&mut batch);
				let start = Instant::now();
				for _ in 0..iterations {
					cnn.process(batch);
				}
				start.elapsed()
			});
		});

		if batch_size == 10 {
			group.bench_function("ten_single_calls", |bencher| {
				bencher.iter_custom(|iterations| {
					let batch = black_box(&mut batch);
					let start = Instant::now();
					for _ in 0..iterations {
						for item in batch.iter_mut() {
							cnn.process(std::slice::from_mut(item));
						}
					}
					start.elapsed()
				});
			});
		}
	}

	group.finish();
}
