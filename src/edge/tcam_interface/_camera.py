"""This modules provides the :func:`Camera` class.

This module is not meant to be imported directly.
"""

import cv2

from edge.typing import Frame


class Camera:
	"""Representational object for a single camera.

	Parameters
	----------
	index : int
		The camera index on the execution hardware.

		This should increment for each camera connected, up from 0.

	Notes
	-----
	For testing purposes, being able to supply a video file could be useful.
	"""

	def __init__(self, index: int = 0) -> None:
		"""Initialiser dunder.

		Calls when a new instance is constructed.
		"""

		if index < 0:
			raise ValueError("Index must be a positive number.")

		self.index: int = index
		self.capture = cv2.VideoCapture(self.index)

		if not self.capture.isOpened():
			raise RuntimeError("Failed to open camera")

	def __del__(self) -> None:
		"""Deletion dunder.

		Called when an instance is deleted.
		"""

		self.capture.release()

	def get_frame(self) -> Frame:
		"""Retrieves the most recent frame from the camera.

		Returns
		-------
		:func:`Frame`
			The current image from the capture.

		Raises
		------
		RuntimeError
			If the capture failed to read.

		Notes
		-----
		We need to handle a disconnected camera gracefully.
		"""

		flag: bool
		frame: Frame
		flag, frame = self.capture.read()

		# TODO Review if this is acceptable.
		if not flag:
			raise RuntimeError("Frame capture failed.")

		return frame
