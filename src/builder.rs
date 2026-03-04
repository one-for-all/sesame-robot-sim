use gorilla_physics::{
    WORLD_FRAME,
    hybrid::{Hybrid, Rigid, articulated::Articulated, rigid},
    joint::Joint,
    na::vector,
    spatial::transform::Transform3D,
    types::Float,
};
use nalgebra::Vector3;

#[cfg(target_arch = "wasm32")]
use crate::control::{SesameESP32Controller, pid::SesameServoController};
#[cfg(target_arch = "wasm32")]
use gorilla_physics::interface::{hybrid::InterfaceHybrid, util::read_web_file_bytes};
#[cfg(target_arch = "wasm32")]
use web_sys::wasm_bindgen::prelude::wasm_bindgen;

pub fn build_sesame() -> Hybrid {
    let mut state = Hybrid::empty();

    // let arm_frame = "arm";
    // let m = 0.1;
    // let w = 0.1;
    // let arm = Rigid::new_cuboid_at(&vector![w / 2., 0., 0.], m, w, 0.02, 0.02, arm_frame);
    // let arm_joint = Joint::new_revolute(
    //     Transform3D::identity(arm_frame, WORLD_FRAME),
    //     Vector3::z_axis(),
    // );

    let mut arms = vec![];
    let mut arm_joints = vec![];
    for i in 0..8 {
        let arm_frame = format!("arm{}", i);
        let m = 0.1;
        let w = 0.1;
        let arm = Rigid::new_cuboid_at(&vector![w / 2., 0., 0.], m, w, 0.02, 0.02, &arm_frame);
        let arm_joint = Joint::new_revolute(
            Transform3D::move_z(&arm_frame, WORLD_FRAME, i as Float * 0.05),
            Vector3::z_axis(),
        );
        arms.push(arm);
        arm_joints.push(arm_joint);
    }

    // let articulated = Articulated::new(vec![arm], vec![arm_joint]);
    let articulated = Articulated::new(arms, arm_joints);
    state.add_articulated(articulated);

    state
}

#[cfg(target_arch = "wasm32")]
#[allow(non_snake_case)]
#[wasm_bindgen]
pub async fn createSesame() -> InterfaceHybrid {
    let mut state = build_sesame();

    let controller = SesameESP32Controller::new().await;
    // let controller = SesameServoController::new();
    state.set_controller(0, controller);

    InterfaceHybrid::new(state)
}
