fn main(){

    let a :i32 = 4;

    // Write a match to print the correct statement
            println!("a is less than 3");
            println!("a is greater than 3");
            println!("a is 3");
            println!("a is > 10");

    // Write a match to assign the correct statement
    let b = match a {
        "0",
        "alpha",
        "2",
        "delta",
        "for",
        "Something else",
    };

    println!("b is {}", b);
}
