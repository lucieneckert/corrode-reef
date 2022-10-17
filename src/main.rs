use std::{thread, time::Duration};
use raylib::prelude::*;

mod board;
mod board_settings;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1200, 800)
        .title("corrode-reef")
        .build();
    let mut board = board::Board::from_dim(100, 100).randomize_cells(4);
    let settings = board_settings::Settings {timescale: 1, prob_mutate: 0.6, momentum_factor: 0.01};
    let mut reset = false;
    // spawn an input thread
    while !rl.window_should_close() {
        if reset {
            board = board.randomize_cells(4);
        }
        // step board with current settings
        board = board.step(&settings);
        // render board
        let mut draw = rl.begin_drawing(&thread);
        draw.clear_background(Color::BLACK);
        board.render(&mut draw, 8);
        // draw UI
        draw.draw_text("corrode reef", 825, 25, 50, Color::WHITE);
        reset = draw.gui_button(Rectangle::new(0.0, 0.0, 100.0, 100.0), None);
        thread::sleep(Duration::from_millis(25));
    };
}
