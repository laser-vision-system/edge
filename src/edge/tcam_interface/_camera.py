"""This modules provides the :func:`Camera` class.

This module is not meant to be imported directly.
"""

from pathlib import Path

import cv2
import time

from edge.typing import Frame


class Camera:
	"""Representational object for a single camera.

	Parameters
	----------
	capture_location : int | str | Path
		What the video source is. The intended use is with a webcam input,
		however a video file input can be supplied for testing.

		- For a webcam input, use an `int`, 0 is the default webcam.
		- For a video file, pass in a file location either as `str` or `Path`
		(preferred).

	frametime : float, default=1.0/30.0
		In seconds, how long between each frame. Use `1 / fps` if you want to pass in
		a framerate.

		NOTE This may need to change when we know more about configuration.
	"""

	def __init__(
		self,
		capture_location: int | str | Path,
		frametime: float = 1.0 / 30.0,
	) -> None:
		"""Initialiser dunder.

		Calls when a new instance is constructed.
		"""

		if isinstance(capture_location, int):  # Webcam input
			if capture_location < 0:
				raise ValueError("Integer input must be a natural number.")
		else:  # Filepath input
			# Convert string to Path object for easier operation.
			if isinstance(capture_location, str):
				capture_location = Path(capture_location)

			if not capture_location.exists() or capture_location.is_dir():
				raise FileNotFoundError("No file was found at this location.")
			
		self.capture_location = capture_location

		self.capture = cv2.VideoCapture(capture_location)

		if not self.capture.isOpened():
			raise RuntimeError("Failed to open camera")

		self.frametime: float = frametime

	def __del__(self) -> None:
		"""Deletion dunder.

		Called when an instance is deleted.
		"""

		self.capture.release()

	def reconnect(self, attempts: int = 5, delay: float = 2.0) -> bool:
		"""Attempts to reconnect if the camera has disconnected.

		Parameters
		-------
		attempts : int, default = 5
			The number of reconnect attempts to be made

		delay : float, default = 2.0
			The time between reconnection attempts in seconds

		Returns
		-------
		bool: True if the camera connection is able to be restablished, false if not
		"""
		self.capture.release()
		for _ in range(attempts):
			self.capture = cv2.VideoCapture(self.capture_location)
			if self.capture.isOpened():
				return True
			time.sleep(delay)
		return False

	def get_frame(self, retries: int = 3) -> Frame:
		"""Retrieves the most recent frame from the camera.

		Returns
		-------
		:func:`Frame`
			The current image from the capture.

		Raises
		------
		RuntimeError
			- If the camera diconnected
			- If the capture failed to read

		Notes
		-----
		We need to handle a disconnected camera gracefully.
		"""

		if not self.capture.isOpened():
			if not self.reconnect():
				raise RuntimeError("Camera disconnected and could not reconnect.")
		
		ret: bool
		frame: Frame

		for _ in range(retries):
			ret, frame = self.capture.read()
			if ret and frame is not None and frame.size > 0:
				return frame
			
		raise RuntimeError(f"Frame capture failed after {retries} attempts.")

		"""
		TODO Review if this is acceptable.
		if not ret:
			raise RuntimeError("Frame capture failed.")
		"""

		return frame

	def detect_configurations(self):
		raise NotImplementedError

	def apply_configurations(self):
		raise NotImplementedError
