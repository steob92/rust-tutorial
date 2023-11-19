fn main(){
    let x: Vec<f32> = vec![1.,2.,3.,43., 3.14];
    let y: Vec<f32> = vec![2.,0.1,5.3,0.001, 3.14];

    // using iter, zip, map and collect
    // z = x + y
    let z  = x.iter().zip(y.iter()).map( |(a,b)| a+b).collect::<Vec<f32>>();
    println!("{:?}", z);


    let values = 0..100;

    // create an array of all the even values between 0-100 squared
    let even_squared = values.clone()
        .filter(|x| x%2 == 0)
        .map(|x| x*x)
        .collect::<Vec<i32>>();

    // get the sum of all the odd numbers between 0-100
    let odd_sum = values.clone()
        .filter(|x| x%2 == 1)
        .sum::<i32>();


    println!("{:?}", even_squared);
    println!("Sum of the odd values = {}", odd_sum);
    
}
