"""This module provides the :func:`EdgeClient` class.

This module is not meant to be imported directly.
"""

from io import BytesIO

import httpx
from pydantic import BaseModel

from edge.settings import SETTINGS


class EdgeClient:
	"""A client for executing RESTful HTTP requests against the local server.

	Not intended for instancing.

	Attributes
	----------
	_client : httpx.Client
		The underlying HTTP client configured with the server's base URL.
	"""

	if SETTINGS.SERVER_ADDRESS is None:
		raise ValueError("No server address provided.")
	if SETTINGS.SERVER_PORT is None:
		raise ValueError("No server port provided.")

	_client: httpx.Client = httpx.Client(
		base_url=f"http://{SETTINGS.SERVER_ADDRESS}:{SETTINGS.SERVER_PORT}"
	)

	@classmethod
	def get_json[T: BaseModel](cls, endpoint: str, response_model: type[T]) -> T:
		"""Executes a HTTP GET request and validates the JSON response.

		Parameters
		----------
		endpoint : str
			The API endpoint path.
		response_model : type[T]
			The Pydantic model class used to parse and validate the response.
		timeout : float, optional
			The maximum time in seconds to wait for a response.

		Returns
		-------
		T
			An instance of the specified Pydantic response model.

		Raises
		------
		httpx.HTTPError
			If the HTTP request fails or returns an error status code.
		pydantic.ValidationError
			If the response JSON payload does not conform to the response model.
		"""

		response: httpx.Response = cls._client.get(endpoint, timeout=SETTINGS.TIMEOUT)
		response.raise_for_status()

		return response_model.model_validate(response.json())

	@classmethod
	def post_json(cls, endpoint: str, payload: BaseModel) -> None:
		"""Executes a HTTP POST request with a Pydantic model payload.

		Parameters
		----------
		endpoint : str
			The API endpoint path.
		payload : T
			The Pydantic model instance containing the request body.
		timeout : float, optional
			The maximum time in seconds to wait for a response.

		Raises
		------
		httpx.HTTPError
			If the HTTP request fails or returns an error status code.
		"""

		json_data: dict = payload.model_dump(mode="json")
		response: httpx.Response = cls._client.post(
			url=endpoint,
			json=json_data,
			timeout=SETTINGS.TIMEOUT,
		)
		response.raise_for_status()

	@classmethod
	def put_file(cls, endpoint: str, payload: BytesIO) -> None:
		"""Executes a HTTP PUT request with a Pydantic model payload.

		Parameters
		----------
		endpoint : str
			The API endpoint path.
		payload : BytesIO
			The file that needs to be sent.
		timeout : float, optional
			The maximum time in seconds to wait for a response.

		Raises
		------
		httpx.HTTPError
			If the HTTP request fails or returns an error status code.
		"""

		response: httpx.Response = cls._client.put(
			url=endpoint,
			files={"upload-file": payload},
			timeout=SETTINGS.TIMEOUT,
		)
		response.raise_for_status()
