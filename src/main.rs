mod characters;
mod utils;
mod story;

fn main() {
    let game_start = true;

    while game_start {
        utils::cls();
        println!("Hello and welcome to CODEIRA!");
        println!("This is a story of a young coder.");
        println!("He is about to embark on a big journey.");
        println!("");
        println!("HERO");
        let hero = characters::create_hero();
        hero.print_details();
        println!("");
        utils::slp(2);
        let base_choice = story::base_choice();
        if base_choice == "1" {
            utils::cls();
            story::select_location();
            utils::slp(2);
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
