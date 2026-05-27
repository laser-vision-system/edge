"""Project settings file.

Create a file named .env and set the variables to the values you want.

e.g.:
INPUT_SOURCE='video'
TESTING_VID='/path/to/the/video/file.mp4'
FRAMERATE=60
"""

from pathlib import Path as _Path
from typing import Annotated as _Annotated

from pydantic import Field as _Field
from pydantic_settings import (
	BaseSettings as _BaseSettings,
	SettingsConfigDict as _SettingsConfigDict
)

INPUT_TYPE_WEBCAM = 'webcam'
INPUT_TYPE_VIDEO = 'video'


class _Settings(_BaseSettings):
	model_config = _SettingsConfigDict(
		env_file='.env',
		env_file_encoding='utf-8',
		case_sensitive=False,
		env_ignore_empty=True,
		extra='ignore'
	)

	INPUT_SOURCE: _Annotated[
		str,
		_Field(frozen=True)
	] = 'webcam'
	"""What input source to use.

	options: 'webcam', 'video'
	"""

	WEBCAM_ID: _Annotated[
		int,
		_Field(frozen=True, ge=0)
	] = 0
	"""The id of the webcam used."""

	TESTING_VID: _Annotated[
		_Path | None,
		_Field(frozen=True)
	] = None
	"""The path to a video using for testing.

	Example
		`TESTING_VID='data/testing/example.mp4'`
	"""

	FRAMERATE: _Annotated[
		float,
		_Field(frozen=True, gt=0)
	] = 30.0
	"""The target for how frequently the video input is processed.

	The actual framerate may be lower due to peformance overhead.
	"""


	def input_source(self) -> int | _Path:
		"""Util function to parse and validate the input source.

		Returns
		-------
		INPUT : int or Path
			If the input source is determined to be a webcam, return the integer ID of the webcam.
			If the input source is determined to be a video, validate that the file exists. Does not confirm the file type.

		Raises
		------
		ValueError
			- If an incorrect input source was somehow set.
			- If an input video was selected but not set.
			- If an input video was selected but set to an invalid path.
		"""
		if self.INPUT_SOURCE == INPUT_TYPE_WEBCAM:
			INPUT = self.WEBCAM_ID
		elif self.INPUT_SOURCE == INPUT_TYPE_VIDEO:
			# Technically redundant
			if not self.TESTING_VID:
				raise ValueError('`TESTING_VID` setting not set.')
			if not self.TESTING_VID.exists():
				raise ValueError(f'File not found: {self.TESTING_VID}')

			INPUT = self.TESTING_VID
		else:
			raise ValueError(f'INPUT_SOURCE somehow set to invalid value. {self.INPUT_SOURCE}')

		return INPUT


SETTINGS = _Settings()
"""All of the project settings.

Module level singleton pattern.
"""

# Verify the input video if it is chosen.
if SETTINGS.INPUT_SOURCE == INPUT_TYPE_VIDEO:
	if not SETTINGS.TESTING_VID:
		raise ValueError('`TESTING_VID` setting not set.')
	if not SETTINGS.TESTING_VID.exists():
		raise ValueError(f'File not found: {SETTINGS.TESTING_VID}')
