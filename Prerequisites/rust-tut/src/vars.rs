pub fn run() {
    let name = "Aaron";
    let mut age = 35;
    println!("my name is {} and im {}", name, age);

    age = 38;
    println!("my name is {} and im {}", name, age);

    // define constant (usually upper, has type)
    const ID: i32 = 001;
    println!("ID: {}", ID);

    //assign multiple
    let (my_name, my_age) = ("Aaron", 35);
    println!("{} is {}", my_name, my_age);
}