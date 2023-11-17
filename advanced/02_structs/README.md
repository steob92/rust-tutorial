# Structs

Structs in Rust form the foundation of object-oriented programming (OOP). They can be seen as collections of variables that serve a related purpose or represent a specific context.

Consider the example below:
```rust
struct Rectangle {
    length : f32,
    width : f32,
}
```

This creates a `struct` with two fields, `length` and `width` both of which are `f32`s.  We can implement functions for the structs like so:

``` rust
impl Rectangle {
    fn new() -> Rectangle {
        Rectangle{
            length : 0.0_f32,
            width : 0.0_f32,
        }
    }

    fn get_area(self: &Self) -> f32{
        self.length * self.width
    }
}
```