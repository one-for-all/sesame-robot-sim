use std::{f64::consts::PI, vec};

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
        let mut torques = vec![]; // 0 torques for floating body
        let body_dof = articulated.joints[0].dof();
        for _ in 0..body_dof {
            torques.push(0.);
        }

        let qs = articulated.q();
        let vs = articulated.v();

        let targets = [
            (135. as Float).to_radians(),
            (45. as Float).to_radians(),
            (45. as Float).to_radians(),
            (135. as Float).to_radians(),
            (0. as Float).to_radians(),
            (180. as Float).to_radians(),
            (0. as Float).to_radians(),
            (180. as Float).to_radians(),
        ];
        for i in 0..8 {
            let q;
            let v;
            if body_dof == 0 {
                q = qs[i];
                v = vs[i];
            } else {
                q = qs[body_dof + 1 + i];
                v = vs[body_dof + i];
            }

            let target = targets[i]; // PI / 4.; // Fixed 45 degree target
            let diff = target - q;

            let scale_factor = MG90S_MAX_TORQUE / MG90S_MAX_SPEED;
            let torque = if diff > 0. {
                let kd = if v > 0. {
                    v.min(MG90S_MAX_SPEED) * scale_factor
                } else {
                    0.
                };
                MG90S_MAX_TORQUE - kd
            } else {
                let kd = if v < 0. {
                    v.max(-MG90S_MAX_SPEED) * scale_factor
                } else {
                    0.
                };
                -MG90S_MAX_TORQUE - kd
            };
            // Note: artificially scale down servo torque.
            // TODO: fix servo torque constant?

            torques.push(torque);
        }

        DVector::from_vec(torques)
    }
}
