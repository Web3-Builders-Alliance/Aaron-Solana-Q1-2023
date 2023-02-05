// fixed lists

use std::mem;

pub fn run(){
    let mut numbers: [i32; 4] = [1,2,3,4];

    //reassign
    numbers[2] = 20;

    println!("all vals: {:?}", numbers);

    //get single val
    println!("single value: {}", numbers[0]);

    //getlength
    println!("arry leng {}",numbers.len());

    // array stack alloc 
    println!("array uses {} memory bytes",mem::size_of_val(&numbers));


    // get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);




}