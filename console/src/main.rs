extern crate game_of_life;
use game_of_life::manager;
use game_of_life::manager::Game;


// tests
// mod manager_tests;

fn main() {
    println!("Starting Game manager ⭐");
    // let game: Game = manager::Game::new(64, 64);
    let mut game: Game = manager::load_game_from_file(&String::from("map"))
        .expect("Error while parsing map file.");
    println!("Initial State");
    manager::print_game(& game);

    for i in 0..1000 {
        std::thread::sleep(std::time::Duration::from_millis(100));
        game.step();
        print!("{}[2J", 27 as char);
        println!("Step {}", i);
        manager::print_game(& game);
    }
}
