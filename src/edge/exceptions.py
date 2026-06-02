"""
Exceptions and Warnings
=======================

General exceptions used in the edge tier.
"""


class CaptureOpenError(RuntimeError):
	"""The camera failed to connect to the capture.

	This is raised when the camera attempts to open the capture but fails.
	"""

	pass


class FrameReadError(RuntimeError):
	"""The camera can no longer read a frame from the capture.

	This is raised when the camera fails to read a frame from the capture too many
	times.
	"""

	pass


class InitialisationError(RuntimeError):
	"""This class was not initialised.

	This is raised when attempting to use a class without initialising it.
	"""

	pass
