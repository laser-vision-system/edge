"""This module provides assorted endpoint request functions.

This module is not meant to be imported directly.
"""

from io import BytesIO

import httpx

from ._client import EdgeClient
from ._models import CameraInfoModel, ConfigurationModel, GUIUpdateModel


class EndpointCameraInfo:
	@staticmethod
	def post(camera_info: CameraInfoModel, /) -> None:
		"""Post the camera info to the server.

		Parameters
		----------
		camera_info : CameraInfo
			The pydantic model containing the camera info.
		"""

		endpoint: str = "/camera_info"
		try:
			EdgeClient.post_json(endpoint=endpoint, payload=camera_info)
		except httpx.HTTPError as http_err:
			print(f"HTTP error occurred while connecting to server: {http_err}")
		except Exception as err:
			print(f"An unexpected error occurred: {err}")


class EndpointConfiguration:
	@staticmethod
	def post(camera_config: ConfigurationModel, /) -> None:
		"""Post the camera configuration info to the server.

		Parameters
		----------
		camera_config : CameraConfig
			The pydantic model containing the camera config.
		"""

		endpoint: str = "/configuration"
		try:
			EdgeClient.post_json(endpoint=endpoint, payload=camera_config)
		except httpx.HTTPError as http_err:
			print(f"HTTP error occurred while connecting to server: {http_err}")
		except Exception as err:
			print(f"An unexpected error occurred: {err}")

	@staticmethod
	def get() -> ConfigurationModel | None:
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
			return EdgeClient.get_json(endpoint=endpoint, response_model=ConfigurationModel)
		except httpx.HTTPError as http_err:
			print(f"HTTP error occurred while connecting to server: {http_err}")
		except Exception as err:
			print(f"An unexpected error occurred: {err}")


class EndpointGUIUpdate:
	@staticmethod
	def get() -> GUIUpdateModel | None:
		"""Get the a request to update from from the server.

		Returns
		-------
		Setpoint
			The request from the server.
		None
			Returned if an exception was caught.
		"""

		endpoint: str = "/gui_update"
		try:
			return EdgeClient.get_json(endpoint=endpoint, response_model=GUIUpdateModel)
		except httpx.HTTPError as http_err:
			print(f"HTTP error occurred while connecting to server: {http_err}")
		except Exception as err:
			print(f"An unexpected error occurred: {err}")


class EndpointImage:
	@staticmethod
	def put(image: BytesIO, /) -> None:
		"""Send an image to the server.

		Parameters
		----------
		image : BytesIO
			The image to be sent to the server.
		"""

		endpoint: str = "/image"
		try:
			EdgeClient.put_file(endpoint=endpoint, payload=image)
		except httpx.HTTPError as http_err:
			print(f"HTTP error occurred while connecting to server: {http_err}")
		except Exception as err:
			print(f"An unexpected error occurred: {err}")
