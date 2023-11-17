// Write a struct that contains the x,y and z coordinates and the coordinate system
struct Point3D {
}


// Implement the required functions
impl Point3D {

}



fn main() {

    // Create a new Point3D with values at the origin (0,0,0)
    let mut my_3d = Point3D::new();
    
    // Add a constant value to the Point3D, each point gets 1/sqrt(3) of the value
    my_3d.add_constant(&4.5);

    // Get the magnitude of the Point3D
    println!("Magnitude of 3D {}", my_3d.get_magnitude());

}
