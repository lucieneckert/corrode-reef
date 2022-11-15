
use raylib::prelude::*;

use crate::board_settings;
struct Cell {
  momentum: i32,
  color: Color
}

impl Copy for Cell {}

impl Clone for Cell {
  fn clone(&self) -> Self {
      *self
  }
}

pub struct Board {
  size_x: i32,
  size_y: i32,
  cells: Vec<Cell>
}

impl Board {

  pub fn step(&self, settings: &board_settings::Settings) -> Board {
    // define probability constants from settings
    let p_mutate_left = 0.25;
    let p_mutate_right = p_mutate_left + 0.25;
    let p_mutate_up = p_mutate_right + 0.25 + (settings.gravity * 0.25);
    // let p_mutate_down = p_mutate_up + 0.25 + (settings.gravity * 0.25);
    let mut new_cells : Vec<Cell> = Vec::new();
    for (i, cell) in self.cells.iter().enumerate() {
        let idx = i as i32;
        let x = idx % self.size_x;
        let y = idx / self.size_x;
        let old_color = cell.color;
        let p = rand::random::<f32>() + cell.momentum as f32 * settings.momentum_factor;
        let mut new_color = old_color;
        if p < settings.prob_mutate {
            let p = rand::random::<f32>();
            // mutate left
            if p < p_mutate_left && x > 0 {
                new_color = self.cells[(idx - 1) as usize].color;
            // mutate right
            } else if p < p_mutate_right && x < self.size_x - 1 {
                new_color = self.cells[(idx + 1) as usize].color;
            // mutate up
            } else if p < p_mutate_up && y > 0 {
                new_color = self.cells[(idx - self.size_x) as usize].color;
            // mutate down
            } else if y < self.size_y - 1{
                new_color = self.cells[(idx + self.size_x) as usize].color;
            }
        }
        new_cells.push(Cell {
            momentum: if new_color == old_color {cell.momentum + 1} else {0},
            color: new_color
        })   
    }
    return Board {
        size_x: self.size_x,
        size_y: self.size_y,
        cells: new_cells
    }
  }

  pub fn from_dim(x: i32, y: i32) -> Board {
      Board {
          size_x: x,
          size_y: y,
          cells: vec![Cell {momentum: 0, color: Color::WHITE}; (x * y) as usize]
      }
  }

  pub fn randomize_cells(&self, num_colors: i32) -> Board {
      // note: num_colors <= 6 
      let colors = vec![Color::WHITE, Color::PINK, Color::PURPLE, Color::YELLOW, Color::MAGENTA, Color::ORANGE, Color::SKYBLUE];
      let mut new_cells : Vec<Cell> = Vec::new();
      for (i, _cell) in self.cells.iter().enumerate() {
          let cell_color = colors[i % num_colors as usize];
          new_cells.push(Cell {
              momentum: 0,
              color: cell_color
          })
      }
      return Board {
          size_x: self.size_x,
          size_y: self.size_y,
          cells: new_cells
      }
  }

  pub fn render(&self, draw : &mut RaylibDrawHandle, blend : &board_settings::BlendModes, scale: i32) {
      for (i, cell) in self.cells.iter().enumerate() {
          let idx = i as i32;
          let cell_color : Color = match blend {
            board_settings::BlendModes::True => cell.color,
            board_settings::BlendModes::NeighborAvg => average_color(vec![
                &cell.color,
                if i as i32 % self.size_x > 0 {&self.cells[i - 1].color} else {&cell.color},
                if i as i32 % self.size_x < self.size_x - 1 {&self.cells[i + 1].color} else {&cell.color},
                if i as i32 / self.size_x > 0 {&self.cells[i - self.size_x as usize].color} else {&cell.color},
                if i as i32 / self.size_x < self.size_y - 1 {&self.cells[i + self.size_x as usize].color} else {&cell.color},
            ]),
        };
          draw.draw_rectangle(idx % self.size_x * scale, idx / self.size_y * scale, scale, scale, cell_color);
      }
  }
}

#[allow(dead_code)]
fn average_color(cols : Vec<&Color>) -> Color {
    let sum_hsv: Vector3 = cols.iter()
      .map(|c| c.color_to_hsv())
      .fold(Vector3{x: 0.0, y:0.0, z:0.0}, |acc, hsv| acc + hsv);
    let avg_hsv = sum_hsv / cols.len() as f32;
    return Color::color_from_hsv(avg_hsv.x, avg_hsv.y, avg_hsv.z);
}