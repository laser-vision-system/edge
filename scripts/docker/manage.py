# /// script
# requires-python = ">=3.13"
# dependencies = []
# ///
import argparse
import subprocess
import sys


IMAGE_NAME = "edge"
DEV_TAG = "dev"
PROD_TAG = "latest"


def run_command(cmd: list[str], /) -> None:
	try:
		print(f"Executing: {' '.join(cmd)}")
		subprocess.run(args=cmd, check=True)
	except subprocess.CalledProcessError as e:
		print(f"Error: Command failed with exit code {e.returncode}")
		sys.exit(e.returncode)
	except FileNotFoundError:
		print("Error: Docker is not installed or not in your PATH.")
		sys.exit(1)


def build_dev(no_cache=False) -> None:
	cmd: list[str] = [
		"docker",
		"build",
		"--target",
		"development",
		"-t",
		f"{IMAGE_NAME}:{DEV_TAG}",
		".",
	]

	if no_cache:
		cmd.append("--no-cache")

	run_command(cmd)


def build_prod(no_cache=False) -> None:
	cmd: list[str] = [
		"docker",
		"build",
		"--target",
		"production",
		"-t",
		f"{IMAGE_NAME}:{PROD_TAG}",
		".",
	]

	if no_cache:
		cmd.append("--no-cache")

	run_command(cmd)


def test() -> None:
	run_command(["docker", "build", "--target", "test", "."])


def run_dev(interactive=False) -> None:
	cmd: list[str] = [
		"docker",
		"run",
		"-it",
		"--rm",
		"-v",
		"./src:/app/src",
	]

	if interactive:
		cmd.extend(
			[
				"-v",
				"./pyproject.toml:/app/pyproject.toml",
				"-v",
				"./uv.lock:/app/uv.lock",
				f"{IMAGE_NAME}:{DEV_TAG}",
				"bash",
			]
		)
	else:
		cmd.append(f"{IMAGE_NAME}:{DEV_TAG}")

	run_command(cmd)


def main() -> None:
	parser = argparse.ArgumentParser(description="Docker Management Script")
	subparsers = parser.add_subparsers(dest="command")

	build_dev_parser = subparsers.add_parser(
		name="build-dev", help="Build the development image"
	)
	build_dev_parser.add_argument(
		"--no-cache",
		name="no_cache",
		action="store_true",
		help="Ignored cached layers and rebuild from scratch",
	)
	build_prod_parser = subparsers.add_parser(
		name="build-prod", help="Build the production image"
	)
	build_prod_parser.add_argument(
		"--no-cache",
		name="no_cache",
		action="store_true",
		help="Ignored cached layers and rebuild from scratch",
	)
	subparsers.add_parser(name="test", help="Run tests via Docker build gate")

	run_parser = subparsers.add_parser(
		name="run-dev", help="Run the development container"
	)
	run_parser.add_argument(
		"-i",
		"--interactive",
		action="store_true",
		help="Start with bash and mounted config files",
	)

	args: argparse.Namespace = parser.parse_args()

	if args.command == "build-dev":
		build_dev(no_cache=args.no_cache)
	elif args.command == "build-prod":
		build_prod(no_cache=args.no_cache)
	elif args.command == "test":
		test()
	elif args.command == "run-dev":
		run_dev(interactive=args.interactive)
	else:
		parser.print_help()


if __name__ == "__main__":
	main()
