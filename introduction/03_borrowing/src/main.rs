fn main() {

    let mut x = 42;

    let y = &x;

    println!("y = {}", y);

    let mut z: i32 = x;
    z += 1;

    println!("x = {}", x);
    println!("y = {}", y);
    println!("z = {}", z);


    {
        let a = y;
        let another = &x;
        println!("a = {}", a);
        println!("another = {}", another);
    }

    x += 1;
    println!("x = {}", x);

    {
        let b  = &mut x;
        *b +=  10;
        println!("b = {}", b);

    }

    println!("x = {}", x);
    let last = &mut x;
    *last -= 100;
    println!("last = {}", last);
}
