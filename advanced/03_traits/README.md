# Traits

Traits in Rust provide a means to define common interfaces that can be implemented by different structs and data types. They enable different types to share behavior or functionality through shared methods.

For instance, let's consider the following struct:
```Rust
struct Point3D{
    x: f32,
    y: f32,
    z: f32,
}
```
If we wanted to print this to the screen using the `println` macro, we could try something like:
```Rust
    let my_point = Point3D{ x: 1.2, y: 3.1, z: -1.5};
    println!("{}", my_point);
```

This would throw an error like:
```
error[E0277]: `Point3D` doesn't implement `std::fmt::Display`
  --> src/main.rs:10:20
   |
10 |     println!("{}", my_point);
   |                    ^^^^^^^^ `Point3D` cannot be formatted with the default formatter
```

This tells us that we are not implementing the `Display` trait. When we try to print with `println!("{}", my_point)`, then `Point3D` (which is the type of `my_point`) must implement `Display`. This makes sense when we think about what we are hoping to print. We haven't determined how to print this `struct`. We can tell how to print this by implementing the trait `Display`:
```rust
use std::fmt::{Display, Formatter, Result};

...

impl Display for Point3D{
    fn fmt(self: &Self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}
...
```

Here, we implement the `Display` trait for `Point3D`. The `Display` trait includes the function `fmt`, which defines the format of the string we want to print. To begin, we import specific modules:

```rust
use std::fmt::{Display, Formatter, Result};
```

We use the `Formatter` type to write to a string. The `write!` macro will return a `Result`, which will be either a successful return of the formatted string or an error.