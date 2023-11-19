fn main(){
    let my_arr: [f32;5] = [1.,2.,3.,43., 3.14];

    // Write a for loop to print out each element
    for a in my_arr{
        println!("{a}");
    }
    println!("{:?}", my_arr);

    // Write a for loop to print out each element of the vector 
    let my_vec: Vec<f32> = vec![1.,2.,3.,43., 3.14];

    for v in my_vec.iter(){
        println!("{}", v);
    }
    println!("{:?}", my_vec);



    let x: Vec<f32> = vec![1.,2.,3.,43., 3.14];
    let y: Vec<f32> = vec![2.,0.1,5.3,0.001, 3.14];
    let mut z: Vec<f32> = vec![0.0_f32; 5];
    
    // Write a for loop to sum z = x + y 
    for ((a,b),i) in x.iter().zip(&y).zip(0..x.len()){
        z[i] = a + b;
        println!("{}+{} = {}", a, b, z[i]);
    }

    println!("{:?}", z);    
}
