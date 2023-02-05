pub fn run() {
    //print to console
    println!("Hello from print.rs file");
    //Basic Formatting
    println!("Number: {}",1);
    println!("{} is from {}", "Aaron", "Portland");

    // Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}",
     "Aaron", "Portland", "code");

    // Named args
    println!("{name} likes to play {activity}",
    name = "Aaron", activity = "games");

    // Binary / Hex
    println!("Binary {:b}, Hex {:x}, Octal {:o}", 10,10,10);

    // Debug trait
    println!("{:?}", (12, true, "hello"));

    // basic math
    println!("10+10={}",10+10);
}