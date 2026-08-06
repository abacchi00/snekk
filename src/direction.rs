use macroquad::prelude::KeyCode;

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
  Up,
  Down,
  Left,
  Right,
}

impl Direction {
  pub fn from_keycode(keycode: KeyCode) -> Option<Self> {
    match keycode {
      KeyCode::Up => Some(Direction::Up),
      KeyCode::Down => Some(Direction::Down),
      KeyCode::Left => Some(Direction::Left),
      KeyCode::Right => Some(Direction::Right),
      _ => None
    } 
  }

  pub fn opposite(self) -> Direction {
    match self {
      Direction::Up => Direction::Down,
      Direction::Down => Direction::Up,
      Direction::Left => Direction::Right,
      Direction::Right => Direction::Left,
    }
  }
}
