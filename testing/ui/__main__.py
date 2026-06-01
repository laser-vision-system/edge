"""A Tkinter user interface to test video output."""

from tkinter.ttk import Label

from edge.settings import SETTINGS
from edge.tcam_interface import Camera
from edge.typing import Frame
from edge.vision_processor import Demonstration

from ..video_feed import VideoFeed
from ._user_interface import UserInterface


display: Label


def apply_demonstratation(frame: Frame) -> Frame:
	val: float = Demonstration.process_frame(frame)
	display.configure(text=str(val))
	return frame


video_feed = VideoFeed(
	capture_location=SETTINGS.input_source(),
	callback=apply_demonstratation,
	frametime=1.0 / 60,
)

app = UserInterface(
	width=SETTINGS.WINDOW_WIDTH,
	height=SETTINGS.WINDOW_HEIGHT,
	title="Testing Display",
	video_feed=video_feed,
)

display = app.add_display()

app.start()
