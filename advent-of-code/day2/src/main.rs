// Use fs to read in file
use std::fs::read_to_string;
// Use env to get the command line arguments
use std::env;


// Enum to handle available shapes
#[derive(Clone,Debug,PartialEq)]
enum Shape{

}

// Enum to handle possible results
#[derive(Debug)]
enum GameResult{

}

// Use a match to get the correct points
fn get_shape_points( player: &Shape) -> i32{
    match player{

    }
}


// Find out the result of the game
fn get_result( player1: &Shape, player2: &Shape) -> i32{

    let result = match player1 == player2 {

    };

    // Match on the result to get the correct score
    match result {

    }
}



// Expecting the file in a format 
// A    X
// B    Z
// etc..
fn read_file(filename: &String) -> Vec<Vec<String>>{
    let mut data :Vec<Vec<String>> = Vec::new(); 
    for line in read_to_string(filename).unwrap().lines(){
        // Loop over each split in the line split
        let tmp_vec = line.split_whitespace() // Split by whitespace in teh line
            .into_iter() // Create iterator
            .map(       // For each
                |x|  // Take x
                x.to_string() // Convert &str to String
            ).collect::<Vec<String>>();

        // Ownership of tmp_vec passes to data
        data.push(tmp_vec);
    }
    data
}

// Calculate the points awarded
fn calculate_points (player1: &String, player2: &String) -> i32{

    // Match player 1 to get the Shape chosen
    let opt_1 = match player1.as_ref() {

        _ => panic!("Error"),
    };
    

    // Match Player2
    let opt_2 = match player2.as_ref() {

        _ => panic!("Error"),
    };
    
    // Get the points from the game result
    let result :i32 = get_result(&opt_2, &opt_1);
    // Get the points from the shape chosen
    let shape :i32 = get_shape_points(&opt_2);

    result + shape
}



fn main() {
    // Collect the command line arguments
    let args  = env::args().collect::<Vec<String>>();

    // Read in a text file
    let filename = &args[1];

    let data = read_file(filename);

    let points = data.iter()
        .map(|x| calculate_points(&x[0], &x[1]))
        .sum::<i32>();
    
    // assert!(points == 15);
    println!("Points = {}", points);

}
