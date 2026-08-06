use macroquad::prelude::rand;
use std::collections::VecDeque;

use crate::position::Pos;
use crate::constants::BOARD_SIZE;

pub struct Apple {
  pub pos: Pos,
}

impl Apple {
  pub fn new(prohibited_positions: &VecDeque<Pos>) -> Self {
    return Self {
      pos: Self::generate_valid_position(prohibited_positions),
    }
  }

  pub fn goto_valid_position(&mut self, prohibited_positions: &VecDeque<Pos>) {
    self.pos = Self::generate_valid_position(prohibited_positions);
  }

  fn generate_valid_position(prohibited_positions: &VecDeque<Pos>) -> Pos {
    loop {
      let candidate = Pos {
        x: rand::gen_range(0, BOARD_SIZE),
        y: rand::gen_range(0, BOARD_SIZE),
      };
  
      if !prohibited_positions.iter().any(|pos| pos == &candidate) {
        break candidate;
      }
    }
  }
}
