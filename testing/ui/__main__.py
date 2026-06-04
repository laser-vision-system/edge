"""A Tkinter user interface to test video output."""

from tkinter.ttk import Label

##from edge.tcam_interface import Camera
from edge.settings import SETTINGS
from edge.typing import Frame
from edge.vision_processor import Demonstration

from ..video_feed import VideoFeed
from ._user_interface import UserInterface


WINDOW_WIDTH = 1000
"""The starting width of the window."""
WINDOW_HEIGHT = 500
"""The starting height of the window."""


display: Label


def apply_demonstratation(frame: Frame) -> Frame:
	results: tuple[float, float] = Demonstration.process_frame(
		frame=frame, setpoint=False
	)
	display.configure(text=str(results))
	return frame


video_feed = VideoFeed(
	capture_location=SETTINGS.input_source(),
	callback=apply_demonstratation,
	frametime=1.0 / 60,
)

app = UserInterface(
	width=WINDOW_WIDTH,
	height=WINDOW_HEIGHT,
	title="Testing Display",
	video_feed=video_feed,
)

display = app.add_display()

app.start()
