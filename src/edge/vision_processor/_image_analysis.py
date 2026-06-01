"""This modules provides the :func:`ImageAnalysis` class.

This module is not meant to be imported directly.
"""

from typing import override

from edge.typing import Frame

from ._vision_interface import VisionInterface


class ImageAnalysis(VisionInterface):
	"""A machine vision implementation that uses traditional image analysis."""

	@override
	@classmethod
	def process_frame(cls, frame: Frame, /) -> float:
		raise NotImplementedError
