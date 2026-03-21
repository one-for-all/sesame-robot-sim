use esp32rs::{
    Float, PI,
    plot::plot,
    servo::{MG90S, MG90S_MAX_TORQUE},
};
use gorilla_physics::{
    WORLD_FRAME,
    hybrid::{Hybrid, Rigid, articulated::Articulated, control::ArticulatedController},
    joint::Joint,
    spatial::transform::Transform3D,
};
use nalgebra::{DVector, Vector3, dvector};

struct MG90SController {
    pub mg90s: MG90S,
}

impl MG90SController {
    pub fn new() -> Self {
        let mut mg90s = MG90S::new();
        mg90s.command_angle = Some(PI / 2.);
        Self { mg90s }
    }
}

impl ArticulatedController for MG90SController {
    fn step(&mut self, dt: Float, articulated: &Articulated) {}

    fn control(&mut self, articulated: &Articulated, input: &Vec<Float>) -> DVector<Float> {
        self.mg90s.angle = articulated.q()[0];
        self.mg90s.vel = articulated.v()[0];

        dvector![self.mg90s.torque()]
    }
}

fn main() {
    let mut state = Hybrid::empty();

    let m = 0.01; // 10 gram
    let w = 0.02; // 2 cm
    let horn_frame = "horn";
    let horn = Rigid::new_cuboid(m, w, w / 2., w / 10., horn_frame);
    let horn_joint = Joint::new_revolute(
        Transform3D::identity(horn_frame, WORLD_FRAME),
        Vector3::z_axis(),
    );
    let articulated = Articulated::new(vec![horn], vec![horn_joint]);
    state.add_articulated(articulated);

    let servo_controller = MG90SController::new();
    state.set_controller(0, servo_controller);

    let mut data = vec![];

    let dt = 1. / (50. * 60.);
    let t_final = 1.0;
    let num_steps = (t_final / dt) as usize;
    for s in 0..num_steps {
        state.step(dt, &vec![]);
        data.push(state.articulated[0].q()[0]);
    }

    plot(&data, dt, "servo sim");
}
