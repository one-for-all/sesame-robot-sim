fn main() {
    let urdf_robot = urdf_rs::read_file("onshape/robot.urdf").unwrap();

    println!("{:#?}", urdf_robot.links);
}
