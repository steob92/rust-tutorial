struct Point3D {
    x: f32,
    y: f32,
    z: f32,
    coord_system: String,
}

impl Point3D {
    fn print(self: &Self) -> (){
        println!("({}, {}, {})", self.x, self.y, self.z);
    }
}


struct Point2D {
    x: f32,
    y: f32,
    coord_system: String,
}


impl Point2D {
    fn print_coord(self: &Self) -> (){
        println!("({}, {})", self.x, self.y);
    }
}



trait PointLike {
    fn new() -> Self;
    fn get_magnitude(self: &Self) -> f32;
    fn add(self: &mut Self, c : f32) -> ();    
}

// Implement the required functions for Point3D
impl PointLike for Point3D{
    fn new() -> Self {
        Self{
            x:0.0_f32,
            y:0.0_f32,
            z:0.0_f32,
            coord_system: String::from("cart"),
        }
    }

    fn get_magnitude(self : &Self) -> f32 {
        (
            self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
        ).sqrt()
    }

    fn add(self : &mut Self, c: f32) -> (){
        let over_sqrt = 1.0_f32 / 3.0_f32;
        self.x += c * over_sqrt;
        self.y += c * over_sqrt;
        self.z += c * over_sqrt;  
    }

}


// Implement the required functions for Point2D
impl PointLike for Point2D{
    fn new() -> Self {
        Self{
            x:0.0_f32,
            y:0.0_f32,
            coord_system: String::from("cart"),
        }
    }

    fn get_magnitude(self : &Self) -> f32 {
        (
            self.x.powi(2) + self.y.powi(2)
        ).sqrt()
    }

    fn add(self : &mut Self, c: f32) -> (){
        let over_sqrt = 1.0_f32 / 2.0_f32;
        self.x += c * over_sqrt;
        self.y += c * over_sqrt;
    }
}


fn main(){

    let mut my_point_3D = Point3D::new();
    let mut my_point_2D = Point2D::new();
    my_point_3D.add(3.14);
    my_point_2D.add(4.5);
    
    my_point_3D.print();
    my_point_2D.print_coord();

    println!("Magnitude of point (3D): {} ",my_point_3D.get_magnitude());
    println!("Magnitude of point (2D): {} ",my_point_2D.get_magnitude());
}