use std::f64::consts::PI;

use esp32rs::servo::{MG90S_MAX_SPEED, MG90S_MAX_TORQUE};
use gorilla_physics::{
    hybrid::{articulated::Articulated, control::ArticulatedController},
    types::Float,
};
use nalgebra::{DVector, dvector};

/// Mimic servo using a PID-like controller
pub struct SesameServoController {}

impl SesameServoController {
    pub fn new() -> Self {
        Self {}
    }
}

impl ArticulatedController for SesameServoController {
    fn step(&mut self, dt: Float, articulated: &Articulated) {}

    fn control(&mut self, articulated: &Articulated, input: &Vec<Float>) -> DVector<Float> {
        let angle = articulated.q()[0];
        let omega = articulated.v()[0];

        let target = PI / 2.; // Fixed 90 degree target
        let diff = target - angle;

        let scale_factor = MG90S_MAX_TORQUE / MG90S_MAX_SPEED;
        let torque = if diff > 0. {
            let kd = if omega > 0. {
                omega.min(MG90S_MAX_SPEED) * scale_factor
            } else {
                0.
            };
            MG90S_MAX_TORQUE - kd
        } else {
            let kd = if omega < 0. {
                omega.max(-MG90S_MAX_SPEED) * scale_factor
            } else {
                0.
            };
            -MG90S_MAX_TORQUE - kd
        };

        dvector![torque]
    }
}
