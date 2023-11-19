struct Point3D {
    x: f32,
    y: f32,
    z: f32,
    coord_system: String,
}


struct Point2D{
    x: f32,
    y: f32,
    coord_system: String,
}



trait PointLike{
    fn get_magnitude(self: &Self) -> f32;
    fn add(self: &mut Self, c : f32) -> ();    
}

impl Point3D {
    
    // Return a new Point3D
    fn new() -> Point3D{
        Point3D{
            x: 0.0_f32,
            y: 0.0_f32,
            z: 0.0_f32,
            coord_system: String::from("cartesian"),
        }
    }

    // Get the magnitude of the Point3D
    fn get_magnitude(self :&Self) -> f32{
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2))
        .sqrt()  
    }

    // Add a constant value to the Point3D
    fn add_constant(self: &mut Self, c : f32) -> (){
        self.x += c / 3.0_f32.sqrt();
        self.y += c / 3.0_f32.sqrt();
        self.z += c / 3.0_f32.sqrt();
    }
}

impl PointLike for Point3D{
    fn get_magnitude(self: &Self) -> f32{
        self.get_magnitude()
    }

    fn add(self: &mut Self, c : f32) -> (){
        self.add_constant(c)
    }    
}

impl Point2D{
    fn new()->Point2D{
        Point2D{
            x: 0.0_f32,
            y: 0.0_f32,
            coord_system: String::from("cartesian"),
        }
    }
}

impl PointLike for Point2D{
    fn get_magnitude(self: &Self) -> f32{
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    fn add(self: &mut Self, c : f32) -> (){
        self.x += c /2.0_f32.sqrt();
        self.y += c /2.0_f32.sqrt();
    }    
}



// Create a struct Planetary system that record the planet names and 
// the x,y,z (Point3D) or x,y (Point2D) of each planet using generic types
// Store both names and positions within vectors
struct PlanetarySystem <T> {
    planet_location : Vec<T>,
    planet_name : Vec<String>,
}

// Implement a function to print the distance between each planet and its star
// Implement this for any generic type that implments the PointLike trait
impl <T: PointLike> PlanetarySystem<T> {
    fn print_distance_from_star(self : &Self) -> (){
        for i in 0..self.planet_location.len(){
            let dist = self.planet_location[i].get_magnitude();
            println!(
                "Distance from {} to it's star: {} AU",
                self.planet_name[i],
                dist,
            );

        }
    }

}


fn main(){

    let mut solar_system: PlanetarySystem<Point3D> = PlanetarySystem{
        planet_location: vec![
            Point3D::new(), 
            Point3D::new(), 
            Point3D::new()
        ],
        planet_name: vec![
            "Mercury".to_string(), 
            "Venus".to_string(), 
            "Earth".to_string()
        ],
    };

    solar_system.planet_location[0].add(0.39);
    solar_system.planet_location[1].add(0.72);
    solar_system.planet_location[2].add(1.);
    solar_system.print_distance_from_star();


    let mut trappst_system: PlanetarySystem<Point2D> = PlanetarySystem{

        planet_location: vec![Point2D::new(), Point2D::new(), Point2D::new()],
        planet_name: vec![ "TRAPPIST-1b".to_string(), "TRAPPIST-1c".to_string(), "TRAPPIST-1e".to_string()],
        
    };

    trappst_system.planet_location[0].add(0.01154);
    trappst_system.planet_location[1].add(0.01580);
    trappst_system.planet_location[2].add(0.029);
    trappst_system.print_distance_from_star();
}