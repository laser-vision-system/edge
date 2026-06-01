"""The main window."""

import tkinter as tk
from tkinter import ttk

from edge.settings import SETTINGS
from edge.tcam_interface import Camera

from ..video_feed import VideoFeed
from ._button import Button
from ._video_widget import VideoWidget


class UserInterface:
	"""Simple user interface build with Tkinter.

	Parameters
	----------
	width : int
		How many pixels wide is the window.
	height : int
		How many pixels tall is the window.
	title : str, optional
		The title of the window.
	video_feed : :func:`VideoFeed`, optional
		A video feed to display in the window.
	"""

	def __init__(
		self,
		width: int,
		height: int,
		title: str | None = None,
		video_feed: Camera | VideoFeed | None = None,
	) -> None:

		if width < 1 or height < 1:
			raise ValueError("Window cannot have a side length of 0 or less")

		self.window = tk.Tk()

		if title is not None:
			self.window.title(title)

		self.window.geometry(f"{width}x{height}")

		# --- Main frame ---
		self.mainframe = ttk.Frame(self.window)
		self.mainframe.pack(expand=True, fill=tk.BOTH)

		self.mainframe.rowconfigure(index=0, weight=1)
		self.mainframe.columnconfigure(index=0, weight=1)
		self.mainframe.columnconfigure(index=1, weight=0)

		# --- Video ---
		self.video_container = ttk.Frame(self.mainframe)
		self.video_container.grid(row=0, column=0, sticky="news")
		self.video_container.pack_propagate(False)

		# --- Input panel ---
		self.input_panel = ttk.Frame(master=self.mainframe, padding=10)
		self.input_panel.grid(row=0, column=1, sticky="ns")

		# --- Title ---
		self.title_label = ttk.Label(
			master=self.input_panel,
			text="Controls",
			font=("Arial", 14, "bold"),
		)
		self.title_label.grid(row=0, column=0, pady=10)

		if video_feed is not None:
			autostart: bool = SETTINGS.INPUT_SOURCE == SETTINGS.InputType.WEBCAM
			self.add_feed(video_feed=video_feed, auto_start=autostart)

	def add_feed(
		self, video_feed: Camera | VideoFeed, auto_start: bool = False
	) -> None:

		if hasattr(self, "video_widget"):
			raise ValueError("Feed already added.")

		self.video_widget: VideoWidget = VideoWidget(
			parent_frame=self.video_container, input=video_feed
		)

		if SETTINGS.INPUT_SOURCE == SETTINGS.InputType.VIDEO:
			self.restart_button: Button = Button(
				input_panel=self.input_panel,
				label="Start",
				order=5,
				command=self.video_widget.restart_video,
			)

		if auto_start:
			self.video_widget.start_playback()

	def start(self) -> None:
		self.window.mainloop()
