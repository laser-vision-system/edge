"""This modules provides the :func:`VisionInterface` interface class.

This module is not meant to be imported directly.
"""

from abc import abstractmethod

from edge.typing import Frame
from edge.utils import BaseInterface


class VisionInterface(BaseInterface):
	"""An interfaces defining the common functions of all vision implementations."""

	@classmethod
	@abstractmethod
	def process_frame(cls, frame: Frame, setpoint: bool) -> tuple[float, float]:
		raise NotImplementedError
