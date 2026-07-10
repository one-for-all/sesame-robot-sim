use esp32rs::util::read_file;
use gorilla_physics::hybrid::mesh::URDFMeshes;
use sesame::{builder::build_sesame, control::SesameESP32Controller};

#[tokio::main]
async fn main() {
    let mut meshes = URDFMeshes::empty();
    let urdf_path = "onshape/robot.urdf";
    let urdf_file = read_file(urdf_path);
    let urdf_robot = urdf_rs::read_from_string(&urdf_file).unwrap();
    let mut state = build_sesame(&mut meshes, &urdf_robot);

    let controller = SesameESP32Controller::new().await;
    state.set_controller(0, controller);

    let dt = 1. / (50. * 60.);
    let mut t: f64 = 0.;

    // Send the command BEFORE the firmware finishes booting
    state.controllers[0].send_uart("rn st\n");
    println!("sent rn st at t=0 (before boot)");

    // Boot until firmware prints its banner
    while t < 60. {
        state.step(dt, &vec![]);
        t += dt;
        if (t / dt) as usize % 3000 == 0 {
            let uart = state.controllers[0].get_uart();
            println!("t={:.2} uart_len={}", t, uart.len());
            if uart.contains("Sesame Robot Simulation") {
                break;
            }
        }
    }
    println!("=== boot done at t={:.2} ===", t);

    // Stress: send commands at pseudo-random intervals to sample many
    // firmware phases (mid-motion, mid-print, mid-ISR, ...)
    let cmds = ["rn st\n", "rn wf\n", "rn rs\n", "rn wv\n"];
    let mut seed: u64 = 0x12345678;
    let mut rand = || {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (seed >> 33) as usize
    };

    for i in 0..300 {
        let cmd = cmds[rand() % cmds.len()];
        state.controllers[0].send_uart(cmd);
        // random wait between 1 and ~400 physics steps
        let n_wait = 1 + rand() % 400;
        for _ in 0..n_wait {
            state.step(dt, &vec![]);
            t += dt;
        }
        if i % 20 == 0 {
            println!("iter={} t={:.2} last_cmd={:?}", i, t, cmd.trim());
        }
    }

    let uart = state.controllers[0].get_uart();
    println!("=== final uart tail ===\n{}", &uart[uart.len().saturating_sub(2000)..]);
    println!("done, no panic");
}
