"""This module provides the :func:`process_frame` function.

This module is not meant to be imported directly.

The plan is to allow for the specific vision implementation to be swapped out during
runtime.
"""

from io import BytesIO

from cv2 import COLOR_BGR2RGB, cvtColor
from PIL.Image import Image as PIL_Image, fromarray

from edge.edge_client import (
	CameraInfo,
	Image as Edge_Image,
	camera_info_post,
	image_put,
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
	img_pydantic: Edge_Image = Edge_Image(image=img_bytes.read())
	image_put(img_pydantic)
	img_bytes.close()

def _send_info(value: float, /) -> None:
	"""Send the calculation info to the server.

	Parameters
	----------
	value : float
		The calculate value.
	"""

	info = CameraInfo(intensity=value, deviation=0)
	camera_info_post(info)

def process_frame(frame: Frame, /) -> float:
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

	perceived_lightness: float = Demonstration.process_frame(frame)
	# Do after processing to reduce delay.
	_send_image(frame)
	_send_info(perceived_lightness)
	return perceived_lightness
