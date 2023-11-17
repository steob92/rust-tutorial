// Function that takes ownership of a string and prints it out
fn print_string( msg ) {

}

// function that borrows a reference to a string a prints it
fn print_string_borrow( msg ) {
    println!("{}", msg);
}

// Function that takes in an i32 and returns a tuple of i32 and f32
fn get_powers( a: i32 ) {
    (a.pow(2), (a as f32).powf(0.33))
}



fn main(){
    let my_string = String::from("Save Ferris!");

    // Correct the inputs
    print_string();
    print_string_borrow();
    println!("{}", my_string);



    let x :i32 = 8;
    let tup = get_powers(x);
    // Deconstruct tuple
    let (y, z) : (i32, f32) = get_powers(x);

    println!("{}, {}", tup.0, tup.1 );
    println!("{}, {}", y, z );

}
