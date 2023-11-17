# Generics

Generics enable us to write flexible code that doesn't depend on specific data types. Consider the following example:

```rust
use std::ops::Mul;
struct Rectangle <T>{
    length : T,
    width : T,
}


impl <T: Mul<Output = T> + Copy> Rectangle<T> {
    fn get_area(self :&Self) -> T {
        self.length * self.width
    }
} 

fn main(){

    let my_shape: Rectangle<u32> = Rectangle{ length: 7, width: 5};
    let my_different_shape: Rectangle<f32> = Rectangle{ length: 3.14, width: 7.1};

    println!("Area = {}", my_shape.get_area());
    println!("Area = {}", my_different_shape.get_area());

}
```

Here we use the `Mul` trait, which defines the behavior of the multiplication operator `*`. We define `Rectangle` using a generic type `T`, and then implement `Rectangle` for any generic type `T`. We specify that the generic type `T` must have both the `Mul` trait and the `Copy` trait. When using the `Mul` trait, we specify that the output type is also `T`.

In the `main` function, we utilize both `u32` and `f32` types when working with `Rectangle`, enabling us to write generic and flexible code.
