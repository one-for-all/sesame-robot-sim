use esp32rs::plot::plot;
use gorilla_physics::hybrid::control::NullArticulatedController;
use sesame::{
    builder::build_sesame,
    control::{SesameESP32Controller, pid::SesameServoController},
};

#[tokio::main]
async fn main() {
    let mut state = build_sesame();

    // let controller = SesameESP32Controller::new().await;
    let controller = SesameServoController::new();
    state.set_controller(0, controller);

    let mut data = vec![];
    let mut data2 = vec![];

    let dt = 1e-3;
    for s in 0..2000 {
        state.step(dt, &vec![]);
        data2.push(state.controllers[0].debug_data());
        data.push(state.articulated[0].q()[0]);
    }

    println!("angle: {}", state.articulated[0].q()[0]);
    plot(&data, dt, "arm angle");
    plot(&data2, dt, "pin");
}
