use api::shared::ConnId;

pub enum CameraRequest{
    SetNameCamera(String),
    StartViewingCamera,
    StopViewingCamera,
    Configuration(String),
    ViewCameraConfig(ConnId),
    Setpoint,
    ForceRetryCamera,
    ForgetCameraRequest(ConnId),
}


