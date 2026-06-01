"""Project settings file.

Create a file named .env and set the variables to the values you want.

e.g.:
INPUT_SOURCE='video'
TESTING_VID='/path/to/the/video/file.mp4'
FRAMERATE=60
"""

from enum import StrEnum
from pathlib import Path
from typing import Annotated

from pydantic import Field
from pydantic_settings import BaseSettings, SettingsConfigDict


class _Settings(BaseSettings):
	model_config = SettingsConfigDict(
		env_file=".env",
		env_file_encoding="utf-8",
		case_sensitive=False,
		env_ignore_empty=True,
		extra="ignore",
		ignored_types=(),
	)

	class InputType(StrEnum):
		WEBCAM = "webcam"
		VIDEO = "video"

	INPUT_SOURCE: Annotated[InputType, Field(frozen=True)] = InputType.WEBCAM
	"""What input source to use.

	options: 'webcam', 'video'
	"""

	WEBCAM_ID: Annotated[int, Field(frozen=True, ge=0)] = 0
	"""The id of the webcam used."""

	TESTING_VID: Annotated[Path | None, Field(frozen=True)] = None
	"""The path to a video using for testing.

	Example
		`TESTING_VID='data/testing/example.mp4'`
	"""

	FRAMERATE: Annotated[float, Field(frozen=True, gt=0)] = 30.0
	"""The maximum framerate that is displayed.

	The actual framerate may be lower due to peformance overhead.
	"""

	WINDOW_WIDTH: Annotated[int, Field(frozen=True, gt=1)] = 1000
	"""The starting width of the window."""

	WINDOW_HEIGHT: Annotated[int, Field(frozen=True, gt=1)] = 500
	"""The starting height of the window."""

	INPUT_BUTTON_WIDTH: Annotated[int, Field(frozen=True, gt=0, lt=WINDOW_WIDTH)] = 30
	"""The width to use for the input button."""

	def input_source(self) -> int | Path:
		"""Util function to parse and validate the input source.

		Returns
		-------
		INPUT : int or Path
			If the input source is determined to be a:
			- webcam, return the integer ID of the webcam.
			- video, validate that the file exists. Does not confirm the file type.

		Raises
		------
		ValueError
			- If an incorrect input source  set.
			- If an input video  but not set.
			- If an input video  but set to an invalid path.
		"""
		if self.INPUT_SOURCE == _Settings.InputType.WEBCAM:
			INPUT = self.WEBCAM_ID
		elif self.INPUT_SOURCE == _Settings.InputType.VIDEO:
			# Technically redundant
			if not self.TESTING_VID:
				raise ValueError("`TESTING_VID` setting not set.")
			if not self.TESTING_VID.exists():
				raise ValueError(f"File not found: {self.TESTING_VID}")

			INPUT = self.TESTING_VID
		else:
			raise ValueError(
				f"INPUT_SOURCE somehow set to invalid value. {self.INPUT_SOURCE}"
			)

		return INPUT


SETTINGS = _Settings()
"""All of the project settings.

Module level singleton pattern.
"""

# Verify the input video if it is chosen.
if SETTINGS.INPUT_SOURCE == SETTINGS.InputType.VIDEO:
	if not SETTINGS.TESTING_VID:
		raise ValueError("`TESTING_VID` setting not set.")
	if not SETTINGS.TESTING_VID.exists():
		raise ValueError(f"File not found: {SETTINGS.TESTING_VID}")
