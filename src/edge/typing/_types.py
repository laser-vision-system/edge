"""Typing implementation details."""

from collections.abc import Callable
from typing import Any

import cv2


type Frame = cv2.typing.MatLike
"""A frame is a single image from a webcam or video input."""

type ProcessFrameCallback = Callable[[Frame], Frame | None]
"""A function that is called on a frame input for processing of some kind.

Can return a modified result."""
