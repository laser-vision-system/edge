"""This modules provides the pydantic models.

This module is not meant to be imported directly.
"""

from pydantic import BaseModel


class CameraInfo(BaseModel):
	intensity: float
	deviation: float


class CameraConfig(BaseModel):
	brightness: float
	framerate: float


class Setpoint(BaseModel):
	should: bool


class Image(BaseModel):
	# TODO Check type.
	image: bytes
