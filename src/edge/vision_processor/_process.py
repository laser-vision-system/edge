"""This module provides the :func:`process_frame` function.

This module is not meant to be imported directly.

The plan is to allow for the specific vision implementation to be swapped out during
runtime.
"""

from io import BytesIO

from cv2 import COLOR_BGR2RGB, cvtColor
from PIL.Image import Image as PIL_Image, fromarray

from edge.edge_client import (
	CameraInfoModel,
	EndpointCameraInfo,
	EndpointImage,
)
from edge.typing import Frame

from ._demonstration import Demonstration


def _send_image(frame: Frame, /) -> None:
	"""Send the current frame as an image to the server.

	Parameters
	----------
	frame : :func:`Frame`
		The current frame.
	"""

	img_pil: PIL_Image = fromarray(obj=cvtColor(src=frame, code=COLOR_BGR2RGB))
	img_bytes = BytesIO()
	img_pil.save(fp=img_bytes, format="png")
	EndpointImage.put(img_bytes)
	img_bytes.close()


def _send_info(value: float, deviation: float) -> None:
	"""Send the calculation info to the server.

	Parameters
	----------
	value : float
		The calculate value.
	"""

	info = CameraInfoModel(intensity=value, deviation=deviation)
	EndpointCameraInfo.post(info)


def process_frame(frame: Frame, setpoint: bool = False) -> tuple[float, float]:
	"""Apply the currently selected vision implementation to the frame.

	Parameters
	----------
	frame : Frame
		The frame to use.

	Returns
	-------
	float
		The calculated value for this frame.
	"""

	results: tuple[float, float] = Demonstration.process_frame(
		frame=frame, setpoint=setpoint
	)

	# Do after processing to reduce delay.
	_send_image(frame)
	_send_info(*results)
	return results
