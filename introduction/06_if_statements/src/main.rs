fn main() {

    let a :i32 = 4;
    // Write an if-else if-else block to output the correct answer
    if a > 3{
        println!("a is greater than 3");
    } else if a < 3 {
        println!("a is less than 3");
    } else {
        println!("a is equal to 3");
    }




    // use a if-else if-else block to assign the correct answer
    let my_string :String = if a > 3{
        "a is greater than 3".to_string()
    } else if a < 3 {
        "a is less than 3".to_string()
    } else {
        "a is equal to 3".to_string()
    };
        
    println!("{}", my_string)
}
