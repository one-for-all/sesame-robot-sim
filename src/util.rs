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

pub fn build_rigid(
    frame: &str,
    link_name: &str,
    urdf: &Robot,
    mesh: &mut Option<RigidMesh>,
) -> Rigid {
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

    if let Some(mesh) = mesh.take() {
        let visual = link_urdf
            .visual
            .iter()
            .find(|&v| match &v.geometry {
                Geometry::Mesh { filename, .. } => filename.contains(link_name),
                _ => false,
            })
            .unwrap();
        let [r, p, y] = visual.origin.rpy.0;
        let iso = Isometry3::from_parts(
            Translation3::from(visual.origin.xyz.0),
            UnitQuaternion::from_euler_angles(r, p, y),
        );
        let [r, g, b, _] = visual
            .material
            .as_ref()
            .unwrap()
            .color
            .as_ref()
            .unwrap()
            .rgba
            .0;
        let color = vector![r, g, b];
        body.visual
            .push((Visual::RigidMesh(mesh), iso, Some(color)));
    }

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
