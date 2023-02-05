pub fn run() {
    let age: u8 = 12; 
    let check_id: bool = false;
    let knows_person: bool = true;
    
    // if else
    if age >= 21 && check_id || knows_person {
        println!("Bartender: what you want to drink");
    } else if age < 21 && check_id {
        println!("sorry leave");
    } else {
        println!("i need to see id");
    }

    // short hand
    let is_of_age = if age >= 21 { true } else {false};
    println!("is of age {}", is_of_age);

}