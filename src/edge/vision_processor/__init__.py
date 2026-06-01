"""Vision processing.

The purpose of the Vision processor is to intake a frame, provide an interface to
perform machine vision analysis, and then pass the results to the Edge Client.
"""

from ._CNN import CNN as CNN
from ._demonstration import Demonstration as Demonstration
from ._image_analysis import ImageAnalysis as ImageAnalysis
from ._vision_interface import VisionInterface as VisionInterface
