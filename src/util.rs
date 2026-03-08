use gorilla_physics::{
    hybrid::{
        Rigid,
        visual::{Visual, rigid_mesh::RigidMesh},
    },
    inertia::SpatialInertia,
    joint::Joint,
    spatial::transform::Transform3D,
    types::Float,
};
use nalgebra::{
    Isometry3, Matrix3, Translation3, UnitQuaternion, UnitVector3, Vector, Vector3, vector,
};
use urdf_rs::{Geometry, Robot};

use crate::mesh::URDFMeshes;

pub fn build_rigid(frame: &str, link_name: &str, urdf: &Robot, meshes: &mut URDFMeshes) -> Rigid {
    let link_urdf = urdf.links.iter().find(|&l| l.name == link_name).unwrap();

    let inertial = &link_urdf.inertial;
    let m = inertial.mass.value;
    let com = Vector::from(inertial.origin.xyz.0);
    let ixx = inertial.inertia.ixx;
    let ixy = inertial.inertia.ixy;
    let ixz = inertial.inertia.ixz;
    let iyy = inertial.inertia.iyy;
    let iyz = inertial.inertia.iyz;
    let izz = inertial.inertia.izz;

    #[rustfmt::skip]
    let moment_com = Matrix3::new(
        ixx, ixy, ixz,
        ixy, iyy, iyz,
        ixz, iyz, izz
    );

    let moment =
        moment_com + m * (com.norm_squared() * Matrix3::identity() - com * com.transpose());
    let cross_part = m * com;

    let mut body = Rigid::new(SpatialInertia::new(moment, cross_part, m, frame));

    if let Some(link_meshes) = meshes.meshes.remove(link_name) {
        for (mesh, iso, color) in link_meshes.into_iter() {
            body.visual
                .push((Visual::RigidMesh(mesh), iso, Some(color)));
        }
    }

    // if let Some(mesh) = meshes.take() {
    //     let visual = link_urdf
    //         .visual
    //         .iter()
    //         .find(|&v| match &v.geometry {
    //             Geometry::Mesh { filename, .. } => filename.contains(link_name),
    //             _ => false,
    //         })
    //         .unwrap();
    //     let [r, p, y] = visual.origin.rpy.0;
    //     let iso = Isometry3::from_parts(
    //         Translation3::from(visual.origin.xyz.0),
    //         UnitQuaternion::from_euler_angles(r, p, y),
    //     );
    //     let [r, g, b, _] = visual
    //         .material
    //         .as_ref()
    //         .unwrap()
    //         .color
    //         .as_ref()
    //         .unwrap()
    //         .rgba
    //         .0;
    //     let color = vector![r, g, b];
    //     body.visual
    //         .push((Visual::RigidMesh(mesh), iso, Some(color)));
    // }

    body
}

pub fn build_joint(
    from: &str,
    to: &str,
    joint_name: &str,
    urdf: &Robot,
    axis: UnitVector3<Float>,
    q: Float,
) -> Joint {
    let urdf_joint = urdf.joints.iter().find(|&j| j.name == joint_name).unwrap();
    let joint = Joint::new_revolute_with_q(
        q,
        Transform3D::new_xyz_rpy(
            from,
            to,
            &Vec::from(urdf_joint.origin.xyz.0),
            &Vec::from(urdf_joint.origin.rpy.0),
        ),
        axis,
    );
    joint
}

pub fn add_collision_points(rigid: &mut Rigid, frame: &str, urdf: &Robot) {
    for i in 1..5 {
        let point_frame_name = format!("{}_p{}_frame", frame, i);
        let point_joint = urdf
            .joints
            .iter()
            .find(|&j| j.name == point_frame_name)
            .unwrap();
        rigid.add_point_at(&Vector3::from(point_joint.origin.xyz.0));
    }
}
