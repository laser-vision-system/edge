"""Connection interface between the edge and a server."""

from ._client import EdgeClient as EdgeClient
from ._endpoints import (
	EndpointCameraInfo as EndpointCameraInfo,
	EndpointConfiguration as EndpointConfiguration,
	EndpointGUIUpdate as EndpointGUIUpdate,
	EndpointImage as EndpointImage,
)
