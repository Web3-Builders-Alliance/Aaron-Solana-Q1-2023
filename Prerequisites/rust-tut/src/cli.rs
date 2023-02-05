use std::env;

pub fn run(){
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = "milz";
    let status = "100%";
    //println!("Args: {:?}", command);
    if command == "hello" {
        println!("Hi {}", name);
    } else if command == "status" {
        println!("status is {}", status)
    } else {
        println!("not valid");
    }
}