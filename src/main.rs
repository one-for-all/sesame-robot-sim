use std::time::Instant;

use esp32rs::{log::init_logging, plot::plot, util::read_file};
use gorilla_physics::{hybrid::control::NullArticulatedController, types::Float};
use nalgebra::Vector;
use sesame::{
    builder::{build_arms, build_sesame},
    control::{SesameESP32Controller, pid::SesameServoController, servo::MG90SController},
    mesh::URDFMeshes,
};

#[tokio::main]
async fn main() {
    let _guard = init_logging();

    // let mut state = build_arms();
    let mut meshes = URDFMeshes::empty();
    let urdf_path = "onshape/robot.urdf";
    let urdf_file = read_file(urdf_path);
    let urdf_robot = urdf_rs::read_from_string(&urdf_file).unwrap();
    let mut state = build_sesame(&mut meshes, &urdf_robot);

    let controller = SesameESP32Controller::new().await;
    // let controller = SesameServoController::new();
    // let mut controller = MG90SController::new();
    // controller.set_target((90. as Float).to_radians());
    state.set_controller(0, controller);

    let mut data = vec![];
    let mut data2: Vec<Float> = vec![];

    let dt = 1. / (50. * 60.);
    let t_final = 1.0;
    let num_steps = (t_final / dt) as usize;

    let start = Instant::now();

    for s in 0..num_steps {
        state.step(dt, &vec![]);
        data.push(state.articulated[0].q()[0]);
        // data2.push(state.controllers[0].debug_data());
    }

    let duration = start.elapsed();

    println!("{}", state.controllers[0].get_uart());

    println!("angle: {}", state.articulated[0].q()[0]);
    plot(&data, dt, "arm angle");
    // plot(&data2, dt, "pin");

    // state.controllers[0].debug();
    println!("Time taken: {:?}", duration);
}
