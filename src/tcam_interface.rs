//! # Tcam Interface
//!
//! Provides connection and configuration for connected cameras.
//!
//! ## Intended functionality
//!
//! - On start-up, identify all connected cameras.
//! - Detect when a new camera is connected.
//! - Detect when a camera disconnects.
//! - Create a connection to a camera.
//! - Read input from the camera.
//! - Provide details about connected camera configurations.
//! - Apply provided configurations.
//!
//! ## Sub-modules
//!
//! ### [`camera`]
//!
//! A module containing a representational object for a single camera.
//!
//! ### [`controls`]
//!
//! A module containing configuration options available to the camera.
//!
//! ### Detection - expected
//!
//! A module containing functions to detect newly connected cameras.
//!
//! These can be called at startup.

// Maybe private the mods and re-export the internals.
pub mod camera;
pub mod controls;
