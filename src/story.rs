use std::io;

pub fn base_choice() -> String {
    println!("1) Explore");
    println!("2) Inventory (not implemented)");
    println!("3) Settings (not implemented)");
    println!("4) Exit");

    println!("Select your path");
    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    let user_choice = choice.trim();
    return user_choice.to_string()
}

pub fn select_location() {
    println!("1) Woods");
    println!("2) Mountains");
    println!("3) Riverside");
    println!("4) Ocean");

    println!("Select location to explore");
    let mut path = String::new();

    io::stdin()
        .read_line(&mut path)
        .expect("Failed to read line");

    if path.trim() == "1" {
        println!("You have selected {}", "Woods")
    } else if path.trim() == "2" {
        println!("You have selected {}", "Mountains")
    } else if path.trim() == "3" {
        println!("You have selected {}", "Riverside")
    } else if path.trim() == "4" {
        println!("You have selected {}", "Ocean")
    } else {
        println!("Not a valid choice")
    }
}