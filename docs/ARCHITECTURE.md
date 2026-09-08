# Edge Architecture

## Programming Language

For the programming language, we are using Rust due to its performance and memory safety. This crucially adheres to the quality requirement of reliability.

## Rust libraries (crates)

### Image

https://crates.io/crates/image

This crate provides methods to load, save, and manipulate image data.

### Nokhwa

https://crates.io/crates/nokhwa

This crates provides platform agnostic access to video input devices.

### Reqwest

https://crates.io/crates/reqwest

This crate provides an implementation of a http client.

### Tungstenite

https://crates.io/crates/tungstenite

This crate provides an implementation of websockets.

There are also asynchronous version:

- https://crates.io/crates/tokio-tungstenite
- https://crates.io/crates/async-tungstenite

### Serde

https://crates.io/crates/serde

This crate provides simple and easy methods of serialising and deserialising data for transmission over the network.

## Camera Video Formats

Ideally, we can use either YUV or YUV2 for our video format, which most cameras we looked at had the option for. If that option for some reason is not available RBG565 is a solid low bandwidth alternative.
