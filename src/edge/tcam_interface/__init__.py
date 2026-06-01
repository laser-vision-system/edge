"""Provides connection and configuration for connected cameras.

Intended functionality:

- On start-up, identify all connected cameras.
- Detect when a new camera is connected.
- Detect when a camera disconnects.
- Create a connection to a camera.
- Read input from the camera.
- Provide details about connected camera configurations.
- Apply provided configurations.
"""

from ._camera import Camera as Camera
