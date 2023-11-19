fn main(){
    let pi  = 3.14_f32;

    // Write a closure to calculate the area of a circle
    let area = | x | {
        pi * x * x
    };
    
    // Write a closure to calculate and print the area of a circle
    let print_area = |x| {
        println!("The area is: {}", area(x));
    };

    println!("The area is: {}", area(2.));
    print_area(1.5);
}