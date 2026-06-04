# /// script
# requires-python = ">=3.13"
# dependencies = []
# ///
import argparse
import subprocess
import sys
from argparse import ArgumentParser


LOCAL_IMAGE_NAME = "edge"
REMOTE_IMAGE_NAME = "oatmeal0505/laser-edge"
DEV_TAG = "dev"
PROD_TAG = "prod"
DEFAULT_ADDRESS = "172.17.0.2"
DEFAULT_PORT = "8080"


def run_command(cmd: list[str], /) -> None:
	try:
		# Remove empty strings
		cmd = [i for i in cmd if i]
		print(f"Executing: {' '.join(cmd)}")
		subprocess.run(args=cmd, check=True)
	except subprocess.CalledProcessError as e:
		print(f"Error: Command failed with exit code {e.returncode}")
		sys.exit(e.returncode)
	except FileNotFoundError:
		print("Error: Docker is not installed or not in your PATH.")
		sys.exit(1)
	except KeyboardInterrupt:
		pass


# TODO Remove prod build when CI/CD implemented
def build(prod: bool, remote: bool, no_cache: bool) -> None:
	tag = PROD_TAG if prod else DEV_TAG
	cmd: list[str] = [
		"docker",
		"build",
		"--no-cache" if no_cache else "",
		"--target",
		f"{'production' if prod else 'development'}",
		"-t",
		f"{LOCAL_IMAGE_NAME}:{tag}",
		".",
	]

	run_command(cmd)

	if remote:
		run_command(
			[
				"docker",
				"image",
				"tag",
				f"{LOCAL_IMAGE_NAME}:{tag}",
				f"{REMOTE_IMAGE_NAME}:{tag}",
			]
		)

		run_command(["docker", "push", f"{REMOTE_IMAGE_NAME}:{tag}"])


# TODO Remove when CI/CD implemented
def run_prod(remote: bool, addr: str, port: str) -> None:
	cmd: list[str] = [
		"docker",
		"run",
		"--rm",
		"--device=/dev/video0",
		"-e",
		f"addr={addr}",
		"-e",
		f"port={port}",
		f"{REMOTE_IMAGE_NAME if remote else LOCAL_IMAGE_NAME}:{PROD_TAG}",
	]

	run_command(cmd)


def run_dev(
	remote: bool, addr: str, port: str, verbosity: int, interactive: bool
) -> None:
	cmd: list[str] = [
		"docker",
		"run",
		"" if verbosity > 0 or interactive else "-it",
		"--rm",
		"--device=/dev/video0",
		"-e",
		f"addr={addr}",
		"-e",
		f"port={port}",
		"-e" if verbosity > 0 else "",
		"debug=True" if verbosity > 0 else "",
		f"{REMOTE_IMAGE_NAME if remote else LOCAL_IMAGE_NAME}:{DEV_TAG}",
		"bash" if interactive else "",
	]

	run_command(cmd)


def run_live(remote: bool = True, interactive: bool = False) -> None:
	cmd: list[str] = [
		"docker",
		"run",
		"-it",
		"--rm",
		"--device=/dev/video0",
		"-v",
		"./src:/app/src",
		# BUG Mounts, but isn't read when run
		"-v",
		"./.env:/.dockerenv",
		"-v",
		"./pyproject.toml:/app/pyproject.toml",
		"-v",
		"./uv.lock:/app/uv.lock",
		"-v",
		"./data:/app/data",
		f"{REMOTE_IMAGE_NAME if remote else LOCAL_IMAGE_NAME}:{DEV_TAG}",
		"bash" if interactive else "",
	]

	run_command(cmd)


def test() -> None:
	run_command(["docker", "build", "--target", "test", "."])


def make_build_parser(build_parser: ArgumentParser, /) -> None:
	build_parser.add_argument(
		"target",
		choices=["prod", "dev"],
		nargs="+",
		help="target image(s) to build",
	)
	build_parser.add_argument(
		"-r",
		"--remote",
		action="store_true",
		help="push the image to remote after building",
	)
	build_parser.add_argument(
		"-n",
		"--no_cache",
		action="store_true",
		help="ignored cached layers and build from scratch",
	)


def make_run_parser(run_parser: ArgumentParser, /) -> None:
	subp_runner = run_parser.add_subparsers(
		title="target",
		required=True,
		dest="target",
	)

	prod_parser: ArgumentParser = subp_runner.add_parser(
		name="prod", help="run the production container"
	)
	dev_parser: ArgumentParser = subp_runner.add_parser(
		name="dev", help="run the development container"
	)
	live_parser: ArgumentParser = subp_runner.add_parser(
		name="live", help="run the development container with mounted files"
	)

	for parser in [prod_parser, dev_parser, live_parser]:
		parser.add_argument(
			"-r", "--remote", action="store_true", help="use the image on the remote"
		)

	for parser in [prod_parser, dev_parser]:
		parser.add_argument(
			"--addr",
			"--address",
			type=str,
			help="the address of the server (default=172.17.0.2)",
			default=DEFAULT_ADDRESS,
		)
		parser.add_argument(
			"--port",
			type=str,
			default=DEFAULT_PORT,
			help="the port the server uses (default=8080)",
		)

	dev_parser.add_argument(
		"-v",
		"--verbose",
		action="count",
		dest="verbosity",
		default=0,
		help="set the verbosity",
	)

	for parser in [dev_parser, live_parser]:
		parser.add_argument(
			"-i",
			"--interactive",
			action="store_true",
			default=False,
			help="start with bash",
		)


def parse_arguments(parser: ArgumentParser, /) -> None:
	args: argparse.Namespace = parser.parse_args()
	match args.action:
		case "build":
			for target in args.target:
				build(prod=target == "prod", remote=args.remote, no_cache=args.no_cache)
		case "run":
			match args.target:
				case "prod":
					run_prod(remote=args.remote, addr=args.addr, port=args.port)
				case "dev":
					run_dev(
						remote=args.remote,
						addr=args.addr,
						port=args.port,
						verbosity=args.verbosity,
						interactive=args.interactive,
					)
				case "live":
					run_live(remote=args.remote, interactive=args.interactive)
		case "test":
			test()
		case _:
			parser.print_help()


def main() -> None:
	parser = ArgumentParser(
		prog="uv run poe manage", description="Docker Management Script"
	)
	# Leaving as not required so running this empty prints the help body.
	subp_command = parser.add_subparsers(title="action", dest="action")
	build_parser: ArgumentParser = subp_command.add_parser(
		name="build", help="build an image"
	)
	run_parser: ArgumentParser = subp_command.add_parser(
		name="run", help="run a container"
	)
	subp_command.add_parser(name="test", help="run tests via Docker build")

	make_build_parser(build_parser)
	make_run_parser(run_parser)
	parse_arguments(parser)


if __name__ == "__main__":
	main()
