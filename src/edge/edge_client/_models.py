"""This modules provides the pydantic models.

This module is not meant to be imported directly.
"""

from pydantic import BaseModel


class CameraInfoModel(BaseModel):
	intensity: float
	deviation: float


class ConfigurationModel(BaseModel):
	brightness: float
	framerate: float


class GUIUpdateModel(BaseModel):
	setpoint: bool
	config: bool


class ImageModel(BaseModel):
	image: bytes
