"""This module provides the :func:`run` function.

This module is not meant to be imported directly.
"""

import time

from edge.settings import SETTINGS
from edge.tcam_interface import Camera
from edge.typing import Frame
from edge.vision_processor import process_frame


def run() -> None:
	cam0 = Camera(capture_location=SETTINGS.input_source())

	delta_t: float = 1.0 / SETTINGS.FRAMERATE

	# Send configuration.

	try:
		while True:
			start_t: float = time.time()

			# Check configuration.

			frame: Frame = cam0.get_frame()
			frame_t: float = time.time() - start_t

			# Check setpoint.

			value: float = process_frame(frame)
			process_t: float = time.time() - start_t

			# Send image.
			# Send camera info.

			# Remove.
			print(
				f"Perceived lightness: {value} "
				f"| Read: {frame_t:.4f}, Process: {process_t:.4f} "
				f"| Press Ctrl+C to stop."
			)

			elapsed_t: float = time.time() - start_t
			wait_t: float = delta_t - elapsed_t
			if wait_t > 0:
				time.sleep(wait_t)

	except KeyboardInterrupt:
		print("\nInterrupting... ")

	except RuntimeError:
		print("\nVideo input stopped... ")

	finally:
		print("Done.")
		del cam0

	# Close camera.
	# Close client.
