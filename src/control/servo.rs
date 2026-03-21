use esp32rs::{Float, PI, servo::MG90S};
use gorilla_physics::hybrid::{articulated::Articulated, control::ArticulatedController};
use nalgebra::{DVector, dvector};

pub struct MG90SController {
    pub mg90s: MG90S,
}

impl MG90SController {
    pub fn new() -> Self {
        let mg90s = MG90S::new();
        Self { mg90s }
    }

    pub fn set_target(&mut self, v: Float) {
        self.mg90s.command_angle = Some(v);
    }
}

impl ArticulatedController for MG90SController {
    fn step(&mut self, dt: Float, articulated: &Articulated) {}

    fn control(&mut self, articulated: &Articulated, input: &Vec<Float>) -> DVector<Float> {
        let dof = articulated.dof();
        let mut tau: DVector<Float> = DVector::zeros(dof);

        self.mg90s.angle = articulated.q()[0];
        self.mg90s.vel = articulated.v()[0];
        tau[0] = self.mg90s.torque();

        tau
    }
}
