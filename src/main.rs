extern crate ggez;

use ggez::{ContextBuilder};
use ggez::event;

mod cell;
mod mouse;
mod mygame;
use mygame::GameOfLife;

fn main() {
    let window_mode = ggez::conf::WindowMode::default().dimensions(600.0, 600.0);

    // Make a Context.
    let (mut ctx, mut event_loop) = ContextBuilder::new("GOL_Rust", "Ajinkya Borade")
        .window_mode(window_mode)
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let mut my_game = GameOfLife::new(&mut ctx);

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    }
}
