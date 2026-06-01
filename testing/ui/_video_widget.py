"""A tkinter widget that displays the output video."""

import tkinter as _tk
from tkinter import ttk as _ttk

import cv2 as _cv
from PIL.Image import fromarray as _np2pil
from PIL.ImageTk import PhotoImage as _Image

from edge.tcam_interface import Camera
from edge.typing import Frame

from ..video_feed import VideoFeed


class VideoWidget:
	active: bool = False

	def __init__(self, parent_frame: _ttk.Frame, input: Camera | VideoFeed) -> None:
		self.parent: _ttk.Frame = parent_frame
		self.input: Camera | VideoFeed = input

		# Access the root window through the parent frame.
		top_level: _tk.Tk | _tk.Toplevel = self.parent.winfo_toplevel()
		if isinstance(top_level, _tk.Toplevel):
			raise ValueError
		self.root: _tk.Tk = top_level

		# Initialise with the width and height of the parent frame.
		self.target_width: int = self.parent.winfo_width()
		self.target_height: int = self.parent.winfo_height()

		self.label_widget = _ttk.Label(master=self.parent, padding=(0, 0, 0, 0))
		# No need to use expand or fill options.
		self.label_widget.pack(anchor="center")

		self.current_image: _Image | None = None

		# Bind the configure event to detect when the parent frame changes size.
		self.parent.bind(sequence="<Configure>", func=self.on_resize)

	def on_resize(self, event: _tk.Event) -> None:
		"""Update target dimensions based on the new size of the parent frame.

		Normally, you'd want to keep this value above zero, but we already do that
		later so it's fine to leave it raw here.
		"""
		self.target_width = event.width
		self.target_height = event.height

	def restart_video(self) -> None:
		self.input.capture.set(propId=_cv.CAP_PROP_POS_FRAMES, value=0)
		self.start_playback()

	def _resize_frame(self, frame: Frame) -> Frame:
		"""Resize the provided frame to the available space."""
		# Retrieve original video dimensions.
		orig_width: int
		orig_height: int
		orig_height, orig_width = frame.shape[:2]

		if orig_width < 1 or orig_height < 1:
			raise ValueError("Input frame cannot have a side length of 0 or less")

		# Calculate scaling factor to fit within the target dimensions.
		scale_width: float = self.target_width / orig_width
		scale_height: float = self.target_height / orig_height
		# Use min() to ensure it fits without cropping.
		scale: float = min(scale_width, scale_height)

		new_width: int = max(1, int(orig_width * scale))
		new_height: int = max(1, int(orig_height * scale))

		if new_width < 1 or new_height < 1:
			raise ValueError("Cannot resize to a side length of 0 or less")

		# Resize proportionally, width comes before height.
		return _cv.resize(src=frame, dsize=(new_width, new_height))

	def _next_frame(self) -> None:
		self.active = False
		self.start_playback()

	def start_playback(self) -> None:
		# Repetitive call protection
		if self.active:
			return
		self.active = True

		try:
			if isinstance(self.input, Camera):
				frame: Frame = self.input.get_frame()
			else:
				frame: Frame = self.input.process_frame()
		except ValueError:
			# When video ends, set it back to the start.
			self.restart_video()
			self.active = False
			return

		# BUG This only occurs while the video is playing.
		frame = self._resize_frame(frame=frame)

		# Convert to correct type and apply.
		self.current_image = _Image(
			image=_np2pil(obj=_cv.cvtColor(src=frame, code=_cv.COLOR_BGR2RGB))
		)
		self.label_widget.configure(image=self.current_image)

		FRAMETIME_MS: int = int(self.input.frametime * 1000)
		self.root.after(ms=FRAMETIME_MS, func=self._next_frame)
