use api::shared::{CameraId, ConnId};

pub enum CameraRequest{
    SetNameCamera(String),
    StartViewingCamera,
    StopViewingCamera,
    Configuration(String),
    ViewCameraConfig(ConnId),
    Setpoint,
}

pub enum PersistenceRequest {
    ForceRetryCamera(CameraId),
    ForgetCameraRequest(ConnId, CameraId),
}

