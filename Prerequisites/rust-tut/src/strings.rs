pub fn run() {
    let mut hello = String::from("Hello ");

    // Get length
    println!("Length: {}", hello.len());

    //for char
    hello.push('W');
    //for strings
    hello.push_str("orld");

    //capacity, bytes
    println!("Capacity: {}", hello.capacity());

    // check empty
    println!("is empty {}", hello.is_empty());

    println!("constains world {}", hello.contains("World"));

    println!("replace {}", hello.replace("World", "There"));

    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    //create string w/ cap 
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    println!("{}",s);

    //assertion
    assert_eq!(2,s.len());
    assert_eq!(10,s.capacity());


    println!("{}",hello);
}