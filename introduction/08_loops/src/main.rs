fn main(){
    let mut i = 0;

    // Write a loop that prints out i
    // iterate i on each step
    // skip the 3rd step
    // break when i goes above 10
    loop {
        println!("{}", i);
        i += 1;
        if i == 2{
            continue;
        }
        if i > 10{
            break;
        }
    }


    i = 0;

    // Write a parent loop where i is iterated by one each loop
    'astra : loop {
        let mut j = 0;

        // Write a nested loop that breaks the parent loop when i >10
        // breaks the nested loop when j > 3
        // prints out i,j one on each step
        // increases j by
        'kafka : loop{
            if i > 10 {
                break 'astra;
            } else if j > 3 {
                break 'kafka;
            } else{
                println!("{},{}", i,j);
            }
            j+=1;

        }
        i+=1;
    }
}
