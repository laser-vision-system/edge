//! Enums defining the data that can be sent across threads.
use api::shared::{
    CameraId,
    ConnId,
};

/// Requests sent to individual cameras by the client thread.
pub enum CameraRequest {
	/// Sets the name of the camera.
	SetNameCamera(String),

	/// Tells the camera to start(with a value of true) or
	/// stop(with a value of false) sending images through the client.
	ToggleViewingCamera(bool),

	/// Updates the configuration of the camera.
	Configuration(String),

	/// Requests the configuration of the camera. The attached `ConnId` should be
	/// included when sending the configuration.
	ViewCameraConfig(ConnId),

	/// Tells the camera to take a setpoint in its analysis method.
	Setpoint,
}

/// Requests sent to the Persistence struct by the client thread.
pub enum PersistenceRequest {
	/// Indicates a camera should be set to active, regardless of current
	/// structure.
	ForceRetryCamera(CameraId),

	/// Indicates a camera should be removed from the persistence struct.
	ForgetCameraRequest(ConnId, CameraId),
}
