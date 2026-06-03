"""This module provides the :func:`run` function.

This module is not meant to be imported directly.
"""

import time

from edge.edge_client import (
	ConfigurationModel,
	EndpointConfiguration,
	EndpointGUIUpdate,
	GUIUpdateModel,
)
from edge.exceptions import FrameReadError, InitialisationError
from edge.settings import SETTINGS
from edge.tcam_interface import Camera
from edge.typing import Frame
from edge.vision_processor import process_frame


class Conductor:
	"""The entrypoint and heartbeat of the edge tier.

	Call :func:`run` to start.
	"""

	__has_initialised: bool = False
	elapsed_t: float = time.time()

	@classmethod
	def _initialise(cls) -> None:
		cls.cam0 = Camera(
			capture_location=SETTINGS.input_source(), frametime=1.0 / SETTINGS.FRAMERATE
		)

		if SETTINGS.DEBUG:
			print("Intialised.")

		# TODO Update.
		EndpointConfiguration.post(
			ConfigurationModel(brightness=1.0, framerate=SETTINGS.FRAMERATE)
		)

		cls.__has_initialised = True

	@classmethod
	def _has_initialised(cls, func, /):
		if cls.__has_initialised is False:
			raise InitialisationError(
				f"{cls.__name__}.{cls._initialise.__name__} not called "
				f"before {func} was called."
			)

	@classmethod
	def _loop(cls) -> None:
		cls._has_initialised(cls._loop.__name__)
		if SETTINGS.DEBUG:
			print(f"Previous frametime: {cls.elapsed_t:.4f} | ", end="")

		start_t: float = time.time()

		gui_update: GUIUpdateModel | None = EndpointGUIUpdate.get()

		# Apply camera configuration.
		if gui_update is not None and gui_update.config:
			config: ConfigurationModel | None = EndpointConfiguration.get()
			if config is not None:
				cls.cam0.frametime = 1.0 / config.framerate

		frame: Frame = cls.cam0.get_frame()
		frame_t: float = time.time() - start_t

		setpoint: bool = gui_update.setpoint if gui_update is not None else False
		results: tuple[float, float] = process_frame(frame=frame, setpoint=setpoint)
		process_t: float = time.time() - start_t

		if SETTINGS.DEBUG:
			print(
				f"Perceived lightness: {results[0]} "
				f"| Deviation: {results[1]}"
				f"| Read: {frame_t:.4f}, Process: {process_t:.4f} "
				f"| Press Ctrl+C to stop."
			)

		cls.elapsed_t = time.time() - start_t
		wait_t: float = cls.cam0.frametime - cls.elapsed_t
		if wait_t > 0:
			time.sleep(wait_t)

	@classmethod
	def _terminate(cls) -> None:
		cls._has_initialised(cls._terminate.__name__)

		del cls.cam0
		if SETTINGS.DEBUG:
			print("Terminated.")

	@classmethod
	def run(cls) -> None:
		cls._initialise()

		try:
			while True:
				cls._loop()

		except KeyboardInterrupt:
			print("\nInterrupting... ", end="")

		except FrameReadError:
			if SETTINGS.INPUT_SOURCE == SETTINGS.InputType.VIDEO:
				print("\nVideo input stopped... ", end="")
			else:
				raise

		finally:
			cls._terminate()
