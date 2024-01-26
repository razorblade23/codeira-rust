mod story;
mod utils;
use story::characters;


fn main() {
    let game_start = true;
    let mut hero = characters::create_hero();

    while game_start {
        utils::cls();
        println!("Hello and welcome to CODEIRA!");
        println!("This is a story of a young coder.");
        println!("He is about to embark on a big journey.");
        println!("");
        println!("HERO");
        hero.print_details();
        println!("");
        utils::slp(2);


        let base_choice = story::base_choice();
        if base_choice == "1" {
            utils::cls();
            let user_location = story::select_location();
            let location = utils::parse_location(user_location);
            // let location = utils::parse_location(user_location);
            hero.set_location(&location);
            utils::slp(2);
            utils::cls();
            let user_action = story::choose_action(&location);

        } else if base_choice == "2" {
            println!("That feature is not yet implemented");

        } else if base_choice == "3" {
            println!("That feature is not yet implemented");

        } else if base_choice == "4" {
            println!("Thanks for playing");
            break

        } else {
            println!("Not a valid choice");
        }
    }
}
