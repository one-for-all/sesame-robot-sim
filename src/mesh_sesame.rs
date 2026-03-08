use gorilla_physics::{
    hybrid::{control::NullArticulatedController, visual::rigid_mesh::RigidMesh},
    util::read_file,
};
use sesame::{
    builder::{SesameMeshes, build_sesame},
    control::pid::SesameServoController,
};

fn main() {
    // let urdf_path = "onshape/robot.urdf";
    // let urdf_file = read_file(urdf_path);
    // let urdf_robot = urdf_rs::read_from_string(&urdf_file).unwrap();

    // let mut meshes = SesameMeshes::new();
    // let file_paths = vec![
    //     "onshape/mesh/internal_frame.obj",
    //     "onshape/mesh/femur_joint_l2.obj",
    //     "onshape/mesh/foot_joint_l4.obj",
    //     "onshape/mesh/femur_joint_r2.obj",
    //     "onshape/mesh/foot_joint_r4.obj",
    //     "onshape/mesh/femur_joint_l1.obj",
    //     "onshape/mesh/foot_joint_l3.obj",
    //     "onshape/mesh/femur_joint_r1.obj",
    //     "onshape/mesh/foot_joint_r3.obj",
    // ];
    // let buffers: Vec<String> = file_paths.iter().map(|path| read_file(path)).collect();
    // for (i, buf) in buffers.iter().enumerate() {
    //     let mesh = Some(RigidMesh::new_from_obj(buf));
    //     match i {
    //         0 => meshes.body = mesh,
    //         1 => meshes.l2 = mesh,
    //         2 => meshes.l4 = mesh,
    //         3 => meshes.r2 = mesh,
    //         4 => meshes.r4 = mesh,
    //         5 => meshes.l1 = mesh,
    //         6 => meshes.l3 = mesh,
    //         7 => meshes.r1 = mesh,
    //         8 => meshes.r3 = mesh,
    //         _ => panic!("unknown index: {}", i),
    //     }
    // }

    // let mut state = build_sesame(&mut meshes, &urdf_robot);
    // // let controller = SesameServoController::new();
    // let controller = NullArticulatedController {};
    // state.set_controller(0, controller);

    // let t_final = 2.0;
    // let dt = 1e-3;
    // let num_steps = (t_final / dt) as usize;
    // for _ in 0..num_steps {
    //     state.step(dt, &vec![]);
    // }
}
