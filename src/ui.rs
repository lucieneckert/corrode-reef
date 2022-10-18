use raylib::prelude::*;

pub struct GUIConstants {
  pub gui_x: i32,
  pub gui_y: i32,
  pub gui_row_offset: i32
}

pub fn draw_settings_slider(
  gui_constants: &GUIConstants,
  draw : &mut RaylibDrawHandle, 
  _label: &str,
  row: i32,
  min_val: f32,
  max_val: f32,
  val: f32
) -> f32 {
  return draw.gui_slider(
      Rectangle::new(
          gui_constants.gui_x as f32, 
          (gui_constants.gui_y + (gui_constants.gui_row_offset * row)) as f32, 
          100.0,
          50.0 
      ),
      None, 
      None, 
      val, 
      min_val, 
      max_val
  )
}