// Use fs to read in file
use std::fs::read_to_string;
// Use env to read command line arguements
use std::env;

#[derive(Debug)]
struct RingBuffer <T>{

}

// We want this to work for any data type which implements partial equality
impl <T: std::cmp::PartialEq + std::fmt::Display> RingBuffer  <T> {
    fn new  (capacity :usize) -> RingBuffer <T>{

    }

    // Get the length of the buffer
    fn len(self  :&Self) -> usize{

    }

    // Pop out the first element
    fn pop (self :&mut Self) -> Option<T> {

    }

    // Push a new element to the buffer
    fn push (self :&mut Self, val : T) -> () {

    }

    // Check if we have repeating values
    fn has_repeats(self: &Self) -> bool{

    }

}




// Function to decode the file
fn decode_file(pattern: &String, capacity :usize) -> usize { 

    // New mutable buffer of characters
    let mut buffer = RingBuffer::<char>::new(capacity);

    // For finding the space
    let mut i = 0;

    // Loop over the characters
    for c in pattern.chars().into_iter(){
        
    }
    panic!("Not found")
}




fn main() {

        // Collect the command line arguments
        let args  = env::args().collect::<Vec<String>>();

        // // Read in a text file
        let filename = &args[1]; 
    
        // Read in each line and convert to a string
        let line =  read_to_string(filename)
                        .unwrap()
                        .lines()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>();
    
        // Loop over lines
        for i in 0..line.len(){
    
            // Star 1
            println!("{}", &line[i]);
            let pre = decode_file(&line[i],4);
            println!("Starting at: {}", pre);
    
        }
    

}
