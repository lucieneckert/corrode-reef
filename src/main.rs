use std::{thread, time::Duration};
use raylib::prelude::*;

mod board;
mod board_settings;
mod ui;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1200, 800)
        .title("corrode-reef")
        .build();
    // define UI constants 
    let gui_constants = ui::GUIConstants {
     gui_x: 825,
     gui_y: 25,
     gui_row_offset: 50,
    };
    
    // define initial board settings
    let mut settings = board_settings::Settings {
        num_colors: 4,
        size: 100,
        timescale: 1, 
        prob_mutate: 0.6, 
        momentum_factor: 0.01,
        gravity: 0.0,
        bias: 0,
        render_blend: board_settings::BlendModes::NeighborAvg,
    };
    let mut board = board::Board::from_dim(settings.size, settings.size).randomize_cells(settings.num_colors);
    let mut reset = false;
    // spawn an input thread
    while !rl.window_should_close() {
        if reset {
            board = board.randomize_cells(settings.num_colors);
        }
        // step board with current settings
        board = board.step(&settings);
        // render board
        let mut draw = rl.begin_drawing(&thread);
        draw.clear_background(Color::BLACK);
        board.render(&mut draw, &settings.render_blend, 8);
        // draw UI
        // draw.draw_text("corrode reef", gui_x, gui_y, 50, Color::WHITE);
        reset = draw.gui_button(Rectangle::new(gui_constants.gui_x as f32, gui_constants.gui_y as f32, 100.0, 40.0), Some(rstr!("reset")));
        settings.num_colors = ui::draw_settings_slider(&gui_constants, &mut draw, rstr!("# Colors"), 1, 2.0, 6.0, settings.num_colors as f32) as i32;
        settings.gravity = ui::draw_settings_slider(&gui_constants, &mut draw, rstr!("Gravity"), 2, 0.0, 1.0, settings.gravity);
        settings.momentum_factor = ui::draw_settings_slider(&gui_constants, &mut draw, rstr!("Momentum"), 3, -1.0, 1.0, settings.momentum_factor);
        settings.bias = ui::draw_settings_toggle_group(&gui_constants, &mut draw, 4, 3, 0);
        // #TODO: Move the render of the board to a separate thread from UI
        thread::sleep(Duration::from_millis(25));
    };
}