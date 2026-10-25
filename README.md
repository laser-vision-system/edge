# Edge

Edge Processing Tier μ-com project.

## Details

The Edge is the code that runs on the Raspberry Pi that ingests camera data, processes it to extract laser intensity and deviation, then sends that data to the server.

The project is written in Rust for performance and memory safety.

Rust projects are managed by `Cargo`; you will need to install this to begin development.

Deployment is done through Docker. You will also need to install it yourself.

To enable the pre-commit hook, run `git config core.hooksPath .githooks`.

See [CONTRIBUTING](docs/CONTRIBUTING.md).

### Feature flags

Available backend features are `cnn-backend-flex`, `cnn-backend-wgpu`, and `cnn-backend-cuda`.

Flex is enabled by default. To build with another Burn backend, disable default features and select exactly one backend, for example:

```sh
cargo build --no-default-features --features cnn-backend-wgpu
```

### CNN benchmarks

Run the inference benchmark once for every backend with:

```sh
./scripts/bench-cnn-backends.sh
```

The script builds each backend separately and measures batches of 1 and 10 images plus ten sequential single-image calls with 100 Criterion samples, a 10-second warm-up, and a 60-second target measurement time for each case. The flex backend will take significantly longer than the other backends. The script generates gnuplot charts and stores each backend's results independently in `target/criterion/cnn_inference_<backend>`.

Once a backend has completed, a summary is printed to the terminal which reports Criterion's mean time spent in `process` for each case. Batch and image setup happens before timing; tensor preparation performed internally by `process` remains part of the measurement.

Pass backend names to limit the run, for example `./scripts/bench-cnn-backends.sh wgpu flex`. Direct runs use the corresponding dedicated target, for example `cargo bench --bench cnn_inference_flex --no-default-features --features cnn-backend-flex`.

The benchmark loads `models/laser-cnn.bpk` once per backend; backends that need unavailable hardware or system libraries may fail, while the script continues with the remaining backends and reports failures at the end.

## Docker image

In order to run our docker image you will need to pull the image from github. to start off with make sure you are logged in to login to the github docker registery by doing the following:
```
# Create a new personal access token (classic) with the appropriate scopes for the tasks you want to accomplish. If your organization requires SSO, you must enable SSO for your new token.
# Select the read:packages scope to download container images and read their metadata.
export CR_PAT=YOUR_TOKEN
echo $CR_PAT | sudo docker login ghcr.io -u USERNAME --password-stdin
```
After you are logged in run the following command to pull the image:
```
# For raspbery pi's or MAC machines
docker pull ghcr.io/laser-vision-system/edge-arm64:BRANCH
# For other hardware
docker pull ghcr.io/laser-vision-system/edge-amd64:BRANCH
```
If you want to make and run a container for testing run the following:
```
docker run --rm --net=host --privileged --name edge --mount type=bind,src=/dev,dst=/dev ghcr.io/laser-vision-system/edge-arm64:BRANCH
```
For deployment on the raspberry pi instead run the following:
```
docker run --net=host --privileged --name edge --restart unless-stopped --mount type=bind,src=/dev,dst=/dev ghcr.io/laser-vision-system/edge-arm64:BRANCH
```
