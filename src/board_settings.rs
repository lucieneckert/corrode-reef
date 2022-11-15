pub enum BlendModes {
  True,
  NeighborAvg,
}

// 🔃: Setting requires board refresh to update.
pub struct Settings {
  
  // number of colors on the board. 🔃
  pub num_colors: i32,
  // size of the board. 🔃
  pub size: i32,

  // timescale. NOT IMPLEMENTED
  pub timescale: i32,
  // prob that a tile will mutate
  pub prob_mutate: f32,
  // influence of momentum on tile mutation
  pub momentum_factor: f32,
  // controls bias between mutating up or down. 
  pub gravity: f32, 
  // horizontal/vertical mutation choice bias
  pub bias: i32,
  // how to render each cell's final color
  pub render_blend: BlendModes,
}