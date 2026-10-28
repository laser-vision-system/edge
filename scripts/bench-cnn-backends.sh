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
	printf '\n== Mean inference time by backend and case ==\n'
	python3 - "${successes[@]}" <<'PY'
import json
import pathlib
import sys

root = pathlib.Path("target/criterion")
print(f"{'backend':<10} {'batch_01':>14} {'batch_02':>14} {'batch_03':>14} {'batch_04':>14} {'batch_05':>14} {'batch_06':>14} {'batch_07':>14} {'batch_08':>14} {'batch_09':>14} {'batch_10':>14}")
for backend in sys.argv[1:]:
	means = []
	for case in ("batch_01", "batch_02", "batch_03", "batch_04", "batch_05", "batch_06", "batch_07", "batch_08", "batch_09", "batch_10"):
		path = root / f"cnn_inference_{backend}" / case / backend / "estimates.json"
		try:
			mean_ms = json.loads(path.read_text())["mean"]["point_estimate"] / 1_000_000
			means.append(f"{mean_ms:.3f} ms")
		except (OSError, KeyError, json.JSONDecodeError):
			means.append("n/a")
	print(f"{backend:<10} {means[0]:>14} {means[1]:>14} {means[2]:>14} {means[3]:>14} {means[4]:>14} {means[5]:>14} {means[6]:>14} {means[7]:>14} {means[8]:>14} {means[9]:>14}")
print(f"{'backend':<10} {'sequential_01':>14} {'sequential_02':>14} {'sequential_03':>14} {'sequential_04':>14} {'sequential_05':>14} {'sequential_06':>14} {'sequential_07':>14} {'sequential_08':>14} {'sequential_09':>14} {'sequential_10':>14}")
for backend in sys.argv[1:]:
	means = []
	for case in ("sequential_01", "sequential_02", "sequential_03", "sequential_04", "sequential_05", "sequential_06", "sequential_07", "sequential_08", "sequential_09", "sequential_10"):
		path = root / f"cnn_inference_{backend}" / case / backend / "estimates.json"
		try:
			mean_ms = json.loads(path.read_text())["mean"]["point_estimate"] / 1_000_000
			means.append(f"{mean_ms:.3f} ms")
		except (OSError, KeyError, json.JSONDecodeError):
			means.append("n/a")
	print(f"{backend:<10} {means[0]:>14} {means[1]:>14} {means[2]:>14} {means[3]:>14} {means[4]:>14} {means[5]:>14} {means[6]:>14} {means[7]:>14} {means[8]:>14} {means[9]:>14}")
PY
fi

if ((${#failures[@]} > 0)); then
	printf '\nFailed backends: %s\n' "${failures[*]}" >&2
	exit 1
fi
