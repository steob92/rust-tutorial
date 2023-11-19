fn main() {
    // Explicitly declaring the type of the variable
    let my_integer: i32 = 42;
    let my_float: f64 = 3.14;
    let my_character: char = 'A';
    let my_boolean: bool = true;

    // Rust can infer types in many cases, so explicit annotation is not always necessary
    let inferred_integer = 10;
    let inferred_float = 2.5;
    let inferred_character = 'B';
    let inferred_boolean = false;


    // Explicitly declaring the type of the variable within the passed value
    let my_integer_in_value = 17_i8;
    let my_float_in_value = 6.28_f32;
    let my_large_unsigned_32 = 1_000_000_u32;


    // Printing the values along with their types
    println!("Integer: {} (Type: i32)", my_integer);
    println!("Float: {} (Type: f64)", my_float);
    println!("Character: {} (Type: char)", my_character);
    println!("Boolean: {} (Type: bool)", my_boolean);

    println!("Inferred Integer: {} (Type: inferred)", inferred_integer);
    println!("Inferred Float: {} (Type: inferred)", inferred_float);
    println!("Inferred Character: {} (Type: inferred)", inferred_character);
    println!("Inferred Boolean: {} (Type: inferred)", inferred_boolean);


    println!("Integer: {} (Type: inferred from value)", my_integer_in_value);
    println!("Float: {} (Type: inferred from value)", my_float_in_value);
    println!("Unsigned: {} (Type: inferred from value)", my_large_unsigned_32);
}
