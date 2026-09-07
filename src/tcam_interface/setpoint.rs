use image::RgbImage;

pub struct EmptySetpoint {
	pub intensity: Option<f32>,
}

impl EmptySetpoint {
	fn new() -> Self {
		Self { intensity: None }
	}
}

pub struct CnnSetpoint {
	pub intensity: Option<f32>,
}

impl CnnSetpoint {
	fn new() -> Self {
		Self { intensity: None }
	}
}

pub struct DiffSetpoint {
	pub image: Option<RgbImage>,
	pub peak: Option<f32>,
	pub mean: Option<f32>,
	pub std_dev: Option<f32>,
}

impl DiffSetpoint {
	fn new() -> Self {
		Self {
			image: None,
			peak: None,
			mean: None,
			std_dev: None,
		}
	}
}

pub struct DemonstrationSetpoint {
	pub intensity: Option<f32>,
}

impl DemonstrationSetpoint {
	fn new() -> Self {
		Self { intensity: None }
	}
}

pub struct Setpoint {
	pub empty_setpoint: EmptySetpoint,
	pub diff_setpoint: DiffSetpoint,
	pub cnn_setpoint: CnnSetpoint,
	pub demonstration_setpoint: DemonstrationSetpoint,
	pub should_calibrate: bool,
}

impl Setpoint {
	fn new() -> Self {
		Self {
			empty_setpoint: EmptySetpoint::new(),
			diff_setpoint: DiffSetpoint::new(),
			cnn_setpoint: CnnSetpoint::new(),
			demonstration_setpoint: DemonstrationSetpoint::new(),
			should_calibrate: false,
		}
	}

    fn reset(&mut self) {
        *self = Self::new();
    }
}
