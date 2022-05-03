use tetra::ContextBuilder;
use rust_pong::state::GameState;
use rust_pong::state::{WINDOW_WIDTH, WINDOW_HEIGHT};

fn main() -> tetra::Result {
    println!("Hi Simon chill");

    let ran = ContextBuilder::new("Pong", WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new);

    // debugging / error / or whatever
    match ran {
        Ok(_) => {
            println!("Hurra");
        }
        Err(_) => {
            println!("Flark frigging dragons frigged it up again");
        }
    }

    ran
}
