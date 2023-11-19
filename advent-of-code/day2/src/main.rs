// Use fs to read in file
use std::fs::read_to_string;
// Use env to get the command line arguments
use std::env;


// Enum to handle available shapes
#[derive(Clone,Debug,PartialEq)]
enum Shape{
    Rock,
    Paper,
    Scissors,
}

// Enum to handle possible results
#[derive(Debug)]
enum GameResult{
    Win,
    Loss,
    Draw,
}

// Use a match to get the correct points
fn get_shape_points( player: &Shape) -> i32{
    match player{
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    }
}


// Find out the result of the game
fn get_result( player1: &Shape, player2: &Shape) -> i32{

    let result = match player1 == player2 {
        true => {
            GameResult::Draw
        },
        false => {
            match (player1, player2){
                // Rock
                (Shape::Rock, Shape::Paper) => GameResult::Loss,
                (Shape::Rock, Shape::Scissors) => GameResult::Win,
                // Paper
                (Shape::Paper, Shape::Rock) => GameResult::Win,
                (Shape::Paper, Shape::Scissors) => GameResult::Loss,
                // Scissors
                (Shape::Scissors, Shape::Rock) => GameResult::Loss,
                (Shape::Scissors, Shape::Paper) => GameResult::Win,
                _ => panic!("Will not reach")
            }
        }

    };

    // Match on the result to get the correct score
    match result {
        GameResult::Win => 6,
        GameResult::Draw => 3,
        GameResult::Loss => 0,
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
        "A" => Shape::Rock,
        "B" => Shape::Paper,
        "C" => Shape::Scissors,
        _ => panic!("Error"),
    };
    

    // Match Player2
    let opt_2 = match player2.as_ref() {
        "X" => Shape::Rock,
        "Y" => Shape::Paper,
        "Z" => Shape::Scissors,
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

    let points = data.iter().map(|x| calculate_points(&x[0], &x[1])).sum::<i32>();
    // assert!(points == 15);
    println!("Points = {}", points);

}
