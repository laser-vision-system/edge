"""This modules provides the pydantic models.

This module is not meant to be imported directly.
"""

from pydantic import BaseModel


class CameraInfoModel(BaseModel):
	intensity: float
	deviation: float


class GUIUpdateModel(BaseModel):
	setpoint: bool
	config: bool


class CameraControlModel(BaseModel):
	type: type
	inactive: bool 


class IntegerControlModel(CameraControlModel):
	min: int
	max: int
	step: int
	default: int
	value: int


class BooleanControlModel(CameraControlModel):
	default: bool
	value: bool


class MenuControlModel(CameraControlModel):
	items: dict[int, str]
	default: int
	value: int


class ConfigurationModel(BaseModel):
	# brightness: float
	# framerate: float
	controls: dict[str, CameraControlModel]