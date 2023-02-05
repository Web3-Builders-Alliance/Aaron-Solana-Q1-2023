// fixed lists

use std::mem;

pub fn run(){
    let mut numbers: Vec<i32> = vec![1,2,3,4];

    //reassign
    numbers[2] = 20;

    // add on 
    numbers.push(5);
    numbers.push(6);
    //remove
    numbers.pop();

    println!("all vals: {:?}", numbers);

    //get single val
    println!("single value: {}", numbers[0]);

    //getlength
    println!("vec leng {}",numbers.len());

    // vec stack alloc 
    println!("vec uses {} memory bytes",mem::size_of_val(&numbers));


    // get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

    
    // loop through all
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // loop & mut (like .map)
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("numbers muted: {:?}", numbers);
}