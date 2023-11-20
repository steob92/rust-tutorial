use rayon::prelude::*;
use std::time::Instant;

fn main() {

    let n = 100_000_000;
    let x: Vec<i32> = (0..n).collect();
    let y: Vec<i32> = (0..n).collect();

    let now_serial = Instant::now();

    let z : Vec<i32>= x.iter().zip(y.iter()).map(|(a,b)| a + b).collect();

    let elapse_serial = now_serial.elapsed().as_secs_f32();

    println!("Serial took {} s", elapse_serial);


    let now_parallel = Instant::now();

    let z_par : Vec<i32>= x.par_iter().zip(y.par_iter()).map(|(a,b)| a + b).collect();

    let elapse_parallel = now_parallel.elapsed().as_secs_f32();
    println!("Parallel took {} s", elapse_parallel);

    println!("Speed up = {} ", elapse_serial / elapse_parallel);

}
