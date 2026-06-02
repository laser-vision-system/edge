"""This module provides assorted endpoint request functions.

This module is not meant to be imported directly.
"""

import httpx

from ._client import EdgeClient
from ._models import CameraConfig, CameraInfo, Image, Setpoint


def camera_info_post(camera_info: CameraInfo) -> None:
	endpoint: str = "/camera_info"
	try:
		EdgeClient.post(endpoint=endpoint, payload=camera_info)
	except httpx.HTTPError as http_err:
		print(f"HTTP error occurred while connecting to server: {http_err}")
	except Exception as err:
		print(f"An unexpected error occurred: {err}")


def configuration_post(camera_config: CameraConfig) -> None:
	endpoint: str = "/configuration"
	try:
		EdgeClient.post(endpoint=endpoint, payload=camera_config)
	except httpx.HTTPError as http_err:
		print(f"HTTP error occurred while connecting to server: {http_err}")
	except Exception as err:
		print(f"An unexpected error occurred: {err}")


def configuration_get() -> tuple[float, float] | None:
	endpoint: str = "/configuration"
	try:
		result: CameraConfig = EdgeClient.get(
			endpoint=endpoint, response_model=CameraConfig
		)
		return result.brightness, result.framerate
	except httpx.HTTPError as http_err:
		print(f"HTTP error occurred while connecting to server: {http_err}")
	except Exception as err:
		print(f"An unexpected error occurred: {err}")


def setpoint_get() -> bool | None:
	endpoint: str = "/setpoint"
	try:
		result: Setpoint = EdgeClient.get(endpoint=endpoint, response_model=Setpoint)
		return result.should
	except httpx.HTTPError as http_err:
		print(f"HTTP error occurred while connecting to server: {http_err}")
	except Exception as err:
		print(f"An unexpected error occurred: {err}")


def image_put(image: Image) -> None:
	endpoint: str = "/image"
	try:
		EdgeClient.put(endpoint=endpoint, payload=image)
	except httpx.HTTPError as http_err:
		print(f"HTTP error occurred while connecting to server: {http_err}")
	except Exception as err:
		print(f"An unexpected error occurred: {err}")
