"""Provides connection and configuration for connected cameras.

Intended functionality
----------------------
- On start-up, identify all connected cameras.
- Detect when a new camera is connected.
- Detect when a camera disconnects.
- Create a connection to a camera.
- Read input from the camera.
- Provide details about connected camera configurations.
- Apply provided configurations.

Expected modules
----------------
Camera
	A module containing a representational object for a single camera.
Configurations
	A module containing configuration options available to the camera.

	Perhaps separated by the what types of cameras they can be applied to.
Detection
	A module containing functions to detect newly connected cameras.

	These can be called at startup.
"""

from ._camera import Camera as Camera
