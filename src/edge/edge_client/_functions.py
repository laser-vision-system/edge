"""This module provides assorted endpoint request functions.

This module is not meant to be imported directly.
"""

import httpx

from ._client import EdgeClient
from ._models import CameraConfig, CameraInfo, Image, Setpoint


def camera_info_post(camera_info: CameraInfo, /) -> None:
	"""Post the camera info to the server.

	Parameters
	----------
	camera_info : CameraInfo
		The pydantic model containing the camera info.
	"""

	endpoint: str = "/camera_info"
	try:
		EdgeClient.post(endpoint=endpoint, payload=camera_info)
	except httpx.HTTPError as http_err:
		print(f"HTTP error occurred while connecting to server: {http_err}")
	except Exception as err:
		print(f"An unexpected error occurred: {err}")


def configuration_post(camera_config: CameraConfig, /) -> None:
	"""Post the camera configuration info to the server.

	Parameters
	----------
	camera_config : CameraConfig
		The pydantic model containing the camera config.
	"""

	endpoint: str = "/configuration"
	try:
		EdgeClient.post(endpoint=endpoint, payload=camera_config)
	except httpx.HTTPError as http_err:
		print(f"HTTP error occurred while connecting to server: {http_err}")
	except Exception as err:
		print(f"An unexpected error occurred: {err}")


def configuration_get() -> CameraConfig | None:
	"""Get the camera configuration from the server.

	Returns
	-------
	CameraConfig
		The configuration option from the server.
	None
		Returned if an exception was caught.
	"""

	endpoint: str = "/configuration"
	try:
		return EdgeClient.get(
			endpoint=endpoint, response_model=CameraConfig
		)
	except httpx.HTTPError as http_err:
		print(f"HTTP error occurred while connecting to server: {http_err}")
	except Exception as err:
		print(f"An unexpected error occurred: {err}")


def setpoint_get() -> Setpoint | None:
	"""Get the state of the setpoint option from the server.

	Returns
	-------
	Setpoint
		The setpoint option from the server.
	None
		Returned if an exception was caught.
	"""

	endpoint: str = "/setpoint"
	try:
		return EdgeClient.get(endpoint=endpoint, response_model=Setpoint)
	except httpx.HTTPError as http_err:
		print(f"HTTP error occurred while connecting to server: {http_err}")
	except Exception as err:
		print(f"An unexpected error occurred: {err}")


def image_put(image: Image, /) -> None:
	"""Send an image to the server.

	Parameters
	----------
	image : Image
		The image to be sent, expected to be the frame.
	"""

	endpoint: str = "/image"
	try:
		EdgeClient.put(endpoint=endpoint, payload=image)
	except httpx.HTTPError as http_err:
		print(f"HTTP error occurred while connecting to server: {http_err}")
	except Exception as err:
		print(f"An unexpected error occurred: {err}")
