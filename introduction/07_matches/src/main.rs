fn main(){

    let a :i32 = 4;

    // Write a match to print the correct statement
    match a {
        0..=2 => {
            println!("a is less than 3");
        }
        4..=10 => {
            println!("a is greater than 3");
        }
        3 => {
            println!("a is 3");
        }
        _ => {
            println!("a is > 10");
        }
    }


    // Write a match to assign the correct statement
    let b = match a {
        0 => "0",
        1 => "alpha",
        2 => "2",
        3 => "delta",
        4 => "for",
        _ => "Something else",
    };

    println!("b is {}", b);
}
