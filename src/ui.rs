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
          200.0,
          25.0 
      ),
      None, 
      None, 
      val, 
      min_val, 
      max_val
  )
}

pub fn draw_settings_toggle_group(
  gui_constants: &GUIConstants,
  draw: &mut RaylibDrawHandle,
  row: i32,
  _range: i32,
  val: i32
) -> i32 {
  return draw.gui_toggle_group(
    Rectangle::new(
      gui_constants.gui_x as f32, 
      (gui_constants.gui_y + (gui_constants.gui_row_offset * row)) as f32, 
      200.0,
      25.0
    ), 
    Some(rstr!("Bias")),
    val
  );
}