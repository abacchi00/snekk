
use std::collections::VecDeque;

use crate::constants::BOARD_SIZE;
use crate::direction::Direction;
use crate::position::Pos;

pub struct Snekk {
  pub alive: bool,
  pub body: VecDeque<Pos>,
  pub pos: Pos,
  direction: Direction,
  next_direction: Direction,
}

impl Snekk {
  const INITIAL_POS: [Pos; 3] = [
    Pos { x: 3, y: 1 },
    Pos { x: 2, y: 1 },
    Pos { x: 1, y: 1 },
  ];

  pub fn new() -> Self {
    return Self {
      alive: true,
      body: VecDeque::from(Self::INITIAL_POS),
      direction: Direction::Right,
      next_direction: Direction::Right,
      pos: Self::INITIAL_POS[0],
    }
  }

  pub fn change_direction(&mut self, new_direction: Direction) {
    if new_direction == self.direction.opposite() { return; }

    self.next_direction = new_direction;
  }

  pub fn advance(&mut self) {
    self.direction = self.next_direction;

    match self.direction {
      Direction::Up => self.pos.y = self.pos.y.checked_sub(1).unwrap_or(BOARD_SIZE - 1),
      Direction::Down => self.pos.y = (self.pos.y + 1) % BOARD_SIZE,
      Direction::Left => self.pos.x = self.pos.x.checked_sub(1).unwrap_or(BOARD_SIZE - 1),
      Direction::Right => self.pos.x = (self.pos.x + 1) % BOARD_SIZE,
    }

    if self.body.iter().any(|pos| pos == &self.pos) {
      self.alive = false;
    }

    self.body.push_front(self.pos);
  }

  pub fn shrink(&mut self) {
    self.body.pop_back();
  }
}
