"""Connection interface between the edge and a server."""

from ._client import EdgeClient as EdgeClient
from ._functions import (
	camera_info_post as camera_info_post,
	configuration_get as configuration_get,
	configuration_post as configuration_post,
	image_put as image_put,
	setpoint_get as setpoint_get,
)
from ._models import (
	CameraConfig as CameraConfig,
	CameraInfo as CameraInfo,
	Image as Image,
	Setpoint as Setpoint,
)
