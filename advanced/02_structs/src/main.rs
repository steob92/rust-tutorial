// Write a struct that contains the x,y and z coordinates and the coordinate system
struct Point3D {
    x : f32,
    y : f32,
    z : f32,
    coords : String,
}


// Implement the required functions
impl Point3D {
    fn new() -> Point3D{
        Point3D{
            x:0.0, y:0.0, z:0.0,
            coords: String::from("cart"),
        }
    }


    fn get_magnitude(self : &Self ) -> f32{
        (
            self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
        ).sqrt()
    } 

    fn add_constant(self : &mut Self, c : f32) -> (){
        let over_sqrt3 = 1.0 / 3.0_f32.sqrt();
        self.x += c * over_sqrt3;
        self.y += c * over_sqrt3;
        self.z += c * over_sqrt3;
    }
}



fn main() {

    // Create a new Point3D with values at the origin (0,0,0)
    let mut my_3d = Point3D::new();
    
    // Add a constant value to the Point3D, each point gets 1/sqrt(3) of the value
    my_3d.add_constant(4.5);

    // Get the magnitude of the Point3D
    println!("Magnitude of 3D {}", my_3d.get_magnitude());

}
