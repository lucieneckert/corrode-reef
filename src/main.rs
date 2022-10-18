use std::{thread, time::Duration};
use raylib::prelude::*;

mod board;
mod board_settings;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1200, 800)
        .title("corrode-reef")
        .build();
    let mut num_colors = 4;
    let mut board = board::Board::from_dim(100, 100).randomize_cells(num_colors);
    let settings = board_settings::Settings {timescale: 1, prob_mutate: 0.6, momentum_factor: 0.01};
    let mut reset = false;
    // spawn an input thread
    while !rl.window_should_close() {
        if reset {
            board = board.randomize_cells(num_colors);
        }
        // step board with current settings
        board = board.step(&settings);
        // render board
        let mut draw = rl.begin_drawing(&thread);
        draw.clear_background(Color::BLACK);
        board.render(&mut draw, 8);
        // draw UI
        let gui_x = 825;
        let gui_y = 25;
        draw.draw_text("corrode reef", gui_x, gui_y, 50, Color::WHITE);
        reset = draw.gui_button(Rectangle::new(gui_x as f32, gui_y as f32 + 50.0, 100.0, 100.0), Some(rstr!("reset")));
        num_colors = draw.gui_slider(
            Rectangle::new(gui_x as f32, gui_y as f32 + 150.0, 300.0, 25.0), 
            Some(rstr!("2")), Some(rstr!("6")), 
            num_colors as f32, 2.0, 6.0
        ) as i32;
        thread::sleep(Duration::from_millis(25));
    };
}
