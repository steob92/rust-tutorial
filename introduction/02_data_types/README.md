# Data Types in Rust

In Rust, types must be known at compile time, and you can explicitly specify the type of a variable using the syntax `let my_variable: type = value`, where the type is indicated after the variable name with a `:`.

Some commonly used data types in Rust include:

- `i32`: a 32-bit signed integer.
- `f32`: a 32-bit floating-point number.
- `u32`: an unsigned 32-bit integer (non-negative).
- `bool`: a boolean value (`true` or `false`).
- `str`: a string slice.
- `char`: a single Unicode character.
- `String`: an owned, growable UTF-8 string.

Rust allows type inference, where the compiler infers the type if it's omitted. For instance:
```rust
let a = 0;
```
The compiler infers that `0` is of type `i32`.

Type conversions in Rust can be performed using into, from, try_into, try_from, or as.

- `into` is used for converting between types that can be directly converted.
- `from` is also used for type conversions but does not consume the original value.
- `try_into` and `try_from` are similar to into and from but allow possible failure.
- `as` is used to cast a value to a different data type.

To better understand these methods, experimenting with code is beneficial. For example:

``` rust
// Example of type conversion using `into`
let x: i32 = 5;
let y: i64 = x.into(); // Converts an i32 into an i64

// Example of type conversion using `from`
let z: i64 = 10;
let w: i32 = i32::from(z); // Converts an i64 into an i32
```