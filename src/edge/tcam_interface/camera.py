'''
This document provides the functions for a camera to:
- Read a frame
- Retrieve current settings
- Write settings

Functions:
- open()
- close()
- get_frame()
'''

import cv2

class Camera:
	# Index is the index of the camera, defaults to 0
	def __init__(self, index=0):
		self.index = index
		self.capture = None

	# Function: open()
	# No params
	# Directly accesses the camera
	# Returns self for easy chaining
	def open(self):
		self.capture = cv2.VideoCapture(self.index)

		if not self.capture.isOpened():
			raise RuntimeError("Failed to open camera")
		
		return self
	
	# Function: close()
	# No params
	# Releases the camera
	# Void
	def close(self):
		if self.capture:
			self.capture.release()
			self.capture = None

	# Function: get_frame()
	# No params
	# Retrieves the most recent frame from the camera
	# Returns an array (image)
	def get_frame(self):
		if not self.capture.isOpened():
			raise RuntimeError("Camera is not open")
		
		flag, frame = self.caption.read()
		
		if not flag:
			raise RuntimeError("Frame capture failed")
		
		return frame