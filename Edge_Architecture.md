# Edge Architecture

## Programming Language for video processing and analysis

For a programming language, we can use Python with OpenCV and NumPy for quick image processing and machine vision. As well as FastAPI for communication with the server.

If we find that python is too resource intensive, we can go with a lower-level approach using Rust as it provides a better build system with cargo and has built in memory safety when compared to C++, it also has OpenCV bindings.

## Python Libraries Used

### OpenCV

https://opencv.org/ 

Very promising machining vision library with image processing capabilities.

### NumPy

https://pypi.org/project/numpy/ 

Allows faster processing of information when compared to python list or array, as it provides an N-dimensional array pre complied with no interpreter overhead to slow down the code.

### FastAPI

https://fastapi.tiangolo.com/

For receiving and sending output and input to the server. 

## Data Recording

For data recording we use json files as the data is being read by machines and not humans.

## Camera Video Formats

Ideally, we can use either YUV or YUV2 for our video format, which most cameras we looked at had the option for. If that option for some reason is not available RBG565 is a solid low bandwidth alternative. 




