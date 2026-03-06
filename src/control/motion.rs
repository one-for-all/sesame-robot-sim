use esp32rs::servo::{MG90S_MAX_SPEED, MG90S_MAX_TORQUE};
use gorilla_physics::{
    hybrid::{articulated::Articulated, control::ArticulatedController},
    types::Float,
};
use nalgebra::{DVector, dvector};

#[derive(Clone, Copy)]
pub struct Servo {
    pub q: Float,
    pub v: Float,

    pub target: Float,
}

impl Servo {
    pub fn new() -> Self {
        Self {
            q: 0.,
            v: 0.,
            target: 0.,
        }
    }

    pub fn set_target(&mut self, target: Float) {
        self.target = target;
    }

    pub fn torque(&self) -> Float {
        let diff = self.target - self.q;
        let v = self.v;
        let tol = 1e-2;

        let scale_factor = MG90S_MAX_TORQUE / MG90S_MAX_SPEED;
        let torque = if diff.abs() <= tol {
            0.
        } else if diff > 0. {
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
        torque
    }
}

/// Sesame motion sequence controller
pub struct SesameMotionController {
    pub servos: [Servo; 8],
}

impl SesameMotionController {
    pub fn new() -> Self {
        Self {
            servos: [Servo::new(); 8],
        }
    }
}

impl ArticulatedController for SesameMotionController {
    fn step(&mut self, dt: Float, articulated: &Articulated) {}

    fn control(&mut self, articulated: &Articulated, input: &Vec<Float>) -> DVector<Float> {
        let mut torques = vec![]; // 0 torques for floating body
        let body_dof = articulated.joints[0].dof();
        for _ in 0..body_dof {
            torques.push(0.);
        }
        let qs = articulated.q();
        let vs = articulated.v();
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
            self.servos[i].q = q;
            self.servos[i].v = v;
        }

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
            self.servos[i].set_target(targets[i]);
            torques.push(self.servos[i].torque());
        }

        DVector::from_vec(torques)
    }
}
