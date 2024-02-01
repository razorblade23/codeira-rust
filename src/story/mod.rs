pub mod locations;
pub mod characters;
use locations::Location;
use rand::prelude::*;
use std::io;

use crate::utils::{self, print_menu};

use self::characters::Hero;
use self::characters::simulate_fight;

#[derive(Debug)]
pub enum Action {
    Fight,
    MeetCharacter,
    FindArmor,
    FindWeapon,
}


impl Action {
    pub fn weighted_random_action() -> &'static Action {
        let actions = [
            (&Action::MeetCharacter, 6),
            (&Action::FindArmor, 5),
            (&Action::FindWeapon, 4),
            (&Action::Fight, 20),    // Higher weight for MoveUp
        ];

        let mut rng = rand::thread_rng();
        let total_weight: i32 = actions.iter().map(|(_, weight)| *weight).sum();
        let mut chosen_weight = rng.gen_range(0..total_weight);

        println!("Total weight: {}, chosen weight: {}", total_weight, chosen_weight);

        for (action, weight) in actions.iter() {
            if chosen_weight <= 0 {
                return *action;
            }
            chosen_weight -= *weight;
        }

        // If no action was chosen within the loop, return a default action
        actions.last().unwrap().0
    }
}

pub fn start_menu() -> String {
    let menu = [
        ("1", "Start game"),
        ("2", "Load game (not implemented)"),
        ("3", "Settings (not implemented)"),
        ("4", "Exit"),
    ];
    utils::print_menu(menu);

    println!("Select option");
    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    let user_choice = choice.trim();
    return user_choice.to_string()
}

pub fn base_choice() -> String {
    let menu = [
        ("1", "Explore"),
        ("2", "Inventory (not implemented)"),
        ("3", "Save game (not implemented)"),
        ("4", "Back to main menu"),
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

pub fn random_action(action: &Action, hero: &mut Hero) {
    match action {
        Action::Fight => {
            let mut enemy = characters::Enemy::new(String::from("Skeleton"), 20, 1.0, 5.0);
            println!("Hero encounters Enemy");
            simulate_fight(hero, &mut enemy);
            println!("Not implemented yet ...");
            utils::slp(10);
            }
        Action::MeetCharacter => {
            println!("Hero meets a character");
            println!("Not implemented yet ...");
            utils::slp(10);
        },
        Action::FindArmor => {
            println!("Hero finds armor");
            println!("Not implemented yet ...");
            utils::slp(10);
        },
        Action::FindWeapon => {
            println!("Hero finds a weapon");
            println!("Not implemented yet ...");
            utils::slp(10);
        },
    }

}


