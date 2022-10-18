
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
              if p < 0.25 && x > 0 {
                  new_color = self.cells[(idx - 1) as usize].color;
              } else if p < 0.5 && x < self.size_x - 1 {
                  new_color = self.cells[(idx + 1) as usize].color;
              } else if p < 0.75 && y > 0 {
                  new_color = self.cells[(idx - self.size_x) as usize].color;
              } else if y < self.size_y - 1{
                  new_color = self.cells[(idx + self.size_x) as usize].color;
              }
          }
          new_cells.push(Cell {
              momentum:  if new_color == old_color {cell.momentum + 1} else {0},
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

  // noteL num_colors <= 5
  pub fn randomize_cells(&self, num_colors: i32) -> Board {
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

  pub fn render(&self, draw : &mut RaylibDrawHandle, scale: i32) {
      for (i, cell) in self.cells.iter().enumerate() {
          let idx = i as i32;
          draw.draw_rectangle(idx % self.size_x * scale, idx / self.size_y * scale, scale, scale, cell.color);
      }
  }
}