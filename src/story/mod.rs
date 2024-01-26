pub mod locations;
pub mod characters;
use locations::Location;

use std::io;

use crate::utils;

pub fn base_choice() -> String {
    let menu = [
        ("1", "Explore"),
        ("2", "Inventory (not implemented)"),
        ("3", "Settings (not implemented)"),
        ("4", "Exit"),
    ];
    utils::print_menu(menu);

    println!("Select your path");
    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    let user_choice = choice.trim();
    return user_choice.to_string()
}

pub fn select_location() -> String {
    let menu = [
        ("1", "Woods"),
        ("2", "Mountains"),
        ("3", "Riverside"),
        ("4", "Ocean"),
    ];
    utils::print_menu(menu);

    println!("Select location to explore");
    let mut path = String::new();

    io::stdin()
        .read_line(&mut path)
        .expect("Failed to read line");

    let user_choice = path.trim();
    return user_choice.to_string()
}

pub fn choose_action(location: &Location) -> String {
    let menu: [(&str, &str); 4] = [
        ("1", "Not Implemented"),
        ("2", "Not Implemented"),
        ("3", "Not Implemented"),
        ("4", "Not Implemented"),
    ];

    println!("Currently at {:?}", location);
    println!("{}", "*".repeat(100));
    utils::print_menu(menu);
    println!("{}", "*".repeat(100));
    println!("Choose action:");
    let mut action = String::new();

    io::stdin()
        .read_line(&mut action)
        .expect("Failed to read line");

    let user_choice = action.trim();
    return user_choice.to_string()
}