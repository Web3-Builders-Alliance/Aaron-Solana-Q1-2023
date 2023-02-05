pub fn run() {
    // default is i32
    let x = 1; 

    // deafult is f64
    let y = 2.5;

    // be explicit
    let z: i64 = 343434343;

    //find max
    println!("Max i32 {}", std::i32::MAX);
    println!("Max i64 {}", std::i64::MAX);

    //oolean
    let is_active: bool = true;

    //boolean from expr
    let is_greater:bool = 10 > 5;

    let a1 = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (x,y,z, is_active, is_greater, a1, face));



}