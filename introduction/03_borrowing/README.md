# Ownership in Rust

Ownership and the borrow checker constitute the foundation of Rust's memory management. When dealing with ownership in Rust, it's essential to remember three fundamental rules:

- Every value in Rust has a designated owner.
- At any given time, there can only be a single owner for a value.
- When the owner goes out of scope, the associated value is automatically dropped.


The fact that all value in Rust only ever has one owner guarentees that we can never acidently drop or delete a value that is still in use. This might seem very limiting and a heavy cost to pay for safety, but we can use "borrowing" to circumvent this issue.

```rust 
fn main() {

    let x = String::from("Hello");

    let y = &x;
    println!("{}", y);

    println!("{}", x);

}
```

In the above value we have "borrowed" the value of x. By borrowing the values instead of taking ownership, x maintains ownership over the value, allowing different parts of the code to access the value of the value.

When we want to change a value that we have borrowed we must take a mutable reference:
```rust 
fn main() {

    let x = String::from("Hello");
    println!("{}", x);

    let y = &mut x;
    *y += 10;
    println!("{}", y);

}
```

We can only ever have one mutable reference or any number of immutable references at a time.