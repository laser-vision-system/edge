#!/usr/bin/env bash
set -uo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")/.."

if (($# == 0)); then
	backends=(flex wgpu cuda)
else
	backends=("$@")
fi

failures=()
successes=()

for backend in "${backends[@]}"; do
	case "$backend" in
		flex|wgpu|cuda) ;;
		*)
			printf 'Unknown CNN backend: %s\n' "$backend" >&2
			exit 2
			;;
	esac

	feature="cnn-backend-$backend"
	printf '\n=== Benchmarking %s ===\n' "$feature"
	if cargo bench --bench "cnn_inference_$backend" --no-default-features \
		--features "$feature cnn-model-zero-copy" -- --sample-size 100 --warm-up-time 10 \
		--measurement-time 60 --save-baseline "$backend"; then
		successes+=("$backend")
		printf 'Results and plots saved under target/criterion/cnn_inference_%s.\n' "$backend"
	else
		failures+=("$feature")
		printf 'Benchmark failed for %s; continuing with remaining backends.\n' "$feature" >&2
	fi
done

if ((${#successes[@]} > 0)); then
	printf '\nMean inference time by backend and case:\n'
	python3 - "${successes[@]}" <<'PY'
import json
import pathlib
import sys

root = pathlib.Path("target/criterion")
print(f"{'backend':<10} {'batch_1':>14} {'batch_10':>14} {'ten_single':>14}")
for backend in sys.argv[1:]:
	means = []
	for case in ("batch_1", "batch_10", "ten_single_calls"):
		path = root / f"cnn_inference_{backend}" / case / backend / "estimates.json"
		try:
			mean_ms = json.loads(path.read_text())["mean"]["point_estimate"] / 1_000_000
			means.append(f"{mean_ms:.3f} ms")
		except (OSError, KeyError, json.JSONDecodeError):
			means.append("n/a")
	print(f"{backend:<10} {means[0]:>14} {means[1]:>14} {means[2]:>14}")
PY
fi

if ((${#failures[@]} > 0)); then
	printf '\nFailed backends: %s\n' "${failures[*]}" >&2
	exit 1
fi
