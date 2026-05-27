"""Typing implementation details."""

from collections.abc import Callable as _Callable

import cv2 as _cv2


type Frame = _cv2.typing.MatLike
"""A frame is a single image from a webcam or video input."""

type ProcessFrameCallback = _Callable[[Frame], Frame | None]
"""A function that is called on a frame input for processing of some kind. Can return a modified result."""
