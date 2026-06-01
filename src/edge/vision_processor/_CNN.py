"""This modules provides the :func:`CNN` class.

This module is not meant to be imported directly.
"""

from typing import override

from edge.typing import Frame

from ._vision_interface import VisionInterface


class CNN(VisionInterface):
	"""A machine vision implementation that uses a convolution neural network."""

	@override
	@classmethod
	def process_frame(cls, frame: Frame, /) -> float:
		raise NotImplementedError
