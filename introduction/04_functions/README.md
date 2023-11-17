# Functions

Functions in Rust are defined using the following syntax:
```Rust linenums="1" title="Examples of functions" hl_lines="1 5 9 13"
fn add_numbers(a: i32, b: i32) -> i32{
    return a + b;
}

fn multiply_numbers(a :i32, b :i32) -> i32{
    a * b
}

fn print_numbers(a :i32, b :i32) {
    println!("{} + {} = {}", a,b, a+b);
}

fn print_numbers_multiply(a :i32, b :i32) -> () {
    println!("{} * {} = {}", a,b, a*b);
}

fn main(){
    let x = 3;
    let y = 4;

    let sum = add_numbers(x,y);
    print_numbers(x,y);
    let product = multiply_numbers(x,y);
    print_numbers_multiply(x,y);

    println!("sum = {}, product = {}", sum, product);

}
```

In the example above, three functions are defined using the `fn` keyword to indicate their creation. When defining functions, specifying the data types of passed arguments is necessary, as demonstrated here by using `i32` types in all cases. Additionally, if a function returns a value, explicit declaration of the return type is required. Lines 1 and 5 explicitly define the return type as `i32`, denoted by `-> T`, where `T` represents the data type.

Lines 9 and 13 introduce functions that do not return any value. When a function doesn't return anything, the `->` can be omitted. Alternatively, it's possible to explicitly state the absence of a return value using `-> ()`.

The functions `add_numbers` and `multiply_numbers` both return an `i32`. However, only `add_numbers` uses a `return` keyword. In Rust, if a statement isn't followed by a `;`, it's assumed to be the return value. In the case of `multiply_numbers`, the absence of `;` specifies that the function should return `a * b`.

It's important to note that in all these functions, ownership of `a` and `b` is taken within the functions. Consequently, when the function's scope ends, both `a` and `b` are dropped. While this behavior might not be problematic for `i32` due to its copy trait, allowing passing a copy of the value rather than the value itself, it's a crucial consideration for other types where ownership might cause different behavior.