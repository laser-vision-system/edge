"""This modules provides the :func:`Camera` class.

This module is not meant to be imported directly.
"""

import time
from pathlib import Path

import cv2
from linuxpy.video.device import Device, MenuControl, BooleanControl, IntegerControl

from edge.exceptions import CaptureOpenError, FrameReadError
from edge.typing import Frame
from edge.models import ConfigurationModel


class Camera:
	"""Representational object for a single camera.

	Parameters
	----------
	capture_location : int | str | Path
		What the video source is. The intended use is with a webcam input,
		however a video file input can be supplied for testing.

		- For a webcam input, use an `int`, 0 is the default webcam.
		- For a video file, pass in a file location either as `str` or `Path`
		(preferred).

	frametime : float, default=1.0/30.0
		In seconds, how long between each frame. Use `1 / fps` if you want to pass in
		a framerate.

		NOTE This may need to change when we know more about configuration.
	"""

	def __init__(
		self,
		capture_location: int | str | Path,
		frametime: float = 1.0 / 30.0,
	) -> None:
		"""Initialiser dunder.

		Calls when a new instance is constructed.
		"""

		if isinstance(capture_location, int):  # Webcam input
			if capture_location < 0:
				raise ValueError("Integer input must be a natural number.")
		else:  # Filepath input
			# Convert string to Path object for easier operation.
			if isinstance(capture_location, str):
				capture_location = Path(capture_location)

			if not capture_location.exists() or capture_location.is_dir():
				raise FileNotFoundError("No file was found at this location.")

		self.capture = cv2.VideoCapture(capture_location)
		if not self.capture.isOpened():
			raise CaptureOpenError("Failed to open camera.")

		self.capture_location: int | Path = capture_location
		self.frametime: float = frametime

	def __del__(self) -> None:
		"""Deletion dunder.

		Called when an instance is deleted.
		"""

		self.capture.release()

	def reconnect(self, attempts: int = 5, delay: float = 2.0) -> bool:
		"""Attempts to reconnect if the camera has disconnected.

		Parameters
		-------
		attempts : int, default = 5
			The number of reconnect attempts to be made

		delay : float, default = 2.0
			The time between reconnection attempts in seconds

		Returns
		-------
		bool: True if the camera connection is able to be restablished, false if not
		"""
		self.capture.release()
		for _ in range(attempts):
			self.capture.open(self.capture_location)
			if self.capture.isOpened():
				return True
			time.sleep(delay)
		return False

	def get_frame(self, retries: int = 3) -> Frame:
		"""Retrieves the most recent frame from the camera.

		Returns
		-------
		:func:`Frame`
			The current image from the capture.

		Raises
		------
		RuntimeError
			- If the camera diconnected
			- If the capture failed to read

		Notes
		-----
		We need to handle a disconnected camera gracefully.
		"""

		if not self.capture.isOpened():
			if not self.reconnect():
				raise CaptureOpenError("Camera disconnected and could not reconnect.")

		ret: bool
		frame: Frame

		for _ in range(retries):
			ret, frame = self.capture.read()
			if ret and frame is not None and frame.size > 0:
				return frame

		raise FrameReadError(f"Frame capture failed after {retries} attempts.")
	
	def detect_configurations(self):
		"""
		Retrieves the writable configurations of a camera.

		Returns
		-------
		dict
			A dictionary of the writable controls of the camera, with the control
			name as the key and a dictionary of the control properties as the value.

			The control property can be on of three types: bool, int, or enumerate. Each type has different properties:

			bool
				- value: The current value of the control
				- default: The default value of the control
				- inactive: Whether the control is currently inactive (true or false)


			int
			    - min: The minimum value of the control
			    - max: The maximum value of the control
			    - step: The step size of the control
			    - value: The current value of the control
			    - default: The default value of the control
				- inactive: Whether the control is currently inactive (true or false)


			enumerate
			    - items: A dictionary of the possible values of the control, with the index as the key and the name as the value
			    - value: The current value of the control
			    - default: The default value of the control
				- inactive: Whether the control is currently inactive (true or false)
		"""
		if self.capture_location is not int:
			raise RuntimeWarning("Could not probe camera: Camera location was not an integer.")
			return {}

		with Device.from_id(self.capture_location) as cam:
			if cam is None:
				raise CaptureOpenError("Camera could not be found.")
			
			if cam.controls is None:
				print("Camera has no accessible controls.")
				return {}
			
			try:
				ctrls = []
				for c in list(cam.controls.values()):
					ctrls.append(c)

			except Exception as e:
				print(f"Could not read controls: {e}")
				
			
			controls = {}
			for c in ctrls:
				inactive = False
				if c.flags == 16:
					"""This is a bit of a hack, but it seems that when a control is inactive, it has a flag value of 16.
					There may be other flags but I haven't come across any yet."""
					inactive = True

				#TODO perhaps put these below three functions into a helper function
				if isinstance(c, BooleanControl):
					print(f"Name: {c.name} - default = {c.default}")
					controls[c.name] = {
						"type": bool,
						"value": c.value,
						"default": c.default
					}
					controls[c.name]["inactive"] = inactive
				
				if isinstance(c, IntegerControl):
					print(f"Name: {c.name} - Min = {c.minimum} - Max = {c.maximum}) - Step size = {c.step} - default = {c.default}")
					controls[c.name] = {
						"type": int,
						"min": c.minimum,
						"max": c.maximum,
						"step": c.step,
						"value": c.value,
						"default": c.default
					}
					controls[c.name]["inactive"] = inactive
				
				if isinstance(c, MenuControl):
					items = {}
					for (index, name) in c.items():
						items[index] = name

					controls[c.name] = {
						"type": enumerate,
						"items": items,
						"value": c.value,
						"default": c.default
					}
					controls[c.name]["inactive"] = inactive


		"""Code for MVP demonstration purposes"""
		demo_controls = ConfigurationModel(
			brightness=controls["Brightness"]["value"],
			hue=controls["Hue"]["value"],
			saturation=controls["Saturation"]["value"]
		)

		return demo_controls
	
	def apply_configurations(self, configs: ConfigurationModel) -> None:
		if self.capture_location is not int:
			raise RuntimeWarning("Could not probe camera: Camera location was not an integer.")
			return {}
		
		with Device.from_id(self.capture_location) as cam:
			if cam is None:
				raise CaptureOpenError("Camera could not be found.")
			
			if cam.controls is None:
				print("Could not access camera controls.")
				return
			
			for config_name, config_value in configs.model_dump().items():
				try:
					cam.controls[config_name].value = config_value
					print(f"Set {config_name} to {config_value}")
				except Exception as e:
					print(f"Could not set {config_name} to {config_value}: {e}")