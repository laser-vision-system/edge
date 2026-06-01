"""A Tkinter user interface to test video output."""

from edge.settings import SETTINGS
from edge.tcam_interface import Camera

from ._user_interface import UserInterface


video_feed = Camera(capture_location=SETTINGS.input_source(), frametime=1.0 / 60)

app = UserInterface(
	width=SETTINGS.WINDOW_WIDTH,
	height=SETTINGS.WINDOW_HEIGHT,
	title="Testing Display",
	video_feed=video_feed,
)

app.start()
