use esp32rs::{log::init_logging, plot::plot};
use gorilla_physics::hybrid::control::NullArticulatedController;
use sesame::{
    builder::build_sesame,
    control::{SesameESP32Controller, pid::SesameServoController},
};

#[tokio::main]
async fn main() {
    let _guard = init_logging();

    let mut state = build_sesame();

    let controller = SesameESP32Controller::new().await;
    // let controller = SesameServoController::new();
    state.set_controller(0, controller);

    let mut data = vec![];
    let mut data2 = vec![];

    let dt = 1e-4;
    let t_final = 1.0;
    let num_steps = (t_final / dt) as usize;
    for s in 0..num_steps {
        state.step(dt, &vec![]);
        data2.push(state.controllers[0].debug_data());
        data.push(state.articulated[0].q()[0]);
    }

    println!("angle: {}", state.articulated[0].q()[0]);
    plot(&data, dt, "arm angle");
    plot(&data2, dt, "pin");
}
