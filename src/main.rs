mod story;
mod utils;
use story::characters;


fn main() {
    let game_start = true;
    let mut hero = characters::Hero::new("David".to_string(), 100, 5.0, 0.0, 0.0, 1);

    utils::cls();
    println!("Hello and welcome to CODEIRA!");
    println!("This is a story of a young coder.");
    println!("He is about to embark on a big journey.");
    println!("");

    while game_start {
        utils::slp(5);
        utils::cls();

        let start_menu = story::start_menu();

        // Start game
        if start_menu == "1"{
            loop {
                utils::cls();
                println!("HERO");
                hero.print_details();
                println!("");

                let base_choice = story::base_choice();
                // Explore
                if base_choice == "1" {
                    utils::cls();
                    // let user_location = story::select_location();
                    // let location = utils::parse_location(user_location);
                    // let location = utils::parse_location(user_location);
                    let random_action = story::Action::weighted_random_action();
                    story::random_action(&random_action, &mut hero);
                
                // Inventory
                } else if base_choice == "2" {
                    println!("That feature is not yet implemented");
                
                // Save game
                } else if base_choice == "3" {
                    println!("That feature is not yet implemented");
                
                // Back to main menu
                } else if base_choice == "4" {
                    break

                } else {
                    println!("Not a valid choice");
                }
            }
        
        // Load game
        } else if start_menu == "2" {
            println!("That feature is not yet implemented");
        }
        // Settings
        else if start_menu == "3" {
            println!("That feature is not yet implemented");
        }
        // Exit
        else if start_menu == "4" {
            println!("Thanks for playing");
            break
        }
    }
}
