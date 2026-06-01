"""A wrapper class for OpenCV for reading a video input.

Can read from either a webcam or video file.

Not intended for deployment, only as reference.
"""

from pathlib import Path as _Path

import cv2 as _cv2

from edge.typing import Frame, ProcessFrameCallback


class VideoFeed:
	"""A class for storing a video input with a callback function.

	You can request a frame processed through the callback function.

	Parameters
	----------
	capture_location : int | str | Path
		What the video source is.

		- For a webcam input, use an `int`, 0 is the default webcam.
		- For a video file, pass in a file location either as `str` or `Path`
		(preferred).

	callback : :func:`ProcessFrameCallback`, optional
		The function that will be applied to a :func:`Frame`.

		This function can return a :func:`Frame` which will be used for display if
		enabled.

		:func:`ProcessFrameCallback` = `(Frame) -> Frame | None`.

		:func:`Frame` is a numpy array with the shape (height, width, colour).

	frametime : float, default=1.0/30.0
		In seconds, how long between each frame. Use `1 / fps` if you want to pass in
		a framerate.
	"""

	def __init__(
		self,
		capture_location: int | str | _Path,
		callback: ProcessFrameCallback | None = None,
		frametime: float = 1.0 / 30.0,
	) -> None:
		# === Input sanitisation ===

		if isinstance(capture_location, int):  # Webcam input
			if capture_location < 0:
				raise ValueError("Integer input must be a positive number.")
		else:  # Filepath input
			# Convert string to Path object for easier operation.
			if isinstance(capture_location, str):
				capture_location = _Path(capture_location)

			if not capture_location.exists() or capture_location.is_dir():
				raise FileNotFoundError("No file was found at this location.")

		self.vid_cap = _cv2.VideoCapture(capture_location)

		if not self.vid_cap.isOpened():
			raise Exception("Failed to open video capture.")

		self.callback: ProcessFrameCallback | None = callback
		self.frametime: float = frametime

	def __del__(self) -> None:
		self.vid_cap.release()

	def process_frame(self) -> Frame:
		"""Read from the video input and apply the callback to it.

		Returns
		-------
		Frame
			Either the raw current frame, or one modified by the callback function.
		"""
		ret: bool
		frame: Frame
		ret, frame = self.vid_cap.read()

		if not ret:
			raise ValueError

		frame_result: Frame | None = (
			self.callback(frame) if self.callback is not None else None
		)
		if frame_result is None:
			frame_result = frame

		return frame_result
