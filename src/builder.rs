use futures::future::join_all;
use gorilla_physics::{
    WORLD_FRAME,
    collision::halfspace::HalfSpace,
    hybrid::{
        Hybrid, Rigid,
        articulated::Articulated,
        control::NullArticulatedController,
        rigid,
        visual::{Visual, rigid_mesh::RigidMesh},
    },
    inertia::SpatialInertia,
    interface::{hybrid::InterfaceHybrid, util::read_web_file},
    joint::Joint,
    na::vector,
    spatial::transform::Transform3D,
    types::Float,
};
use nalgebra::{Isometry3, Matrix3, Translation3, UnitQuaternion, Vector, Vector3};
use urdf_rs::{Geometry, Robot};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    control::{SesameESP32Controller, motion::SesameMotionController, pid::SesameServoController},
    util::{add_collision_points, build_joint, build_rigid},
};

pub fn build_arms() -> Hybrid {
    let mut state = Hybrid::empty();

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

pub fn build_sesame(meshes: &mut SesameMeshes, urdf: &Robot) -> Hybrid {
    let mut state = Hybrid::empty();
    state.add_halfspace(HalfSpace::new(Vector3::z_axis(), 0.));

    let body_frame = "body";
    let body = build_rigid(body_frame, "internal_frame", urdf, &mut meshes.body);
    let body_joint = Joint::new_floating(Transform3D::move_z(body_frame, WORLD_FRAME, 0.05));

    let l2_frame = "l2";
    let l2 = build_rigid(l2_frame, "femur_joint_l2", urdf, &mut meshes.l2);
    let l2_joint = build_joint(
        l2_frame,
        body_frame,
        "l2",
        urdf,
        -Vector3::z_axis(),
        (135. as Float).to_radians(),
    );

    let l4_frame = "l4";
    let mut l4 = build_rigid(l4_frame, "foot_joint_l4", urdf, &mut meshes.l4);
    add_collision_points(&mut l4, "l4", urdf);
    let l4_joint = build_joint(
        l4_frame,
        l2_frame,
        "l4",
        urdf,
        -Vector3::z_axis(),
        (180. as Float).to_radians(),
    );

    let r2_frame = "r2";
    let r2 = build_rigid(r2_frame, "femur_joint_r2", urdf, &mut meshes.r2);
    let r2_joint = build_joint(
        r2_frame,
        body_frame,
        "r2",
        urdf,
        -Vector3::z_axis(),
        (45. as Float).to_radians(),
    );

    let r4_frame = "r4";
    let mut r4 = build_rigid(r4_frame, "foot_joint_r4", urdf, &mut meshes.r4);
    add_collision_points(&mut r4, "r4", urdf);
    let r4_joint = build_joint(
        r4_frame,
        r2_frame,
        "r4",
        urdf,
        -Vector3::z_axis(),
        (0. as Float).to_radians(),
    );

    let l1_frame = "l1";
    let l1 = build_rigid(l1_frame, "femur_joint_l1", urdf, &mut meshes.l1);
    let l1_joint = build_joint(
        l1_frame,
        body_frame,
        "l1",
        urdf,
        -Vector3::z_axis(),
        (45. as Float).to_radians(),
    );

    let l3_frame = "l3";
    let mut l3 = build_rigid(l3_frame, "foot_joint_l3", urdf, &mut meshes.l3);
    add_collision_points(&mut l3, "l3", urdf);
    let l3_joint = build_joint(
        l3_frame,
        l1_frame,
        "l3",
        urdf,
        -Vector3::z_axis(),
        (0. as Float).to_radians(),
    );

    let r1_frame = "r1";
    let r1 = build_rigid(r1_frame, "femur_joint_r1", urdf, &mut meshes.r1);
    let r1_joint = build_joint(
        r1_frame,
        body_frame,
        "r1",
        urdf,
        -Vector3::z_axis(),
        (135. as Float).to_radians(),
    );

    let r3_frame = "r3";
    let mut r3 = build_rigid(r3_frame, "foot_joint_r3", urdf, &mut meshes.r3);
    add_collision_points(&mut r3, "r3", urdf);
    let r3_joint = build_joint(
        r3_frame,
        r1_frame,
        "r3",
        urdf,
        -Vector3::z_axis(),
        (180. as Float).to_radians(),
    );

    let articulated = Articulated::new(
        vec![body, r1, r2, l1, l2, r4, r3, l3, l4],
        vec![
            body_joint, r1_joint, r2_joint, l1_joint, l2_joint, r4_joint, r3_joint, l3_joint,
            l4_joint,
        ],
    );
    state.add_articulated(articulated);

    state
}

// #[cfg(target_arch = "wasm32")]
#[allow(non_snake_case)]
#[wasm_bindgen]
pub async fn createSesame() -> InterfaceHybrid {
    let mut meshes = SesameMeshes::new();

    let urdf_path = "robot.urdf";
    let urdf_file = read_web_file(urdf_path).await;
    let urdf_robot = urdf_rs::read_from_string(&urdf_file).unwrap();

    let file_paths = vec![
        "mesh/internal_frame.obj",
        "mesh/femur_joint_l2.obj",
        "mesh/foot_joint_l4.obj",
        "mesh/femur_joint_r2.obj",
        "mesh/foot_joint_r4.obj",
        "mesh/femur_joint_l1.obj",
        "mesh/foot_joint_l3.obj",
        "mesh/femur_joint_r1.obj",
        "mesh/foot_joint_r3.obj",
    ];
    let fetches = file_paths.iter().map(|path| read_web_file(path));
    let buffers: Vec<String> = join_all(fetches).await;
    for (i, buf) in buffers.iter().enumerate() {
        let mesh = Some(RigidMesh::new_from_obj(buf));
        match i {
            0 => meshes.body = mesh,
            1 => meshes.l2 = mesh,
            2 => meshes.l4 = mesh,
            3 => meshes.r2 = mesh,
            4 => meshes.r4 = mesh,
            5 => meshes.l1 = mesh,
            6 => meshes.l3 = mesh,
            7 => meshes.r1 = mesh,
            8 => meshes.r3 = mesh,
            _ => panic!("unknown index: {}", i),
        }
    }

    let mut state = build_sesame(&mut meshes, &urdf_robot);

    let controller = SesameESP32Controller::new().await;
    // let controller = SesameMotionController::new();
    // let controller = SesameServoController::new();
    // let controller = NullArticulatedController {};
    state.set_controller(0, controller);

    InterfaceHybrid::new(state)
}

pub struct SesameMeshes {
    pub body: Option<RigidMesh>,
    pub l2: Option<RigidMesh>,
    pub l4: Option<RigidMesh>,
    pub r2: Option<RigidMesh>,
    pub r4: Option<RigidMesh>,
    pub l1: Option<RigidMesh>,
    pub l3: Option<RigidMesh>,
    pub r1: Option<RigidMesh>,
    pub r3: Option<RigidMesh>,
}

impl SesameMeshes {
    pub fn new() -> Self {
        Self {
            body: None,
            l2: None,
            l4: None,
            r2: None,
            r4: None,
            l1: None,
            l3: None,
            r1: None,
            r3: None,
        }
    }
}
