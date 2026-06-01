"""This modules provides the :func:`Demonstration` class.

Not an actual vision implementation, only used for demonstration purposes.

This module is not meant to be imported directly.
"""

from typing import override

import cv2
import numpy as np

from edge.typing import Frame

from ._vision_interface import VisionInterface


class Demonstration(VisionInterface):
	"""A \"machine vison implementation\" that just outputs the average perceived
	lightness."""

	@override
	@classmethod
	def process_frame(cls, frame: Frame, /) -> float:
		"""Calculate the average perceived lightness of a given frame.

		Parameters
		----------
		frame : :func:`Frame`
			A three-dimensional numpy array with the third dimension having a size of 3.

			The colour channel should be formatted as (B,G,R).

		Returns
		-------
		float
			The average perceived lightness (L*) of the frame.
			Values typically range from 0.0 (black) to 100.0 (white).

		Raises
		------
		ValueError
			If `frame` does not have exactly 3 dimensions or 3 colour channels.

		Notes
		-----
		Source: https://stackoverflow.com/a/56678483

		Uses OpenCV's CIELAB colour space conversion.
		"""

		if frame.ndim != 3 or frame.shape[-1] != 3:
			raise ValueError(
				"The frame must be a 3D numpy array with 3 colour channels (B,G,R)."
			)

		# Normalise as 32-bit floats.
		if np.issubdtype(frame.dtype, np.integer):
			frame_norm: Frame = frame.astype(np.float32) / 255.0
		else:
			frame_norm = frame.astype(np.float32)

		# Convert BGR to CIELAB colour space.
		lab_frame: Frame = cv2.cvtColor(src=frame_norm, code=cv2.COLOR_BGR2Lab)
		l_star: Frame = lab_frame[..., 0]
		return float(np.average(l_star))
