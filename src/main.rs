use std::{env, thread::sleep, time::Duration};

use game_of_life::GameOfLife;

fn main() {
    let dur = 100; // Speed of animation
    let mut game = GameOfLife::new(40,40);

    if env::args().any(|s| s.contains("rand")) {
         game.randomize();
    }else{
        game.add_gosper_glider_gun();
    }
    loop {
        game.display(); // Clear and display the grid
        sleep(Duration::from_millis(dur));
        game.next_generation(); // Compute next state
    }
}
